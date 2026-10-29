import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const asset: any = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const t of process.argv.slice(4).map(Number)) {
  const st = evaluate(asset, { kind: "clip", clip: process.argv[3], timeS: t });
  const a = Object.entries(st.kinematics.segmentAccelerations).map(([k, v]: any) => [k, Math.hypot(v.x, v.y)]).sort((p: any, q: any) => q[1] - p[1]).slice(0, 4);
  console.log(t, a.map(([k, v]: any) => `${k}:${v.toFixed(0)}`).join(" "));
}
