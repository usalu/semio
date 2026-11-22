/** 🔏️ Writes into the `🌊️flow-obstacles` fixture the digest of the parcels of its figure without a volume (the path of the parcel integrator that existed before obstacles; `parcel-digest.ts` proved that path unchanged). Usage: bun unobstructed-digest.ts */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const S = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const transport = await import(join(S, "🔨️modules/💨️transport/🟦️.ts"));
const path = join(S, "🧫️fixtures/🌊️flow-obstacles/🔣️.json");
const fixture = JSON.parse(readFileSync(path, "utf8"));
const asset = { ...JSON.parse(readFileSync(join(S, "🧫️fixtures", fixture.figure.asset, "🔣️.json"), "utf8")).asset, volumes: {} };
const bare = transport.solveTransport(asset, { kind: "pose", pose: fixture.figure.pose, timeS: fixture.figure.timeS }, { ...asset.thermal.environment, windMS: fixture.figure.windMS, windDirectionDeg: fixture.figure.windDirectionDeg }, { maxAgeS: fixture.figure.maxAgeS, pathlines: false });
fixture.expected.figure.unobstructedParcelsDigest = createHash("sha256").update(JSON.stringify(bare.parcels)).digest("hex");
writeFileSync(path, JSON.stringify(fixture, null, 1) + "\n");
console.log(bare.parcels.length, fixture.expected.figure.unobstructedParcelsDigest);
