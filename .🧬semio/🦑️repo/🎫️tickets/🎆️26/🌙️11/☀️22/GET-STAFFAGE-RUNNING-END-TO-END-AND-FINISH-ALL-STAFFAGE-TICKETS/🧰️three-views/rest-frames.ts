/** 🧊️ Prints the rest frames (origin and the axes u, v, w in asset coordinates, root at the origin) of the named bones of one catalog asset as JSON: the reference in which front and plan outlines are authored, needed to restate such an outline in the local frame of its bone.
 *
 * Usage: `bun rest-frames.ts <id> <bone> [<bone> …]`
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts */
import { restFrames } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { catalog } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

if (import.meta.main) {
  const [id, ...bones] = process.argv.slice(2);
  const asset = catalog().find((candidate) => candidate.id === id)!;
  const frames = restFrames(asset.rig as never) as Record<string, unknown>;
  console.log(JSON.stringify(Object.fromEntries(bones.map((bone) => [bone, frames[bone]]))));
}
