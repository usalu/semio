// [scratch] prints the root y and the active contacts along a clip to find ground jumps
import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const [file, clipId, dt] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file, "utf8")).assets.find((a: any) => a.id === "dairy-cow");
const clip = asset.clips.find((c: any) => c.id === clipId);
let prev = NaN;
for (let t = 0; t <= clip.durationS + 1e-9; t += Number(dt)) {
  const s = evaluate(asset, { kind: "clip", clip: clipId, timeS: Math.min(t, clip.durationS - 1e-6) });
  const y = s.bones["pelvis"].origin.y;
  const flag = Math.abs(y - prev) > 0.03 ? " <-- JUMP" : "";
  console.log(`${t.toFixed(2)} y=${y.toFixed(3)} ${s.contacts.map((c: string) => c.replace("contact-", "")).join(",")}${flag}`);
  prev = y;
}
