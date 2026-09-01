# Resident Primary Implementation Source Review — 2026-08-28

## Status and exact evidence boundary

The authorized neutral primary/recovery candidate is source-complete for review, **not compiled or natively executed**. No Cargo, rustc, Wasm check, native parser, or native retry was run by this lane. The current source roster is 32 (17 original canonical + baseline1 + release7 + primary7); this is textual enumeration, not a native test listing or pass count.

The sole executor's primary R1 remains the actual compiler RED: 65 coded errors, 9 warnings, zero executed tests. Root attributed 63 to the deliberately absent surfaces and two (E0109/E0618) to the local `ordinary` binding shadowing the helper. The ticket leaf changes exactly the two local occurrences to `ordinary_refusal`; all seven names, bodies' other expectations, vectors, fixed bounds and grants remain unchanged. [R1 complete report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-native-r1-compile-red-2026-08-28.md) remains historical evidence.

Original resident R11's 25 PASS is likewise historical, not a result for this enlarged source. The primary leaf's historical staging docstring still says “no canonical include”; the actual existing canonical include is mounted, and this report—not that preserved staging sentence—states the present boundary.

## Exact changed source and inverse

- [🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs)
  - Before: `e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3`
  - Candidate: `23516f6485e700392705dc97f62ffb8807212156c8a51dbdb6002da2106d998e`
  - Diff: +285/−34, 15 hunk(s).
- [.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs)
  - Before: `4f6dc45ffc159ee419529114f0eeb2a95f8ab6e6982436a6aa0a5f9bc098cc7f`
  - Candidate: `4e79891f6bc1fbcf801a344d196bb9d884208fc2e2ad06b6a1972179f146ee3f`
  - Diff: +2/−2, 1 hunk(s).
- [🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs)
  - Before: `30af821b98f7323f487a4ce8c027b6c3b8c7987f0fe64b0d7471ed521945426e`
  - Candidate: `f2336001a31c496606b03a29d65a6d372ce7be13768c249be42922a6f6541e1f`
  - Diff: +20/−6, 2 hunk(s).
- [🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json)
  - Before: `8df81492f42dfa1232a718e917149b209d7151a72d5bea397f354091290f55ad`
  - Candidate: `a4128141d608cbba5ef81c957e99ce7fd7951976e9573afa7e2e8957d67f9f64`
  - Diff: +2/−0, 1 hunk(s).
- [🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json)
  - Before: `3d9b729ec2fef59a179ce4425a7d1c0554c5937d19512065f3bf760568640b6a`
  - Candidate: `61f7c69795318efcc2320503aae8301becce0266eb878beee64170b2ca557eba`
  - Diff: +3/−1, 1 hunk(s).
- [🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts)
  - Before: `50793dbcbf2d873e8391faebfe436322470840a2db5d4e584b95032838f89ab3`
  - Candidate: `60da5c0537b099e1385ea72e836a30a1d82e63de00fd2e75cc39f1f0434a5b3d`
  - Diff: +15/−0, 1 hunk(s).

[Complete readable forward and inverse](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-implementation-full-diff-2026-08-28.md) contains all six files. [Exact preimages, postimages, unified hunks and inverse](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-implementation-source-delta-2026-08-28.json) permits exact reconstruction. [Independent Bun read-only hunk replay](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-implementation-inverse-check-2026-08-28.json) checked all six forward images and inverses against the complete captured texts, and all current source bytes matched the postimages. **No inverse was applied to disk.** This is source integrity, not Rust type/borrow checking.

## One original root, list and Release

[Metadata](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:178) adds only inline root/list metadata:
- Root: `last_consumer_registration:u64`, `primary:Option<ResidentPrimaryAnchor>`, `recovery:Option<ResidentRecoveryCursor>`.
- Anchor: exact nonzero registration + TypeId, original partition, and `Pending(ConsumerPage) → Published(NonNull<ConsumerHeader>) → Releasing`. Releasing is pointerless.
- Cursor: exact anchor stamp, Forward/Closing mode, revoked flag, separately owned next/found counted pins. Each pin retains the exact node pointer plus original nonzero registration.
- Consumer header: recovery pin atomic and original nonzero registration. ConsumerPage retains that same constructor registration; its initializer receives the already reserved stamp.
- Existing Release origin distinguishes primary Pending/Consumer and retains the original registration through Destroy/Free/Refund/Clear.

The finite bootstrap root's real `size_of` and descriptor maximum include the new inline anchor/cursor. Consumer allocations use actual `Layout::new::<ConsumerNode<C>>()`; the native tests independently measure those types. No runtime measurement has yet been executed for the candidate. There is no new queue, Box, page pool, external identity table, caller-issued numeric identity, mutable public alias, or movable-facade backlink.

Both ordinary and primary reservation share one checked monotonic counter. Capacity/short/MAX refusal occurs before counter, pending descriptor, or ordinary prepared-pointer writes. A successful reservation retains exact charge + pending page + stamp in the original root before the first allocator attempt or return. Ordinary registration may replace its own latest pointer; it cannot replace the primary anchor. Primary publish joins the same existing list without changing ordinary latest-prepared state.

The existing root is still the explicit finite bootstrap exclusion, **not funded RuntimeAppCell construction**. Actual registered parent-field receivers, actor chunks, Store FIFO bindings and SyncSession parent/receiver retirement remain separate, unmodified prerequisites.

## Checked per-call work inventory

Symbols are actual native sizes: A = Option<PrimaryAnchor>, B = PrimaryBacking, P = Option<ConsumerPage>, Q = Option<RecoveryCursor>, K = Option<RecoveryPin>, U = AtomicUsize, G = u64, F = bool, V = ResidentResources, L = Option<ResidentRelease>, N = selected ConsumerNode Layout size, H = ConsumerHeader, T = ResidentConsumer handle, I = Option<NonNull<ConsumerHeader>>. Every nonzero accepted phase requires one item and the checked sum; unchanged zero-byte no-op close uses zero items/bytes. Grant 4096 and capacities are not raised.

| Phase | Declared intrinsic byte sum |
|---|---|
| Primary reserve | A + G + V |
| Ordinary reserve | P + G + V + I |
| Shared selected-node allocation | N + I + G |
| Empty typed initialization | N + F |
| Primary publication | B + 3P |
| Ordinary publication | 4P + I |
| Begin recovery | Q + U |
| Nonmatch traversal | H + 2U + 2K |
| Match next→found | H + 2K |
| Capture found→counted alias | H + 2U + Q + T |
| Begin whole-primary close/revoke | F for each actual changed latch |
| Close one next/found pin | K + U |
| Clear empty recovery cursor | Q |
| Detach pending primary to Release | B + L |
| Detach published primary head | 2P + L + B, plus I only if original ordinary prepared pointer also names it |
| Destroy empty selected node | N + L |
| Actual allocator Free | N + L + G |
| Original partition Refund | L + V |
| Primary anchor + Release Clear | L + A |
| Ordinary Release Clear | L |
| Final original root observation/close | actual ResidentLedgerRoot size |

These are the reviewed logical native field/extent work charges, not a claim to count CPU bus traffic or allocator-internal heap bins.

The three root-reviewed read refinements are explicit:
1. **Begin Q+U**: the initialized original list-owned ConsumerPage already holds constructor metadata and retains the head allocation under the gate. Missing/uninitialized/null page is rejected first. Begin reads/writes only the header pin atomic; it does not reload header registration/type.
2. **Drain K+U**: the original root-held counted pin supplies identity and allocation lifetime. Drain reads/writes only that pin atomic; no extra header registration reload occurs.
3. **Nonmatch H+2U+2K**: current header identity and its next field are in the H-charged phase. The original linked successor ConsumerPage supplies successor constructor identity/initialized state. Successor header access is only its pin atomic. Current/match/capture header validation remains real and H-charged.

No silent work-formula change or C/alignment compatibility branch was introduced to accommodate these sites. Source formula changes requiring review were isolated to the approved old ordinary consumer test below.

## Ownership ordering and unsafe-load inventory

All access is under the existing one-attempt gate; no new unsafe impl was added. Existing ConsumerPage/LedgerState Send erasures have narrowed explanatory docstrings, not broadened bounds. Typed C remains Send + 'static, and all new root metadata pointers are either original list ownership or counted same-root pins.

| Actual source site | Authority / ordering |
|---|---|
| [Shared allocation/init](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:545) | Original exact reservation precedes alloc. Null preserves stamp/charge/descriptor. Pointer and allocated-byte total are installed only on success. Init writes empty C and exact constructor metadata; no live C is dropped. |
| [Ordinary/primary publication](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:643) | Initialized exact node under the original gate. Original pending page moves into the same list; primary anchor becomes Published without a fallible callback between ownership writes. |
| [Begin hook/load](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:710) | Original initialized list page retains allocation; Q+U preflight precedes hook/load and checked pin increment. No header stamp/type load. |
| [Advance current hook/load](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:736) | Closed/revoked/type/mode/stamp checks and full phase grant precede any payload pointer load. Current exact counted pin retains node; actual header registration is checked. |
| [Successor hook/load](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:751) | Successor is the initialized next ConsumerPage in current H. Its pin increment is checked/acquired before old pin decrement and cursor replacement. Overflow preserves old pin/cursor; only successor atomic is loaded. |
| [Capture hook/load](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:771) | Exact original found pin/root/stamp/phase precede H validation. One-attempt checked alias CAS succeeds before found pin release and cursor clear. Race/overflow leaves found pin intact. |
| [Close pin hook/load](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:413) | Cursor revoked in its own granted phase first. Exact counted next/found pin survives until this K+U step; decrement precedes clearing its slot. No registration reload. |
| [Pending primary detach](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:384) | Exact original stamp/type/partition and grant. Empty-source function is called only when initialized. Anchor becomes pointerless Releasing before the existing Release takes the descriptor. |
| [Published consumer close](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:337) | Existing list-owned node; revoked writes, structurally empty C, zero aliases/admissions/recovery pins. Original anchor pointer/stamp/partition must match. Anchor is made pointerless before list detach. |
| [Destroy/Free/Refund/Clear](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:444) | Exact original Release stamp/partition checked at each phase. Destroy sees only an empty typed node; Free uses original Layout once. Original charge survives actual free, then exact original Data/Control refund, then separately granted anchor/Release clear. |

There are exactly five new recovery pointer-read hook sites: drain, begin, current traversal, successor traversal, capture. The existing cfg-only hook records fixed scalar events immediately before real pointer loads; it supplies no authority and is not a replacement implementation. Snapshot traversal and count fault injection live solely in the ticket cfg tests.

Closed/revoked/stale resumed recovery calls reject before hooks or pointer loads. Caller continuations retain no next/found payload pointer; those remain root-owned. Captured public consumers intentionally are counted aliases tied to the original root lifetime.

Close drains/revokes the cursor **before** existing pending/admission-head cleanup, allowing the reviewed fair schedule to recover and hand out live primary C in Closing even when its original admission blocks ordinary cleanup. A suspended pointerless caller does not prohibit page retirement after exact pin clear. This is a specified fair schedule and desired native law, not arbitrary scheduler liveness proof.

Sticky poison permits only pointerless Release Refund/Clear with no cursor and no live primary backing, or the original exact-empty root observation. It does not traverse a live/unknown poisoned payload or create a generic poison-recovery result. A null pending allocation enters Refund(None) and performs zero Destroy/Free.

## Exact old-test phase correction

[Canonical aligned-layout test](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:551) was the sole existing body adjusted. The reviewed [proposal](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-aligned-phase-test-proposal-2026-08-28.md) was approved before mount.

The test independently derives reserve-field work and actual node allocation work, asserting positive and within the unchanged full grant **before COUNT_ALLOCATIONS is armed and while the root is empty**. It probes zero items, zero bytes, and one-short bytes separately at each of these two phases, then exact reserve and exact allocation, followed by existing initialization/publication. It collects no-change and exact-work observations and asserts after original cleanup. This is exactly six consumer refusal probes, not all byte lengths.

The record half's original calls/values remain, except moving its refusal array index to six. Original three Layouts, aligned record check, exactly three allocations/three frees, original envelope + intrinsic all-axis charge, and 32-step cleanup bound remain. No known-stale reserve-frontier rerun was used to manufacture a second native RED. Native execution of this changed body is still pending.

## Actual declaration-first source RED/GREEN

Both commands were the existing TS domain route, **not the Rust route**:

```sh
bun x nx run @semio-tech/value-resident:test --skip-nx-cache
```

R1: schema/controller gained required two-phase declarations while the fixture remained unchanged. Actual exit 1 at the existing admission fixture schema assertion; actual=false, expected=true. The existing assertion rendered `null` because it prints the Ajv instance's errors; no fabricated missing-field diagnostic is substituted. It stopped before the new Immer transitions. Full actual output and all nine selected stable pre/post hashes are in [source R1 RED](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-preparation-phase-source-r1-red-2026-08-28.json).

R2: added the exact fixture rows and approved canonical native test join. The unchanged controller then exited 0:
- two preparation phases;
- six refusal states;
- allocation-before-reservation rejected;
- existing nativeOwnership neutral trace7/phaseAccess3/cancellation4 and liveRecord7;
- existing full TS resident suite debug census, strictTS=0.
Full actual output/nine stable selected pre/post hashes are in [source R2 GREEN](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-preparation-phase-source-r2-green-2026-08-28.json).

The third-party reference uses existing Ajv + Immer (plus existing Buffer/BigInt tests). Its phase state model does **not** execute Rust layouts or native grant arithmetic. Both source-run captures contain intermediate native draft `0b7079b701a84dec477d5b1a826c57bf388ae834527f360c89d0fb0b234f0ad8`; the final authority `23516f…` subsequently received only the reviewed drain/successor header-read narrowing. No claim is made that TS compiled either native draft.

## Preserved and prospective native boundary

[Source-only audit and full textual roster](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-implementation-source-audit-2026-08-28.json) retains current hashes and small provenance failures during packet assembly (a transient undefined local while rendering Markdown, an incorrect ASCII suffix before roster execution, and two nonexistent guessed fixture hash paths). These did not run tests or change production; corrected exact canonical leaf reads are retained separately.

Preserved sources:
- Canonical TS runtime: `72b0e0ba9bab57f7b95c988c40171e7237b0e5ca00e84b063df03e2c3edd6530`.
- Canonical capacity fixture/schema: `fd5e4114b67f00a22db17f6b5203f3e78ab4b3c72ae1365223040eaf88f89428` / `6a684a67751efb699db63d374dcc9375fc6f895785802d5c14949e8a57e617a0`.
- Primary neutral fixture/schema/controller: `61abe9bf4f41f6dea97ed67f68f044df5055263f88882f8cc4a2e842ba91f964` / `6ac52fcda67a17989d8f5b632f985046fbd8fb3ef7a4fe9e9e5fb07f019f1e04` / `c4a9fe89e5b06ae1ba6f9a0a344ebe558f59825250f4f3930d6f1dae97a8bcbe`.
- Opening7: `01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1`; full CUT1: `4c6b34304b5d9c0746b922d65d2c570bf31679f21ff0747c4b8ff6363f0b3217`.
- Release7: `8949cb8507c798758108a5b77d01221d8c87ff9d5feb2cd4f8522cca67d55019`; baseline1: `2e73f918e3100c7a232edf22032edfe87ab225ba2d40fa232c3814d5c4420c6f`.

The unchanged native router rejects all arguments and calls existing runCargoTestBudgeted with `["--lib"]`. Any future sole-executor request must therefore use the unchanged no-argument `@semio-tech/value-resident-rs:test` route for the entire prospective 32, with existing target/jobs/profile, not an invented selector. Root review and separate executor GO are required first.

The seven preserved primary names are:
- resident_primary_prepare_layout_and_all_short_frontiers
- resident_primary_lost_returns_keep_original_among_same_types
- resident_primary_partial_cancel_conserves_original_partition
- resident_primary_selected_allocator_null_keeps_reservation
- resident_primary_recovery_short_grants_keep_exact_node_pins
- resident_primary_paused_next_and_found_close_before_resume
- resident_primary_busy_foreign_wrong_type_replay_and_stale

No current compiler success, native grant/allocation/destructor result, all32 pass, actual RuntimeAppCell/Opening funding, Store FIFO receiver/detach, SyncSession ownership, return ABI, whole callback-tail, or generic unknown-fault disposal acceptance is claimed.

