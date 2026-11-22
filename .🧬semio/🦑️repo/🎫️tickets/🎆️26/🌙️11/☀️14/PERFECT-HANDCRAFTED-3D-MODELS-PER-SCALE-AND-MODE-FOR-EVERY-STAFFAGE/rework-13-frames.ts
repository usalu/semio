/** 🦴️ Writes the rest frame (origin and the axes u, v, w in world x ahead, y up, z left) of every bone of a catalog asset to `🗑️generated/rework-13/<slug>.frames.json`.
 *
 * Usage: `bun rework-13-frames.ts <category>/<slug> [--pose ID]`; with `--pose` the frames of that pose go to `<slug>.frames.<pose>.json` (for plants whose default pose spreads or twists bones, so that numbers stated in the world as seen land in the rest frame of the bone).
 * @see ./r3-lib.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { load, rigAt } from "./r3-lib.ts";
import { GENERATED } from "./volumes-lib.ts";

const { entry, asset } = load(process.argv[2], "rework-13-frames.ts <asset>");
const out: Record<string, unknown> = {};
const at = process.argv.indexOf("--pose");
const pose = at < 0 ? undefined : process.argv[at + 1];
if (pose === undefined) for (const [id, solved] of Object.entries(restFrames(asset.rig as never))) out[id] = (solved as { frame: unknown }).frame;
else Object.assign(out, rigAt(asset, 50, { kind: "pose", pose }).frames);
const dir = join(GENERATED, "rework-13");
mkdirSync(dir, { recursive: true });
const path = join(dir, pose === undefined ? `${entry.slug}.frames.json` : `${entry.slug}.frames.${pose}.json`);
writeFileSync(path, JSON.stringify(out));
process.stdout.write(`${path}\n`);
