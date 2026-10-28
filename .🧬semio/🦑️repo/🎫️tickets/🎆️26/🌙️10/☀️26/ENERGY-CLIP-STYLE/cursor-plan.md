# Cursor Driven Teaching UI

Match the regular semio UI interaction contract: native dark cursor assets, canonical control cells and maximize/restore glyphs, ordinary hover/pressed/selected paints, and emphasis of every hovered ancestor without emphasizing siblings. Use a schema-first event reducer for enter/move/press/release/click, including cancellation and disabled ancestors.

Window focus and restore each have their own cursor choreography. Move to the real fullscreen button in the title cap, hover, press/release/click, then zoom. Do not reverse focus frames for restoration. Return to the common overview/cursor state so cross-lesson boundary frames remain identical. The pointer leaves the app viewport during passive content and returns for interactions.

A reusable native authored-scene helper records events and calls activation only after a committed click. An equipment interaction uses this helper in the actual authored physics scene. Cursor-active intervals travel with canonical raw sources, excerpts and concatenated chapters, so the enclosing virtual window/canvas also emphasize while the cursor interacts with content. The cursor is rendered once, inside the authored content, without a second compositor cursor.

Root owns presentation rendering, authored helper and runtime interval propagation. GPT 6.1 Sol High agents own pure interaction geometry/reducer contract and independent tests; two low-effort audits inspect canonical behavior and native output across available slots. Existing narration timing, source frame preservation, candidate publication and boundary equality remain required. All temporary outputs are confined to this ticket generated folder.
