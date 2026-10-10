#!/usr/bin/env bun
/**
 * 🗣️ Package `w2-wp16-coordination`: inserts the en+de labels of the clash, rule and issue editor (entity kinds and groups, fields, pickers, the three panels, their rows, the four commands, their arguments
 * and the refusals) into the one `app_labels!` block of the BIM editor terminology, each block directly after a stable anchor row; rows that already exist are left alone, so the script is idempotent and
 * never rewrites a row.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { subset, child, RS } from "./r3-f1-paths.ts";

const file = join(child(child(subset, "editor"), "terminology"), RS);
const row = (field: string, en: string, de: string) => `    ${field}: ${JSON.stringify(en)}, ${JSON.stringify(de)};`;

const blocks: [anchor: string, rows: string[]][] = [
  ["    export_csv:", [
    row("export_bcf", "BCF issues", "BCF-Hinweise"),
  ]],
  ["    panel_library:", [
    row("panel_clashes", "Clashes", "Kollisionen"),
    row("panel_rules", "Rules", "Regeln"),
    row("panel_issues", "Issues", "Hinweise"),
  ]],
  ["    kind_wall_sweep:", [
    row("kind_clash_set", "Clash set", "Kollisionssatz"),
    row("kind_rule", "Rule", "Regel"),
    row("kind_issue", "Issue", "Hinweis"),
    row("kind_issue_comment", "Comment", "Kommentar"),
  ]],
  ["    group_wall_sweeps:", [
    row("group_clash_sets", "Clash sets", "Kollisionssätze"),
    row("group_rules", "Rules", "Regeln"),
    row("group_issues", "Issues", "Hinweise"),
    row("group_issue_comments", "Comments", "Kommentare"),
  ]],
  ["    field_curtain_wall_type:", [
    row("field_side_a", "Side A", "Seite A"),
    row("field_side_b", "Side B", "Seite B"),
    row("field_tolerance", "Tolerance (m)", "Toleranz (m)"),
    row("field_clearance", "Clearance (m)", "Abstand (m)"),
    row("field_check", "Check", "Prüfung"),
    row("field_limit", "Limit", "Grenzwert"),
    row("field_severity", "Severity", "Schwere"),
    row("field_scope", "Scope", "Geltungsbereich"),
    row("field_title", "Title", "Titel"),
    row("field_status", "Status", "Status"),
    row("field_priority", "Priority", "Priorität"),
    row("field_assignee", "Assignee", "Zuständig"),
    row("field_created", "Raised", "Erfasst"),
    row("field_issue_labels", "Labels", "Stichwörter"),
    row("field_issue_elements", "Elements", "Bauteile"),
    row("field_issue_clash", "Clash", "Kollision"),
    row("field_issue_viewpoint", "Viewpoint", "Blickpunkt"),
    row("field_issue", "Issue", "Hinweis"),
    row("field_written", "Written", "Geschrieben"),
    row("field_hard", "Hard clashes", "Harte Kollisionen"),
    row("field_soft", "Soft clashes", "Weiche Kollisionen"),
    row("field_tested", "Pairs tested", "Geprüfte Paare"),
    row("field_checked", "Members checked", "Geprüfte Bauteile"),
    row("field_violations", "Violations", "Verstöße"),
    row("rule_min_clear_height", "Minimum clear height", "Mindest-Raumhöhe"),
    row("rule_max_riser", "Maximum riser height", "Maximale Steigungshöhe"),
    row("rule_min_tread", "Minimum tread depth", "Mindest-Auftrittstiefe"),
    row("rule_min_stair_width", "Minimum stair width", "Mindest-Treppenbreite"),
    row("rule_min_door_width", "Minimum door width", "Mindest-Türbreite"),
    row("rule_max_ramp_slope", "Maximum ramp slope", "Maximale Rampenneigung"),
    row("rule_min_corridor_width", "Minimum corridor width", "Mindest-Flurbreite"),
    row("rule_max_compartment_area", "Maximum compartment area", "Maximale Brandabschnittsfläche"),
    row("status_open", "Open", "Offen"),
    row("status_in_progress", "In progress", "In Arbeit"),
    row("status_resolved", "Resolved", "Gelöst"),
    row("status_closed", "Closed", "Abgeschlossen"),
    row("priority_low", "Low", "Niedrig"),
    row("priority_normal", "Normal", "Normal"),
    row("priority_high", "High", "Hoch"),
    row("priority_critical", "Critical", "Kritisch"),
  ]],
  ["    diag_empty:", [
    row("clash_surface_describe", "Clash sets with the clashes they find, grouped by the element that takes part in most of them. Open a clash to select its pair, zoom to it, isolate it or raise an issue; the checks run as a job with progress and cancel.", "Kollisionssätze mit den gefundenen Kollisionen, gruppiert nach dem Bauteil mit den meisten. Öffnen Sie eine Kollision, um ihr Paar auszuwählen, darauf zu zoomen, es freizustellen oder einen Hinweis zu erstellen; die Prüfung läuft als Auftrag mit Fortschritt und Abbruch."),
    row("rules_surface_describe", "Rules with the members that break them and the measured value against the limit. Activating a member selects it in the plan, the 3D view and the outliner.", "Regeln mit den Bauteilen, die sie verletzen, und dem gemessenen Wert gegen den Grenzwert. Ein Bauteil auszuwählen markiert es im Grundriss, in der 3D-Ansicht und in der Struktur."),
    row("issues_surface_describe", "Issues by status with their comments. Open an issue to select its elements, restore or capture its viewpoint or add a comment; export writes all issues as a BCF file.", "Hinweise nach Status mit ihren Kommentaren. Öffnen Sie einen Hinweis, um seine Bauteile auszuwählen, seinen Blickpunkt wiederherzustellen oder zu übernehmen oder einen Kommentar hinzuzufügen; der Export schreibt alle Hinweise in eine BCF-Datei."),
    row("clash_run", "Run clash detection", "Kollisionsprüfung starten"),
    row("clash_empty", "No clash set", "Kein Kollisionssatz"),
    row("clash_none", "No clashes", "Keine Kollisionen"),
    row("clash_hard", "hard", "hart"),
    row("clash_soft", "soft", "weich"),
    row("clash_tested", "pairs tested", "Paare geprüft"),
    row("clash_select", "Select the pair", "Paar auswählen"),
    row("clash_zoom", "Zoom to the clash", "Zur Kollision zoomen"),
    row("clash_isolate", "Isolate the clash", "Kollision freistellen"),
    row("clash_raise", "Raise an issue", "Hinweis erstellen"),
    row("clash_show_all", "Show everything again", "Wieder alles anzeigen"),
    row("rules_run", "Check the rules", "Regeln prüfen"),
    row("rules_empty", "No rule", "Keine Regel"),
    row("rules_ok", "All members keep this rule", "Alle Bauteile halten diese Regel ein"),
    row("rule_above", "above", "über"),
    row("rule_below", "below", "unter"),
    row("issue_unassigned", "unassigned", "nicht zugewiesen"),
    row("issue_select", "Select the elements", "Bauteile auswählen"),
    row("issue_restore", "Restore the viewpoint", "Blickpunkt wiederherstellen"),
    row("issue_capture", "Capture the viewpoint", "Blickpunkt übernehmen"),
    row("issue_comment_add", "Add a comment", "Kommentar hinzufügen"),
    row("issue_new_comment", "New comment", "Neuer Kommentar"),
    row("issue_export", "Export BCF", "BCF exportieren"),
    row("issue_new", "New issue", "Neuer Hinweis"),
    row("issue_new_title", "New issue", "Neuer Hinweis"),
    row("issue_empty", "No issue", "Kein Hinweis"),
  ]],
  ["    cmd_select_findings:", [
    row("cmd_view_clash", "View Clash", "Kollision ansehen"),
    row("cmd_view_clash_describe", "Frames the pair of a clash in the 3D window and shows only that pair with a section box around it; the mode clear shows everything again.", "Rahmt das Paar einer Kollision im 3D-Fenster ein und zeigt nur dieses Paar mit einer Schnittbox darum; der Modus clear zeigt wieder alles."),
    row("cmd_raise_issue", "Raise Issue", "Hinweis erstellen"),
    row("cmd_raise_issue_describe", "Writes an issue for a clash: the two elements, the clash, and a viewpoint with the camera, a section box and the isolated pair.", "Schreibt einen Hinweis zu einer Kollision: die beiden Bauteile, die Kollision und einen Blickpunkt mit Kamera, Schnittbox und freigestelltem Paar."),
    row("cmd_capture_viewpoint", "Capture Viewpoint", "Blickpunkt übernehmen"),
    row("cmd_capture_viewpoint_describe", "Stores the camera, the section box and the isolated elements of the 3D window in an issue.", "Speichert Kamera, Schnittbox und freigestellte Bauteile des 3D-Fensters in einem Hinweis."),
    row("cmd_restore_viewpoint", "Restore Viewpoint", "Blickpunkt wiederherstellen"),
    row("cmd_restore_viewpoint_describe", "Puts the stored viewpoint of an issue back into the 3D window.", "Setzt den gespeicherten Blickpunkt eines Hinweises im 3D-Fenster wieder ein."),
  ]],
  ["    arg_entities:", [
    row("arg_first", "First element", "Erstes Bauteil"),
    row("arg_second", "Second element", "Zweites Bauteil"),
    row("arg_clash_mode", "View", "Ansicht"),
    row("arg_clash_set", "Clash set", "Kollisionssatz"),
    row("arg_issue", "Issue", "Hinweis"),
  ]],
  ["    fault_template_value_invalid:", [
    row("fault_clash_window_required", "A clash view belongs to one open 3D window.", "Eine Kollisionsansicht gehört zu einem geöffneten 3D-Fenster."),
    row("fault_clash_world_required", "Only a 3D window shows clashes.", "Nur ein 3D-Fenster zeigt Kollisionen."),
    row("fault_clash_mode_unknown", "This clash view does not exist (zoom, isolate, both or clear).", "Diese Kollisionsansicht gibt es nicht (zoom, isolate, both oder clear)."),
    row("fault_clash_missing", "The model has no such clash.", "Das Modell hat keine solche Kollision."),
    row("fault_clash_inference", "The clashes could not be inferred.", "Die Kollisionen konnten nicht abgeleitet werden."),
    row("fault_issue_missing", "The issue does not exist.", "Den Hinweis gibt es nicht."),
    row("fault_issue_viewpoint_missing", "The issue has no viewpoint.", "Der Hinweis hat keinen Blickpunkt."),
    row("fault_create_issue_missing", "Create an issue first.", "Legen Sie zuerst einen Hinweis an."),
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
