import { load, save } from "../humans/lib.ts";
const doc = load("skateboarder"); const a = doc.asset;
for (const rep of Object.values<any>(a.representations)) {
  if (rep.status !== "authored") continue;
  const fr = rep.geometry.front.find((s: any) => s.id === "upper-arm-l" && s.solid);
  if (!fr) continue;
  for (const v of ["elevation", "plan"]) for (const s of rep.geometry[v]) {
    if (s.id !== "upper-arm-l" || !s.solid) continue;
    s.kind = "poly"; s.points = JSON.parse(JSON.stringify(fr.points)); s.closed = true;
  }
}
save("skateboarder", doc);
