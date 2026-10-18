// 🔭️ Builds only the declaration tree and slices of the converted plan (no measuring) and prints slice sizes by file count plus the union traces that fuse slices.
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { parseRustSourceTree, identifierSet } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { collectDecls, buildSlices, superReach } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧱️decl-tree/🟦️.ts";
import { transformTestClosure, hostPrivate, declaredNames } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { liftModuleKids } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪜️lift-kids/🟦️.ts";
import { convertMutationLeaves } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
import { readCargoLib, topLevelNames } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const pkg = process.argv[2]!;
const convert = (process.argv[3] ?? "on") === "on";
const lib = readCargoLib(resolve(repoRoot, pkg, "Cargo.toml"));
const originalRootSource = readFileSync(lib.entry, "utf8");
const originalTree = parseRustSourceTree(originalRootSource);
const rootItemNames = new Set<string>([...topLevelNames(originalRootSource, originalTree.items)].filter(([, item]) => item.kind !== "use").map(([name]) => name));
for (const node of originalTree.modules) if (!node.inline) rootItemNames.add(node.name);
const testTransform = transformTestClosure(repoRoot, lib.entry, originalRootSource, rootItemNames, { fixtures: true, aggregate: true, hoist: true, detach: true } as any);
const lifts = (process.env.LIFT ?? "").split(",").filter(Boolean);
const transform = lifts.length ? liftModuleKids(testTransform, lib.entry, originalRootSource, lifts) : testTransform;
if (process.env.NOTES) for (const n of transform.notes) console.log("note", n.slice(0, 260));
const rootSource = transform.rootSource;
const overlay = new Map<string, string>([...transform.edits, ...transform.creates]);
const readText = (absolute: string): string | null => { if (absolute === lib.entry) return rootSource; const v = overlay.get(absolute); if (v !== undefined) return v; return existsSync(absolute) ? readFileSync(absolute, "utf8") : null; };
const family = convert ? convertMutationLeaves(lib.entry, rootSource, readText) : null;
for (const [file, text] of [...(family?.edits ?? []), ...(family?.creates ?? [])]) overlay.set(file, text);
const detaching = new Set<string>((family?.aggregates ?? []).filter(() => (family?.converted.length ?? 0) > 0).map((a) => a.name));
const declTree = collectDecls(lib.entry, rootSource, readText);
if (process.env.DEBUG_DETACH) {
  for (const decl of declTree.decls.values()) {
    if (!decl.id.includes(process.env.DEBUG_DETACH) || !decl.testOnly) continue;
    const text = decl.file ? readText(decl.file) : null;
    const parent = decl.parentId ? declTree.decls.get(decl.parentId) : null;
    console.log("DEBUG", decl.id, JSON.stringify({ file: decl.file !== null, inline: decl.node.inline, reach: text === null ? 0 : superReach(text), parent: parent?.id, parentTest: parent?.testOnly, parentRootless: parent ? (parent.parentId === null && !parent.node.inline) : null, declFile: decl.declFile, kids: decl.children.length }));
    const hostText = readText(decl.declFile) ?? "";
    console.log("DEBUG hostText bytes", hostText.length, "parent.eff", parent?.eff);
    console.log("DEBUG hostPrivate", JSON.stringify(hostPrivate(hostText, identifierSet(text ?? ""), declaredNames(text ?? ""), text, parent?.eff ?? null)));
    console.log("DEBUG head", JSON.stringify((text ?? "").split("\n").slice(0, 9)));
  }
}
const traces: string[] = [];
const built = buildSlices(declTree, readText, (m) => traces.push(m), { detachTestsMentioning: detaching, detachAllTests: true });
const rows = built.slices.map((slice) => {
  const files = new Set<string>();
  for (const d of slice.decls) if (d.file) files.add(d.file);
  return { id: slice.id, attach: slice.attach.map((d) => d.eff.join("::")), decls: slice.decls.length, files: files.size };
}).sort((a, b) => b.files - a.files);
console.log("slices", rows.length);
for (const r of rows.slice(0, 40)) console.log(r.files, r.decls, r.attach.slice(0, 3).join(" + ").slice(-120));
const only = process.env.TRACE_FILTER;
for (const t of traces) if (!only || t.includes(only)) console.log("trace", t.slice(0, 400));
if (process.env.FIND_DECL) { for (const slice of built.slices) if (slice.decls.some((d) => d.id.endsWith(process.env.FIND_DECL!))) console.log("FOUND", slice.attach.map((d) => d.id).join(" + "), "| decls", slice.decls.length, slice.decls.slice(0, 6).map((d) => d.id.split("::").slice(-3).join("::")).join(", ")); process.exit(0); }
const pick = process.env.SLICE;
const biggest = pick ? built.slices.find((s) => s.attach.some((d) => d.eff.join("::") === pick))!  : built.slices.map((slice) => ({ slice, files: new Set(slice.decls.filter((d) => d.file).map((d) => d.file)).size })).sort((a, b) => b.files - a.files)[Number(process.env.NTH ?? 0)]!.slice;
const dist = new Map<string, { files: Set<string>; test: number }>();
const depth = Number(process.env.DEPTH ?? 2);
for (const d of biggest.decls) {
  const path = d.eff.filter((segment) => segment !== "component");
  const key = path.slice(6, 6 + depth).join("::");
  const row = dist.get(key) ?? { files: new Set<string>(), test: 0 };
  if (d.file) row.files.add(d.file);
  if (d.testOnly) row.test += 1;
  dist.set(key, row);
}
console.log("-- distribution of slice", biggest.attach.map((d) => d.eff.join("::")).join(",").slice(-80));
for (const [k, v] of [...dist].sort((a, b) => b[1].files.size - a[1].files.size).slice(0, 40)) console.log(v.files.size, "tests", v.test, k);
if (process.env.SUPERLIST) {
  const out: string[] = [];
  for (const d of declTree.decls.values()) {
    if (!d.file) continue;
    const text = readText(d.file);
    if (text === null) continue;
    const reach = superReach(text);
    if (reach === 0) continue;
    const lines = text.split("\n").map((line, index) => ({ line, index })).filter(({ line }) => /\bsuper::/u.test(line) && !/^\s*(\/\/|\/\*|\*)/u.test(line)).slice(0, 4).map(({ line, index }) => `${index + 1}: ${line.trim().slice(0, 150)}`);
    out.push(`${reach}\t${d.eff.filter((s) => s !== "component").join("::").replace("standards::v1::subsets::any::", "")}\t${d.file.replace(/.*\/subsets\/[^/]*\//u, "")}\t${lines.join(" | ")}`);
  }
  console.log("SUPERLIST " + out.length);
  for (const row of out) console.log("SL\t" + row);
}
if (process.env.DUMPDECLS) {
  const rows: string[] = [];
  for (const d of biggest.decls) { if (!d.file) continue; const text = readText(d.file) ?? ""; rows.push(`${text.split("\n").length}\t${d.testOnly ? "T" : "L"}\t${d.eff.filter((s) => s !== "component").join("::")}`); }
  rows.sort((a, b) => Number(b.split("\t")[0]) - Number(a.split("\t")[0]));
  console.log("DECLS " + rows.length);
  for (const row of rows.slice(0, Number(process.env.DUMPDECLS))) console.log("D\t" + row);
}
