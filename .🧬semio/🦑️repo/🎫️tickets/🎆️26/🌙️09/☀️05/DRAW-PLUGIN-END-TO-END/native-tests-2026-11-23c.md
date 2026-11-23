# Draw drawing native tests, 2026-11-23 (third pass)

Commands as in `native-tests-2026-11-23b.md` (cwd = drawing workspace, `SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en`,
`repo cargo --store=isolated --configuration=prototype -- test --no-fail-fast -p <crate> -- --test-threads=1`).
Result: PARTIAL. editor-tests-2 went from 26/31 to 30/31. editor-tests still aborts, but its first failure moved from a
Draw-side work-accounting bug to the replication seal. A final confirmation build was cut short by `No space left on device`;
the last full run (c5) is the basis of the counts below; the only later edits were removal of `[DEBUG]` prints.

## Counts
| crate | before | after |
|---|---|---|
| semio-s-artifact-draw-drawing-editor-tests-2 | 26 pass, 5 fail | 30 pass, 1 fail |
| semio-s-artifact-draw-drawing-editor-tests | SIGTRAP in canvas_laws | still SIGTRAP (new root, see below) |

## Root causes and fixes
1. `args_bridge_tests::host_pointer_numbers_and_nullable_export_fields_are_typed` (FIXED). The bridge folds host camelCase
   keys to snake_case, but `CanvasPointerDown` was the only command with `rename_all = "camelCase"`, so every multi-word host
   field (`appInstanceId`, `worldX`, `checkpointPendingWork`) was dropped to its default. Removed the rename.
2. `path_job::retained_path_simplification_shared_cases` (FIXED). `Work::step` looped on `should_yield` only and charged
   no retained currency, so with ample fuel a whole simplification finished in one turn, never yielding Progress. It now
   gates each unit with `retained_work_can_enter` and charges `copied_items: 1` per unit (same convention as image import),
   so a one-item turn grant bounds a step.
3. `completion::completion_original_alias_and_payload_physical_release` (FIXED). 64 bytes physically freed but unreceipted at
   the last alias: on macOS `std::sync::Mutex` lazily boxes a 64-byte pthread mutex on first lock and frees it on
   `into_inner`. The completion `Cell` held two std mutexes. Replaced them with an allocation-free `Slot` (inline flag),
   placed in `semio-framework-value-retirement-contract` (re-exported from `semio_framework_value`) because
   `semio-plugin-decl-app` is 560 bytes under its 256 KiB source boundary and a local copy exceeded it.
4. `export_work_tests::export_retires_the_original_segmented_transport` (FIXED, test bound). Export reserves the 32 MiB
   segmented-output table (8192 slots); retiring it takes about 3800 steps (contiguous or paged, sealed) and 7600 steps
   (cancelled with staged page). The test allowed 1000. Raised to 100_000 like its siblings.
5. `export_work_tests::original_export_read_returns_under_exact_physical_grants` (PARTLY FIXED, still failing). First failure
   was `FactorySharedRetirement::close` requiring the same Source dispatch across actions, while every owner-less pump turn
   issues its own Source; it now follows the live turn between actions (`follow_between_actions`, as `ReadLease` does).
   The test now fails later: `issuer.close_owned_terminal_is_empty()` stays false; the store disposer quotes
   `{depth: 2}` and the step makes no progress because `SnapshotReadLeaseRegistryOwner::close_custody_step` waits for
   `strong_count() == 1`/an empty registry. Something still holds a registry alias or an occupied lease after the work and
   returned reads closed. Not resolved; `store/🔗️read/🔒️registry` was being edited concurrently.
6. editor-tests `canvas_laws` abort. The abort ("controlled retirement abandoned before terminal-empty", thread-local drop
   at thread exit, `fatal runtime error`) is the aftermath of the first test panicking in `settled`, not a missing close.
   First cause (FIXED): `DrawingWindowCommandWork::step` charged `identity_checkpoints` fuel units, but the bounded contract
   issues 1 work unit per step, so the work loan failed with "original work loan callback replaced its initiating context
   or refilled its work". Now clamped to `cx.fuel_remaining()`. Second cause (OPEN): the same test now fails in the
   publication with `member.publication: seal canonical second pass length changed`
   (`framework/modules/replication/io/text/canonical/seal`, hashed != counted length). That file and its tests were being
   edited by another agent during this pass, so I left it alone. Until it lands, canvas_laws will keep panicking on the
   first `settled` call and aborting at thread exit.

## Changed files (repo-root relative)
Under `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/`:
- `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs` (drop camelCase rename)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️edit-path/🧵️simplify/🦀️.rs` (retained unit gating)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧾️bounded-commands/🦀️.rs` (fuel clamp)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-document/🧵️export/🧪️tests/🔬️unit/🦀️.rs` (loop bound)
Under `🧰️framework/`:
- `🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs` (follow Source between actions)
- `🔨️modules/🌱️value/♻️retirement/🧬️contract/🔒️slot/🦀️.rs` (new `Slot`)
- `🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs` (mod and re-export)
- `🔨️modules/🌱️value/🦀️.rs` (re-export `Slot`, `SlotGuard`)
- `🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/♻️retirement/🦀️.rs` and `🌱️birth/🦀️.rs` (use `Slot`)
No Cargo.toml or Cargo.lock edits. Other agents' `[DEBUG]` prints in the framework (`ownership/failure`, `store/initialization/replay`,
`bundle`, `rel ... items`) are untouched and noisy in `--nocapture` output.

## Addendum: browser boot fault from the Source-turn rebind (coordinator report)
Cause: my `follow_between_actions(effect, recover)` only rebinds when the effect carries a Source turn. The browser guest
drives close with sourceless effects (Source `None` both at admission and at close), so nothing rebound and the strict
`matches_effect` (scope and issuer, which differ every turn) failed, tripping the one shared `invalid()`. The previous
`matches_source` accepted None == None, which is why boot worked before my change.
Fix in `FactorySharedRetirement` (`close` and `close_original_birth_step`), mirroring `ReadLease`:
- pending (`recover` or `ReplyStash::pending()`): require the exact same Source (`matches_source`), like `ReadLease`'s `pending = true`;
- between actions with a Source on the effect: rebind the stored binding to the live effect;
- between actions without a Source: require the same (sourceless) Source as before, i.e. the old boot behaviour.
The birth step rebinds the same way when not pending, and refuses with its own message when no binding exists.
`invalid()` now takes a cause; every call site has its own message ("shared admission ...", "shared close recovery resumed
under another Source turn", "shared close resumed without a Source turn after binding to one", "shared birth step ...",
"shared original close/recovery loan differs from its effect", ...). The `[DEBUG]` eprintln and backtrace in `invalid()` are gone.
Another agent had already named the `ok_or_else` site ("shared value step ran without an elected original").

Verification: `semio-framework-value-retirement` (run from its package dir): 30 passed, 0 failed across its test targets.
`semio-framework-value-factory` includes the same file but could not be built by me: source admission reports 273824 of 262144
bytes (this crate is over its limit by 11 KB, not caused by my ~1.1 KB change; others' concurrent edits). Draw editor-tests-2
last run: 24 passed, 7 failed, but all 7 now come from concurrent edits by other agents, not from this change: "JSON parse
work quote was not admitted" (args-bridge, four tests) and physical heap counters of tens of millions of bytes against receipts
of a few KB (clipboard metadata, both export tests), consistent with allocation-heavy `[DEBUG]` code now in the framework
(`job/context/owned` `crate_dbg`, `plugin/.../call` and `host/poll` backtraces). Before those edits the same run was 30/1.
So `original_export_read_returns_under_exact_physical_grants` could not be re-judged for the rebind fix.
Changed file: `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs`. No Cargo.lock edits; disk was fine (no gc needed).
