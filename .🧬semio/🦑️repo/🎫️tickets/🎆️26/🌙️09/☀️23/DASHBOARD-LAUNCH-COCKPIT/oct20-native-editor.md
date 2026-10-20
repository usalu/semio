# Native Dashboard Document Editing

This feature closes the actual source-edit/navigation gap recorded in oct20-dashboard-workflow-audit.md. Its filesystem backend and editor widget have actual compiler and native test-body acceptance; dashboard controller and real PTY journeys remain pending. The native frontend itself is installed and its help, build help and warm OS MCP handshake have genuine terminal evidence in oct19-e2e-native.md.

## Ownership And Design

The parent owns the new domain-neutral filesystem workspace crate, its schema/neutral vectors, native descriptor/atomic publication implementation, durable events, workspace/lock edges and independent TypeScript oracle. This agent owns the framework Textarea element native editor, its widget mount and edit signal, dashboard browsing/document worker/view/controller/labels/keymap integration, and actual UI/PTY regressions. The controls owner removes the old external IDE open-source route without an alias and preserves shell/debug/Git controls. No external runtime dependency is introduced by this agent.

The editor uses existing framework grapheme segmentation and terminal cell-width primitives. UTF8 byte carets support combining clusters, emoji sequences, multiline editing, CRLF-preserving newline insertion/paste, tab cells, keyboard/mouse selection, copy, bounded delta undo/redo and a 1MiB edit ceiling. Reversible edit events remain local and ephemeral. The payload-free TextEdited widget signal avoids copying the entire document on each keystroke. Thirteen language-neutral cases and a closed schema preceded or accompany the production behavior; an independent SQLite mutation oracle and Intl segmentation compare exact text and byte cursor. A retained foreign predecessor opens as read-only: navigation/selection/copy stay enabled, all mutation paths refuse.

The dashboard's native Files view opens from the overview or customizable prefix action F. Typing filters the current directory; Enter opens directories/files, Backspace returns to the parent when the filter is empty. Filtered results omit the parent row so Enter selects the matching file. Browser root is the backend's canonical empty string and displays as a dot. It does not start an external editor.

Filesystem work runs in a worker, with one active request and at most one latest queued request, a bounded 32-message channel, explicit cancellation and progress, window identity plus a monotonic operation token, and document edit generation checks. Cancelled/obsolete loads cannot overwrite a new selection or dirty buffer. Disconnected workers retire instead of leaving the dashboard busy indefinitely. Save completion updates the authoritative revision but retains text typed after submission. A committed save remains authoritative even when cancellation arrives after publication; a conflict carrying a retained predecessor path is also surfaced after late cancellation.

Customizable prefix actions are F Files, S Save, R Reload, D Discard, C Cancel File Operation and V Inspect Retained Foreign Version. Existing lower-case controls remain. Dirty close/reload/detach/shutdown retains the document and requests an explicit save/discard decision. Native document tabs show path, unsaved/read-only state and line/column. All new visible phases, errors, actions and decisions have English/German text and use the selected locale. The retained version action opens a separate read-only tab without replacing the original local buffer.

A normal optimistic conflict preserves the external file and local buffer. If publication races at the kernel exchange barrier, the backend truthfully reports that the local version was published and retains the foreign predecessor; the UI surfaces its structured recovery path and permits native read-only inspection. This feature does not describe check-then-rename as compare-and-swap and does not promise a noncooperating writer's target pathname remains unchanged in that race.

## Authored Acceptance Gates

- Source oracle: repo run @semio-tech/ui-rs:test-text-editor-source --skip-cache --output=stream.
- Existing TUI widgets native owner: multiline_editor_matches_neutral_vectors_with_bounded_utf8_history; multiline_editor_cursor_and_pointer_follow_physical_cells.
- Existing dashboard terminal native owner: source_ui_browses_saves_conflicts_and_requires_explicit_discard; source_worker_disconnect_retires_without_losing_a_document; source_committed_save_is_adopted_after_a_late_cancel.
- Existing real dashboard PTY journey owner: Native files browse edit Unicode save reopen conflict and discard without an external editor; German native files use customized controls and retain unsaved changes on close. Both consume the shared nativeEditor journey schema/vector. portable-pty and vt100 independently drive/model the actual installed dashboard; filesystem reads independently verify saved/foreign bytes.

No gate above is marked passing yet. Parent will schedule Source bodies serially; controls first settles the nine-library compiler matrix after its actual source-closure repair, then this agent resumes Core and real dashboard build/install/journeys. No duplicate native compiler process was started while those owners were active.

## Exact Additional Attribution

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧬️schema/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧫️fixtures/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/🥒️.feature`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🎯️targets/⌨️tui/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧩️widgets/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui/🪀️widget/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/⌨️keymap/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️labels/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📋️panes/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️journeys/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/🧭️journeys/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🥒️.feature`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🦀️.rs`

The generated deduplicated whole-task source/input inventory now contains 172 paths, maximum 193 UTF-16 units. Earlier attributed paths are retained in oct19-e2e-native.md. This report and oct20-dashboard-workflow-audit.md are additional retained Markdown inputs.

The native editor is mounted as the existing framework Textarea element's TUI target, adjacent to its React implementation. No parallel semantic element was added.

Final source review found and corrected an interior CRLF caret case: vertical movement to a shorter CRLF line now stops before the whole line separator. The thirteenth neutral vector and actual Rust per-step grapheme-boundary assertion cover it; no compiler/runtime pass is inferred.

## Isolated Backend Runtime Gate

After the controls owner's eleven-package value matrix reached existing native-owned/projection lib-test compiler errors, parent scheduled the independent standard-library filesystem crate alone. The installed repo command in session53877 completed with whole-command rc0: ten tests passed, zero failed and zero filtered in 0.46 seconds, after a 12.49-second compiler build. It used the framework manifest, semio-framework-filesystem-workspace, locked lib tests, nocapture and one test thread. The exact incoming six-hour build budget and configured RUST_MIN_STACK134217728 were retained. Evidence is generated/e2e-filesystem-workspace-native.log. Controls received explicit terminal handoff before its next matrix retry; this agent owns no running native job.

The actual bodies covered the shared neutral contract; Unicode save/reopen and retained event predecessor inspection; ordinary and final-checkpoint foreign revision conflicts; prepublication cancellation and authoritative committed saves after late cancellation; partial publication retained recovery; size/binary/foreign revision refusal; ordered and cancellable browsing; source/store symlink refusal; and physical ancestry replacement during save. These backend passes do not establish the still-pending widget, terminal controller or real PTY editor journeys.

The real source admission log also rendered localized configuration/read/completion messages for one filesystem unit, four reads, three files and 55,655 bytes with elapsedMs zero. This validates the earlier structured source progress presentation on this admitted native command.

## Actual Widget Native Gate

The isolated semio-framework-ui-tui-widgets whole-lib command in session5178 completed with whole-command rc0. Both mounted tests passed, zero failed and zero filtered. The actual cursor/pointer test checked physical cells, and the shared editor body printed DEBUG multiline editor neutral cases=13 UTF8=true boundedHistory=true. These cases include grapheme-safe emoji/combining editing, CRLF movement and insertion, selection, byte/history limits and read-only refusal. Evidence is generated/e2e-tui-widgets-native-editor.log, including repository completion. This was the real framework manifest/locked/prototype owner route with nocapture and one test thread, using the original six-hour budget and configured stack. Controls received explicit whole-terminal handoff before its next compiler matrix. Dashboard controller and real PTY bodies are still unexecuted.

## Core Gate And Tree Test Ownership

The next focused core command session43175 ended with whole-command rc101 before the docking law body. It compiled production dependencies, then found 23 existing rows-lib-test errors: stale facade paths, a private WidgetSignal path, and Tree widget calls mixing core's lib-test types with the ordinary core library used by widgets. Evidence is generated/e2e-tui-core-after-production.log. This is not a docking pass.

The six Tree integration laws and their independent Skia fixture/color oracle now live in the existing semantic Tree element's native test owner. The fifteen pure row/filter/viewport/pointer/scrollbar laws remain in core. The actual Tree TUI module mounts its tests, imports use current direct owners, and the shared language-neutral fixture remains unchanged. The core test dependency on widgets is removed; tiny-skia moves to widgets' development dependencies together with the already-existing pack-json and styling owners. No runtime edge or compatibility alias is introduced. Independent review accepted the source correction; parent owns the exact framework/Root lock refresh and subsequent native scheduling.

The earlier widgets whole-lib two-law pass precedes this six-law remount. The current eight-law widget closure must run again after the parent's lock refresh; the earlier pass is retained as evidence for the native editor's actual thirteen cases rather than presented as acceptance of an unexecuted changed closure.

Additional exact attribution:

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/📜️rows/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/⌨️tui/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎯️targets/⌨️tui/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧱️cell-model/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧩️widgets/📦️packages/🦀️rust/Cargo.toml`
