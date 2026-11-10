# Lane flowdoc: flow-document -> flow-host chain

## State 16:00 (11/09)
### Compile findings
- W-flowdoc-1 (`chk.sh ... Cargo.toml native -p semio-framework-artifact-flow-flow-document`) rc1: stops UPSTREAM of flow-document in `semio-os-kernel-neural-engine-model` (`🗣️dsl/…/📝️dsl/🦀️.rs:81 native_field_receiving!` not found in `semio_framework_dsl_record`). That is dslrecv's in-flight edit (claim 15:37); flow-document itself did not reach rustc, so the sqlite snapshot port (flowmodel, unverified) and vcs edits are still unverified. Rerun after dslrecv posts FIXED.
- Task 4: pack-json `controlled-and-borrowed-bridge:189` is the `DBG[...]` temporary-debug line of lane jsonwriter (claim 15:10, unreleased, `DBG` static defined at :245). Not unclaimed, not mine. Not seen red in W-flowdoc-1/-4.
- The catalogue files live in `🌊️flow/📔️registry/📇️catalogue-{work,owner}` (crate `semio-os-flow-operators`, AFTER flow-document; the same-named dirs under `🗿️artifacts/🌊️flow` are empty).

### Scratch compile loop for the registry (works, independent of flow-document)
`1108-flowdoc/scratch/{Cargo.toml,lib.rs}`: standalone `[workspace]` (+`cargo-features=["trim-paths"]`, root Cargo.lock copied) that `#[path]`-includes the REAL registry `🦀️.rs` (which pulls in catalogue-work/owner) and the extensions schema. Rerun: `chk.sh W-flowdoc-<n> "$C/1108-flowdoc/scratch/Cargo.toml" native`. W-flowdoc-4 gave 69 errors, all in registry `🦀️.rs` + catalogue-work + catalogue-owner. Delete the scratch dir when the real crate compiles (not tracked as product code).

### Done (FIXED-verified in scratch W-flowdoc-5: zero errors left in registry `🦀️.rs`; 61 remain, all in catalogue-work/owner)
- registry `🦀️.rs`: `retire_flow_extension_registries_step(body, effect) -> Result<RetainedCloneStep, RetirementPollExit>` (was grant-shaped; `owner.close_step(body,effect)` via `RetirementPollExit::resume`), `test_flow_registry_retirement_step` drives it through `OriginalRetirementPump` (`cold_turn`), `ContributedExtensionStub::retire_cold` drives `retire_step` through the pump, `owner.step(body,effect)` in `retire_step`.

### NOT done: catalogue-work / catalogue-owner (blocked on a design decision, not on effort)
The W-flowdoc-4 errors show these are not signature tweaks:
1. `FlowOperatorInfoMap(Arc<Infos>)` hand `RetireOwned` (`retirement()`, `SharedControlledRetirement::lease`) has no equivalent: the shared allocation is now `SharedOriginalStorage<T>` / `SharedOwner<T>` with `retirement_seed`/`RetirementCursor`, and creation/duplication is effect-funded (`admit_from`, `duplicate_into`), not `Arc::new`/`Arc::clone`.
2. `RetainedCloneSource::admission_owned(registry,())` + `source.project_owned(slot, closure, grant)` are gone. Borrowed originals now go through `OriginalSourceProjection<T>` (one authority type per projected `T`, no slot closure) and `SourceHold<T,A>` (lives in `semio-framework-tool-run-entities::source_hold`, `step_into(body,effect,reply,recover)`), or the `RetainedOwnedProjectionAdmission<T>` field projections. A registry slot needs a per-slot authority (`RegistrySlot{registry,slot}: OriginalSourceProjection<OperatorInfo>`), i.e. one source admission per operator.
3. Clone cursors (`OperatorInfo`, `String`): `advance(source,body,grant)`/`take(grant)`/`begin_close()` -> `advance_into(source,body,max_work,effect,reply)`/`take(&mut out,effect,reply)`/`begin_close_paid(effect)`/`close_step(body,effect)->OriginalRetirementPoll`, with a `ReplyStash` (exists only in `semio-os-store-snapshot-clone`/`replay-operation`, too heavy a dep for flow-operators; a framework-level stash is the clean home).
4. Only consumers of `FlowOperatorCatalogue*` are tests (`host/⚖️laws` `operator_catalogue_work_tests`, `catalogue-work/🧪️laws`); no production caller. Two clean options for the coordinator: (a) full port as above (needs: new deps `semio-framework-tool-run-entities` or a ReplyStash moved into `semio-framework-value`, LOCK NEEDED; ~6-10 compile rounds), or (b) delete the unconsumed catalogue-work/owner + their tests (greenfield, no legacy) and keep only the registry retirement. I did not delete: the task asked for a port.

### Task 3
Waiting for dslrecv FIXED. No receiving cursors added.

## Next steps (in order)
1. (done) W-flowdoc-5: registry code clean.
2. After dslrecv FIXED: `chk.sh W-flowdoc-6 Cargo.toml native -p semio-framework-artifact-flow-flow-document`, then wasm32-wasip2, then flow-host chain (source, read, core, session, host), flow wasm crates, `🌊️flow/📦️packages`.
3. Coordinator decision (a)/(b) for the catalogue.


## Coordinator ruling 16:30 (delete catalogue-work/owner): NOT executed
Grep of the primitive names (FlowOperatorCatalogueOwner, FlowOperatorInfoMap, flow_operator_registry_source[_demands]) finds production consumers outside the registry crate and tests:
- `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🧵️retained/{🏠️instance-owner,📥️original,🧮️evaluation,✏️authoring}` (owner field, `catalogue_infos`, `catalogue_ready`, source capture)
- `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/📇️catalogue`, generation3d `🧿️commands-and-instance-operation-owner`, generation2d `🛰️owner-and-io` (`catalogue_lifecycle`)
Deleting would break the flow editor and both procedural editors, so nothing was deleted. Need a re-ruling: full port (editor plugins are then also on the new API) vs redesign. Scratch crate kept.


## Re-ruling 16:45: FULL PORT (in progress, HANDOVER)
- ReplyStash moved: new `🌱️value/📬️reply-stash/🦀️.rs` (`semio_framework_value::ReplyStash`), store `🧬️snapshot-clone` now `pub use`s it, old `📬️reply` dir deleted (no duplicate). EDITED-unverified (value crate + store compile not run).
- LOCK NEEDED logged: flow-operators Cargo.toml +semio-framework-value-source-hold (EDITED). Scratch Cargo has it too.
- catalogue-work rewritten (SourceHold<SharedRegistry,SharedRegistry> + RetainedCloneRef::project per slot/key, ReplyStash cursors, `SharedOwner::admit_from`/`duplicate_into`, FlowOperatorInfoMap(SharedOwner<Infos>)); catalogue-owner rewritten (effect-shaped, new `close(cx)`, `step_demands(copy)` / `step_demands_within`). `flow_operator_registry_source(effect)`.
- Scratch W-flowdoc-8: 4 borrow errors only (fixed, rerun W-flowdoc-9). Rerun: chk.sh W-flowdoc-<n> "$C/1108-flowdoc/scratch/Cargo.toml" native.
- Editors ported EDITED-unverified: flow editor `🧵️retained/📥️original` (source via with_original_effect, `infos.storage().alias()`), `🏠️instance-owner` (`catalogue.close(same)`), procedural `flow-source/📇️catalogue` (`owner.close(cx)`). generation3d/2d need no edit (call the lifecycle + `step_demands(body)`/`close_demands`). Not compiled (heavy plugin crates).
- TODO: port tests `catalogue-work/🧪️laws/🦀️.rs` and `host/⚖️laws` `operator_catalogue_work_tests` (need driven effect harness; SourceHold needs an effect with a native invocation: check OriginalRetirementPump::with_effect provides it, else use the shared driven harness); compile flow-operators for real; compile the editor crates.
- flow-document W-flowdoc-6: only 2 errors left, `Widget: DslReceivingField` (vcs add-widget:13/change-widget:13); belongs to dslrecv (flow-model document-model), noted in claims.


## HANDOVER 17:45 (context low)
- Registry production code: FIXED-verified in scratch W-flowdoc-9 (native lib, rc0).
- Laws `catalogue-work/🧪️laws/🦀️.rs` ported to the pump harness (`pump(grant,|effect|..)` over `OriginalRetirementPump`); scratch lib.rs mounts it under `#[cfg(test)] mod laws_host`. W-flowdoc-10 (`SLOT_HANG_SECS=100 tst.sh W-flowdoc-10 <scratch> native --lib -- --test-threads=1`) did not reach the tests: upstream `semio-framework-dsl-record-model` `🫴️receiving/📥️owned/🌳️intrinsic:108` E0502 (dslrecv's in-flight edit). Rerun once dslrecv fixes it; open question whether SourceHold works under the pump effect (needs a native invocation); if not, drive through the shared value-testing-admission harness.
- Still to do: (1) rerun W-flowdoc-10; (2) after dslrecv gives `Widget: DslReceivingField`, chk flow-document native then wasm32-wasip2, then flow-operators for real, flow-host chain, flow wasm crates, 🌊️flow/📦️packages; (3) compile store `🧬️snapshot-clone` + value crate (ReplyStash move is unverified beyond the scratch value-crate build, which passed); (4) compile editors: flow editor `🧵️retained` (original, instance-owner), procedural `flow-source/📇️catalogue`, generation3d/2d editors; (5) host `⚖️laws` mounts the laws file via `use ...catalogue_work::FlowOperatorCatalogueWork` (line 90) — it also needs `RetireOwned`, `ControlledRetirement`, `RetirementDemand`, `RetainedCloneGrant`, `SharedRegistry`, `OperatorInfo`, `Infos` in scope (scratch mount lists them); (6) delete the scratch dir `1108-flowdoc/scratch` when the real crate compiles.

STATUS: BLOCKED flow-document waits on dslrecv (Widget: DslReceivingField; dsl-record-model intrinsic:108 E0502 in flight); registry catalogue full port FIXED-verified in scratch W-flowdoc-9, laws ported but unrun, editor crates uncompiled
