# MediaIn terminal publication: `original media import retired without a terminal publication` (2026-11-24)

Follows `📨️media-in-receive.md` and `🚪️pdf-browser-open-export-stall.md`. Every cargo run went through `run-terminal.sh` (`watchdog-cargo.py` + `native-cargo.sh`, one cargo, always filtered); the already built binary was rerun through `run-binary.sh` / `run-four.sh`. No wasm rebuild.

## Reproduction (native, real `PluginRuntime`, no fix)

`shard_media_in_open` (PDF editor unit tests) now behaves like the browser shell: every `Effect::SendMessage` to the shell that carries a typed-operation result page (`semio.typed-operation-page.v1`) is answered with the matching `semio.typed-operation-ack.v1` message (`shell_ack_of`) as an `Event::Message { source: Shell }` on the next turn. Before, the law never acknowledged a page, so only the custody ever acknowledged, which is why it passed natively.

Temporary `[DEBUG]` traces (removed again) on the custody poll and on every accepted acknowledgement showed, for the import operation (id 64):

```
custody poll op=64 stage=Publishing page=Some(Ui)       <- last custody poll that sees the operation
ack op=64 seq=1 accepted=true stage=Retiring            <- terminal page acknowledged by the shell, not by the custody
custody poll op=64 stage=Retiring page=None   (x10)     <- terminal page already closed
typed-operation slots live=0/64
custody op=64 gone terminal=false  -> InvariantViolated "retired without a terminal publication"
```

## Cause

The reserved import operation has two owners that never agreed on an order:

1. The `MediaIn` custody (`advance_original_media_import`) is the only reader of the operation's terminal publication. It captures `progress.terminal` from the operation's result page (Terminal/Download = success, Fault = fault, cancelled terminal = cancelled) while the operation is still in `tool_operations`.
2. The host (`plugin_continue_typed_operations` -> `NativeResultPages::capture` -> `plugin_drive_result_pages`) presents the very same pages to the shell as `Effect::SendMessage`; the shell acknowledges them immediately (`Event::Message` -> `plugin_acknowledge_typed_operation_result`). The terminal-class acknowledgement moves the operation to `Retiring`, and the retirement ladder (continuation, maintenance) closes the page and removes the operation within a few units.

The terminal page is produced by the continuation at the end of turn k, acknowledged by the shell at the start of turn k+1, and retired before the custody is polled again, so the custody found neither operation nor terminal. This is not the `take_typed_operation_completion` / `refresh_cache` prune fix: that fix only withholds the completion witness, it never touches the result page or the operation slot (verified: the sequence above is independent of the prune).

## Fix (ownership at the source of the terminal publication)

The terminal publication becomes a durable hand-over instead of a transient observation:

- `MountedTypedCommandFullOperation::terminal_publication()` (`🧵️retained-command/🔄️full-operation`): the one definition of "the terminal publication this operation presents" (captured fault, fault page, cancelled terminal page, terminal or download page). The custody uses it too, so the classification exists once.
- `MediaTransfers::reserved_terminal: Option<(operation id, Option<Result<(), ArtifactBoundedToolFault>>)>` (`🧵️instance/📄️docs/.../🧱️components`): registered `(id, None)` when the reserved mount inserts its operation (`advance_original_reserved_mount`, phase 7, same step that sets `reserved_receipt`), so no page can exist before the registration.
- `acknowledge_typed_operation_result` (`🧵️instance/🪢️members/.../🧾️typed-operation-results`): when it accepts an acknowledgement (from the shell or from the custody) it records the publication the operation presented at that moment into the registered slot of that operation.
- `advance_original_media_import`: if the operation is gone and `progress.terminal` is still empty, the custody takes the recorded publication (same quote, retained accounting and yield rules as the live capture), then clears the slot. A gone operation without a recorded publication (closed or cancelled operation) still raises the invariant, so the check keeps its meaning.

Result: the terminal publication reaches the waiting custody exactly once whichever side acknowledges first and however far the retirement ran; the shell acknowledgement path is unchanged (no host behaviour change, no gating, no leak when the custody dies: the slot is plain `Copy` data overwritten by the next reserved mount).

## Tests

PDF editor unit tests, `media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_the_{shard_budget,interactive_grant}` and `a_progressing_media_in_is_never_faulted_as_stalled` (3 tests, run `media_in` filter, 177.8 s): all pass with the shell simulation. Without the fix the same law fails (reproduction above). The law now also asserts:

- the command completes (`CommandIngressStatus::CommandComplete`, what the shell turns into `file-opened`) without fault,
- the shell received at least one result page (`shell_pages > 0`),
- the document is visible in the app: `snapshot().pages.len() == 2` (`document_pages`).

Paged natural-source route: it has no terminal custody. `advance_owner` (`🧵️instance/🎟️natural-publication`) treats an absent mounted operation as finished and the shell gets its completion from `NaturalSourceAck(Complete)`; pages are acknowledged by the host only. So the race of this note cannot occur there, and the existing app-level laws `shard_fuel_open_through_the_paged_natural_source_with_a_{warm,cold}_head` (completion exactly once, `Full` dirty scope, 2 pages) are its end-to-end assertion; a through-the-runtime variant would need the whole natural-source wire protocol (`NaturalSourcePrepare`/`Control` events, page cursors of `NATURAL_SOURCE_COMMAND_KIND`) which no native harness drives today. Observation, not changed: `NaturalFilePublication.committed` is never set, and an operation that retires with a fault is also "absent" for the natural driver, so a natural-route import fault reaches the user only through the shell's fault page.

## The four other broader `editor::pdf17` failures (binary run, `RUST_BACKTRACE=1`, none is this bug)

| Test | Result now | Exact cause |
| --- | --- | --- |
| `controlled_pdf_terminal_publication_requires_both_owned_headers` | fails in 0.46 s at `owner.advance(cx).unwrap()` (unit tests line 1548, first unconstrained run, decode loop, not at close) with `OriginalRetirementFailure { InvariantViolated "original page allocation attempt is occupied" }` | Raised only at `🌱️value/📋️list/🦀️.rs:590` (`reserve_page_using_for_limit`, committed code): the `RetainedAllocationEffect` handed to a paged-list page reservation already carries an unsettled `actual()` allocation. A controlled-decoder step reserves a second page inside one allocation effect. Unrelated to MediaIn/terminal custody (no operation, no page, no job); not fixed here, needs the decoder step that issues two reservations in one effect. |
| `registered_pdf_font_change_preserves_encoded_text_and_the_next_line` | `PDF semantic import requires the reference reader for Resources` | By design of the controlled lifter: `resource_bindings` (`🔨️modules/⬇️lift/.../🧭️controlled/🦀️.rs:763`) accepts one `/Font` entry (`single_resource`) and one `/XObject` entry; the synthetic page declares two fonts (`/Fsrc`, `/Fdst`), so `resources` defers with `Barrier::Resources` and the decoder answers `Event::ReferenceRequired` (`🚪️io/📥️import/🔁️controlled/🦀️.rs:419`). The registered Open route has no reference-reader fallback, so a real PDF with two or more fonts on one page cannot be opened in the browser either. Product limitation of the controlled lifter, not a test defect; fix = lift several font resources in the controlled lifter (or give the registered route the reference reader). Not done. |
| `registered_pdf_page_and_authoring_actions_undo_and_redo_step_by_step` | fails after 273 s at line 1815 (`replace-page` step; `insert-page`, `move-page`, `create-annotation`, `fill-form-field` settle) | `original fixture settlement stopped after 32768 turns ... maintenance empty false, close owner Begin`. A temporary settle-loop trace (removed) shows the operation `op 384 'replace-page'` in stage `Worker` with a live session (`s=true`, `res=false`, `term=false`, `wait=None`) from turn 1 to 32768, no result page, lane, effect or completion ever produced; `close owner Begin` is only the idle maintenance owner. The worker job of this verb never reaches its terminal result under the fixture host. |
| `registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit` | fails after 273 s at line 1397 (settle after `insert-image`) | Same signature: `op 512 'insert-image'` stays in `Worker` for 32768 turns (`s=true`, no page, effect or completion). |

The last two share one cause family (worker-stage jobs of `replace-page` / `insert-image` that never settle in the fixture; the verbs that settle are inline mutation verbs); the next step is to trace `drive_worker_step` for these two verbs (which job step answers `Pending`).

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` (`terminal_publication`)
- `…/🔌️plugin/🧵️instance/📄️docs/📥️staging/🧱️components/🦀️.rs` (`reserved_terminal` field, constructor, empty-field inventory)
- `…/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs` (registration at mount)
- `…/🔌️plugin/🧵️instance/🪢️members/📥️staging/🧾️typed-operation-results/🦀️.rs` (record at acknowledgement)
- `…/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🎞️media-transfer/🦀️.rs` (custody takes the recorded publication; live classification shared)
- PDF editor unit tests (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`): `shell_ack_of`, shell simulation in `shard_media_in_open`, `shell_pages` / `document_pages` assertions
- Ticket: `run-terminal.sh`, `run-four.sh`, this note. All `[DEBUG]` lines of this pass are removed; logs are under `🗑️generated/` (`term1..3.log`, `four-*.log`).

## Next

The coordinator must rebuild the wasm; then re-verify in the browser: `open-en` / `open-de` publish `file-opened`, `two-pages.pdf` renders both pages, incremental Save and reopen of the saved file.
