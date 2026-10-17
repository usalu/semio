// 🧰 bd10: applies the raster crate wiring plan (`raster_manifests.py` output): manifests, workspace members and rows, lock blocks, owner rows, package router.
import {readFileSync, writeFileSync, mkdirSync, existsSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import {addWorkspaceMembers, insertBuildOwnerRows, compareBytes, patchRouterPackages} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const art = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as {manifests: Record<string, string>; ws_rows: Record<string, Record<string, unknown>>};
const write = process.argv.includes("--write");
const facadeManifest = `${art}/📦️packages/🦀️rust/Cargo.toml`;
const authoring = `${art}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️authoring-law/🦀️.rs`;
const toml = (v: unknown): string => typeof v === "string" ? JSON.stringify(v) : typeof v === "boolean" ? String(v) : Array.isArray(v) ? `[${v.map(toml).join(", ")}]` : `{ ${Object.entries(v as object).map(([k, x]) => `${k} = ${toml(x)}`).join(", ")} }`;

const manifests: Record<string, string> = {...plan.manifests};
manifests[facadeManifest] = manifests[facadeManifest]!.replace(/\n\[dependencies\]/u, `\n[[test]]\nname = "authoring_law"\npath = "${relative(dirname(join(root, facadeManifest)), join(root, authoring))}"\n\n[dependencies]`);
const newCrates = Object.entries(manifests).filter(([path]) => path !== facadeManifest).map(([path, text]) => ({path, name: /^name = "([^"]+)"/mu.exec(text)![1]!, text}));
const facadeName = "semio-s-artifact-raster-raster";

const wsPath = join(root, art, "Cargo.toml");
let ws = readFileSync(wsPath, "utf8");
ws = addWorkspaceMembers(ws, newCrates.map(({path}) => relative(join(root, art), dirname(join(root, path)))));
const existing = (Bun.TOML.parse(ws) as {workspace: {dependencies?: Record<string, unknown>}}).workspace.dependencies ?? {};
const rows: Record<string, Record<string, unknown>> = {...plan.ws_rows};
for (const crate of newCrates) rows[crate.name] = {path: relative(join(root, art), dirname(join(root, crate.path)))};
const added: string[] = [];
for (const key of Object.keys(rows).sort(compareBytes)) {
  if (key in existing) continue;
  const anchor = ws.indexOf("\n[workspace.lints");
  const block = `\n[workspace.dependencies.${key}]\n${Object.entries(rows[key]!).map(([k, v]) => `${k} = ${toml(v)}\n`).join("")}`;
  ws = ws.slice(0, anchor) + block + ws.slice(anchor);
  added.push(key);
}
Bun.TOML.parse(ws);

const lockPath = join(root, art, "Cargo.lock");
let lock = readFileSync(lockPath, "utf8");
const blocks = lock.split(/\n(?=\[\[package\]\]\n)/u);
const header = blocks[0]!.startsWith("[[package]]") ? "" : blocks.shift()!;
const nameOf = (b: string) => /name = "([^"]+)"/u.exec(b)![1]!;
const isPath = (b: string) => !/^source = /mu.test(b);
const facadeBlockOld = blocks.find((b) => nameOf(b) === facadeName && isPath(b))!;
const oldFacadeDeps = [...(/dependencies = \[([\s\S]*?)\n\]/u.exec(facadeBlockOld)?.[1] ?? "").matchAll(/"([^"]+)"/gu)].map((m) => m[1]!);
const byName = new Map<string, string[]>();
for (const b of blocks) if (!isPath(b) || true) byName.set(nameOf(b), [...(byName.get(nameOf(b)) ?? []), b]);
const lockRef = (pkg: string): string => {
  const hits = oldFacadeDeps.filter((d) => d === pkg || d.startsWith(pkg + " "));
  if (hits.length === 1) return hits[0]!;
  const blocksOfName = byName.get(pkg) ?? [];
  if (blocksOfName.length <= 1) return pkg;
  throw new Error(`ambiguous lock reference for ${pkg}: ${blocksOfName.length} versions, facade has ${hits.join(",")}`);
};
const allManifests = Object.entries(manifests).map(([path, text]) => ({path, text, parsed: Bun.TOML.parse(text) as {package: {name: string; version?: unknown}; dependencies?: Record<string, {package?: string}>; "dev-dependencies"?: Record<string, {package?: string}>}}));
const wsAfter = (Bun.TOML.parse(ws) as {workspace: {dependencies: Record<string, {package?: string; path?: string}>}}).workspace.dependencies;
const lockDeps = (parsed: (typeof allManifests)[number]["parsed"]): string[] => {
  const out = new Set<string>();
  for (const table of [parsed.dependencies ?? {}, parsed["dev-dependencies"] ?? {}]) for (const key of Object.keys(table)) {
    const row = wsAfter[key] ?? {};
    const pkg = (table[key] as {package?: string}).package ?? row.package ?? key;
    out.add(row.path ? pkg : lockRef(pkg));
  }
  return [...out];
};
const render = (name: string, deps: string[]): string => `[[package]]\nname = "${name}"\nversion = "0.1.0"\n${deps.length ? `dependencies = [\n${[...deps].sort(compareBytes).map((d) => ` "${d}",`).join("\n")}\n]\n` : ""}`;
const names = new Set(allManifests.map((m) => m.parsed.package.name));
const pathPackages = new Map<string, string>();
for (const m of allManifests) for (const table of [m.parsed.dependencies ?? {}, m.parsed["dev-dependencies"] ?? {}]) for (const [key, value] of Object.entries(table as Record<string, {path?: string}>)) if (value.path && !(key in wsAfter)) pathPackages.set(key, value.path);
const kept = blocks.filter((b) => !(isPath(b) && names.has(nameOf(b)))).map((b) => b.trimEnd() + "\n");
for (const key of pathPackages.keys()) {
  if (kept.some((b) => nameOf(b) === key && isPath(b))) continue;
  const at = kept.findIndex((b) => compareBytes(nameOf(b), key) > 0);
  kept.splice(at < 0 ? kept.length : at, 0, render(key, []));
}
for (const m of allManifests) {
  const rendered = render(m.parsed.package.name, lockDeps(m.parsed));
  const at = kept.findIndex((b) => compareBytes(nameOf(b), m.parsed.package.name) > 0);
  kept.splice(at < 0 ? kept.length : at, 0, rendered);
}
lock = `${header ? `${header}\n` : ""}${kept.map((b) => b.trimEnd()).join("\n\n")}\n`;

const ownersPath = join(root, "📋️build-owners.json");
const scriptPath = join(root, art, "📦️packages/🦀️rust/📜️script.ts");
console.log(JSON.stringify({manifests: Object.keys(manifests), added, facadeLockDeps: lockDeps(allManifests.find((m) => m.path === facadeManifest)!.parsed)}, null, 1));
if (!write) process.exit(0);
for (const [path, text] of Object.entries(manifests)) { mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text); }
writeFileSync(wsPath, ws);
writeFileSync(lockPath, lock);
const owners = insertBuildOwnerRows(readFileSync(ownersPath, "utf8"), newCrates.map((c) => ({name: c.name, root: dirname(c.path)})));
writeFileSync(ownersPath, owners.text);
console.log("owners added", owners.added.join(","));
const router = patchRouterPackages(readFileSync(scriptPath, "utf8"), facadeName, newCrates.map((c) => c.name));
writeFileSync(scriptPath, router);
