import { readFileSync, writeFileSync } from "node:fs";
let t = readFileSync("r11-w13-csv-wire.mjs", "utf8");
const a = t.indexOf('  [`            "setCamera" => BimCommand::SetCamera(`');
const b = t.indexOf('  [`        let work');
t = t.slice(0, a) + '  [`            "setCamera" => BimCommand::SetCamera(`, `            "exportScheduleCsv" => BimCommand::ExportScheduleCsv(decode(action, only(fold(args, &[], &[("id", text("")), ("pressed", DslValue::Null)]), &["id", "pressed"]))?),\n            "setCamera" => BimCommand::SetCamera(`],\n  [`    fn decode<T: semio_framework_value::FromValue>`, `    fn only(folded: DslValue, keys: &[&str]) -> DslValue {\n        match folded {\n            DslValue::Object(entries) => DslValue::Object(entries.into_iter().filter(|(key, _)| keys.contains(&key.as_str())).collect()),\n            other => other,\n        }\n    }\n\n    fn decode<T: semio_framework_value::FromValue>`],\n' + t.slice(b);
writeFileSync("r11-w13-csv-wire.mjs", t);
const p = "/c/git/semio/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📊️export-schedule-csv/🦀️.rs";
