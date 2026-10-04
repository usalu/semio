# PNG and LAS Canonical Refusal Prerequisite Pair

## Scope and actual mounting

This is a typed prerequisite cutover, without changing module mounting, capability opt-ins, schemas, fixtures, tests, runtime limits, paid backing, callbacks, retirement, or native format algorithms. Root owns Cargo execution. No native/runtime or type-check result is claimed here.

The actual PNG Snapshot mounts its SQL module and implements exactly the SQL projection and reconstruction hooks. Its preflight, native decode and native encode hooks remain the shared trait defaults. PNG snapshot controlled-native and PNG engine controlled-native files are unmounted. Its ordinary engine is mounted; only its two shared pure parsing producers needed by the controlled engine were typed.

The actual LAS Snapshot mounts its Pack module, whose ArtifactPack declares a SQLite codec, but its SQL trait file and Pack controlled-native file are unmounted. This existing missing trait wiring remains unchanged. LAS SQL source contains projection, reconstruction, preflight and subset validation, while its controlled native source remains adjacent preparation. No missing hook or module was filled in before authentic Native RED.

The stdio Deflate controlled-native file is also unmounted. Root authorized its typed producer chain as the prerequisite for the unmounted PNG controlled engine. The framework Deflate implementation was only inspected; shared High owns separate protocol-wrapper work.

## Changes

PNG's mounted SQL hooks now return canonical ValueError directly instead of typed inner closures projected to String. Existing integer/Boolean/null/identity validation and pixel projection remain unchanged.

PNG controlled snapshot/native entrypoints and engine helper chain now return ValueError. Native controller errors flow directly. Envelope errors retain SemioError identity through into_value_error. Literal syntax and data refusals are authored InvalidValue; size/allocation ceilings are OwnershipLimit; entity/work limits are WorkLimit; absent encoding authority and unsupported critical chunks are UnsupportedOwner; encoder-only invariants retain InvariantViolated. The hex UTF-8 conversion is an encoder invariant rather than a source parsing error. The two pure shared producers parse_ihdr and pixel_to_rgba now author typed refusals; ordinary String engine endpoints project their typed errors explicitly at their existing boundary. PngColorType and PngSrgbIntent pure from_u8 producers were inspected: their only refusal is the unsupported numeric enum value, with no control path. Their controlled caller converts that closed intrinsic error to InvalidValue; there is no message classifier.

The stdio Deflate chain retains all controller ValueError identities, canonical envelope identity, and retained allocation error.kind/error.reason. The six-case DeflateError enum is matched intrinsically: OutputLimitExceeded becomes OwnershipLimit; the five physical syntax cases become InvalidValue. No prose is classified. The two expansion passes, paid history admission, History retirement, checksum measurement/materialization and cumulative encoder adapter remain unchanged.

LAS draft SQL hooks and preflight return ValueError directly, and subset errors use IoError::from_value_error. LAS draft controlled native helpers return ValueError; only the existing shared record-codec callback's positioned TextError boundary wraps the kind with TextSpan. Explicit count and collection overflow producers retain WorkLimit. The actual declared Header/Point/VLR controlled factories and all f64::from_bits/f64::to_bits raw-word mappings, GPS optional u64, twelve header IEEE words and three point IEEE words remain unchanged. SQL FloatColumn/FloatRow companions, forecasts, admitted native text/blob/collection copies, and controller scopes are unchanged.

## Files

- [📸️snapshot/🪶️sqlite/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs)
- [📸️snapshot/🚦️native/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs)
- [🚪️io/🚦️native/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🚦️native/🦀️.rs)
- [✳️any/🚪️io/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs)
- [📸️snapshot/🚦️native/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs)
- [📸️snapshot/🪶️sqlite/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs)
- [📦️pack/🚦️native/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/📦️pack/🚦️native/🦀️.rs)

## Verification

Parser-only rustfmt --edition 2021 --emit stdout exited 0 for all seven changed files. Receipts are in generated/png-deflate-typed-parser-0.log through -4.log and generated/las-typed-parser-0.log through -1.log under this ticket's 🗑️generated directory. An initial ordinary PNG engine import was placed before inner module documentation and failed parser validation; the import was moved after the documentation and the receipt was refreshed to exit 0. No files were formatted in place.

No Cargo command was run. No Source fixture or test expectation changed. Prior Source green receipts belong to Root's census and were not rerun in this lane. Actual runtime, typed compilation, Native cancellation and exact admission remain Root's next checks.

## Remaining measured boundaries

The actual OS Store PackError alias resolves through pack's protocol::codec re-export to replication/⚙️codec/🦀️.rs, which still has Schema(String). The separate framework pack/⚠️error enum had already removed that variant; those are distinct definitions. An initial claim that PNG ordinary Schema constructors were obsolete was corrected after checking the actual alias. The ordinary PNG engine/ArtifactPack boundaries are unchanged and are not asserted as compiler blockers by this receipt. LAS's unmounted SQL trait remains a wiring prerequisite for its currently declared ArtifactPack SQLite codec; preserving unmounted state is intentional per Root's instruction. GLTF and SVG are not changed in this pair.
