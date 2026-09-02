# 🔧 `space`'s `⚙️engine/` slice — 50 errors, grouped by root cause

Scope: `✏️s/🔌️plugins/🪐️space/⚙️engine/` only (per
`/private/tmp/claude-501/-Users-ueli-Documents-semio/35c04cfe-e837-497b-a3e3-edbd93a5785f/scratchpad/slices/sx-engine.txt`).
Did not touch `🗿️artifacts/🏠️home/`, `🗿️artifacts/🪐️space/`, or the plugin root — those files shifted
under me during the session (peer agents), left untouched. Did not run `cargo`; read/reasoned/edited only.

## 1. Stale `#[path]`/`include_str!` mount (1 error)

`🎚️config/🧬️schema/🦀️.rs:57` — `include_str!("../../👥️presence/🧬️schema/🦀️component.rs")` pointed at a
filename a repo-wide rename sweep already retired; the file on disk is `🦀️.rs`. Fixed the literal path.

## 2. `command_from_action` trait signature: `Option<&pack::json::Value>` → `Option<&dsl::DslValue>` (1 error, E0053)

`ArtifactApp::command_from_action`'s `args` parameter changed from `pack::json::Value` to
`semio_framework::DslValue`. Rewrote the whole function (`🦀️.rs:689-775`): swapped the `use pack::json::Value`
import for `use semio_framework::DslValue` (added to the existing `semio_framework::{...}` import), and every
closure now calls `DslValue`'s own accessors (`.get`, `.as_str`, `.as_f64`, `.as_array`) instead of
`pack::json::Value`'s. `f64_field` simplified — `DslValue::as_f64` already widens every `Number` variant, so
the old `.as_f64().or_else(as_i64).or_else(as_u64)` chain collapsed to one call. `json_field` (needed for
opaque "value" fields staying JSON text) now does `raw.as_str().map(str::to_string).unwrap_or_else(||
serde_json::Value::from(raw).to_string())`, using the `DslValue → serde_json::Value` `From` impl framework
ships (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:218`). Pattern confirmed against an already-migrated sibling
(`✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️.rs:1502`).

## 3. Dual-compiled workflow-mutation types: `semio_framework::X` vs `semio_framework_os::workflow::X` (23 errors)

Root cause (verified by source read, not error-message inference): `🧰️framework/🛍️products/💻️os/🔨️modules/
🔁️workflow/🦀️.rs` is `#[path]`-mounted into BOTH `semio_framework` (flattened via `pub use workflow::*`) AND
`semio_framework_os` (as `workflow_kernel`, re-exported as `pub mod workflow`) — two separate nominal
compilations of the same source. `semio_framework_os`'s crate root explicitly re-exports only the 4
document/enum types (`WorkflowNode`, `WorkflowEdge`, `WorkflowParameter`, `WorkflowParameterBinding`) —
those are safe bare. The 10 mutation-payload structs (`AddNode`, `AddParameter`, `ChangeParameter`,
`RemoveNode`, `MoveNode`, `RenameNode`, `ConnectPorts`, `RemoveParameter`, `UnbindParameterField`,
`DisconnectEdge`, `BindParameterField`) are NOT in that explicit list, so a bare `semio_framework_os::X`
import silently falls through to `semio_framework_os`'s `pub use semio_framework::*;` glob and resolves to
the WRONG duplicate. Fix: import these 10 from `semio_framework_os::workflow::X` explicitly everywhere.
Files touched: crate-root `🦀️.rs` (`ConnectPorts`), `⚙️engine/🦀️.rs` (`AddNode`, `AddParameter`,
`ChangeParameter`), and 13 command files (`delete-selection`, `node-graph-edit`, `remove-app-instance`,
`rename-app-instance`, `reorganize-workflow`, `unbind-parameter-field`, `remove-parameter`,
`patch-media-nodes`, `patch-app-instances`, `move-media-node`, `disconnect-media-edge`,
`bind-parameter-field` ×2 mutations + the `WorkflowParameterBinding` fallout that came free once
`BindParameterField`'s own field type resolved correctly).

## 4. Missing `.await` on now-async framework calls (16 errors)

A framework sweep made several previously-sync framework fns `async`. Added `.await` at each call site,
no other change:
- `selection_domains_from_surface` (now async) — `🦀️.rs:197`.
- Every per-window/panel `definition()` fn (`ModeDefinition`/`WindowKindDefinition`/`PanelTabDefinition`
  builders) and `space_play_layout()` are `pub async fn` — `create_space_app()`'s builder chain was
  awaiting the OUTER `.mode_def(...)`/`.window_kind_def(...)` call but not the INNER `definition()` future
  passed as its argument. Fixed 8 call sites (`🦀️.rs:881-889`).
- `ActionDefinition::with_category` became `pub async fn` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:926`) —
  7 call sites in `create_space_app()` were missing the inner `.await` (`🦀️.rs:897-928`).
- `open_instance::apply` wraps `open_with_selection` (async) in `Ok(...)` without awaiting — added
  `.await` (`🎮️commands/🔍️open-instance/🦀️.rs:45`).

## 5. `app_commands!`-generated `dispatch()` is sync-only; some `handle` fns were wrongly `async` (1 error, but fans out)

The macro's generated `SpaceCommand::dispatch(&self, doc, cfg) -> Result<Emit<...>, Fault>` calls
`$module::handle(payload, doc, cfg)` directly, no `.await` — it is a plain sync fn (confirmed: it's called
from `space_bounded_reduce`, a genuine `fn` pointer target passed to `BoundedArtifactCommandWork::new`).
14 command modules had been marked `pub async fn handle` despite bodies that never suspend (verified by
reading every one — no `.await` in any body). Stripped `async` from all 14: `set-app-registrations`,
`set-active-panel-tab`, `import-media`, `copy-app-instance` (only its `handle`, NOT its `apply`, which
genuinely is async and awaited from `SpaceApp::handle`), `presence-heartbeat`, `import-media-payload`,
`go-home`, `close-focused-instance`, `compiled-dag-engagement-input`, `import-space-pack`,
`navigate-virtual-file-system-node`, `workflow-engagement-input`, `set-active-example`,
`compiled-dag-engagement-submit`. This is what the compiler reported as one aggregate "match arms have
incompatible types" error at the `app_commands!` invocation site (`🦀️.rs:234`).

## 6. `u64`/`usize` mismatch (1 error)

`ToolExecutionContract::bounded_first_step`'s 3rd param (`max_work_units`) is `u64`;
`SPACE_BOUNDED_WORK_ITEMS` is `usize` (correctly — `ArtifactRetainedCommandPayload::try_new_with_context`'s
`maximum_work_items` param, the OTHER place this constant is used, really is `usize`). Cast at the one call
site that needs `u64` (`🦀️.rs:318`) rather than changing the constant's type.

## 7. `UiNode`→`ComponentTree` SDK migration (6 errors — 3 fixed, 2 flagged, 1 partially)

`ArtifactApp::render` now returns `UiAssemblyResult<ComponentTree>` (contract-based), not the legacy
`ui_wgpu::wgpu::UiNode`. This is the framework-wide "sdk-flip" (ticket `26/08/20/
SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`), not an async-convention issue.

**Fixed** (scene-surface leaves — mechanically portable once the record types are swapped):
- `🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs` — `NodeGraphScene`/`NodeGraphViewport`/etc. now come from
  `semio_framework_ui_scene` (new dependency added to `📦️packages/🦀️rust/Cargo.toml`, same declaration
  style already used by `semio-s-plugin-stdio`/`demonstrator`/`animate`/`fem`/`puzzle`), field-for-field
  identical to the old `ui_wgpu` versions. `build_node_graph_scene(...)` replaced with
  `semio_framework_plugin::scene_surface(id, semio_framework_ui_contract::SurfaceKind::NodeGraph, &scene)`.
- `…/🪟️windows/🕸️compiled-dag/🦀️.rs` — same pattern, `TextEditorScene`, `SurfaceKind::TextEditor`.
- `…/🪟️windows/🗂️media-vfs/🦀️.rs` — same pattern, `VirtualFileSystemScene`, `SurfaceKind::VirtualFileSystem`.
  Flagged in a comment: the old `build_virtual_file_system_scene` also took `pane_id`/`binding_id` params
  with no equivalent on the new `SurfaceBuilder` (only `NodeBase` + `SurfaceProps`) — dropped, not silently
  faked.
- Crate-root `🦀️.rs`'s `render()` dispatcher: wired the 3 fixed arms plus `catalogue::build_catalogue_tree`
  (already returned `UiAssemblyResult<BuiltNode>`, just wasn't wrapped) through
  `.map(semio_framework_plugin::built_to_component_tree)`. Also fixed the `_ =>` fallback arm, which called
  `semio_framework_plugin::ui_text(...)` (the OLD `UiNode`-returning helper via a `pub use ui_wgpu::wgpu::*`
  glob at the plugin crate root — mistyped, same latent bug I found copy-pasted into an already-migrated
  sibling artifact viewer) — replaced with `built_text_to_component_tree(...)`.

**Left unconverted, flagged in-code and here** — `📌️panels/🔢️parameters/🦀️.rs` and `📌️panels/🔍️inspection/🦀️.rs`.
These are NOT scene-surface leaves: they hand-build `UiSectionNode`/`UiFieldNode`/`UiNumberStepperNode`/
`UiSelectNode`/`UiToggleNode`/`UiInputNode`/`UiButtonNode` trees directly (~540 combined lines, ~30 node
constructions). No drop-in bridge exists from the old `UiNode` enum to the new `BuiltNode`/`ComponentTree`
(confirmed by search — only the `BuiltNode → ComponentTree` half of the bridge ships). A real port needs
the `ui::*` contract DSL per node (`ui::field`, `ui::input`, `ui::select`, `ui::toggle`, `ui::button`,
`ui::tree`/`ui::tree_section`/`ui::tree_item`), and the old absolute/delta number-stepper semantics have no
CONFIRMED equivalent builder yet — `ui::slider` exists but is a plain range slider, not verified to carry
the same `on_absolute`/`on_delta` action-binding shape. No other plugin in the repo has done this exact
node-graph-adjacent Field/Stepper port yet to copy from (checked `🕸️dag`, still on the old pattern too).
Given no `cargo` verification loop is available here, I left these two files and their dispatcher arms
exactly as they were (still `UiNode`-returning, still un-wrapped) rather than guess at ~30 node
translations blind. The crate-level `render()` match will still fail to fully unify because of these two
arms — that is a smaller, more localized remainder than the original 6-line error class.

## Files touched

- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/⚙️engine/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/{🔄️workflow,🗂️media-vfs,🕸️compiled-dag}/🦀️.rs`
- 14 command files listed in §5, plus `🎮️commands/🔍️open-instance/🦀️.rs` (§4) and 12 more listed in §3
  (some overlap with §5's list)
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` (added `semio-framework-ui-scene` dependency)

Not claimed to compile — no `cargo` run. `📌️panels/🔢️parameters/🦀️.rs` and `📌️panels/🔍️inspection/🦀️.rs`
are the one deliberately-unfinished item; everything else in the 50-error list was addressed at the
source-reasoning level.
