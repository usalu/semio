import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const T = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/skateboarder/🔣️.json`, "utf8")).asset;
const c = a.clips.find((x: any) => x.id === "push-cycle"); console.log("duration", c.durationS);
for (const f of T.CLIP_FRACTIONS) { const t = c.durationS * f; try { evaluate(a, { kind: "clip", clip: "push-cycle", timeS: t }, { kind: "scale", scale: { denominator: 50 } }); } catch (e: any) { console.log("FAIL at", t, String(e.stack).split(String.fromCharCode(10)).slice(0, 8).join(" | ").slice(0, 600)); } }
