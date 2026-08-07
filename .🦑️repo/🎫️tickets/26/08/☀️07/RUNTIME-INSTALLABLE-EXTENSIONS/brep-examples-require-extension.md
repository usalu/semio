# Procedural3d examples requiring the Brep flow extension

These `.semio` graphs use `brep.*` operator kinds. After Wave 3.c those operators are **not** compile-time builtins.

They work when the packaged extension `semio-s-plugin-flow-extension-brep` (`flow-extension-brep` / manifest id `brep`) is **installed and enabled** for hosts `flow-play` and `procedural3d-play` (dual `Contribution::FlowExtension`).

Operator kind ids are unchanged (`brep.prim3d.box`, `brep.solid.fillet`, …) — no graph rewrites required; contribution registration supplies the same kinds at runtime.

## Graphs

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/box-fillet-preview/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/sphere-cut-with-torus/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/rectangle-wire-preview/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/sphere-box-fuse/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/rectangle-extrude-volume/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/box-shell-preview/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/face-sweep-extrude/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.procedural.procedural3d.dsl.semio`
