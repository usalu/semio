import { frames, loadAsset, resolveAsset } from "../volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const f = frames(asset);
const r = (v: number[]) => v.map((x) => +x.toFixed(4)).join(",");
for (const [b, fr] of Object.entries(f)) console.log(b, "o", r(fr.o), "u", r(fr.u), "v", r(fr.v), "w", r(fr.w));
