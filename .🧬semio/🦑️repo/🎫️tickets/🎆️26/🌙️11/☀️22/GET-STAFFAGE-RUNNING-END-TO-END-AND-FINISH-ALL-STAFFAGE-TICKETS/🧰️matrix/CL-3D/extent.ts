/** 📐️ The extent of every body scene of the GLB of the default pose against `dimensions`, as the bounds check of `🧊️model-catalog` reads it (three's `GLTFLoader`, glyphs left out), and per scene the objects that reach farthest on the lateral axis and how many mesh objects it has; with `--match <text>` the box (lateral, up, ahead) of every mesh whose node name holds the text. extent.ts <asset id | file.json> [scene…] [--match <text>] */
import { readFileSync } from "node:fs";
import { Box3 } from "three";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { isGlyph, loadGlb, sceneExtent } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️.ts";
import { extentMiss } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";

const [target, ...rest] = process.argv.slice(2);
const match = rest.includes("--match") ? rest[rest.indexOf("--match") + 1] : undefined;
const wanted = rest.filter((a, i) => a !== "--match" && rest[i - 1] !== "--match");
const asset = target!.endsWith(".json") ? JSON.parse(readFileSync(target!, "utf8")).asset : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const bytes = await writeGlb(asset, { pose: asset.presentation.defaultPose });
const loaded = await loadGlb(bytes);
console.log(`${asset.id} ${asset.presentation.defaultPose}: ${bytes.length} bytes, dimensions ahead ${asset.dimensions.widthM} up ${asset.dimensions.heightM} lateral ${asset.dimensions.depthM}`);
for (const [index, scene] of loaded.scenes.entries()) {
  const name: string = loaded.parser.json.scenes[index].name;
  if (wanted.length > 0 ? !wanted.includes(name) : !name.endsWith("-body")) continue;
  const extent = sceneExtent(scene);
  const scale = Number(/^s(\d+)-/u.exec(name)![1]);
  const miss = extentMiss(asset, extent, scale);
  const rows: { name: string; lo: number; hi: number; box: Box3 }[] = [];
  scene.traverse((object) => {
    if (!(object as { isMesh?: boolean }).isMesh || isGlyph(object)) return;
    const box = new Box3().expandByObject(object, true);
    rows.push({ name: object.parent?.name ?? object.name, lo: box.min.x, hi: box.max.x, box });
  });
  console.log(`${name}: size ${extent.max.map((v, i) => (v - extent.min[i]!).toFixed(4)).join(" / ")} error ${miss.error.map((e) => `${(e * 100).toFixed(1)} %`).join(" / ")} miss ${miss.miss.toFixed(2)}; ${rows.length} meshes; lateral ${extent.min[2].toFixed(4)} … ${extent.max[2].toFixed(4)}`);
  console.log(`  lowest: ${[...rows].sort((a, b) => a.lo - b.lo).slice(0, 4).map((r) => `${r.name} ${r.lo.toFixed(3)}`).join(", ")}`);
  console.log(`  highest: ${[...rows].sort((a, b) => b.hi - a.hi).slice(0, 4).map((r) => `${r.name} ${r.hi.toFixed(3)}`).join(", ")}`);
  if (match !== undefined) for (const r of rows.filter((row) => row.name.includes(match))) console.log(`  ${r.name}: lateral ${r.box.min.x.toFixed(3)} … ${r.box.max.x.toFixed(3)}, up ${r.box.min.y.toFixed(3)} … ${r.box.max.y.toFixed(3)}, ahead ${r.box.min.z.toFixed(3)} … ${r.box.max.z.toFixed(3)}`);
}
