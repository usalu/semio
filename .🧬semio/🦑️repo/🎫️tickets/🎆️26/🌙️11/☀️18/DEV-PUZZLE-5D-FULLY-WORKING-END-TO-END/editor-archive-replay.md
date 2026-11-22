# Puzzle5D archive replay

The native editor law created a part, edited its board X and world origin X, and failed while reopening the text form. The registered archive hydration driver rejected its first forward operation with `Replay: truncated at offset 73`. A debugger stopped in `Puzzle5dMutation::admit_receiving` and recorded the complete 73-byte payload.

```text
020570617274730f73706c6963655363656e65526f7773010111100407086d75746174696f6e060107057461626c650600070573746172740400070b64656c657465436f756e740401
```

The payload encodes the `SpliceSceneRows` operation selecting `parts` at `start=0` with `deleteCount=1`. The direct `OpBinary` implementation wrote an intrinsic record body without the authored format and operation tag. Its retained receiver interpreted the bytes as a complete Pack document. The mounted prepared writer already used the authored format byte, operation tag and typed record body.

The direct codec now reads and writes the typed body using the same `📡️.protocol.semio` tags as mounted preparation. The archive receiver uses those exact tags and the schema-derived receivers for leaves and their nested records. The authored protocol and existing fixture bytes retain their values. The expanded scene fixture law checks the direct and prepared bytes for a simple delete and the forward and inverse splices, then passes those bytes through retained receiving and compares the typed operations. The editor reopen law exercises document history and undo/redo over both text and Pack.

## Changed sources

Paths are relative to the 5D v1 `✳️any` subset.

- `🚪️io/💾️binary/🧬️mutations/🦀️.rs`
- `🧬️schema/🧱️document-records/🦀️.rs`
- `🧬️schema/🧬️mutations/🦀️.rs`
- `🧪️tests/🧪️every-example/🦀️.rs`
- `🧪️tests/🧫️fixtures/🔀️scene/🔣️.json`
- `🧬️schema/🧬️mutations/⚓change-part-anchor/🦀️.rs`
- `🧬️schema/🧬️mutations/✂️disconnect-grips/🦀️.rs`
- `🧬️schema/🧬️mutations/✋️drag-selection2d/🦀️.rs`
- `🧬️schema/🧬️mutations/✏️edit-part2d-text/🦀️.rs`
- `🧬️schema/🧬️mutations/➕add-part-grip/🦀️.rs`
- `🧬️schema/🧬️mutations/➖remove-part-grip/🦀️.rs`
- `🧬️schema/🧬️mutations/🌀rotate-target-volume/🦀️.rs`
- `🧬️schema/🧬️mutations/🌍create-target-volume/🦀️.rs`
- `🧬️schema/🧬️mutations/🌐change-domain/🦀️.rs`
- `🧬️schema/🧬️mutations/🌱create-part/🦀️.rs`
- `🧬️schema/🧬️mutations/🎨change-part2d-icon/🦀️.rs`
- `🧬️schema/🧬️mutations/🎯change-fastener-kind/🦀️.rs`
- `🧬️schema/🧬️mutations/🏗️change-part-kind/🦀️.rs`
- `🧬️schema/🧬️mutations/🏷️rename-puzzle5d/🦀️.rs`
- `🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦀️.rs`
- `🧬️schema/🧬️mutations/📍move-part2d/🦀️.rs`
- `🧬️schema/🧬️mutations/📏scale-part3d/🦀️.rs`
- `🧬️schema/🧬️mutations/📐scale-target-volume/🦀️.rs`
- `🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦀️.rs`
- `🧬️schema/🧬️mutations/🔀️splice-scene-rows/🦀️.rs`
- `🧬️schema/🧬️mutations/🔃rotate-part3d/🦀️.rs`
- `🧬️schema/🧬️mutations/🔄️rotate-selection3d/🦀️.rs`
- `🧬️schema/🧬️mutations/🔌replace-part-grip/🦀️.rs`
- `🧬️schema/🧬️mutations/🔍️scale-selection3d/🦀️.rs`
- `🧬️schema/🧬️mutations/🔐change-target-volume-locked/🦀️.rs`
- `🧬️schema/🧬️mutations/🔒change-part2d-locked/🦀️.rs`
- `🧬️schema/🧬️mutations/🖋️edit-part3d-label/🦀️.rs`
- `🧬️schema/🧬️mutations/🗑️delete-part/🦀️.rs`
- `🧬️schema/🧬️mutations/🗒️change-description/🦀️.rs`
- `🧬️schema/🧬️mutations/🙈change-part2d-hidden/🦀️.rs`
- `🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🦀️.rs`
- `🧬️schema/🧬️mutations/🚀move-part3d/🦀️.rs`
- `🧬️schema/🧬️mutations/🚀move-target-volume/🦀️.rs`
- `🧬️schema/🧬️mutations/🚚️drag-selection3d/🦀️.rs`
- `🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦀️.rs`
- `🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🦀️.rs`
- `🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🦀️.rs`
- `🧬️schema/🧬️mutations/🧱change-part3d-mesh/🦀️.rs`
- `🧬️schema/🧬️mutations/🪢️connect-grips/🦀️.rs`
- `🧬️schema/🧬️mutations/🪦delete-target-volume/🦀️.rs`

## Verification

The preceding native executable reproduced the error under GDB. SQLite independently assembled the authored delete-part fixture as `010101016101010600` using the fixture's part ID and the declared one-byte format, tag, field and symbol forms. The Hub build compiled the updated document records and production mutation codec. Its nine native surface tests passed, including the exact I/O route restoration and the native/guest codec comparison. The editor reopen and expanded scene law await the shared native rebuild.

## Window projection defaults

The fifth native build reached the paged snapshot, artifact schema, diff and mutation crates. It then failed when the 5D camera's bare `#[value(default)]` required a `NativeDefault` cursor for the shared `WorldProjectionConfig`, whose ordinary default owns seven strings. The 5D camera now names `WorldProjectionConfig::default` explicitly for cold input, just as its zoom names the existing scalar default. The native derive interprets an explicit authored default path as a required received field. Canonical camera producers always emit `projection`, with no skip condition, so the native owner receives the existing owned projection through its original input and never constructs an unbudgeted replacement. Read-only parsing passed for the updated file. The package check and archived config round trip are part of the following native run.

The sixth native build compiled `semio-s-artifact-puzzle-5d-tests` with this receiver, clearing the prior trait error.
