#!/usr/bin/env bun
/**
 * 📊️ R11 `w13-schedules`: wires the command `exportScheduleCsv` (the CSV export of a schedule as a stepped, cancellable job) into the shared files: the artifact root mount, the editor command table, bridge arm
 * and job factory, the labels, the schedule window (list row button, table measure). Idempotent; every swap is an exact string so a peer's change makes it fail loudly instead of corrupting the file.
 */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, subset } from "./r3-f1-paths.ts";

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
const sub = (...parts: string[]) => join(subset, ...parts);
const find = (parent: string, suffix: string) => readdirSync(parent).find((name) => name.endsWith(suffix))!;
const editor = sub(find(subset, "editor"));
const commands = join(editor, find(editor, "commands"));
const exported = find(commands, "export-schedule-csv");

edit(join(artifact, "🦀️.rs"), [[
  `                #[path = "."]
                pub mod set_camera {`,
  `                #[path = "."]
                pub mod export_schedule_csv {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/${exported}/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod set_camera {`]], "pub mod export_schedule_csv");

edit(join(editor, "🦀️.rs"), [
  ["delete_selection, edit_schedule, engagement_input", "delete_selection, edit_schedule, engagement_input, export_schedule_csv"],
  [`            "setCamera" as "camera"`, `            "exportScheduleCsv" as "export-schedule-csv" => export_schedule_csv::ExportScheduleCsv, [HostOnly]; View, cmd_export_schedule_csv, cmd_export_schedule_csv_describe;
            "setCamera" as "camera"`],
  [`            "setCamera" => BimCommand::SetCamera(`, `            "exportScheduleCsv" => BimCommand::ExportScheduleCsv(decode(action, only(fold(args, &[], &[("id", text(""))]), &["id", "pressed"]))?),
            "setCamera" => BimCommand::SetCamera(`],
  [`    fn decode<T: semio_framework_value::FromValue>`, `    fn only(folded: DslValue, keys: &[&str]) -> DslValue {
        match folded {
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().filter(|(key, _)| keys.contains(&key.as_str())).collect()),
            other => other,
        }
    }

    fn decode<T: semio_framework_value::FromValue>`],
  [`        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BimCommandWork::new(tool_id, request.instance_operation_owner.clone()));`,
   `        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = match &*request.command {
            BimCommand::ExportScheduleCsv(_) => Box::new(export_schedule_csv::ScheduleCsvWork::new(tool_id)),
            _ => Box::new(BimCommandWork::new(tool_id, request.instance_operation_owner.clone())),
        };`],
], "exportScheduleCsv");
