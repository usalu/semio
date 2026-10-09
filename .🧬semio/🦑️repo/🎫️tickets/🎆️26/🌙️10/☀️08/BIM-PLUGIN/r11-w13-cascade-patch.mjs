import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let text = readFileSync(file, "utf8");
const replace = (from, to) => {
  if (!text.includes(from)) throw new Error("missing: " + from);
  text = text.replace(from, to);
};
if (!text.includes("schedules => CreateSchedule")) {
  replace("    views => CreateView(create_view, view);\n", "    views => CreateView(create_view, view);\n    schedules => CreateSchedule(create_schedule, schedule);\n");
  replace("    let on = |storey: &String| removal.storeys.contains(storey);\n    let (walls,", "    removal.schedules.extend(base.schedules.iter().filter(|(_, row)| row.storeys.iter().any(|storey| removal.storeys.contains(storey))).map(|(id, _)| id.clone()));\n    let on = |storey: &String| removal.storeys.contains(storey);\n    let (walls,");
  replace("the closure of records that leave with a set of root elements (site → buildings →\n//! storeys and grid lines → everything on a storey →", "the closure of records that leave with a set of root elements (site → buildings →\n//! storeys and grid lines → the views and schedules scoped to them → everything on a storey →");
}
writeFileSync(file, text);
