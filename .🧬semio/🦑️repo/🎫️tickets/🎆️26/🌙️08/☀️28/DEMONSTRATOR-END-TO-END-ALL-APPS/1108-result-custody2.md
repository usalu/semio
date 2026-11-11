# Lane custody2 result

## Verified
- wasm W-custody2-2: semio-plugin-plugin-program compiles (0 errors). Fixes: program 🦀️.rs local `retired` effect-shaped (framework_job::close_original_effect_step + settle), surfaces native/read decoder identity `(NativeAllocationIdentity,u64)`, native.rs `receive` via `with_original_effect` (charges actual performed work).
- native W-custody2-4: plugin-program and runtime-instances compile; runtime-ops red (116 errs), now being ported.

## Edits since (UNVERIFIED, compile W-custody2-5 blocked by femrecv's in-flight derive/native-owned edits: 83 `default_*` field errors in semio-framework-value-native-owned)
- runtime-instances 🚪️opening/🦀️.rs: stale `built`/`retired` replaced with `funded`/`paid`/`retired(effect)->OriginalRetirementPoll`/`settled`; native_factory and actor authority are funded metadata moves (no construction admission); closes use `step(body,effect)`.
- runtime-ops manifest metadata.rs (`receive` via `with_native_receiving(&mut out,..)` + `receive_original_receiving_work`; dropped stale `construction()`; restored `NativeReceivingOriginal` import), manifest 🦀️.rs (`limits.clone()`), window-view/intent/response `decoder:(NativeAllocationIdentity,u64)`, response `receive_original_retained_work::<ValueError>`.
- ingress: `preparation_receipt`/`retirement_receipt` hold `(u64,Grant,Progress)` (OperationId is not RetireOwned, value cannot depend on trace); converted at typed-operations collect, turn reconcile, turn receipt test. `MediaArtifact` path -> document_media::lifted_app. `funded` work u128. Catalog crate: `artifact_retire_leaf!(ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase)`.
- media: `plugin_consume_media` only consumes and returns `MediaConsumption`; plugin-exchange `arm_media_in` rewrites a document import into `AppCommand::LoadDocumentArchive` and admits it through `state.archive_admission` (denied admission retries intact; first reply is the Pending status frame, retries answer Done).
- LOCK NEEDED: runtime-ops Cargo.toml (+semio-framework-value-native-receiving-original).

## Update (11/11 early)
- VERIFIED: native W-custody2-11 `semio-plugin-runtime-ops` lib rc0 (plugin-program, runtime-instances included); wasm W-custody2-12 (hub manifest) plugin-program + runtime-ops rc0. LOCK for runtime-ops Cargo.toml (+semio-framework-value-native-receiving-original) still logged. metadata.rs receiver is `NativeReceivingOriginal<DslValue,ViewModel>` (Arc<ViewModel> cannot be NativeReceive: orphan rule); the Arc is built in a funded wrap step (`received` -> `view`).
- job-receiving tests: compile (dev-dep semio-framework-job testkit). Test grants are exact quotes: `original_steps::birth_grant::<O>(Owner::admission_demands())` (reserve + settle overhead), `grant_of(retirement demand)` for decoder close, fixture `retire_exact()` (additive in job testing-caller). Hand-picked widening removed; stall asserts bound every close loop; `bind` (decoder source loan admit) added after each decoder.
- PRODUCTION FAULT (fix applied, UNVERIFIED): `born` and the pack/copies/transaction `admit` (and tool-machine typing/edit:43) returned through `reset_step` as soon as `output.is_some()`, but a born candidate still owns its unsettled Source outcome; `OriginalConstructionAdmission::settling()` (new, job construction) now lets the call fall through to `admit_original_construction`, which settles then resets. Earlier diagnosis (exact-quote births did not settle, 8/8 fail) predates the fix. NOTE: a diagnostic run showed the output-present branch was never reached, so a second quote gap may remain before the constructor runs (candidate: `with_native_receiving_quoted` quote `complete` is taken after the capture step already consumed items/bytes of the same grant). Check this first if t14 still fails.
- Heap-equals-receipt assert re-enabled; no [DEBUG] left in job-receiving (verify with grep).
- Queue: W-custody2-t14 (full job-receiving --lib) waits on cargo capacity. Rerun: `tst.sh W-custody2-t15 Cargo.toml native -p semio-plugin-job-receiving --lib`.

## Not done
- metadata-tests.rs, response-tests.rs, original-view-read.rs (26 KB rig), staged-tests.rs, completed tests; tool-machine edit tests after the settling fix.

## Update (11/11 08:55)
- W-custody2-t15 (job-receiving --lib, after the settling fix) RAN: compiled, 9 tests; 3 fail fast (media_descriptor, transaction_preparing, transaction_origin; panic text lost to the 7 MB output), 5 spin and were killed by the hang guard (181 s): pack typed_receiving, value_denial, value_receives_without_reparsing, intrinsic, provenance copies, plus the new copies underfunded_admission test. The earlier "birth did not settle" is gone: the settling fix changed behaviour.
- Added fail-fast stall assert `idle(p)` in the `turn` helper of pack/copies/transaction tests: 16 consecutive turns with a default receipt panic "original turns neither progress nor charge" (instead of 262144-iteration spins).
- Running W-custody2-t17 (`SLOT_HANG_SECS=100`, `-- --test-threads=1 --nocapture`) to name the spinner and the three failers; queued in slots since 08:25.
- Hypothesis to confirm (second quote gap): `turn_with_decoder` pre-consumes the previous turn's pending receiving effect (7 items, 2472 B copy, 880 B capacity) before the closure, leaving 3 of 10 items; `admit_original_construction` then needs 3 for the Source settle and 1 more for `reset_step`, which is not funded in that turn. Fix location if confirmed: `original_steps::birth_grant` / `original_construction_settle_overhead` must quote the pending-recovery carry-over plus reset (one more item and a bool of copy), not hand-widening.

STATUS: BLOCKED W-custody2-t17 queued in slots; native W-custody2-11 and wasm W-custody2-12 FIXED-verified; job-receiving tests spin/fail (diagnosis pending t17)
