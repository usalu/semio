import { readdirSync, readFileSync } from "node:fs";
const CAT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
for (const d of readdirSync(CAT)) {
  const a = JSON.parse(readFileSync(`${CAT}/${d}/🔣️.json`, "utf8")).asset;
  const ab = JSON.stringify(a.clips).match(/abductionDeg/g)?.length ?? 0, tw = JSON.stringify(a.clips).match(/twistDeg/g)?.length ?? 0;
  const g = (k: string) => a.representations[k]?.geometry;
  const dots = ["50", "100", "200", "500"].reduce((n, k) => n + (g(k)?.plan.filter((s: any) => /dot|crown-fill/.test(s.id)).length ?? 0), 0);
  const symOk = a.symbols.every((s: any) => s.geometry.elevation?.length && s.geometry.front?.length && s.geometry.plan?.length);
  const az = a.anchors.filter((x: any) => x.z !== undefined).length;
  console.log(d.padEnd(24), "clip abduction", ab, "twist", tw, "| plan dots+fills", dots, "| symbols 3 views", symOk, "| anchors with z", az);
}
