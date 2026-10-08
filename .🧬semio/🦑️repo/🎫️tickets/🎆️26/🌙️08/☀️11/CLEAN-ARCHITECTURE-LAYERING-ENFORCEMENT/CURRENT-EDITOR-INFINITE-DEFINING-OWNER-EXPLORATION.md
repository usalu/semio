# Editor to OSInfinite Defining Owner Exploration

Read-only bounded source inventory. No runtime acceptance or production edits.

Editor uses Infinite crate-root wildcard, which brings both neutral canvas and OS board/world/kernel symbols into its public namespace. Actual consumed neutral symbols: Camera (from Board root export of canvas camera), camera::Viewport, Color new/components/multiply_alpha, Geometry Point/Rect/Affine, Scene new/fill/append, FillRule::NonZero, theme::merge_color_field, text::label_advance/label_span_world_x/label_byte_world_x/append_label/append_label_tspans, render::scale_scene_for_device_pixel_ratio and gpu_session::CanvasGpuSession browser lifecycle. Unit laws additionally use text::label_shape_stats. Scene typed payload decoder uses General Pack/Value and parent Editor settings/adornment types; it has no direct canvas import. Text-input/unit files use parent wildcard, so their defining symbol providers must be preserved explicitly.

Geometry already belongs General `📐️geometry`, not Infinite; consume that owner directly. Canvas renderer first-party Scene, Color, FillRule and draw_list currently live in OSInfinite `🖼️canvas/🦀️.rs:6-1209`; theme, font/icon assets, label shaping and caches, camera, render and GPU session follow in that same file. This canvas file contains no OSKernel/Store/artifact source imports in inspected code; package-level OS coupling is introduced by the Infinite package shell. It does have host/backend imports vello/kurbo/peniko/vello_svg/usvg and generated icons/OUT_DIR shortcodes, so simply copying the monolith to General would carry foreign runtime APIs and generated-build dependencies. Foreign backend types are confined principally inside renderer/backend and browser gpu_session; audit exported session return types before defining its public port.

OSInfinite package root explicitly exports OSKernel os_dsl/os_pack/os_spr/os_store/os_vcs, mounts Surface terrain to avoid a Cargo cycle, World and Board/directed/DAG owners, and depends on artifact-reference, artifact-infinite-dag and OSKernel. Those must stay product-owned. OSInfinite root component itself reexports World. None of that belongs in neutral Canvas extraction.

Minimal coherent move: move the actual defining neutral Canvas tree (renderer first-party command values, text/font assets+cache, camera, render and drawing laws) to one General Canvas owner; import General Geometry directly. Separate backend replay/GPU attach through first-party ports with no public external types, keeping implementations in their appropriate platform owner. Editor imports defining Canvas+Geometry contracts explicitly, removes Infinite wildcard and manifest edge. OSInfinite consumes the same General definitions and ports; do not preserve the old wildcard as an alias facade. Its Board/World/OSKernel integration stays specific. All callers of actual moved canvas symbols need direct General rebinding; Board/World callers remain Infinite. This is a defining-owner move, not a new copied facade.

Preserve original renderer, camera-fit, label-fit, label-shapes, icon-codec, draw-list and retirement laws at their defining owners, plus Editor unit/text-input/scene laws. Source mounts currently use relative include/path and generated assets; handcraft final paths/package build ownership and GUI/Nx routes. Existing fixed corpora are functional law inputs, not schema authority. Editor scene production schema should survive; do not recreate removed fixed testplan schemas. Before deletion admission compare original law roster and inspect both native/browser generated bindings, since EditorSession public browser methods must retain behavior while its neutral owner no longer links OSInfinite.

## Complete bounded lexical Infinite-reference file roster

The following scans actual framework/S Rust and Cargo files, excluding other languages. It is a lexical provider/caller roster, not a resolved import graph. Cross-language TS dynamic/generated Wasm consumers require a separate exact API name scan during implementation.

- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs`
- `🧰️framework/🔨️modules/✍️editor/🦀️.rs`
- `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️lod/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👇️canvas-pointer-down/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️canvas-drop/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛬️canvas-drag-over/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs`

## Explicit Editor canvas-qualified caller roster

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

## Base64 fresh8 join

native-8: actual code0; 22 capture rows, hashesTrue, currentTrue.
source-8: actual code0; 22 capture rows, hashesTrue, currentTrue.
strict-8: actual code0; 22 capture rows, hashesTrue, currentTrue.
