// 🧪️ Probes the library use-pruner on the statements of the sequence editor root.
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { useLeafNames, prunedUse } = await import(`${L}/🌳️module-tree/🟦️.ts`);
for (const s of [
  "use semio_framework_plugin::app::{ChildEmit,ChildEmitPreparation};",
  "use {semio_framework_artifact_reference::Dialect};",
  "use semio_framework_pack_json::{self as json, json};",
  "use infinite_board_port_directed_dag as dag;",
  "use dag::{would_create_cycle, DagHost};",
  "use crate::editor::sequence::commands::step::{add_step, add_step_dropped, move_step};",
  "use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};",
  "use semio_framework_pack_json::Value;",
  "use semio_framework_os_infinite::board::schema::layout::{DagLayoutOptions};",
]) console.log(JSON.stringify(useLeafNames(s)), "|", prunedUse(s, useLeafNames(s).slice(0, 1)));
