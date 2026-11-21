# Retained Generate Text Proof

## Shared input and mutation

Generate text and multiline questions stay on the framework’s semantic Input. Small values use the bounded inline field. Saved larger values use an owned paged-text child addressed by its UTF-8 byte count and content digest. Form pagination counts the retained carrier’s records. Both renderers require a complete body before allowing edits to the published value.

React keeps the existing draft, keyboard and composition handlers. Long browser intent values are packed into the existing UTF-8 scalar pages and the shared Generate operation reconstructs one string before validation and mutation. The native retained input publishes complete string actions. Values above the ordinary action scalar capacity use the existing retained outbox and carry the original input receipt through publication or host close. The generation and question identifiers continue to travel in the authored action binding.

## Verification

- [Mounted React Chromium](form-browser-probe.ts) passed with `--retained`. It opened an existing 515-byte caption and a longer multilingual textarea with a leading Unicode BOM, handled an IME-owned Enter without dispatch, published all edited bytes under the active generation and question, and reopened the saved value. An equal-byte-length external update preserved the conflicting draft and Escape restored the updated authority. The same mounted field committed a short value and reopened it, then grew from short text to 513 bytes. Plain Enter in the textarea added a newline and Ctrl+Enter committed it.
- The Chromium `--short-check` lane passed for English and German, including accessible controls, scrolling, focus, and ordinary text, slider, and toggle edits.
- [Compact native Rust](form-value-retained-native.ts) passed using the [neutral corpus](long-generate-text.json). It compiled the production carrier resolver, byte-counted key generation, shared mutation decoder, and retained string transfer. It verified missing page rejection, exact 512/513-byte and multibyte publication, unchanged generation and question arguments, and the attached input receipt.
- The full native WGPU host and live browser actor dispatch were outside the compact lane. The Chromium mount ran the actual InputView and retained document store with a fixture owner applying dispatched actions. The native outbox unit test covers acknowledgement attachment and close cancellation and has not been run through Cargo.
- The whitespace check passed for the affected files. Browser and witness intermediates were removed from the ticket.

Run the Chromium and native drivers through `repo tools exec --light bun --` from the checkout. The shared [wire schema](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/⌨️retained-input-text/🔣️.json) describes the text body reference and commit pages.
