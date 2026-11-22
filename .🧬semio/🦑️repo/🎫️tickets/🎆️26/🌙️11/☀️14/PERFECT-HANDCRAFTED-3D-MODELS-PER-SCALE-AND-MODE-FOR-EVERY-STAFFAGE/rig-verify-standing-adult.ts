/** 🧍️ Proof on `humans/standing-adult` (lateral clavicles) that every consumer of the bone frames agrees with `evaluate`: per pose and clip instant the drawn arm, forearm and hand shapes of the drawing scene (what SVG and PDF serialise) against the evaluated shapes and the extent of the CAD scene (what DXF and DWG serialise) against the extent of the drawn shapes, the GLB and FBX nodes loaded by three against the evaluated bone frames, the clavicle solids of the GLB against the line from the sternoclavicular to the acromioclavicular joint, and the joint load of the clavicle against the weight and lever of the arm chain. Writes the files and PNGs to `🗑️generated/rig-verify/standing-adult/`. Usage: `bun rig-verify-standing-adult.ts`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";
import { AnimationMixer, Box3, Vector3, type Object3D } from "three";
import { poseModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { defaultDepictionOptions, depict, modelScene } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { writeFbx } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeDxf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📐️dxf/🟦️.ts";
import { resolveSheet } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📑️sheet/🟦️.ts";
import { writePdf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { writeSvg } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { writeGlb } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { solveStructure } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
import { solveThermalAt } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
import { modelOfAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { loadFbx, loadGlb } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️.ts";

const S = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const OUT = join(import.meta.dir, "🗑️generated/rig-verify/standing-adult");
mkdirSync(OUT, { recursive: true });
const asset = (JSON.parse(readFileSync(join(S, "🗂️catalog/👤️humans/standing-adult/🔣️.json"), "utf8")) as { asset: Asset }).asset;
const SCALE = 50;
const select = { kind: "scale", scale: { denominator: SCALE } } as const;
const VIEWPOINTS = ["elevation", "front", "plan"] as const;
let worst = { svg: 0, dxf: 0, glb: 0, fbx: 0, model: 0 };
const queries = [...asset.poses.map((pose) => ({ kind: "pose", pose: pose.id }) as const), ...asset.clips.flatMap((clip) => [0.3, 0.7].map((f) => ({ kind: "clip", clip: clip.id, timeS: f * clip.durationS }) as const))];
const box = (points: readonly { x: number; y: number }[]) => [Math.min(...points.map((p) => p.x)), Math.max(...points.map((p) => p.x)), Math.min(...points.map((p) => p.y)), Math.max(...points.map((p) => p.y))];

for (const query of queries) {
  const tag = query.kind === "pose" ? query.pose : `${query.clip}-t${query.timeS.toFixed(2)}`;
  const state = evaluate(asset, query, select);
  for (const viewpoint of VIEWPOINTS) {
    const scene = depict(state, defaultDepictionOptions("light-solid", "en", SCALE, viewpoint));
    const k = scene.metadata.mmPerMetre, o = scene.metadata.origin;
    const want = new Map<string, number[]>();
    for (const shape of state.shapes[viewpoint]) if (/^(arm|forearm|hand)-[lr]$/u.test(shape.shape.bone) && shape.closed) want.set(`shape:${shape.shape.id}`, box(shape.rings.flat()));
    let seen = 0;
    for (const item of scene.layers.flatMap((layer) => layer.items)) {
      if (item.kind !== "path" || !want.has(item.id) || item.bone === undefined) continue;
      const points = item.commands.flatMap((c) => (c.op === "Z" ? [] : c.op === "C" ? [{ x: c.x1, y: c.y1 }, { x: c.x2, y: c.y2 }, { x: c.x, y: c.y }] : [{ x: c.x, y: c.y }])).map((p) => ({ x: (p.x - o.xMm) / k, y: (o.yMm - p.y) / k }));
      const [got, expected] = [box(points), want.get(item.id)!];
      worst.svg = Math.max(worst.svg, ...got.map((v, i) => Math.abs(v - expected[i]!)));
      seen++;
    }
    if (seen === 0) console.log(`[DEBUG] ${tag} ${viewpoint}: no arm shape drawn`);
    const cad = modelScene(state, { viewpoint });
    const all = cad.layers.flatMap((layer) => layer.polylines);
    const drawn = box(scene.layers.flatMap((layer) => layer.items).flatMap((item) => (item.kind === "path" && item.id.startsWith("shape:") ? item.commands.flatMap((c) => (c.op === "Z" ? [] : [{ x: (c.x - o.xMm) / k, y: (o.yMm - c.y) / k }])) : [])));
    const lines = box(cad.layers.filter((layer) => layer.name !== "GROUND").flatMap((layer) => layer.polylines.flatMap((line) => line.points)));
    worst.dxf = Math.max(worst.dxf, ...lines.map((v, i) => Math.abs(v - drawn[i]!)));
    if (query.kind === "pose" && query.pose === "arms-crossed") {
      writeFileSync(join(OUT, `${tag}.${viewpoint}.svg`), writeSvg(scene));
      writeFileSync(join(OUT, `${tag}.${viewpoint}.dxf`), writeDxf(cad));
      writeFileSync(join(OUT, `${tag}.${viewpoint}.pdf`), writePdf([scene]));
    }
  }
}
for (const [view, pose] of [["light-solid", "arms-crossed"], ["forces", "arms-crossed"], ["moments", "arms-crossed"], ["thermal", "arms-crossed"], ["light-solid", "pointing"]] as const) {
  const { view: _v, viewpoint: _p, scale: _s, representation: _r, ...rest } = defaultDepictionOptions(view, "en", SCALE);
  const { sheet } = resolveSheet(asset, { kind: "pose", pose }, { kind: "scale", denominator: SCALE }, view, rest as never, "first", "en");
  const svg = writeSvg(sheet);
  writeFileSync(join(OUT, `sheet.${pose}.${view}.svg`), svg);
  await sharp(Buffer.from(svg), { density: 500, limitInputPixels: false }).flatten({ background: "#ffffff" }).resize(2200, 1500, { fit: "contain", background: "#ffffff" }).png().toFile(join(OUT, `sheet.${pose}.${view}.png`));
}

const expectedWorld = (f: { o: { x: number; y: number; z: number } }) => [f.o.z, f.o.y, f.o.x];
const document = await modelOfAsset(asset, { at: { kind: "pose", pose: "relaxed" }, poses: "all", clips: "all", scales: [SCALE] });
for (const query of queries) {
  const posed = query.kind === "pose" ? poseModel(document, query.pose) : poseModel(document, { animation: query.clip, timeS: query.timeS });
  if (posed.issues.length > 0) console.log(`model replay ${JSON.stringify(query)}: ${posed.issues.map((issue) => `${issue.code} ${document.nodes[Number(issue.path.split("/")[2])]?.id}`).join(", ")}`);
  let here = 0, where = "";
  const state = evaluate(asset, query, select);
  for (const bone of asset.rig.bones) {
    const w = posed.world[bone.id]!, f = state.bones[bone.id]!.frame;
    const axes = [f.u.z, f.u.y, f.u.x, f.v.z, f.v.y, f.v.x, -f.w.z, -f.w.y, -f.w.x];
    const got = [w[0]!, w[1]!, w[2]!, w[4]!, w[5]!, w[6]!, w[8]!, w[9]!, w[10]!];
    const d = Math.max(...expectedWorld(f).map((v, i) => Math.abs(v - w[12 + i]!)), ...(query.kind === "pose" ? axes.map((v, i) => Math.abs(v - got[i]!)) : []));
    worst.model = Math.max(worst.model, d);
    if (d > here) { here = d; where = bone.id; }
  }
  console.log(`model replay ${query.kind === "pose" ? query.pose : `${query.clip} at ${query.timeS.toFixed(2)} s`}: largest difference ${here.toExponential(2)} m at ${where}`);
}
for (const pose of asset.poses) {
  const bytes = await writeGlb(asset, { pose: pose.id, scales: [SCALE] });
  if (pose.id === "arms-crossed") writeFileSync(join(OUT, "arms-crossed.glb"), bytes);
  const { scene, scenes } = await loadGlb(bytes);
  const state = evaluate(asset, { kind: "pose", pose: pose.id }, select);
  for (const one of scenes) {
    one.updateMatrixWorld(true);
    for (const bone of asset.rig.bones) {
      const node = one.getObjectByName(bone.id);
      if (node === undefined) continue;
      const at = node.getWorldPosition(new Vector3());
      worst.glb = Math.max(worst.glb, ...expectedWorld(state.bones[bone.id]!.frame).map((v, i) => Math.abs(v - [at.x, at.y, at.z][i]!)));
    }
    for (const side of ["l", "r"] as const) {
      const bone = one.getObjectByName(`clavicle-${side}`);
      if (bone === undefined) continue;
      const b = new Box3();
      const names: string[] = [];
      const walk = (object: Object3D): void => {
        if ((object as { isMesh?: boolean }).isMesh) { b.expandByObject(object, true); names.push(object.name); }
        for (const child of object.children) if (!asset.rig.bones.some((c) => c.id === child.name && c.id !== `clavicle-${side}`)) walk(child);
      };
      walk(bone);
      const f = state.bones[`clavicle-${side}`]!;
      if (!b.isEmpty() && pose.id !== "pointing" && pose.id !== "hands-warming") console.log(`glb ${pose.id} scene ${one.name} clavicle-${side}: ${names.length} meshes, left ${b.min.x.toFixed(3)}..${b.max.x.toFixed(3)} up ${b.min.y.toFixed(3)}..${b.max.y.toFixed(3)} ahead ${b.min.z.toFixed(3)}..${b.max.z.toFixed(3)}; bone joint [${f.frame.o.x.toFixed(3)}, ${f.frame.o.y.toFixed(3)}, ${f.frame.o.z.toFixed(3)}] tip [${f.tip.x.toFixed(3)}, ${f.tip.y.toFixed(3)}, ${f.tip.z.toFixed(3)}]`);
    }
  }
  void scene;
}
for (const clip of asset.clips) {
  const fbx = loadFbx(await writeFbx(asset, { clip: clip.id, scale: SCALE }));
  const mixer = new AnimationMixer(fbx);
  mixer.clipAction(fbx.animations[0]!).play();
  for (const f of [0, 0.3, 0.7]) {
    mixer.setTime(f * clip.durationS);
    fbx.updateMatrixWorld(true);
    const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS: f * clip.durationS }, select);
    for (const bone of asset.rig.bones) {
      const at = fbx.getObjectByName(bone.id)!.getWorldPosition(new Vector3());
      worst.fbx = Math.max(worst.fbx, ...expectedWorld(state.bones[bone.id]!.frame).map((v, i) => Math.abs(v - [at.x, at.y, at.z][i]!)));
    }
  }
}
console.log(`largest difference to evaluate, metres: ${JSON.stringify(worst)} over ${queries.length} poses and clip instants`);

const state = evaluate(asset, { kind: "pose", pose: "relaxed" }, select);
const result = solveStructure(state);
const g = result.gravityMS2;
for (const side of ["l", "r"] as const) {
  const joint = result.joints[`clavicle-${side}`]!;
  const origin = state.bones[`clavicle-${side}`]!.frame.o;
  let weight = 0;
  const moment = [0, 0, 0];
  for (const w of result.segmentWeights) {
    let at: string | null = w.bone, inside = false;
    while (at !== null) { if (at === `clavicle-${side}`) inside = true; at = asset.rig.bones.find((b) => b.id === at)!.parent; }
    if (!inside) continue;
    weight += w.fyN;
    const r = [w.at.x - origin.x, w.at.y - origin.y, w.z - origin.z];
    moment[0] += r[1]! * 0 - r[2]! * w.fyN; moment[2] += r[0]! * w.fyN;
  }
  console.log(`structure relaxed clavicle-${side}: subtree weight ${weight.toFixed(4)} N (${(-weight / g).toFixed(4)} kg); joint moment product [${joint.mxNm.toFixed(4)}, ${joint.myNm.toFixed(4)}, ${joint.mzNm.toFixed(4)}] vs r x F summed here [${moment[0]!.toFixed(4)}, ${moment[1]!.toFixed(4)}, ${moment[2]!.toFixed(4)}] N m; force on child fy ${joint.fyN.toFixed(4)} N`);
}
const thermal = solveThermalAt(asset, { kind: "pose", pose: "arms-crossed" }, undefined);
console.log(`thermal arms-crossed solved: ${Object.keys(thermal).slice(0, 8).join(", ")}`);
