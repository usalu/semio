import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const c = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const a = c.assets.find((x) => x.id === "forklift")!;
const sel = { kind: "scale", scale: { denominator: 50 } } as const;
const s = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, sel);
console.log(a.presentation.defaultPose, extentOf(s.shapes.elevation), extentOf(s.shapes.elevation, true));
for (const view of ["light-line", "moments"] as const) {
  const sc = depict(s, { ...defaultDepictionOptions(view, "en", 50), representation: sel, annotations: { dimensions: false, scaleBar: false, labels: false, legend: false, ground: false }, marginMm: 0 });
  console.log(view, sc.widthMm, sc.heightMm);
}
console.log(s.shapes.elevation.map((e) => `${e.shape.id}:${e.shape.role}`).slice(0, 60).join(" "));
