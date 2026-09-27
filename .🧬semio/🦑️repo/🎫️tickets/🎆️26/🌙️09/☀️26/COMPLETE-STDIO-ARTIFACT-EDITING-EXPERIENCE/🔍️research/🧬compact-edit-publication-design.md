# Compact Edit Publication

## Observed Boundary

The complete editor catalog still publishes most shared edits as native `SetSnapshot` events. A small metadata change therefore includes the entire document. The retained store admits at most 1 MiB for one publication item, while the shared fallback counts at most 65,536 value nodes. Neither limit is an appropriate measure of a small edit to an ordinary photo, audio recording, video, or archive. Raising declared budgets does not repair the native store boundary.

WAV and PNG now have native compact range mutations and direct metadata routes. The media owner has one passing retained WAV test with 2,097,152 samples, including cancellation, undo, and redo. PNG's corresponding test is pending. Existing compact native mutation leaves remain the first choice for other format fields.

## Shared Typed Path Design Under Review

The JSON representation belongs to `🌱️value/🔁️codec` and its derive expansion. `DslRecord` knows native DSL shapes and cannot independently reproduce JSON renaming, tags, flattening, or custom field codecs. A shared reflection interface should therefore extend the value layer, with no dependency back to stdio.

Two separate operations avoid requiring every `FromValue` generic argument to also implement `ToValue`:

- A `ToValue` path reader returns only the selected subtree. Scalar defaults use their scalar value; container and derived record/enum implementations walk directly to the requested child.
- A `FromValue` path writer applies a typed set, insert, or remove to a detached native snapshot. Scalar defaults accept replacement only at their own root. Vec/map/optional/record/enum implementations walk directly to native fields. They need no conversion of unmodified siblings and no `ToValue` generic bound.

The shared editor protocol can compose these primitives atomically for move and rename. Before publication, the previous subtree supplies an exact inverse; admission checks encoded forward and inverse native events against the real publication-item ceiling. A shared schema-defined patch leaf is referenced by each native mutation aggregate, preserving the existing artifact history and replay system. Replacing the document root is allowed only when both directions fit; larger documents remain editable through bounded paths.

Custom serializers/deserializers must retain their exact wire representation. Derived traversal must cover renaming, skipped fields, transparent wrappers, all enum representations, optional members, and flattening. A variant tag cannot be changed independently into an invalid variant record. The UI must offer a complete valid replacement record for that operation.

## Validation Requirements Before Rollout

This is a design, not an implemented or verified feature. Introduce schema-first neutral set/insert/remove/read vectors and compare with the existing serde_json/json-patch test oracles. Include untouched multi-megabyte byte arrays, exact 64-bit integers, custom codecs, generic records, escaped map keys, enum variants, invalid atomic edits, and forward/inverse admission. Keep generated derive changes compatible with concurrent repository work through explicit ownership and focused tests.

Schema enforcement must also avoid expanding an unchanged large document. Validate the affected schema fragment where constraints are local; conditional/cross-field constraints need the smallest encompassing record. Exact native conversion remains mandatory. Complete schema validation and editable-source replacement require bounded work, progress, and cancellation; a fast path cannot silently bypass constraints.
