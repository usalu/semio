#!/usr/bin/env bun
/** 🌳️ Wires the view browser into the outliner: the `views` module, the `Views` node below each building, and the exclusion of views from the element groups of a storey (idempotent). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, RS, subset } from "./r3-f1-paths.ts";

const file = join(child(child(subset, "editor"), "panels") && child(child(child(subset, "editor"), "panels"), "outliner"), RS);
let text = readFileSync(file, "utf8");
const swap = (from: string, to: string) => {
  if (text.includes(to)) return;
  if (!text.includes(from)) throw new Error("anchor missing: " + from.slice(0, 50));
  text = text.replace(from, to);
};
swap("//#region 🔖️Rows\n", '#[path = "🖼️views/🦀️.rs"]\npub mod views;\n\n//#region 🔖️Rows\n');
swap('.filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid" | "opening"))', '.filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid" | "opening" | "view"))');
swap(`    let total = storeys.len() + grids.len();
    if total == 0 {
        return leaf(builder);
    }
    semio_framework_plugin::tree_window_indexed_item(windows, builder, id, true, total, |index| match storeys.get(index) {
        Some(storey) => storey_row(windows, snapshot, inference, labels, storey),
        None => leaf(entity_item(snapshot, grid, grids[index - storeys.len()], None)?),
    })`, `    let browser = views::views_row(windows, snapshot, labels, id);
    let total = storeys.len() + grids.len() + usize::from(browser.is_some());
    if total == 0 {
        return leaf(builder);
    }
    let mut browser = browser;
    semio_framework_plugin::tree_window_indexed_item(windows, builder, id, true, total, |index| match storeys.get(index) {
        Some(storey) => storey_row(windows, snapshot, inference, labels, storey),
        None if index < storeys.len() + grids.len() => leaf(entity_item(snapshot, grid, grids[index - storeys.len()], None)?),
        None => browser.take().unwrap_or_else(|| Err(crate::editor::bim::kit::ui_capacity_error())),
    })`);
writeFileSync(file, text);
console.log("outliner wired");
