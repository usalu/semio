# Receiving Outcome Repair Plan

Production files are not edited at this planning checkpoint. The neutral test first reproduces the current flattening defect:1pass1fail9assertions. Its corpus/schema independently compares work3 with physical items0 using lodash and SQLite and keeps semantic, quote and allocation causes across denied/occupied capture turns.

## API and file ownership

1. Store codec/native-snapshot/receiving/🦀️.rs: OriginalArtifactSnapshotReceiving::step returns RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>; blanket adapter directly delegates the whole inherent outcome and removes flat. Its close path also needs an outcome port, because current retire_step selects child.result and the public retirement cursor reports unconditional work1.
2. IFC4 any/io/sqlite/snapshot/native/owned.rs and owned-graph.rs: the custom receiver forwards the complete child outcome. GraphReceiving exposes its actual selected-frontier attempt as an outcome, preserving zero-work pre-action refusals and positive-work failures. Its existing retirement path already has outcomes; the public receiving boundary must retain them too.
3. Store receiving/pair/🦀️.rs: add OriginalArtifactPairCause::Native(OriginalReceivingPrimary) and conversion from the complete owner. Pair step and close carry work on both success and failure. Distinguish newly posted physical receipts from already-posted child outcomes so the same effect is never accepted twice.
4. Window config owner-contract-and-mutation/receiving/🦀️.rs: erase the exact outcome signatures, retaining complete pair failure custody.
5. Window config retained/typed-pack-load/native/🦀️.rs: receiving/native-step/reconciliation pass the actual work through errors and consume fuel once. Replace parent failure overwrite with an explicitly vacant capture; reserve its real fixed header before even querying a fallible demand, then leave capture headroom in the selected child quote. Adopt the complete error before returning a static diagnostic. No second failure may replace the first owner.
6. Tests and actual affected consumers in IFC owned-tests and window receiving/load tests are ported together, with unchanged source caps and no hydration or command-route changes.

## Adjacent work accounting gap requiring coordination

NativeDecodeControl has actual_work, prepare_work and commit_work. Actual native child outcome work must remain the source of truth; progress.copied_items is a physical currency.

RetainedHistorySourceDecode currently returns Result plus a stored physical receipt, with no work outcome. Its publish only stores progress. Some own semantic stages and delegated old child APIs do not post actual work to NativeDecodeControl, so subtracting actual_work alone would silently undercount them. The pair cannot honestly replace this with work1. A coordinated minimal History outcome/accounting port must expose selected own actions and preserve delegated child work; otherwise the requested pair port would merely move the loss to History. Exact owner is spr/history/decode/file/🦀️.rs, with child/auxiliary delegation checked before deciding the smallest necessary scope.

## Native test proposal

An independent integration target in the existing WindowConfig owner can use the genuine OriginalJobFixtureCaller, persistent decoder and actual pair+window erasure. A finite fixture snapshot receiver performs original-funded validation and returns a complete three-cause error owner from explicit fixture inputs with work positive and zero physical items. It is a port/custody regression, not a claim that a natural codec necessarily generates three simultaneous refusals. Measure every turn and paid close against actual heap and source receipts, deny capture currencies before invocation, preserve an occupied failure, and verify all three causes survive trait→pair→window. The real Text typed Pack receiver pilot remains the complementary actual codec proof.

No new native job has been launched. The single editor driver remains the only compiler route. Implementation awaits root review of this API plan and the History seam.
