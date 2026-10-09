#!/usr/bin/env bun
/** 🪟️ Rewrites the window `setView` command for authored views: plan and section windows set their `view`, the storey, cut height, line and depth live in the view (idempotent). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, RS, subset } from "./r3-f1-paths.ts";

const commands = child(child(subset, "editor"), "commands");
const dir = child(commands, "set-view");
const file = join(dir, RS);
let text = readFileSync(file, "utf8");
const swap = (from: string, to: string) => {
  if (!text.includes(from)) return;
  text = text.replace(from, to);
};
swap("the plan's storey and cut height, the world's projection, storey\n//! isolation and visibility and section plane, the section's line and depth. It never touches the document. Choosing the working storey is also shared as presence.", "the plan and section windows' authored view, the world's projection, storey\n//! isolation and visibility and section plane. It never touches the document. Choosing a plan view shares its storey as the working storey in presence.");
swap(`        "storey" => {
            if !payload.value.is_empty() && !snapshot.storeys.contains_key(&payload.value) {
                return Err(fault("bim.view.storey-missing", format!("no storey '{}'", payload.value)));
            }
            config.storey = payload.value.clone();
            config.framed = false;
            ctx.presence_out.push(ctx.presence.on_storey(&payload.value));
        }
        "cut_height" => config.cut_height = number(payload)?.clamp(0.0, 10.0),
        other => return Err(unknown(plan::WINDOW_KIND_ID, other)),`, `        "view" => {
            let storey = snapshot.views.get(&payload.value).filter(|view| view.kind.is_plan()).and_then(|view| view.storey.clone()).ok_or_else(|| fault("bim.view.view-missing", format!("no plan view '{}'", payload.value)))?;
            config.view = payload.value.clone();
            config.framed = false;
            ctx.presence_out.push(ctx.presence.on_storey(&storey));
        }
        other => return Err(unknown(plan::WINDOW_KIND_ID, other)),`);
swap(`fn section_view(payload: &SetView, ctx: &BimDispatchCtx) -> Result<section::config::BimSectionWindowConfig, Fault> {
    let mut config = ctx.section.clone();
    match payload.field.as_str() {
        "line" => {
            let parts: Vec<f64> = payload.value.split(',').filter_map(|part| part.trim().parse::<f64>().ok()).collect();
            match parts.as_slice() {
                [start_x, start_y, end_x, end_y] => (config.start_x, config.start_y, config.end_x, config.end_y, config.framed) = (*start_x, *start_y, *end_x, *end_y, false),
                _ => return Err(fault("bim.view.value-invalid", "a section line is 'x1, y1, x2, y2'")),
            }
        }
        "depth" => config.depth = number(payload)?.max(0.0),
        other => return Err(unknown(section::WINDOW_KIND_ID, other)),`, `fn section_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<section::config::BimSectionWindowConfig, Fault> {
    let mut config = ctx.section.clone();
    match payload.field.as_str() {
        "view" => {
            if !snapshot.views.get(&payload.value).is_some_and(|view| view.kind.is_vertical()) {
                return Err(fault("bim.view.view-missing", format!("no section or elevation view '{}'", payload.value)));
            }
            config.view = payload.value.clone();
            config.framed = false;
        }
        other => return Err(unknown(section::WINDOW_KIND_ID, other)),`);
swap("section::config::addressed(&view, section_view(payload, ctx)?)?", "section::config::addressed(&view, section_view(payload, doc.snapshot, ctx)?)?");
writeFileSync(file, text);
console.log("set-view command updated");
