/** 🔬️ Authoring aid: lists one authored volume of an asset compactly (per solid its kind, binding, flags, its box in the bone frame and its box in the world of the default pose, in millimetres), with the bones, regions, anchors and supports a coarser volume has to name. Reads the catalog, writes nothing.
 *
 * usage: dump.ts <asset id | asset document> [scale=200] [--full id,id,…|all] [--lists body,thermal,structure] */
import { readFileSync } from "node:fs";
import { assetFile, mod } from "./lib.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => { const at = args.indexOf(name); return at < 0 ? undefined : args.splice(at, 2)[1]; };
const full = flag("--full")?.split(",") ?? [];
const lists = flag("--lists")?.split(",") ?? ["body", "thermal", "structure"];
const [target, scale = "200"] = args;
const asset = JSON.parse(readFileSync(target!.endsWith(".json") ? target! : assetFile(target!), "utf8")).asset;
const { evaluate } = await mod("🔨️modules/🧮️evaluation/🟦️.ts");
const { worldSolids, extentOf, copyMesh, repeatMaps } = await mod("🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const { qualityOf } = await mod("🔨️modules/🧊️volume/⏱️job/🟦️.ts");
const mm = (v: number): string => (v * 1000).toFixed(0);
const span = (e: { min: number[]; max: number[] }): string => [0, 1, 2].map((k) => `${mm(e.min[k]!)}..${mm(e.max[k]!)}`).join(" ");
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
console.log(`# ${asset.id} dims w(x) ${asset.dimensions.widthM} h(y) ${asset.dimensions.heightM} d(z) ${asset.dimensions.depthM} pose ${asset.presentation.defaultPose} poses ${asset.poses.map((p: any) => p.id).join(",")}`);
console.log(`materials ${asset.materials.map((m: any) => m.id).join(" ")}`);
for (const bone of asset.rig.bones) {
  const f = state.bones[bone.id].frame;
  console.log(`bone ${bone.id} <- ${bone.parent ?? "-"} ${bone.joint.type} len ${mm(bone.lengthM)} rest(${bone.x},${bone.y},${bone.z ?? 0},${bone.angleDeg}) frame ${JSON.stringify(f, (_, v) => (typeof v === "number" ? Math.round(v * 1000) / 1000 : v))}`);
}
console.log(`regions ${asset.thermal.regions.map((r: any) => `${r.id}:${r.kind}${r.hidden ? ":hidden" : ""}[${r.layers.map((l: any) => l.id).join(",")}]`).join(" ")}`);
console.log(`emitters ${asset.thermal.emitters.map((e: any) => `${e.id}@${e.anchor ?? ""}`).join(" ")} minAir ${asset.thermal.minimumAirPassageM ?? "-"}`);
console.log(`anchors ${asset.anchors.map((a: any) => `${a.id}:${a.kind}@${a.bone}(${a.x},${a.y},${a.z ?? 0})`).join(" ")}`);
console.log(`supports ${(asset.structure.supports ?? []).join(" ")}`);
const volume = asset.volumes[scale];
console.log(`decisions ${JSON.stringify((volume.decisions ?? []).map((d: any) => d.en))}`);
const quality = qualityOf(asset);
const body = worldSolids(asset, state.bones, Number(scale), "body");
console.log(`BODY WORLD 1:${scale} ${span(extentOf(body))} size ${[0, 1, 2].map((k) => mm(extentOf(body).max[k] - extentOf(body).min[k])).join(" x ")}`);
for (const list of lists) {
  console.log(`== ${list} (${volume[list].length})`);
  const worlds = worldSolids(asset, state.bones, Number(scale), list, list === "body" ? undefined : body);
  const refs: string[] = [];
  volume[list].forEach((solid: any, at: number) => {
    if (solid.of !== undefined) return void refs.push(`${solid.id}>${solid.of}${["region", "layer", "member", "members", "part", "anchor", "sectionFaces"].filter((key) => solid[key] !== undefined).map((key) => ` ${key[0]}=${JSON.stringify(solid[key]).replaceAll('"', "")}`).join("")}`);
    const source = solid;
    const mesh = copyMesh(source, quality);
    const local = { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
    for (const { a, b } of source.repeat === undefined ? [{ a: [1, 0, 0, 0, 1, 0, 0, 0, 1], b: [0, 0, 0] }] : repeatMaps(source.repeat)) for (let i = 0; i < mesh.positions.length; i += 3) for (let k = 0; k < 3; k++) {
      const v = a[k * 3]! * mesh.positions[i]! + a[k * 3 + 1]! * mesh.positions[i + 1]! + a[k * 3 + 2]! * mesh.positions[i + 2]! + b[k]!;
      local.min[k] = Math.min(local.min[k]!, v);
      local.max[k] = Math.max(local.max[k]!, v);
    }
    const binding = ["region", "layer", "member", "members", "part", "anchor", "slender", "sheet", "envelope", "texture", "tags", "sectionFaces", "weights"].filter((key) => solid[key] !== undefined).map((key) => `${key}=${JSON.stringify(solid[key])}`).join(" ");
    console.log(`${solid.id} | ${source.kind}${source.repeat ? ` x${source.repeat.count}` : ""} ${source.bone} ${source.material} ${source.role} ${source.tone} | uvw ${span(local)} | xyz ${span(extentOf([worlds[at]]))} | ${binding}`);
    if (full.includes(solid.id) || full.includes("all") || (list === "body" && ["revolve", "loft", "extrude", "polyhedron"].includes(solid.kind))) console.log(`   ${JSON.stringify({ ...solid, id: undefined, bone: undefined, material: undefined, role: undefined, tone: undefined, kind: undefined }).slice(0, 900)}`);
  });
  if (refs.length > 0) console.log(`refs: ${refs.join(" ; ")}`);
}
