# Current Completion Gaps

Read-only source audit, corrected 2026-11-04. No builds, tests or native acceptance were run. Source existence is not passing verification.

## Attribution and Scope Correction

The first revision incorrectly substituted the WGPU guest compiler for the requested ActionArgDef/ArgSchema descriptor producer, and window-config press preview for norm artifact semantic admission. Those infrastructure branches are outside these two requested gaps. Their presence does not establish norm preparation; their refusal branches are not additional blockers for this goal. The findings below replace those two sections. The concrete missing norm RenderWork/request-hook finding remains.

## Actual Descriptor Producer APIs and Callers

`🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🪪️native-input/🦀️.rs:11` defines public `JsonNativeInputCursor<'source,T:NativeReceive<DslValue>+Sync>`, reexported by `json/🦀️.rs:183`. Its public lifecycle is `constructor_demand`, `admit(source:&str,limits:JsonReadLimits,control:&NativeDecodeControl,grant)`, `demands(body)`, `advance(control,body,grant)`, `take(control,grant)`, then `begin_close`, `close_demands`, `close_step(&mut dyn OriginalRetirementControl,grant)` and `terminal_is_empty`. Source lifetime and native decoding/retirement authority remain explicit. Limits include bytes, allocation bytes, depth and items; source parsing rejects duplicate members.

The descriptor-specific caller found is `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🛬️receiving/🧪️tests/native-input-cases.rs:171`, using `SealedShared<Vec<ActionArgDef>>`. The owning agent confirms this is a newly authored lifecycle test awaiting verification. No production cursor caller was found in manifest.

`🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🏗️compiler/🦀️.rs:44` exposes `compile_input_definitions(schema:&str,documents:&[&str])->InputDefinitionCompilation`. Output contains schema, supplied-document and framework-document SHA256 fingerprints and `definitions_json:Result<String,InputSchemaError>`. It compiles immutable native input JSON using `mutation_input_defs`; its docstring explicitly restricts it to the build caller. Only compiler test callers were found within manifest. This audit did not prove an installed generated descriptor catalog or production norm consumer.

Norm still uses synchronous schema preparation: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪺️inspector-forms/plan.rs:6-17` selects `A::Mutation::DESCRIPTORS`, `INPUT_SCHEMAS` and `INPUT_SCHEMA_DOCUMENTS`, then calls `mutation_input_defs`, parsing document JSON and converting it to DslValue. `inspector-forms/🦀️.rs:91` offers another synchronous insertion constructor. A real paid source-preserving descriptor producer/cache and its norm preparation caller remain to be established.

## Actual Artifact Semantic Admission APIs and Callers

`✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🎮️commands/🦀️.rs:220` exposes `resolve_norm_edits<D,M,F>`. Bounds: `D:Clone+ToValue`, `M:Mutation<D>+ColdMutation<D>+RetireOwned`, `F:FnMut(&D,&NormEdit)->Result<Vec<M>,String>`. It bounds edit count by `NORM_LIVE_MAX_WRITES`, clones the artifact into a private `ColdReplayProjection`, resolves ordered operations, and applies each through `fold_operation_into`. Domain refusal retires accumulated operations; success returns admitted operations. Production caller `resolve_field_edits` at line 258 supplies the existing field-write route.

`inspector-forms/plan.rs:71` exposes `admit_norm_inspector_args<A>`, requiring `A:NormRetainedEditor` and `A::Mutation:ColdMutation<A::Snapshot>+RetireOwned`. It constructs InsertItem or SetField, calls the actual resolver against the artifact document, and cold-retires admitted operations. No caller of this function was found anywhere under norm. The same file exposes `norm_inspector_form_plan<A:NormRetainedEditor>(document,request)`; it still synchronously projects the document with `to_value` for variant plans.

`inspector-forms/🦀️.rs:118,127` exposes `status` and `admitted_args` with an externally supplied `FnOnce(&DslValue)->Result<(),String>` admission callback. Shape coercion alone is deliberately insufficient. `body.rs:112` accepts already admitted status but synchronously builds all response fields and controls before pagination. The callback wiring and a retained paid typed semantic preparation adapter against the actual artifact document remain absent from the inspected norm production route. The existing cold helper is not itself a resumable native semantic producer.

## Concrete Norm Render Mounting Gap

No production Rust source under norm declares `type RenderWork`, `begin_render_work` or `receive_render_work_request`. Example: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:205,209` uses synchronous `render_body`, including request-context rendering.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/✏️artifact-editor-trait/🦀️.rs:373-376` defaults to unit work, disables it and rejects staged requests. The staged native transport `⚛️reactor/🔄️turn/📥️receiving/auxiliary.rs:38-39` now calls the real app request hook, renders and transfers its original tree. Thus the absent norm hook is a direct downstream blocker. Its admission-status constant at line 13 remains false and is asserted false by the corresponding test; it is not used as a runtime gate elsewhere in the inspected plugin tree.

## Remaining Acceptance Work

Complete the descriptor producer/cache consumer and paid norm artifact semantic preparation; mount concrete norm RenderWork and request ownership; then verify actual native insert/variant forms, refusal retention, readiness, commit, cancellation, stale context, mounted-tree publication and acknowledgement. Existing synchronous helpers or unrelated infrastructure are not substitutes for that end-to-end evidence.
