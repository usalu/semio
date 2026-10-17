// 🧮️ (c4p17 copy: ONLY is a comma list of name substrings; only matching nodes are measured) Read-only census of first-party Rust crates against the compiler-ingress source boundary (imports the canonical closure counter).
import {readFileSync,readdirSync,existsSync,writeFileSync,mkdirSync} from "node:fs";
import {basename,dirname,join,resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const WS = process.env.WS_DIR ? resolve(ROOT, process.env.WS_DIR) : ROOT;
const OUT = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated");
const LIMIT = {files: 32, bytes: 262144, lines: 4000};
const only = process.env.ONLY ? new Set(process.env.ONLY.split(",")) : undefined;

const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg", ...(process.env.TRIPLE ? ["--target", process.env.TRIPLE] : [])], {encoding: "utf8"}).trim().split("\n")
  .map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));

const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};

const rootToml = Bun.TOML.parse(readFileSync(join(WS, "Cargo.toml"), "utf8")) as any;
const wsDeps: Record<string, any> = rootToml.workspace?.dependencies ?? {};

const memberDirs: string[] = [];
for (const pattern of rootToml.workspace.members as string[]) {
  if (pattern.includes("*")) for (const file of new Bun.Glob(pattern + "/Cargo.toml").scanSync({cwd: WS})) memberDirs.push(resolve(WS, dirname(file)));
  else memberDirs.push(resolve(WS, pattern));
}

const featureSet = (parsed: any): Set<string> => {
  const table: Record<string, string[]> = parsed.features ?? {};
  const out = new Set<string>(["default"]), queue = [...(table.default ?? [])];
  while (queue.length) {
    const token = queue.pop()!;
    if (out.has(token)) continue;
    out.add(token);
    if (token.startsWith("dep:")) continue;
    const [head, rest] = token.split("/") as [string, string | undefined];
    const name = head.replace(/\?$/u, "");
    if (table[name]) queue.push(...table[name]);
    if (rest !== undefined && !head.endsWith("?")) out.add(name);
  }
  return out;
};

const depDirsOf = (parsed: any, dir: string, enabled: Set<string>): string[] => {
  const tables: any[] = [parsed.dependencies, parsed["build-dependencies"], ...Object.values(parsed.target ?? {}).flatMap((t: any) => [t.dependencies, t["build-dependencies"]])];
  const out: string[] = [];
  for (const table of tables) for (const [key, raw] of Object.entries(table ?? {}) as [string, any][]) {
    const spec = typeof raw === "string" ? {} : raw;
    const inherited = spec.workspace === true;
    const resolved = inherited ? (wsDeps[spec.package ?? key] ?? {}) : spec;
    if (typeof resolved.path !== "string") continue;
    if (resolved.optional === true && !enabled.has(key) && !enabled.has("dep:" + key)) continue;
    out.push(resolve(inherited ? WS : dir, resolved.path));
  }
  return out;
};

const targetsOf = (parsed: any, dir: string, pkg: string): {kind: string; name: string; entry: string}[] => {
  const out: {kind: string; name: string; entry: string}[] = [];
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  if (existsSync(libEntry)) out.push({kind: "lib", name: parsed.lib?.name ?? pkg.replaceAll("-", "_"), entry: libEntry});
  for (const bin of parsed.bin ?? []) out.push({kind: "bin", name: bin.name, entry: resolve(dir, bin.path ?? `src/bin/${bin.name}.rs`)});
  if (parsed.autobins !== false) {
    const main = resolve(dir, "src/main.rs");
    if (existsSync(main)) out.push({kind: "bin", name: pkg, entry: main});
    const binDir = resolve(dir, "src/bin");
    if (existsSync(binDir)) for (const row of readdirSync(binDir, {withFileTypes: true})) {
      if (row.isFile() && row.name.endsWith(".rs")) out.push({kind: "bin", name: basename(row.name, ".rs"), entry: join(binDir, row.name)});
      else if (row.isDirectory() && existsSync(join(binDir, row.name, "main.rs"))) out.push({kind: "bin", name: row.name, entry: join(binDir, row.name, "main.rs")});
    }
  }
  const build = resolve(dir, parsed.build ?? "build.rs");
  if (parsed.build !== false && existsSync(build)) out.push({kind: "build-script", name: "build-script-build", entry: build});
  const seen = new Set<string>();
  return out.filter(target => seen.has(target.entry) ? false : (seen.add(target.entry), true));
};

type Limit = {files: number; bytes: number; lines: number};
type Target = {kind: string; name: string; entry: string; files: number; bytes: number; lines: number; error: string; ratio: number; embeddedFiles: number; embeddedBytes: number; binding: string};
type Node = {dir: string; manifest: string; name: string; member: boolean; deps: string[]; targets: Target[]; over: boolean; limit: Limit};
const effectiveLimit = (parsed: any): Limit => {
  const declared = parsed.package?.metadata?.semio?.["rust-source-boundary"];
  return declared ? {files: Math.min(LIMIT.files, declared.maxSourceFiles), bytes: Math.min(LIMIT.bytes, declared.maxSourceBytes), lines: Math.min(LIMIT.lines, declared.maxActiveLines)} : {...LIMIT, files: LIMIT.files};
};
const nodes = new Map<string, Node>(), missing: string[] = [];
const queue: [string, boolean][] = memberDirs.map(dir => [dir, true]);
const features = new Map<string, Set<string>>(), depDirs = new Map<string, string[]>();

while (queue.length) {
  const [dir, member] = queue.shift()!;
  if (nodes.has(dir)) { if (member) nodes.get(dir)!.member = true; continue; }
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) { missing.push(dir); continue; }
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const name = parsed.package?.name ?? basename(dir);
  nodes.set(dir, {dir, manifest, name, member, deps: [], targets: [], over: false, limit: effectiveLimit(parsed)});
  const enabled = featureSet(parsed);
  features.set(dir, enabled);
  const deps = depDirsOf(parsed, dir, enabled);
  depDirs.set(dir, deps);
  for (const dep of deps) queue.push([dep, false]);
}

const cfgFor = (dir: string): string[] => [...hostCfg, "debug_assertions", ...(process.env.CFG_TEST ? ["test"] : []), ...[...features.get(dir)!].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f)];

const wanted = (name: string) => !only || [...only].some(token => name === token);
for (const node of nodes.values()) {
  if (!wanted(node.name)) continue;
  const parsed = Bun.TOML.parse(readFileSync(node.manifest, "utf8")) as any;
  for (const target of targetsOf(parsed, node.dir, node.name)) {
    let measured: Target;
    try {
      const inputs = await readRustSourceClosureV1([target.entry], ROOT, cfgFor(node.dir), readFile, {manifestDirectory: node.dir, generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((sum, input) => sum + input.bytes, 0), lines = inputs.reduce((sum, input) => sum + input.activeLines, 0);
      const embedded = inputs.filter(input => input.kind === "embedded");
      const parts = {files: files / node.limit.files, bytes: bytes / node.limit.bytes, lines: lines / node.limit.lines};
      const binding = (Object.entries(parts).sort((a, b) => b[1] - a[1])[0]![0]);
      measured = {...target, files, bytes, lines, error: "", ratio: Math.max(parts.files, parts.bytes, parts.lines), embeddedFiles: embedded.length, embeddedBytes: embedded.reduce((sum, input) => sum + input.bytes, 0), binding};
    } catch (error) {
      measured = {...target, files: 0, bytes: 0, lines: 0, error: String((error as Error).message ?? error).replaceAll("\n", " ").slice(0, 200), ratio: Infinity, embeddedFiles: 0, embeddedBytes: 0, binding: "refused"};
    }
    node.targets.push(measured);
  }
  node.over = node.targets.some(target => target.ratio > 1);
  node.targets.sort((a, b) => (a.ratio === b.ratio ? 0 : b.ratio > a.ratio ? 1 : -1));
}

for (const node of nodes.values()) node.deps = (depDirs.get(node.dir) ?? []).filter(dep => nodes.has(dep)).map(dep => nodes.get(dep)!.dir);

const transitive = (dir: string, seen = new Set<string>()): Set<string> => {
  for (const dep of nodes.get(dir)?.deps ?? []) if (!seen.has(dep)) { seen.add(dep); transitive(dep, seen); }
  return seen;
};

const rows = [...nodes.values()].sort((a, b) => a.name.localeCompare(b.name));
const csvCell = (value: unknown) => {
  const text = String(value ?? "");
  return /[",\n;]/u.test(text) ? '"' + text.replaceAll('"', '""') + '"' : text;
};
const header = ["name", "manifest", "files", "bytes", "activeLines", "overLimit", "deps", "target", "targetKind", "workspaceMember", "oversizedDeps", "closureCompilable", "carvableLeaf", "limitFiles", "limitBytes", "limitLines", "worstRatio", "binding", "embeddedFiles", "embeddedBytes", "error"];
const lines: string[] = [header.join(",")];
const summary: Record<string, unknown>[] = [];
for (const node of rows) {
  const reach = transitive(node.dir);
  const reachOver = [...reach].filter(dir => nodes.get(dir)?.over);
  const worst = [...node.targets].sort((a, b) => b.ratio - a.ratio)[0];
  const closureCompilable = !node.over && reachOver.length === 0 && node.targets.every(t => !t.error);
  const carvableLeaf = node.over && reachOver.length === 0;
  const error = node.targets.filter(t => t.error).map(t => t.name + ": " + t.error).join(" | ");
  const direct = node.deps.map(dir => nodes.get(dir)!.name).join(";");
  const row = {
    name: node.name, manifest: node.manifest.replace(ROOT + "/", ""), files: worst?.files ?? 0, bytes: worst?.bytes ?? 0, activeLines: worst?.lines ?? 0,
    overLimit: node.over, deps: direct, target: worst?.name ?? "", targetKind: worst?.kind ?? "", workspaceMember: node.member,
    oversizedDeps: reachOver.length, closureCompilable, carvableLeaf, limitFiles: node.limit.files, limitBytes: node.limit.bytes, limitLines: node.limit.lines,
    worstRatio: worst ? (Number.isFinite(worst.ratio) ? Number(worst.ratio.toFixed(3)) : "refused") : 0, binding: worst?.binding ?? "", embeddedFiles: worst?.embeddedFiles ?? 0, embeddedBytes: worst?.embeddedBytes ?? 0, error,
  };
  summary.push(row);
  lines.push(header.map(key => csvCell((row as Record<string, unknown>)[key])).join(","));
}

mkdirSync(OUT, {recursive: true});
if (!only) {
  writeFileSync(join(OUT, "crate-census.csv"), lines.join("\n") + "\n");
  writeFileSync(join(OUT, "census-summary.json"), JSON.stringify({limits: LIMIT, nodes: summary.length, missing, rows: summary}, null, 2));
}
const oversized = summary.filter(row => row.overLimit).sort((a, b) => (b.worstRatio as number) - (a.worstRatio as number));
const leaves = summary.filter(row => row.carvableLeaf).sort((a, b) => (b.worstRatio as number) - (a.worstRatio as number));
console.log(JSON.stringify({
  crates: summary.length, members: summary.filter(row => row.workspaceMember).length, missing: missing.length,
  oversized: oversized.length, compilable: summary.filter(row => row.closureCompilable).length, carvableLeaves: leaves.length,
  withErrors: summary.filter(row => row.error).length,
}));
if (only) { console.log("ROWS", summary.length, [...nodes.values()].filter(n => wanted(n.name)).length); for (const row of summary.filter(row => only.has(row.name as string))) console.log(JSON.stringify(row)); }
else {
  console.log("TOP60");
  for (const row of oversized.slice(0, 60)) console.log([row.name, row.manifest, row.files, row.bytes, row.activeLines, row.worstRatio, row.oversizedDeps, row.carvableLeaf, row.deps].join("\t"));
  console.log("LEAVES");
  for (const row of leaves) console.log([row.name, row.manifest, row.files, row.bytes, row.activeLines, row.worstRatio].join("\t"));
}
