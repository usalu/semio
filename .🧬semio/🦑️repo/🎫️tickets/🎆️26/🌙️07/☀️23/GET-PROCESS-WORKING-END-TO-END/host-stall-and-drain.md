# Host Contribution Stall and Release Drain

## Goal 1: host contribution stall

Instrumenting `OriginalJobWorkDriver::admits` showed the work quote is not refused. Every receiving scope was admitted with `observed=0` and `additional == fuel`, so `OriginalEffectRun::begin` is fine and the "begin refuses" diagnosis of `editor-tests-pass2.md` no longer holds in the current tree. The real failure is one step later.

Root cause: `JsonGrammarCursor::step_source` runs up to 64 grammar units per turn. The host funds the turn for the depth of the first unit's frontier (`normal_step_demands`). Each later unit in the same step quotes `frames.len()+1`, which grows with nesting. `normal_permit` answered that with a hard `DepthLimit` error ("JSON normal frontier exceeds original admitted depth") instead of ending the turn like every other axis (items, copy, capacity, release). The host contribution command therefore failed or retried instead of settling.

Fix (`🎒️pack/🔤️json/🧮️parser-and-grammar-cursor/🦀️.rs`, `🎛️controlled-and-borrowed-bridge/🦀️.rs`): `normal_permit` takes `started`. When the step has already made progress, a depth shortage ends the turn (`Ok(false)`); the next turn quotes the deeper frontier. A shortage on the first unit remains a typed `DepthLimit` refusal.

Law: `🎒️pack/🔤️json/🧪️tests/🪜️frontier-depth/` (fixture `🔣️.json`, test `frontier-depth-law`, registered in the package `Cargo.toml`). It drives nested documents with a grant whose depth is exactly the first unit's demand, 64 units per turn, and compares the value against `serde_json`. Verified red without the fix ("turn 1 refused as DepthLimit") and green with it.

Futile fee: `original_receiving_outcome` (`🧵️job/⏱️context/🛬️owned/🦀️.rs`) used to admit the scope and pay its setup and control fees, and only afterwards refused a quoted scope whose grant or work exceeded the turn. The refusal check now runs before admission, so such a scope charges nothing. Law: `receiving-floor.rs::a_quoted_receiving_scope_beyond_the_turn_is_refused_at_no_charge` with fixture `receiving-quote-refusal.json` and schema, plus a TS twin in `🟦️.ts`. Green; the negative run (red without the early check) was not executed.

Results (`process3d_panels_catalogue`, `--test-threads=1`): before 7 passed / 4 failed; after 9 passed / 2 failed. Fixed: `closes_mid_reserved_commit`, `conflict_history_edits_end_to_end`. Still failing: `history_edits_end_to_end` (the closure-rejected archive cause) and `catalogue_lists_workshop_wood_machines` ("original page allocation attempt is occupied", `member.publication`; a second allocating action inside one effect, the same family as the pack receiving batching fix). Not investigated further.

## Goal 2: release drain

The 63-68 MB release per load/close was not scope settle accounting. In the stalled run the host contribution command failed and was retried against the same Source on every turn, and each attempt paid its receiving scopes. With the depth fix `conflict_history_edits_end_to_end` passes with the fixture stock unchanged at 512 MiB. No stock change was made. I did not measure the per-load release after the fix.

## Not done

- `semio-os-store-member-open-store --lib` (`seed-cargo.sh r1`, 1 passed / 3 failed) is unchanged by the depth fix and no longer shows the Pending stall from `record-open-final.md`. The two receive tests now fail with "native receiving original type, field or phase is invalid" after 26 items / 4163 B of progress (`🫴️record/🧪️tests/🦀️.rs:62`), and the genuine test fails with "original member open close refused" (`:24`). That is the struct-derive multi-part seeding area from `receiving-floor-verification.md`, not the receiving-work admission. Not investigated.
- `semio-framework-job` (framework workspace, `--features testkit`, `--no-fail-fast`): `original-native-progress` is 9/9 with the new law. `original-close-context::archive_close_defers_a_narrow_child_turn_without_spending_the_source` still fails (left 4189280, right 932), which `receiving-floor-verification.md` lists among the close-context failures that predate this work. The package is not in the root workspace, so `stale-cargo.sh ... -p semio-framework-job --features testkit` from the root refuses; run it from `🧰️framework`.
- All `[DEBUG]` prints I added are removed (`🛬️receiving/🎟️work`, `🎟️state`, the JSON cursor).

## Follow-up pass

### Job close-context regression check
`original-close-context::archive_close_defers_a_narrow_child_turn_without_spending_the_source` does not use any receiving scope (`grep receiving` on its file: 0 hits), so my early refusal in `🛬️owned` cannot affect it. It fails in `OriginalCallerChildSource::with_context` (the narrowed child ceiling is not applied: left 4189280, right 932). `🎟️source/👶️child/🦀️.rs` and its fixtures carry another agent's uncommitted edits (`admit_step` unborn-admission, `borrowed_cancel`). The failing path is theirs. `original-native-progress` is 9/9 (the new law included). My scope-plan file edits are none: `🎟️scope/🦀️.rs` was changed only by the other agent.

### Seal cursor, "original page allocation attempt is occupied" (wood machines, now 48 tests)
Backtrace (temporary, removed): `PagedList::reserve_page_using_for_limit` <- `reserve_one` <- `ArtifactCanonicalJsonTreeCursor::advance_one` <- `advance` <- `ArtifactCanonicalEditSealCursor::advance` (`📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`, the `for _ in 0..SEAL_TURN_BYTES` loop) <- `NativeBatchDigest` <- `advance_apply_batch_inner`. The loop calls `tree.advance` repeatedly inside one effect; once an iteration allocates a frame page, the next iteration's reservation finds the effect's allocation marker occupied. Same family as the JSON cursor's `if effect.actual().is_some(){break;}`.
Tried: breaking the loop on `effect.actual().is_some()`, and on `step_progress.retained_capacity_bytes!=0`. Both remove "occupied" but turn every affected test into "seal canonical second pass length changed" (46 tests), so the Count pass and the hash pass diverge when a turn is cut after a page allocation. Not understood yet; both variants were reverted, the tree is back to the original loop. This is the next root cause: find why the second pass hashes a different length once a turn ends at an allocation (suspects: the Count pass `buffered=false` window 0 versus the hash pass `page_length` window, and `hash_pending` being skipped by the early break).
Law added (fails at baseline for an unrelated reason): `🪜️ a_wide_work_turn_seals_the_same_digest_as_a_one_step_turn` in `paged-digest/🦀️.rs` with fixture `📄️wide-work.json`. The whole `paged_seal_digest` target already fails at its heap-witness assertion (left (392,0), right (0,0)) without my change, so the law is unverified.

### Fixed call site
`process3d_workpiece_window` (`🪚️workpiece/🧪️tests/🔬️unit/🦀️.rs:68`): `render(...)` gained a `Locale`; the call now passes `semio_plugin::Locale::En`, as the sibling call at line 107 does.

### Full editor package (`replay-cargo.sh full6`, `--test-threads=2`, break reverted), passed / failed
| Target | Passed / failed |
| --- | --- |
| lib | 26 / 15 |
| original_host_configuration | 1 / 0 |
| pane_smoke | 8 / 10 |
| interaction_behavior | 6 / 19 |
| panels_artifact | 8 / 4 |
| panels_catalogue | 7 / 4 |
| panels_inspection | 3 / 12 |
| panels_workshop | 7 / 10 |
| workpiece | 5 / 6 |

Failure messages: 48 "original page allocation attempt is occupied", 27 "captured shared original changed its Work or destination", 2 "JSON parse work quote was not admitted", 1 "original lifecycle child receipt differs from its whole producer receipt", 1 "host lifetime fuel exhausted across fixture turns". `conflict_history_edits_end_to_end` and `history_edits_end_to_end` are among the failures in this run, so the earlier green `conflict_history_edits_end_to_end` (run `e1`) no longer holds; the failure here is the "occupied" seal error, not the release drain.

Host package: `semio-s-artifact-process-process3d-host --lib` does not compile (`retained-laws/🦀️.rs:74,348` use `semio_plugin::ArtifactColdClose`, which no longer exists). Another agent's API change; not touched.

Disk: the build volume hit ENOSPC during one link; I deleted my own old logs under `🗑️generated` (replay-*, stale-*, seed-rec*), which freed about 1.3 GB.
