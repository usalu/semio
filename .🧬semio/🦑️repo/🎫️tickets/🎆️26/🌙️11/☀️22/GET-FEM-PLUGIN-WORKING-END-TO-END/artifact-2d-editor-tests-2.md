# FEM 2D editor tests, run 2 (successor)

Native cargo (private `C:\t\2de`, `C:\t\2deb`, `C:\t\2detmp`; scripts `b2de-run.sh`, `b2de-t.sh`). Final log `🗑️generated/b2de-e5.log`, rc=0.
Command: lib + `editor_app` + `editor_app_2` of `semio-s-artifact-fem-2d-editor`, plus interaction lib, with the owned-by-others --skip list.

## Result (e5)
- editor lib 18/18, `editor_app` 112/112 (4 skipped), `editor_app_2` 40/40 (5 skipped), interaction lib 29/29 (1 skipped). 0 failures.
- Run e1 (previous fixes only) had: preparation-laws, `setResultDisplay` wire bytes, two remove_selection tests failing; all fixed below.

## Fixes (paths under `...◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor`)
1. `🧪️tests/🔬️fem2d-artifact-preparation-laws/🦀️.rs`: the 32 extra released bytes are the two test-created `Arc` issuers (2 x 16-byte ArcInner) whose births happen outside the measured close loop; expectation now `allocated + capacity + issuers`. Product accounting was correct. `[DEBUG]` eprintln removed.
2. `🧪️tests/🔬️unit/🦀️.rs`: `setResultDisplay` wire pin re-sorted (string table is length-then-bytes ordered: `modal` first).
3. `🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🦀️.rs`: node/load-case ids were `SealedShared` clones dropped unretired (panic "sealed source requires original granted handback"): now `as_str().to_owned()`. The mounted publication test is not a product bug: the `protectedSelection` case (node with unselected referrers) is MEANT to be refused (`mutation.reference-receive` -> `interactive-job.publication-rejected`), the `dispatch` helper unwrapped the settle. Test now asserts the refusal and the unchanged snapshot.
4. `🧪️tests/🧰️context/🦀️.rs`: new helper `dispatch_refused`.
5. Temporary `[DEBUG]` in `🏭️retained-job-factories-and-preparation/🦀️.rs` added and removed (file unchanged net).

## Open
- Skipped (other agents): history replay, multi-user, playback, window config, `frame_cost_stays_flat` (UnsupportedOwner not re-examined).
- Someone else's `[DEBUG]` eprintlns remain in `🧬️schema/📸️snapshot/native.rs`, and in framework retirement/close code (output seen in logs).
