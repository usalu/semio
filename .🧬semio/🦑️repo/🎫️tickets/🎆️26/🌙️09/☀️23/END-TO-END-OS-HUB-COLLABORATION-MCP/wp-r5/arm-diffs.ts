import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧫️fixtures";
const short = (v: unknown) => { const s = JSON.stringify(v); return s.length > 160 ? s.slice(0, 160) + "…" : s; };
const diff = (a: any, b: any, path: string, out: string[]) => {
  if (JSON.stringify(a) === JSON.stringify(b)) return;
  if (Array.isArray(a) && Array.isArray(b)) { if (a.length !== b.length) { out.push(`${path} len ${a.length}->${b.length}`); return; } a.forEach((x, i) => diff(x, b[i], `${path}/${i}`, out)); return; }
  if (a && b && typeof a === "object" && typeof b === "object") { for (const k of new Set([...Object.keys(a), ...Object.keys(b)])) diff(a[k], b[k], `${path}/${k}`, out); return; }
  out.push(`${path}: ${short(a)} -> ${short(b)}`);
};
for (const d of readdirSync(F).filter((x) => x.endsWith("-applied"))) {
  const before = JSON.parse(readFileSync(join(F, d, "⬅️before.json"), "utf8"));
  const after = JSON.parse(readFileSync(join(F, d, "➡️after.json"), "utf8"));
  const mutation = JSON.parse(readFileSync(join(F, d, "🦠️mutation.json"), "utf8"));
  const out: string[] = []; diff(before, after, "", out);
  console.log(`== ${d} ${short(mutation.payload.mutation)}`); for (const l of out.slice(0, 6)) console.log("   ", l);
}
