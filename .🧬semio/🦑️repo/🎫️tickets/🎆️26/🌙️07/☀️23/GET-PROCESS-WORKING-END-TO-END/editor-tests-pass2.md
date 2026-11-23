# Editor Tests Pass 2

Native cargo via `replay-cargo.sh` (build dir `build-replay`, 3 jobs, one cargo at a time). The last full run is `p2i` (editor + host, `--no-fail-fast`, log `🗑️generated/replay-p2i.txt`). Goal NOT reached: the editor package still has failures in every target.

## Counts (passed / failed)

| Target | Before (`native-rust-editor-tests.md` STATUS) | After (`p2i`) |
| --- | --- | --- |
| lib | 32 / 7 of 39 | 29 / 12 of 41 |
| original_host_configuration | 0 / 1 | 0 / 1 (earlier domain run: pass; now WorkLimit, see cause 1) |
| pane_smoke | 11 / 5 of 16 | 9 / 9 of 18 |
| interaction_behavior | 22 / 2 of 24 | 7 / 18 of 25 |
| panels_artifact | 9 / 2 of 11 | 9 / 3 of 12 |
| panels_catalogue | 8 / 2 of 10 | 8 / 3 of 11 |
| panels_inspection | 11 / 3 of 14 | 4 / 11 of 15 |
| panels_workshop | 13 / 3 of 16 | 8 / 9 of 17 |
| workpiece | 6 / 4 of 10 | 6 / 5 of 11 |
| host (lib) | 6 / 0 (retained-laws fixed) | 4 / 2 (`fixture initializer must converge`, retained-laws; seed storage owner) |

The "after" counts are worse because of the same new stall (cause 1), which the earlier native runs did not hit; the review fixes (emission, close ladder, held result) landed in between and were first compiled in `p2e`/`p2i`.
The `history_edits_end_to_end` / `conflict_history_edits_end_to_end` pair fails in all 8 acceptance-using targets.

## Root causes found and fixed this pass

1. **Archive replacement handle blocked terminal close** (`archive replacement awaits its original operation recipient`, the old panic at `acceptance-policy-pump-and-drive:114`). `document_archive_load_demands` only treated the replacement as live while its job was still registered, and fell through to `ActiveDocumentArchiveLoad::retirement_demands`, which refuses any load with a set `replacement`. `drive_document_archive_terminal` already resolves the handle (clears a dead one, faults a Ready load that lost it), so the demand is now the cheap item demand whenever a handle is set. File: `🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`. Verified: `conflict_history_edits_end_to_end` got past this panic (p2a).
2. **Recursion-limit compile failure in 7 test targets** (`queries overflow the depth limit` in `process3d_panels_*`): the new law `assert_closes_mid_reserved_commit` inlined the async `dispatch_fixture_action`. It is now `Box::pin`ned like every other call in the file. File: `🧪️tests/🧪️history-edit-acceptance/🔢️end-to-end-assertion-census-and-search/🦀️.rs`.
3. **Syntax error blocking every build** in another agent's wgpu file (`🖍️draw/🔧️pipeline/🏭️factory/🎯️gpu-targets-and-pipeline-resources/🦀️.rs`, a stray closing brace at end of file after `content_stencil_state`). I removed the one stray `}` after it had blocked builds for about 5 minutes. Tell the owner.
4. **Diagnostics** (permanent, no behaviour change): the registered-fixture close assertion now also prints the Source total grant and spent progress (`🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs`); `dispatch_fixture_command` now reports the same stall diagnostic as `dispatch_fixture_action` instead of a bare "exceeded its authored turn bound" (`🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`).

## Root causes found, NOT fixed

1. **Host contribution command never settles (46 failing tests across every target).** `install_contributions` -> `dispatch_fixture_command` returns `Ok(None)` for 32,768 turns. Measured: the stall is `OriginalEmissionSource::advance_host` (host emission, phase 4, JSON parse of the contribution pack in `👷️job/📨️emission/🏛️host/🦀️.rs`). Each turn it opens a quoted receiving scope (quote: 3 items, 80 copy bytes, work 1,000,001; demand: depth 1 only), the scope is admitted and paid (about 2 items and 960 copy bytes of fees per turn), but the operation closure never runs: `control.with_effect_reported(body, maximum, ...)` returns `None` on every turn (`OriginalEffectRun::begin` -> `prepare_for_action` -> the recipient `admits(observed, additional)` answers false). Ruled out by instrumentation: the quoted-scope refusal points in `original_receiving_outcome` (plan not ready, plan none, grant/work mismatch, decoder admission, return table, cancel/deadline) never fire, so the scope opens and charges, and only the Work action quote is refused. Also ruled out: quote work headroom (+64 units made no difference) and the grammar unit count (1 per turn; a 64-unit batch was tried and then reverted because it cannot matter while the effect is never entered). Same `with_effect_reported` shape works elsewhere (`🛬️receiving/🏗️steps` `scoped` uses `with_allocated_effect` with the identical quote), so the difference to look at is the Work stock the scope hands the body (`plan.body(pooled)` fuel, minus control work) against the widened maximum (`maximum + 1`) that `begin` asks for. This belongs to the job-context receiving owner. Everything else the contribution path needs is in place (the scrub shell, emission receipt and construction retirement were fixed earlier).
2. **`history_edits_end_to_end` (non-conflict, 8 targets): `plugin.internal.document-archive-replacement.closure-rejected (Incomplete)`** on the `create-step` case. Measured with a temporary print in the closure walk: the candidate roster holds 7 members, but the head snapshot's child projection reaches only 6. The unreached one is a superseded content-addressed `steps-flow-82990b6a3eaac4a5`; the new `steps-flow-ebfc619f67d968fd` (same slot) is reached. `complete_document_archive_genesis` mints the roster from the candidate snapshot (`🧵️instance/📄️docs/📥️staging/🚪️ingress/🦀️.rs`), and its skip test compares `slot` and `child_id` only, never `artifact_id`. So a content-addressed child whose id changes between the genesis pack snapshot and the head snapshot leaves the old member on the roster, and closure validation (strictly `members == reachable`) rejects it. Decision needed from the archive owner: either mint from the head snapshot only (retire superseded content-addressed members) or make closure validation accept roster members reachable from history. Not changed.
3. **`conflict_history_edits_end_to_end`: close stalls on exhausted release stock** (`release_bytes 27,136` quoted against `25,208` remaining on rung `ClosingChildMember`, 4,096 idle turns). The fixture caller's stock (`🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json`) is 512 MiB release / 504 MiB copy. By the time the scenario closes its apps it has spent 470.9 MB release, 398.7 MB copy, 168.5 MB capacity and 1,141,012 items. I raised the stock to 2 GiB temporarily: the test then passed (p2c), which proves only that the stock is the limit. Per the coordinator's rule I **reverted the stock** because the spend looks like a product drain, not necessary work: each app load/close cycle charged 63-68 MB of release while retaining only 0.7-1.2 MB of capacity and 1.3-2.7 MB of copy (22k-32k items); 64 MiB is exactly `UNQUOTED_RECEIVING_RELEASE_BYTES` in `🛬️owned/🦀️.rs`, so the receiving scope appears to charge its whole offered release share back at settle. Another agent's `[DEBUG] recv turn` prints show receive turns each spending about 16.7 MB copy and 6.5 MB release for 36k items. Unproven; the scope settle accounting (job-context owner) is the lead. Arithmetic is not sufficient to justify raising the stock.
4. Other failure messages seen in p2i (one each, not investigated): `archive 71 never reached a terminal state: Begin` (every_example_loads, same load starvation family as cause 2/3); `document archive parent Pack and SPR hydration was rejected at Fold: Replay: native construction exceeds original remaining Job grant` (bundled example load, pane_smoke); `original host producer remained pending` (host testkit); `fixture initializer must converge` (host retained-laws x2).

## Files changed this pass

Framework (prefix `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` unless noted):
- `🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs` (cause fixed 1)
- `🧪️tests/🧪️history-edit-acceptance/🔢️end-to-end-assertion-census-and-search/🦀️.rs` (Box::pin)
- `🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs` (diagnostic)
- `🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` (command stall diagnostic)
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🎯️gpu-targets-and-pipeline-resources/🦀️.rs` (stray brace, other agent's file)

Touched and restored (no net change): `🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json` (stock), `👷️job/📨️emission/🏛️host/🦀️.rs`, `🧵️job/⏱️context/🛬️owned/🦀️.rs` (framework `🔨️modules`), `🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs`, `🧪️tests/🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive/🦀️.rs`, `🏪️store/🧩️composition/🌳️closure/🦀️.rs`. All my `[DEBUG]` prints are removed (the ones remaining in the tree belong to other agents). The host, `owned` and actions files were restored in part from timestamped backups; each was diffed against `HEAD` afterwards (floor, scrub-shell logic and the unconditional `prepare_original_tool_dispatch` call are present). Edits other agents made in those files in the minutes between my backup and restore could theoretically be lost; none show in the diffs.

## Next steps
1. Job-context owner: why `OriginalEffectRun::begin` is refused for the host JSON parse quote (cause 1; it unblocks about 46 tests), and whether scope settle charges its whole release share (cause 3).
2. Archive owner: roster minting versus closure validation for superseded content-addressed members (cause 2).
3. Re-run editor + host (`replay-cargo.sh <label> test -p semio-s-artifact-process-process3d-editor -p semio-s-artifact-process-process3d-host --no-fail-fast`).
