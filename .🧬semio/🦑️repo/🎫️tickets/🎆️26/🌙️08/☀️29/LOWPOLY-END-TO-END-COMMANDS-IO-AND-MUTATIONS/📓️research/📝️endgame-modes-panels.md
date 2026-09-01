## Endgame — 🎭️modes / 📌️panels / 🎮️commands / ⚙️engine

**SurfaceKind resolution**: `semio_framework_plugin::SurfaceKind` (crate-root glob re-export via
`pub use ui_wgpu::wgpu::*;`) is the LEGACY wgpu-target enum, distinct from
`semio_framework_ui_contract::SurfaceKind` that `scene_surface`'s own signature actually resolves to
(via a private `use semio_framework_ui_contract::*;` in that inner scope). Both are real, separate
types — not a re-export of one by the other. Confirmed canonical pattern from a landed sibling
(`🧩️puzzle/🧊️3d/…/🧊️main/component.rs`): keep the `semio_framework_plugin::SurfaceKind` import for the
`WindowKindDefinition.surface_kind` field, but pass the fully-qualified
`semio_framework_ui_contract::SurfaceKind::...` directly at the `scene_surface(...)` call site. Applied
to both:
- `✏️editor/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️component.rs:105`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️component.rs:165`

**Await fixes** — both call sites landed on now-async APIs, fixed with a plain `.await` (no
sync/async bridge needed, both are inside `#[async_test] async fn`):
- `✏️editor/📌️panels/📄️artifact/🦀️component.rs:122` — `testkit::render(...)` is now `pub async fn`
  (confirmed in `✏️editor/🦀️component.rs:2098`, owned by another agent). Changed to
  `render(&mut a, ...).await.contains(...)`.
- `✏️editor/🎮️commands/🌞️sun/🦀️component.rs:97` — `a.window_measures()` is now async. Changed to
  `a.window_measures().await` before `.is_empty()`.

No command handler logic, mutation shape, or interactive-job classification touched — compilation-only
fixes in all four cases.

**Final error count in owned directories (🎭️modes, 📌️panels, 🎮️commands, ⚙️engine): 0.**

Remaining `cargo check -p semio-s-plugin-lowpoly --all-targets` errors are outside scope: two
`E0119 conflicting implementations of Serialize/Deserialize for DslValue` in
`🧰️framework/🔨️modules/📡️replication/…/🌱️value/🦀️component.rs` (framework-level, not lowpoly, not my
files) plus the other agents' known in-flight errors in `✏️editor/🦀️component.rs` and
`✏️editor/🖌️session/🦀️component.rs`.
