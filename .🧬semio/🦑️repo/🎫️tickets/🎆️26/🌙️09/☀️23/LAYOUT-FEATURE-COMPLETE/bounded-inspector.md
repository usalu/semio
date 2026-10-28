# Bounded Layout Inspector And Inherited Editing

## Implemented Result

The current inspector already contained earlier repairs for the three findings in editor-review.md: chunked story controls, framework tree-window choice controls, explicit shared-parent fields, and shared story/link dispatch. This slice audited those actual implementations and corrected remaining concrete defects without duplicating the controls.

Text windows now query the original validated UTF-8 string directly. A slot covers 252 nominal bytes and moves its two boundaries backward by at most three bytes, so every materialized text input contains at most 255 complete original UTF-8 bytes. The count is constant work and one requested slot inspects only its two local boundaries. No Vec of every story span is created. Typed byte/hex windows likewise compute only requested 128-byte spans. The original complete source remains borrowed, and every source byte remains reachable, including empty input, a 513-byte story, multibyte stories, and a 1.4-million-byte story.

Chunk addresses are UTF-8 byte offsets in the authoritative schema. Stamped commits validate the complete original source fingerprint, admit exact UTF-8 boundaries, then translate the changed span to a native character-based EditStory splice. Earlier-source edits still reject queued stale drafts. This is a simultaneous greenfield protocol change: existing input fixtures and native command assertions now use byte addresses, with no old character-address compatibility path.

Relationship choice rows no longer collect every source option into a Vec. Paragraph and character style-run controls also stopped materializing all style choices. Existing framework tree windows retain total counts, selected markers, accessible labels, user-configurable offset/row requests, and every option beyond the fixed Select capacity. Static enums still use their small bounded Select controls.

Inherited story/link resource commands now use the effective page override lock flag, including an explicit unlocked instance of a locked parent frame. Explicit parent-prefixed commands use parent frame and parent layer lock flags. Shared text, style-run, image path/metadata, open-source effects, and explicit parent field updates retain meaningful native handlers. The existing English/German shared-parent scope explanation remains visible.

The inspector's stale drawing_canvas_at call referenced an API removed by the genuine member model. It now renders empty disabled numeric artboard controls only when the page owns a real Drawing member reference. Each control has an accessible label and an English/German explanation that the original Drawing source must be connected. There are no fabricated canvas dimensions and no unqualified child reads or edits.

## Executed Evidence

bounded-inspector-check.mjs exports run(root). It performs strict TypeScript checks and executes the actual portable direct-window helpers and actual Layout mutation evaluator/diff application in-process. iconv-lite independently encodes Unicode scalars; Lodash independently locates prefix boundaries and selects option windows; Ajv verifies the authoritative capacity schema; fast-json-patch independently produces the complete expected shared-resource results. These libraries are test-only and introduce no runtime dependency.

Fresh execution passes zero strict diagnostics. Three neutral text cases produce 5567 original spans over 1,402,313 UTF-8 bytes, with exact reconstruction. A measured 1116 distinct Unicode boundary cases pass. Option collections of 33, 80, and 1,000,000 entries retain their exact total and last entry while the single-row query invokes exactly one original source read per collection. Seven typed-byte cases include empty, boundary, 257-byte, and ten-million-byte sources.

A valid actual Layout document with two inherited text frames and one inherited image passes four real portable shared mutations: Unicode story editing, image link path replacement, image metadata update, and explicit shared parent-frame content. Every full resulting document matches independent JSON Patch, and the input document is unchanged.

Nine native source/test files parse using tree-sitter Rust. Native authored laws cover complete scalar-safe text reconstruction, partial-UTF-8-span refusal, actual long-story last-window materialization, existing 513-byte and multibyte full inspector render cases, more-than-32 relationship choices, actual inherited text/image handler effects, resolved instance locks, explicit parent locks, and bilingual accessible disabled artboard fields using a genuine member transport fixture. Native fixture include paths were checked. **Native compilation, macro/type checking, actual panel projection, command runtime, and allocator execution remain unrun because the repo runner is unavailable. Grammar and portable evidence are not native execution evidence.**

## Integration Boundaries

Complete artboard metadata display and editing remain pending a genuine original Drawing source capability and qualified command route. The root agent and Draw member-store owner own that capability integration; this panel does not fabricate a local child provider. Existing patch-page inline Drawing calls remain outside this slice and require that owner's corresponding repair.

Original complete-source fingerprints and ordinary model lookups still scan source data. The new bounded guarantees concern row materialization and direct chunk queries; this slice does not claim every full inspector render has a funded constant-time source traversal. No new runtime credit, provider, migration, fallback, or compatibility API was introduced.

All owned paths remain under 256 characters. No AGENTS.md, git state, worktree, generated compiler output, or external runtime dependency was changed.

## Exact Owned Files

- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️capacity.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/📄️drawing/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/📄️drawing/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/📖️parent/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/🧪️tests/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/🧪️tests/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-frame/🧪️tests/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧵️text-chunks/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧵️text-chunks/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧵️text-chunks/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧵️text-chunks/🧪️tests/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️patch-document/🧪️tests/🔬️unit/🦀️.rs
- C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/bounded-inspector-check.mjs
- C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/bounded-inspector.md
