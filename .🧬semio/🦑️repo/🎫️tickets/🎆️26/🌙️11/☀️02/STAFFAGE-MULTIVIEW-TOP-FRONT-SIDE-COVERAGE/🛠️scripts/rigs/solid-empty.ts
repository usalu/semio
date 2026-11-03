import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { Asset, AuthoredRepresentation } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const ROOT = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const bad: string[] = [];
let solids = 0;
for (const g of readdirSync(ROOT, { withFileTypes: true }).filter((e) => e.isDirectory()))
  for (const f of readdirSync(join(ROOT, g.name), { withFileTypes: true }).filter((e) => e.isDirectory())) {
    const file = join(ROOT, g.name, f.name, "🔣️.json");
    if (!existsSync(file)) continue;
    const asset = (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset;
    for (const r of Object.values(asset.representations).filter((x): x is AuthoredRepresentation => x.status === "authored"))
      for (const pose of asset.poses) {
        const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: r.denominator } });
        for (const view of ["elevation", "front", "plan"] as const)
          for (const s of state.shapes[view]) if (s.shape.solid !== undefined || s.shape.kind === "member") {
            solids++;
            if (s.rings.length === 0 || s.rings.every((ring) => ring.length < 3)) bad.push(`${asset.id} 1:${r.denominator} ${pose.id} ${view} ${s.shape.id} (${s.shape.solid?.kind ?? "member"})`);
          }
      }
  }
console.log(JSON.stringify({ solids, empty: bad.length, assets: [...new Set(bad.map((b) => b.split(" ")[0]))], first: bad.filter((b) => !b.startsWith("excavator")).slice(0, 12) }, null, 1));
