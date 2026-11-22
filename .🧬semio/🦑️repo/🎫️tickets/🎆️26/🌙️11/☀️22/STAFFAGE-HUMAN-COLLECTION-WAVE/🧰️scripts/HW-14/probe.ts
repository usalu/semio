/** 🔎️ Prints world frames (origin, tip, axes) of the given bones of an asset document in a pose: bun probe.ts <file|slug> <pose|rest> bone… */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { CATALOG, frames } from "./lib.ts";
const [target, pose, ...bones] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(target!.endsWith(".json") ? target! : join(CATALOG, target!, "🔣️.json"), "utf8")).asset;
const f = frames(asset, pose === "rest" ? null : pose!);
const r = (v: { x: number; y: number; z: number }) => `(${v.x.toFixed(4)}, ${v.y.toFixed(4)}, ${v.z.toFixed(4)})`;
for (const b of bones.length ? bones : Object.keys(f)) { const x = f[b]!; console.log(`${b.padEnd(20)} o${r(x.o)} tip${r(x.tip)} u${r(x.u)} v${r(x.v)} w${r(x.w)}`); }
