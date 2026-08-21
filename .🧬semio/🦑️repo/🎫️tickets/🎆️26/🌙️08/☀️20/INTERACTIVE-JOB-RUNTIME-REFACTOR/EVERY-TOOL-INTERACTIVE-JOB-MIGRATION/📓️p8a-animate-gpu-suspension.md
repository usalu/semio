# P8a Animate GPU Suspension

## Outcome

Animate video renderer initialization no longer synchronously blocks inside its async constructor. Adapter and device acquisition now use the genuine `wgpu` futures with `.await`, preserving suspension instead of parking an interactive-reachable thread.

## Verification

- The interactivity audit's two Animate `block_on` findings are removed at source.
- `@semio-tech/animate-plugin:test-quick` reached the shared `semio-s-plugin-stdio` dependency and stopped with 981 known de-async errors before compiling Animate. No Animate test-pass claim is made.
- The change introduces no dependency and preserves the existing `VelloRenderer::new` async boundary.

## File

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs`
