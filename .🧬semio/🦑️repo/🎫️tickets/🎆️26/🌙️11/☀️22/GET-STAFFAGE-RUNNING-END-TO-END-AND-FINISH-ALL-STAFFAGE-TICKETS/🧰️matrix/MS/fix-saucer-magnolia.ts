/** 🌸️ The multiview repair of `saucer-magnolia` (idempotent, the last step of its chain, after the ops of `🧰️silhouette/SV-2/repair/saucer-magnolia.json`): the transpiration outlet `tp-shade-3` of the shaded core lay 0.22 m beside the centre of its own leaf patch `th-tp-shade-3`, at the rim of the core where the front elevation draws a sunlit mass of another bone in front of it (100 mm off the drawn core at rest, 144 mm in the gale against a tolerance of 130 mm). It moves 0.21 m onto its patch (0.05 m from the centre), where the core is drawn in all three views in every pose (`anchor-search.ts`: worst distance 0 mm).
 *  bun fix-saucer-magnolia.ts [--in file] [--out file] */
import { load, save } from "./lib.ts";

const loaded = await load("saucer-magnolia");
const anchor = loaded.asset.anchors.find((candidate: any) => candidate.id === "tp-shade-3");
if (anchor === undefined || anchor.bone !== "fol-core") throw new Error("saucer-magnolia: anchor tp-shade-3 on fol-core not found");
Object.assign(anchor, { x: 0.1914, y: 2.2738, z: 0.3912 });
console.log(`saucer-magnolia: tp-shade-3 at (${anchor.x}, ${anchor.y}, ${anchor.z}), file ${(await save(loaded)) ? "written" : "unchanged"}`);
