# Note Window Section Projection

Captured attempt `wsiZgfi` activates the Pencil tool but traps when the resulting dirty-surface turn renders window engagements. In Chromium's exact indexed call stack, `81766@32678762` drives surface rendering, `77633@27802873` resolves the native render, and `77634@27818864` takes the reserved section branch. `88862@37609964` calls the engagement implementation `77530`, which contains the source locators for `🪟️projections/🦀️.rs:19,33`. Its selected destructor call `27219512` reaches `77225@26979415` and the root destructor `30342`: “ordered-map root must be explicitly retired before drop.” The browser console is `🗑️generated/note-capture53-erasers/console.txt`. The function offsets were independently extracted from the exact linked guest core with `camera-frame-probes.py`.

Engagements and window measures previously cloned a window-scoped ViewModel in each iteration and allowed its two ordered roots to reach ordinary drop. Both loops now call `ViewModel::scope_window_instance`. This projection exposes only an immutable view and retains an immutable borrow of the exact original until return. Its guard returns both root aliases through the ordered container's identity check and the existing non-final shared release. The donor still owns both roots, so no node retirement or final allocation release can occur. Scoped cleanup also covers a failed config capture, early exit, or a cancelled future. The existing per-window kind, explicit utility clears, focus, locator table, and trace cursors remain available to both callbacks.

The shared JSON fixture covers split windows of the same kind, explicit utility clear, absent utility, missing identity, and distinct trace runs. SQLite independently selects the expected overlays and trace runs. The focused Bun test first passed the oracle and failed the missing source contract. It passed both checks after the change. The combined run with the two camera fixtures passed 6 tests and 41 assertions. The native manifest regression exercises both real root identities, the early-return branch, and use of the original after scope. Attempt 54's retry compiled all three product crates successfully. Their Wasm Rust compiler dependency claims match exact current source digests for the ordered map, view model, and render projections. Native manifest test execution and the Pencil draw, point erase, whole-stroke erase, and undo browser flow remain pending.

`repo tools exec --light bun -- test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/note-window-section-projection.test.ts'`

## Changed Files

- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🔬️window-view-context/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs`
- `note-window-section-projection.test.ts`
