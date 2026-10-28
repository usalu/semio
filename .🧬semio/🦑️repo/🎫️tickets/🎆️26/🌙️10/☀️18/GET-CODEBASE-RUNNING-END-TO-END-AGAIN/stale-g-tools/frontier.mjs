// 🧪️ stale-g: for the files a fresh closure adds, which fresh files import them (bun's scanner, relative + workspace specifiers), flagging importers that are inside the SEALED closure (the bridge edges). usage: bun frontier.mjs <sealed-vs-fresh.json>
import {readFileSync, existsSync, statSync} from "node:fs";
import {join, dirname, resolve} from "node:path";
const W = "/Users/ueli/Documents/semio"; process.chdir(W);
const d = JSON.parse(readFileSync(process.argv[2], "utf8")), sealed = new Set(d.sealed), added = new Set(d.added), fresh = new Set(d.fresh);
const t = new Bun.Transpiler({loader: "ts"}), tj = new Bun.Transpiler({loader: "js"});
const resolveSpec = (from, spec) => { try { return Bun.resolveSync(spec, join(W, dirname(from))).slice(W.length + 1); } catch { return null; } };
const importers = new Map(); let scanned = 0, resolved = 0;
for (const f of fresh) { if (!/\.(ts|tsx|mjs|js)$/u.test(f) || !existsSync(join(W, f))) continue; let imports; try { imports = (/\.tsx?$/u.test(f) ? t : tj).scanImports(readFileSync(join(W, f), "utf8")).map((i) => i.path); } catch { continue; } scanned++;
  for (const spec of imports) { const r = resolveSpec(f, spec); if (!r) continue; resolved++; if (added.has(r)) (importers.get(r) ?? importers.set(r, new Set()).get(r)).add(f); } }
console.log({scanned, resolved});
const bridge = new Map();
for (const [r, set] of importers) for (const f of set) if (sealed.has(f)) (bridge.get(f) ?? bridge.set(f, []).get(f)).push(r);
console.log("added files with an importer:", importers.size, "of", added.size, "; bridge importers inside the sealed key:", bridge.size);
for (const [f, rs] of bridge) { console.log(f); for (const r of rs.slice(0, 5)) console.log("    ->", r); }
const orphan = [...added].filter((a) => !importers.has(a)); console.log("added without any fresh importer:", orphan.length); for (const o of orphan.slice(0, 15)) console.log("   ?", o);
