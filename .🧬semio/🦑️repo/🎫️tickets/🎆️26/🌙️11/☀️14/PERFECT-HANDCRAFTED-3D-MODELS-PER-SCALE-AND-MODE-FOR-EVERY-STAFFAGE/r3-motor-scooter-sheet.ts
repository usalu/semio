/** 🛴 Round 3 motor-scooter copy of `r3-sheet.ts` that reads a candidate asset document with `--file` (rig and volumes as they are in that file) and writes under `🗑️generated/r3-motor-scooter/sheet/`. Renders one PNG sheet of one list of one scale: six shaded views at the same zoom (front, right side, back, left side, top, three-quarter from above) with silhouette, seam and crease edges, the rig drawn over them (bones from joint to tip, joints as dots, bone ids as labels), the ground line and a scale bar.
 *
 * Usage: `bun r3-sheet.ts <category>/<slug> --scale N [--list body|thermal|structure] [--pose ID] [--clip ID --t S] [--only front|right|back|left|top|three-quarter] [--cut] [--no-rig] [--no-labels] [--labels all] [--turn DEG --tilt DEG]`
 * Output: `🗑️generated/r3-sheet/<category>/<slug>.s<N>.<list>[.<pose>|.<clip>-t<S>][.<view>][.cut].png`
 * @see ./r3-tools.md
 * @see ./volumes-render.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { Canvas, type RGB } from "./volumes-raster.ts";
import { PART_COLOR, bodyItems, structureItems, thermalItems, thermalLegend } from "./volumes-render.ts";
import { GENERATED, entryOf, extentOf, featureM, type ListName, type SolidMesh } from "./volumes-lib.ts";
import { Panel, cameraAt, cli, dot, listsIn, load, paperBox, placeLabels, pointsOf, queryOf, r3Out, rigAt, savePng, thickLine, toScreen, type Cam, type Item, type Placement, type Rig } from "./r3-lib.ts";

type ViewSpec = { id: string; title: string; cam: Cam; ground: boolean };

const BG: RGB = [247, 247, 244];
const BONE: RGB = [225, 0, 95];
const USAGE = "r3-sheet.ts <category>/<slug> --scale N [--list body|thermal|structure] [--pose ID] [--clip ID --t S] [--only front|right|back|left|top|three-quarter] [--cut] [--no-rig] [--no-labels] [--labels all] [--turn DEG --tilt DEG]";

function views(turn: number, tilt: number): ViewSpec[] {
  return [
    { id: "front", title: "FRONT (SEEN FROM +X, PRODUCT VIEW FRONT)", cam: cameraAt(90, 0), ground: true },
    { id: "right", title: "RIGHT SIDE (FROM -Z, PRODUCT VIEW ELEVATION)", cam: cameraAt(0, 0), ground: true },
    { id: "back", title: "BACK (FROM -X)", cam: cameraAt(270, 0), ground: true },
    { id: "left", title: "LEFT SIDE (FROM +Z)", cam: cameraAt(180, 0), ground: true },
    { id: "top", title: "TOP (FROM +Y, PRODUCT VIEW PLAN, +X RIGHT)", cam: cameraAt(0, 90), ground: false },
    { id: "three-quarter", title: `THREE-QUARTER (TURN ${turn} TILT ${tilt})`, cam: cameraAt(turn, tilt), ground: false },
  ];
}

function drawRig(c: Canvas, x: number, y: number, w: number, h: number, p: Placement, rig: Rig, labels: boolean): number {
  const at = (q: readonly number[]): [number, number] => {
    const [sx, sy] = toScreen(p, q);
    return [x + sx, y + sy];
  };
  for (const b of rig.bones) {
    const [a, t] = [at(b.o), at(b.tip)];
    thickLine(c, a[0], a[1], t[0], t[1], [255, 255, 255], 4);
  }
  for (const b of rig.bones) {
    const [a, t] = [at(b.o), at(b.tip)];
    thickLine(c, a[0], a[1], t[0], t[1], BONE, 2);
  }
  for (const b of rig.bones) {
    const [a, t] = [at(b.o), at(b.tip)];
    dot(c, t[0], t[1], 2, BONE);
    dot(c, a[0], a[1], 3, [255, 225, 0], [30, 30, 30]);
  }
  if (!labels) return 0;
  const sorted = [...rig.bones].sort((m, n) => Math.hypot(n.tip[0] - n.o[0], n.tip[1] - n.o[1], n.tip[2] - n.o[2]) - Math.hypot(m.tip[0] - m.o[0], m.tip[1] - m.o[1], m.tip[2] - m.o[2]));
  return placeLabels(c, { x: x + 2, y: y + 24, w: w - 4, h: h - 26 }, sorted.map((b) => {
    const [a, t] = [at(b.o), at(b.tip)];
    return { x: (a[0] + t[0]) / 2, y: (a[1] + t[1]) / 2, text: b.id };
  }), [140, 0, 60], 2);
}

if (import.meta.main) {
  const { spec, flag, has } = cli(["--file", "--scale", "--list", "--pose", "--clip", "--t", "--only", "--turn", "--tilt", "--labels"]);
  const entry = entryOf(spec!).entry;
  const asset = JSON.parse(readFileSync(flag("--file") ?? join(GENERATED, "r3-motor-scooter", "🔣️.json"), "utf8")).asset;
  const scale = Number(flag("--scale"));
  const list = (flag("--list") ?? "body") as ListName;
  if (asset.volumes[String(scale)]?.status !== "authored") {
    process.stderr.write(`scale ${flag("--scale")} is not authored in the source of ${spec} (usage: ${USAGE})\n`);
    process.exit(2);
  }
  if (!["body", "thermal", "structure"].includes(list)) {
    process.stderr.write(`unknown list ${list}\n`);
    process.exit(2);
  }
  const query = queryOf(asset, flag("--pose"), flag("--clip"), flag("--t"));
  const rig = rigAt(asset, scale, query);
  const sm = listsIn(asset, scale, rig.frames);
  for (const e of sm.errors) process.stderr.write(`NOT DRAWN ${e}\n`);
  for (const e of sm.unresolved) process.stderr.write(`UNRESOLVED ${e}\n`);
  const body = bodyItems(asset, sm, scale).map(({ mesh, color }) => ({ mesh, color }));
  const items: Item[] = list === "body" ? body : (list === "thermal" ? thermalItems(asset, sm, scale) : structureItems(asset, sm, scale)).map(({ mesh, color }) => ({ mesh, color }));
  const ghost: Item[] = list === "structure" || (list === "thermal" && has("--cut")) ? body : [];
  const everything: SolidMesh[] = [...sm.body, ...sm[list]];
  const withRig = !has("--no-rig");
  const rigPoints = withRig ? rig.bones.map((b) => b.o) : [];
  const all = views(Number(flag("--turn") ?? 50), Number(flag("--tilt") ?? 28));
  const shown = flag("--only") === undefined ? all : all.filter((v) => v.id === flag("--only"));
  if (shown.length === 0) {
    process.stderr.write(`unknown view ${flag("--only")}\n`);
    process.exit(2);
  }
  const W = 2400;
  const cols = shown.length === 1 ? 1 : 3;
  const rows = Math.ceil(shown.length / cols);
  const pw = Math.floor(W / cols);
  const ph = shown.length === 1 ? 1480 : 740;
  const top = 26;
  const margin = 26;
  const boxes = shown.map((v) => paperBox(v.cam, [...pointsOf(everything), ...rigPoints]));
  const zoom = Math.min(...boxes.map((b) => Math.min((pw - 2 * margin) / (b.maxX - b.minX || 1), (ph - top - 2 * margin) / (b.maxY - b.minY || 1))));
  const legend = list === "thermal" ? thermalLegend(asset).map((l) => ({ label: l.label, color: l.color })) : list === "structure" ? Object.entries(PART_COLOR).map(([label, color]) => ({ label, color })) : [];
  const legendCols = 5;
  const footer = 80 + Math.ceil(legend.length / legendCols) * 20 + (sm.errors.length > 0 ? 20 : 0);
  const sheet = new Canvas(W, rows * ph + Math.max(120, footer), [255, 255, 255]);
  const ext = extentOf(everything);
  const long = ext === null ? 1 : Math.max(ext.max[0] - ext.min[0], ext.max[1] - ext.min[1], ext.max[2] - ext.min[2]);
  const barM = long >= 0.8 ? 1 : long >= 0.08 ? 0.1 : 0.01;
  let dropped = 0;
  shown.forEach((v, k) => {
    const [x, y] = [(k % cols) * pw, Math.floor(k / cols) * ph];
    const b = boxes[k]!;
    const place: Placement = { cam: v.cam, scale: zoom, cx: (b.minX + b.maxX) / 2, cy: (b.minY + b.maxY) / 2, ox: pw / 2, oy: top + (ph - top) / 2 };
    const panel = new Panel(pw, ph, BG, 2);
    const centre = ext === null ? 0 : [0, 1, 2].reduce((n, q) => n + ((ext.min[q]! + ext.max[q]!) / 2) * v.cam.e[q]!, 0);
    if (ghost.length > 0) {
      panel.draw(place, ghost, has("--cut") ? centre : Infinity);
      panel.shade(place);
      panel.ghost(0.28);
    }
    panel.draw(place, items, has("--cut") ? centre : Infinity);
    panel.shade(place);
    const c = panel.canvas();
    if (v.ground) {
      const gy = Math.round(toScreen(place, [0, 0, 0])[1]);
      if (gy > top && gy < ph - 1) {
        c.rect(0, gy, pw, 1, [120, 120, 120]);
        c.text(pw - 60, gy + 4, "Y = 0", [120, 120, 120], 1);
      }
    }
    sheet.blit(c, x, y);
    const d = withRig ? drawRig(sheet, x, y, pw, ph, place, rig, !has("--no-labels") && (flag("--labels") === "all" || shown.length === 1 || v.id === "right" || v.id === "front")) : 0;
    dropped += d;
    sheet.rect(x, y, pw, top - 2, [255, 255, 255]);
    sheet.text(x + 8, y + 5, `${v.title}${d > 0 ? `  (${d} LABELS HIDDEN)` : ""}`, [30, 30, 30], 2);
    const bar = Math.round(barM * zoom);
    sheet.rect(x + 14, y + ph - 20, bar, 4, [20, 20, 20]);
    sheet.rect(x + 14, y + ph - 26, 2, 14, [20, 20, 20]);
    sheet.rect(x + 14 + bar - 2, y + ph - 26, 2, 14, [20, 20, 20]);
    sheet.text(x + 22, y + ph - 40, `${barM} M`, [20, 20, 20], 2);
    sheet.frame(x, y, pw, ph, [170, 170, 165]);
  });
  const fy = rows * ph + 8;
  const what = query.kind === "pose" ? `POSE ${query.pose}` : `CLIP ${query.clip} T ${query.timeS} S`;
  sheet.text(10, fy, `${entry.category}/${entry.slug}  1:${scale}  ${list.toUpperCase()} ${sm[list].length} SOLIDS  ${what}  FEATURE ${(featureM(scale, asset) * 1000).toFixed(1)} MM  BAR ${barM} M${has("--cut") ? "  CUT: NEAR HALF REMOVED" : ""}`, [20, 20, 20], 2);
  sheet.text(10, fy + 20, `${withRig ? "RIG: MAGENTA LINE = BONE FROM JOINT (YELLOW DOT) TO TIP. LABEL = BONE ID AT MID BONE, IN FRONT AND RIGHT SIDE (--LABELS ALL: EVERY VIEW)." : ""}`, [70, 70, 70], 2);
  sheet.text(10, fy + 40, `EDGES: BLACK = OUTLINE OR DEPTH STEP, GREY = SEAM OF TWO SOLIDS OR CREASE.${ghost.length > 0 ? "  PALE = BODY GHOST." : ""}`, [70, 70, 70], 2);
  legend.forEach((e, i) => {
    const lx = 10 + (i % legendCols) * Math.floor((W - 20) / legendCols);
    const ly = fy + 64 + Math.floor(i / legendCols) * 20;
    sheet.rect(lx, ly, 14, 14, e.color);
    sheet.text(lx + 20, ly, e.label.slice(0, 36), [40, 40, 40], 2);
  });
  if (sm.errors.length > 0) sheet.text(10, sheet.height - 20, `NOT DRAWN (SEE STDERR): ${sm.errors.length} SOLIDS FAILED TO TESSELLATE`, [190, 20, 20], 2);
  const tag = query.kind === "pose" ? (flag("--pose") === undefined ? "" : `.${query.pose}`) : `.${query.clip}-t${query.timeS}`;
  const path = savePng(r3Out("r3-motor-scooter/sheet", entry, `${entry.slug}.s${scale}.${list}${tag}${flag("--only") === undefined ? "" : `.${flag("--only")}`}${has("--cut") ? ".cut" : ""}.png`), sheet);
  process.stdout.write(`${path}\n`);
}
