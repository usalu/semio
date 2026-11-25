# 🩹️ MediaIn regression: "the shell received no typed-operation result page" (2026-11-25)

Follows `🏁️media-in-terminal-publication.md`, Round 2 of `🖼️pdf-picker-and-small-turn-save.md` and section B of `🧱️pdf-descriptor-and-retirement.md`. Every cargo run went through `run-media-in-regression.sh` (`watchdog-cargo.py` + `native-cargo.sh`, one cargo, filtered). Built binaries were rerun directly without cargo. No wasm rebuild.

## Reproduction

The four test binaries still in `🗑️generated/native-target` (built 05:54, 06:43, 07:25 and 07:44) were rerun with the shard-budget filter while the wasm build held the machine. They all **pass**: 56 to 67 s each (`mir-1`, `mir-3`, `mir-<hash>.log`). On the 07:44 binary the interactive-grant sibling and `a_progressing_media_in_is_never_faulted_as_stalled` pass too (`mir-2`, 2/2). So the failure is not tied to one source state. It depends on timing, which points to a wall-clock decision in the turn.

## Cause

Two owners decide when the import's result page is acknowledged.

1. **Custody.** The MediaIn custody (`advance_original_media_import`, `🧵️instance/🔌️plugin-app-impl/🎞️media-transfer`) acknowledged **any** page of its operation in `AwaitingAck`. That included a page the host had not yet presented.
2. **Media exchange.** While a MediaIn is pending, its exchange advances the operation through `advance_original_media_operation_output` with `pages = None`, so nothing is captured for the shell. Only the later continuation phase of the reactor turn (`plugin_continue_typed_operations` -> `NativeResultPages::capture` -> `plugin_drive_result_pages`) presents pages as `Effect::SendMessage`.

While the shell got its page before the custody acknowledged it, the law passed. That happened when the continuation of turn k ran before the custody's poll in turn k+1.

The new quiet-slice loops (`⏱️quiet-slice-turn`, used since tonight by `⌨️command-ingress-turn`, the typed-operation continuation and the restored document loads) changed that order. The command exchange now repeats inside one reactor turn while:

- it adds no host-bound output, and
- the executing clock is still under `QUIET_TURN_SLICE_US` (8 ms).

The exchange that produced the terminal page adds no effect, because `pages = None`. So the loop runs the exchange again in the same turn, and the custody acknowledges the page that was never presented. The operation then retires (the terminal outcome still reaches the custody through the live capture). The command completes in about 64 turns, and the shell never sees a page.

Whether the loop gets that second iteration depends on wall time:

- On an idle machine a debug exchange step fits in 8 ms, and the law fails. That is what the descriptor session saw.
- Under the wasm build a step takes longer than 8 ms, the slice closes, the continuation presents the page first, and the law passes. That is what all reruns here saw.

The suspects named in the brief are not the cause:

- (1) The publication-unit change retires closing *Store* publications. The import's genesis publication never reaches that branch.
- (2) `reserved_terminal` is still fed. It only matters when the shell acknowledges first.
- (3) and (4) do not touch the operation's page.

## Fix (at the source: who may acknowledge a page)

- `advance_original_media_import` acknowledges a page only once it was presented: `stage == AwaitingAck && result_page_presented`. This is the rule the maintenance ladder already uses for `MaintenanceTool::Awaiting`. Until then the custody step makes no progress, so the quiet loop closes and the continuation of the same turn presents the page.
- Either the shell or the custody then acknowledges first:
  - If the shell is first, the custody takes the publication recorded in `reserved_terminal`.
  - If the custody is first, the shell's later acknowledgement is refused as stale by the token check.
- Result: the page reaches the shell exactly once, and the terminal outcome reaches the custody exactly once, independent of slice timing and load.
- Docstrings: `PluginApp::advance_original_media_import` (`⚛️reactor/🔌️app`) and `advance_original_media_operation_output` (`🧰️ops/🧾️typed-operations`) state the rule.
- Law: `shard_media_in_open` now also asserts that the shell never receives the same page token twice. This is the "exactly once" half that the existing `shell_pages > 0` assertion does not cover.

## Verification

There were three cargo runs through the gate, all with the media-in filter. Logs are in `🗑️generated/mir-fix-*.log`.

- `mir-fix-1` did not compile: a `[DEBUG]` ledger include of the `GET-PROCESS-WORKING-END-TO-END` session pointed at a deleted file. I waited until its owner removed it.
- `mir-fix-2` did not compile: `close_normal` in `🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs` returned a `NormalProjectionReceipt` where a `GraphImportReceipt` is expected. This was left unchanged for 35 minutes by a concurrent lifter edit, and it also broke the running wasm build (`wasm-r5a.out`). I fixed it forward: `projection(step)`, the same conversion the branch above it uses.
- `mir-fix-3`: **3 / 3 pass** (201 s): `…under_the_shard_budget`, `…under_the_interactive_grant`, `a_progressing_media_in_is_never_faulted_as_stalled`.

I then reran the neighbouring groups once on that binary, without cargo (`run-media-in-neighbours.sh`, `mir-n-*.log`):

| Group | Result |
| --- | --- |
| `media_in` (the three above plus both edit-and-save appends, incl. archive reload) | 5 / 5 (474.6 s) |
| `shard_fuel_open` (media route and paged natural source, warm and cold head) | 4 / 4 |
| `natural_source_route` | 2 / 2 |
| `natural_file_route` | 7 / 7 |
| `registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit` | 1 / 1 (142.1 s) |
| `every_declared_pdf17_window_…` and `every_declared_pdf14_window_…` (window-surface laws) | 2 / 2 |

Total: 21 / 21 pass. No `[DEBUG]` traces were added in this pass. The `[DEBUG] custody insert` and `lifetime` lines in the logs belong to other sessions.

Not reproduced here: a failing run of the old code. The machine stayed loaded by the wasm build for the whole session, so the quiet slice closed after one exchange step. The cause above is derived from the code path and from the 64-turn, zero-page census in the descriptor note.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🎞️media-transfer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔌️app/🦀️.rs` (docstring)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs` (docstring)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (exactly-once delivery assertion)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs` (forward compile fix of a concurrent edit, `projection(step)`)
- Ticket: `run-media-in-regression.sh`, `run-media-in-neighbours.sh`, this note.
