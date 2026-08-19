# terra — packet `gate-ui` report

**Crate:** `semio-framework-ui` (`🧰️framework/🔨️modules/🖱️ui/**`)
**Scope:** hand-work residue after the sibling's await-fixpoint (276→141 errors on `--lib --features wgpu`).

## Headline numbers (all commands: `CARGO_TARGET_DIR=<scratchpad>/target-gate-ui`)

| command | before | after |
|---|---:|---:|
| `cargo check -p semio-framework-ui --lib --features wgpu` | 141 | **3** (all in one registrar-only generated file — see Blocker below) |
| `cargo check -p semio-framework-ui --all-targets --features wgpu` | 307 | **3** (same file, counted twice: lib + lib-test) |
| `cargo check -p semio-framework-plugin --lib` | (blocked by the above 3, transitively) | **3**, all attributable to the same one file |

Every error that was mine to fix — 138 on `--lib`, 304 on `--all-targets` — is fixed. The only errors left anywhere in the dependency graph downstream of this packet trace to a single non-owned file.

## Blocker — `lease-request`

`🧰️framework/🔨️modules/🖱️ui/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs` is listed under the ticket's
"Registrar-only files" as `all 🤖️generated/**`, so I did not edit it (it's also `.gitignore`d, confirmed via
`git check-ignore -v`, and I found no generator script under my owned path that produces it — searched
`🖼️assets/**` and the crate's two `build.rs` files, no hit).

Two methods on `impl IconName` were blindly asyncified and are consumed by external-trait impls that must stay sync:
```rust
// current (broken):
pub async fn as_str(self) -> &'static str { ... }         // consumed by impl Display for IconName (E1)
pub async fn from_str(s: &str) -> Option<Self> { ... }     // consumed by impl From<&str> for IconName (E1)

// needed:
pub fn as_str(self) -> &'static str { ... }
pub fn from_str(s: &str) -> Option<Self> { ... }
```
Both are pure lookups (no I/O) — same R9 shape as everything else in this report. Whoever owns this generator
(or has permission to hand-edit `🤖️generated/**`) can apply this two-line fix and the whole chain (`ui` → `plugin`
→ SDK) goes green. I recommend sol or the owner of the icon-generation pipeline take it — I did not chase the
generator further outside my owned path.

## Mechanism per family (why each fix was made the way it was)

### 1. R9 reversions — pure computation forced sync by an external-trait consumer
The dominant shape, exactly as the ticket predicted (`#[derive(Serialize)]` on fields whose default-value
functions were blindly asyncified). Verified I/O-free (arithmetic/string/lookup only), then de-asyncified and
tagged `// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9`, removing the
matching `.await` at every call site in the same edit:

- 28 `#[serde(default = "…")]` value functions in `component.rs` (`kind_window`, `kind_stack`,
  `world3d_default_*`, `tiled_map_default_*`, `board2d_default_*`, `ink_canvas_default_selection_json`, …).
- `is_default` (free fn) and `UiPresence::is_default` — consumed by `#[serde(skip_serializing_if = "…")]`,
  which is a bare-fn-value / sync-only call site, not a method call.
- `LocalizedLabel::{from_fn, data, native, resolve}` in `🦀️label.rs` — consumed by hand-rolled
  `impl Serialize`/`impl Deserialize for LocalizedLabel`.
- `Label::into_string` — the sole call site is `Option::map(Label::into_string)`, a bare-fn-value sync
  position (same shape as `is_default`).
- `ContextMenuItemSpec` helper family (`context_menu_is_bare_separator`, `_is_header`, `_is_group_row`,
  `_group_category`, `_taxonomy_rank`, and a nested `bucket_mut`) — forced sync by `sort_by_key` comparators
  and `Option::map` fn-values (R10 residue shape 1's own suggested fallback: "if the awaited fn is a pure
  accessor — R9").
- `no_category` (test helper) — forced sync by `&dyn Fn(&str) -> Option<String>`.
- Entire `theme.rs` pure color/metric graph (`Rgba::{from_srgb8, from_chrome, with_alpha}`, `chrome_px`,
  `panel_width`, `hsl_to_srgb8`, `presence_rgba`, `from_chrome`, `Theme::{light, dark, for_name, surface,
  glass, presence_color, glass_mip_level}`) plus `ui_styling::color::{rgba8_to_linear, linear_to_rgba8}` and
  `ui_styling::appearance::AppearanceName::{board, map, canvas, chrome, parse}` in the sibling `🎨️styling`
  crate — `impl Default for Theme` (E1) calls `Theme::dark()`, and dozens of pre-existing sync call sites
  throughout `paint.rs`/`draw.rs` (not in this packet's compiled scope, but real evidence of intended
  sync-ness) already call `theme.glass(...)`/`theme.surface(...)` without `.await`. Reverted the whole
  connected pure-computation component rather than push `.await` through it, per R9 §3's "make the consumer
  sync" branch — the consumer here (`Default`, plus the wider paint code) cannot go async, and every fn in
  the chain is provably I/O-free.
- `PresenceBar::{presence_color, presence_css_var}` — same reasoning, consumed sync by `theme.rs::from_chrome`
  and by the module's own tests.
- `impl Default for UiNode` — rather than asyncify `ui_stack_vertical` (which has ~70 legitimate async call
  sites elsewhere), inlined its one-line pure body directly into `fn default()`.

### 2. Missing `.await` (plain residue, hand-applied where `insert-await.py` declined, tool-applied elsewhere)
Ran `insert-await.py --crate semio-framework-ui --all-targets --features wgpu --scope 🧰️framework/🔨️modules/🖱️ui`
to fixpoint (4 passes, 89 span-keyed edits, 0 ambiguous) after tagging every `#[test]` as
`#[semio_framework_async_macros::async_test]` (added `semio-framework-async-macros` as a dev-dependency in
this crate's `Cargo.toml` — the standard bridge, see its own doc comment). Remaining hand-fixes were mostly
awaits the tool's diagnostic codes don't cover (`E0271`, `E0608`) or awaits nested inside already-broken
expressions the tool refuses as unsafe context.

### 3. Repeated `.await` on one future (R10 residue shape 2 / "shape 6" from the coordinator's list)
`let mut x = Ctor::new(..)` left un-awaited, then every use written `x.await.field` / `x.await` again.
Fixed the constructor once (`let mut x = Ctor::new(..).await;`), then plain field access — in
`ui_inspector_mixed_number`/`_toggle`, `TextEditorScene::{json_view, code_input}`, `World3dScene` points-json
test, and four `UiTreeItemNode::base(...)` presence tests (`loading`/`waiting`/`hidden`/`celebrating`).

### 4. Self/mutually-recursive async fns → `Box::pin` at the call site (R10 residue shape 3)
`organize_context_menu` (recurses into `children`), `collect_window_kind_ids_from_children` (Axis variant
recurses), `ui_declarative_child_to_tree_item`'s `Group` arm (recurses into its own children) — wrapped the
recursive call in `Box::pin(...)`, left the fn itself as plain `async fn` (never introduced a `dyn Future` in
return position — that stays banned under R1).

### 5. `.await` inside a sync closure / `.collect()` of futures (R10 residue shape 1)
`Vec<T>::collect()` over a `.map(|x| async_fn(x))` iterator — hoisted into an explicit `for` loop pushing the
awaited value, in `create_stack_layout`, `create_default_layout`, `even_window_layout`,
`ui_declarative_sections_to_tree`, the `Group`/overflow-fold branches of `ui_declarative_child_to_tree_item` /
`organize_context_menu_*` tests, and `PresenceBar::build_presence_bar_localized`'s peer-row loop.

### 6. One genuinely pre-existing, non-async defect (documented, not silently absorbed into R9)
`scene_records_serialize_to_golden_json`'s `NodeGraphScene::base(...)` call passed JSON-string args
(`"[]".into()`) against a signature that takes typed `Vec<NodeGraphNodeRecord>`/`Vec<NodeGraphEdgeRecord>`/
`NodeGraphViewport` — a schema drift unrelated to asyncify (the struct has real typed fields, not
`*_json: String` like its scene-record siblings). Fixed the call site with real empty vecs and a default
viewport, and corrected `GOLDEN_SCENES_JSON`'s node-graph fragment to match
(`{"nodes":[],"edges":[],"viewport":{"x":0.0,"y":0.0,"zoom":1.0}}`) rather than leaving it broken or
mis-filing it as an async residue shape.

### 7. One stray syntax defect unrelated to await
`window_layout_serializes_to_golden_json`'s fixture literal was missing a comma
(`instance_id: None, template_id: None  corner: None`) — fixed independently of the async work; it happened
to be adjacent to an await-insertion edit but the tool's own edits are span-keyed and this was pre-existing.

## Files touched (all inside the owned path `🧰️framework/🔨️modules/🖱️ui/**`)
- `📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`
- `📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️theme.rs`
- `📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs`
- `📦️packages/🦀️rust/Cargo.toml` (added `semio-framework-async-macros` dev-dependency)
- `🧱️elements/👥️PresenceBar/🧊️component.rs`
- `🎨️styling/📦️packages/🦀️rust/📦️glue.rs`

**Not touched:** `SceneHost`/`scene_slots.rs`, any `dyn` seam, anything under `wgpu-engine` feature gating
(`paint.rs`, `draw.rs`, `engine.rs`, `events.rs`, `cursor.rs`, `shell.rs`, `reconcile.rs`, `flex.rs`, `text.rs`,
`tree.rs`, `arena.rs`, `layout.rs`, `input.rs`, `kernel_3d_scene`, `widgets.rs`) — these are gated behind
`wgpu-engine`, not `wgpu`, so they are outside both acceptance commands' compiled surface and outside this
packet's scope; they carry their own large `#[test] async fn` residue (421 occurrences crate-wide, only 46 of
which — `component.rs` + `PresenceBar` — are actually compiled under `--features wgpu`) that a future packet
should scope separately if `wgpu-engine` ever needs to go green.

## Exit codes observed (pasted, not inferred)
```
$ CARGO_TARGET_DIR=.../target-gate-ui cargo check -p semio-framework-ui --lib --features wgpu
error: could not compile `semio-framework-ui` (lib) due to 3 previous errors; 2 warnings emitted
LIB_EXIT=101   (3 errors, all icon_name.rs:1525/1532/1573)

$ CARGO_TARGET_DIR=.../target-gate-ui cargo check -p semio-framework-ui --all-targets --features wgpu
error: could not compile `semio-framework-ui` (lib) due to 3 previous errors
error: could not compile `semio-framework-ui` (lib test) due to 3 previous errors
ALLTARGETS_EXIT=101   (same 3 locations, counted once per target)

$ CARGO_TARGET_DIR=.../target-gate-ui cargo check -p semio-framework-plugin --lib
error: could not compile `semio-framework-ui` (lib) due to 3 previous errors
PLUGIN_EXIT=101   (same 3 locations — semio-framework-plugin has NO errors of its own; every
                    error in this run is the ui crate's dependency-graph propagation of the
                    icon_name.rs blocker)
```

## What a sibling / coordinator needs to know
1. **The whole SDK is one 2-line fix away from unblocking on the `semio-framework-plugin` side** — apply the
   `as_str`/`from_str` sync fix above to `icon_name.rs` (registrar territory) and re-run
   `cargo check -p semio-framework-plugin --lib`.
2. `semio-framework-ui`'s `wgpu-engine` feature (paint/draw/engine/events/cursor/shell/…) was NOT touched and
   was never in this packet's compiled scope (`--features wgpu` only, per the brief). It has its own
   large async-test residue if a future packet needs it.
3. No `dyn`/`SceneHost` work was touched, per the coordinator's note.
