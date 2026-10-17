// 🔍️ Prints the slice-building trace for one package root after the optional in-memory lift (argv[3] = comma list of lift patterns).
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {collectDecls, buildSlices} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧱️decl-tree/🟦️.ts";
import {transformTestClosure} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {liftModuleKids} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪜️lift-kids/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const manifest = resolve(ROOT, process.argv[2]!, "Cargo.toml");
const patterns = (process.argv[3] ?? "").split(",").filter(Boolean);
const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
const entry = resolve(dirname(manifest), parsed.lib?.path ?? "src/lib.rs");
const disk = (p: string) => existsSync(p) ? readFileSync(p, "utf8") : null;
const orig = disk(entry)!;
const names = new Set<string>(parseRustSourceTree(orig).modules.filter(n => !n.inline).map(n => n.name));
const tt = process.env.NOTESTS ? null : transformTestClosure(ROOT, entry, orig, names, {fixtures: true, aggregate: true, hoist: true, detach: true});
const lifted = patterns.length ? liftModuleKids(tt, entry, orig, patterns) : tt;
const readText = (p: string) => lifted ? (p === entry ? lifted.rootSource : lifted.edits.get(p) ?? lifted.creates.get(p) ?? disk(p)) : disk(p);
for (const n of lifted?.notes ?? []) if (!n.startsWith("lifted")) console.log("NOTE " + n);
const tree = collectDecls(entry, readText(entry)!, readText);
const lines: string[] = [];
const {slices} = buildSlices(tree, readText, m => lines.push(m), {detachAllTests: true});
if (process.env.TRACE) console.log(lines.join("\n"));
console.log(`slices=${slices.length}`);
for (const s of slices.sort((a, b) => b.decls.length - a.decls.length).slice(0, Number(process.env.TOP ?? 8))) console.log(`slice decls=${s.decls.length} attach=${s.attach.map(d => d.id.split("::").slice(-2).join("::")).join(" + ").slice(0, 200)}`);
