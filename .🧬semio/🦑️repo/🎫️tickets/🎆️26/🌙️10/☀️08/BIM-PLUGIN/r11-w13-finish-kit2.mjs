import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let text = readFileSync(file, "utf8");
const swap = (from, to) => { if (!text.includes(from)) throw new Error("missing " + from.slice(0, 70)); text = text.replace(from, to); };
if (!text.includes("column(Surface, false), column(Material, false)")) {
  swap('"finish" => schedule(ScheduleCategory::Finish, vec![column(Material, false), column(Surface, false), column(FinishArea, true)], vec![ascending(Material), ascending(Surface)], Vec::new(), vec![group(Material)], false),', '"finish" => schedule(ScheduleCategory::Finish, vec![column(Number, false), column(Name, false), column(Surface, false), column(Material, false), column(FinishArea, true)], vec![ascending(Number), ascending(Surface)], Vec::new(), vec![group(Storey)], true),');
  swap("`finish` the finished surfaces of the rooms collapsed per material with the finish areas summed,", "`finish` the floor, wall and ceiling finish of every room grouped by storey with the finish areas summed,");
}
writeFileSync(file, text);
