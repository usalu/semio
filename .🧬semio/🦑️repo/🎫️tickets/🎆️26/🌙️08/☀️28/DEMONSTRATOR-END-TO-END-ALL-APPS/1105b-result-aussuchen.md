STATUS: smoke RED (smoke-14): next framework fault "original lifecycle receipt exceeds its exact phase loan" (ownership/failure:57, 2 copied items vs loan), with runtime
- Check: `chk.sh aussuchen-1` manifest sourcing curation Cargo.toml, `--lib -p semio-s-artifact-sourcing-curation`; rc=1; log C/🗑️generated/1105b/aussuchen-1.txt:21 "files=1831/2048, bytes=16788269/16777216". Same fault in hub-1. Reported to main. No compile verdict (UNVERIFIED).
- Smoke test exists: sourcing curation editor 🧪️tests/🪧️pane-smoke/🦀️.rs (`[[test]] aussuchen_pane_smoke`). Last run (s10, 15:30): 0/1, `InvariantViolated: original return source exceeds its admitting lifetime stock`. Not rerun.
- No files changed.
- Next: rerun chk.sh after infra fix, then native `--test aussuchen_pane_smoke`.

## Static root cause: "original return source exceeds its admitting lifetime stock"
- Raised at 🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs:45 (`OriginalReturnBudget::bind`): spent (retained delta since `original_total_retained_start`) not within `original_total_retained`, or work > `original_total_fuel`, at the first bind of the return source.
- Cause in s10: the pane-smoke load (`load_document`) ran as ONE long turn on a single fixture StepContext, so the load's retained/CPU spend exceeded the single admitting stock. Not in sourcing crates; the sourcing smoke only calls `laws::assert_pane_smoke`.
- Since fixed in the harness: `load_document`/`plugin_load_document` (🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs:247,281) now call `testkit::renew_turn(cx, fresh, fresh_grant)` per poll turn (renew_turn at 🧵️job/⏱️context/🦀️.rs:501). So s10 is likely stale; a rerun decides.
- Residual risk (unproven): renew_turn's unbound branch (⏱️context:509) refreshes fuel but never `original_total_retained` or `original_total_work_start`, so work is measured from 0 against accumulated fuel; fails if fuel is consumed outside `work_units_spent` accounting.

## aussuchen-2 (rc=1): CargoLockRefused, log aussuchen-2.txt:191
`semio-framework-ui-declaration` not found: package at 🧰️framework/🔨️modules/🖱️ui/📜️declaration is named `semio-ui-declaration` (HEAD) and 🧰️framework/📦️packages/🦀️rust/🦀️.rs:42 uses `semio_ui_declaration`, but 🧰️framework/📦️packages/🦀️rust/Cargo.toml:21 still keyed the old name. FIXED: dep key renamed to `semio-ui-declaration`. Other crates may still use the old key (not scanned).
FIXED also ui-locale dep key (same rename).

## aussuchen-4 (rc=1): CargoLockRefused, log aussuchen-4.txt:156
`semio-os-store-member-open` not found; required by semio-os-kernel (🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:91,92,108) -> pulled in by sourcing curation examples crate. Packages are named `semio-framework-os-store-member-open` and `semio-framework-os-store-one-item-batch` (HEAD, 🏪️store/🧩️composition/🚪️open and 📬️one-item-owners), while the kernel keys and the Rust sources (e.g. 👥️space-member/📥️lifted/🏪️store/🦀️.rs) use `semio_os_store_member_open` / `semio_os_store_one_item_batch`. Rename sweep is inconsistent in both directions (the ui-declaration/ui-locale fixes above went to the unprefixed name). Direction is the coordinator's call; not edited. `semio-os-store-member-open-store` (line 92) likely has the same drift. Also line with `semio-framework-mutation-diff={path=...}` is oddly formatted (scan false positive).

## aussuchen-5 (rc=1, after lock-ok)
Cargo lock and resolver passed; source admission reached unit 44+/697. Refused: `semio-plugin-decl-app` (reason "files", remainingFiles 0, at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/📣️proofs/🦀️.rs). Known runtime-split item; waiting for the coordinator's note. Log: aussuchen-5.txt line ~38 of non-script output.

## aussuchen-6 (rc=1, after declapp-ok): compiler reached; 2 FOREIGN crates fail, no sourcing-owned error
Log C/🗑️generated/1105b/aussuchen-6.txt.
1. `semio-ui-contract-presentation`: `#[value(..)]` attribute not found at ui/🧬️contract/🖼️presentation/📦️packages/🦀️rust/../../🗺️surface/🦀️.rs:154,166 (struct SurfaceProps): the derive list lacks a derive owning `value` (compiler suggests CanonicalJsonTree/FromValue/NativeReceive/RetainedClone/... + ToValue).
2. `semio-os-channel`: 📡️spr/🧵️channel/📦️packages/🦀️rust/../../📦️archive/🛬️decode/🦀️.rs:18 `owner.receive::<DocumentArchivePack,DocumentArchivePack>(..)` needs 3 generics (T,O,E; io/⏱️control/🦀️.rs:66) and the closure must return `RetainedWorkOutcome` (E0107 + E0308 at :18-25). Work-accounting migration of this caller is missing.

## aussuchen-7 / aussuchen-8
- aussuchen-7 (rc=1): only foreign `semio-os-store-codec-registry` E0308 (store/📦️codec/🦀️.rs:62); fixed by value/energie lanes.
- aussuchen-8 (rc=1): only foreign `semio-os-io` 38 errors in 🚪️io/🧭️io-mechanism/🦀️.rs (:562,:599,:712 old 2-generic receive, map_err on RetainedWorkOutcome, ? in closures). Claimed by runtime at 12:22 in C/1105b-claims.md. No sourcing-owned error has appeared. Log aussuchen-8.txt.

## aussuchen-9 (rc=1)
semio-os-io is through. Remaining errors are foreign, claimed by koordinator 12:49: semio-plugin-guest-contract (turn-output:4 ControlledRetirement unresolved; cold-job:22 missing field returned_source) and semio-plugin-decl-app (lifted/retirement/birth:19 semio_framework_value_native_owned). No sourcing-owned error yet. Log aussuchen-9.txt.

## aussuchen-10 (rc=1)
guest-contract and decl-app are through. Foreign errors only (log aussuchen-10.txt):
- semio-plugin-reactor-jobs (2): plugin/⚛️reactor/💼️jobs/../../🦀️.rs:692,695 E0283 (claimed by flow 12:55).
- semio-plugin-instance-render (6): 🧵️instance/📥️staging/🏷️reserved-jobs/🦀️.rs:48,77,79 E0061/E0277/E0599/E0308 (claimed by statik 12:56).
- semio-plugin-runtime-exchange (2): ⚛️reactor/🏃️runtime/🔄️exchange/../🔀️plugin-exchange/🦀️.rs:310,313 E0308: NOT in claims.
- aussuchen-11 (rc=1): foreign semio-framework-io-sqlite-snapshot artifact/receiving:14 E0308 (store claim 13:01); my plugin-exchange:304 Arc edit not yet reached/verified.
- aussuchen-13: runtime-exchange passes; frontier stdio-binary-snapshot/stdio-semio-value-model (value-lane stdio sqlite claim). Asked main whether I take it.

## aussuchen-14 / aussuchen-15
- Ported (my edits): stdio-binary-snapshot (io/binary/snapshot receive_pack; io/sqlite/snapshot host sigs, native decode/encode, receiving project/reconstruct) -> aussuchen-14 wasm rc 0. stdio-semio-value-model (🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:62,64 wrapped in store::native_snapshot_turn): not among aussuchen-15 failures.
- aussuchen-15 (rc=1) frontier, foreign (value-lane stdio batch): stdio-deflate-snapshot (36), stdio-txt-schema (25; duplicate decode/encode_natural_file E0201 + E0107/E0053), stdio-semio-flow-io (3), stdio-semio-image-editor (3). Log aussuchen-15.txt.

## aussuchen-16 / 17
- aussuchen-16 rc 0: stdio-semio-flow-io + stdio-semio-image-editor FIXED (my edits: removed duplicate semio_natural_file! at image editor 🦀️.rs:185; semio_plugin::app::ChildEmitPreparation at flow editor child 🦀️.rs:12,14,17).
- aussuchen-17 rc 1: ONLY error left is SOURCING-OWNED: semio-s-artifact-sourcing-curation-domain, ✏️editor/👥️presence/🦀️.rs:31 decode_presence_native returned Result; trait ArtifactPresenceSnapshot now returns RetainedWorkOutcome<Self,NativeSnapshotFailure>. FIXED by changing the return type (body already delegates to store::decode_presence_json, which returns RetainedWorkOutcome). Verifying in aussuchen-18.

## aussuchen-19 (rc 1)
Only sourcing-owned error: semio-s-artifact-sourcing-curation-tests (mirrors editor files) E0046 missing `last_turn_work` in ArtifactStoreOneItemPreparation impls for SourcingCurationArtifactPreparation (✏️editor/🗄️artifact-store-preparation/🦀️.rs) and SourcingCurationConfigPreparation (✏️editor/📬️config-store-preparation/🦀️.rs). FIXED: new field `last_work` (0 on denial, 1 for an executed advance) + `fn last_turn_work`. Earlier presence fix (curation-domain) compiled. Verifying in aussuchen-20.
- Coordinator correction: last_turn_work is now ACTUAL work, mirroring stdio semio-brep preparation: the trait `advance` is a wrapper (denial/closing/cancel -> last_work 0, Blocked; already-refused or already-prepared -> 0 work; else `receiving.prepare_work(1)`, run `advance_step`, `receiving.commit_work(self.last_work)`); `advance_step` sets last_work=1 at each executed semantic unit (document/config sizing + candidate prepare; authority prepare_one_item), counted also when it fails. Both preparations (artifact + config). Needs aussuchen-21 to verify.

## aussuchen-smoke-1 (rc 1): compiled and ran, 0 passed / 1 failed (log aussuchen-smoke-1.txt:69143)
`pane smoke: the bundled example loads` -> InvariantViolated "original return source exceeds its admitting lifetime stock", retained_progress all 0. Same fault as s10 (before the harness got per-poll renew_turn), so it survives the renew_turn fix.
Raised at 🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs:45 (OriginalReturnBudget::bind). Since the retained delta is 0, the failing arm must be `input.work > input.fuel` (work = cx.work_units_spent - cx.original_total_work_start, fuel = cx.original_total_fuel; capture() at :11).
Suspects, not yet proven (needs a [DEBUG] print of spent/granted/work/fuel at :45):
 (a) 🧵️job/⏱️context/🦀️.rs:509 renew_turn, unbound branch: refreshes only original_total_fuel (+= refill) and leaves original_total_work_start at 0 and original_total_retained at the first-turn grant, while the bound branch (:508) resets all three. Work then accumulates across turns against fuel that is only topped up by `fuel - fuel_remaining`.
 (b) capture() (:11) takes work as work_units_spent - original_total_work_start without subtracting reserved_return_work, unlike :508 and original_total_fuel_remaining (:287), so reserved return work (:371-374) counts as spent work.
No sourcing-owned code is involved: the test only calls laws::assert_pane_smoke (🧪️artifact-app-laws/🪧️pane-smoke-law/🦀️.rs:16).

## aussuchen-smoke-2 (rc 1) [DEBUG] answer
capture(): work=653277 start=632974 -> 20303 <= fuel 32765; retained delta (736 items, 2976 B, 5449 cap, 478201 released) within granted (65533 items, 128 MiB copy, 256 MiB cap, 512 MiB release). The bind refusal is GONE after the renew_turn unbound-branch fix (job ⏱️context:509). Next failure: renew_turn bound branch -> "original return source with live claims cannot renew its turn" (job source/🦀️.rs:52): the load poll loop renews every poll while the job's return claim still holds currencies. [DEBUG] print removed.
Fix: artifact-app-laws fixture-dispatch-steps-and-document-loading/🦀️.rs `renew_load_turn` (skip the renew while a bound source holds claims/held currencies, else testkit::renew_turn) at the 4 renew sites. W-aussuchen-smoke-3 verifies. capture():11 vs reserved_return_work: reserved was 0 here, not exercised.

## W-aussuchen-smoke-3 (rc 1): native build failed before the test ran
Foreign: semio-plugin-decl-emission 🛬️lifted/📦️preparation/🦀️.rs:387 E0277 `dyn ErasedSnapshotRetirement/FactoryRetirementTicket/RetirementCursor` not Sync, from flow's 18:31 edit making ChildEmitPreparationOwner Send+Sync (claims line 18:31). My renew_load_turn fix is therefore still unproven. Rerun as W-aussuchen-smoke-4 once that settles.

## W-aussuchen-smoke-4 (rc 1): ran 4.9 s, load made no progress
"the document archive load never reached a terminal state" (1_000_000 polls, pane-smoke-law:16): skipping renew while a claim holds currencies (my renew_load_turn) starved the context of fuel, so every poll stayed Pending. Reverted renew_load_turn. Correct emulation of a fresh reactor context: renew the context's fuel/grant/windows always, restore the source's admitting stock only when quiescent. Implemented as OriginalReturnBudget::renew_turn_unless_claimed (job source/🦀️.rs) used by testkit renew_turn (⏱️context/🦀️.rs:503); a failed budget still refuses. W-aussuchen-smoke-5 verifies.

## Host vs testkit turn renewal (coordinator requirement)
Real host path: each reactor turn builds a FRESH StepContext via OriginalCallerSourceParent::borrow_context_with_return (🧵️job/⏱️context/🎟️source/🦀️.rs:47) which calls StepContext::attach_original_return_budget (🧵️job/⏱️context/🦀️.rs:281). For a bound return source that only re-windows the context from `budget.remaining()` (granted - spent) and re-marks original_total_retained_start / original_total_work_start. It NEVER calls OriginalReturnBudget::renew_turn, so the host never restores the lifetime stock: spent accumulates over the life of the source and a long load can exhaust it (bind() on an already bound source is a no-op, :43).
Testkit path (renew_turn, ⏱️context:499+): same re-windowing (:508) PLUS budget renew_turn (restore spent = origin.spent) for a quiescent source. That restore is a fixture-only generosity, i.e. a smoke can pass where a browser would stall on lifetime-stock exhaustion. The claim-held case (renew_turn_unless_claimed, keep stock) already matches the host.
Plan: after W-aussuchen-smoke-5 (restore-when-quiescent) reports, make testkit::renew_turn mirror the host exactly (re-window from remaining(), no restore) and rerun as W-aussuchen-smoke-6. Pass = host-faithful green; fail = the same stall the browser would show.

## W-aussuchen-smoke-5 (rc 1) and the host-faithful switch
smoke-5 still failed with "original return source with live claims cannot renew its turn": my renew_turn_unless_claimed only skips while claims are live and no failure is recorded, so the Err came from `failure.is_some()` — the return budget RECORDED A FAILURE (record_retained/record_work conservation break, source/🦀️.rs ~:60-70) and renew_turn masks it behind the claims message. Applied the host-faithful form: testkit::renew_turn no longer touches the budget (only re-windows from remaining(), as attach_original_return_budget does). The stored failure then surfaces from the next real operation with its own message. W-aussuchen-smoke-6 runs it.

## W-aussuchen-smoke-6 (rc 1), host-faithful renew
Real message now visible: WorkLimit "original return source CPU exceeds spent and held authority" (job source/🦀️.rs record_work: work + held_fuel > fuel). Earlier [DEBUG]: 20303 of fuel 32765 already spent at the first bind, i.e. the fixture's whole lifetime CPU stock = 32768 (🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json "fuel") is consumed by setup + one example load. The host-faithful variant caps cumulative CPU on the one bound return source at its first-turn fuel; the earlier restore variant only hid that until a claim stayed live over the whole load (then it failed too, the failure masked as "live claims cannot renew"). So either the bundled-example load costs > ~12.4k CPU units (per-poll cost) or the lifetime-stock design is too small for a multi-poll load; the browser would see the same WorkLimit. Added a temporary [DEBUG] poll print (harness load_document) to measure; W-aussuchen-smoke-7.

## W-aussuchen-smoke-7 numbers (poll print)
[DEBUG] load poll 0 work_spent=34693 fuel_remaining=0 total_fuel_remaining=0 state=Running. The context is out of fuel at the FIRST load poll: boot/setup already spent ~20.3k (first bind showed 20303 of 32765) and the first load poll burns the rest (>14k units) without finishing. Next: W-aussuchen-smoke-8 prints work_spent at admit / after new_app / after bind / law start / after document_text to locate the hot spot.

## W-aussuchen-smoke-8 (rc 1): where the fuel goes
work_spent / fuel_remaining: sourcing test admit=0, after new_app=0, after bind=0, law start=0 (fuel 32765), after document_text=0 (fuel 32765), load poll 0: work_spent=34693 fuel_remaining=0 total_fuel_remaining=0 state=Running.
=> Boot/new_app/bind/document_text cost ZERO work; runtime's proposed split of boot into its own caller changes nothing for this pane. The FIRST load poll (maintenance_step + poll_document_archive_load, which runs merge_document_archive when awaits_merge: 🧵️instance/🔌️plugin-app-impl/🦀️.rs:689-693) consumes a whole turn's fuel (32765) and returns Running, i.e. it yields cooperatively on fuel. Poll 1 then gets fresh per-turn fuel from renew_turn, but the one bound return source's lifetime CPU stock (origin.fuel = first-turn fuel = 32765, work already 34693) is exhausted, so record_work fails with WorkLimit. Thus the load costs more than one original call's authority (32768) in total; with host-faithful renewal the stock cannot be topped up between polls.
Decision needed (coordinator/runtime): (a) size the stock per law through the caller input (runtime: fixture number, not a production limit) after measuring the true load cost, or (b) make the load a ContinueOriginalCall chain that hands fresh authority per poll. I have NOT raised any fuel. Temporary [DEBUG] prints remain in: pane-smoke-law/🦀️.rs, sourcing pane-smoke test, fixture-dispatch load_document (listed in C/1105b-claims.md).

## W-aussuchen-smoke-9 (rc 1), fuel 2,000,000 (runtime's value): it is a SPIN, not a cost
[DEBUG] load poll 0 work_spent=1999995 fuel_remaining=0 total_fuel_remaining=0 state=Running; poll 1 work_spent=1999996 fuel_remaining=0 state=Running; then failure; 3.56 s.
With 2,000,000 units the first poll again consumes ALL of it (1999995), exactly as it consumed all 32765 before. So the first load poll does not have a finite cost that a bigger stock would cover: something inside maintenance_step/poll_document_archive_load loops, burning one unit per iteration until fuel is 0, and never completes the load (state stays Running). No fuel sizing fixes it; the committed fix is in that loop (and the load should yield within the turn grant). W-aussuchen-smoke-10 adds per-call work prints (maintenance_step vs poll_document_archive_load, step variant) to pin the loop.

## W-aussuchen-smoke-10 (rc 1, fuel 2,000,000) — spin located
- maintenance_step in poll 0: work=1999994, step=Progress{copied_items:36887, copied_bytes:820356, retained_capacity_bytes:629790, released_bytes:4129938}. poll_document_archive_load in poll 0: work=0. Poll 1: maintenance_step work=0 Progress(0...), poll work=0, state Running. => the spin is entirely inside the maintenance ladder (runtime's suspect), not in poll_document_archive_load / merge.
- runtime-gate ladder lines (first 6 iterations): stage 18 (unit copied_items 1, copied_bytes 8), stage 24 (1, 0), stage 18 (1, 8), stage 24 (1, 0), stage 18 (1, 8), stage 24 (1, 252); fuel 1999963 -> 1999833 (~35 and ~20 units per unit). The ladder ping-pongs between stage 18 and stage 24, each answering Progress with copied_items 1 and never reaching a terminal state; the loop only ends when fuel is 0.
- Boot/new_app/bind/document_text: 0 work. No sourcing-owned code on this path (the test only drives the generic pane-smoke law).

## W-aussuchen-smoke-11 (rc 1) on runtime's ladder fix: spin GONE, new failure after 0.22 s
The 2M-fuel spin no longer happens (test finished in 0.22 s). New fault at the first load: InvariantViolated "original lifecycle child receipt differs from its whole producer receipt" (🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs:41-43 original_lifecycle_unposted). Runtime's own print in the log (aussuchen-smoke-11.txt:69357): whole=progress{0,0,0,0} before={items 2048, bytes 541470, cap 672382, released 4129938} after={items 2049, ...same} => a child effect posted ONE copied item (2048 -> 2049) while the whole producer receipt of that lifecycle step is all zero. The first-load work_spent print was never reached (load failed before returning). All my [DEBUG] prints removed. Framework (runtime) fault; sourcing code not involved. Sent to runtime; waiting for the fix before rerunning (W- label).

## W-aussuchen-smoke-12 (rc 1): backtrace of the receipt mismatch (log line 69650, grep "runtime-gate unposted")
Call chain, innermost first (frames 3-16, frames 0-2 are backtrace/closure machinery):
 3-6  🏛️ownership/⚠️failure/🦀️.rs:43:297 -> :43:127 -> :44:40  original_lifecycle_unposted
 7    ownership/⚠️failure/🦀️.rs:59:16  original_lifecycle_step::<LadderCtx<EditorApp<SourcingCurationApp>> ...>   <- inner step, whole receipt zero
 8    plugin-instance-ladders/🦀️.rs:374:9   <LadderCtx<EditorApp<SourcingCurationApp>> as ...>   <- the producer that answers the empty whole receipt after nested work posted
 9    plugin-instance-ladders/🦀️.rs:143:34
 10   plugin-instance-ladders/🦀️.rs:122:82
 11   plugin-instance-app 📥️staging/🪜️maintenance-ladder/🦀️.rs:47:44
 12   ownership/⚠️failure/🦀️.rs:56:305  original_lifecycle_step::<VcsArtifactApp<EditorApp<SourcingCurationApp>>>   <- outer step
 13   maintenance-ladder/🦀️.rs:45:22
 14   maintenance-ladder/🦀️.rs:32:33
 15   plugin-app-impl/🦀️.rs:230:14  maintenance_step
 16   fixture-dispatch-steps-and-document-loading load_document 🦀️.rs:252:58 (maintenance_step in the poll loop)
Producer to fix: plugin-instance-ladders 🦀️.rs:374 (reached via :143 and :122 from the maintenance ladder).

## W-aussuchen-smoke-13 (rc 1): infrastructure, not the test
Died at `Compiling proc-macro2`: "failed to write .../prototype/build/proc-macro2/.../fingerprint/invoked.timestamp: No such file or directory" — the shared build dir was pruned/swept under the running build (disk was fine: 109 GiB free). Not related to the pane. Rerun as W-aussuchen-smoke-14 after lock-16 (rc file present).

## W-aussuchen-smoke-14 (rc 1) on runtime's lifecycle fix + heap-witness allocator
Past the receipt-mismatch fault. New first-load fault (0.35 s): InvariantViolated "original lifecycle receipt exceeds its exact phase loan", retained_progress/effects {copied_items: 2, 0, 0, 0} — raised at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs:57 (`if !progress.fits(grant)` in original_lifecycle_step): a maintenance-ladder step reported 2 copied items against a phase loan that admits fewer (loan items 0/1). No runtime-gate print fires on this path; needs the same backtrace treatment. Reported to runtime. Log: W-aussuchen-smoke-14.txt (grep "exact phase loan").
