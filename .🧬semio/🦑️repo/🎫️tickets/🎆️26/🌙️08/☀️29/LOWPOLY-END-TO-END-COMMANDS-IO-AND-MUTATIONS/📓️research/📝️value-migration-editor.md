# Lowpoly editor — value_derive/UI-contract follow-up migration

## Starting point
`cargo check -p semio-s-plugin-lowpoly --lib` mechanical-derive pass had already taken the crate from
427 → 159 error lines. My subtree (`$A/✏️editor/`) started at 76 `error[` lines (of 240 crate-wide) in
the first full snapshot I captured, and reached **52 remaining editor-attributed errors** in the last
full run I could analyze (`check-3.txt`) before upstream churn (see Blocked below) removed my ability
to get a fresh full compiler signal. Every one of those 52 has a concrete fix applied below; none were
skipped as "too hard" — the two genuinely large ones (window scene renders, inspector panel) were
ported, not stubbed.

## API changes adapted to

1. **`ActionDescriptor` split in two.** The struct itself (`{controller_id, action, args: Option<DslValue>}`)
   is UNCHANGED — it still lives at `ui_wgpu::wgpu::ActionDescriptor` — but the crate's own action-factory
   convention split into two builders: `lowpoly_action` (new contract, `ActionFactory::new(..).action(..)`,
   returns `UiAssemblyResult<(ActionId, Option<UiValue>)>`) for NEW contract nodes/bindings, and a new
   `lowpoly_window_action` (mirrors `cad_window_action`) for OLD retained window chrome
   (`WindowMeasure`/`WindowEngagement*`/`UiFieldNode`/`UiInputNode`/`UiToggleNode`), which still needs a
   real `ActionDescriptor`. Added `lowpoly_window_action` to `✏️editor/🦀️component.rs`; repointed every
   window-chrome call site (component.rs, `🛠️options/🗂️select`, `🛠️options/👁️show-edges`,
   `🛠️options/🌞️sun`, `📌️panels/🔍️inspection`) to it.
2. **`semio_framework_ui_contract::Label` has no `From<LabelText>`/`From<&str>`/`::data()`** — it's a
   minimal `TryFrom<String>`/`TryFrom<&str>` wrapper now (old wgpu `Label` keeps its `.data()`/`From<LabelText>`
   unchanged, coexisting). Added `ui_label(impl AsRef<str>) -> UiAssemblyResult<Label>` next to the other
   `ui_value_*` helpers (mirrors `cad_action`'s `ui_label`), used at every NEW-contract call site
   (`📌️panels/📄️artifact`, `🛍️catalogue`, `🗂️layers`, `🔍️inspection`).
3. **`protocol::Mutation<P>` (unrelated to `ActionDescriptor`) still only needed for `LowpolyConfigMutation`**
   already had it — `LowpolyConfig`/`LowpolyConfigMutation` already derive `ToValue`/`FromValue` and
   implement `Mutation<LowpolyConfig>` without `DESCRIPTORS`/`descriptor` (that trait item pair belongs to
   *document*-schema mutation leaves via `#[derive(dsl::Mutations)]`, a different, much heavier
   leaf-per-file macro scoped to `🧬️schema/🧬️mutations/*` — owned by the schema agent, not editor
   config). No E0046 surfaced against `LowpolyConfig`/`LowpolyConfigMutation` in my final analysis; ticket
   point 3 turned out to be already satisfied.
4. **`LowpolyTransient` needed hand-written `ToValue`/`FromValue`** (mirrors its existing hand-written
   `Serialize`/`Deserialize`, which delegate through `LowpolyTransientStateRef`/`...Wire` because `state`
   is `Arc<LowpolyTransientState>`). Added the two impls in `✏️editor/🖌️session/🦀️component.rs`,
   delegating to the Ref/Wire structs. Also removed `value_derive::FromValue` from
   `LowpolyTransientStateRef` (it holds `&'a` reference fields — `&TransformState`/`&PaintStrokeState`/
   `&BTreeMap<..>` don't and can't implement `FromValue`) and added `value_derive::FromValue` to
   `LowpolyTransientStateWire` (it had `#[value(rename_all)]` with no derive backing it — dead attribute,
   `E0433 cannot find attribute value`).
5. **Schema mutation module reshape** (owned by the schema agent, landed mid-session): each
   `🧬️mutations/<kind>/` folder's payload type moved from `mutations::<kind>::mutation::<Type>` to
   `mutations::<kind>::<Type>` directly. Fixed all 12 call sites across `✏️editor/🦀️component.rs`,
   `🖌️session/🦀️component.rs`, `🎮️commands/🖌️paint`, `🎮️commands/➕️add-primitive`,
   `🎮️commands/✏️patch-object` with a targeted `mutations::([a-z_]+)::mutation::` → `mutations::\1::` sed.
6. **`ArtifactOwnedToolJobRequest::command` is now `Box<LowpolyCommand>`**, not `LowpolyCommand`. Fixed
   the one direct-value call site (`lowpoly_command_admitted(request.command, ..)` →
   `lowpoly_command_admitted(&request.command, ..)`) in `build_tool_job`; the `.command_id()` method call
   and `*request.command` move already worked via autoderef/Box-move.
7. **`ArtifactEditor::render`/`render_with_request_context` now return `UiAssemblyResult<ComponentTree>`**
   (new contract) — previously window/panel renders returned the OLD retained `UiNode` directly. Ported:
   - `🎭️modes/✏️edit/🪟️windows/🌐️model` and `🎭️modes/🎨️paint/🪟️windows/🖼️uv`: their `render()` bodies
     were pure `build_world_3d_scene(..)`/`build_canvas_2d_scene(..)` wrappers around a typed
     `World3dScene`/`Canvas2dScene`. Both scene types already implement `semio_framework_ui_scene::SceneDoc`,
     so I swapped the old `build_world_3d_scene`/`build_canvas_2d_scene` (→ `UiNode`) for the NEW
     `scene_surface(id, SurfaceKind, &scene)` (→ `UiAssemblyResult<BuiltNode>`) — same scene payload,
     new envelope, zero behavior change to the emitted scene JSON.
   - `📌️panels/🔍️inspection`: rewrote the whole inspector (name text field, smooth-shading toggle,
     read-only active-utility field, 11 utility-param number fields, 2 groups + 1 section) from the OLD
     `UiFieldNode`/`UiInputNode`/`UiToggleNode`/`ui_inspector_groups_to_tree` API to the NEW
     `ui::field`/`ui::input`/`ui::toggle`/`ui::section`/`ui::column` builder API (`Trigger::Change`
     bindings via `lowpoly_action`, same action ids/payload shapes as before — `patchObject`,
     `setUtilityParam`). No field was dropped; the selection-summary rows were ALREADY dropped by an
     earlier ticket (26/08/14) per the pre-existing comment, not by me.
   - `lowpoly_render`'s dispatch `match` (top-level `component.rs`) now uniformly collects each branch's
     `UiAssemblyResult<BuiltNode>` and wraps with `built_to_component_tree`; the two `ui_text(..)`
     failure-fallback arms became `built_text_node(..).map_err(..)`.
8. **`engagement_token_matches`/`strip_engagement_prefix` became `async fn`** (framework-wide R2), but
   `app_commands!` dispatch requires every `handle` in this crate to stay plain sync `fn`, and this
   program's own U1 ruling explicitly forbids a `block_on` bridge on any wasm host path. Added a local
   sync mirror of the (pure, no-I/O) token-matching algorithm inside
   `🎮️commands/💬️engagement/🦀️component.rs` rather than bridging or changing `handle`'s signature.
9. **`store::print_document_spr` became `async fn`**, called from the genuinely-sync
   `reset_document_effect` (built for a fresh genesis envelope with empty edits/messages/conflicts, so
   its only real work — `validate_persisted_conflicts` over an empty conflict set — is non-suspending by
   construction). Unlike (8), duplicating VCS envelope-printing logic locally was not viable (drift risk
   on the wire format), so I used the **E5 executor-bridge exception** (`📌️important.md` R2: "at most one
   per crate, tagged `// 🚫️async: E5 executor bridge`") — a poll-once `block_on` mirroring the sanctioned
   `🎠️kernel/🦀️.rs::extension_activation_tests::block_on` shape, panicking (not silently wrong) if ever
   handed a non-genesis envelope that actually suspends. This is the crate's one E5 bridge.
10. Misc small mismatches: `.iter()` needed explicit method-call form for `Arc<BTreeMap<..>>` (a bare
    `for .. in &arc_btreemap` doesn't autoderef through `Arc` the way a method call does) in
    `segmented_extent`/`segment_at`; `UiText::try_from_string` returns `Result` not `Option`
    (`.ok_or_else` → `.map_err`) in `📌️panels/📄️artifact`; `world3d_sun_measures`'s action callback
    needed `lowpoly_window_action`, not `lowpoly_action`, in `🛠️options/🌞️sun`.

## Files changed (all under my exclusive `$A/✏️editor/` subtree)
- `✏️editor/🦀️component.rs` — `lowpoly_window_action`, `ui_label`, `lowpoly_render` match rewrite,
  `build_tool_job` Box fix, E5 `block_on` bridge for `reset_document_effect`, mutation-path fixes.
- `✏️editor/🖌️session/🦀️component.rs` — `LowpolyTransient` `ToValue`/`FromValue`, Ref/Wire derive fix,
  `Arc<BTreeMap>` iterator fix, mutation-path fixes (×9 call sites).
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️component.rs` — `render()` → `scene_surface`.
- `✏️editor/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️component.rs` — `render()` → `scene_surface`.
- `✏️editor/📌️panels/🔍️inspection/🦀️component.rs` — full port to the new `ui::` builder API.
- `✏️editor/📌️panels/📄️artifact/🦀️component.rs` — `ui_label`, `map_err` fix.
- `✏️editor/📌️panels/🛍️catalogue/🦀️component.rs` — `ui_label`, `.into_string()`.
- `✏️editor/📌️panels/🗂️layers/🦀️component.rs` — `ui_label`, dropped old `Label::data`.
- `✏️editor/🛠️options/🗂️select/🦀️component.rs`, `🛠️options/👁️show-edges/🦀️component.rs`,
  `🛠️options/🌞️sun/🦀️component.rs` — `lowpoly_window_action`.
- `✏️editor/🎮️commands/🖌️paint/🦀️component.rs`, `🎮️commands/➕️add-primitive/🦀️component.rs`,
  `🎮️commands/✏️patch-object/🦀️component.rs` — mutation-path fixes.
- `✏️editor/🎮️commands/💬️engagement/🦀️component.rs` — local sync `engagement_token_matches` mirror.

## Blocked (do not attempt to fix — out of my ownership, actively churning)
`cargo check -p semio-s-plugin-lowpoly --lib` cannot currently reach the lowpoly crate at all: its
dependency chain is red two crates deep upstream, both mid-migration by other agents right now
(confirmed via `git status` — files under `🧰️framework/🔨️modules/🌱️value/*` are modified/deleted live):
- `semio-framework-ui` (`🎯️targets/🧊️wgpu/🦀️component.rs`): `DslValue: Serialize` not satisfied on
  `ActionDescriptor`/`Keybinding` — `DslValue`'s `Serialize` impl was just removed
  (`🌱️value/🔀️serde/🦀️component.rs` deleted) as part of the same value_derive rollout.
- `semio-framework-os-kernel` (`🏪️store/🦀️component.rs:2280`): 22 errors, `&ArtifactDialect`/
  `&MigrationProvenance`/`&OwnerRef`/etc. missing `protocol::ToValue` — same rollout, further along.
Every fix above was verified by reading the target APIs' actual current source (not by compiling), since
no full compiler signal for my own files was obtainable this session. All the fixes are self-consistent
(type shapes checked by hand against the callee signatures I read), but the crate has NOT been proven to
compile clean end-to-end — that verification is blocked on the two crates above, not on anything in my
subtree.

## Route oracle
`bunx nx run "@semio-tech/lowpoly-js:test" --skip-nx-cache` — **PASS**: "47 Migrated, 0
BatchOnlyPendingRewrite"; Ajv hostile oracle clean (duplicate/missing-lane/non-null-blocker/lane-mismatch
all correctly rejected). This oracle reads fixture/manifest JSON, not the Rust build, so it is unaffected
by the upstream Rust blocker and confirms I did not touch any command's classification/dispatch shape.

## Handoffs
- Schema agent: mutations directory reshape (point 5) landed clean; no action needed from them on my
  side, purely informational.
- Whoever owns `🧰️framework/🔨️modules/🌱️value/*` / `🏪️store/🦀️component.rs` / `🎯️targets/🧊️wgpu`
  right now: once those compile, please re-run `cargo check -p semio-s-plugin-lowpoly --lib
  --message-format short -j 6 2>&1 | grep -E "error\[" | grep "✏️editor/"` — if anything surfaces in my
  subtree beyond typos, ping the ticket; I could not get that final confirming signal this session.
