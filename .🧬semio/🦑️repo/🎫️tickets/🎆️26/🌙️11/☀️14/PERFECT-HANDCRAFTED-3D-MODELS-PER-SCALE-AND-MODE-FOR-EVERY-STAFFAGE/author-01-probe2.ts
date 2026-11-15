import { loadAsset, resolveAsset, memberBones, memberShapes, shapeBones } from "./volumes-lib.ts";
const a = loadAsset(resolveAsset(process.argv[2]!).path);
console.log(JSON.stringify(a.thermal.emitters[0]));
for (const s of [50, 100, 200, 500]) {
  if (a.representations[String(s)].status === "unavailable") continue;
  console.log(s, "members", memberBones(a, s).join(","), "| memberShapes", memberShapes(a, s).map((m: any) => m.id + ":" + JSON.stringify(m.section)).join(",").slice(0, 300));
  console.log(s, "shapeBones", [...shapeBones(a, s)].join(","));
}
