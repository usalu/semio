# PNG and BMP Owned Sample Closure

The parent approved replacing serialized byte snapshots with `{schema,image}` owned image records. PNG retains native unsigned 16-bit samples, dimensions, bit depth, color type, interlace intent, exact indexed palette/transparency, typed metadata and opaque ancillary payloads placed before or after the raster. BMP retains native indexed samples or precision-preserving direct component values, dimensions, orientation, profile and channel masks, palette, resolution, reserved header metadata, opaque gap and trailer. Native compression, filtering, CRCs, row offsets and padding remain physical I/O concerns. No serialized source cache is part of either semantic record.

Both diff owners replace the typed image as an atomic value. Addressed paint operations update native samples directly; image validation, revision, diff application and inversion must remain pure. Expensive paint operations retain progress, cancellation, bounded ownership and retirement behavior through the shared job interface.

The existing authored byte source fixtures remain physical binder inputs. New neutral sample fixtures and exact declared language contracts are authored before implementation, and each physical round trip must preserve owned values and precision. Canonical native output can differ in compression or padding from imported source bytes. Independent PNG/BMP libraries remain test oracles rather than runtime dependencies.

No runtime verification has yet been performed for this replacement. Office, Flow and XML verification is being completed before image production implementation begins.


## Authored Contract Stage

The PNG and BMP snapshot JSON Schema, GraphQL and protobuf contracts now declare the owned image model. New language-neutral `🧬️owned-native-samples/🔣️.json` vectors author PNG grayscale/RGB 16-bit precision, palette duplicate colors and interlace intent; BMP vectors author precise bitfield component values, duplicate palette indices, opaque gap/trailer, header reserved metadata and untouched reserved sample bits. Typed Rust and TypeScript implementation and physical binders remain outstanding, so these contracts are a staged schema-first change and are not yet runtime-verified.

PNG image fields: width, height, bitDepth, colorType, interlace, samples, palette, transparency, gamma, chromaticities, srgb, physicalDims, timestamp, background, textChunks, ancillaryChunks. Each opaque ancillary record owns kind (four unsigned bytes), payload data and afterRaster placement. BMP image fields: width, height, rowOrder, profile, masks (four unsigned 32-bit values), palette, pixels, xPixelsPerMeter, yPixelsPerMeter, colorsUsed, colorsImportant, reserved1, reserved2, opaqueGap, opaqueTrailer. Pixels are an exact discriminated `storage` union of indexed indices or direct native component sample records; direct sample records own red, green, blue, alpha and unassigned reserved bits.

Manifest:

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧫️fixtures/🧬️owned-native-samples/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️owned-native-samples/🔣️.json


## Contract Verification and Implementation Start

Fresh registered TypeScript test targets for both PNG and BMP passed in 12.5 s. Each now includes two registered owned sample schema tests using third-party Ajv: all three authored neutral cases accepted; former serialized byte authority and an out-of-range native sample rejected. The existing package/editor checks also ran. This verifies the declared contracts; Rust/TypeScript semantic implementations remain outstanding.

Rust BMP production work has started: typed snapshot/image/native sample/pixel union and pure model validation/default/demo, direct owned dimensions and whole-image diff algebra; native admission/emission now decode/encode owned samples. Pure semantic revision and paint operations preserve native channel precision, palette indices, opaque metadata and untouched reserved sample bits. These changes have not yet been compiled or runtime-verified. Native store codecs, editor and other consumers, source fixture binders and typed twins still require closure.

## Continued Rust Closure

BMP registered Rust check passed after repairing typed sum-field DSL roles, schema module import and the binary facet docstring (`bmp-owned-model-check-2.log`). The full BMP native suite was attempted with the registered `:test long --lib --no-fail-fast -- --nocapture` target; it stopped in the parent-owned framework ArtifactRef dependency refactor before BMP test compilation (`bmp-owned-model-native-1.log`). No BMP runtime pass is claimed.

PNG precise native admission now retains u16 channel samples or palette indices with Adam7 geometry. Canonical publication retains native profile, precise transparency/background values, authored metadata and before/after opaque ancillary data. Logical SQLite rows own image fields, samples, metadata records and opaque ancillary BLOBs; native filters, compressed lanes and CRCs are absent from semantic ownership. The previous literal/source recipe implementation is removed. Typed mutations/diff/default/dimensions query the owned image. The pure sample work cursor incrementally copies samples, text and ancillary data and applies paint, with caller grants and explicit close. Both language twins are actively owned by the TypeScript sibling.

PNG registered Rust check attempt 1 stopped before PNG compilation at the concurrent OS I/O import `super::sqlite_snapshot` in framework I/O (`png-owned-model-check-1.log`). Artifact integration and runtime suites remain outstanding. Controlled codec accounting and progress are staged, and require actual native tests before any correctness claim.

## Retained Reader And Physical Control Completion

PNG and BMP retained paint now retain immutable Arc readers supplied by the editor command context. Advancing consumes this reader without an external mutable snapshot. A different Arc reader capability is refused; cloning then mutating uses Arc copy-on-write, so the retained source remains unchanged. The production pending publisher independently checks current document revision and store generation before publication; that seam is owned and verified separately by the root agent.

PNG preparation validates bounded headers and allocation extents. Sample precision, text profiles, opaque ancillary identity and complete FNV source identity are validated during fuelled copy. No whole-image hash or metadata extent scan occurs on each advance. BMP preparation validates headers and sample cardinality; each copied native sample validates its profile within fuel. BMP editor painting yields an owned SetSnapshot document after the addressed sample operation and preserves opaque/reserved data.

BMP physical native admission/emission now accept first-party NativeDecodeControl/NativeEncodeControl; palette, sample and opaque-byte operations report actual progress and typed cancellation/ownership refusals. Added neutral fixture driven control assertions. PNG physical filtering, defiltering and CRC loops report progress, and raster extents are checked before decompression with no32-bit width×sample×bit-depth intermediate overflow.

Typed mutation aggregate protobuf/GraphQL contracts now carry the canonical snapshot and typed leaf sum. Shared SnapshotPatch protobuf/GraphQL descriptors reference the existing first-party intrinsic DslValue owner. PNG/BMP snapshot GraphQL input twins are authored. Snapshot/diff G4 and EBNF sidecars match the final masks and ancillary comma tuple fields. Descriptor verification is pending the sibling's registered checks.

PNG native attempt4 and BMP native attempt6 both exited during registered Cargo preparation after19m58s, before artifact compilation or runtime. PNG failed with EINTR scandir in the resource lease queue; BMP failed with EINTR opening the XLSX oracle manifest during workspace discovery. Logs are png-owned-model-native-4.log and bmp-owned-model-native-6.log under generated. These are infrastructure failures, not passing tests. Fresh attempts PNG5 and BMP7 are running against the current source. Earlier production checks and focused logical carrier results remain the only successful Rust evidence until these retries complete.
