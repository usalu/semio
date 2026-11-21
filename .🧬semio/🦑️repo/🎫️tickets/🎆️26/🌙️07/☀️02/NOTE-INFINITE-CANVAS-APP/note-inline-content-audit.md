# Note Inline Content Persistence

The browser sent complete text and table payloads in `updateBlock` events. Note decoded the events and replaced the blocks in its temporary snapshot. When it compared that snapshot to the committed document, the gesture transaction emitted only placement, visibility, lock, name, and stroke edits. Text paragraphs and table cells therefore had no event to publish or replay.

Note now emits the existing rich-text, font-size, and TeX mutations from those canvas updates. Table edits emit a mutation addressed by the block ID and zero-based row and column. Its table diff records the old and new cell content, verifies that the addressed cell still matches the old content, and reverses those values for undo. The new mutation uses the existing table crate and native binary and text mutation codecs.

The browser-wire fixtures cover rich text with marks and a second paragraph, a cell change with untouched neighboring cells, an edited TeX expression, and a font-size change. A source assertion failed before these fields were wired and passed afterward. Bun SQLite independently replayed and reversed all four fixtures. The existing independent Python table implementation replayed and reversed the new committed table fixture. The focused Rust test checks host event decoding, durable mutation binary transport, document replay, unchanged-event suppression, and undo. The TypeScript parser accepted the new cell diff and rejected a negative column and missing content. The repo graph invocation stalled in workspace inventory and was cancelled. The ticket's `verify-note-inline-content.ts` launcher selects only the Note artifact crate and inline regression through the repository's native test runner. The focused run reached native workspace preparation and was cancelled to prioritize the guest build on the same host. Rust and rebuilt browser verification were pending when this audit was written.

The focused test launcher runs from the repository root with:

```sh
repo tools exec --light bun -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/verify-note-inline-content.ts' test
```

All code paths below are workspace-relative.

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs`
- Under `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets`
  - `🔣️.json`
  - `✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/{🦀️.rs,inline-edits-tests.rs,inline-edits.json,inline-edits.test.ts}`
  - `✳️any/🧬️schema/🔺️diff/{🦀️.rs,🟦️.ts,🔣️.json}`
  - `✳️any/🧬️schema/🧬️mutations/{🦀️.rs,🟦️.ts,🔣️.json,📦️packages/🦀️rust/🪞️mirror/🦀️.rs}`
  - `📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/📦️packages/🦀️rust/🪞️mirror/🦀️.rs`
  - `📊️table/🧬️schema/🧬️mutations/✍️edit-table-cell/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`
  - `📊️table/🔮️oracles/🔣️.json`
  - `📊️table/🧪️tests/📊️mutate-note-1-table/{🥒️.feature,🦀️.rs,🐍️.py}`
  - `📊️table/🧪️tests/🧫️fixtures/🧬️mutations/✍️edit-table-cell/🪶️changes/{📸️snapshot/⬅️before/🔣️.json,📸️snapshot/➡️after/🔣️.json,🦠️mutation/🔣️.json,🎯️outcome/🔣️.json}`
