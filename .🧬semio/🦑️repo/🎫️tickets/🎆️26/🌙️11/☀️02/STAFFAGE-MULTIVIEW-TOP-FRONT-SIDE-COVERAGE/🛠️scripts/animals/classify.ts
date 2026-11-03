import { readFileSync } from "node:fs";
import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , file] = process.argv;
const cur = { id: "", scale: 0 };
const tally = new Map<string, number>();
const rows: string[] = [];
const A = Object.fromEntries(catalog().map((a) => [a.id, a]));
for (const line of readFileSync(file, "utf8").split("\n")) {
  const h = line.match(/^## (\S+) 1:(\d+)/);
  if (h) { cur.id = h[1]; cur.scale = +h[2]; continue; }
  const m = line.match(/^\s+(\S+) (plan-x|front-y) n=(\d+) worst=([\d.]+)/);
  if (!m) continue;
  const [, bone, lab, n, w] = m;
  const view = lab === "plan-x" ? "plan" : "front";
  const r: any = (A[cur.id].representations as any)[cur.scale];
  const bd = A[cur.id].rig.bones.find((b) => b.id === bone)!;
  const shs = r.geometry[view].filter((s: any) => s.bone === bone && s.role !== "ground" && s.role !== "texture");
  const desc = shs.map((s: any) => `${s.kind}${s.followTip ? "+tip" : ""}${s.solid ? "+" + s.solid.kind : ""}`).join(",");
  const key = `${view} ${desc.replace(/(\w+(\+\w+)*)(,\1)+/g, (x) => x.split(",")[0] + "x" + x.split(",").length)}`;
  tally.set(key, (tally.get(key) ?? 0) + 1);
  rows.push(`${cur.id} 1:${cur.scale} ${bone} ${view} worst=${w} len=${bd.lengthM} ang=${bd.angleDeg} :: ${desc}`);
}
console.log([...tally].sort((a, b) => b[1] - a[1]).map(([k, v]) => `${v}\t${k}`).join("\n"));
console.log(rows.length);
require("node:fs").writeFileSync(file + ".classified.txt", rows.join("\n"));
