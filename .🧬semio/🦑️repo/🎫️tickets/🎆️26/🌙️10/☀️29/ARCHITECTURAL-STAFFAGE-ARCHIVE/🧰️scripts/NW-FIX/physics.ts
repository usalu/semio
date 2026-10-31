/** [DEBUG] Structure residuals and thermal surfaces for the assets under repair. */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
import { solveThermal, solveThermalAt } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const ids = "panel-radiator,delivery-van,motor-scooter,heat-pump-outdoor-unit,city-car,city-bus,city-bicycle,forklift,electric-kick-scooter,tram,wheel-loader,concrete-mixer-truck,excavator,diesel-generator,retractable-bollard".split(",");
const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const { readdirSync } = await import("node:fs");

function locate(id: string): string {
  for (const cat of readdirSync(root)) {
    const p = `${root}/${cat}/${id}`;
    try {
      const files = readdirSync(p);
      const doc = files.find((f) => f.endsWith(".json"));
      if (doc) return `${p}/${doc}`;
    } catch { /* not this category */ }
  }
  throw new Error(id);
}

const lines: string[] = [];
for (const id of ids) {
  const asset = JSON.parse(readFileSync(locate(id), "utf8")).asset;
  lines.push(`\n== ${id} dynamics=${asset.structure.dynamics} supports=${asset.structure.supports.join(",") || "-"}`);
  const queries: { kind: "pose" | "clip"; pose?: string; clip?: string; timeS?: number }[] = [];
  for (const pose of asset.poses.filter((p: { key?: boolean }) => !p.key)) queries.push({ kind: "pose", pose: pose.id });
  for (const clip of asset.clips) {
    const samples = 5;
    for (let i = 0; i < samples; i++) queries.push({ kind: "clip", clip: clip.id, timeS: (clip.durationS * i) / (samples - 1) });
  }
  for (const q of queries) {
    const query = q.kind === "pose" ? { kind: "pose" as const, pose: q.pose! } : { kind: "clip" as const, clip: q.clip!, timeS: q.timeS! };
    const label = q.kind === "pose" ? `pose ${q.pose}` : `clip ${q.clip}@${q.timeS!.toFixed(3)}`;
    try {
      const physical = evaluate(asset, query);
      const structure = solveStructure(physical);
      const weight = structure.totalMassKg * structure.gravityMS2;
      const tolerance = Math.max(1e-6, weight * 1e-6);
      const supported = structure.reactions.some((r) => r.active);
      const bad = !supported || Math.abs(structure.residual.fxN) > tolerance || Math.abs(structure.residual.fyN) > tolerance;
      if (bad) lines.push(`  ${label}: supported=${supported} res ${structure.residual.fxN.toExponential(2)} ${structure.residual.fyN.toExponential(2)} m ${structure.residual.momentNm.toExponential(2)} active=${structure.reactions.filter((r) => r.active).map((r) => r.anchor).join(",")}`);
      const field = q.kind === "pose" ? solveThermal(physical) : solveThermalAt(asset, query);
      const hot = Object.values(field.regions).filter((r) => r.surfaceTempC < -15 || r.surfaceTempC > 80);
      if (hot.length) lines.push(`  ${label} thermal ${hot.map((r) => `${r.region} ${r.surfaceTempC.toFixed(1)}`).join(", ")}`);
    } catch (error) {
      lines.push(`  ${label}: ${(error as Error).message}`);
    }
  }
}
const text = lines.join("\n");
writeFileSync(process.argv[2]!, text);
console.log(text);
