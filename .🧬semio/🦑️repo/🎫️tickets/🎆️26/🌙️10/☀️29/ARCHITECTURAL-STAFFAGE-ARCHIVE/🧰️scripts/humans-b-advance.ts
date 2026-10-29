import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { sampleClip } = await import(`${S}/🔨️modules/🎞️animation/🟦️.ts`);
const asset: any = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const c of asset.clips) if (c.rootMotion === "plant" && c.loop) console.log(c.id, (sampleClip(asset, c, c.durationS - 1e-9).root.x - sampleClip(asset, c, 0).root.x).toFixed(8));
