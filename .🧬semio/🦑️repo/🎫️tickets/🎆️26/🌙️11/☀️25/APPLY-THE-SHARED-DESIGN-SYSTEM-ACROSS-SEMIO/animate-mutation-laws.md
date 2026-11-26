# Animate mutation replay

The first native replay passed 93 tests and failed 12. Rejected create, delete, batch delete and crop operations retain their diagnostic and leave the snapshot unchanged. Their undo plans are empty. Identical rename, reorder, replace-source and resize-source operations likewise leave undo history empty. The corresponding cases now assert the applicable missing-target, fatal outcome or unchanged snapshot contract.

The text round-trip replays the stored inverse plan in reverse order, matching the protocol law. This restores a cleared 2 × 2 deck in its original order. The positional diff test now constructs its starting snapshot through the domain constructor so the content-addressed presentation handle matches the tiles. It still compares the complete snapshots after applying and reversing the composed diff. The accepted crop and source-frame tests now explicitly verify the inverse-sum law.

## Verification

The final native mutation replay passed all 105 tests. The public site bundle, MP4 asset compilation and neutral title-card oracle also passed individually. Two attempts through the repo launcher received SIGTERM during the native link. The successful replay ran `replay-animate-mutations.py` directly with the managed Python interpreter, pinned Rust compiler receipt and frozen archives.

## Changed paths

Under `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any`

- `🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🆕create-tile/🧪️tests/🧪️rejects/🦀️.rs`
- `🧬️schema/🧬️mutations/🗑️delete-tile/🧪️tests/🧪️rejects/🦀️.rs`
- `🧬️schema/🧬️mutations/🧹delete-tiles/🧪️tests/🧪️rejects/🦀️.rs`
- `🧬️schema/🧬️mutations/✂️resize-tile-crop/🧪️tests/🧪️rejects/🦀️.rs`
- `🧬️schema/🧬️mutations/✏️rename-tile/🧪️tests/🧪️no/🦀️.rs`
- `🧬️schema/🧬️mutations/🔀reorder-tiles/🧪️tests/🧪️no/🦀️.rs`
- `🧬️schema/🧬️mutations/🖼️replace-source/🧪️tests/🧪️no/🦀️.rs`
- `🧬️schema/🧬️mutations/🔲resize-source-frame/🧪️tests/🧪️no/🦀️.rs`
