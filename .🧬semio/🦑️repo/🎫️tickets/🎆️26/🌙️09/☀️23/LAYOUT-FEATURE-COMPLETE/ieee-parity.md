# Native IEEE Equality Parity

The native value owner now exposes allocation-free binary32/binary64 word equality. Intrinsic floating numbers and retained scalar-list comparison use those identities. Layout persisted records, floating mutation payloads and sparse floating patches compose exact word comparison directly; they do not allocate a whole document value tree merely to compare it. Signed zero is an authored change, identical NaN payloads compare equal, and distinct NaN payloads remain distinct. Layout finite geometry/style admission still refuses nonfinite candidates.

## Production Files

Framework owners: `🧰️framework/🔨️modules/🌱️value/🌳️intrinsic/🦀️.rs`, the value root facade `🦀️.rs`, and `📋️list/🟰️equal/🦀️.rs`.

Layout schema adds `🟰️equality/🦀️.rs`, mounted by its existing Rust root. Explicit word equality covers LayoutRect, LayoutBounds, PageMargins, PageColumns, ParagraphStyle, CharacterStyle, ParentPage, Page, GridSettings and Frame. The three diff owner files `🔺️diff/🧩️patchable-frames/🦀️.rs`, `🔺️diff/🩹️row-patches/🦀️.rs` and `🔺️diff/🧱️grid-text-and-guide-deltas/🦀️.rs` preserve nested nullable presence as well as native floating words.

The 17 floating mutation leaves are change-page-width, change-page-height, drag-frames, change-frame-fill, update-character-style, update-page-columns, resize-frame, update-grid, update-page-margins, update-text-frame, update-parent-page, update-paragraph-style, rotate-frames, rotate-frame, move-frame, change-frame-stroke and scale-frames. Their payload equality is exact. Local no-op guards reuse the same owners. Width/height, margins and color candidate admission now matches the TypeScript finite rules; fill/stroke also refuse a non-rectangle target. Refused operations emit no inverse effects.

## Portable Tests and Independent Oracle

The framework neutral fixture `value/🔢️ieee754/🧪️tests/🧫️fixtures/🔣️.json` contains 16 explicit binary32/binary64 vectors for positive/negative zero, equal/distinct quiet NaN payloads, infinities and subnormals. Its test-only owned oracle wraps lodash byte-array equality. Native intrinsic and retained-list laws consume that same fixture; the retained-list law checks zero allocation and funded scalar pages.

Layout `🧪️tests/🧫️fixtures/🟰️words/🔣️.json` contains 14 authored signed-zero mutation witnesses and six nonfinite candidate refusals. The native semantic graph test consumes them through actual mutation decoding, sparse diff, central applier, central inverse and reversed semantic inverse. It also tests whole ReplaceDocument identity for eight opaque dictionary binary64 vectors and persisted model equality for all 16 vectors. TypeScript consumes the same shared vectors through the canonical parser/evaluator/applier/inverse. The independent graph oracle continues to wrap RFC 6902, Ajv and Canvas transform semantics behind test-owned interfaces.

## Executed Evidence

- `semantic-ts-check.mjs`: 372 actual source files, full strict diagnostics 0; 96 portable semantic cases, 51 golden scenarios, all 58 mutation variants, 40 IEEE word cases and two drawing owner cases pass. Compilation uses actual sources; emitted verification modules are removed by the harness.
- `semantic-graph-check.mjs`: 57 semantic graph cases (17 accepted, 40 refused), three nonfinite gutter witnesses; 61 complete-document and nine parent-content oracle cases; all 102 golden snapshots graph-valid; 16 independent IEEE byte identities pass.
- web-tree-sitter Rust grammar: 42 affected Rust files parse without errors. Maximum affected Rust path is 151 characters. Every new native include_str fixture target was resolved and exists.

## Unverified

No native Rust compilation, native test execution, allocation-law execution or fresh repo runner execution was performed. Grammar acceptance is not Rust type checking. The cached repository executable is stale; this report makes no native pass claim. Drawing bounded lowering remains the next separate execution scope.
