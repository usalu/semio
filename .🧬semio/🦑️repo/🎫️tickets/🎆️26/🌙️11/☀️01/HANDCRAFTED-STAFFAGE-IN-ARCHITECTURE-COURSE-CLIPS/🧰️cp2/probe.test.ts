import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { it } from "vitest";
import { catalogAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermalAt } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

it("probe", () => {
  const asset = catalogAsset("cooking-pot");
  const clip = asset.clips.find((c) => c.id === "heat-to-boil")!;
  const rows: string[] = [];
  const n = 80;
  for (let i = 0; i <= n; i++) {
    const t = 300 + (35 * i) / n;
    const f = solveThermalAt(asset, { kind: "clip", clip: clip.id, timeS: t });
    const r = f.regions as Record<string, { surfaceTempC: number; phaseChangeW: number; phaseChangedKg: number; latentW: number }>;
    const w = r.water!;
    rows.push(`${t.toFixed(0)}\twater ${w.surfaceTempC.toFixed(2)}\tphaseW ${w.phaseChangeW.toFixed(0)}\tlatentW ${w.latentW.toFixed(0)}\tvapourG ${(w.phaseChangedKg * 1000).toFixed(1)}\tbase ${r.base!.surfaceTempC.toFixed(1)}\twall ${r.wall!.surfaceTempC.toFixed(1)}`);
  }
  writeFileSync(resolve(import.meta.dirname, "../🗑️generated/cp2/probe.txt"), rows.join("\n"));
});
