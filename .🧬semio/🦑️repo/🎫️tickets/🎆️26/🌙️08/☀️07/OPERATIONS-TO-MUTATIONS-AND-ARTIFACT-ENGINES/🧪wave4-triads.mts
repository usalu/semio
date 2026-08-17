#!/usr/bin/env bun
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../..");

type Triad = { art: string; plugin: string; mod: string; emojiKebab: string; proj: string; prefix: string };

function stub(t: Triad) {
  const base = join(repo, t.plugin, "🗿️artifacts", t.art, "🧬️mutations", t.emojiKebab);
  const artPath = `crate::artifacts::${t.art.replace(/.*\//, "")}`;
  const applyFn = `apply_${t.art.includes("dag") ? "dag" : t.art.includes("flow") ? "flow" : "form_edit"}_mutation`;
  mkdirSync(join(base, "🦠️mutation"), { recursive: true });
  mkdirSync(join(base, "↩️inverse"), { recursive: true });
  mkdirSync(join(base, "🔺️diff"), { recursive: true });
  writeFileSync(
    join(base, "🦠️mutation", "🦀️component.rs"),
    `use ${artPath}::${t.proj};\nuse ${artPath}::mutations::${t.prefix}Mutation;\n\npub fn apply(projection: &mut ${t.proj}, mutation: &${t.prefix}Mutation) {\n    ${artPath}::mutations::${applyFn}(projection, mutation);\n}\n`,
  );
  writeFileSync(
    join(base, "↩️inverse", "🦀️component.rs"),
    `use ${artPath}::${t.proj};\nuse ${artPath}::mutations::${t.prefix}Mutation;\n\npub fn inverse(base: &${t.proj}, mutation: &${t.prefix}Mutation) -> Vec<${t.prefix}Mutation> {\n    mutation.inverse(base)\n}\n`,
  );
  writeFileSync(join(base, "🔺️diff", "🦀️component.rs"), "//! stub diff leaf\n");
  writeFileSync(join(base, "🦠️mutation", "🟦️component.ts"), "export {};\n");
}

const dag: Triad = {
  plugin: "✏️s/🔌️plugins/🕸️dag",
  art: "🕸️dag",
  prefix: "Dag",
  proj: "DagDocument",
  mod: "",
  emojiKebab: "",
};
for (const [mod, ek] of [
  ["nodes", "🔗nodes"],
  ["edges", "➡️edges"],
  ["set_nodes", "📋set-nodes"],
  ["set_edges", "📋set-edges"],
  ["set_document", "📄set-document"],
] as const) {
  stub({ ...dag, mod, emojiKebab: ek });
}

const flow = { plugin: "✏️s/🔌️plugins/🌊️flow", art: "🌊️flow", prefix: "Flow", proj: "FlowFixture", mod: "", emojiKebab: "" };
for (const [mod, ek] of [
  ["widgets", "🧩widgets"],
  ["synapses", "🔗synapses"],
  ["set_layout", "📐set-layout"],
  ["set_fixture", "📄set-fixture"],
] as const) {
  stub({ ...flow, mod, emojiKebab: ek });
}

const forms = { plugin: "✏️s/🔌️plugins/📋️forms", art: "📋️forms", prefix: "Form", proj: "FormSpec", mod: "", emojiKebab: "" };
for (const [mod, ek] of [
  ["add_step", "➕add-step"],
  ["remove_step", "➖remove-step"],
  ["move_step", "↔️move-step"],
  ["add_block", "➕add-block"],
  ["remove_block", "➖remove-block"],
  ["move_block", "↔️move-block"],
  ["update_block", "🩹update-block"],
  ["update_step", "🩹update-step"],
  ["update_playbook", "📖update-playbook"],
] as const) {
  stub({ ...forms, mod, emojiKebab: ek });
}

console.log("triads ok");
