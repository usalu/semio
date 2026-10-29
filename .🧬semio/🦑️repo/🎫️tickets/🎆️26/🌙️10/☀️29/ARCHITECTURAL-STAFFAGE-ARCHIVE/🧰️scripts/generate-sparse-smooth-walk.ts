import { readFileSync, writeFileSync } from "node:fs";
import { sampleClip } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";

const [source, out] = [process.argv[2]!, process.argv[3]!];
const asset = JSON.parse(readFileSync(source, "utf8"));
const dense = asset.clips.find((c: any) => c.id === "walk");
const COUNT = Number(process.argv[4] ?? 4);
const step = dense.keys.length - 1;
const sparse = Array.from({ length: COUNT }, (_, i) => dense.keys[(i * step) / COUNT]).map((k: any) => ({ ...k, ease: "smooth" }));
const K = Number(process.argv[5] ?? 1);
const scale = (items: any[], fields: string[]) => items.map((item) => Object.fromEntries(Object.entries(item).map(([k, v]) => [k, fields.includes(k) ? (v as number) * K : v])));
const build = (advanceM: number) => ({
  ...asset,
  id: "sparse-smooth-walk",
  title: { en: "Sparse smooth walker", de: "Spärlicher glatter Fußgänger" },
  clips: asset.clips.map((clip: any) => (clip.id === "walk"
    ? { ...clip, advanceM, durationS: clip.durationS * K, contacts: scale(clip.contacts, ["fromS", "toS"]), phases: scale(clip.phases, ["fromS", "toS"]), events: scale(clip.events, ["t"]), tracks: clip.tracks.map((t: any) => ({ ...t, keys: scale(t.keys, ["t"]) })), description: { en: `Walking from ${COUNT} key poses sampled from the dense gait, joined by C1 smooth interpolation.`, de: `Gehen aus ${COUNT} Schlüsselposen des dichten Gangs, verbunden durch glatte C1-Interpolation.` }, keys: scale(sparse, ["t"]) }
    : clip)),
});
let result: any = build(0);
const walk = result.clips.find((c: any) => c.id === "walk");
result = build(Number((sampleClip(result, walk, walk.durationS - 1e-12).root.x - sampleClip(result, walk, 0).root.x).toFixed(6)));
writeFileSync(out, JSON.stringify(result, null, 2) + "\n");
console.log(result.clips[0].advanceM);
