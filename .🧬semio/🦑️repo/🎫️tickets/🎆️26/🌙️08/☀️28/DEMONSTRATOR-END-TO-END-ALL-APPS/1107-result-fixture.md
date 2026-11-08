STATUS: UNVERIFIED — harness edits complete, never compiled: W-fixture-371 (log 🗑️generated/1105b/W-fixture-371.txt) dies in semio-framework-pack-json-canonical (25 errors; earlier 361 also semio-framework-async 11, io-failure-owner 4), all upstream of my crates. No production fault exposed yet. Rerun: 1105b-tools/fixture/smoke.sh <round>

## Edited (all verified against code before acting; audit D-numbers)
- D8 heap witness: new `semio_plugin::install_fixture_heap_witness!()` (testing/🦀️.rs, re-exported in plugin/🦀️.rs) and `artifact_app_laws::assert_heap_witness_installed()` called first by both pane-smoke laws. cad, fem3d, sourcing, generator already had a witness; added to the process3d pane target and the gis laws lib (no allocator there before: energy-style 48 B vs 0 symptom class). Energy mounted tests already have one (mounted-bestest:4).
- D1/D4 lifetime and clock: `fixture_renew_turn` replaces every `testkit::renew_turn` in artifact-app-laws; `arm_host_lifetime(cx)` (armed by both pane laws, guard drops on unwind) debits one native initiating-event stock (HostLifetimeStock = NativeFrameRootPolicyV1::initiating_event minus retirement reserve: 58,720,256 fuel, retained currencies, 30 s real wall) across renewals and refuses a turn the host would not issue. The cx clock stays frozen at 1000 (caller.json) but the wall deadline is now real via Instant. Unarmed laws behave as before (no global change to the ~100 other users of OriginalFixtureScope).
- D7 birth: ensure_fixture_cancellation_issuer now also runs in dispatch_fixture_command and dispatch_fixture_response_action. plugin_load_document needs none: the runtime opening pays birth (opening/🦀️.rs:60), the audit's claim does not apply to runtime-hosted instances.
- D13: cad, bearbeiten, verfolgen, aussuchen smokes now use `assert_pane_smoke_by_example` (setActiveExample -> published LoadDocument -> archive door). Kept their text-constant render tests.
- runtime's edit in registered-app...settlement (retain_original_ownership_receipts split fault message) kept.

## Not changed, with reason
- D2 per-turn fuel 1M/2M vs lane 2M/6M/20M/80M: host native gives each turn the whole remaining lifetime; fixture 2M per turn is stricter; left.
- D3 caller.json grant (65536 items): pane smokes' cx is that grant; stricter than host per turn on items, laxer only on copy bytes; unchanged.
- D9 maintenance cadence, D10 refused-publication, D12 archive door via plugin_exchange, D14 timers/wakes, D17 patch pages, D19 cancel: need production-host wiring, not fixture constants; listed below for canary.
- Aggregator (puzzle3d) has no assert_pane_smoke caller; its smoke is owned by lane aggregator.

## Browser-only behaviour no native test can cover (for lane canary)
1. JSPI suspension / component-model wrapper and the 5 s guest-poll turn ceiling (reactor turn-execution-and-poll-entry:6-13): a 13.9 ms native turn was killed in a throttled tab.
2. WIT lowering/decoding of original turns (retained-turn TS originalTurnWit, frame-worker receipt decode).
3. Frame-worker admit/close yields via setTimeout, block-time accounting, 30 s boot deadline (performance.now clock).
4. Browser-clock lifetime debit (OriginalOperationSource TS) and lane wall budgets 4/16/50/200 ms.
5. Timer/executor wakes and the post-turn relay (event-dispatch Event::Timer).
6. Wasm linear-memory ceiling (guest_linear_memory_bytes native None).
7. UI patch page budget and reconcile ordering per turn.
8. Refused-publication retention on the live turn (collect_original_publication_receipts).
9. Heap receipts are never checked in the browser (no counting allocator on wasm32): a retained-page byte mismatch is native-only.
10. Hidden/throttled browser pane timing (plugin boot).


## Incident
I overwrote the untracked manifest runtime/ops/📄️document/📦️packages/🦀️rust/Cargo.toml (path-depth fix script plus a failed git restore). Rebuilt from the sibling ops-wire manifest; paths verified; description and possibly one non-path dep line may differ from the creator's. Logged in 1105b-claims.md 05:25.

## Reruns
W-fixture-101..211 (tools: 1105b-tools/fixture/smoke.sh <round>): 101-181 died pre-compiler on job-context source budget / missing path deps (not mine); 191/201 on peers' mid-edit Rust source delimiter errors; 211 first to compile.
