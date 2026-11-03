import { load, save } from "../humans/lib.ts";
const doc = load("skateboarder"); const a = doc.asset;
for (const rep of Object.values<any>(a.representations)) {
  if (rep.status !== "authored") continue;
  for (const s of rep.geometry.plan) if (/^hand-[lr]$/.test(s.id) && s.kind === "poly") s.points = s.points.map((p: number[]) => [Math.min(p[0], 0.169), p[1]]);
}
save("skateboarder", doc);
