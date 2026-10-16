import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
const [parent, cfgPath, infoPath, entryPath, docLine] = process.argv.slice(2);
const cfg = (await import(cfgPath)).default;
const info = JSON.parse(readFileSync(infoPath, "utf8"));
const text = readFileSync(parent, "utf8").split("\n");
const hs = text.findIndex((l) => /^\s*\/\/ ?#region .*Header/u.test(l)), he = text.findIndex((l, i) => i > hs && /^\s*\/\/ ?#endregion .*Header/u.test(l));
const header = hs >= 0 ? text.slice(hs, he + 1).join("\n") : "";
const out = [`/** ${docLine} */`, ""];
if (header) out.push(header, "");
const byMod = new Map();
for (const e of info.exports) { if (!byMod.has(e.module)) byMod.set(e.module, []); byMod.get(e.module).push(e); }
for (const m of cfg.modules) {
  const list = byMod.get(m.name); if (!list) continue;
  const spec = relative(dirname(entryPath), join(process.env.RTV_PARENT_DIR ?? dirname(parent), m.folder, "🟦️" + (cfg.ext ?? ".ts")));
  const target = JSON.stringify(spec.startsWith(".") ? spec : "./" + spec);
  const wrap = (names) => { const lines = []; let cur = "  "; for (const n of names) { if (cur.length + n.length + 2 > 150) { lines.push(cur.trimEnd()); cur = "  "; } cur += n + ", "; } lines.push(cur.trimEnd()); return lines.join("\n"); };
  const sorted = (items) => items.map((e) => e.name).sort((a, b) => a.localeCompare(b));
  const values = sorted(list.filter((e) => !e.isType)), types = sorted(list.filter((e) => e.isType));
  if (values.length) out.push(`export {\n${wrap(values)}\n} from ${target};`);
  if (types.length) out.push(`export type {\n${wrap(types)}\n} from ${target};`);
}
out.push("");
writeFileSync(entryPath, out.join("\n"));
console.log("entry", entryPath, out.length);
