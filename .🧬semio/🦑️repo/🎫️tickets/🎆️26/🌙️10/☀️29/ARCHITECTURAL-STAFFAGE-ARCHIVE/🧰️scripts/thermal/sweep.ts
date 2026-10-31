import { writeFileSync } from "node:fs";
import { catalogAsset, catalogIds } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌡️thermal/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal, solveThermalAt } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const out = process.argv[2]!;
const rows: any[] = [];
const range = (xs: number[]) => (xs.length ? [Math.min(...xs), Math.max(...xs)].map((x) => +x.toFixed(1)) : null);
for (const id of catalogIds()) {
  const asset = catalogAsset(id);
  const queries: any[] = asset.poses.filter((p) => !p.key).map((p) => ({ kind: "pose", pose: p.id }));
  const clip = asset.clips[0];
  if (clip) queries.push({ kind: "clip", clip: clip.id, timeS: clip.durationS * 0.37 });
  for (const q of queries) {
    const row: any = { asset: id, category: asset.category, query: q.kind === "pose" ? q.pose : `${q.clip}@${q.timeS.toFixed(2)}`, ok: true, flags: [] as string[] };
    const t0 = performance.now();
    try {
      const f = q.kind === "pose" ? solveThermal(evaluate(asset, q)) : solveThermalAt(asset, q);
      row.ms = Math.round(performance.now() - t0);
      const rs = Object.values(f.regions);
      row.surface = range(rs.map((r) => r.surfaceTempC));
      row.skin = range(rs.flatMap((r) => (r.skinTempC === null ? [] : [r.skinTempC])));
      row.emitted = range(rs.map((r) => r.emittedRadiantWM2));
      row.net = range(rs.map((r) => r.netRadiativeWM2));
      row.sensible = range(rs.map((r) => r.sensibleWM2));
      row.latent = range(rs.map((r) => r.latentWM2));
      row.moisture = range(rs.map((r) => r.moistureGM2H));
      row.core = f.body ? +f.body.coreTempC.toFixed(2) : null;
      row.meanSkin = f.body ? +f.body.meanSkinTempC.toFixed(2) : null;
      row.passengers = Object.fromEntries(Object.entries(f.passengers).map(([k, b]) => [k, [+b.coreTempC.toFixed(2), +b.meanSkinTempC.toFixed(2)]]));
      const air = f.environment.airTempC;
      const bad = JSON.stringify(f, (_, v) => (typeof v === "number" && !Number.isFinite(v) ? "NaN" : v)).includes('"NaN"');
      if (bad) row.flags.push("non-finite");
      for (const r of rs) {
        const region = asset.thermal.regions.find((x) => x.id === r.region)!;
        if (r.skinTempC !== null && r.skinTempC < 25 && air >= 18) row.flags.push(`${r.region} skin ${r.skinTempC.toFixed(1)}`);
        if (r.surfaceTempC > 600) row.flags.push(`${r.region} surface ${r.surfaceTempC.toFixed(0)} C`);
        if (r.surfaceTempC < air - 40 || r.surfaceTempC > air + 60) if (r.surfaceTempC <= 600) row.flags.push(`${r.region} surface ${r.surfaceTempC.toFixed(1)} vs air ${air}`);
        if (region.model !== "leaf" && r.latentWM2 < -1e-9) row.flags.push(`${r.region} latent ${r.latentWM2.toFixed(1)}`);
        if (region.model === "leaf" && r.surfaceTempC > air + 25) row.flags.push(`${r.region} leaf ${r.surfaceTempC.toFixed(1)}`);
        if (Math.abs(r.netRadiativeWM2) > 400) row.flags.push(`${r.region} net rad ${r.netRadiativeWM2.toFixed(0)}`);
      }
      if (f.body && (f.body.coreTempC < 35.8 || f.body.coreTempC > 41.5)) row.flags.push(`core ${f.body.coreTempC.toFixed(2)}`);
    } catch (e) {
      row.ok = false;
      row.error = (e as Error).message;
    }
    rows.push(row);
  }
}
writeFileSync(out, JSON.stringify(rows, null, 1));
console.log("[DEBUG] rows", rows.length, "failed", rows.filter((r) => !r.ok).length, "flagged", rows.filter((r) => r.flags.length).length);
