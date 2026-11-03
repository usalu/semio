import { writeFileSync } from "node:fs";
import { measureSuitability } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const out: string[] = [];
for (const id of (process.env.QA4_ASSETS ?? "").split(",")) {
  const asset = catalog.assets.find((a) => a.id === id)!;
  for (const [scale, rep] of Object.entries(asset.representations)) {
    if ((rep as any).status !== "authored") continue;
    for (const view of ["elevation", "front", "plan"]) {
      const clone: any = structuredClone(asset);
      const g = clone.representations[scale].geometry;
      for (const v of ["elevation", "front", "plan"]) if (v !== view) g[v] = [];
      let s;
      try { s = measureSuitability(clone, clone.representations[scale], Number(scale), DEFAULT_PRINT_PROFILE); } catch (e) { out.push(`${id} 1:${scale} ${view}: ${(e as Error).message}`); continue; }
      for (const v of s.violations) {
        out.push(`${id} 1:${scale} ${view} ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}`);
        for (const pose of asset.poses.filter((q) => !q.key)) {
          const st: any = evaluate(asset, { kind: "pose", pose: pose.id } as any, { kind: "scale", scale: { denominator: Number(scale) } } as any);
          for (const sid of v.shapes) {
            const sh = st.shapes[view].find((q: any) => q.shape.id === sid);
            if (!sh) continue;
            const pts = sh.rings.flat();
            const f = (a: number[]) => (Math.max(...a) - Math.min(...a)).toFixed(3);
            out.push(`    ${pose.id} ${sid}: w ${f(pts.map((q: any) => q.x))} h ${f(pts.map((q: any) => q.y))}`);
          }
        }
      }
    }
  }
}
writeFileSync(process.env.QA4_LOG!, out.join("\n"));
