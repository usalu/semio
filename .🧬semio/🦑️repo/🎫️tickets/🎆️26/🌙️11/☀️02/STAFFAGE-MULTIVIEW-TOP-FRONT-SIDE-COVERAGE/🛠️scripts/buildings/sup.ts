import { load } from "./lib.ts";
for (const id of process.argv.slice(2)) {
  const a = load(id).asset;
  console.log("##", id, "dims", JSON.stringify(a.dimensions), "supports", JSON.stringify(a.structure.supports));
  for (const n of a.anchors) if (n.kind !== "load" && n.kind!=="emitter") console.log("  ", JSON.stringify({ ...n, title: n.title.en }));
  for (const s of a.structure.assumptions ?? []) console.log("   A:", s.en);
  for (const b of a.rig.bones) console.log("   bone", b.id, b.parent, b.x, b.y, b.angleDeg, b.lengthM, b.z ?? "", b.joint?.type);
}
