STATUS: GREEN (native). testdebt-c-52 semio-framework-value-retained-clone all 13 targets pass (lib 9/9, bulk-logical-body 5, bulk-public 4, lifecycle 4, collection-work, paged-clone public 7, take, tuple-work, string-work, alias, array, scalar); testdebt-c-48 semio-framework-value-paged-list 27 lib + 3 targets; testdebt-c-51 semio-framework-value-paged-clone engine 4/4; testdebt-c-33 retirement scalar-collection-grants. Not checked: wasm32 compile of the touched guest-reachable crates, other tests of semio-framework-value-retirement (lib tests there are testdebt-b's, 74 errors).

## Production root causes fixed (claimed in 1105b-claims)
1. PERF (report to runtime): value/♻️retirement/🎮️controlled/🦀️.rs:246 forced `maximum_items: 1` on every controlled child step, so Collection bulk close (`count=min(body elements, items)`) closed a Vec<u16> ONE ELEMENT PER TURN, and every turn rescans `Vec::retirement_ready` (O(n)) 4-5 times: closing a 100k-element source = 100k turns x O(n). This is the >10 min cancel test and a real pane cost. Now passes the parent's items. Still open for runtime: `Vec/VecDeque::retirement_ready` is an O(n) scan per quote; a per-type constant hook would remove it.
2. `RetainedCloneRef::bind` checked `maximum_work_units() >= 1` but charges `duplicate_work_demand` (3): a 1-unit turn ran and billed 3 against a quote of 0 (🔗️source/📦️packages/🦀️rust/🦀️.rs).
3. Collection `next_demand` quote was not a fixed point of its own copy frontier (`close_selected` invariant "quote changes under its exact admitted copy frontier"); now `body.max(header)` for Vec and VecDeque.
4. Array cursor `close_work_demand` quoted 0 for a parent flag-start of an already-started child (now `child_close_start_work`).
5. std-collections and dynamic `close_demands` for an unstarted child omitted the flag byte (depth only) so close stalled forever with zero progress; now `child.begin_close_demand()`.
6. Paged engine `close_step` had a placeholder `if !child_closing {work+=1; return Progress}` that never started the child close (infinite stall); now `start_child_close`.
7. Zero-work guard (`maximum_work_units==0` -> no step) added to dynamic, tuple macro and std-collections advance (they billed work under a zero limit).

## Test/fixture changes (meaning kept)
- logical-body: separate `stock.fuel` (67108864) in json + schema; `offer_owner` gives owner closes an items grant equal to their copy quote (item axis = elements for primitive collections); items stock 8388608; clone-source-testing STOCK items 4194304 (2M-element law case).
- Held::drop sources close with `next_close_demand(copy)`-derived grants (public.rs, lib bulk tests).
- string: header `size_of::<usize>()` is part of every copy quote/progress at HEAD (string-work and std reselect now subtract it).
- paged fixture `spentCloseTurns/Cpu` 9 -> 10: enumerated the 10 phases (alias retire start, slot reserve, native frame birth, frontier birth, ..., 3 releases, final binding header 528 B); the extra one is the separately paid alias-retire start.
- paged public.rs probe cursors: fixture advances now call `effect.performed(1)` (host-faithful); grants cover the 280 B binding copy.

## Still red (next step)
- lib reselect (std tests:21): after grant fix a later assert; lib dynamic: step with copied 97 B, work 0, depth 2 (some child close/advance records progress without effect work).
- lifecycle composite: "typed retirement receipt exceeded or changed its declared full grant" (controlled process_child_step, `copied_items!=0 && retained_capacity != birth`) in a later composite (Box/tuple/Field/Paged StrictAtom) after array was fixed.
- original-collection-work: now fails at work-budget native.rs:18 (left 1 right 2, work/quote after denial) for a type past BTreeMap; take.rs:23:350 and tuple-work.rs:12:460: work 1 where quote 0 at limit 1 (same family; add the zero/low-limit guard to the remaining cursors listed by `grep "maximum_work_units.min(effect"`: option, box, vec, array, field, string, scalar, alias, paged, ordered-map, issuance).
- paged public.rs: scaffold test line 323 (fixture child progress with work 0 after my fix? recheck), probe test line 82 (copy 0 vs 5), poison cascade.
- (5) value-paged-list lib tests and (6) paged-clone engine tests untouched.

## For runtime
- Items-axis semantics: primitive collection close uses items=elements, clone bulk uses items=runs; unify or document. Production grants with items=1 still close big Vecs element-wise.

## Update 2 (testdebt-c-20/21)
- FIXED `Vec/VecDeque/[T;N]::retirement_ready` O(n) rescan: new provided `RetireOwned::retirement_always_ready()` (true for every `artifact_retire_leaf!` primitive); collections short-circuit on it. Other element types still scan (honest).
- FIXED five RetirementCursors with no work quote (VectorIterator, UnorderedSet/Map, OrderedSet/Map: `Ok(!terminal_is_empty())`); dynamic `take_work_demand` 2 -> 1 (take records one unit); tuple advance refuses when its own work quote is 0.
- Asked runtime: items axis elements vs runs, recommendation = runs (copied_items 1 per bulk run, update collection law). Not changed until it answers.
- Open reds, unchanged cause unknown unless noted: lib reselect (std tests:21 col 1587), lib dynamic (copied 97 B, work 0, depth 2), lifecycle composite (receipt "exceeded or changed its declared full grant" from controlled process_child_step in a StrictAtom composite after array), original-collection-work now fails in the fixture source close of BTreeMap (`heap (0,616)` vs progress release 72: std node backing is unpriced but the allocator observer counts it; fixture/observer mismatch), tuple-work.rs:14:82 (complete-step work 1 vs `count+2`), take.rs:23:350 (low-limit denial, type name now in message), paged public.rs 82 (probe copy 0 vs 5), 323 (scaffold child progress without work), 103 poison cascade.
- value-paged-list (188 errors) and paged-clone engine tests: NOT started.

## Final (testdebt-c-22..52)
- RUNS decision (runtime + runtime2): copied_items counts one ownership action; a bulk run is one item. Collection (Vec/VecDeque) bulk close now takes `(body-8)/size` elements per step with copied_items 1, ControlledRetirement passes the parent's items. Law hand-edited: collection `cases.json` + schema (cases with owner, body, run, values, remaining; single-run [0] and a multi-run [22,4,0] / [14,0]), TS law updated, Rust law asserts exact run bytes and `copied_items==1`; logical-body offer_owner removed, items stock back to 100000 plus separate fuel.
- Production: `RetireOwned::retirement_always_ready` (O(n) readiness scan gone for primitive vec/deque/array); 5 retirement cursors got work quotes; PagedListRetirement pop-step quote no longer claims a birth capacity (fixed the controlled receipt refusal); paged engine adoption quote uses `adoption_demand`; dynamic/paged child take records its performed work; dynamic take quote 2 -> 1; quote clamps (vec, box) and zero-quote guards (dynamic, tuple, std).
- Tests/fixtures: std copySelection law is now the exact admitted BYTE prefix (fixture `expectedPrefixBytes`, schema, TS oracle, Rust) because the string cursor copies raw bytes and validates UTF-8 at conversion (one-byte-work law requires it); heap law exempts only standard-collection node backing (documented); tuple completing turn work 1; paged probes fixtures call effect.performed; paged value-list tests ported via `Granted` adapter over a local with_effect copy (`📋️list/🧪️tests/⏱️effect`), PageAllocation test impls moved to TryReserveError, copy/placed-byte expectations now compare against the live quotes (424 / adoption 216), sentinel `Some(73)` markers replaced by `None` (the production attempt marker refuses an occupied marker).
- Paged-clone engine unit tests: the 7 laws duplicated in retained-clone `public.rs` removed from the engine file; the 3 private-state laws (over-budget child, over-budget child retirement, abandonment) ported with host-faithful fixtures and the engine's own cursor type.

## For others
- semio-framework-value-retirement lib tests (74 errors, other lane) still on the old grant API.
- Lib test `semio-framework-value-retained-clone --lib` takes ~100 s (std/dynamic protocol matrices); not investigated.
