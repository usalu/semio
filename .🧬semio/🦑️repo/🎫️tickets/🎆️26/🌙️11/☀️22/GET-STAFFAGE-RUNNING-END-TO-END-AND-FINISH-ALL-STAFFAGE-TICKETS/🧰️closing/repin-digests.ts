/** 📌️ Regenerates the digests of `🧫️fixtures/🧊️model-digests` from the live engine and prints which entries differ from the pinned ones: `bun repin-digests.ts <out.json>`.
 *
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-digests/🟦️.ts — the pinned case
 */
import { readFileSync, writeFileSync } from "node:fs";
import { digestsOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️digests/🟦️.ts";
import { scaledFigure } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️volumes/🟦️.ts";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const pinned = JSON.parse(readFileSync(`${S}/🧫️fixtures/🧊️model-digests/🔣️.json`, "utf8"));
const live = await digestsOf(scaledFigure());
const differing = (kind: "glb" | "fbx") => Object.keys(live[kind]).filter((name) => JSON.stringify(live[kind][name]) !== JSON.stringify(pinned[kind][name]));
console.log("glb differing", differing("glb"), "fbx differing", differing("fbx"));
writeFileSync(process.argv[2]!, JSON.stringify(live));
