import { readFileSync } from "node:fs";
import { parse } from "/Users/ueli/Documents/semio/node_modules/smol-toml/dist/index.js";
const root = "/Users/ueli/Documents/semio";
const C = process.env.C!;
const d = JSON.parse(readFileSync(`${C}/🗑️generated/ext-d/truth0.json`, "utf8"));
const names = process.argv.slice(2);
const cnt = new Map<string, number>(), ex = new Map<string, string>();
for (const e of d.literal) {
  if (e.ecosystem !== "rust" || !names.includes(e.name)) continue;
  for (const x of e.declarations.filter((x: any) => x.kind === "production-runtime")) {
    let t: any;
    try { t = parse(readFileSync(`${root}/${x.user}`, "utf8")); } catch { cnt.set(`${e.name} parse-error`, (cnt.get(`${e.name} parse-error`) ?? 0) + 1); continue; }
    const dep = t.dependencies?.[e.name];
    const opt = typeof dep === "object" && dep?.optional === true;
    const feats = t.features ?? {};
    const enabling = Object.entries(feats).filter(([, v]: any) => v.some((s: string) => s === `dep:${e.name}` || s === e.name || s.startsWith(`${e.name}/`) || s.startsWith(`dep:${e.name}/`))).map(([f]) => f).sort();
    const key = `${e.name} ${opt ? "optional" : dep === undefined ? "none(target?)" : "required"} [${enabling.join(",")}]`;
    cnt.set(key, (cnt.get(key) ?? 0) + 1);
    if (!ex.has(key)) ex.set(key, x.user);
  }
}
for (const [k, v] of [...cnt].sort()) console.log(v, k, (ex.get(k) ?? "").slice(-70));
