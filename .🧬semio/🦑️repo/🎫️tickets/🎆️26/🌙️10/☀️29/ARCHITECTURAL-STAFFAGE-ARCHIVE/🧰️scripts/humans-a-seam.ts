// [DEBUG] root x around the loop seam
import { readFileSync } from "node:fs";
import { sampleClip } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
const [file, id, clipId, t0] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).assets.find((a: any) => a.id === id);
const clip = asset.clips.find((c: any) => c.id === clipId);
const c = Number(t0 ?? 0);
for (let k = -6; k <= 6; k++) { const t = c + k / 240; const s = sampleClip(asset, clip, t); console.log(t.toFixed(4), s.root.x.toFixed(5), s.root.y.toFixed(5), s.contacts.join(",")); }
