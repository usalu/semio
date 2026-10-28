// [cachegc] approximate "closure" scope snapshot size of an entry: imports (static+dynamic, Bun scan + regex fallback) + quoted relative file literals + module-owned 🧬️schema/*.json. usage: bun closure-size.ts <root> <entry-rel> [top]
import { readFileSync, existsSync, readdirSync, statSync } from "node:fs";
import { dirname, resolve, relative, join, extname } from "node:path";
const [root, entry, top = "25"] = process.argv.slice(2);
const seen = new Map<string, number>(), stack = [resolve(root, entry)], parent = new Map<string, string>(); let scanFailures = 0;
const candidates = (p: string) => [p, p + ".ts", p + ".mjs", p + ".js", join(p, "🟦️.ts"), join(p, "🟨️.mjs")];
while (stack.length) {
  const f = stack.pop()!; if (seen.has(f)) continue; let size = 0; try { size = statSync(f).size; } catch { continue; } seen.set(f, size);
  const push = (p: string) => { for (const c of candidates(p)) if (existsSync(c) && statSync(c).isFile()) { if (!seen.has(c)) { if (!parent.has(c)) parent.set(c, f); stack.push(c); } return; } };
  const sd = join(dirname(f), "🧬️schema"); if (existsSync(sd)) for (const n of readdirSync(sd, { recursive: true }) as string[]) if (extname(n) === ".json") push(join(sd, n));
  if (extname(f) === ".json") continue;
  const src = readFileSync(f, "utf8"), specs = new Set<string>();
  try { for (const i of new Bun.Transpiler({ loader: /\.[cm]?tsx?$/.test(f) ? "ts" : "js" }).scan(src).imports) specs.add(i.path); } catch { scanFailures++; }
  for (const m of src.matchAll(/(?:from|import)\s*\(?\s*(["'])([^"'\n]+)\1/gu)) specs.add(m[2]);
  for (const s of specs) if (s.startsWith(".")) push(resolve(dirname(f), s));
  for (const m of src.matchAll(/(["'])(\.{1,2}\/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1/gu)) if (!/\.d\.[cm]?ts$/u.test(m[2])) push(resolve(dirname(f), m[2]));
}
let bytes = 0, json = 0; for (const [f, s] of seen) { bytes += s; json += Math.ceil(s / 3) * 4 + 2 * Buffer.byteLength(relative(root, f)) + 180; }
console.log(JSON.stringify({ files: seen.size, sourceBytes: bytes, approxJsonBytes: json, channelLimit: 16777216, scanFailures }));
const dirs = new Map<string, number>(); for (const [f, s] of seen) { const d = relative(root, f).split("/").slice(0, +(process.env.DEPTH ?? 7)).join("/"); dirs.set(d, (dirs.get(d) ?? 0) + s); }
for (const [d, s] of [...dirs].sort((a, b) => b[1] - a[1]).slice(0, +top)) console.log(s, d);
if (process.env.CHAIN) { let f = [...seen.keys()].find(k => k.includes(process.env.CHAIN!)); while (f) { console.log("chain", relative(root, f)); f = parent.get(f); } }
