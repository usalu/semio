#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../..");

function patchJson(rel: string, mutator: (data: Record<string, unknown>) => void): void {
  const path = join(root, rel);
  const data = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
  mutator(data);
  writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`);
  console.log("patched", rel);
}

patchJson("🎓️teaching/🏛️architecture/❓️quiz/🔣️.json", (data) => {
  data.short = { en: "A&T Quizzes", de: "A&T Quizze" };
});

patchJson("🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json", (data) => {
  data.short = { en: "Physics", de: "Physik" };
});

patchJson("🎓️teaching/🏛️architecture/⚡️energy/📊️demand/❓️quiz/🔣️.json", (data) => {
  data.short = { en: "Demand", de: "Bedarf" };
});

const playCatalogPath = join(root, "🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json");
const playCatalog = JSON.parse(readFileSync(playCatalogPath, "utf8")) as {
  groups: { panes: { label: string; shortLabel?: string }[] }[];
};
const playShort: Record<string, string> = {
  "Procedural 3D": "Proc 3D",
  "Procedural 2D": "Proc 2D",
  "Wave Function Collapse in 2D": "WFC 2D",
  "Wave function collapse in 2D": "WFC 2D",
  "Wave Function Collapse in 3D": "WFC 3D",
  "Wave function collapse in 3D": "WFC 3D",
  "Analyse plane frames": "FEM 2D",
  "Analyse spatial frames": "FEM 3D",
  "Simulate building energy": "Energy",
  "Plan fabrication steps": "Process",
  "Explore places on a map": "Map",
  "Explore terrain in 3D": "Terrain",
  "Program buildings": "Architect",
  "Assemble across time and variants": "Puzzle 5D",
  "Blocks across time and variants": "Block 5D",
  "Author 2D tile grids": "Grid 2D",
  "Author 3D module grids": "Grid 3D",
  "Learn from sample images": "Bitmap",
  "Curate components": "Sourcing",
  "Reconstruct from captures": "Remodel",
  "Model low-poly meshes": "Low Poly",
  "Visual dataflow programming": "Flow",
  "Arrange pages": "Layout",
  "Stage cameras and shots": "Shooting",
  "Vector drawing": "Draw",
  "Paint pixels": "Raster",
};
for (const group of playCatalog.groups) {
  for (const pane of group.panes) {
    const short = playShort[pane.label];
    if (short !== undefined && short !== pane.label) pane.shortLabel = short;
  }
}
writeFileSync(playCatalogPath, `${JSON.stringify(playCatalog, null, 2)}\n`);
console.log("patched play catalog");

const demonstratorBrandPath = join(root, "♻️mit-bestand/🧺️demonstrator/🪧️brand.ts");
let demonstrator = readFileSync(demonstratorBrandPath, "utf8");
const demonstratorShort: Record<string, string> = {
  "Entwerfen mit Bestand · Aggregator": "EmB · Agg",
  "Entwerfen mit Bestand · Generator": "EmB · Gen",
  "Entwerfen mit Bestand · Koordinator": "EmB · Koord",
  "Entwerfen mit Bestand · Aussuchen": "EmB · Aus",
  "Entwerfen mit Bestand · Bearbeiten": "EmB · Bearb",
  "Entwerfen mit Bestand · Verfolgen": "EmB · Verf",
  "Entwerfen mit Bestand · Energie": "EmB · Energie",
  "Entwerfen mit Bestand · Statik": "EmB · Statik",
};
for (const [windowTitle, shortWindowTitle] of Object.entries(demonstratorShort)) {
  const needle = `windowTitle: "${windowTitle}",`;
  const replacement = `windowTitle: "${windowTitle}",\n  shortWindowTitle: "${shortWindowTitle}",`;
  if (!demonstrator.includes(`shortWindowTitle: "${shortWindowTitle}"`)) {
    if (!demonstrator.includes(needle)) throw new Error(`missing brand ${windowTitle}`);
    demonstrator = demonstrator.replace(needle, replacement);
  }
}
writeFileSync(demonstratorBrandPath, demonstrator);
console.log("patched demonstrator brands");
