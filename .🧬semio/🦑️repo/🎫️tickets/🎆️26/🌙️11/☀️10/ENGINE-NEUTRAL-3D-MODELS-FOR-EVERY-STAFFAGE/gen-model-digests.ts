/** 🧬️ Writes `🧫️fixtures/🧊️model-digests/🔣️.json`: GLB and FBX structure digests of the sample figure at 1:50. */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { writeGlb } = await import(`${S}/🔨️modules/📤️export/🧊️glb/🟦️.ts`);
const { writeFbxDetailed } = await import(`${S}/🔨️modules/📤️export/🎬️fbx/🟦️.ts`);
const { glbDigest, fbxDigest } = await import(`${S}/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`);
const { objectCounts, readFbx } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/📖️reader/🟦️.ts");
const asset = JSON.parse(readFileSync(`${S}/🧫️fixtures/👤️sample-figure/🔣️.json`, "utf8")).asset;
const sha = (b: Uint8Array) => createHash("sha256").update(b).digest("hex");
const out: any = { scale: 50, glb: {}, fbx: {} };
for (const p of asset.poses.filter((x: any) => x.key !== true)) out.glb[p.id] = glbDigest(await writeGlb(asset, { pose: p.id, scale: 50 }), sha);
for (const c of asset.clips) { const r = await writeFbxDetailed(asset, { clip: c.id, scale: 50 }); out.fbx[c.id] = fbxDigest(r.bytes, r.notices.map((n: any) => `${n.code}:${n.subject}`), sha, objectCounts(readFbx(r.bytes))); }
writeFileSync(`${S}/🧫️fixtures/🧊️model-digests/🔣️.json`, `${JSON.stringify(out, null, 1)}\n`);
console.log(Object.keys(out.glb), Object.keys(out.fbx));
