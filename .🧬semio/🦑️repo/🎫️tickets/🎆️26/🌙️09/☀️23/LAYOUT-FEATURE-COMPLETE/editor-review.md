# Layout Editor Review

Read-only static review of the current Rust inspector and patch commands on 2026-10-21. No native compile, type check, or runtime execution was performed; syntax parser results elsewhere are not compile evidence.

## P1: Normal Story Content Exceeds UiText and Aborts the Inspector

File: `✏️editor/📌️panels/🔍️inspection/🦀️.rs`, lines 441–443 and 225–230. The newly global stories section passes the complete story to paragraph_input. That calls text(value), whose failure propagates out of render. The UI contract budget schema sets textMaxBytes to 512 and UiText::try_from_str enforces that bound. A document with a visible 513-byte story now fails the entire inspection panel even without selecting a text frame. Existing selected-frame storyContent has the same limit (152). Use a bounded text editing protocol with paging/chunks or an existing long text resource facility, and preserve the rest of the inspector for long stories. Validate a 513-byte and a multi-byte story through panel rendering and editing.

## P1: Style Selects Overflow at 32 Styles

File: `✏️editor/📌️panels/🔍️inspection/🦀️.rs`, lines 307–311 and 323. Each run eagerly includes every paragraph/character style plus the empty selection. SelectBuilder stores items in UiFixedList with default UI_FIXED_LIST_ITEMS = 32 (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📏️budget/🧬️schema/🔣️.json`). Therefore 32 paragraph styles or character styles make the 33rd try_item fail and the whole selected-frame inspector return capacity error. Windowing the style editor sections does not window these selects. Newly exposed spread selector uses the same unbounded select_row path. story_select, parent_select, and thread_select also eagerly enumerate unbounded document collections. Replace these with the framework's bounded search/window selection protocol and render-test beyond 32 options.

## P2: Inherited Frame Controls Silently Drop Most Edits

File: `✏️editor/📌️panels/🔍️inspection/🦀️.rs`, lines 498–526; command: `✏️editor/🎮️commands/🩹️patch-frame/🦀️.rs`, lines 50–83 and 105–106. Selection resolves an inherited parent frame and then exposes all frame_fields, story selection, style-run add/clear/edit, thread selection, image link fields, and open-source action. The command immediately routes frames absent from page.frames to inherited_override, which only handles geometry and locked/visible; every other offered operation returns an empty Emit. Applying a parent containing a text frame to an empty page and editing its story or adding a style run appears editable but silently does nothing. Route shared story edits/open-source actions appropriately and either implement meaningful instance overrides or omit/disable unsupported inherited controls with a localized explanation. Cover inherited text and image selections with command-contract vectors.

## Scope and Evidence

No source files changed. Reviewed document/page/frame patch handlers, structure/data command helpers, inspector/data controls, UI budget and select builder, and native story-run/parent-content validators. Findings are direct source-level behavior deductions; runtime confirmation remains required once the repo runner is operational. No general compile-success claim is made.
