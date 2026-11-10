# mutation: semio-framework-replication-mutation
## Step 1 native lib: W-mutation-1 rc 0 (already compiles against the reply-routed retained-field; no edit needed so far)
## Step 2 wasm32-wasip2 lib: W-mutation-2 rc 0
## Step 3 tests compile: W-mutation-4 (🧰️framework/Cargo.toml native --tests) rc 0
- Added test helper 🧪️tests/🎡️effect-turn/🦀️.rs (`crate::effect_turn::turn(grant, |e| owner.close_step(0,e))` -> (step, born, freed)); foreign tests and native-contract test ported; dev-dep semio-framework-value-testing-effect-bank (LOCK NEEDED logged).
## Step 4 tests run: W-mutation-10 (tst.sh 🧰️framework/Cargo.toml native -p semio-framework-replication-mutation) rc 0, 26 passed
- Test crate global allocator removed from lib 🦀️.rs (testing-allocator, pulled by clone-source-testing, wraps the same RequestedAllocator so trace observers keep working).
- Dev-dep: semio-framework-value-clone-source-testing (poll-drive harness) in mutation Cargo.toml. LOCK NEEDED logged.
- Source of the original reds: lib was already green (retfield/bridge landed the port); only the tests called the removed sync close_step/step.
## Step 5 store codec (semio-os-store-codec-registry): W-mutation-11 native rc 1
- 34 errors, ALL in semio-framework-replication-retained (🧳️retained: set-field RetainedMutationLeafPreparation/Preparation trait now needs decide_reply/inverse_reply/close_reply; patch/🚚️apply advance/close_step on Fields; diagnostic retained_routed import + closure types; retained 🦀️.rs RetainedMutationAdvance/decide_retained_mutation/inverse_retained_mutation). Owned by lane `retained` (claimed 22:38, in progress). None in native-snapshot 🫴️receiving or replication-mutation.
- Rerun after retained lands: chk.sh W-mutation-<n> 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/📦️packages/🦀️rust/Cargo.toml native -p semio-os-store-codec-registry
STATUS: FIXED-verified W-mutation-10 (replication-mutation lib native W-mutation-1, wasm W-mutation-2, tests W-mutation-4/10); store codec recheck pending lane retained
# Follow-up: value test crates on the shared effect helper (coordinator request)
## State (in progress)
- Helper 🧬️contract/🧪️tests/⏱️effect/🦀️.rs: close_step/close_selected/step_granted now drive close_into/step_into through `turn`; added `admit` (real projection admission via clone-source-testing admit_owned), `driven` (poll-shaped close under drive harness), `heap_of`, pub `turn`.
- Ported: tuple-work, original-scalar-public, take, scalar lifecycle, paged-list public.rs/common.rs, std-collections integration (Drain/Close body), work-budget native (std BTreeMap row dropped: BTreeMap is no longer RetainedClone; native sorted maps are covered by native-sorted-collection).
- Fixture/law changes from the 733 design: scalar payloadBytes 8->16 (size_of Option<u64>), bindingCpu 3->4, payloadCpu 1->6 (binding check quote); low-limit quote may exceed limit (advance declines) so `low<=1` law replaced by `low!=1 => zero-work`.
## Interrupt: semio-ui-contract-values
- Edited: Cargo.toml (+semio-framework-value-native-owned.workspace, LOCK NEEDED logged); 🌱️original args/merge/lib: retirement.step_granted(self.body,effect) x4.
- W-mutation-31 showed exactly those 4 errors; W-mutation-34 now stops upstream at native-receiving-frame (OriginalReceivingPrimary::is_unexplained). Native + wasm re-check pending.
## Value test ports still open
- bulk-vec logical-body edited (admit/driven/native_copy asserts) + derive(RetainedClone) bitwise now emits native_copy; not yet compiled. Remaining unported: paged-clone lib unit 🦀️.rs + borrowed.rs (engine redesign), then run all and record pass counts.
STATUS: BLOCKED ui-contract-values verification (upstream native-receiving-frame); value test ports in progress
## Low-limit law (coordinator check)
- Original law `quote(limit 1) <= 1` cannot hold: ScalarCursor::advance_pure deliberately declines when `advance_work_demand > maximum` (733), and the quote is the required minimum (binding check 1 + lease dup 3 = 4; with a binding: check 5 + 1 = 6).
- Exact replacement now asserted in take.rs and tuple-work.rs: if the 1-unit quote `low` is > 1 then `quote(limit=low) == low` (limit-independent minimum) AND advance under limit 1 performs zero work, zero progress, zero heap; `low == 0` keeps the zero-work check; `low == 1` is exercised by the normal turn.
## Pass counts (W-mutation-40, tst.sh, retained-clone tests)
- original-scalar-public: 1/1 pass.
- original-clone-lifecycle: 1/4 pass (scalar_output_and_selected_close). Red: borrow_authority_handback ("original projection must be driven": BorrowAuthorityAdmission has no projection protocol, needs upstream/driver), composite_close (producer action remains live), scalar_compounds (work 0 vs 1 in take).
- take: 0/1 (array child producer "not yet available", upstream UnsupportedOwner, same as retfield debt 2).
- tuple-work: 0/1 (producer action remains live; masks a tuple/array child refusal).
- Not run/ported: bulk-logical-body, original-collection-work, original-paged-clone, native-sorted-collection (compile pending), paged-clone lib unit + borrowed.rs.
STATUS: BLOCKED value test ports partially green (see counts); ui-contract-values verify pending upstream frame fix
## ui-contract-values
- native W-mutation-41 rc 0 (FIXED-verified). wasm W-mutation-42 rc 1, only error is upstream semio-framework-job-native-turn 🌱️birth/🦀️.rs:29 mismatched types.
## Production faults (coordinator items 1-4): NOT done, design notes
1. Array child producer: ArrayCursor (🧬️contract/🧮️array-cursor/🦀️.rs) supports only T::native_copy() elements; every child path returns UnsupportedOwner (advance_demands, advance_work_demand, advance_into, close_into, close_original_into). Plan: mirror VecCursor (📚️vec-cursor/🦀️.rs lines 28-70, 93-96, 113-135): child=RetainedFieldCursor<T>, advance_route keys (inv,route 2=child advance,3=child take,4/5/6=child begin/close/release, index, body, maximum, source id), select_advance quotes for child states, push into NativeArrayPrefix per element, close ladder (child begin_close_into, close_bound_into, native_terminal_release.inline_into), then prefix element release. ~100 dense lines plus quotes; needs a dedicated owner and several build cycles.
2-4. Not started: depend on 1 for tuple/array children (tuple-work and composite close "producer action remains live" likely mask the same refusal), scalar_compounds take work 0 vs 1, BorrowAuthority admission projection.
STATUS: BLOCKED array child producer (+2-4) not implemented; ui-contract-values FIXED-verified native W-mutation-41
## Array child producer (coordinator task) - DONE
- 🧬️contract/🧮️array-cursor/🦀️.rs: child is now RetainedFieldCursor<T>; advance_child_into/child_selection mirror VecCursor (routes 2 child advance, 3 take, 4/5/6 child begin/close_bound/release, 7 prefix push, 8 publish, 1 native copy); recover_advance_into per route; close_into/recover/close_original_into routes 6/7/8/9 for the child; quotes (advance_demands/work, close_demands/work) now include the child frontier. Original at scratchpad array-cursor.orig.rs. retained-clone lib W-mutation-50 rc0.
- Items 2/3 cleared: they were missing begin_close_paid on the tuple cursor (engine tuple) and derive(RetainedClone) records (work 0 vs quote 1), a test-helper that leaked live actions on error (now forgets them so the real refusal surfaces), tuple-work slices below the new minimum quote (5+ check), StrictCursor missing begin_close_into/native_terminal_release. Helper begin_close_paid now drives begin_close_into through the original action (paged engine needs an invocation).
- Law note: PagedList composite close quote carries one extra frame (depth 2 vs 1 for the others); asserted exactly per type.
## Pass counts (W-mutation-60/62, native)
- original-scalar-public 1/1, take 1/1, tuple-work 1/1, original-clone-lifecycle 3/4 (only borrow_authority_handback red: BorrowAuthorityAdmission has no projection protocol = item 4).
## ui-contract-values: native W-mutation-41 rc0, wasm32-wasip2 W-mutation-52 rc0.
STATUS: PARTIAL array producer FIXED-verified native W-mutation-62; item 4 open
## Item 4 DONE: original-clone-lifecycle 4/4 (W-mutation-64)
- BorrowAuthority is opened through the real staged projection admission (`effect::admit`) and wrapped by new `RetainedCloneBorrowAuthority::from_admitted_source` (clone-source lib); handback uses take_authority_into until terminal. The old `RetainedCloneBorrowAuthority::admission` wrapper can no longer reach `ready` (no projection protocol) and should be removed by the clone-source owner (its own tests in 🎟️admission/🧪️tests are stale).
## Remaining ports (per-target runs; the whole-crate run is refused by the 32-file Source budget, so use --test <name>)
- bulk-logical-body (W-mutation-67): 2/5 pass. Red: logical-body.rs:98 total copied bytes 102242 vs expected 102120 (+122, binding/ownership accounting changed by 733), :25 `turn<limit-1` (advance turns exceed maximumTurns in logical-body.json), :13 funding assertion. Fixture/accounting update needed, not a code fault found.
- Not yet run: original-collection-work, original-paged-clone, original-bulk-vector-public, alias-work, original-string-work, array-prefix-selection, native-sorted-collection, paged-clone lib unit + borrowed.rs.
STATUS: PARTIAL FIXED-verified W-mutation-64 (lifecycle 4/4, take 1/1, tuple-work 1/1, scalar-public 1/1); bulk-logical-body 2/5
## Follow-up round (coordinator: remaining targets, remove admission)
- Removed `RetainedCloneBorrowAuthority::admission` + `RetainedCloneBorrowAuthorityAdmission` (clone-source lib, contract re-export) and its stale use in 🎟️admission/🧪️tests (authority block deleted). lifecycle still 4/4 (W-mutation-70). OPEN: `🧪️tests/🧩️neutral-owner/🦀️.rs` (derive crate) still calls the removed admission in `admit_authority`; that whole file is already on the old sync cursor API and needs a port by its owner (admit via `from_admitted_source`).
- bulk-logical-body left untouched per ruling (reply handoff charge owned by handoff/valuebugs lane); recheck after handoff lands.
- original-collection-work (W-mutation-71): low-limit law applied (`q>limit => limit-independent minimum`), now stops at native.rs:20 `!cursor.output_ready()` (a cut cursor reports output ready; needs investigation of which type: Dsl array / PagedList).
- original-paged-clone (W-mutation-71): 2/7 pass. Reds: public.rs:110 PoisonError (cascade from another panic holding RELEASE_PROBE_LOCK), :363 derived scaffold child cursor "no borrowed close-start producer" (derive-generated child without begin_close_into in a test-local cursor or derive gap), :251 left 2 right ?, :328 adversarial fixture must terminate. Not triaged.
- Not run: original-bulk-vector-public, alias-work, original-string-work, array-prefix-selection, native-sorted-collection, paged-clone lib unit + borrowed.rs.
STATUS: PARTIAL; verified green: scalar-public 1/1, take 1/1, tuple-work 1/1, lifecycle 4/4; see open lines above
