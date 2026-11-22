/** 🔎️ Prints bone frames and anchors of an asset file at a pose or clip instant: bun probe.ts <file> <pose|clip@t> [bone,...] */
import { framesOf, loadAsset } from "./lib.ts";

const [file, spec, only] = process.argv.slice(2);
const asset = await loadAsset(file!);
const out = framesOf(asset, spec!);
const keep = only ? new Set(only.split(",")) : null;
if (keep) out.bones = Object.fromEntries(Object.entries(out.bones).filter(([id]) => keep.has(id)));
console.log(JSON.stringify(out));
