# PDF Import Loop

## Root cause

`NaturalPdfDecoder::advance` kept calling `next` with `CosGrant { work: 1 }` while the child returned `Event::More` and `work == 0`. That reply does not move `completed_work_units` and was not a blocked grant, so the loop ran until the browser contract's 7500µs deadline and the host repeated the turn until `plugin.command-ingress-stalled` (about 1e5 continuations) without `Complete`. The unpaid `More` is produced while file preparation or semantic lift stays in the same decoder phase: an object-order scan that inspects nothing, or a declared empty stream span whose start is already at EOF and restarts the `endstream` search without consuming a byte. The host continuation wall stays 300_000 ms.

## Bound

`paged_natural_decode_of_the_browser_contract_terminates_with_every_page` asserts `turns <= 128`.

The fixture is 963 bytes. `NATURAL_SOURCE_TOOL_CONTRACT` allows 4096 work units per turn, so one paid pass covers the file. Each unpaid `More` now ends the host turn instead of spinning for 7500µs. This file has fewer than 128 such handshakes. The lopdf page-count comparison is unchanged.

Sixty-four consecutive unpaid `More` replies at the same preparation location fail with `PDF decoder made no progress`. A move to another preparation location resets that count. A blocked allocation is a separate grant error.

## Result

`paged_natural_decode_of_the_browser_contract_terminates_with_every_page` passed: 27 continuations, two pages, lopdf page count matched. Wasm rebuild is the next step so the browser host serves this decoder.

The first wasm link (`opt-level=0`, `codegen-units=256`) failed in `wasm-component-ld` with `functions count exceeds limit of 1000000` at offset `0x2018`. One codegen unit at `opt-level=1` then allocated a 25 GB rustc footprint on a 16 GB machine and was stopped. `rebuild-pdf-wasm.sh` now uses `opt-level=1`, `codegen-units=16`, `-j 1`.

## Files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📥️import/🔁️controlled/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔗️xref/📚️file-order/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️lexer/📏️stream-span/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
