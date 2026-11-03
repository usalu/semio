import { load, save } from "../humans/lib.ts";
const doc = load("skateboarder"); const a = doc.asset;
for (const rep of Object.values<any>(a.representations)) {
  if (rep.status !== "authored") continue;
  const front: any[] = rep.geometry.front;
  for (const side of ["l", "r"]) {
    if (front.some((s) => s.id === `wheel-rear-${side}`)) continue;
    const src = front.find((s) => s.id === `wheel-front-${side}`);
    if (!src) continue;
    const copy = JSON.parse(JSON.stringify(src));
    copy.id = `wheel-rear-${side}`; copy.bone = "wheel-r"; copy.z = (copy.z ?? 0.3) - 0.01;
    front.push(copy);
  }
}
save("skateboarder", doc);
