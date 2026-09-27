# Media and Spatial Compact Patch Rollout

## Source inventory

The assigned media, image, spatial, CAD, and Semio families expose 54 snapshot-editing editor roots. Five roots now publish the shared typed `SnapshotPatch` carrier through their native mutation aggregate:

- PNG 1.2 any
- JPEG JFIF 1.01 document
- TIFF 6.0 document
- MP4 ISOBMFF any
- WAV RIFF PCM any

The remaining 49 roots still publish a full `SetSnapshot` for at least one reachable Details edit. JPEG baseline and TIFF baseline select compact domain mutations for recognized paths but retain a full-snapshot fallback. The other 47 roots call `snapshot_edit_set_snapshot` directly.

| Family | Remaining roots |
| --- | ---: |
| Semio | 19 |
| STEP | 7 |
| IFC | 5 |
| SVG | 3 |
| DWG | 2 |
| GIF | 2 |
| AVI, BMP, DXF, glTF, JPEG baseline, LAS, MP3, OBJ, PLY, STL, TIFF baseline | 1 each |

The source-derived row inventory is retained temporarily at `🗑️generated/media-snapshot-edit-routes.tsv` while the ticket is active.

## Mounted pilot contract

Each pilot has a native `PatchSnapshot { patch: SnapshotPatch }` leaf with the fourteen-field mutation authority descriptor, an absolute reference to the shared patch schema, a native aggregate variant, aggregate JSON/TypeScript declaration, text opcode, binary tag, diff application, and exact inverse computed from the authoritative base. Editors keep existing semantic mutations for paths with a smaller domain record and route the generic fallback through `editing::snapshot_edit_patch`.

The language-neutral shared fixture declares the aggregate directory, tagging convention, discriminator, text opcode, binary tag, edit event, and a two MiB unrelated payload for CSV, JSON, PNG, JPEG, TIFF, MP4, and WAV. The TypeScript oracle applies the edit independently with `fast-json-patch`, validates the shared and aggregate schemas with Ajv, checks the leaf's absolute schema reference, checks descriptor/protocol codec identifiers, verifies inverse restoration, and requires the serialized patch to remain below one MiB. Native leaf tests additionally exercise aggregate text/binary round trips, malformed Unicode rejection, exact inverse replay, and preservation of real byte/sample/pixel vectors.

The first neutral run passed PNG/JPEG/TIFF and exposed `textOpcode: null` in the newly authored MP4 and WAV descriptors despite both declaring the text surface. Both descriptors now name the aggregate's real `patch-snapshot` opcode.

## Remaining aggregate rollout

Every remaining mutation aggregate needs the same schema-first leaf and native codec registration before its editor fallback changes. Derived roots with their own snapshot and mutation types need their own leaf even when a base/document root is already mounted. Existing semantic leaves remain first choice; only the generic fallback changes from a complete snapshot record to the compact patch.

The repeatable implementation unit is:

1. Add the leaf descriptor and JSON schema with the shared absolute snapshot-patch reference.
2. Add text and binary codecs using the aggregate's existing tagging convention and the next declared protocol tag.
3. Add the native aggregate variant, diff/inverse delegation, schema union, TypeScript union, grammar/protocol entries, catalog kind, and one codec-law fixture.
4. Replace only the full-snapshot fallback with `snapshot_edit_patch`.
5. Prove a metadata or scalar edit beside a payload larger than one MiB, forward and inverse codec bounds, publication, undo, redo, and save/reopen.

## Large structural edits

A compact leaf solves scalar and small structural edits beside a large artifact because unchanged payload stays in the base. It does not make every operation bounded by itself. Replacing the complete typed source, inserting a subtree larger than the store item limit, moving a large subtree, or removing one whose exact inverse is large can still exceed the forward or inverse publication limit.

Complete support for those operations needs a shared atomic patch-plan route: parse and validate the requested final snapshot, compute ordered bounded structural or byte-range patches, compute each exact inverse against its sequential base, validate the final result against the dialect schema, and publish the complete vector only after preparation succeeds. The retained job must report progress, honor cancellation before publication, and prevent partial publication. Media families should continue to use native `PatchPixels`/`PatchData`-style leaves for large contiguous byte ranges; the shared plan covers ordinary records, arrays, and source replacement without silently falling back to a full snapshot.

## Primary interaction repairs

PNG's primary image window now exposes a typed, localized, keyboard-operable pixel-region action. It validates coordinates and RGBA channels, prepares bounded row chunks with progress/cancellation, and publishes native `PatchPixels` mutations only after the full plan is ready.

Semio Mesh and BREP no longer expose a payload-free `set-vertex`. Both require stable entity identifiers and a finite target point, reject missing or stale targets, and publish only the intended vertex mutation.
