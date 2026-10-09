#!/usr/bin/env bun
/** 🗣️ Adds the en/de labels of the view command to the one `app_labels!` block of the BIM editor, and the command to the artifact root mount tree (idempotent). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, child, RS, subset } from "./r3-f1-paths.ts";

const terminology = join(child(child(subset, "editor"), "terminology"), RS);
let text = readFileSync(terminology, "utf8");
const rows = [
  'cmd_create_view: "Create View", "Ansicht anlegen";',
  'cmd_create_view_describe: "Creates a view of the building: the plan or ceiling plan of a storey, a section, an elevation, the four elevations at once (kind elevations) or an orthographic or perspective camera, and binds the addressed window to it.", "Legt eine Ansicht des Gebäudes an: den Grundriss oder Deckenspiegel eines Geschosses, einen Schnitt, eine Fassade, die vier Fassaden auf einmal (Art elevations) oder eine orthografische oder perspektivische Kamera, und bindet das adressierte Fenster daran.";',
  'name_south: "South", "Süd";',
  'name_east: "East", "Ost";',
  'name_north: "North", "Nord";',
  'name_west: "West", "West";',
  'name_section: "Section", "Schnitt";',
  'name_camera: "3D view", "3D-Ansicht";',
  'view_plan_of: "Plan {name}", "Grundriss {name}";',
  'view_ceiling_of: "Ceiling plan {name}", "Deckenspiegel {name}";',
];
if (!text.includes("cmd_create_view:")) {
  const anchor = "    cmd_delete_selection: ";
  if (!text.includes(anchor)) throw new Error("anchor missing");
  text = text.replace(anchor, rows.map((row) => `    ${row}\n`).join("") + anchor);
}
text = text.replace(/cmd_set_view_describe: "Sets one view parameter of the addressed window \(storey, cut height, projection, isolated storey, section plane, section line, depth\); the document does not change\.", "[^"]*";/, 'cmd_set_view_describe: "Sets one view parameter of the addressed window (the authored view it shows, projection, isolated storey, section plane); the document does not change.", "Setzt einen Ansichtsparameter des adressierten Fensters (die gezeigte Ansicht, Projektion, isoliertes Geschoss, Schnittebene); das Dokument ändert sich nicht.";');
writeFileSync(terminology, text);

const root = join(artifact, RS);
let source = readFileSync(root, "utf8");
if (!source.includes("🎮️commands/🔭️create-view")) {
  const anchor = /( {16}#\[path = "\."\]\n {16}pub mod delete_selection \{)/;
  if (!anchor.test(source)) throw new Error("commands anchor missing");
  const commands = child(child(subset, "editor"), "commands");
  const module = join(child(commands, "create-view"), RS);
  const rel = module.slice(artifact.length + 1).replaceAll("\\", "/");
  source = source.replace(anchor, `                #[path = "."]\n                pub mod create_view {\n                    #[path = "${rel}"]\n                    mod component;\n                    pub use component::*;\n                }\n$1`);
  writeFileSync(root, source);
}
console.log("labels and command mounted");
