import { readFileSync, writeFileSync } from "node:fs";
const [inference, mount, snapshot, unit, graph, table] = process.argv.slice(2);
const edit = (file, swaps) => {
  let text = readFileSync(file, "utf8");
  for (const [from, to] of swaps) {
    if (!text.includes(from)) throw new Error(file.slice(-40) + " missing " + from.slice(0, 60));
    text = text.replace(from, to);
  }
  writeFileSync(file, text);
};
let text = readFileSync(inference, "utf8");
const start = text.indexOf("/// 🧾️ The table the third-party oracle reproduces");
const end = text.indexOf("/// 🔑️ What the table of schedule");
if (start < 0 || end < 0) throw new Error("table_json block not found");
writeFileSync(inference, text.slice(0, start) + text.slice(end));
edit(mount, [['#[path = "🖼️view-linework/🦀️.rs"]\npub mod view_linework;', '#[path = "🖼️view-linework/🦀️.rs"]\npub mod view_linework;\n#[path = "📋️schedules/🦀️.rs"]\npub mod schedules;']]);
edit(snapshot, [["schema::inferences::schedules::table_json(&inferred.schedules)", "io::text::inferences::schedules::table_json(&inferred.schedules)"]]);
for (const file of [unit, graph, table]) {
  const body = readFileSync(file, "utf8");
  if (!body.includes("table_json")) continue;
  writeFileSync(file, body.replace(/^use super::\*;\n/m, "use super::*;\nuse crate::standards::v1::subsets::any::io::text::inferences::schedules::table_json;\n"));
}
