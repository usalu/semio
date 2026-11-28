/** 🔬️ Encodes the default pose of assets under several split thresholds without rebuilding the model: thr.ts <asset> <bytes>… */
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { encodeGlb } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { PHASES, jobOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { resolvedOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🗿️model/🟦️.ts";
const [id, ...thresholds] = process.argv.slice(2);
const asset = (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === id)!)).asset;
const pose = asset.presentation.defaultPose;
const job = jobOf({}, [{ label: PHASES.model, weight: 6 }, { label: PHASES.tessellate, weight: 3 }, { label: PHASES.edges, weight: 2 }, { label: PHASES.encode, weight: 1 }]);
const procedural = await modelOfAsset(asset, { at: { kind: "pose", pose }, poses: [pose], clips: [] } as any, job.operation(0));
const edged = await resolvedOf(asset, procedural, job, 1, { shareVertices: true });
for (const t of thresholds) { (globalThis as any).DEBUG_SPLIT = Number(t); const bytes = encodeGlb(edged, { pose, animations: false, variants: true, compact: true }, job.operation(3)); console.log(`THR ${id} ${t} ${bytes.length} ${(bytes.length / 1048576).toFixed(3)}`); }
