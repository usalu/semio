import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const asset: any = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const f = (n: number) => n.toFixed(1).padStart(8);
const show = (label: string, q: any) => {
  const st = evaluate(asset, q); const r = solveStructure(st);
  const reac = r.reactions.filter((x: any) => x.active).map((x: any) => `${x.anchor}:${x.fxN.toFixed(0)}/${x.fyN.toFixed(0)}`).join(" ");
  const j = Object.values(r.joints).filter((x: any) => ["torso", "upper-arm-r", "forearm-r"].includes(x.bone)).map((x: any) => `${x.bone}:M=${x.momentNm.toFixed(0)}`).join(" ");
  console.log(label.padEnd(26), "res", f(r.residual.fxN), f(r.residual.fyN), f(r.residual.momentNm), "|", j, "|", reac, "| ymin", st.bounds.minY.toFixed(3), "H", (st.bounds.maxY-st.bounds.minY).toFixed(2), "W", (st.bounds.maxX-st.bounds.minX).toFixed(2));
};
for (const p of asset.poses) show("pose " + p.id, { kind: "pose", pose: p.id });
for (const c of asset.clips) for (let i = 0; i <= 8; i++) show(`${c.id}@${(c.durationS * i / 8).toFixed(2)}`, { kind: "clip", clip: c.id, timeS: c.durationS * i / 8 });
