# Note WGPU Point Eraser

The native Note canvas now clips each stroke segment against the eraser circle in stroke-local coordinates. It retains cut endpoints and keeps outside runs in drawing order. Tangent contact leaves a stroke untouched. An isolated dot at or inside the circle is erased. The React renderer and the WGPU geometry tests use the same language-neutral cases in `eraser.cases.json`.

The retained interaction scan records each block’s parent, sibling index and ID span alongside its existing JSON range. Point erase visits siblings right to left so the original sibling index remains valid after editing a later sibling. It updates the first surviving run under the original ID and inserts additional runs immediately after it with the same parent. Complete erasure publishes a removal. The scan checks generated fragment IDs against the document, including nested IDs. Whole-stroke erase retains its existing scan order.

The two native regression tests cover shared clipping geometry and a press that splits two nested strokes on either side of a fully erased dot. The second test replays the emitted sibling insertions and removal and checks the placement and retained identities. Additional fragments are still appended from fixed-size pages one per interaction step and the retained gesture continues to publish through the existing stream and commit route.

## Verification

The focused repository Cargo test was started with `repo cargo --store=shared --configuration=prototype -- test -p semio-renderer-wgpu-scene-ink --lib point_eraser`. It cleared admission and compiled workspace dependencies for about 10 minutes. The command was cancelled with SIGINT to free capacity for the active Note guest build. It released its leases and exited with code 130 before reaching scene-ink, so the native routing integration test has no result. The same command restarts both focused tests.

An isolated pinned-nightly Rust run compiled the production clipping functions and passed all 13 cases from the shared JSON. The repository diff whitespace check passed.

The focused crate command selects only library tests named `point_eraser`, including the actual native WGPU gesture job. It does not launch a renderer, GPU session, guest, or browser. Cargo stdout is streamed through the repository command session.

## Changed Paths

```text
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🏁️ink-interaction-job-publish-and-close/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/📋️ink-clipboard-camera-and-wheel/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/📤️ink-event-pages-and-edit-routing/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🔁️ink-interaction-job-scan/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🔎️ink-interaction-document-and-block-scanning/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🧾️ink-document-model-and-edit-helpers/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/✒️scene-ink/🧪️tests/🦀️.rs
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/note-wgpu-point-eraser-audit.md
```
