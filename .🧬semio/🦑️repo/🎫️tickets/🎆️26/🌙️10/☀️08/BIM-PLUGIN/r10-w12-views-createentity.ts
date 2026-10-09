#!/usr/bin/env bun
/** 🏗️ A new storey comes with its plan view (idempotent): `createEntity` of kind storey emits `create-storey` and `create-view` in one undoable step. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, RS, subset } from "./r3-f1-paths.ts";
const file = join(child(child(child(subset, "editor"), "commands"), "create-entity"), RS);
let text = readFileSync(file, "utf8");
const swap = (from: string, to: string) => { if (text.includes(to)) return; if (!text.includes(from)) throw new Error("anchor missing: " + from.slice(0, 50)); text = text.replace(from, to); };
swap(`    let domain = if row.library { BIM_LIBRARY_DOMAIN } else { BIM_ELEMENT_DOMAIN };
    let mut emit = Emit::mutations(vec![mutation]);`, `    let domain = if row.library { BIM_LIBRARY_DOMAIN } else { BIM_ELEMENT_DOMAIN };
    let mut mutations = vec![mutation];
    if let Some(ModelMutation::CreateStorey(created)) = mutations.first().cloned() {
        let view_id = IdMint::new(doc.operation_optional()).mint("view", |view_id| view_id == id || id_taken(snapshot, view_id));
        let title = ctx.labels().map_or_else(|| format!("Plan {}", created.storey.name), |labels| BimLabels::named(labels.view_plan_of, &created.storey.name));
        let view = View::of_storey(&created.storey.building, &unique_name(snapshot, &created.storey.building, &title), ViewKind::Plan, &id);
        mutations.push(ModelMutation::CreateView(CreateView { id: view_id, view }));
    }
    let mut emit = Emit::mutations(mutations);`);
swap("use crate::editor::bim::entities::{id_taken, kind_of, ordered_storeys, EntityKind, ENTITIES};", "use crate::editor::bim::entities::views::unique_name;\nuse crate::editor::bim::entities::{id_taken, kind_of, ordered_storeys, EntityKind, ENTITIES};");
swap("use crate::{ModelMutation, ModelSnapshot};", "use crate::mutations::create_view::CreateView;\nuse crate::{ModelMutation, ModelSnapshot, View, ViewKind};");
writeFileSync(file, text);
console.log("create-entity updated");
