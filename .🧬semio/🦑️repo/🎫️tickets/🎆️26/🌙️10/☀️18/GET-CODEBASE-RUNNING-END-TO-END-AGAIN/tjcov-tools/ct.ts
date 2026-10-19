import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const LAW = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage`;
const mods = ["📋️inventory", "📤️publication", "🔒️lifecycle", "🏭️factory", "🗄️artifact-store", "🛣️routes", "🔁️live-replay", "⚙️fixed-operation"];
const extra: Record<string, unknown> = {};
for (const m of mods) Object.assign(extra, await import(`${LAW}/${m}/🟦️.ts`));
export const helpers = extra;
/** Evaluates each top-level `&&` clause of `return ( ... )` of `export function name(` found in lawFile, with args. */
export function clauses(lawFile: string, name: string, args: unknown[], opts: { quiet?: boolean } = {}): { total: number; bad: string[] } {
  const lines = readFileSync(lawFile, "utf8").split("\n");
  const s = lines.findIndex((l) => l.startsWith(`export function ${name}(`));
  if (s < 0) throw new Error("function not found " + name);
  const e = lines.findIndex((l, i) => i > s && l === "}");
  const fn = lines.slice(s, e + 1);
  const header = fn[0]!;
  const params = header.slice(header.indexOf("(") + 1, header.lastIndexOf("):")).split(/,\s*(?=\w+\??:)/).map((p) => p.split(":")[0]!.trim().replace("?", ""));
  const r = fn.findIndex((l) => /^\s*return\s*(!!)?\(\s*$/.test(l));
  const end = fn.findIndex((l, i) => i > r && /^\s*\);?\s*$/.test(l));
  if (r < 0) throw new Error("no return ( block in " + name);
  const out: string[] = [];
  let cur = "";
  let depthGuard = 0;
  for (const l of fn.slice(r + 1, end)) { cur += (cur ? " " : "") + l.trim(); if (/&&$/.test(cur)) { out.push(cur.replace(/\s*&&$/, "")); cur = ""; } }
  if (cur) out.push(cur);
  const prelude = fn.slice(1, r).join("\n");
  const src = `function __f(${params.join(",")}) {\n${prelude}\nreturn [${out.map((c) => `[${JSON.stringify(c)}, (()=>{try{return !!(${c})}catch(x){return "ERR "+x.message}})()]`).join(",")}];\n}`;
  const js = new Bun.Transpiler({ loader: "ts" }).transformSync(src);
  const names = Object.keys(extra).filter((k) => /^[A-Za-z_$][\w$]*$/.test(k));
  const res = new Function(...names, `${js}\nreturn __f;`)(...names.map((k) => extra[k]))(...args) as [string, unknown][];
  const bad = res.filter(([, ok]) => ok !== true).map(([c, ok]) => `${ok === false ? "FAIL" : String(ok)} ${c}`);
  if (!opts.quiet) { for (const b of bad) console.log(b.slice(0, 380)); console.log(`${name}: ${res.length} clauses, ${bad.length} failing`); }
  return { total: res.length, bad };
}
export const lawFile = (m: string) => `${LAW}/${m}/🟦️.ts`;
