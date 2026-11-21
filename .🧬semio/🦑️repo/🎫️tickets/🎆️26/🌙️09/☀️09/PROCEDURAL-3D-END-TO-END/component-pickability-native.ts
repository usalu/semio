import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const world = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world";
const generated = join(import.meta.dirname, "🗑️generated", "component-pickability");
const inputs = ["🗃️model/🗂️registries/🦀️.rs", "🌉️bridge/🎞️scene-bridge/🦀️.rs", "🧲️cursors/🎯️ray-and-component-pick/🦀️.rs", "🧲️cursors/🔲️marquee/🦀️.rs"];
const [model, bridge, ray, marquee] = await Promise.all(inputs.map(path => readFile(join(world, path), "utf8")));
const { instances, highlightedInput, editable, retained } = await Bun.file(join(import.meta.dirname, "component-pickability.json")).json();
const predicate = model.match(/    pub fn component_pickable\([^]*?^    }/m)?.[0];
const admission = bridge.split("\n").find(line => line.includes("interaction_state.component_unpickable_instance_ids ="));
for (const cursor of [ray, marquee]) {
  if (!cursor.includes("interaction_state.component_pickable(entry.id.as_str(), selection_state.active_object_id.as_deref())")) throw Error("Native picker does not consume the production instance predicate");
}
if (!predicate || !admission) throw Error("Native scene admission is absent");
const rustString = JSON.stringify;
const rows = instances.map(row => "Instance { id: " + rustString(row.id) + ".into(), component_pickable: " + (row.componentPickable === false ? "Some(false)" : "None") + " }");
await mkdir(generated, { recursive: true });
const source = join(generated, "actual.rs");
const binary = join(generated, process.platform === "win32" ? "probe.exe" : "probe");
await writeFile(source, [
  "use std::collections::HashSet;",
  "struct Instance { id: String, component_pickable: Option<bool> }",
  "struct Cursor { instances: Vec<Instance> }",
  "struct WorldInteractionState { component_unpickable_instance_ids: HashSet<String> }",
  "impl WorldInteractionState {", predicate, "}",
  "#[test] fn rejects_display_only_rows_even_when_they_are_the_active_object() {",
  "let cursor = Cursor { instances: vec![" + rows.join(",\n") + "] };",
  "let mut interaction_state = WorldInteractionState { component_unpickable_instance_ids: HashSet::new() };",
  admission,
  "let (input, live, stale) = (" + [highlightedInput, editable, retained].map(value => rustString(value)).join(", ") + ");",
  "assert_eq!([stale, live].into_iter().find(|id| interaction_state.component_pickable(id, None)), Some(live));",
  "assert!(!interaction_state.component_pickable(input, None));",
  "assert!(!interaction_state.component_pickable(stale, Some(stale)));",
  "assert!(!interaction_state.component_pickable(live, Some(stale)));",
  "assert!(interaction_state.component_pickable(live, Some(live)));",
  "}",
].join("\n"));
for (const command of [
  ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024", "--test", source, "-o", binary],
  [binary],
]) {
  const result = await Bun.spawn(command, { stdout: "inherit", stderr: "inherit" }).exited;
  if (result) process.exit(result);
}
