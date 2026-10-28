# Layout Capacity Repair Review

Read-only static re-review on 2026-10-21. No source changes, native compile, type check, or Rust runtime execution. Reviewed bounded text spans, choice windows, dictionary controls, inherited dispatch and shared parent editing. The prior eager-story and dynamic-Select capacity issues are addressed in source. Remaining actionable issues follow.

## P1: Selected Stories Render Duplicate Control And Window IDs

File: ✏️editor/📌️panels/🔍️inspection/🦀️.rs, lines 445–450, 517 and 328–334.

The global stories section calls chunked_text with story.<id>.content. Selecting any text frame referencing the same story calls it again with exactly the same field. chunked_text derives both its tree-window key and every input/tree-item ID only from that field. Thus two simultaneously rendered trees contain identical IDs, and TreeWindows cannot address their positions independently. Selection now introduces this duplication for every visible referenced story (including the default story). Give render instances distinct UI/window namespaces while retaining the same semantic command field, or show one editor and a reference to it. Add a full-tree ID uniqueness check while a text frame is selected and independently window the two views if both remain.

## P1: Chunk Commits Can Replace Unrelated Text After Another Span Changes

File: ✏️editor/🎮️commands/📄️patch-document/🧵️text-chunks/🦀️.rs, lines 25–30; caller ✏️editor/🎮️commands/📄️patch-document/🧩️structure/🦀️.rs, lines 19–22.

The event carries only start/end character positions and the new span value; splice reads the old span from the current document and assumes it is the span the user edited. If two visible inputs have pending commits, or another user inserts before a focused span, an older event can remain in bounds and silently overwrite unrelated content. Example: a 600-character story has ranges 0..256, 256..512, 512..600. An insertion at 0 advances the original second span to 257..513, but the old second-input commit still addresses 256..512. It will replace the last character of the first span and leave one stale character at the end. Minimal common-prefix splicing against that wrong current substring does not repair the address. Carry a revision and expected original span/context; reject stale commits with an explicit fault or rebase them through the event history. Apply the same guard to dictionary chunk events (which also address collection members by index). Validate queued commits to different chunks with an insertion before the second span, plus Unicode and concurrent edits.

## P2: Byte Dictionary Values Still Abort The Inspector At 257 Bytes

File: ✏️editor/📌️panels/🔍️inspection/🧾️data-fields/🦀️.rs, line 35; ✏️editor/📌️panels/🔍️inspection/🦀️.rs, lines 217–232 and 46–47.

String values now use bounded chunks, but a Bytes value is still converted in full to hexadecimal and passed to paragraph_input. A 257-byte value becomes 514 ASCII bytes and fails the 512-byte UiText bound; the error propagates from the data window section through render. This is ordinary valid typed dictionary content. Provide bounded byte/hex editing with byte-aligned span mutation, preserving the rest of the panel. Render and edit a 257-byte fixture, including a non-first visible byte window.

## Other Observations And Evidence Limits

The character-span splitter preserves scalar boundaries and empty text, and story_splice computes character-based offsets without a UTF-8 byte split. Dynamic choice rows use indexed windows with full totals and selected flags. Shared parent controls carry an explicit parent prefix and shared-resource routes now resolve inherited frames. No new source-level blocker was established for these routes in this pass. Fault text is English-only, but whether app Fault messages are localized by a higher layer was not established, so this is not elevated as a separate finding. Parent::handle runs before lock checking; clarify intended editing policy and cover locked parent frames/layers in command tests if these edits should follow local frame lock behavior.

This review makes source-level deductions only. Native compilation and runtime verification remain necessary.
