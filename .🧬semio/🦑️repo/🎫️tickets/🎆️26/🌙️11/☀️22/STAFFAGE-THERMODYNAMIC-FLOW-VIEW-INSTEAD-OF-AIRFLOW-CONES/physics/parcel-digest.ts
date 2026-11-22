/** 🔏️ Digest of the parcels of assets without a volume (fixtures and catalog assets with `volumes` removed) at fixed times: run before and after a change of the parcel integrator to prove the path without obstacles is bit-identical. Usage: bun parcel-digest.ts */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const S = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const transport = await import(join(S, "🔨️modules/💨️transport/🟦️.ts"));
const sources = ["🧫️fixtures/👤️sample-figure", "🧫️fixtures/🔥️sample-heater", "🗂️catalog/👤️humans/standing-adult", "🗂️catalog/🔌️appliances/electric-fan-heater", "🗂️catalog/🔌️appliances/electric-kettle", "🗂️catalog/🌳️vegetation/english-oak"];
for (const source of sources) {
  let asset;
  try {
    asset = JSON.parse(readFileSync(join(S, source, "🔣️.json"), "utf8")).asset;
  } catch {
    console.log(source, "missing");
    continue;
  }
  asset = { ...asset, volumes: {} };
  const hash = createHash("sha256");
  let count = 0;
  for (const env of [asset.thermal.environment, { ...asset.thermal.environment, windMS: 2, windDirectionDeg: 30 }])
    for (const timeS of [1.25, 3.3337, 6]) {
      const field = transport.solveTransport(asset, { kind: "pose", pose: asset.presentation.defaultPose, timeS }, env, { maxAgeS: 6, flow: true });
      count += field.parcels.length;
      hash.update(JSON.stringify([field.parcels, field.pathlines, field.flow.streams, field.flow.puffs]));
    }
  console.log(source, count, hash.digest("hex"));
}
