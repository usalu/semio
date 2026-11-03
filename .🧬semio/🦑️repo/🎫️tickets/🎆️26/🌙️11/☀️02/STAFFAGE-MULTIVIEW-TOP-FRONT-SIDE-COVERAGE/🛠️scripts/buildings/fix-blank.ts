import { readFileSync, writeFileSync } from "node:fs";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components";
const f = `${R}/tilt-window/🔣️.json`;
let t = readFileSync(f, "utf8");
const sill = (kind: "elevation" | "front", x: number, w: number) => `"${kind}": [
            {
              "id": "sill-block",
              "bone": "sill",
              "role": "body",
              "z": 0.5,
              "filled": true,
              "tone": 0.6,
              "kind": "rect",
              "x": ${x},
              "y": 0,
              "w": ${w},
              "h": 0.05,
              "region": "frame-sill"
            },
            {
              "id": "window-outline",`;
for (const [view, x, w] of [["elevation", -0.15, 0.3], ["front", -0.45, 0.9]] as const) {
  const a = `"${view}": [\n            {\n              "id": "window-outline",`;
  const n = t.split(a).length - 1;
  console.log(view, "occurrences", n);
  t = t.split(a).join(sill(view, x, w));
}
const d = `        "denominators": [\n          200,\n          100,\n          50\n        ],`;
console.log("denominators", t.split(d).length - 1);
t = t.replace(d, `        "denominators": [\n          500,\n          200,\n          100,\n          50\n        ],`);
writeFileSync(f, t);

const g = `${R}/stair-flight/🔣️.json`;
let s = readFileSync(g, "utf8");
const old = s.indexOf('"id": "stringer-stroke"');
const doc = JSON.parse(s);
void doc;
console.log("stringer-stroke at", old);
