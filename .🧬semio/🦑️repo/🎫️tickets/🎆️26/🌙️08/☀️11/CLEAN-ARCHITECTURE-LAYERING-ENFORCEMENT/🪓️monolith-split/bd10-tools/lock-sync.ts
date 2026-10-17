// 🧰 bd10: upserts the raster crate blocks (exact dependencies from the manifests on disk) into a consumer Cargo.lock, rewrites chosen path-package dependency lists, and drops the blocks the change orphaned.
// usage: bun lock-sync.ts <Cargo.lock> [--set <package>=<dep,dep,...>]... [--manifest <Cargo.toml>]... [--write]
import {readFileSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";

const args = process.argv.slice(2);
const lockPath = args[0]!;
const write = args.includes("--write");
const sets: Record<string, string[]> = {};
const manifests: string[] = [];
for (let i = 1; i < args.length; i++) {
  if (args[i] === "--set") { const [pkg, deps] = args[++i]!.split("="); sets[pkg!] = (deps ?? "").split(",").filter(Boolean); }
  else if (args[i] === "--manifest") manifests.push(args[++i]!);
}
const root = "/Users/ueli/Documents/semio";
const art = join(root, "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster");
const wsRows = (Bun.TOML.parse(readFileSync(join(art, "Cargo.toml"), "utf8")) as {workspace: {dependencies: Record<string, {package?: string; path?: string}>}}).workspace.dependencies;
const text = readFileSync(lockPath, "utf8");
const parts = text.split(/\n(?=\[\[package\]\]\n)/u);
const header = parts[0]!.startsWith("[[package]]") ? "" : parts.shift()!;
type B = {name: string; version: string; text: string; deps: string[]; path: boolean};
const parse = (t: string): B => ({text: t, name: /name = "([^"]+)"/u.exec(t)![1]!, version: /version = "([^"]+)"/u.exec(t)![1]!, path: !/^source = /mu.test(t), deps: [...(/dependencies = \[([\s\S]*?)\n\]/u.exec(t)?.[1] ?? "").matchAll(/"([^"]+)"/gu)].map((m) => m[1]!)});
let blocks: B[] = parts.map(parse);
const render = (name: string, version: string, deps: string[]): string => `[[package]]\nname = "${name}"\nversion = "${version}"\n${deps.length ? `dependencies = [\n${[...deps].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))).map((d) => ` "${d}",`).join("\n")}\n]\n` : ""}`;
const cmp = (a: string, b: string) => Buffer.compare(Buffer.from(a), Buffer.from(b));
const refFor = (pkg: string, pathDep: boolean): string => {
  const hits = blocks.filter((b) => b.name === pkg);
  if (!hits.length) throw new Error(`lock ${lockPath} has no package ${pkg}`);
  if (hits.length === 1 || pathDep) return pkg;
  const preferred = hits.filter((b) => !b.path);
  throw new Error(`ambiguous ${pkg}: ${hits.map((b) => b.version).join(",")}`);
};
const initialRoots = (): Set<string> => {
  const referenced = new Set<string>();
  for (const b of blocks) for (const d of b.deps) { const [n, v] = d.split(" "); for (const t of blocks.filter((x) => x.name === n && (!v || x.version === v))) referenced.add(t.name + "@" + t.version + (t.path ? "p" : "r")); }
  return new Set(blocks.filter((b) => b.path && !referenced.has(b.name + "@" + b.version + "p")).map((b) => b.name + "@" + b.version));
};
const roots0 = initialRoots();
const raster = manifests.length ? manifests : [];
for (const m of raster) {
  const parsed = Bun.TOML.parse(readFileSync(m, "utf8")) as {package: {name: string}; dependencies?: Record<string, {package?: string}>; "dev-dependencies"?: Record<string, {package?: string}>};
  const deps = new Set<string>();
  for (const table of [parsed.dependencies ?? {}, ...(args.includes("--with-dev") ? [parsed["dev-dependencies"] ?? {}] : [])]) for (const key of Object.keys(table)) {
    const row = wsRows[key] ?? {};
    const pkg = (table[key] as {package?: string}).package ?? row.package ?? key;
    const first = blocks.find((b) => b.name === pkg && b.path) ;
    deps.add(row.path || first ? pkg : refFor(pkg, false));
  }
  const name = parsed.package.name;
  blocks = blocks.filter((b) => !(b.path && b.name === name));
  const b = parse(render(name, "0.1.0", [...deps]));
  const at = blocks.findIndex((x) => cmp(x.name, name) > 0);
  blocks.splice(at < 0 ? blocks.length : at, 0, b);
}
for (const [pkg, deps] of Object.entries(sets)) {
  const hit = blocks.find((b) => b.name === pkg && b.path);
  if (!hit) throw new Error(`no path package ${pkg}`);
  const names = deps.map((d) => { const first = blocks.find((b) => b.name === d && b.path); return first ? d : refFor(d, false); });
  const kept = hit.deps.filter((d) => !d.startsWith("semio-s-artifact-raster-raster")).concat(names);
  const nb = parse(render(pkg, hit.version, [...new Set(kept)]));
  blocks[blocks.indexOf(hit)] = nb;
}
let dropped: string[] = [];
for (;;) {
  const referenced = new Set<string>();
  for (const b of blocks) for (const d of b.deps) { const [n, v] = d.split(" "); for (const t of blocks.filter((x) => x.name === n && (!v || x.version === v))) referenced.add(t.name + "@" + t.version + (t.path ? "p" : "r")); }
  const orphans = blocks.filter((b) => !referenced.has(b.name + "@" + b.version + (b.path ? "p" : "r")) && !roots0.has(b.name + "@" + b.version) && (b.path || true));
  if (!orphans.length) break;
  dropped.push(...orphans.map((o) => o.name));
  blocks = blocks.filter((b) => !orphans.includes(b));
}
const dangling: string[] = [];
for (const b of blocks) for (const d of b.deps) { const [n, v] = d.split(" "); const hit = blocks.filter((x) => x.name === n && (!v || x.version === v)); if (hit.length !== 1) dangling.push(`${b.name} -> ${d} (${hit.length})`); }
console.log(`${lockPath}: blocks ${blocks.length}, dropped ${dropped.length} [${dropped.join(", ")}], dangling ${dangling.length}`);
for (const d of dangling.slice(0, 20)) console.log("DANGLING", d);
if (write && !dangling.length) writeFileSync(lockPath, `${header ? `${header}\n` : ""}${blocks.map((b) => b.text.trimEnd()).join("\n\n")}\n`);
