/** 👁️ Counts, for a catalog asset in a preset, the stream points each planar eye sees, veils and loses behind the body, and lists the streams that cross the body in plan. Usage: bun occlusion-probe.ts <category/slug> [preset] */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const S = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const transport = await import(join(S, "🔨️modules/💨️transport/🟦️.ts"));
const presets = JSON.parse(readFileSync(join(S, "🗂️catalog/🌤️environments/🔣️.json"), "utf8")).presets;
const asset = JSON.parse(readFileSync(join(S, "🗂️catalog", process.argv[2]!, "🔣️.json"), "utf8")).asset;
const environment = { ...asset.thermal.environment, ...(presets.find((p: { id: string }) => p.id === (process.argv[3] ?? "spring-mild-sun"))?.environment ?? {}) };
const field = await transport.developTransport(asset, { kind: "pose", pose: asset.presentation.defaultPose, timeS: 8 }, environment, { maxAgeS: 8, flow: true, pathlines: false });
const flow = field.flow;
console.log("obstacles", flow.obstacles.length, "streams", flow.streams.length, flow.streams.map((s: { role: string; points: unknown[] }) => `${s.role[0]}${s.points.length}`).join(" "));
for (const [name, toward] of [["elevation", { x: 0, y: 0, z: -1 }], ["front", { x: 1, y: 0, z: 0 }], ["plan", { x: 0, y: 1, z: 0 }]] as const) {
  const marks = transport.flowMarks(flow, flow.timeS, { ...transport.flowDetailOf(50), eye: { toward } });
  const all = marks.ribbons.flatMap((r: { visible: number[] }) => r.visible);
  console.log(name, "points", all.length, "hidden", all.filter((v: number) => v === 0).length, "veiled", all.filter((v: number) => v > 0 && v < 1).length);
}
const inside = flow.streams.flatMap((s: { points: { x: number; y: number; z: number }[] }) => s.points).filter((p: { x: number; y: number; z: number }) => flow.obstacles.some((o: unknown) => transport.obstacleLevel(o, p).s < 0.999)).length;
console.log("stream points inside an obstacle", inside, "parcels inside a wall", field.parcels.filter((p: { at: { x: number; y: number; z: number } }) => flow.obstacles.some((o: { blockage: number }) => o.blockage === 1 && transport.obstacleLevel(o, p.at).s < 0.999)).length, "of", field.parcels.length);
flow.streams.forEach((s: { role: string; seed: { x: number; y: number; z: number }; points: { x: number; y: number; z: number }[] }, k: number) => s.points.forEach((p, i) => { for (const o of flow.obstacles) { const l = transport.obstacleLevel(o, p).s; if (l < 0.999) console.log(`  stream ${k} ${s.role} point ${i}/${s.points.length} seed=${p === s.seed} in ${o.id} s=${l.toFixed(4)} at ${p.x.toFixed(3)},${p.y.toFixed(3)},${p.z.toFixed(3)}`); } }));
