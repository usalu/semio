import { readFileSync, writeFileSync } from "node:fs";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components";
const pairs: [string, string][] = [
  ["seen from the left (lateral x: exterior at the right,", "seen from the front, from ahead (lateral x is the asset's lateral z with its left on the right of the sheet: exterior at the left,"],
  ["(lateral y: exterior at -y)", "(lateral y is z with the asset's left up the sheet: exterior at -z, the bottom of the sheet)"],
  ["von links gesehen (seitliches x: außen rechts bei", "von vorn gesehen (seitliches x ist z, die Linke des Assets liegt auf dem Blatt rechts: außen links bei"],
  ["innen links bei +0,075 m)", "innen rechts bei +0,075 m)"],
  ["(seitliches y: außen bei -y)", "(seitliches y ist z, die Linke des Assets liegt auf dem Blatt oben: außen bei -z, unten auf dem Blatt)"],
];
for (const id of ["curtain-wall-bay", "masonry-wall-with-lintel"]) {
  const f = `${R}/${id}/🔣️.json`;
  let t = readFileSync(f, "utf8");
  const before = t;
  for (const [a, b] of pairs) t = t.split(a).join(b);
  const left = (t.match(/exterior at the right|exterior at -y|außen rechts bei|außen bei -y/g) ?? []).length;
  console.log(id, "changed", t !== before, "stale left", left);
  writeFileSync(f, t);
}
