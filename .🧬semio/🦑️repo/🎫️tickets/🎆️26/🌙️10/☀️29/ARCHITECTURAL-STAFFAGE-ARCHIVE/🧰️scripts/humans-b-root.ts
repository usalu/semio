import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { sampleClip } = await import(`${S}/🔨️modules/🎞️animation/🟦️.ts`);
const asset: any = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const c = asset.clips.find((x: any) => x.id === process.argv[3]);
const ts = process.argv.slice(4).map(Number);
for (const t of ts) { const s = sampleClip(asset, c, t); console.log(t.toFixed(4), s.root.x.toFixed(5), s.root.y.toFixed(5), s.contacts.join(",")); }
