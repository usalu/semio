import { readdirSync, writeFileSync } from "node:fs";
import { load } from "./lib.ts";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const out = process.argv[2]!;
const lines: string[] = [];
for (const slug of readdirSync(`${L}/🗂️catalog/👤️humans`).sort()) {
  const a = load(slug).asset;
  const score = (p: any) => Object.values<any>(p.bones).reduce((t, o) => t + Math.abs(o.abductionDeg ?? 0) + Math.abs(o.twistDeg ?? 0) + Math.abs(o.angleDeg ?? 0) * 0.01, 0);
  const extreme = [...a.poses].sort((x: any, y: any) => score(y) - score(x))[0].id;
  const clip = a.clips[0];
  const scales = Object.values<any>(a.representations).filter((r) => r.status === "authored").map((r) => r.denominator);
  for (const sc of [50, 200]) {
    if (!scales.includes(sc)) continue;
    lines.push(`${slug} ${sc} --pose ${a.presentation.defaultPose}`);
    if (extreme !== a.presentation.defaultPose) lines.push(`${slug} ${sc} --pose ${extreme}`);
    if (clip) lines.push(`${slug} ${sc} --clip ${clip.id} --time ${(clip.durationS * 0.37).toFixed(2)}`);
  }
}
writeFileSync(out, lines.join("\n") + "\n"); console.log(lines.length, "proofs planned");
