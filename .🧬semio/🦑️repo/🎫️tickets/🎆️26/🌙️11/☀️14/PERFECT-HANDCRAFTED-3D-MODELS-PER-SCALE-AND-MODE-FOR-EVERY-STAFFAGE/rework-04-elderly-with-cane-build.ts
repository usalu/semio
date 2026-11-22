/** 🏗️ Builds the candidate document of `humans/elderly-with-cane`: the reworked rig (`rig.json`) with the hand-stated volumes of every scale; `--only 50` keeps the other scales as they are while one scale is on the bench.
 *
 * Usage: `bun rework-04-elderly-with-cane-build.ts [--only N]` writes `🗑️generated/rework-04/elderly-with-cane/🔣️.json`.
 * @see ./rework-04-elderly-with-cane-s50.ts */
import { candidatePath, writeJsonFile } from "./rework-04-kit.ts";
import type { Json } from "./volumes-lib.ts";
import { ASSET, SLUG } from "./rework-04-elderly-with-cane-s50.ts";

const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : undefined;
const lists = (await import("./rework-04-elderly-with-cane-lists.ts")) as { volumes: () => Record<string, Json> };
const all = lists.volumes();
const asset = JSON.parse(JSON.stringify(ASSET)) as Json;
for (const scale of Object.keys(all)) if (only === undefined || only === scale) asset.volumes[scale] = all[scale];
writeJsonFile(candidatePath(SLUG), { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset });
for (const scale of Object.keys(asset.volumes)) console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${asset.volumes[scale][l]?.length}`).join(", "));
