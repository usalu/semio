# Lane geom result
Crates: semio-framework-3d-brep-representation, 3d-mesh-document, surface-node-graph, semio-os-infinite-board-schema, tool-typing-edit.
Fixes: owner.step(body,effect) with body = grant.maximum_copy_bytes (mesh-document tessellation/projection, brep topology/compaction x3, surface node-graph scene, tool-typing-edit close_step). infinite-board-schema dag-input-cursor: SharedOwner::admit_from on Option slots (work precheck, restore source on denial), UpdateCursor::advance_native(body,effect) with native_demands/native_work_demand quotes; normal_step_demands(source,body).
Native: W-geom-2 rc 0 (all five).
Rerun: chk.sh W-geom-<n> 🌎️hub/Cargo.toml native -p <crate>
Wasm: W-geom-3 rc 0 (all five).
Tests: framework manifest, W-geom-6 blocked upstream: semio-framework-replication-mutation-messages (retirement lib line 180) cannot find semio_framework_value::RetirementHeaderRelease (repl lane). infinite-board-schema is not in the framework workspace (W-geom-5); its tests untried.
Rerun tests: tst.sh W-geom-<n> 🧰️framework/Cargo.toml native -p <crate>
STATUS: FIXED-verified W-geom-2 (native), W-geom-3 (wasm); tests BLOCKED upstream

## Round 2
- semio-framework-3d-brep-tessellation: into_mesh cold drain passes the quoted body to owner.step(body,effect); native W-geom-13, wasm W-geom-14 rc 0.
- semio-framework-3d: NOT done. 89 errors, all in mesh/modeling/input/translation (MeshTranslationProduction holds Arc<HalfedgeMesh> as clone source, Arc is no longer RetireOwned; StagedSourceAdmission::borrowed now needs OriginalSourceProjection + projection-frame protocol, advance_into/take/close_into reply-shaped) plus modeling-job:251 (step needs body). Needs a SourceHold-style rewrite (reference: tool-run entities source-hold, ToolRunEntityVersion projection impl) with a local newtype around ArcLease<HalfedgeMesh>. Not attempted blind.
- pixels-tests: ported to EffectTurns/BlockingTurns, duplicate allocators removed (no Cargo change), compiles native (W-geom-10). Runtime W-geom-12: 35 ok / 25 FAILED (png/jpeg decode, png encode, retirement); image_decode_phases_record_honest_work aborts (inflate history Drop; "no quoting body funds the next close step"). Failures not yet analysed.
STATUS: BLOCKED 3d (needs redesign decision), tessellation FIXED-verified

## Round 3
- semio-framework-3d: MeshTranslationProduction rewritten reply-shaped (phases ADMIT/CLONE/TAKE/TRANSLATE/PUBLISH/DONE, pending keeps state via reply scopes). New local `MeshOriginal(ArcLease<HalfedgeMesh>)` implements OriginalSourceProjection; local `translation/🔗️source-hold` is a copy of tool-run's source-hold (debt: dedupe into value once the owner decides). API: new(&Arc<HalfedgeMesh>,offset), demand(body), advance_into/recover_advance_into(body,effect,reply), take_output(), cancel(), complete(), progress(); retire it with ControlledRetirement. +RetainedCloneOwner::output_ready (value, additive). modeling-job close_tessellation_handoff passes body=grant copy bytes.
- Native W-geom-15 rc0, wasm W-geom-16 rc0. No test of the translation production exists yet (debt).
- Downstream: generation3d plugin `🎟️original` calls the old step(original,body,grant); needs port by the plugin lane.
STATUS: FIXED-verified W-geom-15/16 (semio-framework-3d, 3d-brep-tessellation); pixels-tests runtime failures open

## Round 4
- semio-framework-3d-brep-mesh-io: close_step body = grant copy bytes; native W-geom-20, wasm W-geom-21 rc 0.
- pixels-tests: shared fixture 🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs `with_effect` now drives a genuine producer invocation (plain_effect private), fixing "native retirement requires its original producer invocation" (and the abort). W-geom-22: 86 pass / 2 fail.
  - mask lease test: assertion corrected to exact law (physical == receipt capacity/release, release found within first 4 quoted steps); rerun W-geom-23 blocked by a peer compile break (rc 101), unverified.
  - OPEN: stalled_close_of_nested_record_owners...: retire_funded's owner.abandon() trips Drop for Deferred ("deferred original abandoned before terminal-empty", value retirement 746). Owner decision: abandon semantics for deferred children.
STATUS: FIXED-verified mesh-io, 3d, 3d-brep-tessellation; pixels-tests 86/88

## Round 5
- retire_funded (pixels image decode): a stalled funded close now drives the owner, deferred children included, to terminal through paid closes at their own quoted grants (close_unfunded) and then refuses with the typed error; abandon is gone. Nested-record test passes (W-geom-28: 87/88).
- source-hold lowered into new crate semio-framework-value-source-hold (🌱️value/🔗️source-hold), used by semio-framework-3d and re-exported as `source_hold` by tool-run-entities; both local copies deleted. LOCK NEEDED recorded (framework members, hub/root dep lines, two Cargo.toml deps). Native W-geom-24, wasm W-geom-25 rc 0.
- MeshTranslationProduction::step_turn(body,effect) added (resident recovery via route); generation3d plugin 🎟️original ported to it (translation_close ControlledRetirement drained in phase 1). UNVERIFIED: the file as a whole still uses pre-733 grant-style calls (owner.step(grant), cx.retained_grant() etc.), so the plugin crate does not build for unrelated reasons; plugin lane owns the rest.
- semio-framework-3d-brep-engine: retention close_step body; native W-geom-26, wasm W-geom-27 rc 0.
- mask-lease test: rewritten to the exact law (Arc backing released exactly once, only without aliases); rerun blocked by peer red in value retained-clone (paged-list:46).
- Own five crates' tests: still blocked upstream (pending reruns).
STATUS: FIXED-verified source-hold/3d/tool-run-entities/engine/mesh-io/tessellation; pixels-tests 87/88 + 1 unverified rewrite
