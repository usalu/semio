import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const lines = readFileSync(file, "utf8").split("\n");
const find = (needle: string, from = 0): number => {
  const i = lines.findIndex((l, n) => n >= from && l.includes(needle));
  if (i < 0) throw new Error(`missing ${needle}`);
  return i;
};
const push = find('(ambiguous.get(shape.bone) ?? ambiguous.set(shape.bone, []).get(shape.bone)!).push(shape.id);');
lines[push] = [
  "      if (shape.solid === undefined && shape.role !== \"ground\" && !path.startsWith(\"symbols\") && bones.has(shape.bone) && !out.some((issue) => issue.code.startsWith(\"rig-\") || issue.code === \"duplicate-id\") && spreadBones().has(shape.bone)) {",
  "        const viewpoint = path.endsWith(\".geometry.front\") ? \"front\" : path.endsWith(\".geometry.plan\") ? \"plan\" : \"elevation\";",
  "        const theta = tiltOf(shape.bone, viewpoint);",
  "        const rings = flattenGeometry(shape).rings.flat();",
  "        const extent = rings.length === 0 ? 0 : Math.max(Math.max(...rings.map((p) => p.x)) - Math.min(...rings.map((p) => p.x)), Math.max(...rings.map((p) => p.y)) - Math.min(...rings.map((p) => p.y)));",
  "        const denominator = Number(/representations\\.(\\d+)\\./u.exec(path)?.[1] ?? 0);",
  "        const errorMm = denominator > 0 ? (extent * Math.sin(theta) * 1000) / denominator : 0;",
  "        if (errorMm > DEFAULT_PRINT_PROFILE.minFeatureMm) {",
  "          const entry = ambiguous.get(shape.bone) ?? ambiguous.set(shape.bone, { ids: [], thetaDeg: 0, errorMm: 0 }).get(shape.bone)!;",
  "          entry.ids.push(shape.id);",
  "          entry.thetaDeg = Math.max(entry.thetaDeg, (theta * 180) / Math.PI);",
  "          entry.errorMm = Math.max(entry.errorMm, errorMm);",
  "        }",
  "      }",
].join("\n");
const decl = find("const ambiguous = new Map<Slug, string[]>();");
lines[decl] = "    const ambiguous = new Map<Slug, { ids: string[]; thetaDeg: number; errorMm: number }>();";
const emit = find("for (const [bone, ids] of ambiguous)");
lines[emit] = [
  "    for (const [bone, { ids, thetaDeg, errorMm }] of ambiguous) add(path, \"lateral-ambiguous\", `${ids.length} flat shape(s) (${ids.slice(0, 6).join(\", \")}${ids.length > 6 ? \", ...\" : \"\"}) on bone \"${bone}\" would be drawn ${errorMm.toFixed(2)} mm wrong on paper (above the ${DEFAULT_PRINT_PROFILE.minFeatureMm} mm print minimum): the bone leaves the shapes' plane by up to ${thetaDeg.toFixed(1)} degrees in a pose or clip, so author them as solids (extruded or revolved).`, `${ids.length} flache Form(en) (${ids.slice(0, 6).join(\", \")}${ids.length > 6 ? \", ...\" : \"\"}) am Knochen \"${bone}\" würden auf dem Papier um ${errorMm.toFixed(2)} mm falsch gezeichnet (über dem Druckminimum von ${DEFAULT_PRINT_PROFILE.minFeatureMm} mm): der Knochen verlässt die Ebene der Formen in einer Pose oder einem Clip um bis zu ${thetaDeg.toFixed(1)} Grad, daher als Körper (extrudiert oder rotiert, solid) zeichnen.`);",
].join("\n");
const spread = find("  let spreadSet: Set<Slug> | undefined;");
lines.splice(spread, 0,
  "  const tilts = new Map<Viewpoint, Map<Slug, number>>();",
  "  const tiltOf = (bone: Slug, viewpoint: Viewpoint): number => {",
  "    let table = tilts.get(viewpoint);",
  "    if (table === undefined) {",
  "      table = new Map();",
  "      tilts.set(viewpoint, table);",
  "      const normal = viewpoint === \"elevation\" ? ([0, 0, 1] as const) : viewpoint === \"front\" ? ([1, 0, 0] as const) : ([0, 1, 0] as const);",
  "      const rest = restFrames(asset.rig);",
  "      const samples = [...asset.poses.map((pose) => samplePose(asset, pose)), ...asset.clips.flatMap((clip) => Array.from({ length: TILT_SAMPLES }, (_, i) => sampleClip(asset, clip, (clip.durationS * i) / (TILT_SAMPLES - 1))))];",
  "      for (const sample of samples) {",
  "        const now = solveChain(asset.rig, sample.offsets, sample.root);",
  "        for (const id of Object.keys(now)) {",
  "          const r = rest[id]!.frame;",
  "          const n = now[id]!.frame;",
  "          const dotWith = (a: readonly number[]): number => a[0]! * normal[0] + a[1]! * normal[1] + a[2]! * normal[2];",
  "          const carried = [0, 1, 2].map((axis) => n.u[axis]! * dotWith(r.u) + n.v[axis]! * dotWith(r.v) + n.w[axis]! * dotWith(r.w));",
  "          const cosine = Math.min(1, Math.max(-1, carried[0]! * normal[0] + carried[1]! * normal[1] + carried[2]! * normal[2]));",
  "          table.set(id, Math.max(table.get(id) ?? 0, Math.acos(cosine)));",
  "        }",
  "      }",
  "    }",
  "    return table.get(bone) ?? 0;",
  "  };",
);
let text = lines.join("\n");
text = text.replace('import { sampleClip } from "../🎞️animation/🟦️.ts";', 'import { sampleClip, samplePose } from "../🎞️animation/🟦️.ts";');
text = text.replace('import { effectiveOffset, restFrames } from "../🦴️rig/🟦️.ts";', 'import { effectiveOffset, restFrames, solveChain } from "../🦴️rig/🟦️.ts";');
text = text.replace('import { measureSuitability } from "../🖨️representation/🟦️.ts";', 'import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../🖨️representation/🟦️.ts";');
text = text.replace("/** 📏️ The overall paper extent in millimetres from which", "/** 🎞️ The instants per clip at which the tilt of a bone out of the plane of its shapes is sampled. */\nconst TILT_SAMPLES = 9;\n\n/** 📏️ The overall paper extent in millimetres from which");
writeFileSync(file, text);
