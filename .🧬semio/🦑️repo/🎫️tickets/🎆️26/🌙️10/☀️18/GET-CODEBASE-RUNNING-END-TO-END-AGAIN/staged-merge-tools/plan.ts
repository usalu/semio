/** 🧩 Merges every staged library patch set into one verified, ordered apply plan and drives it; planning never touches the repository it plans for. */
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {constants, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename, dirname, join, posix} from "node:path";

export type Op = {anchor: string; replacement: string};
export type Kind = "patch" | "write" | "create" | "delete";
export type Target = {path: string; kind: Kind; staged: string | null; baseSha: string | null; stagedSha: string | null; ops: Op[]; alsoTouches: string[]};
export type Verify = {name: string; argv: string[]; gate?: boolean; heavy?: boolean; timeoutSeconds?: number};
export type Spec = {id: string; dir: string; result?: string; adapter?: string; entry?: string[]; ignoreSync?: string | null; confirm?: string; restage?: string[]; after?: string[]; note?: string; verify?: Verify[]};
export type SetInfo = {spec: Spec; id: string; dir: string; state: "ready" | "provisional" | "skipped"; note: string; adapter?: string; entry?: string[]; targets: Target[]; fingerprint: string | null; result: string | null};
export type Run = {id?: string; status: "applied" | "noop" | "refused"; args: string[]; message?: string; changed?: number; undeclared?: string[]};
export type Context = {root: string; ticket: string; here: string; registry: string; planFile: string; scratch: string; keep: boolean};
type Files = Map<string, Buffer | null>;
type Digests = Map<string, string | null>;
type Json = any;

const SCRIPT = "\u{1F4DC}️script.ts";
const OWNER = "\u{1F7E6}️.ts";
const QUIET_SECONDS = 180;

const sha = (data: Buffer | string): string => createHash("sha256").update(data).digest("hex");
const fileSha = (path: string): string | null => (existsSync(path) && statSync(path).isFile() ? sha(readFileSync(path)) : null);
const posixPath = (path: string): string => path.replaceAll("\\", "/");
const absolute = (base: string, path: string): string => join(base, ...path.split("/"));
const tail = (text: string, lines = 12): string => text.trim().split("\n").slice(-lines).join("\n");
const readJson = (path: string): Json => JSON.parse(readFileSync(path, "utf8"));

/** 🔭 Walks up from a directory to the repository root marker pair. */
export function findRoot(from: string): string {
  for (let path = from; ; path = dirname(path)) {
    if (existsSync(join(path, "bunfig.toml")) && existsSync(join(path, "bun.lock"))) return path;
    if (dirname(path) === path) throw new Error("repository root not found");
  }
}

/** 🧭 Builds the context of the real ticket this tool lives in. */
export function defaultContext(): Context {
  const here = import.meta.dir;
  return {root: findRoot(here), ticket: dirname(here), here, registry: join(here, "sets.json"), planFile: join(here, "plan.json"), scratch: process.env.SEMIO_STAGED_MERGE_SCRATCH ?? tmpdir(), keep: false};
}

/** 📐 Validates a value against the subset of JSON Schema the contracts use and returns every violation. */
export function validate(value: Json, node: Json, schema: Json, path = "$"): string[] {
  if (node.$ref) return validate(value, node.$ref.split("/").slice(1).reduce((cursor: Json, key: string) => cursor[key], schema), schema, path);
  const errors: string[] = [];
  const kind = (item: Json): string => (item === null ? "null" : Array.isArray(item) ? "array" : Number.isInteger(item) ? "integer" : typeof item);
  if ("const" in node && value !== node.const) return [`${path}: expected ${JSON.stringify(node.const)}`];
  if (node.enum && !node.enum.includes(value)) return [`${path}: ${JSON.stringify(value)} not in ${JSON.stringify(node.enum)}`];
  if (node.type) {
    const allowed: string[] = [node.type].flat();
    const actual = kind(value);
    if (!allowed.includes(actual) && !(actual === "integer" && allowed.includes("number"))) return [`${path}: ${actual} not in ${allowed.join("|")}`];
  }
  if (typeof value === "string" && node.pattern && !new RegExp(node.pattern, "u").test(value)) errors.push(`${path}: does not match ${node.pattern}`);
  if (Array.isArray(value)) {
    if (node.minItems !== undefined && value.length < node.minItems) errors.push(`${path}: fewer than ${node.minItems} items`);
    if (node.items) value.forEach((item, index) => errors.push(...validate(item, node.items, schema, `${path}[${index}]`)));
  }
  if (value && typeof value === "object" && !Array.isArray(value)) {
    for (const key of node.required ?? []) if (!(key in value)) errors.push(`${path}: missing ${key}`);
    for (const [key, item] of Object.entries(value)) {
      if (node.properties?.[key]) errors.push(...validate(item, node.properties[key], schema, `${path}.${key}`));
      else if (node.additionalProperties === false) errors.push(`${path}: unexpected ${key}`);
    }
  }
  return errors;
}

/** 📜 Loads the contract schema next to this tool. */
export const loadSchema = (ctx: Context): Json => readJson(join(ctx.here, "schema.json"));

/** 🗂️ Loads and validates the registry of staged sets; refuses unbounded test commands. */
export function loadRegistry(ctx: Context): Spec[] {
  const schema = loadSchema(ctx);
  const registry = readJson(ctx.registry);
  const errors = validate(registry, schema.$defs.Registry, schema);
  const ids = new Set<string>();
  for (const spec of registry.sets as Spec[]) {
    if (ids.has(spec.id)) errors.push(`duplicate id ${spec.id}`);
    ids.add(spec.id);
  }
  for (const spec of registry.sets as Spec[]) {
    for (const id of spec.after ?? []) if (!ids.has(id) || id === spec.id) errors.push(`${spec.id}: after ${id} is not another registered set`);
  }
  const hard = (registry.sets as Spec[]).flatMap((spec): [string, string][] => (spec.after ?? []).map((id): [string, string] => [id, spec.id]));
  if (orderSets([...ids], hard, []).cycle.length) errors.push(`after edges form a cycle: ${orderSets([...ids], hard, []).cycle.join(", ")}`);
  for (const spec of registry.sets as Spec[]) {
    for (const item of spec.verify ?? []) {
      const [program, verb, ...rest] = item.argv;
      const paths = rest.filter((word) => !word.startsWith("-"));
      if (program === "bun" && verb === "test" && (!paths.length || paths.some((word) => !/^(\.\/|\/|\{)/u.test(word)))) errors.push(`${spec.id}/${item.name}: bun test needs explicit ./ or absolute file paths`);
    }
  }
  if (errors.length) throw new Error(`invalid registry ${ctx.registry}:\n${errors.join("\n")}`);
  return registry.sets;
}

let pythonCommand: string | undefined;
/** 🐍 Finds the python interpreter the staged apply tools run on. */
export function python(): string {
  if (pythonCommand) return pythonCommand;
  for (const candidate of ["python3", "python"]) if (spawnSync(candidate, ["--version"]).status === 0) return (pythonCommand = candidate);
  throw new Error("python3 not found");
}

/** 🚪 Resolves the command that runs a set's own apply tool, relative to its directory. */
export function resolveEntry(dir: string): string[] | null {
  const shell = join(dir, "apply.sh");
  if (existsSync(shell)) {
    const match = /exec\s+python3?\s+"\$\(dirname "\$0"\)\/([^"]+\.py)"/u.exec(readFileSync(shell, "utf8"));
    return match ? [python(), match[1]!] : ["sh", "apply.sh"];
  }
  if (existsSync(join(dir, "apply.py"))) return [python(), "apply.py"];
  if (existsSync(join(dir, "apply.ts"))) return ["bun", "apply.ts"];
  return null;
}

const EXTRACT = `
import importlib.util, json, os, sys
path = sys.argv[1]
spec = importlib.util.spec_from_file_location("staged_apply_module", path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
here = os.path.dirname(os.path.abspath(path))
bases = json.load(open(os.path.join(here, "bases.json"), encoding="utf-8")) if os.path.exists(os.path.join(here, "bases.json")) else {}
patches = getattr(module, "PATCHES", {})
staged = os.path.relpath(getattr(module, "STAGED", os.path.join(here, "staged")), here)
rows = []
for name, relative in getattr(module, "TARGET", {}).items():
    ops = patches.get(name)
    rows.append({"name": name, "path": relative, "kind": "write" if ops is None else "patch", "base": bases.get(name), "staged": (staged + "/" + name) if ops is None else os.path.basename(path) + ":PATCHES[" + name + "]", "ops": [[a, b] for a, b in (ops or [])]})
for name, relative in getattr(module, "NEW", {}).items():
    rows.append({"name": name, "path": relative, "kind": "create", "base": None, "staged": staged + "/" + name, "ops": []})
sys.stdout.reconfigure(encoding="utf-8")
sys.stdout.write(json.dumps(rows, ensure_ascii=False))
`;

const target = (fields: Partial<Target> & Pick<Target, "path" | "kind">): Target => ({staged: null, baseSha: null, stagedSha: null, ops: [], alsoTouches: [], ...fields, path: posixPath(fields.path)});

const ADAPTERS: Record<string, (dir: string, entry: string[]) => Target[]> = {
  declared: (dir) =>
    (readJson(join(dir, "inventory.json")).targets as Json[]).map((row) =>
      target({path: row.path, kind: row.kind, staged: row.staged ?? null, baseSha: row.baseSha ?? null, stagedSha: row.stagedSha ?? (row.staged ? fileSha(join(dir, row.staged)) : null), ops: row.ops ?? [], alsoTouches: row.alsoTouches ?? []}),
    ),
  "ops-json": (dir) =>
    (readJson(join(dir, "staged", "ops.json")).ops as Json[]).map((op) =>
      op.kind === "patch"
        ? target({path: op.path, kind: "patch", staged: `staged/ops.json[${basename(op.path)}]`, baseSha: op.base ?? null, stagedSha: op.sha ?? null, ops: (op.edits as Json[]).map((edit) => ({anchor: edit.old, replacement: edit.new}))})
        : target({path: op.path, kind: op.kind === "delete" ? "delete" : op.base === null ? "create" : "write", staged: op.kind === "delete" ? null : `staged/blobs/${op.blob}`, baseSha: op.base ?? null, stagedSha: op.kind === "delete" ? null : op.sha}),
    ),
  "patches-json": (dir) =>
    Object.entries(readJson(join(dir, "patches.json")) as Record<string, Json>).map(([path, row]) =>
      target({path, kind: "patch", staged: `patches.json[${basename(path)}]`, baseSha: row.baseSha256, ops: (row.ops as string[][]).map(([anchor, replacement]) => ({anchor: anchor!, replacement: replacement!})), alsoTouches: basename(path) === SCRIPT ? [posixPath(join(dirname(path), OWNER))] : []}),
    ),
  "py-module": (dir, entry) => {
    const run = spawnSync(python(), ["-c", EXTRACT, join(dir, entry[1]!)], {cwd: dir, encoding: "utf8", maxBuffer: 1 << 28, env: {...process.env, PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1"}});
    if (run.status !== 0) throw new Error(tail(run.stderr || run.stdout));
    return (JSON.parse(run.stdout) as Json[]).map((row) =>
      target({path: row.path, kind: row.kind, staged: row.staged, baseSha: row.base, stagedSha: row.kind === "patch" ? null : fileSha(join(dir, row.staged)), ops: row.ops.map(([anchor, replacement]: string[]) => ({anchor, replacement}))}),
    );
  },
  "edits-json": (dir) => {
    const merged = new Map<string, Op[]>();
    for (const name of readdirSync(join(dir, "staged", "patches")).filter((file) => file.endsWith(".json")).sort()) {
      for (const spec of readJson(join(dir, "staged", "patches", name)) as Json[]) merged.set(spec.file, [...(merged.get(spec.file) ?? []), ...(spec.edits as Json[]).map((edit) => ({anchor: edit.old, replacement: edit.new}))]);
    }
    return [...merged].map(([path, ops]) => target({path, kind: "patch", staged: "staged/patches", ops}));
  },
  "batch-json": (dir) => {
    const batch = readJson(join(dir, "batch.json"));
    const tiers = new Set(["export", "move", "removal", "third-party", "types", "requested", "tst6"]);
    const entries = (batch.entries as Json[]).filter((entry) => entry.apply && tiers.has(entry.tier));
    const selected = new Set(entries.map((entry) => entry.id));
    const found: Target[] = [];
    for (const manifest of [...new Set(entries.map((entry) => entry.packageJson as string))].sort()) {
      const consumed = entries.filter((entry) => entry.packageJson === manifest && entry.op === "add" && entry.field === "exports").map((entry) => posix.normalize(posix.join(posix.dirname(manifest), entry.value)));
      found.push(target({path: manifest, kind: "patch", staged: `batch.json[${manifest}]`, baseSha: batch.baseline[manifest]?.sha256 ?? null, alsoTouches: consumed}));
    }
    for (const glue of (batch.glue as Json[]).filter((row) => row.entry === null || selected.has(row.entry))) {
      const reads = ((glue.targets ?? []) as Json[]).map((row) => row.path as string);
      if (glue.kind === "create") found.push(target({path: glue.path, kind: "create", staged: `batch.json[${glue.id}]`, stagedSha: sha(glue.content), alsoTouches: reads}));
      else if (glue.kind === "extend") found.push(target({path: glue.path, kind: "patch", staged: `batch.json[${glue.id}]`, baseSha: glue.baselineSha256, alsoTouches: reads}));
      else found.push(target({path: glue.path, kind: "delete", staged: null, baseSha: glue.baselineSha256}));
    }
    const rewritten = new Map<string, Op[]>();
    for (const row of batch.rewrites as Json[]) if (selected.has(row.entry)) rewritten.set(row.file, [...(rewritten.get(row.file) ?? []), {anchor: `"${row.from}"`, replacement: `"${row.to}"`}]);
    for (const [path, ops] of rewritten) found.push(target({path, kind: "patch", staged: `batch.json[${path}]`, ops}));
    return found;
  },
  "tree-bases": (dir) =>
    Object.entries(readJson(join(dir, "bases.json")) as Record<string, string | null>).map(([path, base]) =>
      target({path, kind: base === null ? "create" : "write", staged: `staged/${path}`, baseSha: base, stagedSha: fileSha(join(dir, "staged", ...path.split("/")))}),
    ),
  tree: (dir) => {
    const found: Target[] = [];
    const walk = (folder: string): void => {
      for (const entry of readdirSync(folder, {withFileTypes: true})) {
        const full = join(folder, entry.name);
        if (entry.isDirectory()) walk(full);
        else if (entry.isFile()) found.push(target({path: posixPath(full.slice(join(dir, "staged").length + 1)), kind: "write", staged: posixPath(full.slice(dir.length + 1)), stagedSha: fileSha(full)}));
      }
    };
    walk(join(dir, "staged"));
    return found;
  },
};

/** 🕵️ Picks the adapter that understands the format a set staged its patches in. */
export function detect(dir: string, entry: string[]): string | null {
  const module = entry[1] && entry[1].endsWith(".py") && existsSync(join(dir, entry[1])) ? readFileSync(join(dir, entry[1]), "utf8") : "";
  if (existsSync(join(dir, "inventory.json"))) return "declared";
  if (existsSync(join(dir, "batch.json")) && existsSync(join(dir, "apply.ts"))) return "batch-json";
  if (existsSync(join(dir, "staged", "patches"))) return "edits-json";
  if (existsSync(join(dir, "staged", "ops.json"))) return "ops-json";
  if (existsSync(join(dir, "patches.json"))) return "patches-json";
  if (/\bTARGET\b/u.test(module) && /__name__\s*==\s*["']__main__["']/u.test(module)) return "py-module";
  if (existsSync(join(dir, "bases.json")) && existsSync(join(dir, "staged"))) return "tree-bases";
  if (existsSync(join(dir, "staged"))) return "tree";
  return null;
}

/** 🔖 Classifies a target's live file against the base and the staged result it was derived from. */
export function baseState(ctx: Context, item: Target): "base" | "applied" | "drifted" | "absent" | "unknown" {
  const live = fileSha(absolute(ctx.root, item.path));
  if (live === null) return item.kind === "create" || item.kind === "delete" ? (item.kind === "create" ? "base" : "applied") : "absent";
  if (item.stagedSha && live === item.stagedSha) return "applied";
  if (item.baseSha && live === item.baseSha) return "base";
  return item.baseSha === null && item.kind !== "create" ? "unknown" : "drifted";
}

const entryFile = (dir: string, entry: string[]): string | null => entry.map((word) => join(dir, word)).find((path) => existsSync(path) && statSync(path).isFile()) ?? null;

/** 🧾 Reads one registry entry into its inventory, readiness and fingerprint; never throws. */
export function describe(ctx: Context, spec: Spec): SetInfo {
  const dir = join(ctx.ticket, spec.dir);
  const base: SetInfo = {spec, id: spec.id, dir, state: "skipped", note: "", targets: [], fingerprint: null, result: null};
  if (!existsSync(dir)) return {...base, note: "directory missing (not started)"};
  const entry = spec.entry ?? resolveEntry(dir);
  if (!entry) return {...base, note: "no apply entry point (apply.sh, apply.py, apply.ts)"};
  const adapter = spec.adapter ?? detect(dir, entry);
  if (!adapter) return {...base, entry, note: "nothing staged yet (no inventory.json, ops.json, patches.json, bases.json, staged/)"};
  let targets: Target[];
  try {
    targets = ADAPTERS[adapter]!(dir, entry);
  } catch (error) {
    return {...base, entry, adapter, note: `inventory failed: ${tail(String((error as Error).message), 3)}`};
  }
  if (!targets.length) return {...base, entry, adapter, note: "empty inventory"};
  const inputs = [entryFile(dir, entry), ...["bases.json", "patches.json", "inventory.json", "batch.json", "staged/ops.json"].map((name) => join(dir, ...name.split("/")))].filter((path): path is string => !!path && existsSync(path));
  const age = (Date.now() - Math.max(...inputs.map((path) => statSync(path).mtimeMs))) / 1000;
  const result = spec.result && existsSync(join(ctx.ticket, spec.result)) ? spec.result : null;
  const fingerprint = sha(JSON.stringify([adapter, entry, fileSha(entryFile(dir, entry) ?? ""), fileSha(join(dir, "batch.json")), targets.map((item) => [item.path, item.kind, item.baseSha, item.stagedSha, item.alsoTouches, item.ops.map((op) => sha(`${op.anchor}\0${op.replacement}`))])]));
  const notes = [result ? null : "no result report yet (set still in progress)", age < QUIET_SECONDS ? `staged files modified ${Math.round(age)} s ago` : null].filter(Boolean);
  return {spec, id: spec.id, dir, state: result ? "ready" : "provisional", note: notes.join("; "), adapter, entry, targets, fingerprint, result};
}

const touched = (info: SetInfo): string[] => info.targets.flatMap((item) => [item.path, ...item.alsoTouches]);

/** 🏖️ A scratch repository root holding copies of set tools and of exactly the files they may touch. */
export class Sandbox {
  readonly dir: string;
  readonly tracked: string[];
  private readonly dirs = new Map<string, string>();
  private readonly initial = new Set<string>();

  constructor(readonly ctx: Context, sets: SetInfo[], paths: Iterable<string>) {
    mkdirSync(ctx.scratch, {recursive: true});
    this.dir = mkdtempSync(join(ctx.scratch, "sm-"));
    this.tracked = [...new Set(paths)];
    for (const name of ["bunfig.toml", "bun.lock"]) writeFileSync(join(this.dir, name), "");
    for (const info of sets) {
      const copy = join(this.dir, "t", info.id);
      cpSync(info.dir, copy, {recursive: true, mode: constants.COPYFILE_FICLONE, filter: (source) => !source.split(/[\\/]/u).includes("node_modules")});
      this.dirs.set(info.id, copy);
    }
    for (const path of this.tracked) {
      const from = absolute(ctx.root, path);
      if (!existsSync(from) || !statSync(from).isFile()) continue;
      const to = absolute(this.dir, path);
      mkdirSync(dirname(to), {recursive: true});
      cpSync(from, to, {mode: constants.COPYFILE_FICLONE});
      this.initial.add(path);
    }
  }

  snapshot(): Digests {
    return new Map(this.tracked.map((path) => [path, fileSha(absolute(this.dir, path))]));
  }

  contents(): Files {
    return new Map(this.tracked.map((path) => [path, existsSync(absolute(this.dir, path)) ? readFileSync(absolute(this.dir, path)) : null]));
  }

  undeclared(): string[] {
    const known = new Set([...this.tracked, "bunfig.toml", "bun.lock"]);
    const found: string[] = [];
    const walk = (folder: string, prefix: string): void => {
      for (const entry of readdirSync(folder, {withFileTypes: true})) {
        const path = prefix ? `${prefix}/${entry.name}` : entry.name;
        if (!prefix && entry.name === "t") continue;
        if (entry.isDirectory()) walk(join(folder, entry.name), path);
        else if (!known.has(path)) found.push(path);
      }
    };
    walk(this.dir, "");
    return found;
  }

  run(info: SetInfo, args: string[]): {rc: number; out: string} {
    const [command, ...rest] = info.entry!;
    const run = spawnSync(command!, [...rest, ...args], {cwd: this.dirs.get(info.id), encoding: "utf8", timeout: 300_000, maxBuffer: 1 << 28, env: {...process.env, PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1", PYTHONUTF8: "1"}});
    return {rc: run.status ?? -1, out: `${run.stdout ?? ""}${run.stderr ?? ""}${run.error ? String(run.error) : ""}`};
  }

  dispose(): void {
    if (!this.ctx.keep) rmSync(this.dir, {recursive: true, force: true});
  }
}

const standardFlags = (info: SetInfo): string[] => (info.spec.ignoreSync === null ? [] : [info.spec.ignoreSync ?? "--ignore-sync"]);

/** 🎯 Runs one set's own dry-run and apply once in a sandbox; a refusal stays a refusal, the drift fence is never relaxed unless the coordinator named the set for an explicit rebase. */
export function attempt(sandbox: Sandbox, info: SetInfo, rebase = false): Run {
  const extra = rebase ? ["--rebase"] : [];
  const args = ["apply", ...standardFlags(info), ...extra];
  const before = sandbox.snapshot();
  const known = new Set(sandbox.undeclared());
  const dry = sandbox.run(info, [...args, "--dry-run"]);
  if (dry.rc !== 0) return {id: info.id, status: "refused", args: extra, message: tail(dry.out)};
  const afterDry = sandbox.snapshot();
  const leaked = [...afterDry].filter(([path, digest]) => before.get(path) !== digest).map(([path]) => path);
  const strays = sandbox.undeclared().filter((path) => !known.has(path));
  if (leaked.length || strays.length) return {id: info.id, status: "refused", args: extra, message: `its --dry-run wrote files (${[...leaked, ...strays].slice(0, 3).join(", ")}${leaked.length + strays.length > 3 ? ", ..." : ""}); the contract is that --dry-run writes nothing`};
  const real = sandbox.run(info, [...args, ...(info.spec.confirm ? [info.spec.confirm] : [])]);
  if (real.rc !== 0) return {id: info.id, status: "refused", args: extra, message: tail(real.out)};
  const after = sandbox.snapshot();
  const changed = [...after].filter(([path, digest]) => before.get(path) !== digest).length;
  const undeclared = sandbox.undeclared();
  return {id: info.id, status: changed ? "applied" : "noop", args: extra, changed, ...(undeclared.length ? {undeclared} : {})};
}

const equalDigests = (left: Digests, right: Digests): boolean => left.size === right.size && [...left].every(([path, digest]) => right.get(path) === digest);

/** ⛓️ Lists the transitive predecessors of a set in an order that honours every after edge. */
export function predecessors(all: SetInfo[], id: string): SetInfo[] {
  const needed = new Set<string>();
  const visit = (name: string): void => {
    for (const before of all.find((info) => info.id === name)?.spec.after ?? []) if (!needed.has(before)) (needed.add(before), visit(before));
  };
  visit(id);
  const members = all.filter((info) => needed.has(info.id));
  const order = orderSets(members.map((info) => info.id), members.flatMap((info) => (info.spec.after ?? []).filter((before) => needed.has(before)).map((before): [string, string] => [before, info.id])), []).order;
  return order.map((name) => members.find((info) => info.id === name)!);
}

/** 🔁 Applies two sets in the given order on a fresh sandbox, each preceded by its own predecessors, and reports each step and the final file digests; an order that contradicts an after edge is not applicable. */
export function orderRun(ctx: Context, first: SetInfo, second: SetInfo, paths: string[], rebase: string[] = [], all: SetInfo[] = []): {steps: Run[]; ok: boolean; digests: Digests; files: Files} {
  const before = (info: SetInfo): SetInfo[] => predecessors(all, info.id);
  if (before(first).some((info) => info.id === second.id)) return {steps: [{id: first.id, status: "refused", args: [], message: `${first.id} declares after ${second.id}; applying ${first.id} first contradicts a hard edge`}], ok: false, digests: new Map(), files: new Map()};
  const sequence: SetInfo[] = [];
  for (const info of [...before(first), first, ...before(second), second]) if (!sequence.some((row) => row.id === info.id)) sequence.push(info);
  const sandbox = new Sandbox(ctx, sequence, [...new Set([...paths, ...sequence.flatMap(touched)])]);
  try {
    const steps: Run[] = [];
    for (const info of sequence) {
      steps.push(attempt(sandbox, info, rebase.includes(info.id)));
      if (steps[steps.length - 1]!.status === "refused") break;
    }
    return {steps, ok: steps.length === sequence.length && steps.every((step) => step.status !== "refused"), digests: sandbox.snapshot(), files: sandbox.contents()};
  } finally {
    sandbox.dispose();
  }
}

/** ⚖️ Reduces both application orders of a pair to its composability verdict. */
export function pairVerdict(a: string, b: string, ab: {ok: boolean; digests: Digests}, ba: {ok: boolean; digests: Digests}): {verdict: "commute" | "ordered" | "order-dependent" | "conflict"; first: string | null} {
  if (ab.ok && ba.ok) return equalDigests(ab.digests, ba.digests) ? {verdict: "commute", first: null} : {verdict: "order-dependent", first: null};
  if (ab.ok) return {verdict: "ordered", first: a};
  if (ba.ok) return {verdict: "ordered", first: b};
  return {verdict: "conflict", first: null};
}

/** 📍 Tells whether the anchors of two patch targets cover intersecting regions of the live file; null when an anchor cannot be located. */
export function regionOverlap(ctx: Context, left: Target, right: Target): boolean | null {
  const file = absolute(ctx.root, left.path);
  if (!existsSync(file)) return null;
  const text = readFileSync(file, "utf8");
  const spans = (item: Target): ([number, number] | null)[] =>
    item.ops.map((op) => {
      const at = text.indexOf(op.anchor);
      return at < 0 || text.indexOf(op.anchor, at + 1) >= 0 ? null : [at, at + op.anchor.length];
    });
  const a = spans(left), b = spans(right);
  if (a.some((x) => x && b.some((y) => y && x[0] < y[1] && y[0] < x[1]))) return true;
  return [...a, ...b].some((x) => x === null) ? null : false;
}

const stagedText = (info: SetInfo, item: Target): string | null => {
  const file = item.staged && (item.kind === "write" || item.kind === "create") ? join(info.dir, ...item.staged.split("/")) : null;
  return file && existsSync(file) && statSync(file).isFile() ? readFileSync(file, "utf8") : null;
};
const failingOp = (text: string, ops: Op[]): number => ops.findIndex((op) => text.split(op.anchor).length !== 2 && !text.includes(op.replacement));

/** 🩺 Explains per shared file why the patcher cannot be applied on top of what the writer leaves behind. */
export function composeDetails(writer: SetInfo, patcher: SetInfo): {path: string; detail: string}[] {
  const found: {path: string; detail: string}[] = [];
  for (const item of patcher.targets) {
    const names = [item.path, ...item.alsoTouches];
    const hits = writer.targets.filter((other) => names.includes(other.path));
    if (!hits.length) continue;
    if (item.kind === "patch") {
      const texts = hits.map((other) => stagedText(writer, other)).filter((text): text is string => text !== null);
      if (texts.some((text) => failingOp(text, item.ops) < 0)) continue;
      if (texts.length) {
        const index = failingOp(texts[texts.length - 1]!, item.ops);
        found.push({path: item.path, detail: `${patcher.id} op ${index} (anchor ${JSON.stringify(item.ops[index]!.anchor.slice(0, 60))}) is absent from ${writer.id}'s result${hits.some((other) => other.kind === "delete") ? " (file deleted)" : ""}`});
      } else if (hits.some((other) => other.kind === "delete")) found.push({path: item.path, detail: `${writer.id} deletes the file ${patcher.id} patches`});
    } else if (writer.id < patcher.id && (item.kind === "write" || item.kind === "create") && hits.some((other) => other.kind === "write" || other.kind === "create")) found.push({path: item.path, detail: `${writer.id} and ${patcher.id} both replace the file`});
    else if (hits.some((other) => other.kind === "delete") && item.kind !== "delete") found.push({path: item.path, detail: `${writer.id} deletes the file ${patcher.id} replaces`});
  }
  return found;
}

/** 🔬 Cross-checks a pair's merged file against the system three-way merge of the two solo results. */
export function diff3Check(ctx: Context, path: string, soloA: Buffer | null, soloB: Buffer | null, merged: Buffer | null): "agree" | "differs" | "conflict" | "skipped" | "unavailable" {
  const live = absolute(ctx.root, path);
  if (!soloA || !soloB || !merged || !existsSync(live)) return "skipped";
  const original = readFileSync(live);
  if (Buffer.compare(soloA, original) === 0 || Buffer.compare(soloB, original) === 0) return "skipped";
  mkdirSync(ctx.scratch, {recursive: true});
  const folder = mkdtempSync(join(ctx.scratch, "d3-"));
  try {
    writeFileSync(join(folder, "a"), soloA);
    writeFileSync(join(folder, "o"), original);
    writeFileSync(join(folder, "b"), soloB);
    const run = spawnSync("diff3", ["-m", join(folder, "a"), join(folder, "o"), join(folder, "b")], {maxBuffer: 1 << 28});
    if (run.error) return "unavailable";
    if (run.status === 0) return Buffer.compare(run.stdout, merged) === 0 ? "agree" : "differs";
    return run.status === 1 ? "conflict" : "unavailable";
  } finally {
    rmSync(folder, {recursive: true, force: true});
  }
}

/** 🧮 Orders ids so every hard edge (first, then) holds; soft hints are honoured unless they would close a cycle. */
export function orderSets(ids: string[], hard: [string, string][], soft: [string, string][]): {order: string[]; cycle: string[]} {
  const edges = [...hard];
  const reaches = (from: string, to: string, seen = new Set<string>()): boolean => from === to || (!seen.has(from) && (seen.add(from), edges.some(([a, b]) => a === from && reaches(b, to, seen))));
  for (const [a, b] of soft) if (ids.includes(a) && ids.includes(b) && !reaches(b, a)) edges.push([a, b]);
  const left = new Set(ids), order: string[] = [], cycle: string[] = [];
  while (left.size) {
    const free = ids.filter((id) => left.has(id) && !edges.some(([a, b]) => b === id && left.has(a)));
    const pick = free[0] ?? ids.find((id) => left.has(id))!;
    if (!free.length) cycle.push(pick);
    order.push(pick);
    left.delete(pick);
  }
  return {order, cycle};
}

const IMPORT = /(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s+|\brequire\(\s*)["'](\.{1,2}\/[^"'\n]*?\.(?:[cm]?[jt]sx?|json))["']|new URL\(\s*["'](\.{1,2}\/[^"'\n]+)["']\s*,\s*import\.meta\.url/gu;
const specifiers = (text: string): string[] => [...text.matchAll(IMPORT)].map((match) => (match[1] ?? match[2])!);

/** 🔗 Resolves the relative imports of every changed source file in the merged tree and separates regressions from existing breakage. */
export function checkImports(ctx: Context, tracked: string[], merged: Files): {checked: number; regressions: string[]; unresolved: string[]} {
  const known = new Set(tracked);
  const after = (path: string): boolean => (known.has(path) ? merged.get(path) != null : existsSync(absolute(ctx.root, path)));
  const before = (path: string): boolean => existsSync(absolute(ctx.root, path));
  const regressions: string[] = [], unresolved: string[] = [];
  let checked = 0;
  for (const [path, data] of merged) {
    if (!data || !/\.(?:[cm]?[jt]sx?)$/u.test(path)) continue;
    const live = absolute(ctx.root, path);
    const old = existsSync(live) ? readFileSync(live) : null;
    if (old && Buffer.compare(old, data) === 0) continue;
    const previous = new Set(old ? specifiers(old.toString("utf8")) : []);
    for (const spec of specifiers(data.toString("utf8"))) {
      checked++;
      const resolved = posix.normalize(posix.join(posix.dirname(path), spec.split("?")[0]!));
      const swapped = resolved.replace(/\.([cm]?)js$/u, ".$1ts");
      if (resolved.startsWith("..") || after(resolved) || (swapped !== resolved && after(swapped))) continue;
      if (!previous.has(spec)) unresolved.push(`${path} -> ${spec}`);
      else if (before(resolved) || (swapped !== resolved && before(swapped))) regressions.push(`${path} -> ${spec}`);
    }
  }
  return {checked, regressions: regressions.slice(0, 50), unresolved: unresolved.slice(0, 50)};
}

/** 🧪 Parses every changed source and JSON file of the merged tree with the runtime's own transpiler and parser. */
export function checkSyntax(ctx: Context, merged: Files): {checked: number; errors: string[]} {
  const errors: string[] = [];
  let checked = 0;
  const loaders: Record<string, "ts" | "tsx" | "js" | "jsx"> = {".ts": "ts", ".mts": "ts", ".cts": "ts", ".tsx": "tsx", ".js": "js", ".mjs": "js", ".cjs": "js", ".jsx": "jsx"};
  for (const [path, data] of merged) {
    if (!data) continue;
    const old = fileSha(absolute(ctx.root, path));
    if (old === sha(data)) continue;
    const extension = path.slice(path.lastIndexOf("."));
    try {
      if (extension === ".json") JSON.parse(data.toString("utf8"));
      else if (loaders[extension]) new Bun.Transpiler({loader: loaders[extension]!}).transformSync(data.toString("utf8"));
      else continue;
      checked++;
    } catch (error) {
      checked++;
      errors.push(`${path}: ${tail(String((error as Error).message), 1).slice(0, 160)}`);
    }
  }
  return {checked, errors: errors.slice(0, 50)};
}

type Plan = Json;

/** 🧱 Plans every ready set: inventory, solo run, pairwise composability, one ordered replay, import resolution. */
export function buildPlan(ctx: Context, options: {allPairs?: boolean; prefer?: string[]; rebase?: string[]} = {}): Plan {
  const rebase = options.rebase ?? [];
  const specs = loadRegistry(ctx);
  const infos = specs.map((spec) => describe(ctx, spec));
  const excluded: {id: string; reason: string; detail: string}[] = [];
  const soloRuns = new Map<string, Run>(), soloFiles = new Map<string, Files>();
  for (const info of infos) {
    if (info.state === "skipped") {
      excluded.push({id: info.id, reason: "not-ready", detail: info.note});
      continue;
    }
    const sandbox = new Sandbox(ctx, [info], touched(info));
    try {
      const run = attempt(sandbox, info, rebase.includes(info.id));
      soloRuns.set(info.id, run);
      soloFiles.set(info.id, sandbox.contents());
      if (run.status === "refused") excluded.push({id: info.id, reason: "blocked", detail: run.message ?? ""});
      if (run.status === "noop") excluded.push({id: info.id, reason: "already-applied", detail: "applying changes nothing on the current tree"});
    } finally {
      sandbox.dispose();
    }
  }
  const viable = infos.filter((info) => ["applied", "refused"].includes(soloRuns.get(info.id)?.status ?? ""));
  const pairs: Plan[] = [];
  for (let i = 0; i < viable.length; i++) {
    for (let j = i + 1; j < viable.length; j++) {
      const a = viable[i]!, b = viable[j]!;
      const own = new Set(touched(a));
      const shared = [...new Set(touched(b).filter((path) => own.has(path)))];
      if (!shared.length && !options.allPairs) continue;
      const paths = [...new Set([...touched(a), ...touched(b)])];
      const ab = orderRun(ctx, a, b, paths, rebase, infos), ba = orderRun(ctx, b, a, paths, rebase, infos);
      const verdict = shared.length || options.allPairs ? pairVerdict(a.id, b.id, ab, ba) : {verdict: "independent" as const, first: null};
      const kinds = (info: SetInfo, path: string): Target | undefined => info.targets.find((item) => item.path === path || item.alsoTouches.includes(path));
      const overlaps = shared.map((path) => (kinds(a, path)?.kind === "patch" && kinds(b, path)?.kind === "patch" ? regionOverlap(ctx, kinds(a, path)!, kinds(b, path)!) : true));
      const winner = verdict.verdict === "commute" || verdict.verdict === "ordered" ? (verdict.first === b.id ? ba : ab) : null;
      const checks = shared.filter(() => winner).map((path) => diff3Check(ctx, path, soloFiles.get(a.id)!.get(path) ?? null, soloFiles.get(b.id)!.get(path) ?? null, winner!.files.get(path) ?? null));
      const diff3 = !checks.length ? "skipped" : checks.includes("unavailable") ? "unavailable" : checks.includes("conflict") ? "conflict" : checks.includes("differs") ? "differs" : "agree";
      const downgraded = verdict.verdict === "commute" && diff3 === "conflict";
      const files = verdict.verdict === "commute" && !downgraded ? [] : [...composeDetails(a, b), ...composeDetails(b, a), ...(downgraded ? shared.map((path) => ({path, detail: "the system three-way merge of both solo results conflicts although the digests commute"})) : [])];
      const final = downgraded ? "conflict" : shared.length ? verdict.verdict : verdict.verdict === "commute" ? "independent" : verdict.verdict;
      pairs.push({a: a.id, b: b.id, shared, files, regionOverlap: overlaps.length ? (overlaps.includes(true) ? true : overlaps.includes(null) ? null : false) : null, verdict: final, first: downgraded ? null : verdict.first, ab: ab.steps, ba: ba.steps, diff3});
    }
  }
  const active = viable.filter((info) => soloRuns.get(info.id)!.status === "applied");
  const reachable = new Set(active.map((info) => info.id));
  for (let grown = true; grown; ) {
    grown = false;
    for (const row of pairs.filter((pair) => pair.verdict === "ordered")) {
      const later = row.first === row.a ? row.b : row.a;
      if (reachable.has(row.first) && !reachable.has(later)) {
        reachable.add(later);
        grown = true;
      }
    }
  }
  for (const id of reachable) {
    const at = excluded.findIndex((row) => row.id === id && row.reason === "blocked");
    if (at >= 0) excluded.splice(at, 1);
  }
  const prefer = options.prefer ?? [];
  const rank = (id: string): number => (prefer.includes(id) ? prefer.indexOf(id) - 1000 : infos.findIndex((info) => info.id === id));
  const kept: string[] = [];
  for (const info of viable.filter((row) => reachable.has(row.id)).sort((x, y) => rank(x.id) - rank(y.id))) {
    const clash = pairs.find((row) => row.verdict === "conflict" && ((row.a === info.id && kept.includes(row.b)) || (row.b === info.id && kept.includes(row.a))));
    if (!clash) {
      kept.push(info.id);
      continue;
    }
    const winner = clash.a === info.id ? clash.b : clash.a;
    const sample = (clash.files as {path: string; detail: string}[]).find((row) => row.detail.startsWith(info.id)) ?? clash.files[0];
    excluded.push({id: info.id, reason: "conflict", detail: `neither application order composes with ${winner} (${clash.shared.length} shared file(s), ${clash.files.length} incompatible${sample ? `, e.g. ${sample.detail}` : ""}); restage ${info.id} on top of ${winner}'s result, or plan with --prefer ${info.id}`});
  }
  const included = new Set(kept);
  let replay: Plan = {status: "empty", steps: [], changedFiles: 0, imports: {checked: 0, regressions: [], unresolved: []}};
  let order: string[] = [];
  let stepRuns = new Map<string, Run>();
  const satisfied = new Set(infos.filter((info) => soloRuns.get(info.id)?.status === "noop").map((info) => info.id));
  const closeAfter = (): void => {
    for (let again = true; again; ) {
      again = false;
      for (const info of infos) {
        const missing = included.has(info.id) ? (info.spec.after ?? []).filter((id) => !included.has(id) && !satisfied.has(id)) : [];
        if (!missing.length) continue;
        included.delete(info.id);
        excluded.push({id: info.id, reason: "predecessor-excluded", detail: `declares after ${missing.map((id) => `${id} (${excluded.find((row) => row.id === id)?.reason ?? "absent"})`).join(", ")}; a predecessor edge is a hard constraint, so ${info.id} waits until it is applicable`});
        again = true;
      }
    }
  };
  for (;;) {
    closeAfter();
    const members = infos.filter((info) => included.has(info.id));
    const hard = [
      ...pairs.filter((row) => row.verdict === "ordered" && included.has(row.a) && included.has(row.b)).map((row): [string, string] => (row.first === row.a ? [row.a, row.b] : [row.b, row.a])),
      ...members.flatMap((info) => (info.spec.after ?? []).filter((id) => included.has(id)).map((id): [string, string] => [id, info.id])),
    ];
    order = orderSets(members.map((info) => info.id), hard, []).order;
    if (!order.length) break;
    const byId = new Map(members.map((info) => [info.id, info]));
    const paths = [...new Set(members.flatMap(touched))];
    const sandbox = new Sandbox(ctx, members, paths);
    try {
      const steps: Run[] = [];
      for (const id of order) {
        const run = attempt(sandbox, byId.get(id)!, rebase.includes(id));
        steps.push(run);
        if (run.status === "refused") break;
      }
      const failed = steps.find((run) => run.status === "refused");
      if (failed) {
        included.delete(failed.id!);
        excluded.push({id: failed.id!, reason: "replay-refused", detail: failed.message ?? ""});
        continue;
      }
      const merged = sandbox.contents();
      const before = new Map(paths.map((path) => [path, fileSha(absolute(ctx.root, path))]));
      const changedFiles = [...merged].filter(([path, data]) => (data ? sha(data) : null) !== before.get(path)).length;
      stepRuns = new Map(steps.map((run) => [run.id!, run]));
      replay = {status: "ok", steps, changedFiles, undeclared: [...new Set(steps.flatMap((run) => run.undeclared ?? []))], imports: checkImports(ctx, paths, merged), syntax: checkSyntax(ctx, merged)};
    } finally {
      sandbox.dispose();
    }
    break;
  }
  const plannedSets = infos.map((info) => ({
    id: info.id,
    dir: info.spec.dir,
    state: info.state,
    note: [info.spec.note, info.note].filter(Boolean).join("; "),
    ...(info.state === "skipped" && !info.entry ? {} : {adapter: info.adapter, entry: info.entry}),
    result: info.result,
    fingerprint: info.fingerprint,
    targets: info.targets.map((item) => ({path: item.path, kind: item.kind, staged: item.staged, baseState: baseState(ctx, item), ...(item.alsoTouches.length ? {alsoTouches: item.alsoTouches} : {}), anchors: item.ops.map((op) => op.anchor.slice(0, 100))})),
    ...(soloRuns.has(info.id) ? {solo: soloRuns.get(info.id)} : {}),
  }));
  const steps = order.map((id, position) => ({
    index: position + 1,
    id,
    applyArgs: stepRuns.get(id)?.args ?? [],
    needs: [...new Set([...pairs.filter((row) => row.verdict === "ordered" && row.first !== id && (row.a === id || row.b === id) && included.has(row.a) && included.has(row.b) && order.indexOf(row.first!) < position).map((row) => row.first!), ...(infos.find((info) => info.id === id)!.spec.after ?? []).filter((before) => order.indexOf(before) >= 0 && order.indexOf(before) < position)])],
    provisional: infos.find((info) => info.id === id)!.state === "provisional",
    verify: infos.find((info) => info.id === id)!.spec.verify ?? [],
  }));
  const plan: Plan = JSON.parse(JSON.stringify({
    version: 1,
    generatedAt: new Date().toISOString(),
    prefer,
    rebase,
    inventoryFingerprint: inventoryFingerprint(ctx, infos),
    liveFingerprint: liveFingerprint(ctx, infos),
    sets: plannedSets,
    pairs,
    order: replay.status === "ok" ? steps : [],
    excluded,
    replay,
    unregistered: unregistered(ctx, specs),
  }));
  const schema = loadSchema(ctx);
  const errors = validate(plan, schema.$defs.Plan, schema);
  if (errors.length) throw new Error(`plan violates its schema:\n${errors.slice(0, 20).join("\n")}`);
  return plan;
}

/** 🔒 Fingerprints the registry and every set's staged inputs so a stale plan is detectable. */
export const inventoryFingerprint = (ctx: Context, infos: SetInfo[]): string => sha(JSON.stringify([fileSha(ctx.registry), infos.map((info) => [info.id, info.state, info.fingerprint])]));

/** 🌍 Fingerprints the live state of every file any set may touch. */
export const liveFingerprint = (ctx: Context, infos: SetInfo[]): string => sha(JSON.stringify([...new Set(infos.flatMap(touched))].sort().map((path) => [path, fileSha(absolute(ctx.root, path))])));

/** 🔎 Lists tool directories of the ticket that look like staged sets but are not in the registry. */
export function unregistered(ctx: Context, specs: Spec[]): string[] {
  const known = new Set(specs.map((spec) => spec.dir));
  return readdirSync(ctx.ticket, {withFileTypes: true})
    .filter((entry) => entry.isDirectory() && entry.name.endsWith("-tools") && !known.has(entry.name) && ["apply.sh", "apply.py", "apply.ts"].some((name) => existsSync(join(ctx.ticket, entry.name, name))) && ["staged", "bases.json", "patches.json", "inventory.json"].some((name) => existsSync(join(ctx.ticket, entry.name, name))))
    .map((entry) => entry.name)
    .sort();
}

/** 🧬 Replays the transitive predecessors of one set in a sandbox and returns the digest of every file that set targets on top of them; with a directory it also writes those files there so the set can be restaged on its real input. */
export function materialize(ctx: Context, id: string, out: string | null, rebase: string[] = []): Record<string, string | null> {
  const infos = loadRegistry(ctx).map((spec) => describe(ctx, spec));
  const byId = new Map(infos.map((info) => [info.id, info]));
  const target = byId.get(id);
  if (!target) throw new Error(`unknown set ${id}`);
  const needed = new Set<string>();
  const visit = (name: string): void => {
    for (const before of byId.get(name)?.spec.after ?? []) if (!needed.has(before)) (needed.add(before), visit(before));
  };
  visit(id);
  const members = infos.filter((info) => needed.has(info.id));
  const hard = members.flatMap((info) => (info.spec.after ?? []).filter((before) => needed.has(before)).map((before): [string, string] => [before, info.id]));
  const order = orderSets(members.map((info) => info.id), hard, []).order;
  const paths = [...new Set([...members.flatMap(touched), ...touched(target)])];
  const sandbox = new Sandbox(ctx, members, paths);
  try {
    for (const name of order) {
      const run = attempt(sandbox, byId.get(name)!, rebase.includes(name));
      if (run.status === "refused") throw new Error(`predecessor ${name} refused in the sandbox: ${run.message}`);
    }
    const files = sandbox.contents();
    const digests: Record<string, string | null> = {};
    const deleted: string[] = [];
    const own = new Set(touched(target));
    for (const path of new Set([...own, ...paths])) {
      const data = files.get(path) ?? null;
      const changed = (data ? sha(data) : null) !== fileSha(absolute(ctx.root, path));
      if (own.has(path)) digests[path] = data ? sha(data) : null;
      if (!changed && !own.has(path)) continue;
      if (!data && existsSync(absolute(ctx.root, path))) deleted.push(path);
      if (out && data) {
        mkdirSync(dirname(absolute(out, path)), {recursive: true});
        writeFileSync(absolute(out, path), data);
      }
    }
    if (out) {
      mkdirSync(out, {recursive: true});
      writeFileSync(join(out, ".materialized.json"), JSON.stringify({version: 1, set: id, deleted}));
    }
    return digests;
  } finally {
    sandbox.dispose();
  }
}

/** 🔄 Re-anchors one set on the real input it will see: replays its predecessors in a sandbox and runs the set's own restage command against that tree. */
export function restage(ctx: Context, id: string, rebase: string[] = []): number {
  const spec = loadRegistry(ctx).find((row) => row.id === id);
  if (!spec) throw new Error(`unknown set ${id}`);
  if (!spec.restage) throw new Error(`${id} declares no restage command in the registry`);
  mkdirSync(ctx.scratch, {recursive: true});
  const tree = mkdtempSync(join(ctx.scratch, "rs-"));
  try {
    materialize(ctx, id, tree, rebase);
    const argv = spec.restage.map((word) => word.replaceAll("{tree}", tree));
    const run = spawnSync(argv[0]!, argv.slice(1), {cwd: join(ctx.ticket, spec.dir), stdio: "inherit", env: {...process.env, PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1"}});
    return run.status ?? -1;
  } finally {
    if (!ctx.keep) rmSync(tree, {recursive: true, force: true});
  }
}

export type DriveOptions = {dryRun: boolean; yes: boolean; ignoreSync: boolean; includeProvisional: boolean; only: string[]; from: string | null; skipVerify: boolean; heavy: boolean; forceStale: boolean};

/** 🚦 Runs the planned sets in order: each set's own dry-run, then its apply, stopping at the first refusal; verification last. */
export function drive(ctx: Context, options: DriveOptions, out: (line: string) => void = console.log): number {
  if (!existsSync(ctx.planFile)) {
    out(`no ${ctx.planFile}; run: bun plan.ts plan`);
    return 4;
  }
  const plan: Plan = readJson(ctx.planFile);
  const schema = loadSchema(ctx);
  const errors = validate(plan, schema.$defs.Plan, schema);
  if (errors.length) {
    out(`plan.json violates its schema:\n${errors.slice(0, 10).join("\n")}`);
    return 4;
  }
  const specs = loadRegistry(ctx);
  const infos = specs.map((spec) => describe(ctx, spec));
  const stale = [inventoryFingerprint(ctx, infos) !== plan.inventoryFingerprint ? "staged inputs or registry" : null, !options.from && !options.only.length && liveFingerprint(ctx, infos) !== plan.liveFingerprint ? "live target files" : null].filter(Boolean);
  if (stale.length && !options.forceStale) {
    out(`plan.json is stale (${stale.join(", ")} changed since planning); re-run: bun plan.ts plan  (or pass --force-stale)`);
    if (!options.dryRun) return 4;
  }
  if (plan.replay.status !== "ok" && !options.forceStale) {
    out(`plan replay status is ${plan.replay.status}; nothing to apply`);
    return 4;
  }
  if (!options.dryRun && !options.yes) {
    out("refusing to modify the repository without --yes (the coordinator lifts the HARD FREEZE and stops the dependency sync first); use --dry-run to rehearse");
    return 6;
  }
  const fail = (code: number, id: string, text: string): number => {
    out(`STOP at ${id}: ${text}`);
    return code;
  };
  const byId = new Map(infos.map((info) => [info.id, info]));
  let steps: Plan[] = plan.order;
  if (options.only.length) steps = steps.filter((step) => options.only.includes(step.id));
  if (options.only.some((id) => !plan.order.some((step: Json) => step.id === id))) return fail(4, options.only.join(","), "not a step of the plan order (excluded, skipped or unknown)");
  if (options.from && !plan.order.some((step: Json) => step.id === options.from)) return fail(4, options.from, "--from is not a step of the plan order (excluded, skipped or unknown)");
  if (options.from) steps = steps.slice(steps.findIndex((step) => step.id === options.from));
  const skipped = steps.filter((step) => step.provisional && !options.includeProvisional);
  for (const step of skipped) out(`skip ${step.id}: provisional (no result report yet); pass --include-provisional to include it`);
  steps = steps.filter((step) => !skipped.includes(step));
  for (const [position, step] of steps.entries()) {
    const info = byId.get(step.id);
    for (const id of info?.spec.after ?? []) {
      const earlier = steps.slice(0, position).some((other) => other.id === id);
      const predecessor = byId.get(id);
      if (!earlier && !(predecessor && predecessor.targets.length && predecessor.targets.every((item) => baseState(ctx, item) === "applied"))) return fail(4, step.id, `declares after ${id}, which is neither an earlier selected step nor applied on the live tree; a predecessor edge is a hard constraint`);
    }
  }
  const touchedByPlan = (): string => liveFingerprint(ctx, steps.map((step) => byId.get(step.id)!).filter(Boolean));
  for (const step of steps) {
    const info = byId.get(step.id);
    if (!info?.entry) return fail(4, step.id, "set is no longer staged; re-run: bun plan.ts plan");
    const [command, ...rest] = info.entry;
    const flags = options.ignoreSync && info.spec.ignoreSync !== null ? [info.spec.ignoreSync ?? "--ignore-sync"] : [];
    const args = ["apply", ...flags, ...step.applyArgs];
    const live = touchedByPlan();
    const dry = spawnSync(command!, [...rest, ...args, "--dry-run"], {cwd: info.dir, encoding: "utf8", maxBuffer: 1 << 28, env: {...process.env, PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1"}});
    const text = `${dry.stdout ?? ""}${dry.stderr ?? ""}`;
    if (touchedByPlan() !== live) return fail(7, step.id, "its --dry-run changed files on the live tree; the contract is that --dry-run writes nothing (the tree may be partially patched, inspect git status)");
    if (dry.status !== 0) {
      if (dry.status === 2) return fail(2, step.id, `a dependency sync is running; ${tail(text, 3)}\n(pass --ignore-sync to rehearse anyway)`);
      if (options.dryRun && step.needs.some((id: string) => steps.some((other) => other.id === id))) {
        out(`defer ${step.id}: dry-run refused on the current tree because it needs ${step.needs.join(", ")} applied first (verified in the plan replay)`);
        continue;
      }
      return fail(3, step.id, `its own dry-run refused:\n${tail(text, 20)}`);
    }
    out(`${options.dryRun ? "dry-run ok " : "dry-run ok, applying "}${step.id}${step.applyArgs.length ? ` ${step.applyArgs.join(" ")}` : ""}\n${tail(text, 6).replace(/^/gmu, "    ")}`);
    if (!options.dryRun) {
      const real = spawnSync(command!, [...rest, ...args, ...(info.spec.confirm ? [info.spec.confirm] : [])], {cwd: info.dir, encoding: "utf8", maxBuffer: 1 << 28, env: {...process.env, PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1"}});
      const result = `${real.stdout ?? ""}${real.stderr ?? ""}`;
      if (real.status !== 0) return fail(3, step.id, `apply failed after a passing dry-run (the tree may be partially patched):\n${tail(result, 20)}`);
      out(tail(result, 6).replace(/^/gmu, "    "));
    }
  }
  if (options.dryRun && steps.length) {
    const members = steps.map((step) => byId.get(step.id)!);
    const sandbox = new Sandbox(ctx, members, [...new Set(members.flatMap(touched))]);
    try {
      for (const step of steps) {
        const run = attempt(sandbox, byId.get(step.id)!, step.applyArgs.includes("--rebase"));
        out(`sandbox replay ${step.id}: ${run.status}${run.changed !== undefined ? ` (${run.changed} file(s))` : ""}${run.message ? `\n${run.message.replace(/^/gmu, "    ")}` : ""}`);
        if (run.status === "refused") return fail(3, step.id, "the sandbox replay of the plan order refused this set");
      }
    } finally {
      sandbox.dispose();
    }
  }
  let failures = 0;
  for (const step of steps) {
    for (const item of step.verify as Verify[]) {
      const heavy = item.heavy === true;
      if (options.skipVerify || (heavy && !options.heavy)) {
        out(`verify ${step.id}/${item.name}: ${options.skipVerify ? "skipped (--skip-verify)" : "skipped (heavy; pass --heavy)"}`);
        continue;
      }
      if (options.dryRun) {
        out(`verify ${step.id}/${item.name}: would run ${item.argv.join(" ")}${item.gate === false ? " (informational)" : ""}`);
        continue;
      }
      const temporary = mkdtempSync(join(ctx.scratch, "vf-"));
      const argv = item.argv.map((word) => word.replaceAll("{root}", ctx.root).replaceAll("{set}", byId.get(step.id)!.dir).replaceAll("{tmp}", temporary));
      const artifacts = join(temporary, "art");
      mkdirSync(artifacts, {recursive: true});
      const run = spawnSync(argv[0]!, argv.slice(1), {cwd: ctx.root, encoding: "utf8", maxBuffer: 1 << 28, timeout: (item.timeoutSeconds ?? 600) * 1000, env: {...process.env, SEMIO_TEST_ARTIFACT_DIR: process.env.SEMIO_TEST_ARTIFACT_DIR ?? artifacts}});
      rmSync(temporary, {recursive: true, force: true});
      const ok = run.status === 0;
      out(`verify ${step.id}/${item.name}: ${ok ? "pass" : item.gate === false ? "informational non-zero" : "FAIL"} (rc ${run.status})\n${tail(`${run.stdout ?? ""}${run.stderr ?? ""}`, ok ? 3 : 20).replace(/^/gmu, "    ")}`);
      if (!ok && item.gate !== false) failures++;
    }
  }
  out(`${options.dryRun ? "rehearsal" : "applied"} ${steps.length} set(s): ${steps.map((step) => step.id).join(" -> ") || "none"}; verification failures: ${failures}`);
  return failures ? 5 : 0;
}

const flag = (args: string[], name: string): boolean => args.includes(name);
const value = (args: string[], name: string): string | null => (args.includes(name) ? (args[args.indexOf(name) + 1] ?? null) : null);

/** ⌨️ Command line: plan | check | inventory | apply. */
export function main(args: string[]): number {
  const ctx = defaultContext();
  ctx.keep = flag(args, "--keep");
  const command = args[0] ?? "plan";
  if (command === "plan") {
    const plan = buildPlan(ctx, {allPairs: flag(args, "--all-pairs"), prefer: (value(args, "--prefer") ?? "").split(",").filter(Boolean), rebase: (value(args, "--rebase-set") ?? "").split(",").filter(Boolean)});
    ctx.planFile = value(args, "--out") ?? ctx.planFile;
    writeFileSync(ctx.planFile, `${JSON.stringify(plan, null, 1)}\n`);
    const states = plan.sets.map((row: Json) => `${row.id}:${row.state}${row.solo ? `/${row.solo.status}` : ""}`).join(" ");
    console.log(`sets ${states}\norder ${plan.order.map((step: Json) => step.id).join(" -> ") || "(empty)"}\nreplay ${plan.replay.status}, ${plan.replay.changedFiles} changed file(s)\nexcluded ${plan.excluded.map((row: Json) => `${row.id}(${row.reason})`).join(" ") || "none"}\nunregistered ${plan.unregistered.join(" ") || "none"}\nwrote ${ctx.planFile}`);
    return plan.replay.status === "failed" ? 3 : 0;
  }
  if (command === "inventory") {
    for (const info of loadRegistry(ctx).map((spec) => describe(ctx, spec)).filter((row) => !args[1] || row.id === args[1])) {
      console.log(`${info.id} [${info.state}] ${info.adapter ?? "-"} ${info.note}`);
      for (const item of info.targets) console.log(`  ${item.kind.padEnd(6)} ${baseState(ctx, item).padEnd(8)} ${item.path} (${item.ops.length} anchors, staged ${item.staged ?? "-"})`);
    }
    return 0;
  }
  if (command === "restage") {
    if (!args[1]) {
      console.error("usage: bun plan.ts restage <set id> [--keep] [--rebase-set a,b]");
      return 64;
    }
    return restage(ctx, args[1], (value(args, "--rebase-set") ?? "").split(",").filter(Boolean));
  }
  if (command === "materialize") {
    if (!args[1]) {
      console.error("usage: bun plan.ts materialize <set id> [--out dir] [--rebase-set a,b]");
      return 64;
    }
    console.log(JSON.stringify(materialize(ctx, args[1], value(args, "--out"), (value(args, "--rebase-set") ?? "").split(",").filter(Boolean)), null, 1));
    return 0;
  }
  if (command === "check") {
    const infos = loadRegistry(ctx).map((spec) => describe(ctx, spec));
    const plan: Plan = existsSync(ctx.planFile) ? readJson(ctx.planFile) : null;
    console.log(plan ? `inventory ${inventoryFingerprint(ctx, infos) === plan.inventoryFingerprint ? "current" : "STALE"}, live ${liveFingerprint(ctx, infos) === plan.liveFingerprint ? "current" : "STALE"}` : "no plan.json");
    return 0;
  }
  if (command === "apply") {
    return drive(ctx, {
      dryRun: flag(args, "--dry-run"),
      yes: flag(args, "--yes"),
      ignoreSync: flag(args, "--ignore-sync"),
      includeProvisional: flag(args, "--include-provisional"),
      only: (value(args, "--only") ?? "").split(",").filter(Boolean),
      from: value(args, "--from"),
      skipVerify: flag(args, "--skip-verify"),
      heavy: flag(args, "--heavy"),
      forceStale: flag(args, "--force-stale"),
    });
  }
  console.error("usage: bun plan.ts plan [--all-pairs] [--keep] [--prefer a,b] [--rebase-set a,b] [--out file] | check | inventory [id] | materialize <id> [--out dir] | restage <id> [--keep] | apply [--dry-run|--yes] [--ignore-sync] [--include-provisional] [--only a,b] [--from id] [--skip-verify] [--heavy] [--force-stale]");
  return 64;
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
