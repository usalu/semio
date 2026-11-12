# Verify wave 6 (verify10-)

## Outcome
No repo run executed. The one run I queued (`verify10-intrinsic.log`: `test -p semio-framework-pack-intrinsic`) never left the host admission queue (3018 MB needed, behind workspaces-check / job-receiving). BUILD PAUSE #5 arrived while it was still queued, so I cancelled it (not started, nothing to lose). No source edits were made in this wave. No `[DEBUG]` lines exist.

## Coordinator note: replication `paged_seal_digest`
`generous()` in `📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests/📄️paged-digest/🦀️.rs` is already `RetainedCloneGrant{maximum_items:1,..STOCK}` (derived from the test's stock, depth 256). Its turns run through `drive` (`🔗️source/🧪️testing`), whose private bank is larger (depth 1024), so the grant cannot exceed it; no `within_stock` was needed. Still unrun.

## Code-reading findings (all unverified, no run)
- encode-cursor deflate close: the quote (`retirement()` in `🗜️deflate/🛫️encode/⏳️cursor`) and `PagedList::release_empty_page` use the same helpers (`next_release_*`), so a quote/check off-by-one is not visible by reading. Remaining candidates: (a) `release_empty_page` also denies when `effect.maximum_work_units()==0`, and the quote (`RetirementDemand`) has no work field, so the work funding of the nested effect (`nested` = remaining work of the parent turn) is the suspect; (b) `release_page` reports `progressed:true` with `released_allocation_bytes` 0 for a leaf with reserved slots but zero capacity, which matches the observed "copied_items 1, released 0" and means `PagedList::terminal_is_empty` (root capacity, length, capacity, allocated all 0) may never be reached if `allocated` is not reduced by the page-vector bytes. Check: `allocated` bookkeeping versus `released_allocation_bytes` for the root `Vec<Page>` release.
- preflight spin: the test never prints between turn 2000 of case 0, so the hang may be anywhere after that point of case 0 (enum sort `sort_turn`/`comparison`, classification/refinement), not necessarily case 1. `classify_turn`/`refine_turn` converge by reading; `comparison` offset logic looks sound. Needs an instrumented run: `[DEBUG] ` `eprintln!` in the test loop (outside `turn`, allocation-free) printing case/turns/`stage_for_test()` every 100 turns after 2000.
- pack-intrinsic close stall: the test `close()` loop (`🎮️retained/🧪️tests`) has a 100000-step bound and uses `original_close_demands(4096)`; which of the 12 tests stalls is unknown (log never produced).
- io-control-testing, replication/replication-causal, io-sqlite 6 failures: not examined.

## Not done (need builds)
Step 1 re-runs, step 3 typegen + async-tests, step 4/5 crate sweeps, ✏️s `allocation_stage` migration.

## Resume order (one run at a time, `--no-fail-fast`, per-test timeouts)
1. pack-intrinsic (alone) -> identify stalling test, instrument.
2. encode-cursor with `[DEBUG]` on `release_empty_page` Denied reason (work/depth/copy/bytes) and `allocated` before/after.
3. preflight with the loop prints above.
4. replication, replication-causal, io-control-testing, io-sqlite, then typegen/async-tests and the sweeps.
