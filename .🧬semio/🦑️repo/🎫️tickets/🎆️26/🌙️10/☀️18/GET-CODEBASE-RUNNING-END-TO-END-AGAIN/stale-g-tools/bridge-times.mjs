// 🧪️ stale-g: for each bridge importer (sealed-key file importing a file the sealed roster lacks) its mtime, so edits after the sealing time stand out. usage: bun bridge-times.mjs <raw-vs-recipe.json> <sealedAtISO>
import {readFileSync, existsSync, statSync} from "node:fs";
import {join, dirname} from "node:path";
const W = "/Users/ueli/Documents/semio"; process.chdir(W);
const d = JSON.parse(readFileSync(process.argv[2], "utf8")), sealedAt = Date.parse(process.argv[3]), sealed = new Set(d.sealed), added = new Set(d.added);
const t = new Bun.Transpiler({loader: "ts"}), tj = new Bun.Transpiler({loader: "js"});
const res = (from, spec) => { try { return Bun.resolveSync(spec, join(W, dirname(from))).slice(W.length + 1); } catch { return null; } };
const rows = [];
for (const f of d.fresh) { if (!sealed.has(f) || !/\.(ts|tsx|mjs|js)$/u.test(f) || !existsSync(f)) continue; let imports; try { imports = (/\.tsx?$/u.test(f) ? t : tj).scanImports(readFileSync(f, "utf8")).map((i) => i.path); } catch { continue; } const targets = [...new Set(imports.map((s) => res(f, s)).filter((r) => r && added.has(r)))]; if (targets.length) rows.push({f, mtime: statSync(f).mtimeMs, targets: targets.map((x) => x.slice(-70))}); }
rows.sort((a, b) => b.mtime - a.mtime);
let after = 0; for (const r of rows) { const late = r.mtime > sealedAt; if (late) after++; console.log(late ? "AFTER " : "before", new Date(r.mtime).toISOString(), r.f.slice(-80), "->", r.targets.length); }
console.log(JSON.stringify({bridges: rows.length, editedAfterSealing: after}));
