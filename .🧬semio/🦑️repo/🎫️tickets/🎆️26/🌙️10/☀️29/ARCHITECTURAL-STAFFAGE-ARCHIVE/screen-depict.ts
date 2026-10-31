/** 🖥️ Depicts one asset the office-laser profile rejects, using the same zero floors the site uses on screen. */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { depict, evaluate, solveStructure, solveThermal, VIEWS, type AssetDocument, type PrintProfile, type View } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const screen: PrintProfile = {
  id: "screen",
  title: { en: "Screen", de: "Bildschirm" },
  description: { en: "On-screen reading of the archive. It does not certify that the drawing meets a print profile.", de: "Bildschirmlesung des Archivs. Sie bescheinigt nicht, dass die Zeichnung ein Druckprofil erfüllt." },
  minStrokeMm: 0,
  minGapMm: 0,
  minFeatureMm: 0,
  minHatchSpacingMm: 0,
  source: "authored-estimate",
};

const dir = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/walking-commuter";
const file = readdirSync(dir).find((name) => name.endsWith(".json"));
if (file === undefined) throw new Error("missing walking-commuter");
const asset = (JSON.parse(readFileSync(join(dir, file), "utf8")) as AssetDocument).asset;
const pose = asset.poses[0];
if (pose === undefined) throw new Error("no pose");
const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: 100 } });
const views: View[] = ["dark-line", "thermal", "forces"];
for (const view of views) {
  if (!(VIEWS as readonly string[]).includes(view)) throw new Error(view);
  const thermal = view === "thermal" ? solveThermal(state) : undefined;
  const structure = view === "forces" ? solveStructure(state) : undefined;
  const scene = depict(state, {
    view,
    viewpoint: "elevation",
    scale: { denominator: 100 },
    profile: screen,
    representation: { kind: "scale", scale: { denominator: 100 } },
    language: "en",
    transparent: true,
    marginMm: 5,
    annotations: { dimensions: false, scaleBar: false, legend: true, labels: false, ground: true },
    thermalQuantity: "surface-temperature",
    forceQuantity: "axial",
    diagrams: { axial: false, shear: false, moment: true, freeBody: true },
    range: { mode: "auto" },
    patterns: false,
    transport: [],
  }, { ...(thermal === undefined ? {} : { thermal }), ...(structure === undefined ? {} : { structure }) });
  console.log(`OK ${view} layers=${scene.layers.length} ${scene.widthMm.toFixed(1)}x${scene.heightMm.toFixed(1)}mm legends=${scene.legends.length}`);
}
