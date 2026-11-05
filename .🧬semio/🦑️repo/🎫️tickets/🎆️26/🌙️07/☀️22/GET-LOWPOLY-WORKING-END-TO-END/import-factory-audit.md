# Import Factory Audit

Read-only source review, 2026-11-05. Runtime owns changes. No builds/source edits.

## Registry Seam

Plugin /📜️declaration/👷️job/🧾️bounded-proof-and-factory-registry/🦀️.rs:122 exposes register<F>(&mut self,factory:F)->Result<(),Fault> where F:ArtifactOwnedToolJobFactory<Owner=A>. Register multiple concrete factory structs sequentially with ?; both can use ArtifactRetainedCommandPayload<EditorApp<LowpolyPlayApp>> and ArtifactRetainedCommandJob of same owner/schema.

Each factory keys() must exactly equal its static TOOL_IDS and every key uses live registry.controller_id(). PUBLICATION_CONTRACTS must have exactly those IDs, nonempty lanes; HostOnly must be sole lane. Registered type ID/name derives from actual F and is retained in each registration. Duplicate tool IDs fail. Therefore remove importMeshFile from small factory key/TOOL_IDS/publications before registering dedicated import factory; retain all other IDs once.

execution_contract(&self) has no tool key and is read once per concrete F. Two factories are appropriate for distinct wire policies.

## Proof Seam

bounded_first_step_tool_proofs! in plugin /📜️declaration/🧩️app/📣️proofs/🦀️.rs expands one trait method and one optional factory_type shared by all tools. Two macro invocations duplicate the method; leaving all tools under small factory_type yields a false import proof.

Existing public manual construction is supported:
ArtifactBoundedFirstStepProof::new::<Owner>(owner_file,controller_id,factory_name,tool_id,artifact_schema,contract).with_factory_type::<Owner,ActualFactory>().

Implement a single bounded_first_step_tool_proofs() returning combined vector with exact per-row concrete factory qualification. Do not widen framework macro solely for this local split. Original new signature takes static string fields and ToolExecutionContract; with_factory_type generic bound F:ArtifactOwnedToolJobFactory<Owner=A> derives exact TypeId/type_name and asserts owner.

## Actual Admission Macro

Plugin /🧵️retained-command/🏭️factory/🦀️.rs:33 supports retained_command_factory_admission!(maximum_raw_bytes:EXPR,checkpoint_resume:BOOL) and payload-expression variant maximum_raw_bytes:|payload|EXPR. It defines admission_demands and admit_job using measured original input declared bytes and checkpoint policy. Both concrete factories must quote the same policy their execution_contract declares. Payload.max_raw_bytes and actual original command writer also need agreement, not just advertised proof.

## Boundary Review Status

At inspection, mounted tests contain cancellation/workflows only; no new payload boundary regression visible yet. Previous payload-boundary-plan.md remains exact public chunk+effect capture recipe. A future test must dispatch actual kernel import chunks, avoid context settle helpers that discard effects, assert no partial effect/document change, capture exactly one final LoadDocument, and explicitly distinguish emitted replacement from host-applied archive load. Contract checks should derive actual registered import row/factory identity rather than mirror a new limit literal.
