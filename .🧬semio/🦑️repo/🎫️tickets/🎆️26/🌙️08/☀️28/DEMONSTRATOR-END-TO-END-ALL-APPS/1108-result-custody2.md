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

## Next
1. rerun `chk.sh W-custody2-6 Cargo.toml native -p semio-plugin-runtime-ops` after femrecv releases native-owned; then wasm (hub manifest); then tst.sh job-receiving pack/copies/transaction, metadata-tests, response-tests; then original-view-read.rs, staged-tests, completed tests.

STATUS: BLOCKED native-owned derive/native-owned edits in flight (femrecv) and LOCK for runtime-ops Cargo.toml; custody2 edits unverified
