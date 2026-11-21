# Full text in channel previews

The shared evaluated-channel tree now opens nonempty text values in the framework's read-only Text Editor. It works for a standalone input or output and for text nested inside an array or typed object. Empty strings retain their complete inline representation.

Short strings open directly. Longer strings expose numbered pages through the existing retained tree window. Each page reads from a fixed UTF-8 byte interval, adjusts only the interval endpoints to code-point boundaries and publishes the requested content through the existing Text Editor buffer lane. A page does not need the preceding pages to be mounted or traversed. The original graph handle remains the hover target for the parent row and each numbered page.

The scene and channel tree now have distinct explicit sibling keys. Feeding the native projection into the browser revealed duplicate automatic sibling keys at their shared parent.

The browser run also exposed shared renderer issues. Text Editor scene packets now omit decoded absent options, while an explicitly encoded null still clears hover. Authored read-only settings control the keyboard and textarea in both React rendering branches. Pack decoders retain a leading byte-order mark when it is part of a length-delimited string. Retained trees count mounted full-width content in the window length so the host can finish loading a direct text page.

## Verification

- The language-neutral fixture covers five cases, including an empty value, CRLF, tabs, a combining mark, BOM, NUL, a supplementary code point crossing a nominal page boundary, and an exact page multiple. Ajv validated the schema and iconv-lite independently reproduced the expected page strings and the concatenated UTF-8 bytes. All six Bun checks passed.
- The native witness uses the actual shared presenter, mounted tree-window view, framework Text Editor and buffer lane. It requests long pages out of order, checks that one page is admitted at a time, compares its hover binding with the original port binding, and checks the direct input text. The registered Generation3d preview fixture also opens a retained nested output through the same window request path.
- Chromium rendered native projection snapshots through the real interpreter and Text Editor host. It opened a nested output page, checked its exact readonly editor packet and the graph hover intent, collapsed and retired the content, then opened the direct input. It also verified that the retained text content fills its one requested row. The test used a session fixture at the Wasm binding port.
- Twenty-two Vitest checks of the editor packet and tree scheduler passed, including the absent-field regression, exact leading code point and explicit hover clear.
- The registered linked Generation3d suite remains gated by the local disk reserve.

Scripts in this ticket can reproduce the focused checks. The [portable fixture](../../../../../../../✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values/🧪️tests/🔣️.json) is inside the shared semantic UI source.
