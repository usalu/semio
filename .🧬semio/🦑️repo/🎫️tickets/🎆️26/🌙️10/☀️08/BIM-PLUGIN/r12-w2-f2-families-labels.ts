#!/usr/bin/env bun
/**
 * 🗣️ Package `w2-f2-families`: inserts the en+de labels of the family editor (windows, entity kinds, fields, categories, parameter kinds, solid shapes, the `editFamily` command and its row buttons)
 * into the one `app_labels!` block of the BIM editor terminology, each block directly after a stable anchor row; rows that already exist are left alone, so the script is idempotent.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { subset, child, RS } from "./r3-f1-paths.ts";

const file = join(child(child(subset, "editor"), "terminology"), RS);
const row = (field: string, en: string, de: string) => `    ${field}: ${JSON.stringify(en)}, ${JSON.stringify(de)};`;

const blocks: [anchor: string, rows: string[]][] = [
  ["    window_schedule:", [
    row("window_family", "Family editor", "Familieneditor"),
    row("window_family_view", "Family preview", "Familienvorschau"),
  ]],
  ["    surface_schedule_describe:", [
    row("surface_family_describe", "Table of the parameters and the solids of one family: change a formula and read its evaluated value and its issues beside it; the row buttons add and remove parameters and solids.", "Tabelle der Parameter und Körper einer Familie: Ändern Sie eine Formel und lesen Sie daneben ihren ausgewerteten Wert und ihre Probleme; die Zeilenschaltflächen fügen Parameter und Körper hinzu oder entfernen sie."),
    row("surface_family_view_describe", "Three-dimensional preview of the visible solids of one family; it follows every change of a formula.", "Dreidimensionale Vorschau der sichtbaren Körper einer Familie; sie folgt jeder Änderung einer Formel."),
  ]],
  ["    kind_wall_sweep:", [
    row("kind_family", "Family", "Familie"),
    row("kind_family_solid", "Family solid", "Familienkörper"),
  ]],
  ["    group_wall_sweeps:", [
    row("group_families", "Families", "Familien"),
    row("group_family_solids", "Family solids", "Familienkörper"),
  ]],
  ["    field_volume:", [
    row("fam_field_visible", "Visible when", "Sichtbar wenn"),
    row("fam_field_issues", "Issues", "Probleme"),
  ]],
  ["    cmd_edit_schedule_describe:", [
    row("cmd_edit_family", "Edit Family", "Familie bearbeiten"),
    row("cmd_edit_family_describe", "Edits one part of a family (its name or category, a parameter, a solid or one formula of a solid) as one undoable step; the values, the solids and the outline follow by inference, and a formula that does not parse, names an unknown parameter or closes a circle is refused.", "Bearbeitet einen Teil einer Familie (Name oder Kategorie, einen Parameter, einen Körper oder eine Formel eines Körpers) als einen rückgängig zu machenden Schritt; Werte, Körper und Umriss folgen durch Ableitung, und eine Formel, die sich nicht lesen lässt, einen unbekannten Parameter nennt oder einen Zirkelbezug schließt, wird abgelehnt."),
  ]],
  ["    fault_template_edit_invalid:", [
    row("fault_create_family_missing", "A family solid needs its family first.", "Ein Familienkörper braucht zuerst seine Familie."),
    row("fault_family_missing", "That family does not exist.", "Diese Familie gibt es nicht."),
    row("fault_family_part_unknown", "A family has no such part to edit.", "Diese Familie hat keinen solchen Teil zum Bearbeiten."),
    row("fault_family_operation_unknown", "That edit does not exist for this part of the family.", "Diese Änderung gibt es für diesen Teil der Familie nicht."),
    row("fault_family_formula_invalid", "That formula cannot be read; write a value with its unit or an expression of the parameters.", "Diese Formel lässt sich nicht lesen; schreiben Sie einen Wert mit Einheit oder einen Ausdruck der Parameter."),
    row("fault_family_kind_unknown", "That parameter kind does not exist.", "Diese Parameterart gibt es nicht."),
    row("fault_family_category_unknown", "That family category does not exist.", "Diese Familienkategorie gibt es nicht."),
    row("fault_family_shape_unknown", "That solid shape does not exist.", "Diese Körperform gibt es nicht."),
    row("fault_family_material_missing", "The model has no material to give a new solid.", "Das Modell hat keinen Baustoff, den ein neuer Körper erhalten könnte."),
    row("fault_family_material_unquotable", "That material id cannot be written as a formula.", "Diese Baustoff-Kennung lässt sich nicht als Formel schreiben."),
    row("fault_family_solid_missing", "That solid is not part of the family.", "Dieser Körper gehört nicht zur Familie."),
    row("fault_family_slot_unknown", "This solid has no such formula.", "Dieser Körper hat keine solche Formel."),
    row("fault_family_axis_unknown", "That axis does not exist; choose X, Y or Z.", "Diese Achse gibt es nicht; wählen Sie X, Y oder Z."),
    row("fault_family_axis_unavailable", "Only a revolution turns about an axis.", "Nur ein Rotationskörper dreht sich um eine Achse."),
  ]],
  ["    sch_actions:", [
    row("fam_part", "Part", "Teil"),
    row("fam_item", "Item", "Eintrag"),
    row("fam_formula", "Formula", "Formel"),
    row("fam_value", "Value", "Wert"),
    row("fam_issue", "Issue", "Problem"),
    row("fam_section_family", "Family", "Familie"),
    row("fam_section_parameters", "Parameters", "Parameter"),
    row("fam_section_solids", "Solids", "Körper"),
    row("fam_none", "Select a family, or add one in the library, to edit it.", "Wählen Sie eine Familie oder legen Sie in der Bibliothek eine an, um sie zu bearbeiten."),
    row("fam_add_parameter", "Add a parameter", "Parameter hinzufügen"),
    row("fam_add_solid", "Add a solid", "Körper hinzufügen"),
    row("fam_hidden", "hidden", "verborgen"),
    row("fam_true", "yes", "ja"),
    row("fam_false", "no", "nein"),
    row("fam_act_kind_named", "Make it {name}", "Zu {name} machen"),
    row("fam_act_add_parameter_named", "Add {name} parameter", "Parameter {name} hinzufügen"),
    row("fam_act_add_solid_named", "Add {name} solid", "Körper {name} hinzufügen"),
    row("fam_act_axis_named", "Turn about {name}", "Um {name} drehen"),
    row("fam_cat_furniture", "Furniture", "Möbel"),
    row("fam_cat_equipment", "Equipment", "Ausstattung"),
    row("fam_cat_casework", "Casework", "Einbaumöbel"),
    row("fam_cat_plumbing", "Plumbing", "Sanitär"),
    row("fam_cat_lighting", "Lighting", "Beleuchtung"),
    row("fam_cat_mechanical", "Mechanical", "Mechanik"),
    row("fam_cat_electrical", "Electrical", "Elektro"),
    row("fam_cat_generic", "Generic", "Allgemein"),
    row("fam_cat_profile", "Profile", "Profil"),
    row("fam_kind_length", "Length", "Länge"),
    row("fam_kind_angle", "Angle", "Winkel"),
    row("fam_kind_real", "Real number", "Reelle Zahl"),
    row("fam_kind_integer", "Integer", "Ganzzahl"),
    row("fam_kind_boolean", "Yes/no", "Ja/nein"),
    row("fam_kind_text", "Text", "Text"),
    row("fam_kind_material", "Material", "Material"),
    row("fam_shape_cuboid", "Cuboid", "Quader"),
    row("fam_shape_extrusion", "Extrusion", "Extrusion"),
    row("fam_shape_revolution", "Revolution", "Rotationskörper"),
    row("fam_shape_sweep", "Sweep", "Sweep"),
  ]],
];

let source = readFileSync(file, "utf8");
const crlf = source.includes("\r\n");
if (crlf) source = source.replaceAll("\r\n", "\n");
const lines = source.split("\n");
let added = 0;
for (const [anchor, rows] of blocks) {
  const at = lines.findIndex((line) => line.startsWith(anchor));
  if (at < 0) throw new Error(`anchor ${anchor} not found`);
  const fresh = rows.filter((candidate) => !lines.some((line) => line.startsWith(candidate.slice(0, candidate.indexOf(":") + 1))));
  lines.splice(at + 1, 0, ...fresh);
  added += fresh.length;
}
const next = lines.join("\n");
writeFileSync(file, crlf ? next.replaceAll("\n", "\r\n") : next);
console.log(`terminology: ${added} rows added`);
