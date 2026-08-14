# MP4 Lossless-Source Implementation

## Scope

This implementation wave was narrowed by coordination to the single dedicated fixture:

`/Users/ueli/Documents/semio/temp/bauen-mit-bestand.mp4`

No PPTX file was edited in this wave. The earlier PPTX work was discovery-only and is recorded in `📓️mp4-pptx-research.md`; the dedicated PPTX implementation agent starts from zero partial PPTX implementation edits from this agent.

## Fixture evidence

- Size: 16,086,051 bytes.
- SHA-256: `54b0672cca68a474d44c6096abb6579160b4d33b0f637f588e2e0752373e05c7`.
- ISO-BMFF top-level order observed before implementation: `ftyp(32) → moov(11272) → free(8) → mdat(16074739)`.
- Earlier read-only `ffprobe` evidence: one H.264/`avc1` video stream, 1200×1080, time base 1/15360, 901 frames, duration 30.033333 seconds, no audio.

The prior canonical MP4 writer always emitted `ftyp → unknown → mdat → moov`, so the exact source image could not roundtrip through semantic reconstruction.

## Changed files

### Snapshot and artifact state

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  - Added persisted `Option<crate::ArtifactSource>`.
  - Added `projection()` which clones the snapshot and clears `source`.
  - Updated authored snapshot fixtures and pack/DSL expectations for imported-source provenance.
  - Added exact fixture pack and DSL export-equality coverage.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  - Mirrored persisted source state through artifact/snapshot conversions.

### Native I/O

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
  - `decode_mp4` now captures the exact native input bytes with the source-free semantic projection fingerprint.
  - `encode_mp4` returns captured bytes only when `source.matches(snapshot.projection())`.
  - Source-free authored snapshots and changed imported projections retain the existing canonical writer.
  - Added direct byte-equality coverage for the exact fixture.

The existing `encode_mp4` signature returns `Vec<u8>` and has no error channel. Therefore this wave did not fabricate an error return for dirty imported states. Canonical writing remains limited to the fields the current semantic model exposes. Native no-op export never enters that writer.

### Diff and mutation laws

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  - Added tri-state `source: Option<Option<ArtifactSource>>`, allowing no change, set source, and clear source.
  - Added source handling to apply, absorb, between and `is_empty`.
  - Extended the field sweep.
  - Added exact fixture empty-diff, inverse, absorb and source-removal laws.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  - Updated every handcrafted diff constructor to preserve source by default.
  - Added exact fixture no-mutation, mutation/inverse, and source-bearing set-snapshot binary-op codec coverage.

The MP4 mutation codecs use serde JSON for the full operation payload, so `ArtifactSource` is included automatically in both text and binary set-snapshot operations.

## Static validation

- `git diff --check` over the five MP4 files completed with no output, meaning no whitespace-error diagnostics were found.
- Fixture SHA-256 was re-read after the edits and remained `54b0672cca68a474d44c6096abb6579160b4d33b0f637f588e2e0752373e05c7`.

## Nx validation status

The required Bun/Nx route was invoked as:

`bun nx run @semio-tech/stdio-plugin:test -- exact_bauen_mit_bestand_fixture_round_trips_byte_for_byte`

The existing `TestScript` ignores the test-name segment and starts the crate-wide configured build/test workload. The first run waited more than fifteen minutes on shared Cargo package-cache/build-directory locks and produced no Rust diagnostic; it was interrupted. A second run used a ticket-local `CARGO_TARGET_DIR` to avoid the shared build-directory lock. It reached dependency compilation (`zerocopy`, `autocfg`, `arrayvec`, `find-msvc-tools`) but was explicitly stopped on root coordination request to free the agent slot. It also produced no MP4 compile or test diagnostic.

Consequently, no claim is made that Rust compilation or the new tests passed. Root will centrally compile all artifact lanes after concurrent edits stabilize.

## Expected invariant

For the exact fixture, `decode_mp4(bytes)` captures `bytes` and fingerprints `snapshot.projection()`. Immediate export, pack decode/export, DSL parse/export, empty diff, no mutation, and a mutation followed by its inverse all restore the matching projection and therefore select the exact-source fast path. Any persistent semantic change makes the fingerprint mismatch and selects canonical MP4 reconstruction instead of returning stale native bytes.
