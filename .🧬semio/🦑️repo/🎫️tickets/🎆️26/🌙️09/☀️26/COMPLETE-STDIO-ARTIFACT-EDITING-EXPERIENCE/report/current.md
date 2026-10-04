# Current BMP Canonical Editing Checkpoint

Date: 2026-10-03

This is an accepted BMP checkpoint inside the still-open Stdio artifact editing ticket. It is not whole-ticket completion.

## Persisted authority

`BmpSnapshot` persists only its schema identifier and exact source bytes. Checked layout, palette and RGBA views are derived. The v3 route admits indexed BI_RGB 1/4/8, direct BI_RGB 16/24/32 and BI_BITFIELDS 16/32 with a 40-byte BITMAPINFOHEADER. Other DIB families are rejected rather than normalized through this standard. No-op export preserves source headers, reserved fields, masks, palettes and duplicate indices, row padding, gaps, trailers and package ordering exactly.

## Editing surface

The editor has revision-guarded indexed and direct region paint actions, retained progress/cancellation, stale-root refusal, exact byte publication and exact SetSnapshot inverses. Indexed edits address packed palette indices. Direct edits address only declared channel masks and retain unmasked precision bits. The ordinary package test target enables component app assembly, so these mounted editor paths compile in the accepted native receipt.

The browser editor and viewer derive a bounded ephemeral RGBA8 view and encode it as PNG with the first-party `semio-framework-pixels` codec. A projection above the 64 MiB RGBA display ceiling or any invalid/unsupported projection renders ImageWindowKit's localized unavailable state while the artifact stays mounted. Raw BMP bytes are no longer sent to the browser as the preview and codec failures no longer become an empty image silently.

## Semio image conversion

BMP import uses the checked canonical layout and RGBA projection. It records the actual BMP profile, bits per pixel and row order, and retains resolution metadata. The resolved neutral frame is explicitly 8-bit per channel and does not claim it can reconstruct palette identity, packed precision or uninterpreted bytes.

BMP export explicitly authors a Direct RGB24 v3 document. It validates dimensions, byte counts and DPI metadata, produces aligned bottom-up BGR rows and reparses the result. Because Direct RGB24 has no alpha channel, any alpha other than 255 is refused. The language-neutral fixture covers alpha 0, 128 and 255 and provides exact expected bytes for the accepted case.

## Remaining BMP work

Dedicated accessible header, DPI, palette-entry and profile-conversion controls are still absent. These must use addressed byte patches, revision guards and exact inverses. A profile conversion must be an explicit representation-changing command. Details/source access and paint controls do not satisfy those remaining natural controls. No all-authoring-complete claim is made.

## PNG authority handoff

The deferred PNG authority plan from this BMP checkpoint was subsequently implemented. The current PNG checkpoint below supersedes that plan without changing the accepted BMP source.

## Exact changed-file inventory

Status is the current shared checkout's two-column Git status for files comprising this checkpoint. `MM`/`AM` can include already-staged work plus the current unstaged continuation. The list describes checkpoint scope and does not assign every concurrent hunk to one executor.

- `MM` `.vscode/launch.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔮️oracles/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔮️oracles/🦀️.rs`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧪️tests/🧬️mutation-regressions/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎨️replace-palette-entry/🎯️direct/🎯️outcome/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎨️replace-palette-entry/🎯️direct/📸️snapshot/➡️after/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎨️replace-palette-entry/🎯️direct/📸️snapshot/⬅️before/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎨️replace-palette-entry/🎯️direct/🔺️diff/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎨️replace-palette-entry/🎯️direct/🦠️mutation/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️change-header-fields/🎯️direct/🎯️outcome/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️change-header-fields/🎯️direct/📸️snapshot/➡️after/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️change-header-fields/🎯️direct/📸️snapshot/⬅️before/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️change-header-fields/🎯️direct/🔺️diff/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️change-header-fields/🎯️direct/🦠️mutation/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📤️remove-palette-entry/🎯️direct/🎯️outcome/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📤️remove-palette-entry/🎯️direct/📸️snapshot/➡️after/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📤️remove-palette-entry/🎯️direct/📸️snapshot/⬅️before/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📤️remove-palette-entry/🎯️direct/🔺️diff/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📤️remove-palette-entry/🎯️direct/🦠️mutation/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📥️insert-palette-entry/🎯️direct/🎯️outcome/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📥️insert-palette-entry/🎯️direct/📸️snapshot/➡️after/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📥️insert-palette-entry/🎯️direct/📸️snapshot/⬅️before/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📥️insert-palette-entry/🎯️direct/🔺️diff/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📥️insert-palette-entry/🎯️direct/🦠️mutation/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎨️keeps/🎯️outcome/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎨️keeps/📸️snapshot/➡️after/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎨️keeps/📸️snapshot/⬅️before/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎨️keeps/🔺️diff/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎨️keeps/🦠️mutation/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎯️direct-behavior/🎯️outcome/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎯️direct-behavior/📸️snapshot/➡️after/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎯️direct-behavior/📸️snapshot/⬅️before/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎯️direct-behavior/🔺️diff/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎯️direct-behavior/🦠️mutation/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `AM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs`
- `A ` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️sqlite/🪶️copy/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json`
- `AM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🛬️native-control/🔣️.json`
- `AM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🛬️native-control/🧬️schema/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔗️.graphql`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🛰️.proto`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/💾️binary/🦀️.rs`
- `MD` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/📝️text/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🔗️.graphql`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🛰️.proto`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🟦️.ts`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🧪️tests/🎯️direct/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️replace-palette-entry/🧬️schema/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🌶️.spicy`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🔠️.abnf`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🟦️.ts`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🥋️.ksy`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/💾️binary/🦀️.rs`
- `MD` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/📝️text/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🔗️.graphql`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🛰️.proto`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🟦️.ts`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🧪️tests/🎯️direct/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-header-fields/🧬️schema/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🅰️.g4`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🔤️.ebnf`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/💾️binary/🦀️.rs`
- `MD` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/📝️text/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🔗️.graphql`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🛰️.proto`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🟦️.ts`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🧪️tests/🎯️direct/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📤️remove-palette-entry/🧬️schema/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/💾️binary/🦀️.rs`
- `MD` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/📝️text/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🔗️.graphql`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🛰️.proto`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🟦️.ts`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🧪️tests/🎯️direct/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-palette-entry/🧬️schema/🔣️.json`
- `M ` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/💾️binary/🦀️.rs`
- `M ` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/📝️text/🦀️.rs`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/💾️binary/🦀️.rs`
- `MD` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/📝️text/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🔗️.graphql`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🔣️.json`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🛰️.proto`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🟦️.ts`
- `MD` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🧪️tests/🎨️keeps/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🧪️tests/🎯️direct-behavior/🦀️.rs`
- ` D` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🧬️schema/🔣️.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️.proto`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/Cargo.toml`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/📋️project.json`
- ` M` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/📜️script.ts`
- `MM` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🖼️raster/🟦️.ts`
- `??` `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🖼️raster/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🖼️raster/🧪️tests/🟦️.test.ts`
- `??` `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🖼️raster/🧪️tests/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🖼️raster/🧫️fixtures/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🖼️raster/🧬️schema/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧵️bytes/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🟦️.ts`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧪️tests/🟦️.test.ts`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧫️fixtures/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧬️schema/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields16-565.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields32.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/direct-rgb16-555.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-top-down.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/direct-rgb32-reserved-sample.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb1-duplicate-palette.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb4-duplicate-palette.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/reject-dib-108.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/reject-dib-12.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/reject-dib-124.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/reject-dib-52.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/reject-dib-56.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/rgba8-direct-rgb24.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/rgba8-opaque-direct-rgb24.bmp`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/🧬️schema/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🎨️paint-indexed-region/🎯️direct/🎯️outcome/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🎨️paint-indexed-region/🎯️direct/📸️snapshot/➡️after/🗣️.dsl.semio`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🎨️paint-indexed-region/🎯️direct/📸️snapshot/⬅️before/🗣️.dsl.semio`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🎨️paint-indexed-region/🎯️direct/🦠️mutation/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🖌️paint-direct-region/🎯️direct/🎯️outcome/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🖌️paint-direct-region/🎯️direct/📸️snapshot/➡️after/🗣️.dsl.semio`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🖌️paint-direct-region/🎯️direct/📸️snapshot/⬅️before/🗣️.dsl.semio`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️history-edits/🖌️paint-direct-region/🎯️direct/🦠️mutation/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔤️source-hex/🟦️.ts`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔤️source-hex/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🔤️source-hex/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🔤️source-hex/🧬️schema/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/💰️backing/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/💾️binary/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/📝️text/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🔗️.graphql`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🛰️.proto`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🟦️.ts`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🧬️schema/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/💾️binary/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/📝️text/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🔗️.graphql`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🛰️.proto`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🟦️.ts`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🧬️schema/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/💾️binary/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/📝️text/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🔣️.json`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs`
- `??` `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🧬️schema/🔣️.json`

# Current PNG Canonical Source Checkpoint

Date: 2026-10-03

This is a validated PNG checkpoint inside the still-open Stdio artifact editing ticket. It does not close the whole ticket or claim the root browser acceptance run.

## Persisted authority

`PngSnapshot` consists of the `stdio.png` schema identifier and required exact source bytes. Checked chunk layout, profile, typed metadata, decoded samples and RGBA8 preview are derived. No-op save is byte exact. The one-table SQLite carrier stores the same source as a singleton intrinsic BLOB.

This preserves packed/indexed and 16-bit sample identity, duplicate palette values, tRNS/bKGD semantics, Adam7, filter/compression choices, multiple IDAT chunks, private chunks and source ordering. The five neutral fixtures cover the admitted fidelity boundary.

## Editing and UI

The editor mounts revision-guarded exact snapshot/patch operations, addressed gAMA editing and bounded region paint. Region paint is enabled only for non-interlaced 8-bit RGBA, where it has a stable exact interpretation; other source profiles refuse the operation while retaining their source bytes. Every mounted mutation has an exact inverse. The retained command covers progress, cancellation, stale-root refusal, 128-patch admission and DCI 4K bounded work.

The editor and viewer use an explicit derived RGBA8 PNG preview. Projection failure renders the shared localized unavailable state and keeps the artifact mounted. No preview or export failure silently becomes an empty image.

## Direct consumers

Semio image import consumes the checked projection and records the source profile beside its explicitly normalized RGBA8 frame. Semio image and drawing export explicitly author a new PNG and mount its returned exact bytes. The direct consumers no longer construct or read the removed parallel semantic snapshot fields.

## Terra audit repair

The checked source boundary now enforces PNG structure and profile-dependent ancillary rules before any semantic or preview projection. It requires one 13-byte IHDR first, one empty IEND last, at least one consecutive IDAT run, valid critical/reserved chunk type bits, and singleton modeled chunks. PLTE, tRNS and bKGD placement and cardinality are checked against the IHDR color type and bit depth. gAMA, cHRM and sRGB must precede PLTE and IDAT; gAMA zero is refused. Checked text projections also require the declared compression methods, ASCII language tags and exact UTF-8 instead of lossy decoding.

`ChangeGamma` uses a dedicated controlled byte operation. A missing gAMA is inserted before the first PLTE or IDAT, existing gAMA bytes are replaced in place, deletion removes only that chunk, and every unrelated chunk remains byte-exact and ordered. The operation keeps revision, progress and cancellation checks and validates its authored result. The shipped demo PNG, DSL and pack forms were reordered without changing chunk payloads so all three remain identical valid authorities.

The neutral audit corpus carries the indexed no-gAMA edit, zero refusal, singleton declarations and explicit malformed structure/profile cases. Native laws apply the real mutation, verify exact inverse and unrelated chunk identity, reopen the result with `pngjs`, and reject every malformed fixture.

## Remaining PNG work

Packed, indexed, 16-bit and Adam7 natural sample editing still requires native-profile commands or an explicit reversible profile conversion; the checkpoint does not claim those edit surfaces. Physically present old semantic mutation directories are unmounted and remain only because they contain concurrent staged work in the shared checkout. Root still owns the full Stdio wasm/browser receipt.

## Evidence and inventory

Design and fidelity evidence: `../🔍️research/🧬png-canonical-source-authority-2026-10-03.md`.

Exact scoped Git status: `./png-changed-files.md`.

Validation receipts and results are in `./validation.md`.

# TIFF Tiled 8-Bit Checkpoint

Date: 2026-10-03

TIFF preview now derives exact RGBA8 pixels from canonical strip or tile chunks for checked 8-bit chunky grayscale, RGB and unassociated-alpha profiles using uncompressed or PackBits storage. Tiled edge padding remains canonical and is clipped only in the ephemeral display projection. The normal editor/viewer render path receives a first-party PNG with real dimensions; refused profiles retain their bytes and use the localized mounted-unavailable image state.

The schema-owned `paint-region` action edits raw samples only for exact uncompressed tiled profiles. It is revision and bounds guarded, English/German labeled, retained-route classified, progress/cancellation aware, and exact-inverse. Painting preserves tag order, unknown tags, tile partition and padding, all unaddressed samples, and every other page.

The detailed design, profile matrix and remaining frontier are in `../🔍️research/🧬tiff-tiled-8bit-preview-editing-2026-10-03.md`. The scoped file ledger is `./tiff-changed-files.md`; validation receipts are below in `./validation.md`. The ticket remains open for the larger Stdio experience.

# TIFF IFD Selection Checkpoint

Date: 2026-10-03

The TIFF editor now has schema-owned local page selection, retained Config-only publication, localized native previous/next controls, selected-page preview and selected-page paint routing. Artifact undo leaves the selection intact; reopen and example load start at IFD 0; a removed selected page clamps to IFD 0.

Design and red/green evidence are in `../🔍️research/🧬tiff-ifd-selection-editor-2026-10-03.md`. The exact scoped inventory is `./tiff-ifd-selection-changed-files.md`. Full native run 5 passed 116/116 with no skips, and the package TypeScript check passed all four suites.
