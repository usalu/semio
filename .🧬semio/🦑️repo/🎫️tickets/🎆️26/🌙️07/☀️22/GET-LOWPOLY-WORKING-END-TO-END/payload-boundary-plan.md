# Payload Boundary Plan

Read-only design exploration, 2026-11-05. No native test run, source change or limit increase.

## Neutral Regression

Use a valid single-triangle OBJ plus ASCII comment lines to obtain a declared payload extent above16KiB and at least two actual kernel chunks, below2MiB string limit. Derive chunk boundaries from kernel import_payload_chunks rather than duplicate constants. Fixture should describe name, base geometry, padding generator/count, wire form (raw or data URL), expected object/face counts, no partial effect, one final load, unchanged source document/history. Count UTF-8 encoded argument/typed wire separately from decoded file bytes; base64 increases payload extent.

Prefer both raw and data URL cases because picker explicitly requests dataUrl. A test-only independent JSON/base64 parser or format oracle should validate fixture data without depending on production writer. Current implementation should first fail at the public typed admission boundary for oversized full wire; do not represent the planned passing assertions as current evidence.

## Exact Mounted APIs

Construct context::app_with_registry().await (same mounted root helper). Snapshot/history before import. For each kernel::import_payload_chunks(payload) result, build kernel::import_chunk_arguments(name,&chunk,None), then call:

artifact_app_laws::dispatch_fixture_action(&mut app.0, importMeshFile, Some(args), &context::action_meta(), &mut artifact_app_laws::fixture_identity(), app.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await

This is existing public string-action admission with genuine issuer handling. It reaches import staging and final typed production. Do not call context::act/context::dispatch/context::settle: their settle loops discard every effect.

After each nonfinal chunk, assert document/history unchanged, no typed effect, and no premature completed import. Staged receipt is not a file import. After final chunk, custom bounded pump uses maintenance_step(cx,INTERACTIVE_COMPONENT.maintenance) and advance_typed_operation_publication(&mut fixture_identity(),cx).await. Capture EVERY take_typed_operation_effect()->Option<Effect> into a Vec; drain result pages after recording lanes and acknowledge exact tokens, then event/UI/completion queues. Fail on Fault pages. Loop while has_pending_typed_operations with deadline, avoiding dropping the only LoadDocument.

Assert exactly one captured Effect::LoadDocument, decode its pack using LowpolySnapshot::decode_pack, and inspect expected object/face geometry. Source app snapshot remains unchanged because host-only import emits replacement rather than a mutation. The harness does not apply host effects. Compare history count only with its actual narrow meaning (context::committed_edits counts applied mutation rows).

Production React applyLoadDocumentEffect (ShellHost/⚡️host-effects/📥️document-load-effects/🟦️.ts) coerces pack+spr and calls stepped loadDocumentPair on plugin handle, with cancellation/progress and history refresh. A test wishing to prove actual replacement must drive registered archive-load fixture APIs too; merely decoding emitted pack proves effect contents and no partial publication.

Framework existing capture precedent: plugin /🧪️tests/🔬️tool-run-part-1/🔌️integration-seams/🦀️.rs:254-259 collects take_typed_operation_effect into Vec and matches exact sequence. PluginApp signature in reactor/app:206 is Option<Effect>, no wrapper.

## Per Tool Contract Seam

ToolJobFactory::execution_contract(&self)->ToolExecutionContract has no key parameter (action-bus:421). LowpolyCommandJobFactory currently owns all migrated keys, one lowpoly_contract, retained_command_factory_admission maximum_raw_bytes16KiB, static TOOL_IDS all migrated IDs and static publication contracts. Play-app separately lists command-specific advertised contracts.

Clean existing seam is multiple concrete factories with disjoint key sets and static TOOL_IDS/publication contracts, registered through ArtifactToolFactoryRegistry. A dedicated import factory can own the import payload contract without widening mesh edits and unrelated tools. Ensure key/schema/owner/factory proof identity, factory actual maximum, original command producer wire maximum and authored per-command declaration all agree. Avoid pretending execution_contract can dispatch per key or retaining duplicate registration of import in the generic factory. No chosen new numeric limit is implied by this plan.

Public mounted regression should precede any design choice; separate bounded file source lane may be appropriate but is not present in current inline String schema.
