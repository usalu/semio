# Owned Scene Preparation Boundary

## Verified Source

Interpreter `surfacePropsToComponentSceneNode` synchronously reconstructs `Uint8Array(props.doc.bytes)`, calls `decodeScenePackValue`, then casts the result to the optional scene field expected by one of fifteen hosts. It is not compatible with an owned byteAt-only SurfaceDoc read view. The current UiNodeView remains on its old store hook, so no retained-byte live regression is claimed.

The scene packet is not the UI record packet. OS `ScenePackCodec` uses tags 0–13 for unit, booleans, u64, zigzag i64, f64, text, bytes, option, sequence, char, variant and map. The UI decoder instead starts with an interned-symbol table and a record. A schema-faithful scene parser must preserve the distinct format.

Large scene strings and sequences can occupy the admitted SurfaceDoc envelope. Converting them to a flat String/Array in one finishing step, or reconstructing the whole byte array to call the old decoder, would hide an unbounded finishing operation. Several scene fields are themselves encoded JSON/pack strings; preparing only the outer packet would not certify those host-side decoders.

## Chosen Representation

Build a scene-owned, retained flat value arena from an exact captured typed component owner. Text and byte values are validated slices into the already owned paged SurfaceDoc, not eagerly concatenated strings or copied arrays. Container records refer to numeric child/sibling IDs, so no recursive object/GC chain is manufactured. The source component remains anchored until the arena and all its readers are retired.

A retained typed host projection consumes this arena and constructs the precise scene model; it must also account for nested encoded fields. Text access emits bounded decoded chunks. The design does not add a whole-array or whole-string compatibility adapter. Scene projection remains a publication/read-owner dependency before live UiNodeView cutover.

The per-instance aggregate still must own surface resolution, pending patches/projections, consumer roots and retained acknowledgement handoff. The concrete surface close currently waits exact active subscriptions and committed receipt obligations. No per-index/global-empty shortcut is authorized.

## Verification Status

Root independently reports the current OwnedSurface five tests passing and the full React aggregate passing 542 tests in five files. Strict R20 is running under the coordinator. The scene work is currently schema/design stage; no scene parser/projection runtime pass is claimed.
