#!/usr/bin/env bun
/**
 * 📊️ R11 `w13-schedules`: the words and controls of the CSV export and of the room finish preset: the labels (en first, de second), the localized preset name, an export button on every schedule row of the
 * list and an export measure in the chrome of the schedule window while a schedule is shown. Idempotent; exact swaps.
 */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { subset } from "./r3-f1-paths.ts";

const find = (parent: string, suffix: string) => readdirSync(parent).find((name) => name.endsWith(suffix))!;
const edit = (file: string, swaps: [string, string][], marker: string) => {
  let text = readFileSync(file, "utf8");
  if (text.includes(marker)) return console.log("already wired:", file.split(/[\\/]/).slice(-3).join("/"));
  const crlf = text.includes("\r\n");
  text = text.replaceAll("\r\n", "\n");
  for (const [from, to] of swaps) {
    if (!text.includes(from)) throw new Error(`${file}: missing ${from.slice(0, 80)}`);
    text = text.replace(from, to);
  }
  writeFileSync(file, crlf ? text.replaceAll("\n", "\r\n") : text);
  console.log("wired:", file.split(/[\\/]/).slice(-3).join("/"));
};
const editor = join(subset, find(subset, "editor"));
const descend = (parent: string, ...suffixes: string[]) => suffixes.reduce((path, suffix) => join(path, find(path, suffix)), parent);
const windows = descend(editor, "modes", "edit", "windows");
const scheduleWindow = join(windows, find(windows, "schedule"));

edit(join(editor, find(editor, "terminology"), "🦀️.rs"), [
  [`    cmd_edit_schedule_describe:`, `    cmd_export_schedule_csv: "Export Schedule as CSV", "Bauteilliste als CSV exportieren";
    cmd_export_schedule_csv_describe: "Writes the table of a schedule (the one named, else the one the window shows) as an RFC 4180 CSV file and offers it as a download. The inference runs in steps with progress and can be cancelled; the file lists the rows and totals exactly as the window shows them, with stable tokens instead of localized words.", "Schreibt die Tabelle einer Bauteilliste (der genannten, sonst der im Fenster gezeigten) als CSV-Datei nach RFC 4180 und bietet sie zum Herunterladen an. Die Inferenz läuft schrittweise mit Fortschritt und lässt sich abbrechen; die Datei enthält Zeilen und Summen genau wie das Fenster, mit stabilen Kennungen statt lokalisierter Wörter.";
    cmd_edit_schedule_describe:`],
  [`    sp_wall:`, `    sp_finish: "Room finish schedule", "Raumbelagsliste";
    sp_wall:`],
  [`    sch_show: "Show", "Anzeigen";`, `    sch_show: "Show", "Anzeigen";
    sch_export_csv: "Export CSV", "CSV exportieren";`],
], "cmd_export_schedule_csv:");

edit(join(scheduleWindow, "🦀️.rs"), [
  [`        "room" => labels.sp_room,`, `        "room" => labels.sp_room,
        "finish" => labels.sp_finish,`],
  [`button("trash", &BimLabels::named(labels.act_delete_named, &schedule.name), "deleteSelection"`, `button("download", labels.sch_export_csv.as_str(), "exportScheduleCsv", object([("id", id.as_str())])), button("trash", &BimLabels::named(labels.act_delete_named, &schedule.name), "deleteSelection"`],
], `"exportScheduleCsv"`);

edit(join(editor, find(editor, "chrome"), "🦀️.rs"), [
  [`        measures.push(WindowMeasure::Toggle { id: "bim.measure.schedule.editing".into()`, `        measures.push(WindowMeasure::Toggle { id: "bim.measure.schedule.export".into(), icon_id: "download".into(), label: Some(labels.sch_export_csv.as_str().to_string()), pressed: false, text: None, on_change: bim_window_action("exportScheduleCsv", Some(DslValue::object([("id".to_string(), DslValue::String(config.schedule.clone()))]))) });
        measures.push(WindowMeasure::Toggle { id: "bim.measure.schedule.editing".into()`],
], `bim.measure.schedule.export`);
