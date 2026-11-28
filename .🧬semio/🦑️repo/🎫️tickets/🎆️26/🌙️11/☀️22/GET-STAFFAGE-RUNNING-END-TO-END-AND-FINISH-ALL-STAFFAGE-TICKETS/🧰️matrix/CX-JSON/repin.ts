/** 📌️ Re-pins the GLB and FBX digests of the sample figure with the live writers and prints what changed per entry. */
import { readFileSync, writeFileSync } from "node:fs";
import { writeGlb } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { writeFbxDetailed } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { fbxName } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🗿️model/🟦️.ts";
import { authoredScales } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { DIGEST_VIEWS, fbxDigestOf, glbDigestOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️digests/🟦️.ts";
import { scaledFigure } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️volumes/🟦️.ts";
const file = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧊️model-digests/🔣️.json";
const text = readFileSync(file, "utf8"); const json = JSON.parse(text); const asset = scaledFigure();
const diff = (a: any, b: any) => Object.keys({ ...a, ...b }).filter((k) => JSON.stringify(a?.[k]) !== JSON.stringify(b?.[k]));
for (const pose of asset.poses.filter((p: any) => p.key !== true)) { const next = glbDigestOf(await writeGlb(asset, { pose: pose.id })); console.log("[DEBUG] glb", pose.id, diff(json.glb[pose.id], next).join(","), json.glb[pose.id].byteLength, "->", (next as any).byteLength); json.glb[pose.id] = next; }
for (const clip of asset.clips) for (const scale of authoredScales(asset.volumes)) for (const view of DIGEST_VIEWS) { const name = fbxName(asset.id, clip.id, scale, view); const next = fbxDigestOf(await writeFbxDetailed(asset, { clip: clip.id, scale, view })); const d = diff(json.fbx[name], next); if (d.length) console.log("[DEBUG] fbx", name, d.join(","), json.fbx[name].byteLength, "->", (next as any).byteLength); json.fbx[name] = next; }
writeFileSync(file, JSON.stringify(json, null, 1) + (text.endsWith("\n") ? "\n" : ""));
