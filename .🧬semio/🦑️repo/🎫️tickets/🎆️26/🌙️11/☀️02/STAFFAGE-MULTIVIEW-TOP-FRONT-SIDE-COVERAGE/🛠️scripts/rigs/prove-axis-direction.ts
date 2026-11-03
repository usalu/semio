import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { Asset, AuthoredRepresentation } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate as after } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { evaluate as before } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/before.tmp.ts";

const ROOT = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
let worst = 0;
let compared = 0;
let assets = 0;
for (const g of readdirSync(ROOT, { withFileTypes: true }).filter((e) => e.isDirectory()))
  for (const f of readdirSync(join(ROOT, g.name), { withFileTypes: true }).filter((e) => e.isDirectory())) {
    const file = join(ROOT, g.name, f.name, "🔣️.json");
    if (!existsSync(file)) continue;
    const text = readFileSync(file, "utf8");
    if (!text.includes('"revolved"')) continue;
    assets++;
    const asset = (JSON.parse(text) as { asset: Asset }).asset;
    for (const r of Object.values(asset.representations).filter((x): x is AuthoredRepresentation => x.status === "authored"))
      for (const pose of asset.poses)
        for (const view of ["elevation", "front", "plan"] as const) {
          const sel = { kind: "scale", scale: { denominator: r.denominator } } as const;
          const a = before(asset, { kind: "pose", pose: pose.id }, sel).shapes[view].flatMap((s) => s.rings.flat());
          const b = after(asset, { kind: "pose", pose: pose.id }, sel).shapes[view].flatMap((s) => s.rings.flat());
          compared++;
          if (a.length !== b.length) worst = Infinity;
          for (let i = 0; i < a.length; i++) worst = Math.max(worst, Math.hypot(a[i]!.x - b[i]!.x, a[i]!.y - b[i]!.y) * 1000);
        }
  }
console.log(JSON.stringify({ assets, viewsCompared: compared, worstVertexDifferenceMm: worst }));
