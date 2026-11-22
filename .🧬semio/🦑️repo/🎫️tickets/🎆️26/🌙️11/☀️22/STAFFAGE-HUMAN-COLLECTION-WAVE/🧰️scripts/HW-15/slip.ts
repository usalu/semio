/** 🦶️ Slip of every planted contact of a clip: the world x travel of each anchor during its spans (optionally with the dog contacts left out of the plant): bun slip.ts <slug> <clip> [human-only] */
import { join } from "node:path";
import { S, CATALOG } from "./lib.ts";
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const [slug, clipId, mode] = process.argv.slice(2);
const a = (await Bun.file(join(CATALOG, slug === "labrador-dog" ? "🐕️animals" : "👤️humans", slug!, "🔣️.json")).json()).asset;
const clip = a.clips.find((c: any) => c.id === clipId);
const spans = [...clip.contacts];
if (mode === "human-only") clip.contacts = clip.contacts.filter((s: any) => !s.anchor.startsWith("dog-"));
const n = 240;
const xs: Record<string, number[]> = {};
for (let i = 0; i <= n; i++) {
  const t = (clip.durationS * i) / n * 0.9999;
  const st = evaluate(a, { kind: "clip", clip: clipId, timeS: t });
  spans.forEach((s: any, k: number) => { if (t >= s.fromS && t <= s.toS) { const key = `${k}:${s.anchor}`; (xs[key] ??= []).push(st.anchors[s.anchor].at.x); } });
}
for (const [k, v] of Object.entries(xs)) console.log(k.padEnd(24), "slip mm", ((Math.max(...v) - Math.min(...v)) * 1000).toFixed(1), "n", v.length);
console.log("root travel", (evaluate(a, { kind: "clip", clip: clipId, timeS: clip.durationS - 1e-9 }).root?.x ?? "?"));
