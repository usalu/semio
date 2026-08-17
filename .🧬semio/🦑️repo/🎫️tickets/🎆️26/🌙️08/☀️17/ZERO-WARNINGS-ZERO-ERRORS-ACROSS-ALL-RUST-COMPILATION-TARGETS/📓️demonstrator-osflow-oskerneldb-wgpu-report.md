# Report: demonstrator, os-flow, os-kernel-db, os-renderer-wgpu

All four crates verified `(lib)` target: **0 warnings, 0 errors**. `(lib test)` untouched (pre-existing,
out-of-scope migration breakage — verified no *new* test-target errors were introduced by this pass,
see per-crate notes below).

## 1. `semio-s-plugin-demonstrator` — was reported possibly still broken; now: 11 → 0 warnings, 0 errors
`(lib)` target compiles clean. The `Mutex` import bug in `semio-framework-os` that used to block this
crate's dependency graph is confirmed already fixed by an earlier part of this session — no new fix
needed there.

Real bug found and fixed (same class as the earlier `os-kernel` "js" feature bug this session):
`📦️glue.rs`'s `#[cfg(feature = "plugin-entry")] semio_framework_plugin::plugin_exports!(manifest::plugin);`
gated the crate's OWN `semio_plugin_install_bundle` wasm export behind a `plugin-entry` Cargo feature
that this crate's `Cargo.toml` **never declared** (only the six *bundled pane* dependencies declare and
toggle that feature, via `default-features = false`, to avoid duplicate-symbol wasm links — this crate is
the terminal bundle, not embeddable elsewhere, so it has no reason to carry the same toggle for its own
export). The gate was therefore permanently false: the demonstrator wasm component never exported its
plugin-install entry point on any build. Fixed by making the export unconditional (confirmed via
crate-wide `Cargo.toml` grep: no other crate depends on `semio-s-plugin-demonstrator` as a lib, so there
is no scenario needing the toggle). This also resolved 4 cascading `dead_code` warnings (the manifest's
`PLUGIN_ID`/`PLUGIN_LABEL`/`PLUGIN_VERSION` consts and `plugin()` fn, all only reachable through the
export).

Also fixed: 5 unused imports (`ArtifactBuilder`, `semio_framework_plugin::ArtifactAnalyzer as _`,
`write_json_text`, `STDIO_JSON_DOCUMENT_SCHEMA`, `parse_json_text` in the export-side serializer), and 1
hidden-lifetime-in-type warning (`&[ComposeSource]` → `&[ComposeSource<'_>]`, matching every sibling
plugin's own `compose()` signature convention).

Files touched:
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`

## 2. `semio-framework-os-flow` — 11 → 0 warnings, 0 errors
Much lower than the ~127 measured earlier in the session (the workspace-wide `cargo fix --keep-going`
pass had already knocked most of it out before this crate was picked up).

- 6× unused `use crate::infinite::board::ports::directed_dag as dag;` (catalogue/registry/bridge/
  drawing/vcs: genuinely unreferenced anywhere in the crate, including tests — deleted; wasm's own copy
  IS used, but only inside a `#[wasm_bindgen(...)]`-gated wasm32 impl block — gated the import itself
  with `#[cfg(target_arch = "wasm32")]` rather than deleting it).
- 1× unused `use neural_engine as neural;` in `🌉️wasm/🦀️component.rs` (zero references anywhere in that
  file — deleted; every sibling file's own copy IS used via `use neural::{...}` sub-imports, left alone).
- 3× unused `use crate::drawing::*;` glob imports (catalogue/bridge/vcs — zero `drawing::` symbols used
  in any of the three, deleted).
- 1× private-type-leak (`FlowExtensionRegistryState` is a plain private `struct` but reachable via the
  `pub(crate) static FLOW_EXTENSION_STATE: LazyLock<Mutex<FlowExtensionRegistryState>>`) — confirmed a
  real cross-module consumer (`🖥️host/🦀️component.rs` reads `FLOW_EXTENSION_STATE` too), so widened the
  type to `pub(crate)` to match, per the established "widen, don't narrow" rule for confirmed real
  callers.

Files touched:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📚️catalogue/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️bridge/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`

## 3. `semio-framework-os-kernel-db` — 15 → 0 warnings, 0 errors
All 15 warnings were actually one root cause: `ambiguous_glob_reexports`-style "`X` is ambiguous:
ambiguous name" warnings (4 distinct names × several call sites) — the `--future-incompat-report` note
at the end of every earlier check for this crate was literally about this. Root cause: `db`'s own
`📦️glue.rs` aliases the *same* `semio-framework-os-kernel` crate under two different `extern crate`
names, `pack` and `protocol`. That crate's own root then does `pub use crate::os_pack::*;` **and**
`pub use crate::os_spr::*;` — both `os_pack::codec` and `os_spr::wire::codec` independently define
`read_varint_u64`/`write_varint_u64`, and both `os_pack::format` and `os_spr::format` independently
define `WriteOptions`/`VerificationLevel`, so referencing the bare crate-root re-exported name (e.g.
`pack::read_varint_u64`, the `pack::` prefix being the `extern crate ... as pack` alias, NOT a submodule
path) is genuinely ambiguous between the two. `db`'s own code consistently intended the `os_pack::`
family everywhere this fired (confirmed via `pack::PackSource`/`pack::PackFile`/`pack::PackLimits`
neighbors at every one of the ~19 call sites, never a `protocol::`-prefixed equivalent) — fixed by
qualifying each site to `pack::os_pack::{read_varint_u64,write_varint_u64,WriteOptions,VerificationLevel}`,
which resolves through the real submodule path instead of the ambiguous crate-root glob, with zero
behavior change. Left the shared `semio-framework-os-kernel` crate itself untouched (it was already
verified 0 warnings/0 errors earlier this session, and the ambiguity only actually bites a *consumer*
that references the bare re-exported name — os-kernel's own code never does).

Files touched:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️component.rs`

## 4. `semio-framework-os-renderer-wgpu` — 44 → 0 warnings, 0 errors
Much lower than the ~154-with-135-duplicates measured earlier (already reduced by the earlier
workspace-wide pass; the 44 seen here were all distinct, no duplicates reported by `cargo`).

### Wasm32/test-only reachability (gated, NOT deleted)
- **`🔬️Introspection` region** in `Interpreter/🧊️component.rs` (~330 lines: `DumpViewport`/
  `DumpNodeState`/`DumpNode`/`DumpStructure`/`DumpFrameStats` types + `ui_node_kind_tag`/
  `ui_node_declared_id`/`ui_node_path_segment`/`rgba_array`/`dim`/`dump_visual_fields`/
  `effective_hovered`/`walk_dump`/`primary_window_id`/`build_structure_dump`/`layer_is_nonempty`/
  `is_glyph_instance`/`build_frame_stats`, 17 items total, plus the `UiState` import they need): all
  real, all reachable only from the two `#[cfg(target_arch = "wasm32")] #[wasm_bindgen(js_name =
  "dumpStructure"/"dumpFrameStats")]` JS exports at the bottom of the same region (confirmed via the
  region's own doc comment: JS calls these off `window.wasmBindings` for the wgpu↔React UI-parity test
  harness) **and** from `#[cfg(test)] mod introspection_tests` in the same file — neither cfg alone
  covers both, so gated each item with `#[cfg(any(target_arch = "wasm32", test))]` rather than deleting
  live wasm-export code.
- **`draft_theme` field + `begin_custom_theme_draft`/`set_draft_theme_color`/`save_draft_theme`/
  `discard_draft_theme`** in `Shell/🧊️component.rs`: a doc comment two screens above them
  (`build_settings_theme_ui`) explicitly states these "stay unwired here on purpose" — a real,
  documented, deliberately-scoped-down ticket decision (`w3-prefs-i18n-themes`), not accidental dead
  code, and they have real round-trip tests (`custom_theme_draft_round_trips_and_deletes`,
  `discard_draft_theme_clears_in_progress_draft`) in `ui_prefs_themes_i18n_tests`
  (`#[cfg(all(test, not(target_arch = "wasm32")))]`). Gated the field and all 4 fns with that exact same
  compound cfg (verified via a scratch `rustc` compile that `#[cfg(...)]` on an individual struct field
  **and** its field:value pair in a struct-literal both compile cleanly). `set_active_ui_layout` got the
  same treatment (only ever called from the same test mod; its getter `active_ui_layout()` and sibling
  `set_active_theme_id` DO have real production callers and were correctly left alone).
- **`action_host_window_id`** in `Shell/🧊️component.rs`: doc comment describes real intended behavior
  ("Architecture Decision 8, P3/P4") but its only current caller is `Dock/🧊️component.rs`'s own
  `action_host_window_id_finds_scoping_window` test, under the identical
  `#[cfg(all(test, not(target_arch = "wasm32")))]` mod — gated to match.

### Deleted (confirmed truly dead — zero callers anywhere, including tests, including wasm-gated code)
- `Shell/🧊️component.rs`: `synthetic_panel_tab`, `execute_search_item` (superseded — real callers use
  `activate_search_item` directly instead), `app_icon_id`, `window_engagement_chrome_visible`,
  `render_palette` (a `// render_palette removed` marker comment one screen away confirmed this was a
  real, deliberate removal that just forgot to delete the now-orphaned function — comment deleted too),
  `set_active_worker_count` (unlike its sibling `set_active_ui_layout`, this one has **zero** callers,
  not even in tests — the sibling getter `active_worker_count()` remains, real caller intact).
- `Scenes/🧊️component.rs`, biggest single finding: `SceneDragMode::MoveNode`/`ConnectPort`/`Marquee`
  enum variants (plus their match-arm handlers in `handle_scene_pointer_move`, plus a dead
  `moveMediaNode`-dispatching `if let` block in the pointer-up handler that could only ever match
  `MoveNode`). Traced end-to-end before touching: `SurfaceKind::NodeGraph` pointer-down/move/up route
  exclusively through `engine_canvas::node_graph_pointer_{down,move,up}` (confirmed via
  `handle_scene_pointer_button`'s own `match scene.component_kind` — the `NodeGraph` arm calls nothing
  but `engine_canvas::node_graph_pointer_down`), which itself delegates to a `flow`/`dag` plugin host
  object's `pointer_{down,move,up}_screen` methods — a completely separate implementation from this
  generic `SceneDragMode` system, confirmed by this same file's own doc comment on
  `excluded_from_generic_scene_dispatch` listing `NodeGraph` explicitly. `push_bezier`/`node_screen_pos`
  (also in the `NodeGraph` region) are the same story: `render_node_graph` (the real, live render path)
  delegates all painting to `engine_canvas::paint_node_graph{,_labels,_overlays}`, never these two local
  helpers. All five were dead-by-supersession (an old inline implementation not cleaned up after the
  `engine_canvas`/host-object cutover), not in-progress scaffolding — deleted with reasonable confidence
  after tracing the call graph, not guessed.
- `Scenes/🧊️component.rs` dead fields: `SceneDrag.button` (written at every one of 10 construction
  sites, never read anywhere — deleted the field and updated all 10 sites; verified each enclosing
  function's own `button: i16` *parameter* is independently used for real conditionals elsewhere in the
  same function, so this didn't cascade into new `unused_variables` warnings), `editor_cursor`/
  `hover_row_id`/`raster_digest` on `SceneSurfaceState` (zero references anywhere, not even a writer —
  deleted), `surface_args` fn (zero callers, deleted), and three `#[derive(Deserialize)]`-only JSON DTO
  fields whose data is parsed but never subsequently read by any Rust code:
  `BlockListBlockJson.description` (the *step*-level `BlockListStepJson.description` sibling field IS
  used for rendering; this one is the *block*-level field, confirmed separately unused),
  `HistoryColumnAuthorJson.id` (sibling `.name` IS used), `CanvasLayer.source`/`.target` (sibling
  `x0`/`y0`/`x1`/`y1` coordinate fields on the same big multi-kind-layer DTO ARE used for `"line"`-kind
  layers; `source`/`target` aren't consumed by anything). All four are `#[serde(default)]` `Option`
  fields, so removing them doesn't change JSON parsing behavior for any payload that still sends those
  keys — serde silently ignores unknown keys.
- Two "unused doc comment" warnings (`rustdoc does not generate documentation for macro invocations`):
  a `/// ...` block directly above a `thread_local! { ... }` macro invocation in `📦️glue.rs` (the
  `BOOT_APP_ROLE` static) and a `/** ... */` block above the `UI_ENGINE`/`POINTER_EDGE_STATE`
  `thread_local!` in `Interpreter/🧊️component.rs` — both converted from doc-comment to plain `//`/`/*`
  comment (content unchanged, matches the existing sibling `ICON_ATLAS_RUNTIME` thread_local's own
  plain-comment convention right above it).
- One genuine dead-store: `x += fixture_rect.w + theme.gap_standard;` in `Shell/🧊️component.rs`'s
  navbar-fixture rendering — traced forward through the rest of the function and confirmed `x` is never
  read again after that point (the function moves on to a `rx`-based right-aligned layout pass); deleted
  the line, no behavior change (pure arithmetic, no side effects).
- One irrefutable `if let`: `HostUserEvent` in `📦️glue.rs` has exactly one variant (`RuntimeReady`), so
  `if let HostUserEvent::RuntimeReady { .. } = event { ... }` always matches — rewritten to an
  unconditional `let HostUserEvent::RuntimeReady { .. } = event;` destructure per rustc's own suggestion.

### Codegen fix (touches the shared plugin-registry generator, verified safe)
- One `dead_code` warning came from a **`@generated by framework/plugin/registry/script.ts — do not
  edit.`** file (`🤖️generated/🦀️hosts.rs`, `#[path]`-mounted into this crate's `📦️glue.rs`):
  `DEFAULT_HOST_VARIANT` const, confirmed via repo-wide grep to have **zero** Rust consumers anywhere
  (only the TypeScript side's own `export const DEFAULT_HOST_VARIANT` in `script.ts` is real — used by
  `emitPlaygroundsTypeScript`). Per this repo's own rule (permanent logic lives in `📜️script.ts`, not
  hand-patched generated output), fixed the *source*: removed the dead `defaultHostVariant` parameter
  and its one Rust-template emission from `emitRustHosts()` in `script.ts`, updated its one call site,
  then hand-applied the exact equivalent, deterministic diff to the already-generated `🦀️hosts.rs` (did
  **not** run the full `generate` command, to avoid touching unrelated generated content in a live,
  concurrently-edited repo). Verified via `bun 📜️script.ts check`: the generator's own byte-comparison
  check confirms the hand-edited `🦀️hosts.rs` is **fresh** (byte-identical to what regeneration would
  now produce) — the "package discovery problems" that same check command also printed are pre-existing,
  unrelated to this change (a totally different validation category about manifest markers across dozens
  of unrelated plugin directories). Confirmed via grep that only this one crate mounts `🦀️hosts.rs` at
  all, so no other crate was affected.

### Verification of no new `(lib test)` breakage
`cargo check --tests -p semio-framework-os-renderer-wgpu` still fails (exit 101, 12 errors) — but every
one of the 12 is `E0433: cannot find type LocalizedLabel`/`UiPresence` in `Dock/🧊️component.rs` and
`Interpreter/🧊️component.rs`'s test code, unrelated to anything touched here (grepped the full error
list for every identifier this pass touched — zero matches). This matches the pre-existing,
already-documented "another session's in-flight migration" pattern from `📓️progress.md` — confirmed not
newly introduced by this pass, left alone per the ticket's explicit scope boundary.

Files touched:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Scenes/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🦀️hosts.rs`

## Left alone / not touched (judgment calls, noted rather than guessed)
None within these four crates' `(lib)` targets — all reached 0/0. The only things deliberately left
untouched were the wasm32/test-only-reachable items and the documented-as-intentionally-unwired cluster
described above (both handled via `cfg` gating, not deletion), and the pre-existing `(lib test)`
migration errors in `Dock`/`Interpreter` (out of scope per the ticket).

## Process notes
- Two `cargo check` invocations this run hit the Bash tool's own timeout and were auto-backgrounded by
  the harness (not something I initiated via `run_in_background`/Monitor) — both cases the harness *did*
  deliver a completion notification, unlike the "subagent never notified" failure mode
  `📓️progress.md` warns about for explicit backgrounding. Re-ran with correct `2>&1 | tee`/redirect
  ordering afterward regardless, to get clean, complete captured output for the later triage work.
- No git commands run. No `#[allow(...)]` used anywhere.
