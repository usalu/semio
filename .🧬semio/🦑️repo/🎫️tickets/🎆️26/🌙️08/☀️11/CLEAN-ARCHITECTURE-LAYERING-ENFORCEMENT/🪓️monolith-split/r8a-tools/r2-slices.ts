// 🔭️ Prints the slice unions (trace) for one artifact package with mutation conversion on or off.
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { buildSlices, collectDecls } = await import(`${L}/🧱️decl-tree/🟦️.ts`);
const { transformTestClosure } = await import(`${L}/🧪️test-closure/🟦️.ts`);
const { convertMutationLeaves } = await import(`${L}/🧬️mutation-families/🟦️.ts`);
const { parseRustSourceTree } = await import(`${L}/🌳️module-tree/🟦️.ts`);
const [pkg, mode = "on", filter = ""] = process.argv.slice(2);
const repo = "/Users/ueli/Documents/semio";
const manifest = resolve(repo, pkg!, "Cargo.toml");
const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
const entry = resolve(dirname(manifest), parsed.lib.path);
const src = readFileSync(entry, "utf8");
const names = new Set<string>();
const tr = transformTestClosure(repo, entry, src, names, { fixtures: true, aggregate: true, hoist: true, detach: true });
const rootSource = tr.rootSource;
const overlay = new Map<string, string>([...tr.edits, ...tr.creates]);
const readText = (a: string): string | null => a === entry ? rootSource : overlay.get(a) ?? (existsSync(a) ? readFileSync(a, "utf8") : null);
const fam = mode === "on" ? convertMutationLeaves(entry, rootSource, readText) : null;
for (const [f, t] of fam?.edits ?? []) overlay.set(f, t);
if (fam) for (const r of fam.report ?? []) console.log("report:", JSON.stringify(r).slice(0, 300));
const detaching = new Set<string>((fam?.aggregates ?? []).map((a: any) => a.name));
const tree = collectDecls(entry, rootSource, readText);
const out = buildSlices(tree, readText, (m: string) => { if (!filter || m.includes(filter)) console.log("trace:", m.replace(/\/Users\/ueli\/Documents\/semio\//g, "").slice(-260)); }, { detachTestsMentioning: detaching, detachAllTests: true });
console.log("slices", out.slices.length);
for (const s of out.slices) {
  const files = s.decls.filter((d: any) => d.file).length;
  console.log(`slice ${s.attach.map((d: any) => d.id.split("::").slice(-3).join("::")).join(" | ").slice(0, 120)} decls=${s.decls.length} files=${files}`);
}
for (const a of fam?.aggregates ?? []) {
  const d = [...tree.decls.values()].find((x: any) => x.file === a.file);
  console.log("aggregate", a.name, "effPath", a.effPath.join("::"), "decl.id", d?.id, "eff", d?.eff.join("::"), "isComponent", d?.isComponent, "raw", d?.raw.join("::"));
}
const fx = tr.notes.filter((n: string) => n.startsWith("runtime fixtures"));
console.log("fixture notes", fx.length, fx.slice(0, 3).map((n: string) => n.slice(-90)));
console.log("other notes", tr.notes.filter((n: string) => !n.startsWith("runtime fixtures") && !n.startsWith("made ")).map((n: string) => n.slice(0, 200)));
const lifts = [...overlay.keys()].filter((k) => k.includes("move-vertex") && k.includes("tests"));
for (const k of lifts) console.log("overlay", k.slice(-60), "include_str:", (overlay.get(k)!.match(/include_str/g) ?? []).length, "runtime_fixture_text:", (overlay.get(k)!.match(/runtime_fixture_text/g) ?? []).length);
const fx2 = readText([...tree.decls.values()].find((d: any) => d.file?.includes("move-vertex/🧪️tests"))?.file ?? "");
console.log("readText sample include_str:", (fx2?.match(/include_str/g) ?? []).length);
