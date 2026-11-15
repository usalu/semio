import { Fauna } from "./fauna-lib.ts";
const sc = Number(process.argv[2]);
const S = new Fauna("animals/herring-gull", sc);
const ids = sc === 200 ? ["arm", "hand"] : ["near-arm", "near-hand"];
for (const x of [0.03, -0.02, -0.07, -0.12, -0.15, -0.18, -0.2, -0.22, -0.25, -0.3]) console.log(x, JSON.stringify(S.ys(ids, x)));
for (const b of (S.c.vol.body as any[]).filter((b) => b.id.startsWith("near-wing"))) { const f: any = S.frame(b.bone); console.log(b.id, b.bone, (b.sections as any[]).map((q: any) => (f.o[0] + q.at[0] * f.u[0] + q.at[1] * f.v[0]).toFixed(3)).join(" ")); }
