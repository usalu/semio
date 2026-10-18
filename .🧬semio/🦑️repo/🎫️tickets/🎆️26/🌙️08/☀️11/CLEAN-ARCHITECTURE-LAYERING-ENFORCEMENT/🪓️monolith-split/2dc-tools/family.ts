/** 🧮️ Runs only the compute-family ownership inspection of rust-source-direction and dumps full problems (with the offending manifest) plus a source-to-manifest map. */
import { lstatSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, posix } from "node:path";
import { inspectRustCompileReferences } from "../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { inspectRustModuleGraph } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { loadDependencyDirectionPolicy } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";
import { COMPUTE_OWNERSHIP_CONTRACT_PATH, rustFamilyOwnershipActive, inspectRustFamilyOwnership, readRustFamilyOwnershipContract } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
if (!out) throw Error("usage: family.ts <out-prefix>");
const snapshot = loadDependencyDirectionPolicy(root), { taxonomy } = snapshot;
const ignore: string[] = taxonomy.implementationLeafPolicy.ignoredPathPatterns.map((path: string) => path.replace(/^\*\*\//u, ""));
const excluded: string[] = Object.values(taxonomy.pathExclusions).map((value: { path: string }) => value.path.replace(/\/$/u, ""));
const sources = new Map<string, string>();
const inventory = new Map<string, "file" | "directory" | "symlink">([[".", "directory"]]);
const walk = (path: string): void => {
  if (ignore.some((part) => path === part || path.endsWith(`/${part}`)) || excluded.some((part) => path === part || path.startsWith(`${part}/`))) return;
  const stat = lstatSync(join(root, path));
  inventory.set(path, stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file");
  if (stat.isSymbolicLink()) return;
  if (stat.isDirectory()) { for (const entry of readdirSync(join(root, path)).sort()) walk(`${path}/${entry}`); return; }
  if (stat.isFile() && (path === COMPUTE_OWNERSHIP_CONTRACT_PATH || path.endsWith(".rs") || path === "Cargo.toml" || path.endsWith("/Cargo.toml"))) sources.set(path, readFileSync(join(root, path), "utf8"));
};
for (const area of Object.keys(taxonomy.areaLayers)) { try { lstatSync(join(root, area)); } catch { continue; } walk(area); }
for (const entry of readdirSync(root, { withFileTypes: true })) if ((entry.isFile() || entry.isSymbolicLink()) && (entry.name.endsWith(".rs") || entry.name === "Cargo.toml")) { inventory.set(entry.name, "file"); sources.set(entry.name, readFileSync(join(root, entry.name), "utf8")); }
if (process.env.CONTRACT_OVERRIDE) sources.set(COMPUTE_OWNERSHIP_CONTRACT_PATH, readFileSync(process.env.CONTRACT_OVERRIDE, "utf8"));
const sorted = new Map([...sources].sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right))));
const compileReferences = new Map<string, ReturnType<typeof inspectRustCompileReferences>>();
for (const [path, source] of sorted) { if (!path.endsWith(".rs")) continue; try { compileReferences.set(path, inspectRustCompileReferences(source, () => {})); } catch {} }
console.error(`[2dc] files=${sorted.size}`);
const graph = inspectRustModuleGraph([...sorted.keys()], (path) => sorted.get(path), { compileReferences, strictManifests: true });
console.error(`[2dc] graph built`);
const active = rustFamilyOwnershipActive({ sources: sorted, graph, inventory });
const contract = readRustFamilyOwnershipContract(JSON.parse(sorted.get(COMPUTE_OWNERSHIP_CONTRACT_PATH)!));
const problems = inspectRustFamilyOwnership({ contract, sources: sorted, graph, inventory });
writeFileSync(`${out}-problems.json`, JSON.stringify({ active, count: problems.length, problems }, null, 1));
const bySource: Record<string, string[]> = {};
for (const [path] of sorted) { if (!path.endsWith(".rs")) continue; const manifests = [...new Set((graph.contexts.get(path) ?? []).flatMap((context) => context.manifestPath ? [context.manifestPath] : []))]; if (manifests.length) bySource[path] = manifests; }
writeFileSync(`${out}-manifest-of.json`, JSON.stringify(bySource));
const counts: Record<string, number> = {};
for (const problem of problems) counts[problem.code] = (counts[problem.code] ?? 0) + 1;
console.error(`[2dc] active=${active} problems=${problems.length} ${JSON.stringify(counts)}`);
