import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
const c = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const a = c.assets.find((x) => x.id === "skateboarder")!;
for (const d of [100, 200]) {
  const sel = { kind: "scale", scale: { denominator: d } } as const;
  const s = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, sel);
  const ext = (vp: "front" | "elevation", ground: boolean) => { const pts = s.shapes[vp].filter((e) => (e.shape.role === "ground") === ground).flatMap((e) => e.rings.flat()); return pts.length ? [Math.min(...pts.map((p) => p.x)), Math.max(...pts.map((p) => p.x))] : null; };
  console.log(d, "front figure", ext("front", false), "front ground-role", ext("front", true), "elev figure", ext("elevation", false));
  const sc = depict(s, { ...defaultDepictionOptions("dark-line", "en", d, "front"), representation: sel });
  const g = sc.layers.find((l) => l.id === "ground")!.items.map((i) => (i.kind === "path" ? `${i.id}:${i.commands.map((c) => ("x" in c ? c.x.toFixed(2) : "")).join(",")}` : ""));
  console.log(g, "origin", sc.metadata.origin.xMm, "k", sc.metadata.mmPerMetre);
}
