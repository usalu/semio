/** 🏷️ Which workspace package owns a framework file, and is the file one of its export targets? Usage: bun owners.ts <plan.json> */
import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const F = process.argv[2];
const list = readFileSync(F + "/🗑️generated/sdkb-framework-package-jsons.txt", "utf8").split("\n").filter(Boolean);
type P = { name: string; dir: string; owner: string; targets: Set<string> };
const pkgs: P[] = [];
for (const rel of list) {
  let j: any; try { j = JSON.parse(readFileSync(ROOT + rel, "utf8")); } catch { continue; }
  if (!j.name) continue;
  const dir = rel.replace(/\/package\.json$/, "");
  const owner = dir.includes("/📦️packages/") ? dir.split("/📦️packages/")[0] : dir;
  const targets = new Set<string>();
  const collect = (v: any) => { if (typeof v === "string") targets.add((dir + "/" + v.slice(2)).normalize("NFC")); else if (Array.isArray(v)) v.forEach(collect); else if (v && typeof v === "object") Object.values(v).forEach(collect); };
  collect(j.exports);
  pkgs.push({ name: j.name, dir, owner, targets });
}
const owners = [...new Set(pkgs.map(p => p.owner))].sort((a, b) => b.length - a.length);
export function ownerOf(file: string): { owner: string; pkgs: P[] } | null {
  const o = owners.find(x => file.startsWith(x + "/"));
  return o ? { owner: o, pkgs: pkgs.filter(p => p.owner === o) } : null;
}
if (process.argv[3] !== "lib") {
  const plan = JSON.parse(readFileSync(F + "/🗑️generated/sdkb-plan1.json", "utf8"));
  const rows = new Map<string, { names: string[]; n: number }>();
  for (const f of plan.facades) for (const a of f.adds ?? []) {
    const t = a.from.normalize("NFC");
    const o = ownerOf(t);
    if (!o) continue;
    const exported = o.pkgs.some(p => p.targets.has(t));
    if (!exported) { const e = rows.get(o.pkgs.map(p => p.name).join("+") + " @ " + o.owner) ?? { names: [], n: 0 }; e.n++; e.names.push(t.split("/").slice(-4, -1).join("/")); rows.set(o.pkgs.map(p => p.name).join("+") + " @ " + o.owner, e); }
  }
  for (const [k, v] of rows) console.log(v.n, k.replace(ROOT, ""), "\n   ", v.names.join(", ").slice(0, 400));
}
