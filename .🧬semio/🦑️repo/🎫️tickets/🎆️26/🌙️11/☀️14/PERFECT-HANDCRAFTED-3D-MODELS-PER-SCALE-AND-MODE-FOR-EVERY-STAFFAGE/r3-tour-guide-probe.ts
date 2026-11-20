/** 🔎 Prints the joints and tips of `humans/tour-guide` in a pose (world: x ahead, y up, z left), from the catalog or from the corrected roots.
 *
 * Usage: `bun r3-tour-guide-probe.ts [pose] [--roots]` */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { rigAt } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const { asset: cat } = entryOf("humans/tour-guide");
const roots = join(import.meta.dir, "🗑️generated", "r3-tour-guide", "roots.json");
const asset: Json = process.argv.includes("--roots") && existsSync(roots) ? { ...cat, ...JSON.parse(readFileSync(roots, "utf8")) } : cat;
const pose = process.argv[2] && !process.argv[2].startsWith("--") ? process.argv[2] : asset.presentation.defaultPose;
const f = rigAt(asset, 50, { kind: "pose", pose }).frames;
const len: Record<string, number> = Object.fromEntries(asset.rig.bones.map((b: Json) => [b.id, b.lengthM]));
for (const [k, v] of Object.entries(f)) console.log(k.padEnd(14), "o", v.o.map((x) => x.toFixed(3)).join(" "), " tip", v.o.map((x, i) => (x + v.u[i]! * (len[k] ?? 0)).toFixed(3)).join(" "), " u", v.u.map((x) => x.toFixed(2)).join(" "), " v", v.v.map((x) => x.toFixed(2)).join(" "));
