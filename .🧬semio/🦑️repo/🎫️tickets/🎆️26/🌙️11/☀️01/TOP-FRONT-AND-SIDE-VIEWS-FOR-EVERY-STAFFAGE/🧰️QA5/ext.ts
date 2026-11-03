import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const ids = (process.env.QA5_ASSETS ?? "").split(",");
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const f = (n: number) => n.toFixed(3);
for (const id of ids) {
  const a = catalog.assets.find((x) => x.id === id)!;
  console.log("[DEBUG]", id, "dims", JSON.stringify(a.dimensions), "pose", a.presentation.defaultPose);
  for (const sc of [50, 100, 200, 500]) {
    if (a.representations[String(sc)]?.status !== "authored") continue;
    const s: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose } as any, { kind: "scale", scale: { denominator: sc } } as any);
    const row: string[] = [];
    for (const v of ["elevation", "front", "plan"]) {
      let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9, ax = "", ay = "", bx = "", by = "";
      for (const e of s.shapes[v]) { if (e.shape.role === "ground") continue; for (const p of e.rings.flat()) { if (p.x < x0) { x0 = p.x; ax = e.shape.id; } if (p.x > x1) { x1 = p.x; bx = e.shape.id; } if (p.y < y0) { y0 = p.y; ay = e.shape.id; } if (p.y > y1) { y1 = p.y; by = e.shape.id; } } }
      row.push(`${v} x[${f(x0)}(${ax}),${f(x1)}(${bx})] y[${f(y0)}(${ay}),${f(y1)}(${by})]`);
    }
    console.log("[DEBUG]", id, "1:" + sc, row.join(" | "));
  }
}
