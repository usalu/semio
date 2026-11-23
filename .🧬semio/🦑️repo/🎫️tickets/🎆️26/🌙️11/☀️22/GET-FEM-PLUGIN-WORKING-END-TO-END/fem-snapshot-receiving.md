# FEM snapshot receiving: root-cause measurements (no product fix landed)

Scope: `semio-s-artifact-fem-{2d,3d}-snapshot-tests`, receiving path `Fem{2,3}dSnapshot::receive_authoring` (`🧬️schema/📸️snapshot/native.rs`) over the framework native receiving stack (`🌱️value/🛬️decode/🧳️owned`).

## Status
- No product code changed; all `[DEBUG]` code removed (files restored: 2D `native-tests.rs`, `native.rs`, `snapshot/🦀️.rs`).
- The five failures are NOT fixed. 3D was not run (it is the structural twin; same code).
- Environment: the shared `2d-a` artifact lock was held for hours by other agents' `cargo test` runs (cargo holds it while tests execute). `C:\t` was also deleted once mid-session (link error 1104), forcing a full rebuild in `C:\t\snap2`. New helper `snap-run.sh` (build with `--no-run`, run exe directly) sits in the ticket folder; its exe lookup still needs fixing (it greps `Executable` lines; the build-dir layout puts the exe in `C:\t\snap2b\debug\build\<crate>\<hash>\out\`).

## Measurements (2D, debug profile, unlimited grant, `receive_authoring` of N nodes)
| N | work | actions (copied_items) | copy bytes | capacity charged | per node capacity | per node copy | wall |
|---|---|---|---|---|---|---|---|
| 10 | 15384 | 11266 | 7.97 MB | 2.13 MB | 212 KB | 797 KB | 1.1 s |
| 50 | 72378 | 53576 | 35.3 MB | 9.36 MB | 187 KB | 707 KB | 5.4 s |
| 100 | 143256 | 106294 | 69.2 MB | 18.4 MB | 184 KB | 692 KB | 18.8 s |
| 200 | 285012 | 211730 | 136.9 MB | 36.5 MB | 182 KB | 684 KB | 32.9 s |

- Scaling is linear in N (no quadratic identity/reference work in the node path). Released bytes equal charged capacity (rows are transient), so the live footprint is tiny; the law "retained capacity < 64 KiB per node" counts cumulative charged capacity, which is 182 KB/node, about 2.9x over.
- Cost per node: about 1060 funded actions, about 1430 work units, about 680 KB copy bytes, about 107 ms in debug.
- So `receiving_charges_below_sixty_four_kibibytes...` (1000 nodes, 64 MiB), `..._ten_thousand_node_fixture...` (512 MiB grant for 10000 nodes = 52 KB/node) and the scaling test fail because the grant is exhausted ("original grant cannot admit its frontier"), exactly as reported.

## Root cause
Every receiver in the native stack is a large inline struct and every one is charged by `size_of` (`NativeOwnedStorage::reserve_demand` = `Layout::new::<T>().size()`).
Sizes (bytes): `ValueError` 64, `NativeReceivingEffect` 5680, `NativeReceivingEffect<OriginalRetirementFailure>` 6496, `OriginalReceivingPrimary` 2544, `RetainedCloneBinding` 976, `RetainedCloneClose` 560, `NativeReceiveSlot` of an `f64` field 15744, of a `SealedShared<String>` 15792, persistent list receiver 12624, list slot 15792, record receiver of a 3-field `FemNode` **66192**, `Fem2dReceiving` 36304.
- A slot embeds two `NativeReceivingEffect`s (5.7 KB + 6.5 KB); each effect embeds an inline `OriginalReceivingPrimary` (2.5 KB, "no allocation after refusal" by design), plus binding/close state.
- A `FemNode` row therefore costs: record receiver 66 KB (3 slots: id, x, y, about 15.7 KB each, plus its own two effects) + shared-receiver wrapper + the `SealedShared<String>` child ≈ 180 KB per row, allocated and released per row. Two of the three slots only receive a single `f64`.
- This also explains the load test: `load_tests::every_actual_checkpoint_prefix_and_original_grant_denial_closes_without_publication` is NOT a deadlock. Running its exe directly (with `RUST_MIN_STACK=256 MiB`) it burns 100% CPU (83 s CPU in 100 s). It loops `for cut in 0..calls` with a full decode per cut, and `calls` is about 1000 per row, so it is O(calls x decode) = hours in debug. The earlier "0.12 s CPU in 7 min" was cargo waiting on the artifact lock. The default-stack overflow is the deep nested receiver frames (66 KB records on the stack).

## What a real fix needs (not attempted, too invasive for this shared tree)
1. Shrink the per-slot footprint by at least 3x: move the rare failure state (`OriginalReceivingPrimary`, partial work outcome, completion data) out of the per-field effect, e.g. one shared failure owner per record receiver instead of two inline copies per field, or a slim scalar-leaf slot for `f64`/`bool`/small ints in the `NativeReceive` derive (`🌱️value/✨️derive/🛬️receive`).
2. Cut actions per leaf (about 1060 per node) so `calls` and the per-cut-decode test shrink.
Both change the funded-allocation protocol that many framework tests pin, so they need a framework owner, not a drive-by edit. Bounds were not loosened.

## Not run
- 3D snapshot tests, full 2D snapshot-tests crate, `cargo test -p semio-s-fem-receiving` (nothing touched there).
- `native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix` ("native partial template release changed its original Work"): not investigated; likely a separate template-release bug, unverified.

## Update 2 (framework-owner pass)

### Fixed (product): cancelled admission with a pending source publication shell never closed
`native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix` first stalled in `retire()` (1,000,000 steps with a zero demand / zero work quote). Cause: `Fem2dReceiving` closed `source` before `admission`. While the admission is in its two-step publication (`take_output` writes a publication destination shell into the caller's `source` slot, then moves the live fields), the shell reports `terminal_is_empty() == false` but `next_close_demand` quotes zero (`publication` is not considered), so the close loop made no progress forever. Fix in `Fem2dReceiving::close_quote` and `close_poll`: while `source.publication_pending()` and the admission is present, quote and close through `admission.close_demand()/close_work_demand()/close_into(&mut pending, &mut source, ..)`, which completes the publication and returns the inputs. Files: `◻️2d/.../🧬️schema/📸️snapshot/native.rs` and the 3D twin. Verified in 2D: the test no longer stalls, and the 37 other 2D snapshot-tests pass (`--skip` the 5 heavy ones; log of this run in the session, exe in `C:\t\snap2b`).

### Next failure of the same test (investigated, not changed)
After the stall is gone the test fails with `native partial template release changed its original Work` (the message the 3D run showed). This is the framework law pinned in `📦️storage/🧪️tests/template.rs` (a foreign Work is refused when it initializes or releases a partially seeded vacant prefix; release must use the original Work). The test closes the cancelled receiver with the testkit `retire(rx)`, which drives an independent effect bank, so any cut inside a template prefix is refused by design. The product behaves per its law; the test's close route is wrong. The correct route is the original decode owner: `original_owners::with_decoding_granted(grant, &mut cancel_after_n, |owner| Fem2dSnapshot::receive_authoring(..))` (that route passes in `actual_pack_cancelled_inside_large_material_name_keeps_original_retirement_owner`). Rewriting this test to that route is cheap, but each cut is a full receive; with about 1060 actions per row the loop is minutes, so it should wait for the receiver slimming below.

### Receiver anatomy (type sizes via `-Zprint-type-sizes`)
- `FemNodeNativeReceiver` 66192 B = state 54016 (3 slots `id` 15792, `x` 15744, `y` 15744; matcher 2768; source 1024; projection 992; binding 976; retirement 560) + `effect` 5680 + `retirement_effect` 6496.
- Every slot = `action: NativeReceivingChild<OriginalReceivingPrimary>` 13280 B (+ binding 976, backing 680, retirement 560, authority 112, receiver 96). That `action` is a `NativeReceivingEffect<OriginalReceivingPrimary>`: base effect 5680 plus the 2544-byte `OriginalReceivingPrimary` repeated in `producer`, `producer_reply` and `child` because the failure type is generic over `E = OriginalReceivingPrimary`, and each failure also carries its own `receiving: OriginalReceivingPrimary`.
- `OriginalReceivingPrimary` 2544 = `allocation_failure` 840 + `receipt_completion_live` 432 + `retirement_failure` 336 + `native_allocation` 320 + 7 inline `ValueError` (64 each) + identities; the doc comment states the inline layout is deliberate (no allocation after a refusal).
- `NativeReceivingEffect` 5680 = `receipt_primary` 2544 + `allocation` 568 + `work_proof` 352 + `action` 352 + `outcome` 320 + `producer` 256 + `partial_work` 144 + `performed` 128 + 7 inline refusals 64 each + the rest.
- Seeding writes these vacant structs one field part per funded action (`NativeSeedTemplate::fields`, one action per field; `Bytes` kind is one action per 1024 B) and charges the full field size as copy bytes, which is why a row costs about 1060 actions and 690 KB of copy.

### Why the receiver slimming is not done
Test arithmetic (2D, 10,000 nodes): the grant allows 512 MiB copy, which is about 52 KB per row; a row copies about 690 KB today (13x too much). Capacity is allowed 52 KB per row (now 182 KB, 3.5x) and items 1000 per row (now 1060). A 2-3x shrink of the structs cannot meet the copy bound, and `1 000 000 x`-scale tests would still take far too long. What would meet it:
1. A scalar-leaf fast path in the `NativeReceive` derive (`🌱️value/✨️derive/🛬️receive`): `f64`/ints/bool fields received by the record's own effect (it already has `pending_x/pending_y`) instead of a 15.7 KB `NativeReceiveSlot`, removing about 31 of 66 KB per `FemNode`.
2. One failure owner per record receiver instead of two `NativeReceivingEffect`s per slot (the `OriginalReceivingPrimary` copies), keeping the no-allocation-on-refusal rule by owning the failure frame once in the record.
3. Charging template copies by bytes actually written (a vacant `Option` writes its tag only), and re-seeding a recycled row receiver instead of reserving/releasing 180 KB per row.
All three change the funded-allocation protocol and many protocol vectors (`frame`, `storage`, `owned`, `derive` suites) at once; with the shared `2d-a` lock held for hours by other agents and a build cycle of 10 to 25 minutes per attempt I did not start them half-way in the shared tree.

### Load test (`every_actual_checkpoint_prefix...`)
By design: `for cut in 0..calls` where `calls` is the number of progress callbacks of one full receive (about 1000 per row), each cut a full decode, so it is O(calls x decode). It is CPU-bound, not blocked (83 s CPU in 100 s). Making it reasonable needs either the slimmer receivers above (fewer callbacks) or sampling cuts (for example the first 32, the last 32 and every k-th in between); I left the test unchanged because sampling changes what it proves.

### Housekeeping
All `[DEBUG]` code removed (verified by grep over the FEM tree); testkit and test files restored byte for byte. `C:\t\sizes` (type-size build), `C:\t\snap2`/`snap2b` (2D snapshot build) are mine and can be removed.
