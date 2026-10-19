import { readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const fam = (...d: string[]) => d.map((x) => readSourceFamilySource(`${P}/${x}`)).join("");
const instance = fam("🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job");
console.log("instance bytes", instance.length);
const anchors = [
 "impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A>",
 "struct TypedCommandFullOperationJobFactory",
 "async fn dispatch_typed_command_inner",
 "self.require_complete_tool_operation_pipeline(&admission)?",
 "self.refresh_cache().await",
 "let command = Box::new(A::command_from_action(action, source_args).await?)",
 "let command = A::command_from_action(command_id, Some(args)).await",
 "self.dispatch_typed_command_inner(",
 "async fn handle_intent_frame(&mut self, intent: &UiIntent",
 "async fn resume_task_emit",
 "pub fn dispatch_typed",
 "async fn admit_command_json_with_proof(&self",
 "pub async fn new(app: A, actor",
 ".begin_exact_wire",
 "semio_framework_pack_json::to_json_string(&(verb, wire_args))",
 "finish_admitted_prefix",
 "dispatch_wire_retained_with_spec",
 "retained_wire.take()",
 "fn publish_mounted_typed_operation_unit",
 "fn require_tool_operation_authority",
 "pending_artifact_publication","begin_apply_batch","advance_apply_batch","ArtifactStoreOneItemAdvance::Published","pending_publication_outcome.pending()",
 "typed-operation pending publication rejected a stale immutable document root",
 "\n            if let Some(pending) = mounted.pending_artifact_publication.as_mut()",
 "pending.begin_close()","self.handoff_mounted_durable_publication(mounted)?",
 "typed_effect_outbox.push","typed_event_outbox.push","typed_ui_outbox.push",
 "mounted.publication_attempt = mounted.publication_attempt.saturating_add(1)",
 "take_typed_operation_effect","take_typed_operation_event","take_typed_operation_ui_scope",
 "pub(crate) async fn dispatch_action","async fn dispatch_command(","async fn fill_agent_revisions(",
 "A::host_configuration_mutation(action, args)?","self.admit_host_configuration_json(action, args).await?","self.require_tool_operation_authority(&admission)?","Emit::config(vec![config_mutation])",
 "A::host_configuration_mutation(command_id, Some(&args))?","self.admit_host_configuration_json(command_id, Some(&args)).await?",
 "fn qualified_host_configuration_tool_proof","async fn admit_command_wire_with_proof","QualifiedToolProof::Bounded(proof.clone())",
];
for (const a of anchors) console.log(instance.includes(a) ? "ok  " : "MISS", JSON.stringify(a));
