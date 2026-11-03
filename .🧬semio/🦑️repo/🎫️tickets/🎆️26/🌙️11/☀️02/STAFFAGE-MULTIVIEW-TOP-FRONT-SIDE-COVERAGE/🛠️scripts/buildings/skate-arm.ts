import { load, save } from "../humans/lib.ts";
import { flattenPath, simplifyClosed } from "../humans/simplify.ts";
const eps = Number(process.argv[2] ?? 0.001);
const doc = load("skateboarder"); const a = doc.asset;
for (const rep of Object.values<any>(a.representations)) {
  if (rep.status !== "authored") continue;
  for (const v of ["elevation", "plan"]) for (const s of rep.geometry[v]) {
    if (s.id !== "upper-arm-l" || s.kind !== "path" || !s.solid) continue;
    const { rings } = flattenPath(s.d);
    const pts = simplifyClosed(rings[0]!, eps);
    s.kind = "poly"; s.points = pts; s.closed = true; delete s.d;
  }
}
save("skateboarder", doc);
