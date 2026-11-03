import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { it } from "vitest";
import { catalogAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermalAt } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

it("probe", () => {
  const asset = catalogAsset("cooking-pot");
  const clip = asset.clips.find((c) => c.id === "heat-to-boil")!;
  const rows: string[] = [];
  const n = 24;
  for (let i = 0; i <= n; i++) {
    const t = (clip.durationS * i) / n;
    const f = solveThermalAt(asset, { kind: "clip", clip: clip.id, timeS: t });
    const r = f.regions as Record<string, { surfaceTempC: number }>;
    rows.push(`${t.toFixed(0)}\twater ${r.water!.surfaceTempC.toFixed(1)}\tbase ${r.base!.surfaceTempC.toFixed(1)}\twall ${r.wall!.surfaceTempC.toFixed(1)}\thandle ${r.handle!.surfaceTempC.toFixed(1)}\t${JSON.stringify((f as any).emitters ?? {}).slice(0, 400)}`);
  }
  const steady = solveThermalAt(asset, { kind: "pose", pose: "boiling" } as never);
  rows.push("steady boiling pose water " + (steady.regions as any).water.surfaceTempC);
  writeFileSync(resolve(import.meta.dirname, "../🗑️generated/cp/probe.txt"), rows.join("\n"));
});
