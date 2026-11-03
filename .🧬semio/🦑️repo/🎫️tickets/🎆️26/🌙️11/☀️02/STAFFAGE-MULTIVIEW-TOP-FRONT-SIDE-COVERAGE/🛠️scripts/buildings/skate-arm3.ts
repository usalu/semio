import { load, save } from "../humans/lib.ts";
import { flattenPath, simplifyClosed } from "../humans/simplify.ts";
const eps = Number(process.argv[2] ?? 0.006);
const doc = load("skateboarder"); const a = doc.asset;
for (const rep of Object.values<any>(a.representations)) {
  if (rep.status !== "authored") continue;
  for (const v of ["elevation", "plan"]) for (const s of rep.geometry[v]) {
    if (!/^forearm-[lr]$/.test(s.id) || !s.solid) continue;
    let pts: number[][];
    if (s.kind === "path") pts = simplifyClosed(flattenPath(s.d).rings[0]!, eps);
    else if (s.kind === "poly") pts = simplifyClosed(s.points, eps);
    else continue;
    s.kind = "poly"; s.points = pts; s.closed = true; delete s.d;
  }
}
save("skateboarder", doc);
