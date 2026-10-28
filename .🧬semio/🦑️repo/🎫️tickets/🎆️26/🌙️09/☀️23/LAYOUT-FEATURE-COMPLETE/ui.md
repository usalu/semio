# Layout Editor UI

Implement structured editable controls through existing patch commands and central native semantic mutations. Cover dynamic text style runs with paragraph and character assignments, structure and style lifecycles, page spread and layer ordering, optional colors and alpha, and typed data dictionary members. Use English and German labels and virtualize unbounded inspector collections with the framework tree window protocol.

Validation starts from language-neutral inspector control vectors projected to JSON and verified with serde_json. Command vectors verify emitted semantic mutations and text boundaries. All paths remain below 256 characters.

Implemented typed data dictionary rows (including nested values), optional fill/stroke and alpha, arbitrary run ranges with paragraph and character format selects, create/delete style/parent/spread buttons, spread assignment, empty-layer deletion/order, and capture of active-page content into a parent. Inspector collection sections now use TreeWindows instead of fixed truncation. Pending native test execution through repo infrastructure.

Additional coverage: standalone story creation/content editing/deletion for unused stories; inherited frame inspection resolves the active page before other pages; new layer names use the current locale; numeric input ranges bound opacity and positive dimensions. The semantic fixture edits six style runs and preserves Unicode character boundaries for A😀éB.

Independent syntax validation: installed web-tree-sitter 0.20.8 with installed tree-sitter-rust WASM grammar parsed 12 changed UI/command/test Rust files. 0 syntax errors or missing nodes. This validates syntax only; native compile, type checking, and runtime tests remain pending repo runner availability.

## Files

- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧵️patch-page/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧩️structure/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧾️data-fields/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧾️data-fields/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️layout-play-app/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧪️tests/🔬️unit/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️controls.json`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧪️tests/🧫️fixtures/🔣️editing.json`

## Verification Limits

Both new language-neutral JSON vector files parse successfully. All 12 changed Rust sources parse with the installed independent Rust grammar without errors or missing nodes. Native compile/type checking and runtime fixture execution have not run: the cached repo test entrypoint returns without test evidence and the permitted repo-only runner is under investigation by the parent. No passing native test claim is made.

## Inspector Review Repairs

Regression vectors added before implementation for 513-byte and multi-byte stories, 80-option pickers scrolled to the last entries, shared inherited text/link actions, and explicit shared parent-frame edits. The framework nested tree window protocol will preserve complete extents for both chunked text and choices.

## Completed Capacity And Inheritance Repairs

- Story and typed dictionary text uses Unicode-safe inputs containing at most 256 UTF-8 bytes each, leaving insertion headroom under the UI contract’s 512-byte value budget. Nested framework tree windows publish every chunk; each committed story span becomes a minimal native EditStory splice, preserving surrounding text. Empty stories retain an insertable span.
- All dynamic paragraph/character style, story, thread, layer, parent-page, and spread choices use the framework tree_window_indexed_item protocol. Choice windows publish complete totals and selected state, including entries after the 32-item Select limit. Fixed language enums remain bounded Select controls.
- Inherited geometry and flags still produce per-page overrides. Shared story/run/link/open-source operations resolve the inherited source and dispatch native shared-resource mutations. Explicit, localized Edit Shared Parent Frame controls edit source geometry, flags, fill/stroke/opacity, text columns/insets/wrap/story/thread, layer, and stacking through SetParentPageContent. Unsupported implicit instance writes now return a scope fault. Existing instance lock overrides are resolved before checking geometry permissions.

Regression coverage includes 513-byte ASCII and 1800-byte Unicode stories, independently computed UTF-8 span vectors (3 and 8 spans), exact Unicode span commits, 80-option selection windows reaching the last selected option, full inspector render with every relationship collection beyond 32 items, and inherited text/image/shared-parent dispatch. EN/DE parent scope descriptions distinguish page overrides from shared edits.

### Repair Files

- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧾️data-fields/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧩️structure/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧾️data-fields/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧵️text-chunks/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/📖️parent/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧪️tests/🔬️unit/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️capacity.json`

### Repair Validation

Installed web-tree-sitter 0.20.8 with installed tree-sitter-rust WASM grammar parsed all 11 changed repair Rust files after final edits: 0 ERROR nodes or missing nodes. The neutral capacity JSON parses successfully. Node UTF-8 encoding independently generated expected chunk boundaries. No new runtime dependency was introduced. All repair source and fixture paths are below 256 characters (maximum 166).

Native Rust compile/type checking and native runtime fixture execution remain **unverified** because an evidence-producing permitted repo test runner is unavailable. Syntax parsing is not a compile or runtime test pass.

## Span Identity, Concurrency And Byte Regression Vectors

Before repair, regression tests now describe independent global/selected editor keys, stale queued ASCII/Unicode spans, typed 257-byte values, non-first byte-span replacement, and member-reordering refusal. Neutral concurrency vectors preserve exact byte oracle outputs.

## Completed UI Identity, Stale-Span And Typed-Byte Repairs

- UI location keys are independent of semantic mutation fields. Global and selected story editors have distinct root, window, row and input identities. Their host window requests scroll independently while edits target the same stable story.
- Every story chunk event carries the fingerprint of the full story content as rendered. A changed source produces the explicit layout.chunk.stale fault before any positional splice. The fingerprint guard rejects queued drafts after insertions and Unicode edits rather than replacing unrelated text.
- Dictionary chunk events carry a full ordered-dictionary fingerprint, protecting both contents and indexed-member identity. Intrinsic object member order is included explicitly even if canonical JSON orders object keys. The same guard also protects dictionary scalar/kind/key/add/delete and clear controls, preventing stale indexed actions outside chunk editors. Their UI location identities stay stable when fingerprints change.
- Typed Bytes now use byte-aligned windows of 128 original bytes / 256 hex characters, with insertion headroom beneath the 512-byte UiText limit. A commit parses complete byte pairs and splices the native byte vector through ChangeDataFields; no string or raw-JSON conversion replaces the stored type. Empty byte values remain insertable.

Regression tests cover full-tree key uniqueness with a selected text frame, independently scrolled global/selected editors, queued ASCII and Unicode spans after prefix insertion, dictionary text changes and member reordering, scalar actions after nested object reordering, 257-byte rendering, non-first byte windows, and exact typed byte replacement. Node Buffer independently confirms the neutral byte replacement oracle.

### Identity/Concurrency Repair Files

- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧾️data-fields/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧩️structure/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧾️data-fields/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧵️text-chunks/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧪️tests/🔬️unit/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️concurrency.json`

### Identity/Concurrency Validation

Installed web-tree-sitter 0.20.8 with the installed Rust WASM grammar parsed all 8 changed Rust sources after final edits: 0 ERROR or missing nodes. All native fixture include paths resolve. The neutral concurrency JSON parses and its expected replacement bytes match the Node Buffer oracle. Maximum source/fixture path: 169 characters. No new runtime dependencies.

Native Rust compile/type checking and native runtime regression execution remain **unverified**. Independent syntax parsing and oracle checks do not constitute a native test pass.
