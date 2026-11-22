/** 🏹️ Probe: the directions of the stream arrows of an asset's flow, to find a field with a stream along a viewing axis. `bun arrow-probe.ts <asset> <timeS> [windMS]` */
import { loadCatalogFromFiles } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { flowDetailOf, flowMarks, solveTransport } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/💨️transport/🟦️.ts";

const [id, time, wind] = process.argv.slice(2);
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT, undefined, new Set([id!]));
const asset = catalog.loaded.find((entry) => entry.asset.id === id)!.asset;
const environment = wind === undefined ? asset.thermal.environment : { ...asset.thermal.environment, windMS: Number(wind) };
const field = solveTransport(asset, { kind: "pose", pose: asset.presentation.defaultPose, timeS: Number(time) }, environment, { maxAgeS: 2, flow: true });
console.log(JSON.stringify(environment), field.emitters.map((e) => [e.emitter, e.active, e.direction]));
const marks = flowMarks(field.flow!, field.flow!.timeS, flowDetailOf(50));
console.log(marks.ribbons.length, marks.trails.length, marks.ribbons.flatMap((r) => r.arrows.map((a) => [r.role, +(Math.abs(a.direction.x) + Math.abs(a.direction.z)).toFixed(2), +a.direction.y.toFixed(2)].join(" "))).join(" | "));
