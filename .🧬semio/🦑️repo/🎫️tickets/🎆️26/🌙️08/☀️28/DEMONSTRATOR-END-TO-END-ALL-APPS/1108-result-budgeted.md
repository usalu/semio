# Lane budgeted: semio-os-kernel-neural-engine-budgeted

Step 1 (EDITED-unverified, compile W-budgeted-1 running): crate port onto reply-shaped contract.
- Arc<Tree/seeds/infos/cache/channels> and the *Lease wrappers replaced by SharedOwner (RetireOwned directly).
- topology NodeKey is a Copy pointer+index key (no alias per key; aliases must be retired, never dropped).
- input BudgetedOwnedEntry wraps OriginalOwnedEntry with an own reply slot (advance_into/close_into, recover_* after a suspended turn); retirement via EntryOwners derive.
- cursors: close_into + model close_reply; inner close_step(body,effect); ControlledRetirement::step(body,effect), body = grant.maximum_copy_bytes.
- evaluation phase 26 uses SharedOwner::admit_from on the turn effect (quote source_admission_demand, no pay).
- NeuralCacheRetirement = ControlledRetirement<SharedOwner<NeuralCache>>; cold_cache_step via model cold_effect (made pub, additive).
Rerun: chk.sh W-budgeted-<n> 🌎️hub/Cargo.toml native -p semio-os-kernel-neural-engine-budgeted

Step 2: budgeted lib native GREEN W-budgeted-3 rc 0 (fixes after W-budgeted-1: OriginalEffectReply from semio_framework_value_ordered, EntryOwners send_only+pub(doc hidden), tree copy nested close body). Wasm W-budgeted-4 running.

Step 3: budgeted wasm32-wasip2 GREEN W-budgeted-4 rc 0; semio-os-kernel-neural-engine native GREEN W-budgeted-5 rc 0 (re-export crate).
Next: flow-host chain (W-budgeted-6 native -p semio-os-flow-host), then budgeted/registry tests.

Step 4 (tests, EDITED-unverified, compile W-budgeted-10): all budgeted + registry lib-test files ported off the grant-era API and off ObservedSystem/SHELL_OBSERVATION/AuthoringWallet.
- model testkit gains EffectCaller (genuine job fixture caller turns; poll/result/turn run each close/advance as the one original effect of a funded turn and charge its receipt); model Cargo testkit feature also enables semio-framework-job/testkit (feature only; LOCK NEEDED logged).
- Registry shell law: replacement observation = observe_ownership totals over SharedRegistry::new (born bytes) vs bytes released during funded close (equal), release-before-grant 0 after last reader drop; the per-allocation count is not observable with the shared allocator, so fixture physicalShell.allocations was dropped.
- Evaluation tests rewritten around helpers (sources/drain/drain_registry/grant_of); static fuel law retargeted to `if performed==0&&moved{cx.consume_fuel(1)}`.
- Input tests: BudgetedOwnedEntry stepped through entry_step (effect turn); copy tests through copy_step.
Rerun: SLOT_HANG_SECS=100 tst.sh (see header) with root Cargo.toml for -p semio-os-kernel-neural-engine-budgeted / -registry.

Step 5: semio-os-flow-drawing (coordinator W-main-174): kernel-impl boolean advance now runs under wallet effect, retired.close_step(body,effect); native W-budgeted-12 rc0, wasm W-budgeted-13 rc0.
Step 6: tests compile GREEN native W-budgeted-14 (budgeted + registry --tests). Run W-budgeted-15: pass cache_read, cache retirement, static fuel law, registry identity/fixture/handoff/fault tests (7/9 registry). Fixed after: tree-copy/output/wave/input drains must grant copy.max(quoted copy) (reply-shaped child births need their quoted bytes; narrow-work exemption gone); registry shell law physical == shell_bytes + retirement births. Rerun W-budgeted-16 blocked by peer red in semio-framework-value-paged-clone (advance_reply/select missing, clonetests claim).
OPEN: flow-host chain (semio-os-flow-host, W-budgeted-8: 85 errors; Arc<TreeSnapshot>/Arc<EvalChannels>/Arc<NeuralCache> -> SharedOwner, WorkLease, old `retirement(self)->Box<dyn RetirementCursor>` impls in evaluation-source work/read, publication work, eval-session-state) NOT ported; evaluation + input/wave/copy test results pending rerun; production per-event work quote for BudgetedEvalState (admit_from needs 4 fuel, one-fuel host loan starves phases 26/29/46) NOT added.

Step 7: budgeted lib tests W-budgeted-20 (--test-threads=1, root Cargo.toml): 9/13 pass (merge, plan cursor, tree copy, wave admission, cache read, cache retirement, fuel law, eval close roster); registry lib 9/9 (W-budgeted-17).
Open failures (test/lib, not yet diagnosed to root cause):
- eval normal-boundary test: completes evaluation now, ends with releases 192 B above original+births (same 192 B unposted wire heap noted by construct lane; fixture-caller side).
- input owned-entry test: stalls at demand copy 745; the quote depends on the body (re-quote with grant copy until stable; test loop only quotes once).
- topology test: owner never completes under the effect-shaped step (same one-quote suspicion, or reserve_page work); wave test: JSON encoder reports "exceeded its caller grant" at copy 1 where the old cursor yielded (wave next_demands understates the JSON receipt).
Rerun: SLOT_HANG_SECS=100 1105b-tools/runtime/tst.sh W-budgeted-<n> Cargo.toml native -p semio-os-kernel-neural-engine-budgeted --lib -- --test-threads=1
STATUS: BLOCKED flow-host chain not ported (85 errors W-budgeted-8) and 4 budgeted tests red; verified: budgeted lib native W-budgeted-3 + wasm W-budgeted-4, engine W-budgeted-5, flow-drawing native W-budgeted-12 + wasm W-budgeted-13, registry tests 9/9 W-budgeted-17
