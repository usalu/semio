STATUS: HELD - smoke s18 hit the 1 h cargo:test deadline again (cold native compile); running s19a (--no-run warm-up) then s19b (real run). wasm GREEN generator crates; native laws UNVERIFIED.

## Smoke W-proc-s8/s9 (generation3d pane-smoke)
- s8: transient repo fault `Cargo profile policy Source differs from running canonical policy` (policy file edited mid-run); s9 reran clean.
- s9: generator_pane_loads_applies_undoes_and_closes FAIL (0.61 s, no SIGABRT any more, close path no longer the first fault): pane-smoke-law:42 load-example publication -> WorkLimit "original fixture settlement made no progress for its idle turn bound" (4096 idle turns, retained progress all zero). The [DEBUG] aggregator-gate Q1 probe printed lw_empty=false ops_empty=true lw_turn=true next=None stage=None items=65530 — the load work is pending but nothing drives it (same signature as aggregator's "typed command production pending forever"). Fixture opportunity (WorkLimit) and destructor panic are fixed; the 1.67 MB close release chunking is untested because load never completes.
- Not a generator source fault so far.


## Smoke W-proc-s7 (generation3d pane-smoke): 0/1
- generator_pane_loads_applies_undoes_and_closes: FAIL at pane-smoke-law:42 `load-example publication: WorkLimit "original fixture settlement exhausted its caller opportunity"` (retained_progress all 0) — the known fixture-stock fault (not initializer-cancelled). Cascade: store drop witness not reached, last close quote release_bytes 1675904 > grant maximum_release_bytes 1048576 on rung GrantedMounted, then SIGABRT (panic in destructor during cleanup). first-load work_spent is not printed by the law; add it once the fixture is aligned.
- Not a generator source fault so far; P2/P3 tick wiring not needed to reach this point.


## GREEN (finished runs, wasm check rc 0)
- semio-s-artifact-procedural-retained-source (proc-14); 3d retained-commands/editor/editor-presence/model (proc-19); 2d retained-provisional/editor/editor-presence (proc-22). Censuses within the 32-file/256 KiB cap in both cfgs.

## Generator pane smoke (single owner)
- Owner: generation3d root package test `pane-smoke` (🧪️tests/🪧️pane-smoke/🦀️.rs, [[test]] in generation3d/📦️packages/🦀️rust/Cargo.toml). Run: C/1105b-tools/proc/smoke.sh W-proc-<n>.
- W-proc-s1 (composition-laws variant) died on foreign semio-s-artifact-sequence-sequence-v1 (offclosure fixed it afterwards). To keep ONE owner, the include of the pane-smoke file in composition-laws (🧮️editor-unit-laws/📚️editor-laws/🦀️.rs:7) should be removed once pane-smoke passes (not done yet).
- W-proc-s2: cargo:test timeout 3587 s on the cold native build (no verdict). s3: transient repo-tool syntax fault (unit-cache lane ts:92). s4: foreign semio-plugin-host-wake E0624 (fixed by coordinator). s5: foreign semio-plugin-host-routers E0425 GuestRelayWakeAuthority (hostcheck lane). No test has executed: pass/fail per test, first-load work_spent and the exact-phase-loan fault status are UNVERIFIED.

## Native laws (UNVERIFIED)
- proc-t1..t4 compiled the laws crates and surfaced stale-law errors, all fixed in source (reads import, ScratchCaller/clock visibility, retire_decoded/ColdRetire imports, alias retirement free fns). 3d/2d laws crates never reached test execution; rerun with C/1105b-tools/proc/tst.sh <label> <manifest> native -p <laws crate>.

## Source done
- Crate split: shared retained-source, 3d-retained-commands, 2d-retained-provisional, per-app retained-source-laws; NativeProvisionalFolds<M> replaces the orphan-violating trait; funded_construction adapter.
- FlowHost cold close: retire_cold_or_park / FlowHostCloseRefusal::{retry_cold,resolve,parked,Display} with thread-local parking (no leak) + law (UNRUN). Model with_host/with_host_session -> Result<R,ValueError> + close_host.
- Ported: tick-and-resolve (3d+2d), io-text, flow-eval-tick, store-preparation last_turn_work, presence decode shape, mode-generate value dep, transforms admit closures, reserved-import path.

## Not done
- P2/P3 tick consumers + publication/emission wiring (no evidence yet that the pane needs it; decide after smoke).
- serde_json in new crates is testkit/dev-only (moved test code), pre-existing debt.
- I once overwrote 2d 🧵️retained-commands/🦀️.rs; restored byte-identical from HEAD.
