# Cancellation Metadata Audit

Read-only source audit, 2026-11-05. No production change/native/server.

CapabilityExecution at framework manifest /🕹️actions/🎯️audience-and-execution/🦀️.rs:79 derives Default; cancellable bool:95 defaults false. AppBuilder action_interactive_job changes only interactive_job, and interactive_jobs similarly sets classification only. It does not derive cancellation from retained contract.

No action_cancellable, with_cancellable or cancellable=true setter found by targeted tracked-source framework/Raster search. AppBuilder action_with can accept a fully specified ActionDefinition whose semantics.execution.cancellable is authored manually; no dedicated convenience API found. Lowpoly/Raster inspected manifests do not author this bool.

Actual ToolExecutionContract::resumable in action-bus:274-285 sets ToolCancellationPolicy::PerOperation. Mounted typed-operation admission requires that policy and retains cancellation lease. Progress snapshot cancelling checks lease token; shared controls emit cancel bound to exact operation/generation, disabled during cancellation. Handler authority is exact identity+actor+instance+nonterminal+lease. Descriptor bool is absent from these predicates (see cancellation-audit.md).

Therefore exportMesh/paintFill descriptor migrated+cancellablefalse is genuine default public metadata that is not synchronized with actual retained capability, not proof that UI cancel transport is disabled. It may mislead a consumer reading capability declarations, but no current inspected consumer gate based on this field was established. Treat as a separate declaration/projection design issue; do not change byte/control behavior or claim runtime failure from this field. A clean future contract projection should avoid hand-maintained contradictory flags, but this audit makes no infrastructure edits.

## Smallest Typed Assembly Join

Use one shared definition-stamping helper at the existing typed surface assembly boundary, alongside stamp_fault_notices and mount_natural_file_actions:
- PluginBuilder::editor_app and viewer in plugin /🏗️builder/🦀️.rs (before def.clone into App/AppFactory).
- editor_surface/viewer_surface in plugin /🧵️instance/🛬️lifted/🌳️tree/🦀️.rs (before SurfaceDeclaration).
These are two authoring pathways to the same manifest; both know concrete EditorApp<E>/ViewerApp<V> and can access owner proof contracts. A single shared helper avoids four manual per-app flag lists.

For each declared app/window action (and command if same semantic projection is needed), derive cancellable only from its exact owner/controller/schema/tool proof contract cancellation == PerOperation, preserving all other semantics. Explicitly cover migrated retained tools, including import factory split. No match must not become true from blanket migrated classification. Generic bounded-first-step policy must use its actual cancellation enum.

Do not put mutation in with_factory_type: it receives no action definition and provides owner/type witness only. Final untyped PluginManifest assembly is too late to recover concrete proof source reliably. Proof stamp is presentation, not execution authority.

Existing authoritative mount join remains validate_tool_job_rows in plugin /👷️job/🏛️reserved/🦀️.rs:901 onward: exact registered compares factory TypeId/name, owner/controller/tool, contract equality and bus admission. Stamp should retain/add consistency validation against that join, not replace it with trusting strings.

## Narrow Test Expectations

One neutral matrix with PerOperation and actual noncancellable policy, app-level and window-level action, unrelated untouched action, exact editor/viewer typed assembly paths, and missing/duplicate/wrong-owner proof refusal at existing catalog validation. Assert exported definition bool agrees with contract; independent JSON/Ajv checks final emitted descriptor. Lowpoly exportMesh and paintFill should project true only because their actual resumable contracts are PerOperation; execution cancellation tests remain separate. Preserve other semantics (approval, destructive, preview) unchanged. No unrelated new scheduling/API surface needed.

## Bounded OpBinary Interfaces

Domain-neutral replication OpBinary currently offers monolithic encode_op(&self)->Result<Vec<u8>,ProtocolError> and decode_op only. OpBinaryReceiving adds bounded decode cursors, not encode. Hence default OpBinaryProducer ENCODE remains one monolithic encode/allocation even when later wire pages are retained.

Existing domain-neutral components can support an eventual bounded encoder: OperationByteOutput and OperationByteLimitedOutput with NativeEncodeControl; OS dsl variants_binary::encode_op_into accepts BorrowedProjectedPackOperation<T>, EncodeOptions, output trait and native control. BorrowedProjectedPackCursor/ArtifactPreparedOperationCursor already offer per-step grants and same-source traversal. Those are available seams, not a universal bounded OpBinary encoding trait today. A clean future producer can opt into an exact borrowed command source/cursor; do not claim simply raising selector max makes monolithic encoding interaction-friendly. No implementation made.

## Concrete Proposed Corpus And Scope Boundary

Added ticket inputs cancellation-projection.json and cancellation-projection.schema.json (strict additionalProperties:false). Independent Ajv validation passed8cases; implementationTested:false. These are design vectors only, not production regression evidence.

Correction to earlier generic test proposal: current ToolCancellationPolicy has ONLY PerOperation (action-bus:262). There is no noncancellable policy variant to test; do not invent one or expand enum scope. Cases cover exact app/window projection true, unrelated authored flag preserved, and existing wrong-owner/wrong-schema/duplicate/missing-migrated catalog refusal. Contract comparison symbol is semio_framework::ToolCancellationPolicy::PerOperation; boolean target is action.semantics.execution.cancellable. Shared stamping helper belongs next to semio_framework_plugin_decl_registry::declarations::stamp_fault_notices; bounded proof source is ArtifactApp::bounded_first_step_tool_proofs and proof.contract.cancellation.

Public assembly regression proposals: existing plugin /🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6/🏭️tool-factory-keys-and-shared-actions/🦀️.rs for exact factories and reserved semantics, plus /🧪️tests/🪟️surface-fixture/🦀️.rs or typed surface carrier tests for both PluginBuilder and editor_surface/viewer_surface paths. Keep mismatch/duplicate rejection in existing job catalog tests, not create parallel authority rules. No blanket migrated=>true stamp.

Risk ownership: monolithic default OpBinaryProducer is owned by plugin declaration /👷️job/🎮️command/🧬️op-binary; bounded operation output primitive is replication binary operation_bytes; borrowed record Pack cursor is framework Pack; ArtifactPreparedOperationCursor is OS store operation-wire. Universal OpBinary remains Vec-returning, so any bounded producer migration must deliberately choose exact borrowed source, not claim existing trait already enforces step time. This is design evidence only and no further framework work is authorized here.
