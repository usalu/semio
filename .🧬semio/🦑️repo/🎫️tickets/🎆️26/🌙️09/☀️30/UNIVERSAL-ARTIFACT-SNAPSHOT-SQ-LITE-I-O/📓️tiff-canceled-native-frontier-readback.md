# TIFF Native Cancellation Frontier Readback

## Receipt and scope

Root reports authentic TIFF19 run d60e2c2b…: 19 tests ran, 18 passed, one failed, 101 outside the selector. The failing law is native_producers_share_cumulative_construction_admission, at the admitted-backing-on-interior-refusal assertion. Root is obtaining a fresh receipt with encoding, phase, remaining allowance and result before deciding a repair. This readback changes no production code, fixture or test.

## Current source boundaries

The neutral allocationBridge declares 16,777,216 maximum bytes, 100,000 copy bytes, threshold 65,536 and two cumulative replay passes. The Rust test selects any progress event in DecodeNative or EncodeNative with completed >= threshold and completed < total, without selecting a particular construction stage.

Text TIFF input passes a borrowed canonical semio preamble, then decode_hex. The preamble comparison admits no owned payload. decode_hex begins a stage with the borrowed body length, scans every character to validate hexadecimal grammar and measure byte count, and advances progress during that scan. Its first allocate_vec(count / 2) occurs only after that entire borrowed scan. A threshold callback can therefore refuse Text/DecodeNative before any backing admission. allocation_stage settling native.owned_bytes() == 0 at that frontier is correct; it does not show a ledger refund.

Binary TIFF input instead enters read, which charges sizeof(TiffSnapshot) before directories and ASCII values. The ASCII reader itself performs borrowed UTF/lossy measurement before its String backing allocation, so the same unscoped callback can reach validation rather than the particular String allocation, but prior snapshot/container admission is already positive.

Encoding forecasts count rows/tags and uses ASCII .len() without traversing the long string. It then admits directory and Entry backing, physical output backing, and writes long values. Thus its large completed/total progress follows paid native backing for this small-directory witness.

## Source-supported candidate and bounded repair shape

The supported candidate is Text/DecodeNative remaining == maximum at cancellation during decode_hex's first borrowed scan. This is an inference from the actual current call order, pending Root's encoding/phase context receipt. No production ledger repair follows from that inference.

If the receipt confirms it, the neutral witness should explicitly name borrowed hex validation versus admitted hex byte materialization. The existing producer emits two stage starts with the same borrowed body length: the first before validation, the second after allocate_vec(count / 2) and before owned byte writes. A test can observe those two starts without a production hook, select the second stage for the retained-paid cancellation law, and independently retain the first-stage cancellation law requiring zero admitted backing. The 100,000-byte payload, threshold, limits, positive admission, exact replay and cumulative refusal laws remain unchanged. The independent Source reference must distinguish both stages rather than silently relaxing a remaining-bytes comparison.

## Sources

- [Actual native admission law](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🚦️cohort/🦀️.rs:76)
- [TIFF controlled decoding](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🛬️decoding/🦀️.rs)
- [TIFF controlled encoding](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🛫️encoding/🦀️.rs)
- [Borrowed semio preamble](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:237)

No Cargo command or test was run in this diagnostic lane. No runtime claim is made beyond Root's supplied authentic receipt.
