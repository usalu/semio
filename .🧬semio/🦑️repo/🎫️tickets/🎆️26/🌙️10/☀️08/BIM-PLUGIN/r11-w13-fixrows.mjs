import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let text = readFileSync(file, "utf8");
const swap = (from, to) => { if (!text.includes(from)) throw new Error("missing " + from.slice(0, 70)); text = text.replace(from, to); };
swap('use super::kit::{column, house, items, plain, table_of, texts};', 'use super::kit::{house, items, plain, table_of, texts};');
swap(`fn ids(list: Vec<String>) -> Vec<&'static str> {
    let known = ["o-door-1", "o-door-2", "o-win-1", "o-win-2", "o-win-arc"];
    list.iter().filter_map(|id| known.iter().copied().find(|known| known == id)).collect()
}

`, '');
swap(`    let _ = column(ScheduleField::Id, false);
    let _ = ids(Vec::new());
    let _ = ModelInference::infer(&snapshot).map(|_| ()).map_err(|error| panic!("{error:?}"));
    let _: Option<&dyn Inference> = None;
`, '');
writeFileSync(file, text);
