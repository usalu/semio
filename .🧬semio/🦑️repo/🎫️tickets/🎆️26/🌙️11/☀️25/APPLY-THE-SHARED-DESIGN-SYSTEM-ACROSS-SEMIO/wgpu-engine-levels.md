# WGPU engine surface laws

The retained Puzzle3D settings law now expects four outer edges and two internal dividers on each resting stepper. Its center and outer background inherit the surface. The same law still checks layout, hits, both catalog icons, formatted value glyphs and change bindings.

The engine law for unavailable extensions and media now verifies transparent slot backgrounds. An unavailable extension still draws its fallback glyphs. The reserved transport still has a visible host rectangle at least as high as the declared reservation. The native painter documentation now describes that inherited surface.

The registry law previously rejected its byte descriptor before attempting bounded-thread construction. The measured 64-slot table has 60,528 bytes per element and a 520-byte owning descriptor. The fixture updates only the element measurement. The capacity, heap-first structural check, conversion threshold and 1,048,576-byte thread stack are preserved.

## Media controls and file inputs

React's media alert and resting or disabled transport buttons inherit the owner's background. The native painter now follows that rule, using a shared focus border for the enabled focused button. Slider track and thumb still show range state. React's file picker keeps the Choose caption on the field background. The retained file input paints its caption and filename over that same background and keeps real color swatches for color inputs.

The paint tests check that ordinary media states produce no row fill, keyboard focus emits one four-edge outline, and unfilled file input chrome keeps both captions.

## Verification

- The complete pinned native engine suite passed 72 of 72 laws, including the transparent media host, all settings stepper assertions and heap-first construction on the original 1 MiB thread stack. The engine used check-native-levels.py semio-ui-wgpu-engine-laws.
- The native retained paint suite passed 113 of 113 laws. Its new cases verified the media focus outline against the shared compositor and the file caption and filename on an unfilled field. The pinned Cargo runner declined the shared workspace's current lock file on the retry. check-wgpu-engine-levels.py used a ticket-local workspace manifest and lockfile with symlinks to source directories, then ran Cargo offline. The shared workspace lock was untouched.
- The OS renderer integration binary passed 59 of 59 tests, including World3d pointer selection parity with React and the World3d bounded-thread law. Both originally failed when the native fallback tried to launch the independent Three.js oracle without Bun on its PATH. The retry supplied repo-managed Bun and restored the 64 MiB stack environment already set by .cargo/config.toml for direct execution. The bounded law still uses its own explicit stack limit. The ticket's check-native-levels.py now resolves Bun through repo for this integration suite.

Changed paths under 🧰️framework/🔨️modules/🖱️ui

- 🧪️tests/🔬️targets-wgpu-engine-unit/⚙️settings-close-and-accessibility-commit-laws/🦀️.rs
- 🧪️tests/🔬️targets-wgpu-engine-unit/🎭️scene-host-overlay-body-and-frame-laws/🦀️.rs
- 🧪️tests/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json
- 🎯️targets/🧊️wgpu/🖌️paint/🎨️node-painting-with-driver/🖼️media-and-slot-nodes/🦀️.rs
- 🎯️targets/🧊️wgpu/🖌️paint/🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs
- 🎯️targets/🧊️wgpu/🖌️paint/✍️retained-edit-text-steps/🦀️.rs
- 🎯️targets/🧊️wgpu/🖌️paint/🎨️node-painting-with-driver/🎛️control-nodes/🦀️.rs
- 🧪️tests/🔬️targets-wgpu-paint-unit/📶️progress-and-image-paint-laws/🦀️.rs
- 🧪️tests/🔬️targets-wgpu-paint-unit/✍️input-caret-and-focus-ring-paint-laws/🦀️.rs
