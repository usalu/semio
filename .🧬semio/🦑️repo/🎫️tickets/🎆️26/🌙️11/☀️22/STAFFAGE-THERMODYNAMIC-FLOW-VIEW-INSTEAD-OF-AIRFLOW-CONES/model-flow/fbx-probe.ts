/** 🎬️ Prints the triangles of the thermal FBX of a clip of the sample figure with and without flow geometry, and what the kernel's FBX reader finds in the file. Run with bun from the repo root. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { objectCounts, readFbx } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/📖️reader/🟦️.ts";
import { writeFbxDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { modelOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";

const S = join(import.meta.dir, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const asset = JSON.parse(readFileSync(join(S, "🧫️fixtures/👤️sample-figure/🔣️.json"), "utf8")).asset;
for (const clip of ["walk", "idle-breathing"]) {
  const document = await modelOf(asset, { scale: 50, view: "thermal", at: { kind: "clip", clip, timeS: 0 }, poses: [], clips: [clip] });
  console.log(clip, "flow solids", document.solids.filter((s) => s.tags?.includes("flow")).length, "flow nodes", document.nodes.filter((n) => n.tags?.includes("flow")).length, "channels on flow", document.animations[0]!.channels.filter((c) => "node" in c.target && c.target.node.startsWith("flow")).length);
  const withFlow = await writeFbxDetailed(asset, { clip, scale: 50, view: "thermal" });
  const without = await writeFbxDetailed(asset, { clip, scale: 50, view: "thermal", flowGeometry: false });
  console.log(clip, "fbx triangles", withFlow.triangles, "without flow", without.triangles, "bytes", withFlow.bytes.length, without.bytes.length, JSON.stringify(objectCounts(readFbx(withFlow.bytes))), withFlow.notices.map((n) => n.code).join(","));
}
