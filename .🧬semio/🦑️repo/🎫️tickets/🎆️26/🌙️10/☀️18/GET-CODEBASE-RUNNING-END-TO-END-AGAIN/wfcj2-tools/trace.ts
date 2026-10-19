import { readFileSync } from "node:fs";
import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import * as inv from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const P = `${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`;
import * as pubm from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts";
export const plugin = pubm.toolJobPluginInstanceSource(root);
export const store = pubm.toolJobStoreSource(root);
export function trace(lawPath: string, name: string, args: unknown[]) {
  const lines = readFileSync(lawPath, "utf8").split("\n");
  const s = lines.findIndex((l) => l.startsWith(`export function ${name}(`));
  const e = lines.findIndex((l, i) => i > s && l === "}");
  const fn = lines.slice(s, e + 1);
  const header = fn[0]!;
  const params = header.slice(header.indexOf("(") + 1, header.lastIndexOf("):")).split(/,\s*(?=\w+:)/).map((p) => p.split(":")[0]!.trim());
  const r = fn.findIndex((l) => l.trim() === "return (");
  const end = fn.findIndex((l, i) => i > r && l.trim() === ");");
  const clauses: string[] = [];
  let cur = "";
  for (const l of fn.slice(r + 1, end)) { cur += (cur ? " " : "") + l.trim(); if (/&&$/.test(cur)) { clauses.push(cur.replace(/\s*&&$/, "")); cur = ""; } }
  if (cur) clauses.push(cur);
  const prelude = fn.slice(1, r).join("\n");
  const src = `function __f(${params.join(",")}) {\n${prelude}\nreturn [${clauses.map((c) => `[${JSON.stringify(c)}, (()=>{try{return !!(${c})}catch(x){return "ERR "+x.message}})()]`).join(",")}];\n}`;
  const js = new Bun.Transpiler({ loader: "ts" }).transformSync(src);
  const extra = { ...inv, ...pubm } as Record<string, unknown>; const names = Object.keys(extra).filter((k) => /^[A-Za-z_$][\w$]*$/.test(k));
  const out = new Function(...names, `${js}\nreturn __f;`)(...names.map((k) => extra[k]))(...args) as [string, unknown][];
  let bad = 0;
  for (const [c, ok] of out) if (ok !== true) { bad++; console.log("FAIL", ok, c.slice(0, 300)); }
  console.log(`${name}: ${out.length} clauses, ${bad} failing`);
}
if (import.meta.main) {
  const pub = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts`;
  const which = process.argv[2];
  if (which === "store") trace(pub, "toolJobStoreBatchPublicationBounded", [store, plugin]);
  if (which === "eph") trace(pub, "toolJobEphemeralOneItemPublicationBounded", [store, plugin]);
  if (which === "full") trace(pub, "toolJobFullOperationBounded", [plugin]);
  if (which === "found") trace(pub, "toolJobTypedPersistentFoundation", [plugin]);
}
