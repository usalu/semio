# Layer Protection Implementation Boundary

Status: implementation in progress. TypeScript capabilities, lock mutation schema, guarded Merge Down and mounted React protection checks pass. Native integration, retained history and live end-user acceptance remain unverified.

Layer protection belongs to the persisted `RasterLayerNode` variants, not `RasterConfig` or a React-only flag. The artifact root currently derives value/DSL codecs for Pixel, Group and Adjustment. The bounded clone skeleton in `🧬️schema/🧬️mutations/💾️binary/🦀️.rs` explicitly copies scalar fields for all three variants and must carry protection too; overlooking this would lose locks during retained publication even if the cold codec passed. Snapshot binary wrappers delegate to ArtifactPack. JSON/GraphQL/protobuf snapshot/diff mirrors and the TypeScript parser must agree with the new scalar.

The appropriate semantic event is ChangeLayerLocked, with an expected-value guard for conflicting lock changes and a precise inverse. The sparse layer patch, DSL event codec, binary mutation encoder/owner retirement, retained candidate apply, mutation catalog and exhaustive catalog tests must all include it. The current closed mutation vocabulary has 15 entries. About 51 Rust files mention RasterLayerNode; constructors, complete destructuring patterns and committed fixtures need inspection rather than assuming derive coverage is sufficient.

Proposed full-lock behavior: allow visibility changes, selection, inspection and unlocking; prevent pixel/mask edits, geometric and other property edits, deletion, reordering and destructive layer baking. A locked ancestor protects its descendants. A structural operation that moves/deletes/replaces a subtree containing a locked descendant must refuse. Insertion into a protected parent must refuse. Reading a locked layer to duplicate it is distinct from modifying it; preserve the copied lock metadata, and validate its insertion parent. Flatten Image must refuse if it would destroy a protected node; Merge Down must validate both subtrees and their ancestor chain.

Enforce protection at authored command admission/preparation and against the captured publication revision. History replay must still apply previously accepted semantic events, including undo of lock changes; blindly rejecting every diff over a locked node would break exact undo/reload. Verify the store's live-authority revision checks with a lock-versus-paint race before considering multi-user protection complete.

The Inspector should expose a localized lock control and disable protected mutation fields while leaving visibility and unlock available. The paint overlay needs an effective ancestor-aware protection flag and an understandable localized explanation. Selection/read operations should remain possible where they do not mutate image content; disabling a visual control alone is insufficient. Native paths must be protected by the same command policy.

Required neutral/test matrix: pixel/group/adjustment persistence, nested inherited protection, independent siblings, direct and subtree deletion/movement, mask and pixel commands, flatten/merge refusal, visibility allowance, own unlock versus locked ancestor, conflict guard, undo/redo, editable save/reload, and two-client lock/edit races. Check both ordinary codec round-trips and bounded clone/apply/retirement; the latter is a separate manually implemented path in this artifact.


## September 27 Implementation and Evidence

Added persisted `locked` to Pixel, Group and Adjustment, a guarded ChangeLayerLocked semantic mutation with inverse, text/binary mutation vocabulary, sparse patches and coalescing, retained clone metadata and digest. Canonical JSON/GraphQL/protobuf mirrors and authored JSON assets carry the field. Low-level replay remains independent of command protection. Native model guards cover direct properties, pixel/mask authoring, subtree deletion/movement, destination parents, flattening, merging and example replacement. Inspector controls expose localized protection and capability-based disabled states. Tree drag and flatten control presentation are still pending.

Neutral protection fixtures cover own, inherited and descendant protection, plus expected-value mutation conflicts. The TypeScript policy is compared with the independent d3-hierarchy ancestor/descendant oracle. Raster TypeScript run `raster-lock-typescript-2.log` follows two expected locked-Merge Down failures in `raster-lock-merge-red-1.log`; final count must be read before claiming success. Earlier model-only verification passed 70 tests.

Mounted test red: 27 passed, two new protection cases failed because no lock explanation existed. First green attempt exposed a refused keyboard Delete briefly taking the operation slot, so an immediate Select All was lost. The early authoring guard now prevents entering the busy state. Final `raster-lock-overlay-green-2.log`: **29 passed**, including own and inherited locks, pixel/mask action disabling, keyboard Delete/stroke non-dispatch and usable selection. This is mounted component evidence, not live browser proof. A lock change aborts in-progress preparation through the existing epoch/abort mechanism; a dedicated mid-preparation lock race test is still needed.

Native model attempt 2 failed compilation due to an unqualified snapshot module in new tests and a stray locked field on a stdio DrawLayer test fixture; both corrected. Inspector red attempt failed earlier in the framework plugin on `UiValue: Clone`, never reaching the Raster test. New native run 3 includes property capability tests, patch coalescing, lock persistence/inverse and retained undo/redo; status pending. Activation 10 failed on the earlier stray DrawLayer field in Raster IO; that source is corrected, activation remains unverified. No browser navigation was attempted after the recorded security-policy rejection.


## Follow-up Verification and Remaining Integration

Final Raster TypeScript `raster-lock-typescript-3.log`: **75 passed, zero failed**, including Merge Down own, ancestor and descendant protection. Mounted `raster-lock-overlay-race-1.log`: **31 passed**, including lock arrival while a selected pixel edit is preparing, no dispatch after cancellation, and successful retry after unlocking with the original selection intact. This is a component publication update, not a two-client authoritative race proof.

The tree now marks protected rows, keeps them selectable, disables structural dragging according to the same capabilities and disables Flatten when any protected subtree would be consumed. Merge hints mention protected parents/descendants in English and German. Native tests were added for property admission, atomic mixed selection refusal, delete/move and destination guards, pixel/mask preparation, protected flatten/merge, Inspector/tree control states, sparse patch coalescing and retained lock undo/redo.

Native run 3 stopped in framework plugin compilation (`HostMediaHandlerDescriptor::clone` and `UiValue: Clone`); run 4 still stopped on `UiValue: Clone`. Neither reached Raster tests. The latest shared framework compiler receipt changed and contained no errors, so native run 5 was started against the current source; pending. No framework ownership implementation was changed by this work. Browser activation remains unverified, and no browser policy workaround was attempted.

Renderer parity chat independently reported its earlier full-suite13 result (2476 passed, two protection cases failed during propagation edits). Our later focused 31-test pass includes those same cases after propagation and the refused-Delete fix. The whole React suite has not been rerun here; no reply was sent to the other chat without user authorization.


## Native Run 5 Reached the Raster Suite

`raster-lock-native-5.log` is terminal: **296 tests run, 281 passed, 15 failed, zero skipped**. The failures are the kinds catalog plus canonical/produced diff pairs for seven pre-existing scalar mutations. The new protection model, command admission, Inspector/tree, coalescing and retained history tests did not fail. The suite is not green and does not close native acceptance.

The new optional sparse `locked` field is emitted as null consistently with the existing name/visibility/opacity scalars. Updated committed patch fixtures to include it, added `change-layer-locked` to the catalog and subject/reference mutation vocabulary, authored a complete before/mutation/after/diff/outcome fixture, and added a native canonical-diff assertion. The independent Python reference now preserves the required lock property, checks the expected state and computes the inverse. Its cross-language harness has not yet been run; no Python oracle pass is claimed. The demo DSL carrier now explicitly authors unlocked layers.

Native run 6 was started with these repairs and the localized Add Adjustment row/test. It is pending. This run, rather than an old all-green suite, must establish final-source native coverage before current-source activation/live checks.


The native lock fixture exposed a schema mismatch: canonical native pixel nodes emit null for absent mask/extent/image keys, while the artifact JSON schema rejected those values even though the TypeScript parser already supports them. Added a shared fixture validation test: red was 75 pass/1 fail, then aligned those optional schema members with the canonical representation. `raster-lock-canonical-schema-green-1.log`: **76 passed, zero failed**. The mask's own optional members were already nullable; no additional mask representation was introduced.

Activation 11 (`raster-lock-activation-11.log`) was started after the Raster suite compiled and the earlier DrawLayer source error was corrected. It remains pending and is not live UI evidence. The Add Adjustment projection test now checks the exact dispatched action name and kind argument, not only visible text. A native run must include these final assertions before claiming that control verified.


Native run 6 completed with 297 passing tests and zero skipped; see generated/raster-lock-native-6.log. A subsequent native run is checking final assertion/source changes along with duplication regressions. This does not establish live browser acceptance or true multi-client race handling.
