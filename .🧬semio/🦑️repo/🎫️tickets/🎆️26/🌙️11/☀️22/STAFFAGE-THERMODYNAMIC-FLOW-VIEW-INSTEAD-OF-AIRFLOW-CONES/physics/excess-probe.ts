/** 🌡️ Compares, per asset and pose, the largest temperature excess a parcel carries with the largest one the flow field shows on its streams (twice the parcel value is the Gaussian centreline of one stream; more means streams were summed). Usage: bun excess-probe.ts <category/slug[:pose]>... */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const S = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const transport = await import(join(S, "🔨️modules/💨️transport/🟦️.ts"));
for (const target of process.argv.slice(2)) {
  const [path, pose] = target.split(":") as [string, string | undefined];
  const asset = JSON.parse(readFileSync(join(S, "🗂️catalog", path, "🔣️.json"), "utf8")).asset;
  const field = transport.solveTransport(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose, timeS: 8 }, asset.thermal.environment, { maxAgeS: 8, flow: true });
  const parcels = Math.max(0, ...field.parcels.map((p: { excessK: number }) => Math.abs(p.excessK)));
  const streams = Math.max(0, ...field.flow.streams.flatMap((s: { excessK: number[] }) => s.excessK.map(Math.abs)));
  console.log(asset.id, pose ?? asset.presentation.defaultPose, "parcels", parcels.toFixed(2), "K, streams", streams.toFixed(2), "K, ratio", (streams / Math.max(parcels, 1e-9)).toFixed(2), "active", field.emitters.filter((e: { active: boolean }) => e.active).length);
}
