# Original Flow Core Failure Projection

Root owns exact OriginalDomainProjection and OriginalDomainFailure for the defining FlowCoreError. The original enum has 31 variants; its Dag payload has eight variants. Schema-first neutral fixtures cover 38 complete cases, including all owned text, minimum-port counts, all four incompatible-endpoint fields and each nested DAG error. These cases were authored before the projection implementation. No formatted string, boxed error wrapper, discarded payload or invented retained receipt replaces the original error.

The authoring consumer must reserve OriginalDomainReservation<FlowCoreError> before stepping Work, then move the actual Err into that frame. Existing authoring events have already posted their physical receipts; FlowCoreError carries no separate receipt, so domain_retained_progress is Default. Native projection, full usize and finite-budget outcome/cancellation proofs remain pending. No code has been mounted yet.

## Draft Complete, Native Pending

The unmounted implementation borrows every original field with only intrinsic object/text/unsigned-number views. Projection paths longer than the declared maximum depth two refuse immediately. All 31 outer variants are matched exhaustively; the nested Dag branch preserves all eight variants and all four of its text payloads. Minimum counts convert with checked u64::try_from rather than truncation. Messages borrow original text/widget/source fields or fixed unit text, and no new formatted String is born. Domain receipt remains Default because this type owns no independent physical receipt.

The strict 38-case schema enumerates exact permissible fields and expected key sets. Registered Bun genuinely passed Ajv plus an independent SQLite JSON projection for every case, with explicit 31-variant/eight-DAG coverage checks. These portable JSON cases use counts through u32::MAX; a separate authored native/serde test checks usize::MAX losslessly on the actual target.

Three native laws are AUTHORED, NOT RUN: all 38 complete payloads plus allocation-free borrowing/invalid ordinals, the actual native usize bound, and one original op97146/gen17/32768-fuel caller family for 1/512-item profiles with actual cancel_now at cuts0/1/2/3. The family reserves its typed frame before body, moves the actual preborn error into it, preserves the published original primary, closes all owners/decoder/initiating roots under the same stock, checks every observed heap receipt and final source-inclusive physical balance, and limits the entire family to 200 opportunities. No source/fuel/budget replenishment is used. These tests require the existing first-party Trace and existing workspace serde_json DEV edges in the rightful Core owner; no production dependency is added. The manifest/mount batch stays held until Runtime's fast four-law baseline finishes, to avoid obscuring that intended result.

## Mounted and Dependency Publication Pending

After the genuine four-law publication baseline passed, Root mounted the exact Core projection and its three native laws. The defining Core test owner now declares the existing workspace Trace and serde_json DEV dependencies; its Job testkit edge already existed. These are test-only edges, with no runtime dependency or new library. Runtime will publish the combined dependency batch after the coordinated all-edit freeze. Native behavior remains unexecuted.

## Corrected Numeric Owner Observation

The apparent second disappearance was an intentional generic owner relocation, not established data loss. Root verified Job/📥️field/🔢️float/🦀️.rs mtime21:14:57.222Z, its native law21:14:57.223Z, and old Flow wrapper removal21:14:57.224Z. Job root explicitly exports OriginalFloatFieldWork. The actual original_float_word_completes_under_actual_one_fuel_tool_frames law remains present under that generic Job owner. Preserve this current rightful owner; do not restore a duplicate thin Flow wrapper. Native FUEL1 execution remains pending.

## Corrected Cargo Publication Route

Runtime inspected the actual dependency input owner: repo dependencies lock/sync retain only Bun manifests/locks/config/patches, not Cargo.toml/Cargo.lock. Root's five preceding manifest changes were Cargo-only existing first-party DEV features/edges. Requiring another all-Bun lock/sync for those edits was unnecessary; Root corrected this sequencing mistake. Rust metadata/lock/config readiness belongs to the repo native Cargo owner. Lock-r2 did finish0, while sync-r2 ended1 before Source/installer due an independently investigated native caller capability refusal. Neither is native typing or a Rust test result. Do not block Rust on another unnecessary Bun installer cycle or claim that Cargo edges were synchronized by Bun.
