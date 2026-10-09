#!/usr/bin/env bun
/**
 * 🗣️ Wave W1 `w11-annotations`: inserts the en+de labels of the annotation entities, fields, utilities and commands into the one `app_labels!` block of the BIM editor
 * terminology, each block directly after a stable anchor row; rows that already exist are left alone, so the script is idempotent and never rewrites the file.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { subset, child, RS } from "./r3-f1-paths.ts";

const file = join(child(child(subset, "editor"), "terminology"), RS);
const row = (field: string, en: string, de: string) => `    ${field}: ${JSON.stringify(en)}, ${JSON.stringify(de)};`;

const blocks: [anchor: string, rows: string[]][] = [
  ["    kind_view:", [
    row("kind_dimension", "Dimension", "Bemaßung"),
    row("kind_tag", "Tag", "Kennzeichnung"),
    row("kind_text_note", "Text note", "Textnotiz"),
    row("kind_leader", "Leader", "Hinweislinie"),
    row("kind_annotation_style", "Annotation style", "Beschriftungsstil"),
  ]],
  ["    group_views:", [
    row("group_dimensions", "Dimensions", "Bemaßungen"),
    row("group_tags", "Tags", "Kennzeichnungen"),
    row("group_text_notes", "Text notes", "Textnotizen"),
    row("group_leaders", "Leaders", "Hinweislinien"),
    row("group_annotation_styles", "Annotation styles", "Beschriftungsstile"),
  ]],
  ["    utility_split_wall:", [
    row("utility_dimension", "Dimension", "Bemaßen"),
    row("utility_tag", "Tag", "Kennzeichnen"),
    row("utility_text_note", "Text", "Text"),
    row("utility_leader", "Leader", "Hinweislinie"),
  ]],
  ["    cmd_arm_split_wall_describe:", [
    row("cmd_arm_dimension", "Dimension Tool", "Bemaßungswerkzeug"),
    row("cmd_arm_dimension_describe", "Arms the dimension utility in the addressed window (Shift+D): click the anchors (wall faces, ends, opening and column centres, grid lines or points; hold Shift to chain more), then click where the dimension line goes.", "Aktiviert das Bemaßungswerkzeug im adressierten Fenster (Umschalt+D): Klicken Sie die Anker (Wandflächen, -enden, Öffnungs- und Stützenmitten, Rasterlinien oder Punkte; mit Umschalt weitere anfügen), dann die Lage der Maßlinie."),
    row("cmd_arm_tag", "Tag Tool", "Kennzeichnungswerkzeug"),
    row("cmd_arm_tag_describe", "Arms the tag utility in the addressed window (Shift+T): a click on an element places a tag that reads its name, type, number or size and follows the element.", "Aktiviert das Kennzeichnungswerkzeug im adressierten Fenster (Umschalt+T): Ein Klick auf ein Bauteil setzt eine Kennzeichnung, die Name, Typ, Nummer oder Größe liest und dem Bauteil folgt."),
    row("cmd_arm_text_note", "Text Tool", "Textwerkzeug"),
    row("cmd_arm_text_note_describe", "Arms the text utility in the addressed window (Shift+N): a click places a free text note.", "Aktiviert das Textwerkzeug im adressierten Fenster (Umschalt+N): Ein Klick setzt eine freie Textnotiz."),
    row("cmd_arm_leader", "Leader Tool", "Hinweislinienwerkzeug"),
    row("cmd_arm_leader_describe", "Arms the leader utility in the addressed window (Shift+L): click the element or point to point at, then click where the text goes.", "Aktiviert das Hinweislinienwerkzeug im adressierten Fenster (Umschalt+L): Klicken Sie auf das Bauteil oder den Punkt und dann auf die Lage des Textes."),
  ]],
  ["    field_footprint:", [
    row("field_anchors", "Anchors", "Anker"),
    row("field_anchor", "Anchor", "Anker"),
    row("field_direction_angle", "Direction (rad)", "Richtung (rad)"),
    row("field_lock", "Lock (m)", "Sperre (m)"),
    row("field_style", "Annotation style", "Beschriftungsstil"),
    row("field_element", "Element", "Bauteil"),
    row("field_reads", "Reads", "Liest"),
    row("field_text", "Text", "Text"),
    row("field_text_height", "Text height (m)", "Texthöhe (m)"),
    row("field_terminator", "Line end", "Linienende"),
    row("field_unit", "Unit", "Einheit"),
    row("field_precision", "Decimals", "Nachkommastellen"),
    row("field_mark_size", "Mark size (m)", "Markengröße (m)"),
    row("field_extension_gap", "Extension gap (m)", "Hilfslinienabstand (m)"),
    row("field_extension_overshoot", "Extension overshoot (m)", "Hilfslinienüberstand (m)"),
    row("field_measured", "Measured (m)", "Gemessen (m)"),
    row("field_printed", "Printed", "Gedruckt"),
    row("field_lock_difference", "Lock difference (m)", "Abweichung von der Sperre (m)"),
  ]],
  ["    choice_net:", [
    row("choice_name", "Name", "Name"),
    row("choice_type", "Type", "Typ"),
    row("choice_number", "Number", "Nummer"),
    row("choice_size", "Size", "Größe"),
    row("choice_tick", "Tick", "Schrägstrich"),
    row("choice_arrow", "Arrow", "Pfeil"),
    row("choice_dot", "Dot", "Punkt"),
    row("choice_metre", "Metres", "Meter"),
    row("choice_centimetre", "Centimetres", "Zentimeter"),
    row("choice_millimetre", "Millimetres", "Millimeter"),
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
