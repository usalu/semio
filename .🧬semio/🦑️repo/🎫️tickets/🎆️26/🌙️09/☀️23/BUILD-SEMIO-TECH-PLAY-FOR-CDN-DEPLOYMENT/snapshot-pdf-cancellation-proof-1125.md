# 🧾️ Snapshot PDF Cancellation Proof (2026-11-25)

Snapshot `/Users/ueli/.sinc/play`. Nothing was compiled: no cargo or rustc run. The running release build is the verifier.

## Validation Rule

`project_tool_cancellation::<A>` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🌳️tree/🦀️.rs` (lines 140-177) is called from `stamp_app_contracts`, and its panic is the one in the log. It works out
`expected = A::Command::TOOL_JOB_IDS ∖ {"typed-command"} ∩ registry.migrated_tool_ids()`. The registry side is every app, window, app-command and mode-command entry whose `semantics.execution.interactive_job == Migrated`.

Each `A::bounded_first_step_tool_proofs()` row gets `plugin-assembly.cancellation-proof` unless all of these hold:
- `owner == ToolOwnerWitness::of::<A>()`
- `controller_id == def.id`
- `artifact_schema == A::DOCUMENT_SCHEMA`
- `owner_file` is non-empty
- `tool_id ∈ expected`
- the `tool_id` is not a duplicate

After that, `seen == expected` must hold, or the error is `plugin-assembly.cancellation-incomplete`. The rule never checks the execution contract kind or the cancel budget.

For stdio editors, `TOOL_JOB_IDS` is the `snapshot_editing_command_roster!` list plus the six snapshot-edit ids. The proof rows are the snapshot-edit proofs plus one row per `bounded_native_editing_editor!` `tools`/`host_tools`/`transient_tools` id, in that order.

## Root Cause

The PDF 1.4 kit macro `pdf14_editor!` (`…/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧰️kit/🧾️pdf14/🦀️.rs`) declares `presence-pointer`, `presence-pointer-leave`, `presence-caret` and `presence-caret-clear` in two places: its roster and its `transient_tools`. That gives each of them a proof row.

The 1.4 canvas window (`…/4️⃣1.4/…/🖼️page/🔭️canvas/🦀️.rs` `window_definition`) starts from the shared page window, which appends the Migrated `presence_action_definitions()`. It then `retain`ed only `CANVAS_ACTIONS` and `VIEW_ACTION_IDS`, which dropped the four presence actions. No other 1.4 window or mode declares them. So `presence-pointer` was missing from `migrated_tool_ids()`, and therefore from `expected`.

Every proof row before it passed: the snapshot edits, the 10 native tools and the 10 view tools. This hits all three 1.4 subsets (`base`, `a`, `x`), because they share the kit; `a` is described first.

The rest of the 1.4 code already treats presence as a canvas action: `is_page_action`, `static_action`, the presence lane, and rendering with peers. The live repo's canvas keeps the full page window, presence included. Its fixture `🧪️pdf14-canvas.json` lists all four presence ids.

## Fix

1. 1.4 canvas `window_definition` now also retains `semio_s_artifact_stdio_pdf_presence_lane::PRESENCE_ACTION_IDS`. The crate already depends on it.
2. The 1.4 canvas fixture `actions` gains the four presence ids. It is hand-authored (the oracle script writes only `probes`) and now matches live. The unit test `canvas_window_offers_exactly_the_fixture_actions` asserts the exact count, so the fixture had to change too.
3. A second provable violation is in the PDF 1.7 kit `pdf17_editor!`, which serves subsets `a`, `e`, `h`, `ua`, `vt` and `x`. Its roster lacked the 10 view ids (`zoom-in` … `previous-hit`), while `transient_tools` declares them. They are Migrated in the page window. So each 1.7 subset editor would panic with `cancellation-proof for zoom-in` once 1.4 passes. The fix prepends the view ids to the roster, matching the live repo and the 1.7 base editor's own roster.

## Files Changed

- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🖼️page/🔭️canvas/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🖼️page/🔭️canvas/🧪️tests/🔬️unit/🧪️pdf14-canvas.json`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧰️kit/🧾️pdf17/🦀️.rs`

Not ported from live: the canvas `edit_actions_with` / `edit_action_definitions` restructure. It needs `page::edit_action_definitions` and `page::resolve_page`, and neither exists in the snapshot.

## Other Checks

- **The 16 `bounded_native_editing_editor!` users** (epw, semio mesh/brep, pptx×3, idml, xlsx×3, docx×3, pdf 1.7 base, pdf14 kit, pdf17 kit). After the fix, every proof id (tools, host, transient and `native:` tools) is in the same file's roster, there are no duplicates, and no roster id is left without a proof.
- **The 54 `snapshot_editing_bounded_first_step_tool_proofs!` users.** Their `TOOL_JOB_IDS` constants, with constant ids resolved, cover every proof id. The only id the scanner could not resolve is `SET_ACTIVE_EXAMPLE_ACTION_ID`, and it is the same constant on both sides.
- **PDF definition side.** Every pdf14 and pdf17 kit tool is a Migrated action in the shared page window or in the 1.4 page window: `set-page` is multi-line, and the canvas pointer, view and presence actions are all there. Every subset `main::definition()` is its page `window_definition()`. The 1.7 base editor's `insert-image`, `replace-image` and `import-local-image` are Migrated through `image-authoring/📥️admission.rs`.
- **The viewer repairs from 09-23** (`pinned-snapshot-compile-fixes.md`) are unrelated. They touched viewer windows, not editor proofs.

## Unresolved

- Nothing was compiled or run. The descriptor emitter in the running release build is the verifier.
- For non-PDF stdio editors, I did not check whether each proof id is also Migrated in the app definition. Only the roster side was audited, for all of them.
