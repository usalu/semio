# Current General to Product Cargo Source Roster

Read-only authored TOML inventory: 53 general module manifests, 10 declared product path edges across 6 consumer manifests. Workspace inheritance and target/build/dev sections were parsed with existing @iarna/toml. These are exact declared source edges, not a Cargo resolution/native runtime or no-follow admission receipt. No Cargo was run.

| Consumer manifest | Line | Owner | Section | Binding / package | Product provider path | Optional |
| --- | --- | --- | --- | --- | --- | --- |
| 🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml | 24 | semio-framework-tool-run | dependencies | semio-framework-os-kernel / semio-framework-os-kernel | 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml | 37 | semio-framework-editor | dependencies | semio-framework-os-kernel / semio-framework-os-kernel | 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml | 39 | semio-framework-editor | dependencies | infinite_canvas / semio-framework-os-infinite | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml | 84 | semio-framework-ui | dependencies | semio-framework-os-kernel / semio-framework-os-kernel | 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust | true |
| 🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml | 42 | semio-framework-surface | dependencies | semio-framework-artifact-infinite-dag / semio-framework-artifact-infinite-dag | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml | 45 | semio-framework-surface | dependencies | infinite_canvas / semio-framework-os-infinite | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml | 46 | semio-framework-surface | dependencies | semio-framework-os-kernel / semio-framework-os-kernel | 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml | 25 | semio-framework-graph | dependencies | semio-framework-os-kernel / semio-framework-os-kernel | 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml | 26 | semio-framework-graph | dependencies | neural_engine / semio-framework-os-kernel-neural-engine | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust | false |
| 🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/Cargo.toml | 22 | semio-framework-compiler | dependencies | dsl_core / semio-framework-os-kernel | 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust | false |

The main Schema, lower State/Composition/Registry, Value, Replication, general2D and general3D current authored manifests contain no declared product path edge in this inventory. Remaining next owner families are compiler DSL, Graph neural-engine/DSL, UI locale/DSL, editor and surface canvas/DSL, and tool-run DSL. This inventory does not prove their source deletion closure or decide the complete extraction boundary.

## Bounded Next-Provider Source Witnesses

Current UI Rust package root11 still aliases product Kernel as dsl, with wgpu feature enabling that optional edge. Actual IconName value owner24–34 implements dsl traits through foreign serde_json conversion; action13/reconcile19/tree278 also name dsl Value. Current node-wire original test430 constructs product Kernel Viewport2d rather than UI-owned viewport. PresenceBar130 uses Locale::default, so a locale-authority cut requires real explicit-host-locale semantics rather than only Cargo deletion. Those are read-only current candidates; no source edits or UI tests ran here.

Graph LayoutRun manifest already directly binds neutral Value but transitively selects UI wgpu and ToolRun. Its comments about derives still name retired OS-root output; actual product closure remains via those direct generic dependencies. Redirecting one Value path does not prove LayoutRun deletion closure. ToolRun current real schema/record/control/Pack dependencies were independently inspected by Root; a proper record/parser/codec extraction is required if selected.
