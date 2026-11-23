# Job, retirement and native-owned test repair

Builds: `stale-cargo.sh <label> ... --manifest-path 🧰️framework/Cargo.toml`. The shared build lock was held by other agents for 20 to 60 minutes per run, so the final re-runs (`stale-jrx3.txt`, `stale-rtx2.txt`, `stale-ds-*.txt`) may still be queued; verification status is stated per item.

## 1. semio-framework-job (testkit)
- Test code compiled again by the time of the first run (`stale-jr1.txt`, `--no-run` ok); the compile breakage had been resolved by concurrent edits. First full run (`stale-jr2.txt`): 5 failures.
- `original-close-context::archive_close_defers_a_narrow_child_turn_without_spending_the_source` (left 4189280, right 932): `StepContext::original_total_retained_grant` (`🧵️job/⏱️context/🦀️.rs`) ignored the turn's issued ceiling whenever a return budget was bound, so a narrow child turn saw the whole Source stock. It now takes `min(total remaining, turn ceiling remaining)` first and then the bound budget. Law (the test) is correct; `original_source_total_grant` keeps the Source-wide view.
- The other four (construction over-quote, worker payload, page-return contiguous bytes, retained-loan) were off by about 108 B born (e.g. `(3008,108)` vs `(2800,0)`, `(500,0)` vs `(392,0)`): same cause as item 2.
- `original-native-progress` was 9/9 in that run.

## 2. semio-framework-value-retirement and the 108 B
- Cause: `♻️retirement/🧭️support/🦀️.rs` had gained a thread-local `RefCell<HashMap<TypeId,bool>>` memo of `controlled_retirement_supported`. `ControlledRetirement::new` calls it, so the first construction per type and thread allocated the map (108 B), contradicting the module's own "without allocating" contract. Decision: construction must be allocation-free (the laws are right, product was wrong).
- Fix: the memo is a const-initialised 64-slot direct-mapped `Cell<Option<(TypeId,bool)>>` array (no heap, no lazy init).
- Second failure (`controlled_tests::original_controlled_failed_reservation...`, heap `(400,144)` vs `(0,0)`): a stray `eprintln!("[DEBUG] Retained close completed with owners ...")` in `admit_retained_clone_close` (`🎟️control/🧬️contract/📏️physical/🦀️.rs`) allocated inside an observed scope. Removed.
- Before the fixes `stale-rt1.txt` showed 1 failing target (the second case); the other earlier failures were the memo.
- Open (not fixed, not mine to fix blind): the presence agent traced 208 to 408 B of unreceipted heap to the first cursor page (`PagedList::with_page_items(8)` in `ControlledCore`) during the first source close step (`🧬️retained-clone/🔗️source/🦀️.rs` `RetainedOwnerGate::new(ControlledRetirement::retain(owner))`). That page is allocated lazily at the first `reserve_cursor_slot` push, and the reservation is only receipted on the funded `reserve_cursor_slot` path; the source close takes a path that pushes without it. Needs a dedicated trace.

## 3. semio-framework-value-native-owned
- Cause: `🔗️sealed-shared-receiver/🧪️tests/🧫️fixtures/` held the data in `🔣️.json` (and `🧬️schema/🔣️.json` the 2020-12 schema) while the Rust driver reads `🧫️fixtures/📐️.json` as data. Layout restored consistently with the other owners (data `📐️.json`, schema `🧬️schema/🔣️.json`): fixture file renamed to `📐️.json`, TS twin (`🧪️tests/🟦️.ts`) import updated. The 6 failures dropped to 2 (`stale-no1.txt`).
- Remaining 2 in `native_shared`: `native_flattened_ordinals_refuse_ambiguous_declared_ids` (`Field(0)` instead of `Ambiguous`; flatten matcher in `🧩️record`, off-limits) and `native_dsl_enum_ordinals_accept_only_non_negative_u32_numbers` (heap `(110091,77259)` vs `(0,0)`: looks like cross-thread allocator noise from the parallel tests, the new thread-attributed memory reading in `⏱️trace/🧮️memory` is the intended cure; not re-checked single-threaded).

## Status after the re-run (`stale-jrx3.txt`, framework workspace, testkit)
- Green now: `original-close-context` incl. `archive_close_defers_a_narrow_child_turn_without_spending_the_source`, `original-native-progress`, the retained-loan test (its 108 B is gone).
- Still red (3), all heap-receipt drift and not the 108 B memo: `original_construction::over_quote_preserves_the_born_candidate...` (`(76864,0)` vs `(77064,24)`), `original_worker_payload::genuine_worker_preparation_uses_the_admitting_ledger...` (`(715,315)` vs `(0,0)`), `original-page-return::original_contiguous_bytes_keep_exact_birth_fill_and_return_on_the_same_source` (`(16681,72)` vs `(16385,0)`: 296 B extra born, 72 released). The 208 B cursor page plus 88 B matches the presence agent's unreceipted page; the reserve path itself is receipted (`reserve_cursor_slot_with`), so the allocation happens somewhere the source close does not account for. Needs a probe-based trace (no printing in observed scope).
- `stale-rtx2.txt` (retirement) and `stale-ds-*` (channel-archive, store-codec-registry, space-collection `--lib`) were still blocked on the shared build lock when I stopped; no result for them.

## Final re-run results
- `semio-framework-value-retirement` (`stale-rtx2.txt`): all result blocks ok, 0 failures.
- `semio-os-channel-archive --lib` (`stale-ds-semio-os-channel-archive.txt`): 5 passed, 0 failed.
- `semio-os-store-codec-registry --lib` and `semio-framework-artifact-space-collection --lib`: still building/queued at the time of writing (`stale-ds-*.txt`).
- Final downstream: `semio-os-store-codec-registry --lib` 9 passed; `semio-framework-artifact-space-collection --lib` 22 passed / 7 failed, and the failures are no longer a 108 B allocation: `boxed_custody` and `cancellation` expect "relational SQLite transfer cancelled" but get "native encoding/decoding canceled"; `cumulative` heap `(232202,198106)` vs `(34617,521)`; `refusals` progress zero vs 10 items; `builtin_creation...` "captured shared original changed its Work or destination"; `preflight_admits_actual_owner` "snapshot retirement exceeds original physical grant". These belong to the forwarded-port / snapshot work, not to construction allocation.
