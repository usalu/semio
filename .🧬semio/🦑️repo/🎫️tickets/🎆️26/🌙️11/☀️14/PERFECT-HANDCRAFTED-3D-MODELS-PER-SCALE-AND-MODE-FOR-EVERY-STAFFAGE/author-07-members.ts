import { entryOf, memberBones, memberShapes } from "./volumes-lib.ts";
for (const spec of process.argv.slice(2)) {
  const { asset } = entryOf(spec);
  for (const s of [50, 100, 200, 500]) {
    if (asset.representations?.[String(s)]?.status !== "authored") continue;
    console.log(spec, s, "members:", memberBones(asset, s).join(","), "| shapes:", memberShapes(asset, s).map((x: any) => `${x.id}@${x.bone}`).join(","));
  }
}
