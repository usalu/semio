/** 🧭️ Per body scene of the GLB of one pose: the extent in the staffage axes (ahead, up, left) against `dimensions` as the bounds check of `🧊️model-catalog` reads it, and the meshes that reach farthest ahead and behind with their boxes, so the solid that breaks the authored width is named; the extent leaves out the contact surfaces of role `ground` as the oracle does and states the whole extent beside it. ahead.ts <asset id | file.json> [pose] [--all] (`--all` lists every pose with its miss at the finest scale) */
import { readFileSync } from "node:fs";
import { Box3 } from "three";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { isGlyph, loadGlb, sceneExtent } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️.ts";
import { contactSurfaces, extentMiss } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";

const args = process.argv.slice(2);
const all = args.includes("--all");
const [target, wanted] = args.filter((a) => a !== "--all");
const asset = target!.endsWith(".json") ? JSON.parse(readFileSync(target!, "utf8")).asset : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const f = (v: number): string => v.toFixed(4);
const outside = contactSurfaces(asset);
console.log(`${asset.id}: dimensions ahead ${asset.dimensions.widthM} up ${asset.dimensions.heightM} left ${asset.dimensions.depthM}, default pose ${asset.presentation.defaultPose}`);
for (const pose of all ? asset.poses.map((p: { id: string }) => p.id) : [wanted ?? asset.presentation.defaultPose]) {
  const bytes = await writeGlb(asset, { pose });
  const loaded = await loadGlb(bytes);
  for (const [index, scene] of loaded.scenes.entries()) {
    const name: string = loaded.parser.json.scenes[index].name;
    if (!name.endsWith("-body")) continue;
    const extent = sceneExtent(scene, undefined, outside);
    const whole = sceneExtent(scene);
    const scale = Number(/^s(\d+)-/u.exec(name)![1]);
    const miss = extentMiss(asset, extent, scale);
    console.log(`${pose} ${name}: ahead ${f(extent.min[0])} … ${f(extent.max[0])}, up ${f(extent.min[1])} … ${f(extent.max[1])}, left ${f(extent.min[2])} … ${f(extent.max[2])}; error ${miss.error.map((e) => `${(e * 100).toFixed(1)} %`).join(" / ")} miss ${miss.miss.toFixed(2)}; with the ${outside.size} contact surfaces ${[0, 1, 2].map((at) => f(whole.max[at]! - whole.min[at]!)).join(" / ")}`);
    if (all) break;
    const rows: { name: string; box: Box3 }[] = [];
    scene.traverse((object) => {
      if (!(object as { isMesh?: boolean }).isMesh || isGlyph(object)) return;
      rows.push({ name: `${object.parent?.name ?? ""}/${object.name}${(object as { isSkinnedMesh?: boolean }).isSkinnedMesh ? " (skinned)" : ""}`, box: new Box3().expandByObject(object, true) });
    });
    const line = (r: { name: string; box: Box3 }): string => `    ${r.name}: ahead ${f(r.box.min.z)} … ${f(r.box.max.z)}, up ${f(r.box.min.y)} … ${f(r.box.max.y)}, left ${f(r.box.min.x)} … ${f(r.box.max.x)}`;
    console.log(`  ${rows.length} meshes; farthest ahead:`);
    for (const r of [...rows].sort((a, b) => b.box.max.z - a.box.max.z).slice(0, 5)) console.log(line(r));
    console.log("  farthest behind:");
    for (const r of [...rows].sort((a, b) => a.box.min.z - b.box.min.z).slice(0, 5)) console.log(line(r));
  }
}
