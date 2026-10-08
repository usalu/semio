#!/usr/bin/env bun
/**
 * 🗂️ Launch inventory: classifies every `.vscode/launch.json` configuration, the seed that renders it and
 * `.claude/launch.json`, verifies Nx targets and referenced files, and writes
 *   - `🗑️generated/launch-inventory/inventory.json` (one record per configuration + summaries)
 *   - `launch-inventory.md` (the report; every number in it is computed here)
 *
 * Usage (from the repository root):
 *   bun <ticket>/launch-inventory.ts [--source=snapshot|live] [--verify-generator] [--skip-env-scan] [--snapshot-live]
 *
 * `--source=snapshot` (default when the snapshot exists) reads the copies under `🗑️generated/launch-inventory/snapshot/`
 * so that the numbers stay stable while other developers regenerate `.vscode/launch.json`; `--source=live` reads the
 * repository files; `--snapshot-live` refreshes the snapshot from the live files first.
 * Read-only on the repository: nothing outside this ticket folder is written.
 */
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { basename, dirname, join, resolve } from "node:path";

//#region paths & arguments
const TICKET_DIR = import.meta.dirname;
const ROOT = resolve(TICKET_DIR, "../../../../../../..");
if (!existsSync(join(ROOT, "nx.json"))) throw new Error(`launch-inventory: ${ROOT} is not the repository root`);
const GEN = join(TICKET_DIR, "🗑️generated", "launch-inventory");
const SNAP = join(GEN, "snapshot");
const argv = process.argv.slice(2);
const flag = (name: string): boolean => argv.includes(`--${name}`);
const option = (name: string): string | undefined => argv.find((a) => a.startsWith(`--${name}=`))?.slice(name.length + 3);
const LIVE_FILES = {
  launch: join(ROOT, ".vscode/launch.json"),
  seed: join(ROOT, ".vscode/🧩️launch.seed.jsonc"),
  claude: join(ROOT, ".claude/launch.json"),
  playgrounds: join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json"),
};
const SNAP_FILES = { launch: join(SNAP, "launch.json"), seed: join(SNAP, "launch.seed.jsonc"), claude: join(SNAP, "claude-launch.json"), playgrounds: join(SNAP, "playgrounds.json") };
mkdirSync(SNAP, { recursive: true });
if (flag("snapshot-live")) for (const key of Object.keys(LIVE_FILES) as (keyof typeof LIVE_FILES)[]) copyFileSync(LIVE_FILES[key], SNAP_FILES[key]);
const SOURCE = option("source") ?? (existsSync(SNAP_FILES.launch) ? "snapshot" : "live");
const FILES = SOURCE === "live" ? LIVE_FILES : SNAP_FILES;
const sha = (path: string): string => createHash("sha256").update(readFileSync(path)).digest("hex");
const TIMING: Record<string, number> = {};
const timed = <T>(label: string, work: () => T): T => { const t = Date.now(); const r = work(); TIMING[label] = Date.now() - t; return r; };
//#endregion

//#region JSONC
type Json = any;
const readText = (path: string): string => readFileSync(path, "utf8");
const parseJsonc = (text: string): Json => Bun.JSONC.parse(text);
/** 🔎️ Counts comments and trailing commas outside strings, so the report can say what the JSONC parser had to tolerate. */
function scanJsonc(text: string): { lineComments: number; blockComments: number; trailingCommas: number; bytes: number } {
  let lineComments = 0, blockComments = 0, trailingCommas = 0, i = 0;
  const n = text.length;
  while (i < n) {
    const c = text[i];
    if (c === '"') { i++; while (i < n && text[i] !== '"') i += text[i] === "\\" ? 2 : 1; i++; continue; }
    if (c === "/" && text[i + 1] === "/") { lineComments++; while (i < n && text[i] !== "\n") i++; continue; }
    if (c === "/" && text[i + 1] === "*") { blockComments++; i = text.indexOf("*/", i + 2) + 2 || n; continue; }
    if (c === ",") { let j = i + 1; while (j < n && /\s/.test(text[j]!)) j++; if (text[j] === "}" || text[j] === "]") trailingCommas++; }
    i++;
  }
  return { lineComments, blockComments, trailingCommas, bytes: Buffer.byteLength(text) };
}
//#endregion

//#region generic helpers
const tally = <T>(items: readonly T[], key: (item: T) => string): [string, number][] => { const m = new Map<string, number>(); for (const i of items) m.set(key(i), (m.get(key(i)) ?? 0) + 1); return [...m].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0])); };
const tallyObj = <T>(items: readonly T[], key: (item: T) => string): Record<string, number> => Object.fromEntries(tally(items, key));
const uniq = <T>(items: readonly T[]): T[] => [...new Set(items)];
const sample = <T>(items: readonly T[], n = 3): T[] => items.slice(0, n);
//#endregion

//#region shell tokenizer & command parser
type Tok = { text: string; op: boolean };
const OPERATORS = ["&&", "||", ">>", "|", ";", ">", "<", "&"];
export function tokenize(source: string): Tok[] {
  const out: Tok[] = [];
  let i = 0;
  const n = source.length;
  while (i < n) {
    while (i < n && /\s/.test(source[i]!)) i++;
    if (i >= n) break;
    const operator = OPERATORS.find((o) => source.startsWith(o, i));
    if (operator) { out.push({ text: operator, op: true }); i += operator.length; continue; }
    let text = "";
    while (i < n && !/\s/.test(source[i]!) && !OPERATORS.some((o) => source.startsWith(o, i))) {
      const c = source[i]!;
      if (c === "'") { const end = source.indexOf("'", i + 1); const stop = end === -1 ? n : end; text += source.slice(i + 1, stop); i = stop + 1; }
      else if (c === '"') { i++; while (i < n && source[i] !== '"') { if (source[i] === "\\" && i + 1 < n && '"\\$`'.includes(source[i + 1]!)) i++; text += source[i]; i++; } i++; }
      else if (c === "\\" && i + 1 < n) { text += source[i + 1]; i += 2; }
      else { text += c; i++; }
    }
    out.push({ text, op: false });
  }
  return out;
}

/** 🚩️ Nx CLI flags that are not arguments of the target (they steer Nx itself). */
const NX_FLAG_NAMES = new Set(["--skip-nx-cache", "--skipNxCache", "--skip-remote-cache", "--excludeTaskDependencies", "--exclude-task-dependencies", "--output-style", "--outputStyle", "--verbose", "--parallel", "--configuration", "-c", "--nxBail", "--batch", "--tui", "--no-tui", "--graph", "--cloud", "--no-cloud", "--runner", "--dte", "--exclude", "--projects", "--targets", "--watch"]);
const NX_FLAG_WITH_VALUE = new Set(["--configuration", "-c", "--output-style", "--outputStyle", "--parallel", "--runner", "--exclude", "--projects", "--targets"]);

export type Wrapper = "bun-nx" | "bun-x-nx" | "direct-nx-js";
export type ParsedCommand = {
  inlineEnv: Record<string, string>;
  argv: string[];
  operators: string[];
  kind: "nx-run" | "nx-exec" | "nx-other" | "bun-script" | "bun-test" | "bun-x" | "bun-other" | "external" | "env-only";
  wrapper?: Wrapper;
  nxSub?: string;
  spec?: string;
  project?: string;
  projectInput?: string;
  target?: string;
  nxConfiguration?: string;
  nxFlags: string[];
  targetArgs: string[];
  passthru: string[];
  execProjects?: string;
  execCommand?: string[];
  script?: string;
  scriptArgs?: string[];
  tool?: string;
  argsViaFlag?: boolean;
};
export function parseCommand(command: string): ParsedCommand {
  const tokens = tokenize(command);
  const operators = tokens.filter((t) => t.op).map((t) => t.text);
  const words = tokens.filter((t) => !t.op).map((t) => t.text);
  const inlineEnv: Record<string, string> = {};
  let at = 0;
  while (at < words.length && /^[A-Za-z_][A-Za-z0-9_]*=/.test(words[at]!)) { const w = words[at]!; inlineEnv[w.slice(0, w.indexOf("="))] = w.slice(w.indexOf("=") + 1); at++; }
  const rest = words.slice(at);
  const result: ParsedCommand = { inlineEnv, argv: rest, operators, kind: "external", nxFlags: [], targetArgs: [], passthru: [] };
  if (rest.length === 0) { result.kind = "env-only"; return result; }
  const head = rest[0]!;
  if (head !== "bun") { result.kind = "external"; result.tool = head; return result; }
  const second = rest[1] ?? "";
  let nxAt = -1;
  if (second === "nx") { result.wrapper = "bun-nx"; nxAt = 1; }
  else if (second === "x" && rest[2] === "nx") { result.wrapper = "bun-x-nx"; nxAt = 2; }
  else if (/nx\/dist\/bin\/nx\.js$/.test(second)) { result.wrapper = "direct-nx-js"; nxAt = 1; }
  if (nxAt === -1) {
    if (second === "x") { result.kind = "bun-x"; result.tool = rest[2]; result.scriptArgs = rest.slice(3); }
    else if (second === "test") { result.kind = "bun-test"; result.scriptArgs = rest.slice(2); }
    else if (/\.(ts|tsx|js|mjs|cjs|py)$/.test(second)) { result.kind = "bun-script"; result.script = second; result.scriptArgs = rest.slice(2); }
    else { result.kind = "bun-other"; result.scriptArgs = rest.slice(1); }
    return result;
  }
  const sub = rest[nxAt + 1] ?? "";
  result.nxSub = sub;
  const tail = rest.slice(nxAt + 2);
  const dashAt = tail.indexOf("--");
  const before = dashAt === -1 ? tail : tail.slice(0, dashAt);
  const after = dashAt === -1 ? [] : tail.slice(dashAt + 1);
  const split = (list: string[], from: number): void => {
    for (let k = from; k < list.length; k++) {
      const a = list[k]!;
      const name = a.includes("=") ? a.slice(0, a.indexOf("=")) : a;
      if (NX_FLAG_NAMES.has(name)) {
        result.nxFlags.push(a);
        if (!a.includes("=") && NX_FLAG_WITH_VALUE.has(name) && list[k + 1] !== undefined && !list[k + 1]!.startsWith("-")) result.nxFlags.push(list[++k]!);
      } else result.targetArgs.push(a);
    }
  };
  if (sub === "run") {
    result.kind = "nx-run";
    const spec = before[0] ?? "";
    result.spec = spec;
    const m = spec.match(/^(\$\{input:[^}]+\}|[^:]+):([^:]+)(?::(.+))?$/);
    if (m) {
      if (m[1]!.startsWith("${input:")) result.projectInput = m[1]!.slice(8, -1); else result.project = m[1];
      result.target = m[2];
      result.nxConfiguration = m[3];
    }
    split(before, 1);
    result.passthru = after;
    const expanded: string[] = [];
    result.targetArgs = result.targetArgs.filter((a) => { if (!a.startsWith("--args=")) return true; expanded.push(...tokenize(a.slice("--args=".length)).filter((t) => !t.op).map((t) => t.text)); return false; });
    if (expanded.length) { result.argsViaFlag = true; result.passthru = [...expanded, ...result.passthru]; }
  } else if (sub === "exec") {
    result.kind = "nx-exec";
    split(before, 0);
    const projects = result.nxFlags.find((f) => f.startsWith("--projects="));
    result.execProjects = projects?.slice("--projects=".length);
    result.execCommand = after;
  } else { result.kind = "nx-other"; split(before, 0); result.passthru = after; }
  return result;
}
//#endregion

//#region sources
const rawTexts = { launch: readText(FILES.launch), seed: readText(FILES.seed), claude: readText(FILES.claude), playgrounds: readText(FILES.playgrounds) };
const LAUNCH: Json = parseJsonc(rawTexts.launch);
const SEED: Json = parseJsonc(rawTexts.seed);
const CLAUDE: Json = parseJsonc(rawTexts.claude);
const PLAYGROUNDS: Json[] = JSON.parse(rawTexts.playgrounds);
const SOURCE_META = Object.fromEntries((Object.keys(FILES) as (keyof typeof FILES)[]).map((k) => [k, { path: FILES[k].replace(ROOT + "/", ""), sha256: sha(FILES[k]), jsonc: scanJsonc(rawTexts[k]) }]));
const LIVE_SHA = Object.fromEntries((Object.keys(LIVE_FILES) as (keyof typeof LIVE_FILES)[]).map((k) => [k, existsSync(LIVE_FILES[k]) ? sha(LIVE_FILES[k]) : null]));
const driftFromLive = Object.fromEntries((Object.keys(FILES) as (keyof typeof FILES)[]).map((k) => [k, LIVE_SHA[k] === SOURCE_META[k]!.sha256]));
//#endregion

//#region ticket references
const TICKET_ROOT = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets");
const TICKET_REF = /tickets\/([^\/\s'"]+)\/([^\/\s'"]+)\/([^\/\s'"]+)\/([^\/\s'"]+)/g;
const digits = (s: string): string => s.match(/\d+/)?.[0] ?? s;
export type TicketRef = { id: string; raw: string[] };
export function ticketRefsIn(text: string): TicketRef[] {
  const out = new Map<string, TicketRef>();
  for (const m of text.matchAll(TICKET_REF)) {
    const id = `${digits(m[1]!)}/${digits(m[2]!)}/${digits(m[3]!)}/${m[4]}`;
    if (!out.has(id)) out.set(id, { id, raw: [m[1]!, m[2]!, m[3]!, m[4]!] });
  }
  return [...out.values()];
}
const ticketInfoCache = new Map<string, { id: string; dir: string | null; exists: boolean; status: string; title?: string; goal?: string; ticketJson: string | null }>();
export function ticketInfo(ref: TicketRef): ReturnType<typeof ticketInfoCache.get> & {} {
  const hit = ticketInfoCache.get(ref.id);
  if (hit) return hit;
  let dir: string | null = join(TICKET_ROOT, ...ref.raw);
  if (!existsSync(dir)) {
    dir = null;
    let level = TICKET_ROOT;
    const want = [digits(ref.raw[0]!), digits(ref.raw[1]!), digits(ref.raw[2]!), ref.raw[3]!];
    for (let k = 0; k < 4; k++) {
      const next = existsSync(level) ? readdirSync(level).find((e) => (k < 3 ? digits(e) === want[k] && !/^[A-Za-z]/.test(e) : e === want[k])) : undefined;
      if (!next) { level = ""; break; }
      level = join(level, next);
    }
    if (level) dir = level;
  }
  const info = { id: ref.id, dir: dir ? dir.replace(ROOT + "/", "") : null, exists: !!dir, status: "missing", title: undefined as string | undefined, goal: undefined as string | undefined, ticketJson: null as string | null };
  if (dir) {
    const file = readdirSync(dir).find((e) => /ticket\.json$/.test(e));
    if (file) {
      info.ticketJson = join(dir, file).replace(ROOT + "/", "");
      try { const j = JSON.parse(readText(join(dir, file))); info.status = String(j.status ?? "unknown"); info.title = j.title; info.goal = j.goal; } catch { info.status = "unreadable"; }
    } else info.status = "no-ticket-json";
  }
  ticketInfoCache.set(ref.id, info);
  return info;
}
//#endregion

//#region seed analysis & provenance
const seedItems: Json[] = SEED.configurations;
const seedObjects = seedItems.filter((x) => typeof x === "object" && x !== null);
const seedPlaceholders = seedItems.filter((x): x is string => typeof x === "string");
const placeholderParts = seedPlaceholders.map((p) => { const m = p.match(/^@generated:([^:]+):([^:]+)$/)!; return { variant: m[1]!, kind: m[2]! }; });
const devLaunchers: Record<string, Json> = SEED.devLaunchers ?? {};
const playgroundByVariant = new Map(PLAYGROUNDS.map((p) => [p.variant as string, p]));
export type Provenance = "seed-authored" | "playground-seed-placeholder" | "playground-synthesized" | "project-launcher-family" | "project-launcher-target";
type Prov = { provenance: Provenance; seedIndex?: number; placeholder?: string };
const provenanceByIndex: Prov[] = [];
const walkWarnings: string[] = [];
let BEFORE_PROJECT = 0;
{
  let pos = 0;
  seedItems.forEach((item, seedIndex) => {
    if (typeof item === "string") {
      const [, variant, kind] = item.match(/^@generated:([^:]+):([^:]+)$/)!;
      const launcher = devLaunchers[variant!];
      const playground = playgroundByVariant.get(variant!);
      const count = kind === "users" ? (playground?.userPorts ? playground.userPorts.react.length + (launcher?.wgpuOrder !== undefined ? playground.userPorts.wgpu.length : 0) : 0) : 1;
      for (let k = 0; k < count; k++) provenanceByIndex[pos++] = { provenance: "playground-seed-placeholder", seedIndex, placeholder: item };
    } else provenanceByIndex[pos++] = { provenance: "seed-authored", seedIndex };
  });
  const rows: Json[] = LAUNCH.configurations;
  const isDevRow = (r: Json): boolean => !!r.serverReadyAction && /^bun nx run workspace:dev -- \S+$/.test(r.command) && Object.keys(r).join(",") === "name,type,request,command,cwd,env,presentation,serverReadyAction";
  while (pos < rows.length && isDevRow(rows[pos]!)) provenanceByIndex[pos++] = { provenance: "playground-synthesized" };
  BEFORE_PROJECT = pos;
  for (; pos < rows.length; pos++) provenanceByIndex[pos] = { provenance: String(rows[pos]!.command).includes("${input:projectTarget.") ? "project-launcher-family" : "project-launcher-target" };
  if (provenanceByIndex.length !== rows.length) walkWarnings.push(`provenance walk length ${provenanceByIndex.length} != ${rows.length}`);
}
//#endregion

//#region project catalog (declared manifests, Cargo packages, cached graph, inference rules)
type TargetInfo = { configurations: string[]; command?: string };
export type Proj = { name: string; root: string; source: "manifest" | "plain-project-json" | "cargo-package" | "cached-graph"; manifestPath?: string; targets: Map<string, TargetInfo>; cargo?: { package: string; component: boolean }; printDocuments?: Set<string> };
const SKIP_DIRS = new Set(["node_modules", ".git", "target", "dist", "temp", "🤖️generated", "🗑️generated", "storybook-static", "pkg", ".nx"]);
const MANIFEST_NAMES = new Set(["📋️project.json", "project.json"]);
type WalkResult = { projects: Map<string, Proj>; duplicates: string[]; directories: number; manifests: number; plainProjectJson: number; cargoOnly: number; invalid: string[] };
function readCargoFacts(dir: string): { package: string; component: boolean } | undefined {
  const file = join(dir, "Cargo.toml");
  if (!existsSync(file)) return undefined;
  const text = readText(file);
  const section = text.match(/^\[package\]\s*$([\s\S]*?)(?=^\[|(?![\s\S]))/m);
  const name = section?.[1]?.match(/^\s*name\s*=\s*"([^"]+)"/m)?.[1];
  if (!name) return undefined;
  return { package: name, component: /component-kind\s*=\s*"(?:plugin|extension)"/.test(text) && /^\[package\.metadata\.component\]/m.test(text) };
}
/** 🚶️ Walks one workspace root like the dashboard/generator do (`📋️project.json`), additionally reading plain `project.json` and Cargo packages. */
function walkWorkspace(rootAbs: string, skipHidden: boolean): WalkResult {
  const result: WalkResult = { projects: new Map(), duplicates: [], directories: 0, manifests: 0, plainProjectJson: 0, cargoOnly: 0, invalid: [] };
  const stack = [rootAbs];
  const cargoDirs: string[] = [];
  const manifestDirs = new Set<string>();
  while (stack.length) {
    const dir = stack.pop()!;
    let entries;
    try { entries = readdirSync(dir, { withFileTypes: true }); } catch { continue; }
    result.directories++;
    for (const e of entries) {
      if (e.isDirectory() && !e.isSymbolicLink()) { if (!SKIP_DIRS.has(e.name) && !(skipHidden && e.name.startsWith("."))) stack.push(join(dir, e.name)); continue; }
      if (!e.isFile()) continue;
      if (e.name === "Cargo.toml") cargoDirs.push(dir);
      if (!MANIFEST_NAMES.has(e.name)) continue;
      const path = join(dir, e.name);
      let json: Json;
      try { json = JSON.parse(readText(path)); } catch { result.invalid.push(path.replace(ROOT + "/", "")); continue; }
      if (!json?.name) continue;
      const plain = e.name === "project.json";
      if (plain) result.plainProjectJson++; else result.manifests++;
      const targets = new Map<string, TargetInfo>();
      for (const [tn, tv] of Object.entries<Json>(json.targets ?? {})) targets.set(tn, { configurations: Object.keys(tv?.configurations ?? {}), command: typeof tv?.options?.command === "string" ? tv.options.command : undefined });
      const root = dir === rootAbs ? "." : dir.slice(rootAbs.length + 1);
      if (result.projects.has(json.name)) result.duplicates.push(json.name);
      const proj: Proj = { name: json.name, root, source: plain ? "plain-project-json" : "manifest", manifestPath: path.replace(ROOT + "/", ""), targets, cargo: undefined };
      const catalogPath = json.metadata?.printCatalog;
      if (typeof catalogPath === "string" && existsSync(join(rootAbs, catalogPath))) {
        try { const cat = JSON.parse(readText(join(rootAbs, catalogPath))); proj.printDocuments = new Set<string>((cat.documents ?? []).flatMap((d: Json) => [`build-${d.id}`, `watch-${d.id}`])); proj.printDocuments.add("build"); proj.printDocuments.add("build-viz"); } catch {}
      }
      result.projects.set(json.name, proj);
      manifestDirs.add(dir);
    }
  }
  for (const dir of cargoDirs) {
    const facts = readCargoFacts(dir);
    if (!facts) continue;
    const owner = [...result.projects.values()].find((p) => join(rootAbs, p.root) === dir);
    if (owner) { owner.cargo = facts; continue; }
    if (!result.projects.has(facts.package)) {
      result.cargoOnly++;
      result.projects.set(facts.package, { name: facts.package, root: dir === rootAbs ? "." : dir.slice(rootAbs.length + 1), source: "cargo-package", targets: new Map(), cargo: facts });
    }
  }
  return result;
}
const FACTS_PATH = join(SNAP, "nx-facts.json");
type SerializedProj = { name: string; root: string; source: Proj["source"]; manifestPath?: string; targets: [string, TargetInfo][]; cargo?: Proj["cargo"]; printDocuments?: string[] };
const serializeProj = (p: Proj): SerializedProj => ({ name: p.name, root: p.root, source: p.source, manifestPath: p.manifestPath, targets: [...p.targets], cargo: p.cargo, printDocuments: p.printDocuments ? [...p.printDocuments] : undefined });
const reviveProj = (p: SerializedProj): Proj => ({ name: p.name, root: p.root, source: p.source, manifestPath: p.manifestPath, targets: new Map(p.targets), cargo: p.cargo, printDocuments: p.printDocuments ? new Set(p.printDocuments) : undefined });
/** 🧊️ Nx facts (manifest walk, root router commands, cached graph) are frozen with the snapshot so that the report does not drift while other developers edit manifests or Nx republishes its graph. Path existence and ticket status stay live checks. */
const SNAPSHOT_FACTS: Json | null = SOURCE === "snapshot" && !flag("snapshot-live") && existsSync(FACTS_PATH) ? JSON.parse(readText(FACTS_PATH)) : null;
const MAIN: WalkResult = SNAPSHOT_FACTS ? { projects: new Map<string, Proj>(SNAPSHOT_FACTS.main.projects.map((p: SerializedProj) => [p.name, reviveProj(p)])), duplicates: SNAPSHOT_FACTS.main.duplicates, directories: SNAPSHOT_FACTS.main.directories, manifests: SNAPSHOT_FACTS.main.manifests, plainProjectJson: SNAPSHOT_FACTS.main.plainProjectJson, cargoOnly: SNAPSHOT_FACTS.main.cargoOnly, invalid: SNAPSHOT_FACTS.main.invalid } : timed("walk-main-workspace", () => walkWorkspace(ROOT, true));
const ROOT_REGISTERED = new Set<string>(SNAPSHOT_FACTS ? SNAPSHOT_FACTS.rootRegistered : [...readText(join(ROOT, "📜️script.ts")).matchAll(/\.register\(\s*"([^"]+)"/g)].map((m) => m[1]!));
const GRAPH_PATHS = [join(ROOT, ".nx/workspace-data/project-graph.json"), join(ROOT, ".nx/workspace-data/dashboard/project-graph.json")];
type GraphCopy = { file: string; mtime: string; computedAt: string | null; nodes: Map<string, Proj> };
const GRAPH_COPIES: GraphCopy[] = SNAPSHOT_FACTS ? SNAPSHOT_FACTS.graphCopies.map((c: Json) => ({ file: c.file, mtime: c.mtime, computedAt: c.computedAt, nodes: new Map<string, Proj>(c.nodes.map((n: SerializedProj) => [n.name, reviveProj(n)])) })) : timed("parse-cached-graphs", () => GRAPH_PATHS.filter((p) => existsSync(p)).map((path) => {
  const g = JSON.parse(readText(path));
  const nodes = new Map<string, Proj>();
  for (const [name, n] of Object.entries<Json>(g.nodes ?? {})) nodes.set(name, { name, root: n.data.root, source: "cached-graph", targets: new Map(Object.entries<Json>(n.data.targets ?? {}).map(([tn, tv]) => [tn, { configurations: Object.keys(tv?.configurations ?? {}) }])) });
  return { file: path.replace(ROOT + "/", ""), mtime: statSync(path).mtime.toISOString(), computedAt: g.computedAt ? new Date(g.computedAt).toISOString() : null, nodes };
}));
if (!SNAPSHOT_FACTS && (SOURCE === "snapshot" || flag("snapshot-live"))) writeFileSync(FACTS_PATH, JSON.stringify({ frozenAt: new Date().toISOString(), main: { projects: [...MAIN.projects.values()].map(serializeProj), duplicates: MAIN.duplicates, directories: MAIN.directories, manifests: MAIN.manifests, plainProjectJson: MAIN.plainProjectJson, cargoOnly: MAIN.cargoOnly, invalid: MAIN.invalid }, rootRegistered: [...ROOT_REGISTERED], graphCopies: GRAPH_COPIES.map((c) => ({ file: c.file, mtime: c.mtime, computedAt: c.computedAt, nodes: [...c.nodes.values()].map(serializeProj) })) }));
const GRAPH: GraphCopy = GRAPH_COPIES.reduce((best, copy) => ((copy.computedAt ?? "") > (best.computedAt ?? "") ? copy : best), GRAPH_COPIES[0] ?? { file: "(none)", mtime: "", computedAt: null, nodes: new Map<string, Proj>() });
const PLAYGROUND_VARIANTS = PLAYGROUNDS.map((p) => p.variant as string);
const PLAYGROUND_TARGETS = new Set<string>(["build-all-playground-cdn-sites"]);
for (const v of PLAYGROUND_VARIANTS) {
  for (const n of [`session-${v}`, `runtime-input-admission-${v}`, `build-${v}-react-release`, `build-${v}-site`]) PLAYGROUND_TARGETS.add(n);
  for (const profile of ["dev", "release"]) {
    for (const engine of ["react", "wgpu"]) for (const op of ["prepare", "activate", "serve", "dev"]) PLAYGROUND_TARGETS.add(`${op}-${v}-${engine}-${profile}`);
    for (const op of ["prepare", "run", "smoke"]) PLAYGROUND_TARGETS.add(`${op}-${v}-native-${profile}`);
    for (const transport of ["stdio", "http"]) PLAYGROUND_TARGETS.add(`mcp-${v}-${transport}-${profile}`);
  }
}
const PLAYGROUND_OWNER_PROJECTS = new Set(["@semio-tech/plugin-registry", "@semio-tech/framework-os-dev"]);
/** 📐️ Which inference rule of `📚️library/🟨️.mjs` would create `target` on `project`; undefined when none can be shown. */
export function inferenceRule(p: Proj, target: string): string | undefined {
  if (p.name === "workspace" && ROOT_REGISTERED.has(target)) return "root-router-register";
  if (p.cargo && ["build", "check", "test"].includes(target)) return "cargo-target";
  if (p.cargo?.component && (target === "describe" || /^component-(?:dev|release)$/.test(target) || /^materialize-(?:dev|release)$/.test(target))) return "component-target";
  const base = p.targets.get("test");
  if (base?.command && !base.command.includes("⚡️caching/🦀️cargo/📜️script.ts") && /^test-(?:quick|long|exhaustive)$/.test(target)) return "leveled-test";
  if (p.printDocuments?.has(target)) return "print-document";
  if (PLAYGROUND_OWNER_PROJECTS.has(p.name) && PLAYGROUND_TARGETS.has(target)) return "playground-target";
  if (PLAYGROUND_TARGETS.has(target) && PLAYGROUNDS.some((pg) => pg.cratePath === p.root)) return "playground-site";
  return undefined;
}
const ticketWorkspaceCache = new Map<string, WalkResult | null>();
function ticketWorkspace(rootAbs: string): WalkResult | null {
  if (!ticketWorkspaceCache.has(rootAbs)) ticketWorkspaceCache.set(rootAbs, existsSync(rootAbs) ? walkWorkspace(rootAbs, false) : null);
  return ticketWorkspaceCache.get(rootAbs)!;
}
//#endregion

//#region env classification
/** 🔇️ Environment that only exists to steer Nx/Node/VS Code and changes nothing the dev asked for. */
export const NOISE_ENV = new Set(["NX_DAEMON", "NX_ISOLATE_PLUGINS", "NX_CACHE_PROJECT_GRAPH", "NX_TUI", "FORCE_COLOR"]);
/** 🏝️ Private Nx workspace/state locations used to run a ticket-local Nx workspace (isolation, not a dev-facing option). */
export const SANDBOX_ENV = new Set(["NX_WORKSPACE_ROOT_PATH", "NX_WORKSPACE_DATA_DIRECTORY", "NX_CACHE_DIRECTORY"]);
/** 📤️ Output locations: need not exist before the run. */
export const OUTPUT_ENV = new Set(["NX_CACHE_DIRECTORY", "NX_WORKSPACE_DATA_DIRECTORY", "SEMIO_TEST_ARTIFACT_DIR", "SEMIO_TEST_ARTIFACTS_DIR", "SEMIO_TEST_ARTIFACTS_ROOT", "CARGO_TARGET_DIR", "CARGO_BUILD_BUILD_DIR", "OS_HUB_DATA", "S_DATA_DIR", "PLAYWRIGHT_BROWSERS_PATH", "SEMIO_TEST_OUTPUT_SCOPE"]);
/** 🧭️ Cross-cutting axes: the same variable steers many unrelated targets. */
export const AXIS_OF_ENV: Record<string, string> = {
  SEMIO_TEST_LEVEL: "test-level", NEXTEST_SUCCESS_OUTPUT: "nextest-output", CARGO_BUILD_JOBS: "cargo-jobs",
  SEMIO_BUILD_MODE: "build-mode", NX_SKIP_NX_CACHE: "cache-policy", NX_SKIP_REMOTE_CACHE: "cache-policy", SEMIO_BUILD_BUDGET_MS: "build-budget",
  SEMIO_RENDERER: "renderer", SEMIO_PLUGIN: "playground", SEMIO_APP: "playground", S_OS_PORT: "port",
  SEMIO_DEFAULT_EXAMPLE: "example", PLAYGROUND_LOCKED_EXAMPLE_ID: "example", SEMIO_APP_ROLE: "app-role",
  PRINT_NATIVE_GRAMMAR_PHASE: "print-phase", PRINT_NATIVE_GEO_PALETTE_PHASE: "print-phase", PRINT_NATIVE_GEO_PLANAR_PHASE: "print-phase", PRINT_NATIVE_GEO_PLANAR_SOURCE: "print-phase", PRINT_NATIVE_DIAGRAM_FAMILY: "print-phase",
  SEMIO_TEST_ARTIFACT_DIR: "artifact-dir", SEMIO_TEST_ARTIFACTS_DIR: "artifact-dir", SEMIO_TEST_ARTIFACTS_ROOT: "artifact-dir",
};
export const isPortVar = (key: string): boolean => key === "S_OS_PORT" || /(?:^|_)PORT$/.test(key);
//#endregion

//#region row analysis
export type Ref = { role: string; raw: string; path: string; exists: boolean; required: boolean };
export type Existence = { verdict: string; rule?: string; detail?: string; workspace: "main" | "ticket-local" | "n/a"; checked?: string };
export type Rec = {
  index: number; name: string; group: string | null; order: number | null;
  provenance: Provenance; seedIndex?: number; placeholder?: string;
  command: string; cwd: string; runner: string; wrapper?: Wrapper; nxSub?: string;
  project?: string; projectInput?: string; target?: string; nxConfiguration?: string;
  nxFlags: string[]; targetArgs: string[]; passthru: string[]; argsViaFlag?: boolean; execProjects?: string; execCommand?: string[]; script?: string; tool?: string;
  env: Record<string, string>; envInline: Record<string, string>; envSemantic: Record<string, string>; envNoise: Record<string, string>; envSandbox: Record<string, string>;
  ready?: { action?: string; pattern?: string; uriFormat?: string; extra?: Json }; inputs: string[];
  tickets: string[]; ticketScoped: boolean; ticketReasons: string[]; refs: Ref[]; refsMissing: string[];
  existence: Existence;
  ticketStatus: string; deadReasons: string[];
  serves: number[]; needsServe: { port: number; servedBy: string[] }[];
  axes: Record<string, string>; axisVia: Record<string, string[]>; selection: { scope: string[]; filter: string[]; files: string[]; reporting: string[] }; residualArgs: string[]; residualEnv: string[];
  class: string; family: string; recommendation: string; notes: string[];
};
const WS = "${workspaceFolder}";
const expand = (s: string): string => s.replaceAll(WS, ROOT);
const relRoot = (s: string): string => s.replace(ROOT + "/", "").replace(ROOT, ".");
const INPUT_RE = /\$\{input:([^}]+)\}/g;
const inputsIn = (text: string): string[] => [...text.matchAll(INPUT_RE)].map((m) => m[1]!);

const FILE_TOKEN = /\/[^/]*\.(?:tsx?|mjs|cjs|jsx?|json|jsonc|py|feature|rs|md|toml)$/;
/** 🔗️ Paths a row refers to. Relative arguments of `nx run P:T -- …` are relative to the project root (Nx `cwd`), then to the workspace root. */
function collectRefs(row: Json, parsed: ParsedCommand, envAll: Record<string, string>, bases: string[]): Ref[] {
  const refs: Ref[] = [];
  const push = (role: string, raw: string, path: string, required: boolean): void => { refs.push({ role, raw, path: relRoot(path), exists: existsSync(path), required }); };
  const add = (role: string, raw: string, required: boolean): void => {
    if (!raw || raw.includes("${input:") || raw.includes("${env:") || /[*?]/.test(raw)) return;
    const path = expand(raw);
    if (!path.startsWith("/")) return;
    push(role, raw, path, required);
  };
  if (typeof row.cwd === "string") add("cwd", row.cwd, true);
  for (const [k, v] of Object.entries(envAll)) if (v.includes(WS) || v.startsWith("/")) add(`env:${k}`, v, !OUTPUT_ENV.has(k));
  const consider = (tokens: string[], role: string): void => {
    for (const t of tokens) {
      const cand = t.includes("=") && t.startsWith("-") ? t.slice(t.indexOf("=") + 1) : t;
      if (cand.startsWith("-") || cand.includes("${input:") || /[*?\s]/.test(cand)) continue;
      if (cand.includes(WS) || cand.startsWith("/")) { if (cand.startsWith("/") || cand.startsWith(WS)) add(role, cand, true); continue; }
      if (!(/^\.{1,2}\//.test(cand) || cand.startsWith(".🧬semio") || FILE_TOKEN.test(cand))) continue;
      const hit = bases.map((b) => join(b, cand)).find((p) => existsSync(p));
      push(role, cand, hit ?? join(bases[0] ?? ROOT, cand), true);
    }
  };
  consider([...parsed.argv.slice(1)], "arg");
  return refs;
}

function isTicketPath(text: string): boolean { return text.includes("/tickets/") || text.includes("🎫️tickets") ; }

function verifyNx(parsed: ParsedCommand, row: Json, envAll: Record<string, string>): Existence {
  const na: Existence = { verdict: "n/a", workspace: "n/a" };
  if (parsed.kind !== "nx-run" && parsed.kind !== "nx-exec") return na;
  const rootOverride = envAll.NX_WORKSPACE_ROOT_PATH ? expand(envAll.NX_WORKSPACE_ROOT_PATH) : undefined;
  const cwdAbs = typeof row.cwd === "string" ? expand(row.cwd) : ROOT;
  const ticketCwd = isTicketPath(cwdAbs) && cwdAbs !== ROOT;
  let local: WalkResult | null = null, localRoot = "";
  if (rootOverride || ticketCwd) {
    localRoot = rootOverride ?? cwdAbs;
    let probe = localRoot;
    while (probe.length > ROOT.length && !existsSync(join(probe, "nx.json")) && !existsSync(join(probe, "package.json"))) probe = dirname(probe);
    localRoot = probe.length > ROOT.length ? probe : localRoot;
    if (!existsSync(localRoot)) return { verdict: "ticket-workspace-missing", workspace: "ticket-local", detail: relRoot(localRoot) };
    local = ticketWorkspace(localRoot);
  }
  const names = parsed.kind === "nx-run" ? (parsed.project ? [parsed.project] : []) : (parsed.execProjects ?? "").split(",").filter(Boolean);
  if (parsed.kind === "nx-run" && parsed.projectInput) return { verdict: "input-selected", workspace: "main", detail: parsed.projectInput };
  if (names.length === 0) return { verdict: "missing-project-spec", workspace: "main" };
  const workspace = local ? "ticket-local" : "main";
  const checked = local ? relRoot(localRoot) : ".";
  for (const name of names) {
    const sources: Proj[] = [];
    const declared = (local ?? MAIN).projects.get(name);
    if (declared) sources.push(declared);
    const graph = GRAPH.nodes.get(name);
    if (!declared && !local && !graph) return { verdict: "missing-project", workspace, detail: name, checked };
    if (parsed.kind === "nx-exec") continue;
    const full = parsed.nxConfiguration && (declared?.targets.has(`${parsed.target}:${parsed.nxConfiguration}`) || graph?.targets.has(`${parsed.target}:${parsed.nxConfiguration}`));
    const target = full ? `${parsed.target}:${parsed.nxConfiguration}` : parsed.target!;
    const configuration = full ? undefined : parsed.nxConfiguration;
    if (declared) {
      const t = declared.targets.get(target);
      if (t) {
        if (configuration && !t.configurations.includes(configuration)) return { verdict: "missing-configuration", workspace, detail: `${name}:${target}:${configuration}`, checked };
        return { verdict: "ok-declared", workspace, checked };
      }
      const rule = inferenceRule(declared, target);
      if (rule) return { verdict: "ok-inferred-rule", rule, workspace, checked };
    }
    if (!local && graph?.targets.has(target)) return { verdict: "ok-inferred-cached-graph", workspace, checked, detail: declared ? "project declared, target only in the cached graph" : "project only in the cached graph" };
    if (!declared && graph) return { verdict: "missing-target", workspace, detail: `${name}:${target} (project only known from the cached graph)`, checked };
    if (!declared && local) return { verdict: "missing-project", workspace, detail: name, checked };
    return { verdict: "missing-target", workspace, detail: `${name}:${target}`, checked };
  }
  return { verdict: "ok-declared", workspace, checked };
}
//#endregion

//#region record builder
const READY_KEYS = new Set(["action", "pattern", "uriFormat"]);
const INPUT_DEFS = new Map<string, Json>((LAUNCH.inputs as Json[]).map((i) => [i.id, i]));
function buildRec(row: Json, index: number): Rec {
  const parsed = parseCommand(String(row.command ?? ""));
  const env: Record<string, string> = Object.fromEntries(Object.entries<Json>(row.env ?? {}).map(([k, v]) => [k, String(v)]));
  const envAll = { ...parsed.inlineEnv, ...env };
  const envSemantic: Record<string, string> = {}, envNoise: Record<string, string> = {}, envSandbox: Record<string, string> = {};
  for (const [k, v] of Object.entries(envAll)) (NOISE_ENV.has(k) ? envNoise : SANDBOX_ENV.has(k) ? envSandbox : envSemantic)[k] = v;
  const prov = provenanceByIndex[index]!;
  const baseProject = parsed.project ? MAIN.projects.get(parsed.project) : undefined;
  const refs = collectRefs(row, parsed, envAll, [...(baseProject && baseProject.root !== "." ? [join(ROOT, baseProject.root)] : []), ROOT]);
  const blob = [row.command, row.cwd, ...Object.values(envAll)].join("\n");
  const ticketRefs = ticketRefsIn(blob);
  const reasons: string[] = [];
  if (ticketRefs.length) reasons.push("ticket-path");
  if (/^ticket-/.test(parsed.project ?? "") ) reasons.push("ticket-named-project");
  if (/^current-native-origin-/.test(parsed.project ?? "")) reasons.push("ticket-named-project");
  if (blob.includes("🗑️generated")) reasons.push("generated-dir");
  const usedInputs = [...new Set(inputsIn(blob))];
  const inputTicketRefs = ticketRefsIn(usedInputs.map((id) => JSON.stringify(INPUT_DEFS.get(id) ?? {})).join("\n"));
  if (inputTicketRefs.length) reasons.push("ticket-input");
  const ready = row.serverReadyAction ? { action: row.serverReadyAction.action, pattern: row.serverReadyAction.pattern, uriFormat: row.serverReadyAction.uriFormat, extra: Object.fromEntries(Object.entries(row.serverReadyAction).filter(([k]) => !READY_KEYS.has(k))) } : undefined;
  const existence = verifyNx(parsed, row, envAll);
  return {
    index, name: row.name, group: row.presentation?.group ?? null, order: row.presentation?.order ?? null,
    provenance: prov.provenance, seedIndex: prov.seedIndex, placeholder: prov.placeholder,
    command: row.command, cwd: row.cwd, runner: parsed.kind, wrapper: parsed.wrapper, nxSub: parsed.nxSub,
    project: parsed.project, projectInput: parsed.projectInput, target: parsed.target, nxConfiguration: parsed.nxConfiguration,
    nxFlags: parsed.nxFlags, targetArgs: parsed.targetArgs, passthru: parsed.passthru, argsViaFlag: parsed.argsViaFlag, execProjects: parsed.execProjects, execCommand: parsed.execCommand, script: parsed.script, tool: parsed.tool,
    env, envInline: parsed.inlineEnv, envSemantic, envNoise, envSandbox, ready, inputs: usedInputs,
    tickets: [...new Set([...ticketRefs, ...inputTicketRefs].map((t) => t.id))], ticketScoped: reasons.length > 0, ticketReasons: [...new Set(reasons)], refs, refsMissing: refs.filter((r) => r.required && !r.exists).map((r) => `${r.role}:${r.path}`),
    existence, ticketStatus: "n/a", deadReasons: [],
    serves: [], needsServe: [], axes: {}, axisVia: {}, selection: { scope: [], filter: [], files: [], reporting: [] }, residualArgs: [], residualEnv: [],
    class: "", family: "", recommendation: "", notes: [],
  };
}
const RECS: Rec[] = timed("build-records", () => (LAUNCH.configurations as Json[]).map((row, i) => buildRec(row, i)));
//#endregion

//#region axes (cross-cutting dimensions extracted from env, Nx flags, target names and passthrough tokens)
const LEVELS = new Set(["quick", "long", "exhaustive"]);
const SCOPE_VALUE_FLAGS = new Set(["--test", "--bin", "--features", "--package", "-p", "--example", "--bench"]);
const SCOPE_BOOL_FLAGS = new Set(["--lib", "--doc", "--all-features", "--no-default-features", "--workspace", "--bins", "--tests"]);
const FILTER_VALUE_FLAGS = new Set(["--filter-expr", "-E", "-t", "--testNamePattern"]);
const REPORT_VALUE_FLAGS = new Set(["--status-level", "--final-status-level", "--test-threads", "--success-output", "--failure-output", "--reporter", "--silent"]);
const REPORT_BOOL_FLAGS = new Set(["--nocapture", "--no-capture", "--no-fail-fast", "--ignore-default-filter", "--offline", "--run", "--disable-console-intercept", "--verbose", "--quiet", "--locked"]);
export type Passthru = { level?: string; scope: string[]; filter: string[]; files: string[]; reporting: string[]; rest: string[] };
/** ✂️ Splits `-- …` tokens of test-like targets into the level, the test scope, the test filter, test files, reporting flags and the rest. */
export function parsePassthru(tokens: string[]): Passthru {
  const out: Passthru = { scope: [], filter: [], files: [], reporting: [], rest: [] };
  let i = 0;
  if (tokens[0] && LEVELS.has(tokens[0])) { out.level = tokens[0]; i = 1; }
  for (; i < tokens.length; i++) {
    const t = tokens[i]!;
    const eq = t.startsWith("--") && t.includes("=") ? t.slice(0, t.indexOf("=")) : t;
    if (t === "--") continue;
    if (SCOPE_BOOL_FLAGS.has(t)) out.scope.push(t);
    else if (SCOPE_VALUE_FLAGS.has(t) || SCOPE_VALUE_FLAGS.has(eq)) { out.scope.push(t.includes("=") ? t : `${t} ${tokens[++i] ?? ""}`.trim()); }
    else if (FILTER_VALUE_FLAGS.has(t)) out.filter.push(`${t} ${tokens[++i] ?? ""}`.trim());
    else if (FILTER_VALUE_FLAGS.has(eq)) out.filter.push(t);
    else if (REPORT_VALUE_FLAGS.has(t)) out.reporting.push(`${t} ${tokens[++i] ?? ""}`.trim());
    else if (REPORT_BOOL_FLAGS.has(t) || REPORT_VALUE_FLAGS.has(eq)) out.reporting.push(t);
    else if (t.startsWith("-")) out.rest.push(t);
    else if (FILE_TOKEN.test(t) || /^\.{1,2}\//.test(t) || t.includes("/")) out.files.push(t);
    else out.filter.push(t);
  }
  return out;
}
export type AxisValue = { value: string; via: string[] };
export type Axes = Record<string, AxisValue>;
const addAxis = (axes: Axes, axis: string, value: string, via: string): void => {
  const hit = axes[axis];
  if (!hit) axes[axis] = { value, via: [via] };
  else { hit.via.push(via); if (hit.value !== value && !hit.value.split("+").includes(value)) hit.value += `+${value}`; }
};
export function rowAxes(r: Pick<Rec, "envSemantic" | "nxFlags" | "target" | "passthru" | "env" | "envInline" | "name" | "runner" | "nxConfiguration" | "project">): { axes: Axes; passthru: Passthru; unexplainedEnv: string[] } {
  const axes: Axes = {};
  const passthru = parsePassthru(r.passthru);
  const explained = new Set<string>();
  for (const [k, v] of Object.entries(r.envSemantic)) {
    const axis = AXIS_OF_ENV[k] ?? (isPortVar(k) ? "port" : undefined);
    if (!axis) continue;
    explained.add(k);
    if (axis === "cache-policy") addAxis(axes, axis, k === "NX_SKIP_NX_CACHE" ? "skip-local" : "skip-remote", `env:${k}`);
    else if (axis === "playground") continue;
    else addAxis(axes, axis, v, `env:${k}`);
  }
  for (const f of r.nxFlags) {
    const [name, value] = f.split("=");
    if (name === "--skip-nx-cache" || name === "--skipNxCache") addAxis(axes, "cache-policy", "skip-local", `flag:${name}`);
    else if (name === "--skip-remote-cache") addAxis(axes, "cache-policy", "skip-remote", `flag:${name}`);
    else if (name === "--excludeTaskDependencies" || name === "--exclude-task-dependencies") addAxis(axes, "task-dependencies", "excluded", `flag:${name}`);
    else if (name === "--output-style" || name === "--outputStyle") addAxis(axes, "output-style", value ?? "?", `flag:${name}`);
  }
  const suffix = r.target?.match(/^test-(quick|long|exhaustive)$/)?.[1];
  if (suffix) addAxis(axes, "test-level", suffix, "target-name");
  if (passthru.level && /^test/.test(r.target ?? "")) addAxis(axes, "test-level", passthru.level, "positional");
  if (/^test/.test(r.target ?? "")) {
    if (passthru.scope.length) addAxis(axes, "test-scope", passthru.scope.join(" "), "passthru");
    if (passthru.filter.length) addAxis(axes, "test-filter", passthru.filter.join(" "), "passthru");
    if (passthru.files.length) addAxis(axes, "test-files", passthru.files.join(" "), "passthru");
  }
  return { axes, passthru, unexplainedEnv: Object.keys(r.envSemantic).filter((k) => !explained.has(k) && !(AXIS_OF_ENV[k] === "playground")) };
}
//#endregion

//#region classification
const VARIANT_TOKENS = new Set<string>([...PLAYGROUNDS.map((p) => p.variant as string), ...PLAYGROUNDS.flatMap((p) => (p.aliases ?? []) as string[])]);
const isTestTarget = (target: string | undefined): boolean => !!target && /^test/.test(target);
const NX_SPECIFIC_FLAG_RE = /^--(?:skip-nx-cache|skip-remote-cache|excludeTaskDependencies|output-style|outputStyle)/;
export function isPlain(r: Rec): boolean {
  return r.runner === "nx-run" && !!r.project && Object.keys(r.envSemantic).length === 0 && Object.keys(r.envSandbox).length === 0 && r.nxFlags.length === 0 && r.targetArgs.length === 0 && r.passthru.length === 0 && !r.ready && r.inputs.length === 0;
}
export function rollUpTicketStatus(r: Rec): string {
  if (!r.tickets.length) return r.ticketScoped ? "no-ticket-reference" : "n/a";
  const statuses = r.tickets.map((id) => ticketInfoCache.get(id)?.status ?? "unknown");
  if (statuses.every((s) => s === "closed")) return "closed";
  if (statuses.some((s) => s === "open")) return "open";
  if (statuses.every((s) => s === "missing")) return "missing";
  return [...new Set(statuses)].sort().join("+");
}
function classify(r: Rec): void {
  const ax = rowAxes(r);
  r.axes = Object.fromEntries(Object.entries(ax.axes).map(([k, v]) => [k, v.value]));
  r.axisVia = Object.fromEntries(Object.entries(ax.axes).map(([k, v]) => [k, v.via]));
  const testLike = isTestTarget(r.target);
  r.selection = testLike ? { scope: ax.passthru.scope, filter: ax.passthru.filter, files: ax.passthru.files, reporting: ax.passthru.reporting } : { scope: [], filter: [], files: [], reporting: [] };
  r.residualEnv = ax.unexplainedEnv;
  r.residualArgs = testLike ? [...r.targetArgs, ...ax.passthru.rest] : [...r.targetArgs, ...r.passthru];
  for (const id of r.tickets) ticketInfo(ticketRefsIn(`tickets/${id.split("/").slice(0, 3).map((x, k) => (k === 0 ? "🎆️" : k === 1 ? "🌙️" : "☀️") + x).join("/")}/${id.split("/")[3]}`)[0]!);
  r.ticketStatus = rollUpTicketStatus(r);
  if (r.existence.verdict.startsWith("missing") || r.existence.verdict === "ticket-workspace-missing") r.deadReasons.push(r.existence.verdict + (r.existence.detail ? `: ${r.existence.detail}` : ""));
  if (r.refsMissing.length) r.deadReasons.push(`missing-path: ${r.refsMissing.slice(0, 3).join(", ")}${r.refsMissing.length > 3 ? ` (+${r.refsMissing.length - 3})` : ""}`);

  if (r.ticketScoped) {
    r.class = "ticket-scoped";
    r.family = r.runner === "nx-exec" ? (/\.ts$/.test(r.execCommand?.[1] ?? "") || /\.ts$/.test(r.execCommand?.[0] ?? "") ? "ticket/nx-exec-ticket-script" : "ticket/nx-exec") : r.runner === "nx-run" ? "ticket/nx-run-preset" : r.runner === "bun-script" ? "ticket/bun-script" : `ticket/${r.runner}`;
    return;
  }
  if (r.provenance === "project-launcher-family") { r.class = "project-picker-family"; r.family = "project-picker"; return; }
  if (r.runner === "nx-run" && r.project === "workspace" && r.target === "dev") {
    if (VARIANT_TOKENS.has(r.passthru[0] ?? "") && r.ready) {
      r.class = "playground-dev";
      r.family = r.provenance === "playground-seed-placeholder" || r.provenance === "playground-synthesized" ? (/👤️\d/.test(r.name) ? "playground-dev/user-slot" : "playground-dev/registry-generated") : "playground-dev/seed-variant";
    } else { r.class = "tool-launcher"; r.family = "dev-tool/workspace-dev-mcp"; }
    return;
  }
  if (isPlain(r)) { r.class = "plain-nx"; r.family = "plain-nx"; return; }
  if (r.runner === "nx-run") {
    r.class = "nx-preset";
    if (r.target === "test-native-grammar" && Object.keys(r.axes).includes("print-phase")) r.family = "print-native-phase";
    else if (/^native/.test(r.target ?? "") && r.project === "@semio-tech/framework-renderer-wgpu" && VARIANT_TOKENS.has(r.passthru[0] ?? "")) r.family = "wgpu-native-launch";
    else if (r.ready) r.family = "dev-server-port";
    else if (r.inputs.length) r.family = "prompted-argument";
    else if (testLike) {
      const selected = r.selection.filter.length > 0 || r.selection.files.length > 0 || r.selection.scope.length > 0;
      r.family = selected ? (r.selection.files.length > 0 && !MAIN.projects.get(r.project ?? "")?.cargo ? "test-selection/vitest-files" : r.selection.filter.length > 0 ? "test-selection/filter" : "test-selection/scope") : "test-axis-variant";
    } else if (r.residualArgs.length || r.residualEnv.length) r.family = "target-specific-arguments";
    else r.family = "axis-variant";
    return;
  }
  if (r.runner === "nx-exec") { r.class = (r.execCommand ?? [])[0] === "bun" && (r.execCommand ?? [])[1] === "test" ? "nx-exec-bun-test" : "nx-exec-other"; r.family = r.class === "nx-exec-bun-test" ? "bun-test-file" : "nx-exec-other"; return; }
  r.class = "tool-launcher";
  r.family = r.runner === "nx-other" ? `nx-${r.nxSub}` : r.runner === "bun-script" ? "bun-script" : r.runner === "bun-x" ? "external-tool" : "external-tool";
}
function recommend(r: Rec): void {
  const dead = r.deadReasons.length > 0;
  if (r.class === "ticket-scoped") {
    r.recommendation = r.ticketStatus === "closed" ? "drop (ticket-scoped, closed ticket)" : r.ticketStatus === "open" ? "drop (ticket-scoped, open ticket)" : r.ticketStatus === "missing" ? "drop (ticket-scoped, ticket folder gone)" : r.ticketStatus === "no-ticket-reference" ? "drop (ticket-scoped, no ticket reference)" : `drop (ticket-scoped, ticket status ${r.ticketStatus})`;
    return;
  }
  if (dead) { r.recommendation = "drop (dead)"; return; }
  switch (r.family) {
    case "plain-nx": case "project-picker": r.recommendation = "drop (redundant with Nx target discovery)"; return;
    case "playground-dev/registry-generated": r.recommendation = "drop (redundant with playground catalog discovery)"; return;
    case "playground-dev/user-slot": r.recommendation = "express as a global axis"; return;
    case "playground-dev/seed-variant": case "wgpu-native-launch": r.recommendation = "express as a global axis"; return;
    case "nx-run-many": r.recommendation = "express as a compound"; return;
    case "test-axis-variant": case "axis-variant": r.recommendation = "express as a global axis"; return;
    case "test-selection/filter": case "test-selection/vitest-files": case "test-selection/scope": case "prompted-argument": case "bun-test-file": r.recommendation = "needs an input prompt"; return;
    default: r.recommendation = "keep as declared command preset in the owner's manifest";
  }
}
const SERVED_BY = new Map<number, string[]>();
for (const r of RECS) if (r.ready && !r.ticketScoped) {
  const ports = uniq([...Object.entries(r.env).filter(([k]) => isPortVar(k)).map(([, v]) => Number(v)), ...[...(r.ready.pattern ?? "").matchAll(/:(\d{4,5})\b/g)].map((m) => Number(m[1]))]).filter((n) => Number.isFinite(n));
  r.serves = ports;
  for (const port of ports) SERVED_BY.set(port, [...(SERVED_BY.get(port) ?? []), r.name]);
}
const URL_PORT = /https?:\/\/(?:127\.0\.0\.1|localhost|0\.0\.0\.0):(\d{4,5})/g;
for (const r of RECS) {
  const text = [r.command, ...Object.values(r.env), ...r.inputs.map((id) => String(INPUT_DEFS.get(id)?.default ?? ""))].join(" ");
  r.needsServe = uniq([...text.matchAll(URL_PORT)].map((m) => Number(m[1]))).filter((port) => !r.serves.includes(port)).map((port) => ({ port, servedBy: SERVED_BY.get(port) ?? [] }));
}
timed("classify", () => { for (const r of RECS) classify(r); for (const r of RECS) recommend(r); });
//#endregion

//#region helpers for analysis
const NON_TICKET = RECS.filter((r) => !r.ticketScoped);
const TICKET = RECS.filter((r) => r.ticketScoped);
//#endregion

//#region 1. seed & generator
function analyzeSeed() {
  const kinds = tallyObj(placeholderParts, (p) => p.kind);
  const launcherEntries = Object.entries<Json>(devLaunchers);
  const fieldFreq = tallyObj(launcherEntries.flatMap(([, v]) => Object.keys(v)), (k) => k);
  const generated = (p: Provenance) => RECS.filter((r) => r.provenance === p);
  const userRows = RECS.filter((r) => r.provenance === "playground-seed-placeholder" && /👤️\d/.test(r.name));
  const curated = new Set(Object.keys(devLaunchers));
  const uncurated = PLAYGROUND_VARIANTS.filter((v) => !curated.has(v));
  const policy = SEED.projectLaunchers ?? {};
  const covered = new Set<string>();
  for (const r of RECS.slice(0, BEFORE_PROJECT)) for (const m of String(r.command).matchAll(/nx run ([^\s:$]+):(\S+)/gu)) covered.add(`${m[1]}:${m[2]}`);
  const declaredPairs: { project: string; target: string }[] = [];
  const skip = new Set<string>(policy.skipDirectories ?? []);
  for (const p of MAIN.projects.values()) if (p.source === "manifest" && !p.root.split("/").some((seg) => skip.has(seg))) for (const t of p.targets.keys()) declaredPairs.push({ project: p.name, target: t });
  const owners = new Map<string, number>(); for (const d of declaredPairs) owners.set(d.target, (owners.get(d.target) ?? 0) + 1);
  const familyTargets = [...owners].filter(([, n]) => n >= (policy.familyMinimumProjects ?? 3)).map(([t]) => t);
  const familySet = new Set(familyTargets);
  const pending = declaredPairs.filter((d) => !familySet.has(d.target) && !covered.has(`${d.project}:${d.target}`));
  const classTokens = (policy.classes ?? []).map((c: Json) => ({ id: c.id, emoji: c.emoji, group: c.group, orderBase: c.orderBase, tokens: c.tokens.length }));
  const classOfProjectRow = tallyObj(generated("project-launcher-target").concat(generated("project-launcher-family")), (r) => `${r.group}`);
  return {
    topLevelKeys: Object.keys(SEED),
    configurations: { total: seedItems.length, objects: seedObjects.length, placeholders: seedPlaceholders.length, placeholderKinds: kinds },
    compounds: (SEED.compounds ?? []).length, inputs: (SEED.inputs ?? []).length,
    devLaunchers: { entries: launcherEntries.length, fieldFrequency: fieldFreq, withUsers: launcherEntries.filter(([, v]) => v.users).length, withEnvExtras: launcherEntries.filter(([, v]) => v.env && Object.keys(v.env).length).length, curatedVariants: curated.size, registryVariants: PLAYGROUND_VARIANTS.length, uncuratedVariants: uncurated.length, orderMin: Math.min(...launcherEntries.map(([, v]) => v.order)), orderMax: Math.max(...launcherEntries.map(([, v]) => v.order)), usersTemplates: launcherEntries.filter(([, v]) => v.users).map(([k, v]) => ({ variant: k, ...v.users })) },
    provenance: tallyObj(RECS, (r) => r.provenance),
    playgroundRows: { fromPlaceholders: generated("playground-seed-placeholder").length, userSlotRows: userRows.length, synthesized: generated("playground-synthesized").length, total: generated("playground-seed-placeholder").length + generated("playground-synthesized").length, withSemioPluginEnv: RECS.filter((r) => r.env.SEMIO_PLUGIN).length, seedAuthoredWithSemioPluginEnv: RECS.filter((r) => r.provenance === "seed-authored" && r.env.SEMIO_PLUGIN).length },
    projectLaunchers: { policy: { familyMinimumProjects: policy.familyMinimumProjects, familyEmoji: policy.familyEmoji, fallbackClass: policy.fallbackClass, classes: classTokens, skipDirectories: policy.skipDirectories, transparentSegments: policy.transparentSegments, languageSegments: policy.languageSegments }, declaredProjectsInWalk: uniq(declaredPairs.map((d) => d.project)).length, declaredPairs: declaredPairs.length, familyTargetNames: familyTargets.length, pairsInFamilyTargets: declaredPairs.filter((d) => familySet.has(d.target)).length, pairsCoveredByExistingRows: declaredPairs.filter((d) => !familySet.has(d.target) && covered.has(`${d.project}:${d.target}`)).length, pendingPairs: pending.length, familyRows: generated("project-launcher-family").length, targetRows: generated("project-launcher-target").length, rowsByGroup: classOfProjectRow, projectInputs: (LAUNCH.inputs as Json[]).filter((i) => String(i.id).startsWith("projectTarget.")).length },
    seedAuthored: { rows: generated("seed-authored").length, byClass: tallyObj(generated("seed-authored"), (r) => r.class), byGroup: tallyObj(generated("seed-authored"), (r) => r.group ?? "(none)"), ticketScoped: generated("seed-authored").filter((r) => r.ticketScoped).length },
    outputCountsByProvenanceAndClass: tallyObj(RECS, (r) => `${r.provenance} | ${r.class}`),
  };
}
const SEED_ANALYSIS = analyzeSeed();
//#endregion

//#region 2. duplicates, compounds, inputs, server-ready
function analyzeDuplicates() {
  const groups = new Map<string, Rec[]>();
  for (const r of RECS) { const k = JSON.stringify([r.command, r.cwd, r.env]); if (!groups.has(k)) groups.set(k, []); groups.get(k)!.push(r); }
  const dup = [...groups.values()].filter((g) => g.length > 1);
  const cmdEnv = new Map<string, Rec[]>();
  for (const r of RECS) { const k = JSON.stringify([r.command, r.env]); if (!cmdEnv.has(k)) cmdEnv.set(k, []); cmdEnv.get(k)!.push(r); }
  const dup2 = [...cmdEnv.values()].filter((g) => g.length > 1);
  return { namesUnique: new Set(RECS.map((r) => r.name)).size === RECS.length, duplicateCommandEnvCwdGroups: dup.length, duplicateRows: dup.reduce((a, g) => a + g.length, 0), duplicateCommandEnvGroups: dup2.length, groups: dup2.map((g) => ({ command: g[0]!.command.slice(0, 200), names: g.map((r) => r.name), classes: uniq(g.map((r) => r.class)), ticketScoped: g.every((r) => r.ticketScoped) })) };
}
function analyzeCompounds() {
  const byName = new Map(RECS.map((r) => [r.name, r]));
  return ((LAUNCH.compounds ?? []) as Json[]).map((c) => {
    const members = (c.configurations as Json[]).map((m) => {
      const name = typeof m === "string" ? m : m.name;
      const r = byName.get(name);
      return { name, found: !!r, index: r?.index, command: r?.command, class: r?.class, provenance: r?.provenance, project: r?.project, target: r?.target, port: r ? Object.entries(r.env).filter(([k]) => isPortVar(k)).map(([k, v]) => `${k}=${v}`) : [], envSemantic: r?.envSemantic ?? {}, ready: r?.ready ? { pattern: r.ready.pattern, uriFormat: r.ready.uriFormat } : null, folder: typeof m === "object" ? m.folder : undefined };
    });
    const hubDependent = members.filter((m) => Object.keys(m.envSemantic).some((k) => /HUB/.test(k)));
    return { name: c.name, stopAll: c.stopAll, preLaunchTask: c.preLaunchTask, presentation: c.presentation, extraKeys: Object.keys(c).filter((k) => !["name", "configurations", "stopAll", "presentation"].includes(k)), members, allMembersFound: members.every((m) => m.found), membersWithReadyAction: members.filter((m) => m.ready).length, hubDependentMembers: hubDependent.map((m) => m.name) };
  });
}
function analyzeInputs() {
  const used = new Map<string, Rec[]>();
  for (const r of RECS) for (const id of r.inputs) { if (!used.has(id)) used.set(id, []); used.get(id)!.push(r); }
  const declared = new Set((LAUNCH.inputs as Json[]).map((i) => i.id));
  const list = (LAUNCH.inputs as Json[]).map((i) => {
    const rows = used.get(i.id) ?? [];
    const text = JSON.stringify(i);
    const ticketRefs = ticketRefsIn(text).map((t) => t.id);
    return { id: i.id, type: i.type, description: i.description, default: i.default, options: i.options, optionCount: i.options?.length, generated: String(i.id).startsWith("projectTarget."), usedBy: rows.length, usedByClasses: tallyObj(rows, (r) => r.class), usedByNonTicket: rows.filter((r) => !r.ticketScoped).length, usedByTicketScoped: rows.filter((r) => r.ticketScoped).length, ticketScoped: ticketRefs.length > 0, tickets: ticketRefs, examples: sample(rows.map((r) => r.name)) };
  });
  const dangling = [...used.keys()].filter((id) => !declared.has(id));
  const optionOwnerCheck = list.filter((i) => i.generated).map((i) => {
    const target = String(i.id).slice("projectTarget.".length);
    const rows = RECS.filter((r) => r.projectInput === i.id);
    const bad = (i.options as string[]).filter((p) => !(MAIN.projects.get(p)?.targets.has(rows[0]?.target ?? "") || false));
    return { id: i.id, target: rows[0]?.target ?? target, options: i.optionCount, optionsWithoutTarget: bad };
  });
  return { total: list.length, byType: tallyObj(list, (i) => i.type), generated: list.filter((i) => i.generated).length, seedInputs: list.filter((i) => !i.generated).length, unused: list.filter((i) => i.usedBy === 0).map((i) => i.id), ticketScoped: list.filter((i) => i.ticketScoped).map((i) => i.id), danglingReferences: dangling, usedByNonTicketRows: list.filter((i) => i.usedByNonTicket > 0).map((i) => i.id), generatedPickerChecks: { checked: optionOwnerCheck.length, withInconsistentOptions: optionOwnerCheck.filter((c) => c.optionsWithoutTarget.length).length }, list };
}
function analyzeReady() {
  const withReady = RECS.filter((r) => r.ready);
  const norm = (r: Rec): string => {
    const portVars = Object.entries(r.env).filter(([k]) => isPortVar(k)).map(([, v]) => v);
    let pattern = r.ready!.pattern ?? "";
    for (const p of portVars) pattern = pattern.replaceAll(`:${p}`, ":{PORT}");
    pattern = pattern.replace(/:\d{4,5}\b/g, ":{PORT}");
    return JSON.stringify({ action: r.ready!.action, pattern, uriFormat: r.ready!.uriFormat, extra: r.ready!.extra });
  };
  const groups = new Map<string, Rec[]>();
  for (const r of withReady) { const k = norm(r); if (!groups.has(k)) groups.set(k, []); groups.get(k)!.push(r); }
  const portMatch = withReady.map((r) => { const ports = Object.entries(r.env).filter(([k]) => isPortVar(k)).map(([k, v]) => ({ k, v })); const inPattern = (r.ready!.pattern ?? "").match(/:(\d{4,5})\b/)?.[1]; return { index: r.index, portVars: ports, inPattern, matches: ports.some((p) => p.v === inPattern) }; });
  return {
    total: withReady.length, nonTicket: withReady.filter((r) => !r.ticketScoped).length, ticketScoped: withReady.filter((r) => r.ticketScoped).length, byProvenance: tallyObj(withReady, (r) => r.provenance), byClass: tallyObj(withReady, (r) => r.class),
    actions: tallyObj(withReady, (r) => r.ready!.action ?? "(none)"),
    extraKeys: tallyObj(withReady, (r) => Object.keys(r.ready!.extra ?? {}).join(",") || "(none)"),
    distinct: [...groups].sort((a, b) => b[1].length - a[1].length).map(([k, rows]) => ({ ...JSON.parse(k), rows: rows.length, examples: sample(rows.map((r) => r.name)) })),
    portInPatternMatchesPortEnv: { rows: portMatch.length, matching: portMatch.filter((p) => p.matches).length, nonMatching: portMatch.filter((p) => !p.matches).map((p) => ({ index: p.index, name: RECS[p.index]!.name, portVars: p.portVars, inPattern: p.inPattern })) },
    uriFormats: tallyObj(withReady, (r) => r.ready!.uriFormat ?? "(none)"),
    withoutPortEnv: withReady.filter((r) => !Object.keys(r.env).some(isPortVar)).map((r) => r.name).slice(0, 10),
  };
}
const DUPLICATES = analyzeDuplicates();
const COMPOUNDS = analyzeCompounds();
const INPUTS = analyzeInputs();
const READY = analyzeReady();
//#endregion

//#region 3. families & cross-cutting axes
const baseTargetId = (r: Rec): string => r.runner === "nx-run" ? `${r.project ?? r.projectInput}:${r.target}${r.nxConfiguration ? ":" + r.nxConfiguration : ""}` : r.runner === "nx-exec" ? `exec[${r.execProjects}]:${(r.execCommand ?? []).slice(0, 2).join(" ")}` : `${r.runner}:${r.tool ?? r.script ?? r.nxSub ?? ""}`;
const domainOf = (values: string[], limit = 12): { value: string; rows: number }[] => tally(values, (v) => v).slice(0, limit).map(([value, rows]) => ({ value, rows }));
function axisSummary(rows: Rec[]) {
  const axisNames = uniq(rows.flatMap((r) => Object.keys(r.axes)));
  return axisNames.map((axis) => {
    const users = rows.filter((r) => axis in r.axes);
    const forms = tallyObj(users.flatMap((r) => r.axisVia[axis]!), (v) => v);
    const values = users.map((r) => r.axes[axis]!);
    return { axis, rows: users.length, baseTargets: uniq(users.map(baseTargetId)).length, owners: uniq(users.map((r) => r.project ?? r.execProjects ?? "")).length, expressedAs: forms, distinctValues: uniq(values).length, domain: domainOf(values), exampleBaseTargets: sample(uniq(users.map(baseTargetId)), 4) };
  }).sort((a, b) => b.rows - a.rows);
}
const constantOf = (lists: string[][]): string[] => (lists.length === 0 ? [] : lists.reduce((acc, l) => acc.filter((x) => l.includes(x))));
function analyzeFamilies() {
  const scope = NON_TICKET.filter((r) => !["plain-nx", "project-picker-family"].includes(r.class));
  const groups = new Map<string, Rec[]>();
  for (const r of scope) { const k = `${r.class} / ${r.family}`; if (!groups.has(k)) groups.set(k, []); groups.get(k)!.push(r); }
  const families = [...groups].map(([id, rows]) => {
    const bases = new Map<string, Rec[]>();
    for (const r of rows) { const k = baseTargetId(r); if (!bases.has(k)) bases.set(k, []); bases.get(k)!.push(r); }
    const consts = (key: (r: Rec) => string | undefined): string[] => { const sets = rows.map((r) => key(r)); return sets.every((s) => s === sets[0]) && sets[0] ? [sets[0]!] : []; };
    return {
      id, class: rows[0]!.class, family: rows[0]!.family, rows: rows.length, baseTargets: bases.size, owners: uniq(rows.map((r) => r.project ?? r.execProjects ?? r.tool ?? r.script ?? (r.nxSub ? `nx ${r.nxSub} (project set)` : ""))).sort(),
      axes: axisSummary(rows),
      residualArgDomain: domainOf(rows.map((r) => r.residualArgs.join(" ")).filter(Boolean), 10),
      residualEnvKeys: tallyObj(rows.flatMap((r) => r.residualEnv), (k) => k),
      constantProject: consts((r) => r.project), constantTarget: consts((r) => r.target),
      ready: uniq(rows.filter((r) => r.ready).map((r) => `${(r.ready!.pattern ?? "").replace(/:\d{4,5}\b/g, ":{PORT}")} -> ${r.ready!.uriFormat}`)).slice(0, 4),
      constantEnv: constantOf(rows.map((r) => Object.entries({ ...r.envInline, ...r.envSemantic }).map(([k, v]) => `${k}=${v}`))),
      constantArgs: constantOf(rows.map((r) => [...r.nxFlags, ...r.targetArgs, ...r.passthru])),
      examples: sample(rows.map((r) => r.name), 3),
      topBaseTargets: [...bases].sort((a, b) => b[1].length - a[1].length).slice(0, 8).map(([base, rs]) => ({ base, rows: rs.length })),
    };
  }).sort((a, b) => b.rows - a.rows);
  const baseTargets = [...(() => { const m = new Map<string, Rec[]>(); for (const r of scope) { const k = baseTargetId(r); if (!m.has(k)) m.set(k, []); m.get(k)!.push(r); } return m; })()].map(([base, rows]) => ({
    base, project: rows[0]!.project ?? rows[0]!.execProjects, target: rows[0]!.target, classes: uniq(rows.map((r) => r.class)), families: uniq(rows.map((r) => r.family)), rows: rows.length,
    axes: Object.fromEntries(axisSummary(rows).map((a) => [a.axis, a.domain.map((d) => d.value)])),
    argSets: domainOf(rows.map((r) => r.residualArgs.join(" ")).filter(Boolean), 8).map((d) => d.value),
    selection: { scope: uniq(rows.flatMap((r) => r.selection.scope)).slice(0, 6), withFilter: rows.filter((r) => r.selection.filter.length).length, withFiles: rows.filter((r) => r.selection.files.length).length },
    ready: uniq(rows.filter((r) => r.ready).map((r) => `${r.ready!.pattern}`)).slice(0, 2), examples: sample(rows.map((r) => r.name), 2), indexes: rows.map((r) => r.index),
  })).sort((a, b) => b.rows - a.rows);
  const allNx = RECS.filter((r) => r.runner === "nx-run" || r.runner === "nx-exec" || r.runner === "nx-other" || r.provenance === "playground-seed-placeholder" || r.provenance === "playground-synthesized");
  const axesNonTicket = axisSummary(NON_TICKET.filter((r) => !["plain-nx", "project-picker-family"].includes(r.class)));
  const axesTicket = axisSummary(TICKET);
  const axesAll = axisSummary(allNx);
  return { families, baseTargets, axesNonTicket, axesTicket, axesAll, globalAxes: axesNonTicket.filter((a) => a.baseTargets >= 3).map((a) => a.axis), targetSpecific: axesNonTicket.filter((a) => a.baseTargets < 3).map((a) => a.axis) };
}
const FAMILIES = analyzeFamilies();
//#endregion

//#region 4. tickets
function analyzeTickets() {
  const ids = uniq(TICKET.flatMap((r) => r.tickets));
  const noRef = TICKET.filter((r) => r.tickets.length === 0);
  const tickets = ids.map((id) => {
    const rows = TICKET.filter((r) => r.tickets.includes(id));
    const info = ticketInfoCache.get(id)!;
    const allRefs = rows.flatMap((r) => r.refs.filter((x) => x.required));
    const missing = rows.filter((r) => r.refsMissing.length || r.existence.verdict === "ticket-workspace-missing" || r.existence.verdict === "missing-target" || r.existence.verdict === "missing-project");
    const missingPaths = tally(rows.flatMap((r) => r.refsMissing.map((m) => m.replace(/^[^:]+:/, ""))), (m) => m.replace(/(?:\/[^/]+){1,}$/, (s) => (s.length > 90 ? s.slice(0, 90) : s))).slice(0, 5);
    return {
      id, title: info.title, status: info.status, ticketJson: info.ticketJson, folderExists: info.exists, goal: info.goal, rows: rows.length,
      byFamily: tallyObj(rows, (r) => r.family), byReason: tallyObj(rows, (r) => r.ticketReasons.join("+")), byGroup: tallyObj(rows, (r) => r.group ?? "(none)"),
      rowsAllRequiredPathsExist: rows.filter((r) => r.refsMissing.length === 0 && r.existence.verdict !== "ticket-workspace-missing").length,
      rowsWithMissingPaths: rows.filter((r) => r.refsMissing.length > 0).length, rowsWithMissingWorkspace: rows.filter((r) => r.existence.verdict === "ticket-workspace-missing").length,
      requiredReferences: allRefs.length, requiredReferencesMissing: allRefs.filter((x) => !x.exists).length,
      outputDirReferences: rows.flatMap((r) => r.refs.filter((x) => !x.required)).length, outputDirsPresent: rows.flatMap((r) => r.refs.filter((x) => !x.required && x.exists)).length,
      rowsWithExistingNxWorkspace: rows.filter((r) => r.existence.workspace === "ticket-local" && r.existence.verdict.startsWith("ok")).length,
      missingRowCount: missing.length, topMissingPathPrefixes: missingPaths, examples: sample(rows.map((r) => r.name), 3),
    };
  }).sort((a, b) => b.rows - a.rows);
  return { ticketScopedRows: TICKET.length, byReason: tallyObj(TICKET, (r) => r.ticketReasons.join("+")), referencedTickets: ids.length, byStatus: tallyObj(tickets, (t) => t.status), rowsByTicketStatus: tallyObj(TICKET, (r) => r.ticketStatus), tickets, rowsWithoutTicketReference: noRef.map((r) => ({ index: r.index, name: r.name, reasons: r.ticketReasons, command: r.command.slice(0, 160) })), byFamily: tallyObj(TICKET, (r) => r.family), byProvenance: tallyObj(TICKET, (r) => r.provenance), nxWrapper: tallyObj(TICKET, (r) => r.wrapper ?? r.runner), ticketLocalNxWorkspaces: [...ticketWorkspaceCache].map(([root, w]) => ({ root: relRoot(root), exists: !!w, projects: w?.projects.size ?? 0 })) };
}
const TICKETS = analyzeTickets();
//#endregion

//#region 5. dead entries
function analyzeDead() {
  const dead = RECS.filter((r) => r.deadReasons.length);
  const kind = (r: Rec): string => r.existence.verdict.startsWith("missing") || r.existence.verdict === "ticket-workspace-missing" ? r.existence.verdict : "missing-path";
  return {
    total: dead.length, nonTicket: dead.filter((r) => !r.ticketScoped).length, ticketScoped: dead.filter((r) => r.ticketScoped).length,
    byKind: tallyObj(dead, kind), byKindNonTicket: tallyObj(dead.filter((r) => !r.ticketScoped), kind),
    nxVerdicts: tallyObj(RECS.filter((r) => r.existence.verdict !== "n/a"), (r) => `${r.existence.verdict} (${r.existence.workspace})`), nxRules: tallyObj(RECS.filter((r) => r.existence.rule), (r) => r.existence.rule!),
    nonTicketList: dead.filter((r) => !r.ticketScoped).map((r) => ({ index: r.index, name: r.name, command: r.command.slice(0, 200), reasons: r.deadReasons })),
    ticketMissingWorkspace: tally(dead.filter((r) => r.existence.verdict === "ticket-workspace-missing"), (r) => r.existence.detail ?? "").slice(0, 20),
    projectsMissing: RECS.filter((r) => r.existence.verdict === "missing-project").length,
    all: dead.map((r) => ({ index: r.index, name: r.name, ticketScoped: r.ticketScoped, kind: kind(r), command: r.command.slice(0, 240), reasons: r.deadReasons })),
    missingPathKinds: tallyObj(dead.flatMap((r) => r.refs.filter((x) => x.required && !x.exists).map((x) => (x.path.includes("🗑️generated") ? "inside a deleted 🗑️generated folder" : /\.(ts|tsx|mjs|js|json|sh|py)$/.test(x.path) ? "missing file" : "missing directory"))), (k) => k),
    missingRefsNonTicket: dead.filter((r) => !r.ticketScoped).flatMap((r) => r.refs.filter((x) => x.required && !x.exists).map((x) => ({ index: r.index, role: x.role, path: x.path }))),
  };
}
const DEAD = analyzeDead();
//#endregion

//#region 6. presentation grammar & name derivation
const taxonomyFolderSlug = (name: string): string => name.replace(/^(?:[0-9#*]️?⃣|\p{Extended_Pictographic}|\p{Emoji_Presentation}|\p{Emoji_Modifier}|️|‍)+/u, "");
const NAME_KEY = /^((?:\p{Extended_Pictographic}|\p{Emoji_Presentation}|️|‍)+)([A-Za-z]+)?/u;
export function nameKey(name: string): string { const m = name.match(NAME_KEY); return m ? `${m[1]}${m[2] ?? ""}` : "(no leading emoji)"; }
function verifyProjectLauncherNames() {
  const policy = SEED.projectLaunchers;
  const skip = new Set<string>(policy.skipDirectories);
  const projects = [...MAIN.projects.values()].filter((p) => p.source === "manifest" && !p.root.split("/").some((seg) => skip.has(seg))).map((p) => ({ project: p.name, path: p.root === "." ? "" : p.root, targets: [...p.targets.keys()].sort() })).sort((a, b) => a.project.localeCompare(b.project));
  const segments = new Map(projects.map((p) => [p.project, p.path === "" ? [] : p.path.split("/").filter((s) => !policy.transparentSegments.includes(s)).map((s) => policy.languageSegments[s] ?? s)]));
  const labels = new Map<string, string>();
  for (const [project, own] of segments) {
    let label = own.join("");
    for (let length = 1; length <= own.length; length++) {
      const suffix = own.slice(-length).join("");
      if ([...segments].every(([other, theirs]) => other === project || theirs.slice(-length).join("") !== suffix)) { label = suffix; break; }
    }
    labels.set(project, label);
  }
  const classOf = (target: string): Json => { for (const token of taxonomyFolderSlug(target).split("-")) { const found = policy.classes.find((c: Json) => c.tokens.includes(token)); if (found) return found; } return policy.classes.find((c: Json) => c.id === policy.fallbackClass); };
  const targetRows = RECS.filter((r) => r.provenance === "project-launcher-target");
  const familyRows = RECS.filter((r) => r.provenance === "project-launcher-family");
  const expectedTarget = new Map(targetRows.map((r) => [`${r.project}:${r.target}`, r]));
  let nameMatches = 0, orderMatches = 0, groupMatches = 0;
  const counters = new Map<string, number>();
  const pending = [...expectedTarget.keys()].map((k) => { const r = expectedTarget.get(k)!; return { project: r.project!, target: r.target!, label: labels.get(r.project!) ?? "" }; }).sort((a, b) => a.label.localeCompare(b.label) || a.target.localeCompare(b.target));
  for (const p of pending) {
    const cls = classOf(p.target);
    const index = counters.get(cls.id) ?? 0; counters.set(cls.id, index + 1);
    const row = expectedTarget.get(`${p.project}:${p.target}`)!;
    if (row.name === `${cls.emoji}${p.target}${p.label}`) nameMatches++;
    if (row.order === Math.round((cls.orderBase + index / 10000) * 10000) / 10000) orderMatches++;
    if (row.group === cls.group) groupMatches++;
  }
  const familyCounters = new Map<string, number>();
  let familyNameMatches = 0, familyOrderMatches = 0;
  for (const r of [...familyRows].sort((a, b) => String(a.target).localeCompare(String(b.target)))) {
    const target = r.target!;
    const cls = classOf(target);
    const index = familyCounters.get(cls.id) ?? 0; familyCounters.set(cls.id, index + 1);
    if (r.name === `${cls.emoji}${target}${policy.familyEmoji}`) familyNameMatches++;
    if (r.order === Math.round((cls.orderBase - 1 + index / 10000) * 10000) / 10000) familyOrderMatches++;
  }
  return { targetRows: targetRows.length, nameDerivable: nameMatches, orderDerivable: orderMatches, groupDerivable: groupMatches, familyRows: familyRows.length, familyNameDerivable: familyNameMatches, familyOrderDerivable: familyOrderMatches };
}
function analyzePresentation() {
  const groups = [...new Set(RECS.map((r) => r.group ?? "(none)"))].sort();
  const stat = groups.map((g) => {
    const rows = RECS.filter((r) => (r.group ?? "(none)") === g);
    const orders = rows.map((r) => r.order).filter((o): o is number => typeof o === "number");
    return { group: g, rows: rows.length, orderMin: orders.length ? Math.min(...orders) : null, orderMax: orders.length ? Math.max(...orders) : null, distinctOrders: uniq(orders).length, provenance: tallyObj(rows, (r) => r.provenance), classes: tallyObj(rows, (r) => r.class), topNameKeys: tally(rows, (r) => nameKey(r.name)).slice(0, 6).map(([k, n]) => `${k} ${n}`), examples: sample(rows.map((r) => r.name), 2) };
  });
  const keyed = tally(RECS, (r) => nameKey(r.name)).slice(0, 30).map(([key, n]) => ({ key, rows: n, groups: tallyObj(RECS.filter((r) => nameKey(r.name) === key), (r) => r.group ?? "(none)"), classes: tallyObj(RECS.filter((r) => nameKey(r.name) === key), (r) => r.class) }));
  const orderPairs = tally(RECS.filter((r) => r.group && typeof r.order === "number"), (r) => `${r.group}|${r.order}`);
  const compoundNames = ((LAUNCH.compounds ?? []) as Json[]).map((c) => c.name as string);
  const rendererSuffix = tallyObj(RECS.filter((r) => r.class === "playground-dev"), (r) => r.name.endsWith("⚛️react") ? "⚛️react" : r.name.endsWith("🧊️wgpu🌐️wasm") ? "🧊️wgpu🌐️wasm" : r.name.endsWith("🧊️wgpu🖥️native") ? "🧊️wgpu🖥️native" : "(other)");
  return {
    groups: stat, topNameKeys: keyed, distinctGroupOrderPairs: orderPairs.length, rowsSharingGroupOrder: orderPairs.filter(([, n]) => n > 1).reduce((a, [, n]) => a + n, 0),
    rowsWithoutPresentation: RECS.filter((r) => r.group === null).length, rowsWithoutPresentationExamples: sample(RECS.filter((r) => r.group === null).map((r) => r.name), 3),
    projectLauncherDerivation: verifyProjectLauncherNames(), compoundNames, playgroundRendererSuffix: rendererSuffix,
    namesWithoutLeadingEmoji: RECS.filter((r) => nameKey(r.name) === "(no leading emoji)").length,
    namesWithSpaces: RECS.filter((r) => /\s/.test(r.name)).length,
    userSlotNames: RECS.filter((r) => /👤️\d/.test(r.name)).map((r) => r.name),
  };
}
const PRESENTATION = analyzePresentation();
//#endregion

//#region 7. .claude/launch.json
function analyzeClaude() {
  const entries = (CLAUDE.configurations as Json[]).map((c, index) => {
    const exec = c.runtimeExecutable as string | undefined;
    const command = exec ? [exec, ...(c.runtimeArgs ?? [])].join(" ") : "";
    const parsed = exec ? parseCommand(command) : undefined;
    const env: Record<string, string> = Object.fromEntries(Object.entries<Json>(c.env ?? {}).map(([k, v]) => [k, String(v)]));
    const semantic = Object.fromEntries(Object.entries(env).filter(([k]) => !NOISE_ENV.has(k)));
    const noise = Object.fromEntries(Object.entries(env).filter(([k]) => NOISE_ENV.has(k)));
    let kind = "attach-only", canonical = "", variant: string | undefined, renderer: string | undefined;
    const notes: string[] = [];
    if (parsed?.kind === "nx-run") {
      const spec = `${parsed.project}:${parsed.target}`;
      if (parsed.project === "workspace" && parsed.target === "dev") {
        const first = parsed.passthru[0] ?? "";
        if (VARIANT_TOKENS.has(first)) { kind = "playground-dev"; variant = PLAYGROUNDS.find((p) => p.variant === first || (p.aliases ?? []).includes(first))?.variant; renderer = env.SEMIO_RENDERER ?? "react"; canonical = `playground ${variant ?? first} (${renderer})${parsed.passthru.length > 1 ? " args: " + parsed.passthru.slice(1).join(" ") : ""}`; }
        else { kind = "dev-tool"; canonical = `workspace:dev -- ${parsed.passthru.join(" ")}`; }
      } else if (parsed.project === "@semio-tech/framework-os-dev" && /^(?:serve|dev)(?:-|$)/.test(parsed.target ?? "")) {
        const m = (parsed.target ?? "").match(/^(?:serve|dev)-(.+)-(react|wgpu)-(dev|release)$/);
        kind = "playground-dev"; variant = m?.[1] ?? parsed.passthru[0]; renderer = m?.[2] ?? env.SEMIO_RENDERER ?? "wgpu";
        const known = PLAYGROUND_VARIANTS.includes(variant ?? "") ? variant : PLAYGROUNDS.find((p) => (p.aliases ?? []).includes(variant ?? ""))?.variant;
        canonical = `playground ${known ?? variant} (${renderer})`;
        if (!known) notes.push(`variant ${variant} not in the playground registry`);
      } else if (parsed.project === "@semio-tech/framework-renderer-wgpu" && /^native/.test(parsed.target ?? "")) { kind = "playground-native"; variant = parsed.passthru[0]; renderer = "wgpu-native"; canonical = `playground ${variant} (wgpu native)`; }
      else if (isTestTarget(parsed.target) || /check|verify/.test(parsed.target ?? "")) { kind = "gate"; canonical = spec; }
      else if (parsed.target === "dev" || parsed.target === "dev-site") { kind = "service-dev"; canonical = spec; }
      else { kind = "nx-target"; canonical = spec; }
    } else if (parsed) {
      kind = /\/🎫️tickets\/|^\.🧬semio\/🦑️repo\/🎫️tickets/.test(command) ? "ticket-script" : parsed.kind === "bun-script" ? "script" : `external:${exec}`;
      canonical = kind === "ticket-script" ? "(ticket-local script)" : command.slice(0, 80);
    }
    const ex = parsed && parsed.kind === "nx-run" ? verifyNx(parsed, { cwd: "${workspaceFolder}" }, env) : undefined;
    const registryPort = variant && renderer ? (PLAYGROUNDS.find((p) => p.variant === variant)?.ports?.[renderer.startsWith("wgpu") ? "wgpu" : "react"] as number | undefined) : undefined;
    const sameAsLaunchRow = command ? RECS.find((r) => r.command.replace(/\s+/g, " ") === command.replace(/\s+/g, " ") && JSON.stringify(Object.fromEntries(Object.entries(r.env).filter(([k]) => !NOISE_ENV.has(k)))) === JSON.stringify(semantic)) : undefined;
    const commandRef = command && (/\.🧬semio\/🦑️repo\/🎫️tickets\/\S+/.exec(command)?.[0] ?? "");
    const refExists = commandRef ? existsSync(join(ROOT, commandRef.split(/\s+/)[0]!)) : undefined;
    const pathRefs = (command.match(/(?:\S*\.(?:ts|sh)\b)/g) ?? []).filter((p) => p.includes("/")).map((p) => ({ path: p, exists: existsSync(join(ROOT, p)) }));
    return { index, name: c.name as string, hasCommand: !!exec, command, kind, canonical, variant, renderer, port: c.port as number | undefined, url: c.url as string | undefined, registryPort, portMatchesRegistry: registryPort === undefined || c.port === undefined ? undefined : registryPort === c.port, envSemantic: semantic, envNoise: noise, existence: ex?.verdict ?? "n/a", existenceDetail: ex?.detail, identicalLaunchJsonRow: sameAsLaunchRow?.name, pathRefs, missingPathRefs: pathRefs.filter((p) => !p.exists).map((p) => p.path), notes, extraKeys: Object.keys(c).filter((k) => !["name", "runtimeExecutable", "runtimeArgs", "env", "port", "url"].includes(k)) };
  });
  const byKind = tallyObj(entries, (e) => e.kind);
  return {
    total: entries.length, withCommand: entries.filter((e) => e.hasCommand).length, attachOnly: entries.filter((e) => !e.hasCommand).length, withPort: entries.filter((e) => e.port !== undefined).length, withUrl: entries.filter((e) => e.url !== undefined).length, withEnv: entries.filter((e) => Object.keys(e.envSemantic).length + Object.keys(e.envNoise).length > 0).length,
    byKind, topKeys: tallyObj(entries.flatMap((e) => ["name", ...(e.hasCommand ? ["runtimeExecutable", "runtimeArgs"] : []), ...(e.port !== undefined ? ["port"] : []), ...(e.url !== undefined ? ["url"] : []), ...(Object.keys(e.envSemantic).length + Object.keys(e.envNoise).length ? ["env"] : [])]), (k) => k),
    identicalToLaunchJsonRow: entries.filter((e) => e.identicalLaunchJsonRow).length, deadByNx: entries.filter((e) => e.existence.startsWith("missing")).map((e) => ({ name: e.name, detail: e.existenceDetail })), missingPathRefs: entries.filter((e) => e.missingPathRefs.length).map((e) => ({ name: e.name, paths: e.missingPathRefs })),
    portMismatches: entries.filter((e) => e.portMatchesRegistry === false).map((e) => ({ name: e.name, port: e.port, registryPort: e.registryPort, variant: e.variant, renderer: e.renderer })), noiseEnvKeys: tallyObj(entries.flatMap((e) => Object.keys(e.envNoise)), (k) => k),
    ticketScripts: entries.filter((e) => e.kind === "ticket-script").length, entries,
  };
}
const CLAUDE_ANALYSIS = analyzeClaude();
//#endregion

//#region 8. environment variables: counts and where they are read
const ALL_ENV_KEYS = uniq(RECS.flatMap((r) => [...Object.keys(r.env), ...Object.keys(r.envInline)])).concat(uniq((CLAUDE.configurations as Json[]).flatMap((c) => Object.keys(c.env ?? {})))).filter((k, i, a) => a.indexOf(k) === i).sort();
const CODE_EXT = /\.(?:ts|tsx|mts|cts|mjs|cjs|js|rs|py|go|sh|cs)$/;
const DOC_EXT = /\.(?:json|jsonc|toml|yml|yaml|md|feature)$/;
const readPatterns = (k: string): RegExp => new RegExp(String.raw`(?:process\.env|Bun\.env|import\.meta\.env|\benv|\benvironment|\bchildEnv|\binvocation\.env)(?:\.${k}\b|\[\s*["']${k}["']\s*\])|env::var(?:_os)?\(\s*"${k}"|(?:option_)?env!\(\s*"${k}"|environ(?:\.get)?\s*[\[(]\s*["']${k}["']|getenv\(\s*["']${k}["']|Getenv\("${k}"\)|LookupEnv\("${k}"\)|Environment\.GetEnvironmentVariable\("${k}"\)`);
export type EnvEvidence = { key: string; filesMentioning: number; filesReading: number; readSites: string[]; nxInternalReaders: string[]; scannedFiles: number };
function scanEnvEvidence(keys: string[]): Record<string, EnvEvidence> {
  const ls = Bun.spawnSync(["git", "ls-files", "-z"], { cwd: ROOT, stdout: "pipe", stderr: "ignore" });
  const files = new TextDecoder().decode(ls.stdout).split("\0").filter(Boolean);
  const selected = files.filter((f) => (CODE_EXT.test(f) || DOC_EXT.test(f)) && !f.startsWith(".vscode/") && !f.startsWith(".claude/") && !f.includes("🎫️tickets/") && !f.includes("/node_modules/") && !f.includes("/🗑️generated/"));
  const combined = new RegExp(String.raw`\b(?:${keys.map((k) => k.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|")})\b`, "g");
  const readers = new Map(keys.map((k) => [k, readPatterns(k)]));
  const out: Record<string, EnvEvidence> = Object.fromEntries(keys.map((k) => [k, { key: k, filesMentioning: 0, filesReading: 0, readSites: [], nxInternalReaders: [], scannedFiles: selected.length }]));
  for (const f of selected) {
    let text: string;
    try { const size = statSync(join(ROOT, f)).size; if (size > 8 * 1024 * 1024) continue; text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
    const found = new Set<string>();
    for (const m of text.matchAll(combined)) found.add(m[0]);
    for (const k of found) {
      out[k]!.filesMentioning++;
      if (CODE_EXT.test(f) && readers.get(k)!.test(text)) { out[k]!.filesReading++; if (out[k]!.readSites.length < 4) out[k]!.readSites.push(f); }
    }
  }
  const nxDir = [join(ROOT, ".nx/installation/node_modules/nx"), join(ROOT, "node_modules/nx")].find((d) => existsSync(join(d, "dist")));
  if (nxDir) for (const k of keys.filter((x) => /^NX_|^FORCE_COLOR$/.test(x))) {
    const res = Bun.spawnSync(["/usr/bin/grep", "-rlE", "--include=*.js", `process\\.env\\.${k}\\b|process\\.env\\[['"]${k}['"]\\]`, "dist"], { cwd: nxDir, stdout: "pipe", stderr: "ignore" });
    out[k]!.nxInternalReaders = new TextDecoder().decode(res.stdout).split("\n").filter(Boolean).slice(0, 4).map((p) => `${nxDir.replace(ROOT + "/", "")}/${p}`);
  }
  return out;
}
const ENV_EVIDENCE: Record<string, EnvEvidence> = flag("skip-env-scan") ? {} : timed("env-evidence-scan", () => scanEnvEvidence(ALL_ENV_KEYS));
//#endregion

//#region 9. environment judgement table
type EnvKind = "runner-noise" | "ticket-sandbox" | "global-axis" | "playground-binding" | "port" | "output-location" | "service-config" | "target-config" | "toolchain" | "secret-like" | "unread-probe-knob";
const ENV_JUDGEMENT: Record<string, [EnvKind, string]> = {
  NX_DAEMON: ["runner-noise", "Nx daemon off; the `bun nx` wrapper already defaults it to \"false\" (bootstrap `nxChildEnvironment`), only the rows that call `nx.js` directly need it. Drop."],
  NX_ISOLATE_PLUGINS: ["runner-noise", "Wrapper sets `NX_ISOLATE_PLUGINS ?? \"false\"` itself (bootstrap `run`). Drop."],
  NX_CACHE_PROJECT_GRAPH: ["runner-noise", "Nx graph-cache switch; the repo never sets or reads it, only Nx does. Rows pin it to false (twice to true). Drop; the dashboard owns its graph policy (`NX_FORCE_REUSE_CACHED_GRAPH`, see build-wait-investigation.md)."],
  NX_TUI: ["runner-noise", "Nx terminal UI off; `devToolingEnv()` already does `NX_TUI ??= \"false\"`. Drop."],
  FORCE_COLOR: ["runner-noise", "Colour off for the VS Code terminal / captured logs; read by Nx and Node. A dashboard that renders ANSI does not need it. Drop."],
  NX_PLUGIN_NO_TIMEOUTS: ["runner-noise", "Only in .claude/launch.json (Nx plugin timeouts off for a slow cold graph). Drop."],
  NX_WORKSPACE_ROOT_PATH: ["ticket-sandbox", "Points Nx at a ticket-local workspace (own nx.json). Exists only to run ticket probes; drops with the ticket rows."],
  NX_WORKSPACE_DATA_DIRECTORY: ["ticket-sandbox", "Private Nx graph/daemon state per ticket run. Drops with the ticket rows; a global `private nx state` axis is the dashboard's job (it already isolates graph state)."],
  NX_CACHE_DIRECTORY: ["ticket-sandbox", "Private Nx cache per ticket run. Drops with the ticket rows."],
  NX_SKIP_NX_CACHE: ["global-axis", "Cache policy axis (`skip local cache`); same meaning as `--skip-nx-cache`. Read by Nx (`command-line-utils`)."],
  NX_SKIP_REMOTE_CACHE: ["global-axis", "Cache policy axis (`skip remote cache`); same meaning as `--skip-remote-cache`."],
  SEMIO_TEST_LEVEL: ["global-axis", "Test level quick|long|exhaustive; read by the shared test harness (`🧪️tests/🎚️config` files). Same axis as the positional `quick|long|exhaustive` after `--` and the `test-quick|test-long|test-exhaustive` targets."],
  NEXTEST_SUCCESS_OUTPUT: ["global-axis", "cargo-nextest success output (`immediate|final`); read by nextest itself, not by repo code."],
  CARGO_BUILD_JOBS: ["global-axis", "Cargo parallelism cap (1|2) used as a machine-load guard; read by cargo."],
  SEMIO_BUILD_MODE: ["global-axis", "Build mode `ship` (release-like wasm/site builds); read by the wgpu server config and the wasm build scripts. Always set together with both cache-skip variables."],
  SEMIO_BUILD_BUDGET_MS: ["global-axis", "Per-build wall-clock budget (30 min / 60 min); read by the build scripts."],
  SEMIO_RENDERER: ["global-axis", "Renderer react|wgpu; read by the dev server/builder. Registry-bound for playgrounds."],
  SEMIO_PLUGIN: ["playground-binding", "Playground variant id; fully determined by the playground registry entry (generator `playgroundDevEnv`)."],
  SEMIO_APP: ["playground-binding", "Pinned app coordinate; fully determined by the playground registry entry."],
  S_OS_PORT: ["playground-binding", "The only port variable any playground dev server binds (generator `playgroundDevEnv`); value = registry `ports.react|wgpu` or `userPorts`."],
  SEMIO_DEFAULT_EXAMPLE: ["target-config", "Initial example of a playground (`hexagonal-mushroom-column`, `capsule-dream`); read by the wgpu server. Axis `example` of a playground."],
  PLAYGROUND_LOCKED_EXAMPLE_ID: ["target-config", "Locks a playground to one example; read by the playground host. Axis `example` (locked)."],
  SEMIO_APP_ROLE: ["target-config", "Opens the playground as `viewer` instead of editor. Axis `app role` of a playground."],
  PRINT_NATIVE_GRAMMAR_PHASE: ["target-config", "Phase selector of the native chart-grammar test (`@semio-tech/print:test-native-grammar`); the values are the owner's own phase list."],
  PRINT_NATIVE_GEO_PALETTE_PHASE: ["target-config", "Sub-phase of the same chart-grammar test."],
  PRINT_NATIVE_GEO_PLANAR_PHASE: ["target-config", "Sub-phase of the same chart-grammar test."],
  PRINT_NATIVE_GEO_PLANAR_SOURCE: ["target-config", "Source language selector of the same chart-grammar test."],
  PRINT_NATIVE_DIAGRAM_FAMILY: ["target-config", "Diagram family selector of the same chart-grammar test."],
  STORYBOOK_PORT: ["port", "Storybook dev server port (6010/6011); read by the storybook runner."],
  OS_HUB_PORT: ["port", "Hub port 8787; read by the hub bootstrap (Rust)."],
  PROCTOR_PORT: ["port", "Proctor port 8791 (8793 for the second instance)."],
  PETS_STORIES_PORT: ["port", "Pets stories port."],
  TEACHING_ARCHITECTURE_QUIZ_PORT: ["port", "Architecture quiz port; the owner's `📋️project.json` options.env and `package.json` already declare 6061 for `dev` and `dev-site`, the rows only repeat it."],
  SEMIO_TECH_PLAY_PORT: ["port", "semio-tech play site port."],
  MIT_BESTAND_DEMONSTRATOR_PORT: ["port", "Demonstrator port."],
  PRAESENTATION_PROJEKTETAGE_PORT: ["port", "Port variable named by the package's own `package.json` (`\"env\": \"PRAESENTATION_PROJEKTETAGE_PORT\"`, default 6050): the owner already declares it, the launch row only repeats the value."],
  CAD_JS_RENDERER_PLAY_PORT: ["port", "Legacy per-app port variable: the cad package manifest still sets 6041 while the two seed rows set 6020/6120 and the dev server binds `S_OS_PORT`: stale knob (conflicting values)."],
  PUZZLE_3D_PLAY_PORT: ["port", "Legacy per-app port variable (not read in tracked code): dead knob."],
  PUZZLE_5D_PLAY_PORT: ["port", "Legacy per-app port variable (not read in tracked code): dead knob."],
  SHOOTING_PLAY_PORT: ["port", "Legacy per-app port variable (not read in tracked code): dead knob."],
  CLIENT_PORT: ["port", "MCP inspector client port (6274); read by the inspector, not by the repo."],
  SERVER_PORT: ["port", "MCP inspector proxy port (6277); read by the inspector, not by the repo."],
  SEMIO_TEST_ARTIFACT_DIR: ["output-location", "Where a test writes its receipts (read by the shared test harness). Every row that sets it is ticket-scoped: a literal ticket path, or an `${input:…Artifacts}` whose default is a ticket path. An output-location concern for the dashboard's run directory, not a per-row string."],
  SEMIO_TEST_ARTIFACTS_DIR: ["output-location", "Plural spelling of the same variable (a few files read it, only ticket rows set it): inconsistent name."],
  SEMIO_TEST_ARTIFACTS_ROOT: ["output-location", "Third spelling, not read in tracked code (ticket rows only): dead knob."],
  SEMIO_TEST_OUTPUT_SCOPE: ["output-location", "Scopes test output below a root; read by the test plugin (`🧪️test/🟦️.ts`)."],
  SEMIO_TEST_RETAIN_ARTIFACTS: ["output-location", "Keep artifacts after the run; not read in tracked code (ticket probes)."],
  SEMIO_TICKET_DIR: ["output-location", "Ticket directory handed to ticket-owned e2e scripts; ticket-scoped by nature."],
  OS_HUB_DATA: ["service-config", "Hub data directory (per backend); read by the hub bootstrap."],
  OS_HUB_URL: ["service-config", "Hub URL for the admin UI."],
  S_HUB_URL: ["service-config", "Hub URL the `s` shell joins; part of the hub+shell compounds and the multi-user rows."],
  S_DATA_DIR: ["service-config", "Per-user local data directory of the `s` shell (user slots)."],
  S_LOCAL_ONLY: ["service-config", "Local-only mode of the `s` shell (no hub)."],
  OS_MCP_HUB_ORIGIN: ["service-config", "Hub origin for the os-mcp acceptance gates (bound to `${input:acceptanceHubUrl}`)."],
  S_OS_MCP_LIVE_SHELL_URL: ["service-config", "Shell URL for the os-mcp acceptance gates (bound to `${input:acceptanceServeUrl}`)."],
  S_OS_MCP_LIVE_LOCALE: ["service-config", "Locale `en|de` of the os-mcp acceptance gates."],
  OS_HUB_ADMIN_CAPABILITY_FILE: ["service-config", "Hub admin capability file (bound to an input)."],
  MCP_AUTO_OPEN_ENABLED: ["service-config", "MCP inspector auto-open off; read by the inspector, not by the repo."],
  MCP_PROXY_AUTH_TOKEN: ["secret-like", "Literal dev token `repo-mcp-token` committed in the launch file; must not be copied into the canonical registry as a value."],
  PROCTOR_DATA: ["service-config", "Proctor data directory (.claude only)."],
  TEACHING_ARCHITECTURE_QUIZ_WATCH: ["service-config", "Quiz watch switch (.claude only)."],
  PETS_MENAGERIE: ["target-config", "Which pets menagerie the stories server loads."],
  SEMIO_REPO_IMPLEMENTATION: ["target-config", "Selects the `go` implementation of the repo client (read by the dashboard command-tree and the container lifecycle)."],
  GOWORK: ["toolchain", "Go workspace file for the Go repo client; read by the go toolchain."],
  CARGO_TARGET_DIR: ["toolchain", "Private cargo target dir (ticket probes only); read by cargo and by repo scripts."],
  CARGO_BUILD_BUILD_DIR: ["toolchain", "Private cargo build dir (ticket probes only)."],
  CARGO_INCREMENTAL: ["toolchain", "Incremental compilation off for one probe; read by cargo."],
  RUST_BACKTRACE: ["toolchain", "Backtraces on; read by Rust."],
  PLAYWRIGHT_BROWSERS_PATH: ["toolchain", "Shared Playwright cache; `repoToolCacheEnv()` already defaults it (`PLAYWRIGHT_BROWSERS_PATH ??=`). Drop."],
  SEMIO_CARGO_PREPARATION_TIMING: ["target-config", "Prints cargo preparation timings; read by the cargo preparation scripts."],
  SEMIO_DEBUG_CLOSE_PHASE: ["target-config", "Debug switch of one process3d test; read by the plugin crate."],
  SEMIO_EARLY_SEED_CONTROL: ["unread-probe-knob", "Ticket probe knob; no reader outside the ticket scripts."],
  SEMIO_EARLY_SEED_RED_ATTEMPT: ["unread-probe-knob", "Ticket probe knob; no reader outside the ticket scripts."],
  SEMIO_ALIAS_PLAN_CONTROL: ["unread-probe-knob", "Ticket probe knob; no reader outside the ticket scripts."],
  SEMIO_RUNTIME_SOURCE_CONTROL: ["unread-probe-knob", "Ticket probe knob; no reader outside the ticket scripts."],
};
function analyzeEnv() {
  const rowsByKey = new Map<string, Rec[]>();
  for (const r of RECS) for (const k of uniq([...Object.keys(r.env), ...Object.keys(r.envInline)])) { if (!rowsByKey.has(k)) rowsByKey.set(k, []); rowsByKey.get(k)!.push(r); }
  const claudeKeys = new Map<string, number>();
  for (const c of CLAUDE.configurations as Json[]) for (const k of Object.keys(c.env ?? {})) claudeKeys.set(k, (claudeKeys.get(k) ?? 0) + 1);
  const norm = (v: string): string => v.replace(/\.?🧬semio\/🦑️repo\/🎫️tickets\/[^ '"]*/g, "<ticket-path>").replace(/^\/Users\/[^/]+\/Documents\/semio/, "${workspaceFolder}");
  const keys = uniq([...rowsByKey.keys(), ...claudeKeys.keys()]).sort();
  const table = keys.map((key) => {
    const rows = rowsByKey.get(key) ?? [];
    const values = rows.map((r) => norm(r.env[key] ?? r.envInline[key] ?? ""));
    const j = ENV_JUDGEMENT[key] ?? (isPortVar(key) ? (["port", "Port variable."] as [EnvKind, string]) : (["target-config", "UNCLASSIFIED"] as [EnvKind, string]));
    const ev = ENV_EVIDENCE[key];
    return { key, rows: rows.length, nonTicketRows: rows.filter((r) => !r.ticketScoped).length, ticketRows: rows.filter((r) => r.ticketScoped).length, claudeEntries: claudeKeys.get(key) ?? 0, inlineInCommand: rows.filter((r) => key in r.envInline).length, distinctValues: uniq(values).length, topValues: tally(values, (v) => v).slice(0, 4).map(([v, n]) => `${v} (${n})`), kind: j[0], judgement: j[1], filesMentioning: ev?.filesMentioning, filesReading: ev?.filesReading, readSites: ev?.readSites ?? [], nxInternalReaders: ev?.nxInternalReaders ?? [], inClasses: tallyObj(rows, (r) => r.class), unclassified: j[1] === "UNCLASSIFIED" };
  }).sort((a, b) => b.rows - a.rows || a.key.localeCompare(b.key));
  const noiseRows = RECS.filter((r) => Object.keys(r.envNoise).length > 0);
  const onlyNoise = RECS.filter((r) => Object.keys(r.envNoise).length > 0 && Object.keys(r.envSemantic).length === 0 && Object.keys(r.envSandbox).length === 0);
  return {
    distinctKeys: keys.length, byKind: tallyObj(table, (t) => t.kind), rowsByKind: tallyObj(RECS.flatMap((r) => uniq([...Object.keys(r.env), ...Object.keys(r.envInline)]).map((k) => ENV_JUDGEMENT[k]?.[0] ?? (isPortVar(k) ? "port" : "target-config"))), (k) => k),
    rowsWithAnyEnv: RECS.filter((r) => Object.keys(r.env).length + Object.keys(r.envInline).length > 0).length, rowsWithNoiseEnv: noiseRows.length, rowsWithOnlyNoiseEnv: onlyNoise.length, rowsWithOnlyNoiseEnvNonTicket: onlyNoise.filter((r) => !r.ticketScoped).length,
    rowsWithSandboxEnv: RECS.filter((r) => Object.keys(r.envSandbox).length > 0).length, rowsWithSandboxEnvNonTicket: RECS.filter((r) => Object.keys(r.envSandbox).length > 0 && !r.ticketScoped).length,
    unclassified: table.filter((t) => t.unclassified).map((t) => t.key), noiseRowsByWrapper: tallyObj(noiseRows, (r) => r.wrapper ?? r.runner), scanned: Object.values(ENV_EVIDENCE)[0]?.scannedFiles ?? 0, table,
  };
}
const ENV = analyzeEnv();
//#endregion

//#region 10. recommendation totals & registry size
function analyzeTotals() {
  const rec = tally(RECS, (r) => r.recommendation);
  const matrix = tallyObj(RECS, (r) => `${r.class} | ${r.family} | ${r.recommendation}`);
  const keepRows = RECS.filter((r) => r.recommendation.startsWith("keep"));
  const presetKey = (r: Rec): string => JSON.stringify([r.runner, r.project ?? r.execProjects ?? r.tool ?? r.script, r.target, r.nxConfiguration, r.residualArgs, r.residualEnv.map((k) => `${k}=${r.env[k] ?? r.envInline[k]}`), r.family === "playground-dev/seed-variant" ? r.name : ""]);
  const keepPresets = uniq(keepRows.map(presetKey));
  const keepByFamily = tallyObj(keepRows, (r) => r.family);
  const keepPresetsByFamily = tallyObj(uniq(keepRows.map((r) => `${r.family}\u0000${presetKey(r)}`)).map((s) => s.split("\u0000")[0]!), (f) => f);
  const axesUsed = FAMILIES.axesNonTicket.map((a) => ({ axis: a.axis, rows: a.rows, baseTargets: a.baseTargets, kind: a.baseTargets >= 3 ? "global" : "target-specific" }));
  const axisCatalogue = uniq([...FAMILIES.axesNonTicket.filter((a) => a.baseTargets >= 3 && !["port", "test-filter", "test-scope", "test-files"].includes(a.axis)).map((a) => a.axis), "user-slot", "example", "app-role"]);
  const promptRows = RECS.filter((r) => r.recommendation === "needs an input prompt");
  const promptInputs = uniq(promptRows.flatMap((r) => r.inputs));
  const genericPrompts = uniq(promptRows.flatMap((r) => (r.family.startsWith("test-selection") ? [r.selection.filter.length ? "test-filter" : "", r.selection.files.length ? "test-files" : "", r.selection.scope.length && !r.selection.filter.length && !r.selection.files.length ? "test-scope" : ""] : r.family === "bun-test-file" ? ["bun-test-file"] : [])).filter(Boolean));
  const toolRows = RECS.filter((r) => r.class === "tool-launcher");
  const retained = RECS.filter((r) => !r.recommendation.startsWith("drop"));
  const vscodeCompounds = (LAUNCH.compounds ?? []).length;
  const compoundRows = RECS.filter((r) => r.recommendation === "express as a compound");
  const compounds = vscodeCompounds + compoundRows.length;
  const keepBaseTargets = uniq(keepRows.map(baseTargetId));
  const registry = {
    declaredPresets: keepPresets.length, keepBaseTargets: keepBaseTargets.length, keepRows: keepRows.length, vscodeCompounds, nxRunManyCompounds: compoundRows.length, compounds, globalAxes: axisCatalogue.length, inputPrompts: promptInputs.length + genericPrompts.length,
    handMaintainedEntries: keepPresets.length + compounds + axisCatalogue.length + promptInputs.length + genericPrompts.length,
    axisCatalogue, promptInputs, genericPrompts, keepPresetsByFamily, keepByFamily,
    rowsRetainedAsSomething: retained.length, rowsDropped: RECS.length - retained.length,
  };
  return { total: RECS.length, byRecommendation: Object.fromEntries(rec), matrix, axesUsed, registry, toolRows: toolRows.map((r) => ({ name: r.name, family: r.family, command: r.command.slice(0, 90) })) };
}
const TOTALS = analyzeTotals();
//#endregion

//#region 10b. generator verification & skip-directory collision
async function verifyGenerator() {
  const policySkip = new Set<string>(SEED.projectLaunchers?.skipDirectories ?? []);
  const skippedNxProjects = [...MAIN.projects.values()].filter((p) => p.source === "manifest" && p.root.split("/").some((seg) => policySkip.has(seg))).map((p) => ({ project: p.name, root: p.root, skippedBecause: p.root.split("/").find((seg) => policySkip.has(seg)) }));
  const result: Json = { requested: flag("verify-generator"), skippedNxProjects };
  if (!flag("verify-generator")) return result;
  const gen = await import(join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts"));
  const readSeedText = (path: string): string => (path === ".vscode/🧩️launch.seed.jsonc" ? rawTexts.seed : readFileSync(join(ROOT, path), "utf8"));
  const view = { entries: (path: string) => readdirSync(join(ROOT, path), { withFileTypes: true }).map((e) => ({ name: e.name, nodeKind: e.isSymbolicLink() ? "symlink" : e.isDirectory() ? "directory" : "file" })), readText: readSeedText };
  const t0 = Date.now();
  const targets = gen.declaredProjectTargets(ROOT, view);
  const rendered: string = gen.generateLaunchJson(ROOT, PLAYGROUNDS, targets, readSeedText);
  const withoutProjects: string = gen.generateLaunchJson(ROOT, PLAYGROUNDS, [], readSeedText);
  Object.assign(result, { renderedBytesIdenticalToLaunchFile: rendered === rawTexts.launch, renderedConfigurations: JSON.parse(rendered).configurations.length, withoutProjectLaunchersConfigurations: JSON.parse(withoutProjects).configurations.length, structuralBoundaryMatches: JSON.parse(withoutProjects).configurations.length === BEFORE_PROJECT, generatorProjects: targets.length, generatorDeclaredTargets: targets.reduce((a: number, t: Json) => a + t.targets.length, 0), elapsedMs: Date.now() - t0 });
  return result;
}
const GENERATOR_CHECK = await verifyGenerator();
//#endregion

//#region 11. output
const INVENTORY = {
  meta: {
    generatedBy: "launch-inventory.ts", generatedAt: new Date().toISOString(), source: SOURCE, repoRoot: ROOT, files: SOURCE_META, liveFilesIdenticalToSource: driftFromLive, timingMs: TIMING,
    counts: { configurations: RECS.length, compounds: (LAUNCH.compounds ?? []).length, inputs: (LAUNCH.inputs ?? []).length, seedConfigurations: seedItems.length, claudeConfigurations: (CLAUDE.configurations as Json[]).length, playgrounds: PLAYGROUNDS.length },
    graph: { used: GRAPH.file, computedAt: GRAPH.computedAt, mtime: GRAPH.mtime, nodes: GRAPH.nodes.size, copies: GRAPH_COPIES.map((c) => ({ file: c.file, computedAt: c.computedAt, mtime: c.mtime, nodes: c.nodes.size })) },
    manifestWalk: { directories: MAIN.directories, nxManifests: MAIN.manifests, plainProjectJson: MAIN.plainProjectJson, cargoOnlyProjects: MAIN.cargoOnly, projects: MAIN.projects.size, declaredTargets: [...MAIN.projects.values()].reduce((a, p) => a + p.targets.size, 0), duplicates: MAIN.duplicates, invalid: MAIN.invalid },
    warnings: walkWarnings, nxFacts: SNAPSHOT_FACTS ? `frozen in snapshot (${FACTS_PATH.replace(ROOT + "/", "")})` : "read live",
  },
  records: RECS.map((r) => ({
    index: r.index, name: r.name, group: r.group, order: r.order, provenance: r.provenance, seedIndex: r.seedIndex, placeholder: r.placeholder,
    class: r.class, family: r.family, runner: r.runner, wrapper: r.wrapper, project: r.project ?? r.projectInput ?? r.execProjects, projectIsInput: !!r.projectInput, target: r.target, nxConfiguration: r.nxConfiguration,
    command: r.command, cwd: r.cwd, nxFlags: r.nxFlags, targetArgs: r.targetArgs, passthru: r.passthru, execCommand: r.execCommand, script: r.script, tool: r.tool,
    axes: r.axes, axisVia: r.axisVia, selection: r.selection, residualArgs: r.residualArgs, residualEnv: r.residualEnv,
    env: r.envSemantic, envNoise: r.envNoise, envSandbox: r.envSandbox, envInline: r.envInline,
    ready: r.ready ? { action: r.ready.action, pattern: r.ready.pattern, uriFormat: r.ready.uriFormat } : null, inputs: r.inputs,
    ticketScoped: r.ticketScoped, ticketReasons: r.ticketReasons, tickets: r.tickets, ticketStatus: r.ticketStatus,
    existence: r.existence, refsMissing: r.refsMissing, deadReasons: r.deadReasons, dead: r.deadReasons.length > 0,
    recommendation: r.recommendation,
  })),
  generatorCheck: GENERATOR_CHECK, seed: SEED_ANALYSIS, duplicates: DUPLICATES, compounds: COMPOUNDS, inputs: INPUTS, serverReady: READY, families: FAMILIES, tickets: TICKETS, dead: DEAD, presentation: PRESENTATION, claude: CLAUDE_ANALYSIS, env: ENV, totals: TOTALS,
  classes: tallyObj(RECS, (r) => r.class), classesNonTicket: tallyObj(NON_TICKET, (r) => r.class), classFamilyCounts: tallyObj(RECS, (r) => `${r.class} / ${r.family}`),
};
mkdirSync(GEN, { recursive: true });
if (!flag("no-write")) writeFileSync(join(GEN, "inventory.json"), JSON.stringify(INVENTORY, null, 1));
//#endregion

//#region 12. markdown report
const esc = (s: unknown): string => String(s ?? "").replaceAll("|", "\\|").replaceAll("\n", " ");
const C = (s: unknown): string => "`" + String(s ?? "").replaceAll("`", "'") + "`";
const short = (s: string, n = 80): string => (s.length > n ? s.slice(0, n - 1) + "…" : s);
const pct = (a: number, b: number): string => (b === 0 ? "-" : `${((100 * a) / b).toFixed(1)}%`);
const table = (head: string[], rows: unknown[][]): string => [`| ${head.join(" | ")} |`, `| ${head.map(() => "---").join(" | ")} |`, ...rows.map((r) => `| ${r.map(esc).join(" | ")} |`)].join("\n");
const kv = (o: Record<string, number>): string => Object.entries(o).map(([k, v]) => `${k} ${v}`).join(", ");
const list = (items: string[], max = 6): string => (items.length <= max ? items.join(", ") : `${items.slice(0, max).join(", ")}, +${items.length - max} more`);
const rel = (p: string): string => p.replace(GEN.replace(ROOT + "/", "") + "/", "<generated>/");
const T_REL = TICKET_DIR.replace(ROOT + "/", "");

function sectionHeader(): string {
  const f = SOURCE_META;
  const drift = Object.entries(driftFromLive).filter(([, same]) => !same).map(([k]) => k);
  return [
    "# Launch inventory: `.vscode/launch.json`, its seed and `.claude/launch.json`",
    "",
    `Read-only audit for ticket \`DASHBOARD-LAUNCH-COCKPIT\`. Every number below is computed by \`${T_REL}/launch-inventory.ts\`; the machine-readable result is \`${T_REL}/🗑️generated/launch-inventory/inventory.json\` (one record per configuration plus all summaries).`,
    "",
    "Reproduce (repository root): `bun \"" + T_REL + "/launch-inventory.ts\" --verify-generator` (reads the snapshot; `--source=live` reads the live files; `--snapshot-live` refreshes the snapshot; `--skip-env-scan` skips the 8 s repository scan for environment readers).",
    "",
    table(["Input", "Bytes", "sha256", "Content"], [
      [C(".vscode/launch.json"), f.launch.jsonc.bytes, f.launch.sha256.slice(0, 16) + "…", `${LAUNCH.configurations.length} configurations, ${(LAUNCH.compounds ?? []).length} compounds, ${(LAUNCH.inputs ?? []).length} inputs`],
      [C(".vscode/🧩️launch.seed.jsonc"), f.seed.jsonc.bytes, f.seed.sha256.slice(0, 16) + "…", `${seedItems.length} configuration items (${seedObjects.length} objects, ${seedPlaceholders.length} placeholders), ${(SEED.compounds ?? []).length} compounds, ${(SEED.inputs ?? []).length} inputs, ${Object.keys(devLaunchers).length} devLaunchers, projectLaunchers policy`],
      [C(".claude/launch.json"), f.claude.jsonc.bytes, f.claude.sha256.slice(0, 16) + "…", `${(CLAUDE.configurations as Json[]).length} preview-server entries`],
      [C("…/📇️registry/🤖️generated/🚀️playgrounds.json"), f.playgrounds.jsonc.bytes, f.playgrounds.sha256.slice(0, 16) + "…", `${PLAYGROUNDS.length} playground variants (the generator's second input)`],
    ]),
    "",
    `Source used for this report: **${SOURCE}**${SOURCE === "snapshot" ? ` (copies taken at the start of the audit under \`<generated>/snapshot/\`; other developers keep regenerating the live files, so the live counts drift by a few rows)` : ""}. Live files identical to the used source: ${Object.entries(driftFromLive).map(([k, same]) => `${k} ${same ? "yes" : "no"}`).join(", ")}${drift.length ? ` (the coordinator's 4583/3511 were measured on an earlier revision of the same files)` : ""}.`,
    "",
    `JSONC: all three files parse with \`Bun.JSONC.parse\`; the scan found ${Object.entries(f).map(([k, v]) => `${k}: ${v.jsonc.lineComments} line comments, ${v.jsonc.blockComments} block comments, ${v.jsonc.trailingCommas} trailing commas`).join("; ")}. The seed is JSONC by contract (the generator parses it with \`Bun.JSONC.parse\`) but contains no comment today; the launch file is plain JSON.`,
  ].join("\n");
}

function sectionSummary(): string {
  const cls = tally(RECS, (r) => r.class);
  const dead = DEAD;
  const t = TOTALS;
  const rec = t.byRecommendation;
  const drops = Object.entries(rec).filter(([k]) => k.startsWith("drop")).reduce((a, [, n]) => a + n, 0);
  const reg = t.registry;
  return [
    "## 1. Result in numbers",
    "",
    `- **${RECS.length} configurations**, all \`node-terminal\`/\`launch\` (${pct(RECS.filter((r) => r.group !== null).length, RECS.length)} carry \`presentation\`, ${PRESENTATION.rowsWithoutPresentation} do not), ${(LAUNCH.compounds ?? []).length} compounds, ${(LAUNCH.inputs ?? []).length} inputs; names are unique (${DUPLICATES.namesUnique}); ${DUPLICATES.duplicateCommandEnvGroups} groups of rows share command+env (${DUPLICATES.duplicateRows} rows, i.e. ${DUPLICATES.duplicateRows - DUPLICATES.duplicateCommandEnvGroups} surplus rows).`,
    `- **${TICKET.length} rows (${pct(TICKET.length, RECS.length)}) are ticket-scoped**, all of them hand-written seed rows, all referencing only ${TICKETS.referencedTickets} tickets, **all of which are still \`open\`** (${TICKETS.byStatus.closed ?? 0} closed). One ticket (\`CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT\`) owns ${TICKETS.tickets[0]?.rows ?? 0} of them.`,
    `- **${dead.total} rows are dead** (${dead.nonTicket} outside tickets, ${dead.ticketScoped} ticket-scoped): ${dead.nxVerdicts["ticket-workspace-missing (ticket-local)"] ?? 0} rows run a ticket-local Nx workspace whose directory was deleted, ${dead.byKindNonTicket["missing-target"] ?? 0} non-ticket rows name an Nx target that no longer exists, ${dead.byKindNonTicket["missing-path"] ?? 0} non-ticket rows name a moved/missing file. No row names a missing Nx *project*.`,
    `- **${RECS.filter((r) => r.provenance === "seed-authored").length} rows are seed-authored, ${RECS.filter((r) => r.provenance.startsWith("playground")).length} are generated from the playground registry, ${RECS.filter((r) => r.provenance.startsWith("project-launcher")).length} are generated from Nx project targets** (exact attribution in section 2; regenerating from seed + registry + project walk reproduces the launch file byte for byte${GENERATOR_CHECK.requested ? `: ${GENERATOR_CHECK.renderedBytesIdenticalToLaunchFile ? "verified" : "NOT identical"} in this run` : ""}).`,
    `- **${rec["drop (redundant with Nx target discovery)"] ?? 0} plain/picker rows are redundant with Nx target discovery** and **${rec["drop (redundant with playground catalog discovery)"] ?? 0} playground rows are redundant with the playground catalog**; together with the ticket and dead rows **${drops} of ${RECS.length} rows (${pct(drops, RECS.length)}) are dropped**, ${RECS.length - drops} rows carry intent worth keeping.`,
    `- Those ${RECS.length - drops} rows reduce to a canonical registry of about **${reg.handMaintainedEntries} hand-maintained entries**: ${reg.declaredPresets} declared presets (${reg.keepBaseTargets} distinct base targets), ${reg.compounds} compounds (${reg.vscodeCompounds} VS Code compounds + ${reg.nxRunManyCompounds} \`nx run-many\` target groups), ${reg.globalAxes} global axes and ${reg.inputPrompts} input prompts (section 11).`,
    "",
    "### Classes (mutually exclusive, first match wins)",
    "",
    table(["Class", "Rows", "Share", "Definition"], cls.map(([c, n]) => [C(c), n, pct(n, RECS.length), CLASS_TEXT[c] ?? ""])),
    "",
    "",
    "### Surprises worth knowing before migrating",
    "",
    `- Every ticket row is a **seed** row; the generator never produces one. \`reconcile-launch-seed\` keeps adopting anything a developer adds to \`launch.json\`, so the seed grows by whole ticket test-matrices (the dominant ticket alone contributes ${TICKETS.tickets[0]?.rows ?? 0} rows, ${TICKETS.tickets[0]?.rowsWithMissingWorkspace ?? 0} of them pointing at a workspace that no longer exists).`,
    `- ${SEED_ANALYSIS.seedAuthored.byClass["plain-nx"] ?? 0} seed rows are plain \`nx run P:T\` rows the project-launcher walk would generate anyway; the generator even skips its own row when a seed row already runs that target.`,
    `- The Nx/VS Code workaround env (\`NX_DAEMON\`, \`NX_ISOLATE_PLUGINS\`, \`NX_CACHE_PROJECT_GRAPH\`, \`NX_TUI\`, \`FORCE_COLOR\`) sits on ${ENV.rowsWithNoiseEnv} rows and is already defaulted by the \`bun nx\` wrapper (section 6); the ticket sandbox variables (\`NX_WORKSPACE_ROOT_PATH\`, …) sit on ${ENV.rowsWithSandboxEnv} rows, **none** outside tickets.`,
    `- The cross-cutting axes have several spellings each: test level ${Object.keys(FAMILIES.axesNonTicket.find((a) => a.axis === "test-level")?.expressedAs ?? {}).length} (env var, positional after \`--\`, target-name suffix), cache policy ${Object.keys(FAMILIES.axesNonTicket.find((a) => a.axis === "cache-policy")?.expressedAs ?? {}).length} (two env vars, two Nx flags); section 5.1 lists them all.`,
    `- ${RECS.filter((r) => !r.ticketScoped && r.needsServe.length).length} non-ticket rows only work while another row's server runs (hub on 8787, \`s\` shell on 6070/6071) — they are compounds in disguise (section 5.4).`,
    `- All ${PLAYGROUNDS.length} playground ports match between registry, generated rows and \`.claude/launch.json\` (${CLAUDE_ANALYSIS.portMismatches.length} mismatches); the ${CLAUDE_ANALYSIS.attachOnly} \`attach\` entries there carry no command at all.`,
    "",
    `The coordinator's rough classes map as follows: *1788 ticket* -> \`ticket-scoped\` is ${TICKET.length} here because ${TICKETS.byReason["ticket-input"] ?? 0}+ rows reach a ticket only through an \`\${input:…Artifacts}\` whose default is a ticket path (reasons: ${kv(TICKETS.byReason)}); *1447 plain* -> \`plain-nx\` ${RECS.filter((r) => r.class === "plain-nx").length} plus ${RECS.filter((r) => r.class === "project-picker-family").length} project-picker rows (ticket rows and rows with noise-only env are classified before/with the plain ones here); *318 playground launchers* -> \`playground-dev\` ${RECS.filter((r) => r.class === "playground-dev").length} (${SEED_ANALYSIS.playgroundRows.total} generated + ${RECS.filter((r) => r.class === "playground-dev" && r.provenance === "seed-authored").length} seed variants; ${SEED_ANALYSIS.playgroundRows.withSemioPluginEnv} rows carry \`SEMIO_PLUGIN\`); *71 nx exec bun test* -> \`nx-exec-bun-test\` ${RECS.filter((r) => r.class === "nx-exec-bun-test").length} (non-ticket; the ${RECS.filter((r) => r.runner === "nx-exec" && r.ticketScoped).length} ticket-scoped \`nx exec\` rows are counted as tickets).`,
  ].join("\n");
}
const CLASS_TEXT: Record<string, string> = {
  "ticket-scoped": "Command, cwd, env or a used input default points into `🎫️tickets/…`, into a `🗑️generated` folder, or names a ticket-named Nx project.",
  "plain-nx": "`bun nx run <project>:<target>` with fixed project and no semantic env, no flags, no arguments, no input, no ready action (noise env allowed).",
  "nx-preset": "`nx run P:T` with Nx flags, passthrough arguments, semantic env, an input or a ready action.",
  "playground-dev": "`workspace:dev -- <playground variant>` with a ready action (generated from the playground registry or a seed variant).",
  "nx-exec-bun-test": "`nx exec --projects=P -- bun test <file>`.",
  "project-picker-family": "Generated `nx run ${input:projectTarget.<target>}:<target>` row (one per target name declared by at least 3 projects).",
  "tool-launcher": "Not an `nx run`: `nx run-many`, a `bun` script, an external tool, `workspace:dev -- mcp …`.",
  "nx-exec-other": "`nx exec` running something other than `bun test`.",
};

function sectionGenerator(): string {
  const S = SEED_ANALYSIS, P = S.projectLaunchers, D = S.devLaunchers, G = GENERATOR_CHECK;
  const prov = S.provenance as Record<string, number>;
  return [
    "## 2. The seed and the generator",
    "",
    "Pipeline (`📇️registry/🚀️launch/🟦️.ts`, run by `generate` in `📇️registry/📽️projection/🟦️.ts`; `reconcile-launch-seed` and `check-launch-seed` are the other two entry points):",
    "",
    "1. `readSeed` parses the seed with `Bun.JSONC.parse`, validates the container shapes with `🧱️placement` (configurations may only be rows or `@generated:<variant>:<kind>` strings, inputs only input objects; schema `🧬️schema/🧱️placement/🔣️.json`), splits off `devLaunchers` and `projectLaunchers`, and keeps the rest as the **skeleton**.",
    "2. **devLaunchers** (variant -> `{namePrefix, order, wgpuOrder?, env?, users?}`): for each entry the `@generated:<variant>:react|wgpu|users` placeholder inside the skeleton is replaced by a row whose command (`bun nx run workspace:dev -- <variant>`), env (`S_OS_PORT`, `SEMIO_PLUGIN`, `SEMIO_RENDERER`, `SEMIO_APP`) and `serverReadyAction` come **only** from the playground registry entry; the seed contributes the display name prefix, the `order`, optional extra env and the multi-user template (`users`: one row per registry `userPorts` slot, tokens `{N}`, `{PORT}`, `{EMAIL}`). A seed row can never drift from the variant it launches.",
    "3. `refreshDevLaunchNames` rewrites `🛠️dev…` names so their emoji match the plugin/artifact taxonomy folders (`🏷️name-prefix`: `<plugin folder><artifact folder>[<standard folder>]<subset folder>` + `⚛️react` | `🧊️wgpu🌐️wasm`; injective over variants, `launch-name-prefix` in a crate manifest overrides).",
    "4. **Synthesis**: every registry variant that has no row of that name yet gets a `⚛️react` and a `🧊️wgpu🌐️wasm` row appended (order `420 + index*0.01` unless curated). Then `refreshDevLaunchNames` again.",
    "5. **projectLaunchers** (policy in the seed; `declaredProjectTargets` walks every `📋️project.json`, skipping hidden dirs and `skipDirectories`): a target name declared by >= `familyMinimumProjects` projects becomes one **family row** `<class emoji><target>📋️` with a `${input:projectTarget.<target>}` pickString input (options = the owning projects); every other declared (project, target) not already run by some `nx run P:T` row gets a **target row** `<class emoji><target><project label>`. The class (`dev` 🛠️ / `build` 📦️ / `gate` ⚖️ / fallback `run` ▶️) is the first target-name token listed by a class; the label is the shortest unique trailing run of the project's path segments (language folders shortened to 🦀️/🟦️/🐍️).",
    "6. The result is re-serialised with `JSON.stringify(…, null, 2)`. `reconcile-launch-seed` moves rows that exist only in `launch.json` into the seed as the bytes they were written in (this is how hand-added rows, including every ticket row, end up in the seed).",
    "",
    `**Verification.** ${G.requested ? `Re-running the real generator on the audited seed + playground registry + current project tree yields ${G.renderedConfigurations} configurations and is **${G.renderedBytesIdenticalToLaunchFile ? "byte-identical" : "NOT byte-identical"}** to the audited launch file; without project launchers it yields ${G.withoutProjectLaunchersConfigurations} rows, ${G.structuralBoundaryMatches ? "exactly the boundary used below" : "NOT the boundary used below"} (generator walk: ${G.generatorProjects} projects, ${G.generatorDeclaredTargets} declared targets).` : "Not requested in this run (pass `--verify-generator`)."} The attribution below is structural (seed order + placeholder expansion + appended synthesized rows + appended project rows) and does not depend on that run.`,
    "",
    table(["Provenance", "Rows", "How it is produced"], [
      ["seed-authored", prov["seed-authored"], `${S.configurations.objects} object rows in the seed \`configurations\`, copied verbatim (names are rewritten only for \`🛠️dev…\` rows that map to a playground)`],
      ["playground-seed-placeholder", prov["playground-seed-placeholder"], `${S.configurations.placeholderKinds.react} \`@generated:<v>:react\` + ${S.configurations.placeholderKinds.wgpu} \`:wgpu\` + ${S.playgroundRows.userSlotRows} rows of 1 \`:users\` placeholder (template ${C(D.usersTemplates[0]?.namePrefixPattern)}, ${C(D.usersTemplates[0]?.emailPattern)}, env \`S_HUB_URL\` + \`S_DATA_DIR\`)`],
      ["playground-synthesized", prov["playground-synthesized"], `${D.uncuratedVariants} registry variants without a seed entry x 2 renderers + ${prov["playground-synthesized"] - 2 * D.uncuratedVariants} curated variants that lack a \`wgpuOrder\` (react-only seed entry): appended after the skeleton`],
      ["project-launcher-family", prov["project-launcher-family"], `${P.familyTargetNames} target names declared by >= ${P.policy.familyMinimumProjects} projects (covering ${P.pairsInFamilyTargets} of the ${P.declaredPairs} declared (project, target) pairs) -> ${P.projectInputs} generated \`projectTarget.*\` inputs`],
      ["project-launcher-target", prov["project-launcher-target"], `${P.declaredPairs} declared pairs - ${P.pairsInFamilyTargets} in family targets - ${P.pairsCoveredByExistingRows} already run by an earlier row = ${P.pendingPairs} pending pairs`],
      ["**total**", RECS.length, `seed items ${S.configurations.total} -> ${BEFORE_PROJECT} rows before project launchers -> ${RECS.length}`],
    ]),
    "",
    "Provenance x class (rows):",
    "",
    table(["Provenance", ...[...new Set(RECS.map((r) => r.class))].map((c) => c)], Object.keys(prov).map((pv) => [pv, ...[...new Set(RECS.map((r) => r.class))].map((c) => RECS.filter((r) => r.provenance === pv && r.class === c).length || "")])),
    "",
    `Duplicates: names are unique; ${DUPLICATES.duplicateCommandEnvGroups} groups of rows share command+env (${DUPLICATES.duplicateRows} rows, ${DUPLICATES.duplicateRows - DUPLICATES.duplicateCommandEnvGroups} surplus):`,
    "",
    table(["Command", "Rows (names)", "Classes"], DUPLICATES.groups.map((g) => [C(short(g.command, 80)), g.names.map((n) => C(short(n, 38))).join(" = "), g.classes.join(", ")])),
    "",
    "What the seed's metadata encodes:",
    "",
    `- \`devLaunchers\`: ${D.entries} entries (of ${D.registryVariants} registry variants; ${D.uncuratedVariants} variants have none), field frequency ${kv(D.fieldFrequency)}; \`order\` ${D.orderMin}..${D.orderMax}; \`env\` extras on ${D.withEnvExtras} entry, \`users\` on ${D.withUsers} entry (variant ${C(D.usersTemplates[0]?.variant)}). Per entry the seed owns only presentation (name prefix, order) — everything operational is registry-owned.`,
    `- \`projectLaunchers\`: ${P.policy.classes.length} classes (${P.policy.classes.map((c: Json) => `${c.id} ${c.emoji} -> ${c.group}, base ${c.orderBase}, ${c.tokens} tokens`).join("; ")}), fallback \`${P.policy.fallbackClass}\`, \`familyMinimumProjects\` ${P.policy.familyMinimumProjects}, family emoji ${P.policy.familyEmoji}, transparent segment ${C((P.policy.transparentSegments ?? []).join(","))}, language shortening ${C(Object.entries(P.policy.languageSegments ?? {}).map(([k, v]) => `${k}->${v}`).join(", "))}, skipDirectories ${C((P.policy.skipDirectories ?? []).join(", "))}. Output rows land in groups ${kv(P.rowsByGroup)}.`,
    `- The seed's ${S.seedAuthored.rows} object rows are by class: ${kv(S.seedAuthored.byClass)}; by \`presentation.group\`: ${kv(S.seedAuthored.byGroup)}. In particular **${S.seedAuthored.byClass["plain-nx"] ?? 0} seed rows are plain \`nx run P:T\`** that the project-launcher walk would otherwise generate itself (they win through the \`covered\` check and keep hand-chosen names/orders), and all ${S.seedAuthored.ticketScoped} ticket rows live here.`,
    `- Project-launcher naming is fully derivable: recomputing \`<class emoji><target><label>\` from (project path, target) reproduces ${PRESENTATION.projectLauncherDerivation.nameDerivable}/${PRESENTATION.projectLauncherDerivation.targetRows} target-row names, ${PRESENTATION.projectLauncherDerivation.orderDerivable}/${PRESENTATION.projectLauncherDerivation.targetRows} orders and ${PRESENTATION.projectLauncherDerivation.groupDerivable}/${PRESENTATION.projectLauncherDerivation.targetRows} groups, and ${PRESENTATION.projectLauncherDerivation.familyNameDerivable}/${PRESENTATION.projectLauncherDerivation.familyRows} family names, ${PRESENTATION.projectLauncherDerivation.familyOrderDerivable}/${PRESENTATION.projectLauncherDerivation.familyRows} family orders.`,
    `- **Side finding:** \`skipDirectories\` contains \`🎫️tickets\`, which also matches the repository's own module folder \`🔨️modules/🎫️tickets\`; ${GENERATOR_CHECK.skippedNxProjects.length} real Nx projects (${GENERATOR_CHECK.skippedNxProjects.map((p: Json) => C(p.project)).join(", ")}) therefore get no project-launcher row (the walk finds ${GENERATOR_CHECK.requested ? GENERATOR_CHECK.generatorProjects : P.declaredProjectsInWalk} projects, my manifest walk finds ${MAIN.manifests}).`,
  ].join("\n");
}

function sectionExistence(): string {
  const D = DEAD;
  const nonTicket = D.nonTicketList;
  const gm = INVENTORY_META_GRAPH();
  return [
    "## 3. Do the commands still exist?",
    "",
    `Method: every \`nx run P:T[:cfg]\` and \`nx exec --projects=P\` row is parsed with a shell tokenizer (quotes, inline \`VAR=value\` prefixes, \`--\` passthrough) and checked against three sources — (a) the declared manifests (walk of all \`📋️project.json\`/\`project.json\`: ${MAIN.manifests + MAIN.plainProjectJson} files, ${MAIN.projects.size - MAIN.cargoOnly} named projects with ${[...MAIN.projects.values()].reduce((a, p) => a + p.targets.size, 0)} declared targets, plus ${MAIN.cargoOnly} Cargo-only packages that the inference plugin turns into projects; walk of ${MAIN.directories} directories), (b) the cached Nx graph (\`${gm.used}\`, computed ${gm.computedAt}, ${gm.nodes} nodes; the older copy \`.nx/workspace-data/project-graph.json\` is from ${gm.copies[0]?.computedAt} but has identical node and target sets), (c) re-implemented inference rules of \`📚️library/🟨️.mjs\` (root \`router.register\` commands, Cargo \`build|check|test\`, component targets, leveled \`test-quick|long|exhaustive\`, print documents, playground-derived targets). Declared manifests explain every one of the graph's declared targets (0 declared targets missing from the graph) and rules + manifests explain all but ${GRAPH_ONLY_UNEXPLAINED} graph-only targets (\`nx-release-publish\` and two \`test:*\` colon targets), so the verdicts below do not depend on running Nx.`,
    "",
    table(["Verdict (workspace)", "Rows"], Object.entries(D.nxVerdicts)),
    "",
    `Rule-explained rows: ${kv(D.nxRules)}. Ticket-local workspaces (\`NX_WORKSPACE_ROOT_PATH\` or a ticket cwd): ${TICKETS.ticketLocalNxWorkspaces.length} distinct roots, ${TICKETS.ticketLocalNxWorkspaces.filter((w) => w.exists).length} still exist.`,
    "",
    `Path references: cwd, \`NX_WORKSPACE_ROOT_PATH\`, script and test-file arguments are *required* paths; cache/artifact/data env values are *outputs* and need not exist. Relative \`nx run P:T -- ../x.ts\` arguments are resolved against the project root first (that is Nx's \`cwd\`), then the workspace root. Missing required paths: ${kv(D.missingPathKinds)}.`,
    "",
    `**Dead rows: ${D.total}** = ${D.nonTicket} non-ticket + ${D.ticketScoped} ticket-scoped (${kv(D.byKind)}).`,
    "",
    `Non-ticket dead rows (${nonTicket.length}):`,
    "",
    table(["#", "Name", "Command", "Why dead"], nonTicket.map((r) => [r.index, r.name, short(r.command, 110), short(r.reasons.join("; "), 150)])),
    "",
    "Reading of the non-ticket causes: `@semio-tech/framework-rs` and `@semio-tech/repo-lib` renamed or removed their per-test targets (`test-fixture-ownership` -> `test-fixture-ownership-source`, `test-snapshot-sqlite-io` -> `test-snapshot-sqlite`, …), `framework-io-schema-rs:test-binding` no longer exists, the `ui-react` engagement-status test moved to `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📣️engagement-status/🟦️.tsx`, and the trinity shell row points to a fixture file that is not tracked anywhere. These eight go away with the rows; nothing needs migrating.",
    "",
    `Ticket-scoped dead rows: ${D.nxVerdicts["ticket-workspace-missing (ticket-local)"] ?? 0} rows share one deleted ticket-local workspace (${D.ticketMissingWorkspace.map(([p, n]) => `${C(rel(p))} x${n}`).join(", ")}), the remaining ${D.ticketScoped - (D.nxVerdicts["ticket-workspace-missing (ticket-local)"] ?? 0)} reference files that sit inside \`🗑️generated\` folders deleted at ticket cleanup (${D.missingPathKinds["inside a deleted 🗑️generated folder"] ?? 0} required references) or a missing script (${D.missingPathKinds["missing file"] ?? 0}).`,
  ].join("\n");
}
const INVENTORY_META_GRAPH = () => ({ used: GRAPH.file, computedAt: GRAPH.computedAt, nodes: GRAPH.nodes.size, copies: GRAPH_COPIES });
const GRAPH_ONLY_UNEXPLAINED = (() => { let n = 0; for (const p of MAIN.projects.values()) { const g = GRAPH.nodes.get(p.name); if (!g) continue; for (const t of g.targets.keys()) if (!p.targets.has(t) && !inferenceRule(p, t)) n++; } return n; })();

function sectionTickets(): string {
  const T = TICKETS;
  return [
    "## 4. Ticket-scoped rows (pollution)",
    "",
    `${T.ticketScopedRows} rows; reasons: ${kv(T.byReason)}. By provenance: ${kv(T.byProvenance)} (every one is a hand-written seed row — \`reconcile-launch-seed\` adopts whatever a developer adds to \`launch.json\`). By shape: ${kv(T.byFamily)}. By Nx entry point: ${kv(T.nxWrapper)} (the ${T.nxWrapper["direct-nx-js"] ?? 0} \`nx.js\` rows bypass the \`bun nx\` wrapper, which is why they repeat \`NX_DAEMON\`/\`NX_ISOLATE_PLUGINS\`).`,
    "",
    table(["Ticket", "Title", "Status", "Rows", "Rows whose required paths all exist", "Rows with missing paths", "Rows with deleted Nx workspace", "Required refs (missing)"], T.tickets.map((t) => [C(t.id), t.title ?? "", t.status, t.rows, t.rowsAllRequiredPathsExist, t.rowsWithMissingPaths, t.rowsWithMissingWorkspace, `${t.requiredReferences} (${t.requiredReferencesMissing})`])),
    "",
    `Ticket status is read from each \`🎫️ticket.json\`: ${kv(T.byStatus)}; rows by roll-up status ${kv(T.rowsByTicketStatus)}. **No referenced ticket is closed**, so "closed ticket" cannot be used as the drop criterion here — the rows are still dropped because they are one-off verification probes bound to a ticket folder, not developer entry points (the open tickets can keep their commands in their own folder).`,
    "",
    T.rowsWithoutTicketReference.length ? `Rows flagged ticket-scoped without any ticket reference (they only run into \`🗑️generated\`): ${T.rowsWithoutTicketReference.map((r) => `#${r.index} ${C(r.name)}`).join(", ")}.` : "",
    "",
    "Top shapes of the dominant ticket (examples): " + T.tickets.slice(0, 3).map((t) => `${C(t.id)}: ${list(t.examples.map((e) => C(e)), 3)}`).join("; ") + ".",
  ].join("\n");
}

const FAMILY_TEXT: Record<string, string> = {
  "nx-preset / test-axis-variant": "A `test*` target run with only cross-cutting axes (level, nextest output, cargo jobs, cache policy, ship build, task dependencies, output style). No selection.",
  "nx-preset / test-selection/filter": "A `test*` target with a test-name filter (`--filter-expr`/`-E`/bare names/`--testNamePattern`): one-off reproduction of a single test or test group, plus the axes above.",
  "nx-preset / test-selection/vitest-files": "A `test*` target of a TypeScript package with explicit test file paths (vitest `--run <file>`), plus axes.",
  "nx-preset / test-selection/scope": "A `test*` target with a cargo scope (`--lib`, `--test X`, `--features f`), no filter, plus axes.",
  "nx-preset / axis-variant": "A non-test target (build/describe/check/verify) run with only cross-cutting axes; mostly the `ship` build + both cache skips pair.",
  "nx-preset / print-native-phase": "`@semio-tech/print:test-native-grammar` with one `PRINT_NATIVE_*` selector per row (and `--families-only` on some).",
  "nx-preset / wgpu-native-launch": "`@semio-tech/framework-renderer-wgpu:native[-release] -- <playground variant>`: the wgpu-native renderer of a playground.",
  "nx-preset / dev-server-port": "A dev server with a `*_PORT` env and a `serverReadyAction` that opens the printed URL.",
  "nx-preset / prompted-argument": "Arguments that are `${input:…}` prompts (acceptance URLs, proctor handle/age/backup file).",
  "nx-preset / target-specific-arguments": "Genuinely target-specific arguments: sub-commands (`daemon start|attach`), backend names (`sqlite|postgres|neo4j`), serve URLs for acceptance matrices, `--hub`/`--locale`/`--tag` options.",
  "playground-dev / playground-dev/registry-generated": "Generated `workspace:dev -- <variant>` rows: two per registry variant (react, wgpu).",
  "playground-dev / playground-dev/user-slot": "Multi-user rows of the `s` shell: slot N of the registry `userPorts` with `S_HUB_URL` and `S_DATA_DIR`.",
  "playground-dev / playground-dev/seed-variant": "Seed variants of a playground: fixture argument, locked example, viewer role, per-app legacy port variable.",
  "nx-exec-bun-test / bun-test-file": "`nx exec --projects=P --excludeTaskDependencies --skip-nx-cache -- bun test <file>`: run one TypeScript test file with Nx's environment.",
  "nx-exec-other / nx-exec-other": "`nx exec` around another command (ship+cache axes on a bun script).",
  "tool-launcher / nx-run-many": "`nx run-many -t <target> -p <projects>` or `--target=<t> --args=…`: one target over a project set.",
  "tool-launcher / dev-tool/workspace-dev-mcp": "`workspace:dev -- mcp …` (MCP inspector, repo MCP proxy, os-mcp stdio/http).",
  "tool-launcher / external-tool": "External programs: gemini, kiro-cli, f3d, gitkraken, MCP inspector.",
  "tool-launcher / bun-script": "A `bun ./📜️script.ts dev mcp stdio client` row.",
};
const AXIS_TEXT: Record<string, string> = {
  "cache-policy": "Skip local / remote Nx cache. Expressed as `--skip-nx-cache`, `--skip-remote-cache` and the env pair `NX_SKIP_NX_CACHE`/`NX_SKIP_REMOTE_CACHE` (the three forms are synonyms).",
  "test-level": "quick | long | exhaustive. Expressed as `SEMIO_TEST_LEVEL`, a first positional after `--`, or the `test-<level>` target name.",
  "port": "Dev-server port. Registry-owned for playgrounds (`S_OS_PORT`); owner-declared per service otherwise (`*_PORT`). Belongs next to the ready action, not in a row.",
  "renderer": "react | wgpu (`SEMIO_RENDERER`), plus `wgpu-native` via the `native` target.",
  "test-filter": "Free-text test selection (`--filter-expr`, `-E`, bare names, `--testNamePattern`). One-off by nature: a prompt, never stored.",
  "task-dependencies": "`--excludeTaskDependencies`: run only the target, not its `dependsOn` chain.",
  "test-scope": "cargo scope (`--lib`, `--test X`, `--features f`, …).",
  "build-mode": "`SEMIO_BUILD_MODE=ship`; always together with the cache-policy pair.",
  "nextest-output": "`NEXTEST_SUCCESS_OUTPUT` immediate|final (cargo-nextest's own variable).",
  "print-phase": "`PRINT_NATIVE_*` phase/family selectors of one test target.",
  "cargo-jobs": "`CARGO_BUILD_JOBS` cap (1|2), a load guard.",
  "test-files": "Explicit test file paths (vitest).",
  "output-style": "`--output-style=stream|static` / `--outputStyle=stream`.",
  "build-budget": "`SEMIO_BUILD_BUDGET_MS` wall-clock budget (30 or 60 min).",
  "example": "`SEMIO_DEFAULT_EXAMPLE` / `PLAYGROUND_LOCKED_EXAMPLE_ID`: initial or locked example of a playground.",
  "app-role": "`SEMIO_APP_ROLE=viewer`.",
  "artifact-dir": "`SEMIO_TEST_ARTIFACT_DIR`-style output location (ticket rows only).",
};

function sectionFamilies(): string {
  const F = FAMILIES;
  const nonTicketDims = F.families;
  const out: string[] = [
    "## 5. Families and axes (non-ticket rows)",
    "",
    "Rows that are neither plain nor generated pickers were grouped by *base target* (`project:target[:configuration]`, or the `nx exec`/tool entry point) and then by the dimensions along which rows of one base target differ. A dimension is **cross-cutting** when it appears on at least 3 distinct base targets, otherwise **target-specific**.",
    "",
    "### 5.1 Cross-cutting axes",
    "",
    table(["Axis", "Rows", "Base targets", "Owner projects", "Expressed as", "Value domain", "Scope"], F.axesNonTicket.map((a) => [C(a.axis), a.rows, a.baseTargets, a.owners, Object.entries(a.expressedAs).map(([k, v]) => `${k} (${v})`).join(", "), a.axis === "port" || a.axis.startsWith("test-f") || a.axis === "test-scope" ? `${a.distinctValues} distinct values` : a.domain.slice(0, 5).map((d) => `${d.value} (${d.rows})`).join(", "), a.baseTargets >= 3 ? "global" : "target-specific"])),
    "",
    "What each axis means and where it should live:",
    "",
    ...F.axesNonTicket.map((a) => `- ${C(a.axis)} — ${AXIS_TEXT[a.axis] ?? ""}`),
    "",
    `The same axes also steer the ticket rows (ticket-row axis usage: ${F.axesTicket.slice(0, 8).map((a) => `${a.axis} ${a.rows}`).join(", ")}). Two expressions per axis coexist today (test level has ${Object.keys(F.axesNonTicket.find((a) => a.axis === "test-level")?.expressedAs ?? {}).length} forms, cache policy ${Object.keys(F.axesNonTicket.find((a) => a.axis === "cache-policy")?.expressedAs ?? {}).length}); a registry should store one canonical spelling and let each target map it (env var, flag or positional).`,
    "",
    "### 5.2 Families",
    "",
    table(["Family", "Rows", "Base targets", "Owner projects", "Varying dimensions", "Constant", "Ready action"], nonTicketDims.map((f) => [C(f.id), f.rows, f.baseTargets, f.owners.length <= 3 ? f.owners.map((o) => C(short(o, 38))).join(", ") : `${f.owners.length} projects (${f.topBaseTargets.slice(0, 2).map((b) => C(short(b.base, 34))).join(", ")}, …)`, f.axes.filter((a) => a.rows > 0).slice(0, 6).map((a) => `${a.axis}[${a.distinctValues}]`).join(", ") || (f.residualArgDomain.length ? `args[${f.residualArgDomain.length}+]` : "-"), f.constantProject.length ? `project ${C(f.constantProject[0])}` : f.constantTarget.length ? `target ${C(f.constantTarget[0])}` : "-", f.ready.length ? short(f.ready[0]!, 60) : "-"])),
    "",
  ];
  for (const f of nonTicketDims) {
    out.push(`#### ${C(f.id)} — ${f.rows} rows, ${f.baseTargets} base target${f.baseTargets === 1 ? "" : "s"}`, "");
    out.push(FAMILY_TEXT[f.id] ?? "");
    out.push("");
    out.push(`- Owners: ${list(f.owners.map((o) => C(short(o, 44))), 6)}.`);
    const dims = f.axes.filter((a) => a.rows > 0);
    if (dims.length) out.push(`- Varying axes: ${dims.slice(0, 8).map((a) => `${a.axis} on ${a.rows} row${a.rows === 1 ? "" : "s"} (${Object.keys(a.expressedAs).join(", ")}; ${a.axis === "port" || a.axis.startsWith("test-f") || a.axis === "test-scope" ? a.distinctValues + " values" : a.domain.slice(0, 4).map((d) => d.value).join(" | ")})`).join("; ")}.`);
    const variantFamily = f.class === "playground-dev" || f.family === "wgpu-native-launch";
    if (f.residualArgDomain.length) out.push(variantFamily ? `- Argument = the playground variant id (${new Set(RECS.filter((r) => `${r.class} / ${r.family}` === f.id).map((r) => r.passthru[0])).size} distinct variants; extra tokens after it: ${uniq(RECS.filter((r) => `${r.class} / ${r.family}` === f.id).map((r) => r.passthru.slice(1).join(" ")).filter(Boolean)).slice(0, 4).map((x) => C(x)).join(", ") || "none"}).` : `- Target-specific arguments (${f.residualArgDomain.length}${f.residualArgDomain.length === 10 ? "+" : ""} distinct sets): ${f.residualArgDomain.slice(0, 5).map((d) => C(short(d.value, 70)) + ` x${d.rows}`).join(", ")}.`);
    out.push(`- Constant on every row: env ${f.constantEnv.length ? f.constantEnv.slice(0, 6).map((x) => C(short(x, 50))).join(", ") : "none"}; args ${f.constantArgs.length ? f.constantArgs.slice(0, 6).map((x) => C(short(x, 40))).join(" ") : "none"}.`);
    if (Object.keys(f.residualEnvKeys).length) out.push(`- Target-specific env keys: ${kv(f.residualEnvKeys)}.`);
    if (f.ready.length) out.push(`- Ready action: ${f.ready.map((x) => C(short(x, 90))).join("; ")}.`);
    if (f.topBaseTargets.length > 1) out.push(`- Largest base targets: ${f.topBaseTargets.slice(0, 5).map((b) => `${C(short(b.base, 52))} x${b.rows}`).join(", ")}.`);
    out.push(`- Examples: ${f.examples.map((e) => C(short(e, 60))).join(", ")}.`, "");
  }
  const specific = F.baseTargets.filter((b) => b.families.some((x) => ["target-specific-arguments", "prompted-argument", "print-native-phase", "wgpu-native-launch", "dev-server-port"].includes(x)) && b.argSets.length > 1);
  out.push("### 5.3 Genuinely target-specific argument sets (base targets with more than one argument set)", "", table(["Base target", "Rows", "Argument sets (domain)"], specific.slice(0, 25).map((b) => [C(short(b.base, 60)), b.rows, b.argSets.slice(0, 6).map((a) => C(short(a, 50))).join(", ") + (b.argSets.length > 6 ? ", …" : "")])), "", `Examples of the pattern the coordinator named: ${F.baseTargets.filter((b) => b.base.startsWith("@semio-tech/repo-cli-rs:")).map((b) => `${C(b.base)} -> ${b.argSets.map((a) => C(a)).join("|")}`).join("; ")}.`);
  const dep = RECS.filter((r) => !r.ticketScoped && r.needsServe.length);
  out.push("", "### 5.4 Rows that depend on a running server", "", `${dep.length} non-ticket rows contain a \`http://127.0.0.1:<port>\` (command, env or input default) that is **not** the port they serve themselves: ${tally(dep, (r) => r.needsServe.map((n) => `${n.port}${n.servedBy.length ? " (served by " + short(n.servedBy[0]!, 30) + (n.servedBy.length > 1 ? ` +${n.servedBy.length - 1}` : "") + ")" : " (served by no launch row)"}`).join(" + ")).map(([k, n]) => `${k} x${n}`).join("; ")}. Examples: ${dep.slice(0, 4).map((r) => C(short(r.name, 40))).join(", ")}. These are the rows that need compound semantics (start a serve, wait for its ready URL, then run): the hub on 8787 and the \`s\` React shell on 6070/6071 are the hard dependencies.`);
  return out.join("\n");
}

function sectionEnv(): string {
  const E = ENV;
  return [
    "## 6. Environment variables: noise versus meaning",
    "",
    `${E.distinctKeys} distinct keys (${E.rowsWithAnyEnv} rows carry env; inline \`VAR=value\` command prefixes are counted too). Kinds: ${kv(E.byKind)}. Rows carrying only Nx/VS Code noise env: ${E.rowsWithOnlyNoiseEnv} (${E.rowsWithOnlyNoiseEnvNonTicket} outside tickets). Rows carrying a ticket sandbox (\`NX_WORKSPACE_ROOT_PATH\`/\`NX_WORKSPACE_DATA_DIRECTORY\`/\`NX_CACHE_DIRECTORY\`): ${E.rowsWithSandboxEnv} (${E.rowsWithSandboxEnvNonTicket} outside tickets). Readers were searched in ${E.scanned} tracked code/config files (\`git ls-files\`, excluding tickets, \`.vscode\`, \`.claude\`, \`🗑️generated\`) with read-site patterns (\`process.env.X\`, \`env::var("X")\`, …); Nx-owned variables were additionally searched in the installed Nx (\`.nx/installation/node_modules/nx/dist\`). The wrapper behind \`bun nx\` (root \`package.json\` script -> \`⚡️caching/🚀️bootstrap/📜️script.ts\`) already defaults \`NX_DAEMON=false\` (\`nxChildEnvironment\`), \`NX_ISOLATE_PLUGINS=false\` and \`NX_WORKSPACE_DATA_DIRECTORY=.nx/workspace-data\` (\`run\`), and \`devToolingEnv()\` (\`🏃️process/🌿️environment/🟦️.ts\`) defaults \`NX_TUI=false\`; \`repoToolCacheEnv()\` defaults \`PLAYWRIGHT_BROWSERS_PATH\`.`,
    "",
    table(["Variable", "Rows (non-ticket / ticket)", ".claude", "Top values", "Judgement", "Read by (files reading / mentioning)"], E.table.map((t) => [C(t.key), `${t.rows} (${t.nonTicketRows} / ${t.ticketRows})`, t.claudeEntries || "", t.topValues.slice(0, 2).map((v) => short(v, 44)).join("; "), `**${t.kind}** — ${t.judgement}`, [...(t.nxInternalReaders.length ? [`Nx: ${short(t.nxInternalReaders[0]!.replace(".nx/installation/node_modules/nx/dist/", ""), 52)}`] : []), `${t.filesReading ?? "-"} / ${t.filesMentioning ?? "-"}${t.readSites[0] ? ": " + short(t.readSites[0].split("/").slice(-3).join("/"), 50) : ""}`].join("; ")])),
    "",
    "Judgement summary: **noise** (drop, never store) = `NX_DAEMON`, `NX_ISOLATE_PLUGINS`, `NX_CACHE_PROJECT_GRAPH`, `NX_TUI`, `FORCE_COLOR`, `NX_PLUGIN_NO_TIMEOUTS`; **ticket sandbox** (drops with the ticket rows) = `NX_WORKSPACE_ROOT_PATH`, `NX_WORKSPACE_DATA_DIRECTORY`, `NX_CACHE_DIRECTORY`; **global axes** = `SEMIO_TEST_LEVEL`, `NEXTEST_SUCCESS_OUTPUT`, `CARGO_BUILD_JOBS`, `SEMIO_BUILD_MODE` + `NX_SKIP_NX_CACHE` + `NX_SKIP_REMOTE_CACHE`, `SEMIO_BUILD_BUDGET_MS`, `SEMIO_RENDERER`; **registry-owned** = `SEMIO_PLUGIN`, `SEMIO_APP`, `S_OS_PORT`. Port variables: playground servers bind `S_OS_PORT` only; services such as the quiz, projektetage and demonstrator declare their own port variable in their manifests (`options.env` or a `package.json` `\"env\"` key, which the code-read count does not see, hence the mention column), so those rows merely repeat the owner's default. `PUZZLE_3D_PLAY_PORT`, `PUZZLE_5D_PLAY_PORT` and `SHOOTING_PLAY_PORT` are not mentioned anywhere in tracked files and `CAD_JS_RENDERER_PLAY_PORT` conflicts with its owner's value: dead or stale knobs. `MCP_PROXY_AUTH_TOKEN=repo-mcp-token` is a literal token committed in the launch file.",
  ].join("\n");
}

function sectionCompoundsInputs(): string {
  const I = INPUTS;
  const out: string[] = [
    "## 7. Compounds and inputs",
    "",
    `### 7.1 Compounds (${COMPOUNDS.length}; all in the seed, none generated)`,
    "",
    table(["Compound", "Members (in start order)", "Ready actions", "Needs", "Group / order"], COMPOUNDS.map((c) => [C(c.name), c.members.map((m) => `${C(m.name)}${m.port.length ? ` [${m.port.join(",")}]` : ""}`).join(" -> "), `${c.membersWithReadyAction}/${c.members.length}`, [c.hubDependentMembers.length ? `hub URL in ${c.hubDependentMembers.length} member(s) (${[...new Set(c.members.flatMap((m) => Object.keys(m.envSemantic).filter((k) => /HUB|DATA/.test(k))))].join(", ")})` : "", `all members found: ${c.allMembersFound}`].filter(Boolean).join("; "), `${c.presentation?.group} / ${c.presentation?.order}`])),
    "",
    `Compound keys used: ${[...new Set(((LAUNCH.compounds ?? []) as Json[]).flatMap((c) => Object.keys(c)))].join(", ")} (no \`preLaunchTask\`, no \`folder\`). What the dashboard must reproduce: (1) members are launched **in array order** (hub before shell: the shell's \`S_HUB_URL=http://127.0.0.1:8787\` needs the hub up), (2) VS Code starts them in parallel without waiting, so a dashboard should **wait for each member's ready URL** (its \`serverReadyAction\` pattern) before starting a dependent member, (3) \`stopAll: true\` — stopping one stops all, (4) every member is an existing playground/hub row, so a compound is simply \`[command refs]\` + order + stop policy; ${COMPOUNDS.filter((c) => c.members.some((m) => /👤️\d/.test(m.name))).length} compound uses the multi-user rows (user slots 1 and 2 with separate \`S_DATA_DIR\`).`,
    "",
    `### 7.2 Inputs (${I.total}: ${kv(I.byType)}; ${I.generated} generated \`projectTarget.*\` pickStrings, ${I.seedInputs} seed inputs)`,
    "",
    `Generated pickers: ${I.generatedPickerChecks.checked} checked, ${I.generatedPickerChecks.withInconsistentOptions} with an option that does not declare the target. Dangling \`\${input:…}\` references: ${I.danglingReferences.length}. Declared but unused: ${I.unused.length ? I.unused.map((u) => C(u)).join(", ") : "none"}. Ticket-scoped (default/description references a ticket): ${I.ticketScoped.length} (${I.ticketScoped.map((u) => C(u)).join(", ")}).`,
    "",
    table(["Input", "Type", "Default / options", "Used by (non-ticket / ticket rows)", "Ticket-scoped", "Meaning"], I.list.filter((i) => !i.generated).map((i) => [C(i.id), i.type, i.options ? `options: ${list((i.options as string[]).map((o) => (typeof o === "string" ? o : JSON.stringify(o))), 6)}${i.default !== undefined ? `; default ${C(short(String(i.default), 40))}` : ""}` : i.default !== undefined ? C(short(String(i.default), 70)) : "(no default)", `${i.usedByNonTicket} / ${i.usedByTicketScoped}`, i.ticketScoped ? "yes" : "no", short(String(i.description ?? ""), 100)])),
    "",
    `The ${I.generated} \`projectTarget.<target>\` inputs are pickStrings whose options are the owning projects (${list(I.list.filter((i) => i.generated).map((i) => `${i.id.slice(14)}[${i.optionCount}]`), 8)}); the dashboard already lists every project target, so they are redundant. Input kinds a dashboard needs: free text with optional default (${I.byType.promptString ?? 0}) and fixed choice (${(I.list.filter((i) => !i.generated && i.type === "pickString").length)} non-generated pickStrings); no \`command\` inputs and no \`password\` flags are used.`,
  ];
  return out.join("\n");
}

function sectionReady(): string {
  const R = READY;
  return [
    "## 8. serverReadyAction",
    "",
    `${R.total} rows (${R.nonTicket} non-ticket, ${R.ticketScoped} ticket-scoped); all use \`action: ${Object.keys(R.actions).join(", ")}\`; extra keys: ${kv(R.extraKeys)}. By class: ${kv(R.byClass)}; by provenance ${kv(R.byProvenance)}. In ${R.portInPatternMatchesPortEnv.matching} of ${R.portInPatternMatchesPortEnv.rows} rows the port inside the pattern equals a port env variable of the same row${R.portInPatternMatchesPortEnv.nonMatching.length ? `; exceptions: ${R.portInPatternMatchesPortEnv.nonMatching.map((x) => `#${x.index} ${C(short(x.name, 36))} (pattern port ${x.inPattern}, env ${x.portVars.map((p) => p.k + "=" + p.v).join(",") || "none"})`).join("; ")}` : ""}.`,
    "",
    table(["Pattern (port normalised to {PORT})", "uriFormat", "Rows", "Examples"], R.distinct.map((d) => [C(d.pattern), C(d.uriFormat), d.rows, d.examples.map((e: string) => C(short(e, 34))).join(", ")])),
    "",
    "What a dashboard needs to reproduce: watch the task's output for the first match of the regex (a localhost/127.0.0.1/0.0.0.0 `http://host:port`; the generator uses the host alternation `127.0.0.1|localhost|0.0.0.0` because a devcontainer prints `0.0.0.0`), take capture group 1 (the whole URL) and open `uriFormat` with `%s` replaced by it (`%s`, `%s/admin`, `%s/?plugin=…`, or the MCP inspector URL with auth token and server arguments); `openExternally` = default browser. Because the port equals the row's own port variable, a single declaration **ready: {port, path-suffix}** reproduces every pattern except the MCP inspector one (token query string) and the two `?plugin=…` suffix rows — the pattern itself is derivable. For playgrounds the port, URL suffix and host are registry facts (`ports.react`/`wgpu`/`userPorts`).",
  ].join("\n");
}

function sectionPresentation(): string {
  const P = PRESENTATION;
  return [
    "## 9. Presentation: groups, order and the name grammar",
    "",
    table(["`presentation.group`", "Rows", "Order range", "Distinct orders", "Leading name keys", "Provenance"], P.groups.map((g) => [C(g.group), g.rows, g.orderMin === null ? "-" : `${g.orderMin} .. ${g.orderMax}`, g.distinctOrders, g.topNameKeys.slice(0, 4).join(", "), kv(g.provenance as Record<string, number>)])),
    "",
    `- \`group\` is VS Code's sort bucket (lexicographic: \`0_dev\`, \`1_keyboard\`, \`2_mouse\`, \`3_dev\`, \`4_build\`, \`4_gate\`, \`4_test\`, \`9_gates\`, then ad-hoc names); \`order\` sorts inside a group. ${P.distinctGroupOrderPairs} distinct (group, order) pairs for ${RECS.length - P.rowsWithoutPresentation} rows, ${P.rowsSharingGroupOrder} rows share an order with another row. **Neither carries information that is not already in (verb, project, target)**: generated rows use \`orderBase + index/10000\` in label-then-target order; seed numbers are insertion positions (\`386.15\`, \`900.0586999…\`).`,
    `- Groups map to verbs: \`3_dev\` = dev servers and workspace commands (class \`dev\` + fallback \`run\`), \`4_gate\` = checks/tests/verification (class \`gate\`), \`4_build\` = build/generate/format/clean (class \`build\`), \`9_gates\` = the hand-written \`🧪️test…\` single-file probes, \`1_keyboard\`/\`2_mouse\` = tool shortcuts (gemini, kiro / f3d, gitkraken, MCP inspector), the rest are one-off labels (\`🧿️ Semio Snapshot SQLite\`, \`repo-gate\`, \`🧹clean🛡️gates\`, …). ${P.rowsWithoutPresentation} rows have no \`presentation\` at all (they sort last).`,
    "",
    "Leading name keys (emoji + first lowercase word):",
    "",
    table(["Key", "Rows", "Groups", "Meaning"], P.topNameKeys.slice(0, 22).map((k) => [C(k.key), k.rows, kv(k.groups as Record<string, number>), NAME_KEY_TEXT[k.key] ?? ""])),
    "",
    "Grammar, as far as it is mechanical:",
    "",
    "- Generated project rows: `<verb emoji><target><project label>`; verb emoji from the target-name token (⚖️ gate: test/check/verify/lint/…, 📦️ build, 🛠️ dev, ▶️ run), label = shortest unique trailing path segments with 🦀️/🟦️/🐍️ (example `⚖️test-artifact-kind🧰️framework🦀️`). Family rows end in `📋️`. **Fully derivable** (section 2).",
    `- Playground rows: \`🛠️dev<plugin folder><artifact folder>[<standard folder>]<subset folder><renderer suffix>\` with suffix ${kv(P.playgroundRendererSuffix)}; user slot rows insert \`👤️<N>\` before the suffix (${P.userSlotNames.length} rows). Derivable from registry + taxonomy folders (needs the folder walk of \`🏷️name-prefix\`), or simply \`<variant> <renderer>\`.`,
    `- Compounds: \`🧭️compound<member shorthand>\` (${P.compoundNames.join("; ")}).`,
    `- Seed-authored names are free-form hand labels (e.g. \`⚖️gate🏢️semio-tech🎡️play🌊️flow-fixtures\`, \`🧪️test📜️history🎮️operation…\`): ${P.namesWithoutLeadingEmoji} names have no leading emoji at all, ${P.namesWithSpaces} contain spaces. They carry no information beyond the underlying command and are not derivable; a registry should derive the label from (verb, target, owner label) and let an owner override it only when needed.`,
  ].join("\n");
}
const NAME_KEY_TEXT: Record<string, string> = {
  "⚖️gate": "gate: check/test/verify run (seed hand name, or `⚖️gate` + domain emojis)", "⚖️test": "gate named after the target (`⚖️test-<target><label>`; generated or hand-made)", "🛠️dev": "dev server / dev tool (playground launcher or `🛠️dev<domain>`)", "🧪️test": "single-file or probe test (`9_gates`)",
  "(no leading emoji)": "free-form (`🧰️ Framework …` style with space)", "🧫️fixtures": "fixture-isolation rows of the FIXTURES ticket", "🧱️": "ticket probe rows", "⚖️verify": "verification gates", "📥️native": "ticket probe rows", "🧿️semio": "snapshot SQLite rows", "📦️build": "build", "📦️🏭️generator": "generator build", "🧹clean": "clean/guard rows",
};

function sectionClaude(): string {
  const K = CLAUDE_ANALYSIS;
  const kindText: Record<string, string> = {
    "playground-dev": "playground dev server (`workspace:dev -- <variant>`, `framework-os-dev:serve-<v>-react-dev`, `framework-os-dev:dev -- <v>`): canonical = playground catalog entry + renderer; port equals the registry port",
    "playground-native": "wgpu native renderer of a playground (`framework-renderer-wgpu:native -- <v>`)",
    "gate": "an `nx run P:test…` gate: canonical = the Nx target (+ level axis)",
    "service-dev": "service dev server (`P:dev`) with a port: canonical = Nx target `dev` + declared port/ready",
    "dev-tool": "`workspace:dev -- storybook …|gis 2d`: workspace dev command",
    "ticket-script": "ticket-owned script (bash/bun file under `🎫️tickets`): no canonical command",
    "script": "a bun script outside Nx",
    "attach-only": "no command: `{name, port|url}` — attach the preview to a server that is already running",
  };
  return [
    "## 10. `.claude/launch.json` (Claude Code preview servers)",
    "",
    `${K.total} entries; keys used: ${kv(K.topKeys)}. ${K.withCommand} start a process (\`runtimeExecutable\` bun/bash + \`runtimeArgs\`), ${K.attachOnly} only declare \`port\`/\`url\` (attach to a running server). ${K.withEnv} entries have env (noise keys: ${kv(K.noiseEnvKeys)}). By kind: ${kv(K.byKind)}. ${K.identicalToLaunchJsonRow} entries run exactly the same command+env as a \`.vscode/launch.json\` row. All playground ports match the playground registry (${K.portMismatches.length} mismatches). Nx verification: ${K.deadByNx.length} entr${K.deadByNx.length === 1 ? "y" : "ies"} dead (${K.deadByNx.map((d) => C(d.detail)).join(", ") || "-"}); ${K.ticketScripts} entries run ticket-local scripts; ${K.missingPathRefs.length} entries point at script files that no longer exist (${K.missingPathRefs.map((m) => C(m.name)).join(", ")}).`,
    "",
    table(["Kind", "Entries", "Corresponds to"], Object.entries(K.byKind).map(([k, n]) => [C(k), n, kindText[k] ?? ""])),
    "",
    table(["Name", "Kind", "Canonical command", "Port (registry)", "Env (semantic)", "Same as launch.json row", "Verdict"], K.entries.map((e) => [C(short(e.name, 34)), e.kind, e.canonical ? short(e.canonical, 56) : e.url ? `attach ${e.url}` : "attach", e.port === undefined ? "-" : `${e.port}${e.registryPort ? ` (${e.registryPort}${e.portMatchesRegistry ? " ok" : " MISMATCH"})` : ""}`, Object.keys(e.envSemantic).join(",") || "-", e.identicalLaunchJsonRow ? C(short(e.identicalLaunchJsonRow, 30)) : "-", e.existence.startsWith("missing") ? `dead (${e.existenceDetail})` : e.missingPathRefs.length ? "script missing" : "ok"])),
    "",
    "Recommendation: the preview entries are a *second* registry for the same playground servers. Every `playground-dev`, `playground-native`, `service-dev`, `gate` and `dev-tool` entry is a view of a canonical command the dashboard already discovers (+ port); the `attach-only` entries become a dashboard-side **attach** (port/url of a running dev server, registry-derived) and the ticket scripts (supervisors, probes) are ticket-scoped and drop. The only data worth keeping is the port, which the playground registry already owns; the file can be generated from the dashboard's catalog (name, port, command) if Claude Code's preview tool still needs it.",
  ].join("\n");
}

function sectionRecommendations(): string {
  const t = TOTALS;
  const rec = t.byRecommendation;
  const order = ["drop (dead)", "drop (redundant with Nx target discovery)", "drop (redundant with playground catalog discovery)", "drop (ticket-scoped, closed ticket)", "drop (ticket-scoped, open ticket)", "drop (ticket-scoped, ticket folder gone)", "drop (ticket-scoped, no ticket reference)", "keep as declared command preset in the owner's manifest", "express as a global axis", "express as a compound", "needs an input prompt"];
  const cells = new Map<string, Map<string, number>>();
  for (const r of RECS) { const k = `${r.class} / ${r.family}`; if (!cells.has(k)) cells.set(k, new Map()); cells.get(k)!.set(r.recommendation, (cells.get(k)!.get(r.recommendation) ?? 0) + 1); }
  const rows: unknown[][] = [];
  for (const [fam, m] of [...cells].sort((a, b) => [...b[1].values()].reduce((x, y) => x + y, 0) - [...a[1].values()].reduce((x, y) => x + y, 0))) for (const [r, n] of [...m].sort((a, b) => b[1] - a[1])) rows.push([C(fam), n, r, r.startsWith("drop (dead") ? "The Nx target or file no longer exists." : r.startsWith("drop (ticket") ? "One-off ticket probe; belongs in the ticket folder." : REC_WHY[fam] ?? ""]);
  const reg = t.registry;
  const dropRows = Object.entries(rec).filter(([k]) => k.startsWith("drop")).reduce((a, [, n]) => a + n, 0);
  return [
    "## 11. Recommendation per class and family",
    "",
    "A ticket-scoped or dead row takes that recommendation first (ticket status and deadness are properties of the row, not of its family); the families then decide the rest. \"Playground catalog discovery\" is the dashboard's existing discovery of playground leaves from the generated playground catalog (the same registry the generator consumes); it is the second kind of \"redundant with discovery\" and is listed separately from plain Nx targets.",
    "",
    table(["Class / family", "Rows", "Recommendation", "Why"], rows),
    "",
    "### Totals",
    "",
    table(["Recommendation", "Rows", "Share"], order.filter((o) => rec[o]).map((o) => [o, rec[o], pct(rec[o]!, t.total)]).concat([["**total**", t.total, "100%"]])),
    "",
    `Dropped: **${dropRows}** (${pct(dropRows, t.total)}) — ${rec["drop (dead)"] ?? 0} dead, ${(rec["drop (redundant with Nx target discovery)"] ?? 0)} redundant with Nx discovery, ${rec["drop (redundant with playground catalog discovery)"] ?? 0} redundant with the playground catalog, ${Object.entries(rec).filter(([k]) => k.startsWith("drop (ticket")).reduce((a, [, n]) => a + n, 0)} ticket-scoped (0 of them with a closed ticket). Retained in some form: **${t.total - dropRows}**.`,
    "",
    "### How small is the hand-maintained canonical registry?",
    "",
    table(["Registry element", "Entries", "Covers", "Content"], [
      ["Global axes", reg.globalAxes, `${RECS.filter((r) => !r.ticketScoped && Object.keys(r.axes).length > 0).length} non-ticket rows spell at least one axis`, reg.axisCatalogue.join(", ")],
      ["Declared presets (owner manifest)", reg.declaredPresets, `${reg.keepRows} rows`, `${reg.keepBaseTargets} distinct base targets; ${Object.entries(reg.keepPresetsByFamily).map(([k, v]) => `${k} ${v}`).join(", ")}`],
      ["Compounds", reg.compounds, `${RECS.filter((r) => r.recommendation === "express as a compound").length} rows + ${COMPOUNDS.length} VS Code compounds`, `${reg.vscodeCompounds} VS Code compounds (hub+shell, multi-user hub, quiz+proctor) + ${reg.nxRunManyCompounds} \`nx run-many\` target groups`],
      ["Input prompts", reg.inputPrompts, `${RECS.filter((r) => r.recommendation === "needs an input prompt").length} rows`, `${reg.promptInputs.length} seed inputs used outside tickets (${reg.promptInputs.join(", ")}) + generic prompts ${reg.genericPrompts.join(", ")}`],
      ["**Total hand-maintained**", reg.handMaintainedEntries, `${t.total - dropRows} retained rows from ${t.total}`, "everything else is discovered (Nx targets, playground catalog) or dropped"],
    ]),
    "",
    `Upper bound: the ${reg.declaredPresets} presets still contain families that parameterise further (the \`two-client-e2e\`/\`document-growth-e2e\`/\`backend-*\` rows differ only by backend sqlite|postgres|neo4j, \`daemon\` only by start|attach, ${RECS.filter((r) => r.family === "dev-server-port" && r.env.STORYBOOK_PORT).length} storybook dev servers only by their target); with a per-target \`args: [...]\` domain the preset count approaches the ${reg.keepBaseTargets} distinct base targets. The ${reg.globalAxes} global axes replace ${RECS.filter((r) => !r.ticketScoped && Object.keys(r.axes).length > 0).length} non-ticket rows that spell an axis, and the user-slot/example/role axes replace ${RECS.filter((r) => ["playground-dev/user-slot", "playground-dev/seed-variant"].includes(r.family)).length} playground rows (${RECS.filter((r) => r.family === "playground-dev/user-slot").length} generated user-slot rows + ${RECS.filter((r) => r.family === "playground-dev/seed-variant").length} seed variants).`,
    "",
    "### Order of migration (suggested)",
    "",
    `1. Delete the ${TICKET.length} ticket rows with their ${INPUTS.ticketScoped.length} ticket-path inputs (and the ${INPUTS.unused.length} unused inputs); a dashboard that never reads a launch file also stops \`reconcile-launch-seed\` from re-adopting new ones. 2. Drop the ${DEAD.nonTicket} dead rows and the ${RECS.filter((r) => r.class === "plain-nx").length}+${RECS.filter((r) => r.class === "project-picker-family").length}+${RECS.filter((r) => r.family === "playground-dev/registry-generated").length} discovered-anyway rows. 3. Hand-migrate the remaining rows in this order: playground axes (user slot, example, role, wgpu-native), the test axes (level, nextest output, cargo jobs, ship build + cache policy, task dependencies, output style) with the filter/file prompts, then the ${TOTALS.registry.declaredPresets} declared presets in their owners' manifests (ports + ready patterns of the ${RECS.filter((r) => r.family === "dev-server-port").length} dev servers first), then the ${TOTALS.registry.compounds} compounds, then the remaining prompts. 4. Retire \`.claude/launch.json\` last (${CLAUDE_ANALYSIS.total} entries; derivable from the same catalog).`,
  ].join("\n");
}
const REC_WHY: Record<string, string> = {
  "plain-nx / plain-nx": "The dashboard discovers every Nx target.",
  "project-picker-family / project-picker": "One row per target name with a project pick list = the dashboard's own target list.",
  "playground-dev / playground-dev/registry-generated": "Generated from the playground registry that the dashboard discovers; renderer is an axis.",
  "playground-dev / playground-dev/user-slot": "User slot is an axis of a playground (registry `userPorts`).",
  "playground-dev / playground-dev/seed-variant": "Example / role / fixture argument are playground axes.",
  "nx-preset / test-axis-variant": "Only cross-cutting axes on a discovered target.",
  "nx-preset / test-selection/filter": "Axes + a free-text test selection (prompt).",
  "nx-preset / test-selection/vitest-files": "Axes + test file prompt.",
  "nx-preset / test-selection/scope": "Axes + cargo scope prompt.",
  "nx-preset / axis-variant": "Only cross-cutting axes on a discovered target.",
  "nx-preset / wgpu-native-launch": "Renderer axis value `wgpu-native` of a catalogued playground.",
  "nx-preset / print-native-phase": "One preset with a phase parameter owned by `@semio-tech/print`.",
  "nx-preset / dev-server-port": "Port + ready pattern belong in the owner's manifest.",
  "nx-preset / prompted-argument": "Already an input prompt (acceptance URLs, proctor arguments).",
  "nx-preset / target-specific-arguments": "Sub-commands / backend names / serve URLs are target facts; declare in the owner's manifest.",
  "nx-exec-bun-test / bun-test-file": "One generic `bun test <file>` command with a file prompt.",
  "tool-launcher / nx-run-many": "A named project set for one target.",
  "tool-launcher / dev-tool/workspace-dev-mcp": "Repo-level MCP tools.",
  "tool-launcher / external-tool": "External developer tools (not Nx).",
};

function sectionCaveats(): string {
  return [
    "## 12. Caveats and open points",
    "",
    `- Ticket scoping is evidence-based (paths, ticket-named Nx projects, \`🗑️generated\`, ticket input defaults); a legitimate non-ticket row that merely writes its output below \`🗑️generated\` would be counted as ticket-scoped (${TICKETS.rowsWithoutTicketReference.length} rows are in that situation: ${TICKETS.rowsWithoutTicketReference.map((r) => "#" + r.index).join(", ")}; both are \`${TICKETS.rowsWithoutTicketReference[0]?.command.slice(0, 40) ?? ""}…\`-style gates that hand their receipts to a root-level \`🗑️generated\`).`,
    `- Existence is verified against declared manifests, inference rules and the cached graph without running Nx; a target created by an inference plugin that neither the rules nor the graph know would be reported \`missing-target\`. The graph copy used was computed ${GRAPH.computedAt}; ${GRAPH_ONLY_UNEXPLAINED} of its targets are not explained by manifests or rules (\`nx-release-publish\` and two \`test:*\`).`,
    `- Relative file arguments after \`--\` are checked against the project root and the workspace root only; a program that resolves them elsewhere would be reported missing (the two non-ticket missing-path rows were confirmed by hand: one file moved, one is untracked).`,
    `- \`readers\` of an env variable are found by regex over tracked code; a variable read only through a computed key is reported with 0 readers (the \`unread-probe-knob\` and "dead knob" judgements say "no tracked reader found", not "proved unused").`,
    `- The live \`.vscode/launch.json\` and seed are rewritten by other developers while this audit ran (live differs from the audited snapshot: ${Object.entries(driftFromLive).filter(([, same]) => !same).map(([k]) => k).join(", ") || "no"}); re-run with \`--source=live\` for current numbers. The inventory is deterministic for a given snapshot.`,
    `- Nothing outside the ticket folder was written; no \`nx\`, \`cargo\` or build was run. \`--verify-generator\` imports the repository's own generator in memory (read-only).`,
  ].join("\n");
}

function renderReport(): string {
  return [sectionHeader(), sectionSummary(), sectionGenerator(), sectionExistence(), sectionTickets(), sectionFamilies(), sectionEnv(), sectionCompoundsInputs(), sectionReady(), sectionPresentation(), sectionClaude(), sectionRecommendations(), sectionCaveats()].join("\n\n") + "\n";
}
if (!flag("no-write")) { writeFileSync(join(TICKET_DIR, "launch-inventory.md"), renderReport()); console.log(`launch-inventory: ${RECS.length} configurations -> ${T_REL}/launch-inventory.md and inventory.json (source ${SOURCE})`); }
//#endregion
