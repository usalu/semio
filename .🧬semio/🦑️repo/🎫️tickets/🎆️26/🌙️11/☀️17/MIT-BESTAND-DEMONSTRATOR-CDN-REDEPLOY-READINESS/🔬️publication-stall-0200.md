# Publication stalls (0200 matrix): Verfolgen receipt overshoot, Koordinator blocked, silent refusal retry

## 1. Verfolgen: `canonical original child receipt exceeds caller grant` (3975 > 3336)

Evidence (`🗑️generated/ov-0200.log`): `canon kind 3 phase 4 ... d {copy 3336, depth 3} p {copied_bytes 3975} g {maximum_copy_bytes 3336, depth 3}`. `kind 3` is the Hash traversal (the seal cursor, every seal phase CountStart..Finalize), `phase 4` the canonical source phase that owns it. The caller grant is exactly the quote (`with_allocated_effect_reported` grants `maximum_copy_bytes = demand.copy_bytes`), so any overshoot of one turn is refused.

Root cause: the working-tree `ArtifactCanonicalEditSealCursor::advance` (`🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`, loop `for _ in 0..SEAL_TURN_BYTES`, not in HEAD) runs several tree steps per turn and sums their receipts. When each step is lent the full turn grant instead of the grant minus what earlier steps already spent, the sum exceeds the quote: 3975 = 3336 (one alias step quoted at the whole grant) + 639 (a following flat run of 639 bytes admitted against the same full grant). The current file lends `left` (grant minus accumulated copy, capacity, release) to every step (`effect.nested(child_grant(left))`), which is the correct accounting; the 02:00 build carried the full-grant variant (inferred from the exact arithmetic and the red run below).

Law (no production change by me for this item): `canonical::seal::tests::canonical_native_edit_seal_settles_every_exact_quote_within_its_own_grant_for_large_edits` (`🔏️seal/🧪️tests/🦀️.rs`). It scales the dense fixture edit (id, forwards, inverse, nested mutation meta; scale 1, 8, 64), drives the seal with grants equal to each turn's own quote and with a wide grant, asserts every receipt fits its grant, and asserts both grants give the same digest.
- Red: with `child_grant(grant)` in the loop (02:00 behaviour) it fails `assertion failed: progress.fits(grant)`.
- Green: with `child_grant(left)` (current tree) it passes.
Command: `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -j 4 -p semio-framework-replication-edit --lib canonical::seal`: 3 passed (this law, the Node-digest fixture law, terminal polling), 1 failed: `canonical_native_source_handoff_refuses_extracted_original_without_losing_custody`, a pre-existing shared-harness failure ("original fixture grant ... exceeds its finite Source stock"), unrelated.

Side finding: a foreign `[DEBUG] phase {} window ...` `eprintln!` inside the seal loop allocates and made every heap-exact seal law fail inside `semio_framework_value_clone_source_testing::drive` (line 214, heap (392,0)/(666,98) vs reported progress). I removed that one line. `🔬️bearbeiten-seal-second-pass.md` blamed an accounting regression for this: it was the print. Heap-exact laws and `eprintln!` in a metered turn cannot coexist.

Not runnable now: the store-level `semio-os-store-tests-06` crate does not compile because three callers still use the removed `arc_custody::admit_shared_retirement` (`🌱️value/📸️snapshot/👁️read/🧪️testing/👥️presence-retirement/🦀️.rs:16`, plus `♻️retirement` tests, `field-clone`, flow): a peer's in-flight rename. The seal-level law covers the same arithmetic.

## 2. Silent retry of deterministic refusals

`preparation_refusal()` exposes the refusal, `typed-operation-unit` printed it three times and returned `Ok(())`, `retained_structural_refusal` only classified `UnsupportedOwner`, and nothing in production read it (only a test fixture). A deterministic `InvariantViolated` therefore re-ran until the 4096-unit stall fault.

Fix:
- `🔄️full-operation/🦀️.rs` (retained-command): `retained_structural_refusal` also classifies `InvariantViolated`; new `deterministic_preparation_refusal()`.
- `👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs` (instance-jobs): `fault_stalled_typed_operation_publication` and new `fault_refused_typed_operation_publication` share `fault_typed_operation_publication(operation_id, message)`; the refusal fault carries the owner's message.
- `🔁️stall-witness-and-unit-advance/🦀️.rs`: after each publication unit, `fault_refused_typed_operation_publication(operation_id)` terminates a refused operation immediately (same code `interactive-job.publication-stalled`, no new localized code).
- Language-agnostic law: the `structural-refusal` corpus (`🪟️mounted/🚨️structural-refusal/🧪️tests/🧫️fixtures/🔣️.json`): kinds `UnsupportedOwner`, `InvariantViolated`; prefix `is refused by its owner without a retry path: `; new case for `canonical original child receipt exceeds caller grant`. Rust twin `test_retained_structural_refusal` (`🪟️mounted/♻️frontier/🦀️.rs`) maps the new kind; the TS twin reads the corpus.

## 3. Koordinator `one-item publication blocked` (not root-caused)

Aussuchen and Koordinator (`setContributions`, a large edit) take about 28 batch progress units (`coord 1`), then `advance_apply_batch` answers `Blocked` 4094 times, then the stall fault. Verfolgen (small edit) reaches the seal after 3 units and fails there, so it never shows Blocked. `Blocked` is silent by design and has 15+ return sites between the batch fold and the sealer (`⚙️engine/🧳️bundle`, demand-vs-grant in the lifted sealer, work-quote denial `with_allocated_effect_reported -> None`, zero-progress seal turn). Static reading cannot tell which; no module build was allowed. Item 1 does not explain it (an overshooting step is `PreparationRefused`, not `Blocked`).

Next evidence: after rebuild, a one-shot non-allocating `[DEBUG]` in `ArtifactStoreOneItemSealer::advance` naming which of {permits_one, per-axis demand vs grant, work-quote `None`, zero progress} answered `Blocked`.

## Rebuild
Changed crates: `semio-framework-replication-edit` (seal test and debug-line removal; the loop fix was already in tree), `semio-plugin-retained-command`, `semio-plugin-instance-jobs`, `semio-plugin-instance-app`(typed-operations unit). Rebuild the `demonstrator` module root (the other modules mounting the instance runtime only for their next release), then the site.
