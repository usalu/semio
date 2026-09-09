# Wave M — `engagements` / `measures` / `tools` as first-class retained surfaces

Date: 2026-09-09 · Target: React shell (wgpu stub untouched) · Ticket: `26/09/02/PUZZLE-3D-END-TO-END`

## 1. The defect, restated

The React shell's `refreshUi` turned only `PluginUiRefreshRequest.windows` / `.panels` into
`surface-visible` events and projected only window/panel retained surfaces back. The request's
`engagements` / `measures` / `tools` sections were built by `buildUiRefreshRequest` and consumed by
`ShellHost` (`cache.get("measures")` → `SET_WINDOW_MEASURES_BY_WINDOW_ID`, …) but **never produced by
anything**, because the only Rust producer was the legacy JSON `RefreshRequest` handler
(`plugin_refresh_ui`) whose wire was deleted in channel v12. Measured symptom: `windowMeasuresByWindowId
=== {}`, `toolMeasuresByToolId === {}`, Fill tool panel with only its activate toggle.

## 2. What was built

The three sections are now **reserved retained surfaces**, published, re-published and paged by exactly
the same `surface-visible` → `plugin_mount_surface` → `dirty.try_surface` → `plugin_render_surface` →
reconcile → `UiPatch` law as a window body.

### 2.1 Reserved identities (one declaration, two mirrors, one neutral fixture)

| section key (response field / refresh-cache key) | reserved body key **and** retained surface key | plugin accessor |
| --- | --- | --- |
| `engagements` | `framework.section.engagements` | `PluginApp::window_engagements` |
| `measures`    | `framework.section.measures`    | `PluginApp::window_measures` |
| `tools`       | `framework.section.tools`       | `PluginApp::tool_measures` |

Retained surface id is `${instanceId}:${bodyKey}`, e.g. `7:framework.section.measures`. The surface key
is the *body key* (not the bare section key) so it can never collide with an app-authored window
instance id or panel tab id, which are plain identifiers.

- Rust: `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4357-4407` (enum at `:4370`) — `UiRefreshSection`,
  `UI_REFRESH_SECTION_KEYS`, `UI_REFRESH_SECTION_BODY_KEYS`, `key()`, `body_key()`, `from_body_key()`.
  Re-exported at the `semio_framework` root via the existing `pub use manifest::*`.
- TypeScript: `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1134-1160` (`UI_REFRESH_SECTIONS` at `:1146`) — `UiRefreshSectionKey`,
  `UiRefreshSection`, `UI_REFRESH_SECTIONS`, `sectionViewContext`.
- Language-neutral declaration both sides are pinned against:
  `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️ui-refresh-section/🔣️.json`.

### 2.2 TS — request → section surface events

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`

- `uiRefreshSectionTargets` (`:1132`) — the sections this request asks for, in contract order.
- `sectionValueFromBuiltNode` (`:1141`) — rebuilds the section value from its retained tree.
- `uiRefreshSurfaceEvents` (`:1156`, section tail at `:1194`) — emits one `surface-visible` per requested
  section, with the reserved body key and **`sectionViewContext(request.viewState)` — the FULL,
  unnarrowed view**. This is load-bearing: `window_measures` / `window_engagements` iterate
  `view_state.window_instances` and call `for_window_instance` themselves, and `tool_measures` keys off
  `active_tool_id`; any window/panel narrowing here returns `{}` (proven in §4.3).
- `retainedUiRefreshResponse` (`:1206`) and `ownedUiRefreshResponse` (`:1367`) project each section
  surface into `PluginUiRefreshResponse.engagements/measures/tools` as `{ key, hash, value }`, where
  `hash` is the retained surface hash and `value` is the parsed section map. `applyUiRefreshResponseToCache`
  then fills `cache.get("measures")` / `("tools")` / `("engagements")` unchanged.
- `refreshUi`'s `missingSurfaceIds` / `hasRequiredUiPatches` need no change: they are derived from
  `events`, which now include the section surfaces, so the settle loop waits for their first publication.
- `uiRefreshBodyKeys` deliberately unchanged — it reports *request-carried* body keys, and a section's
  body key is a contract constant, not something the request names.

### 2.3 Rust — mount and render

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs` (rewritten)

- `SurfaceBinding.section: bool`, set from `UiRefreshSection::from_body_key(&body_key).is_some()`.
- A section binding stores `window_id: None`, so `prune_windows` never evicts it when a window closes.
- New `SurfaceContexts.section_view: Option<ViewModel>` — the last full view a section was mounted with,
  kept **apart from the shared `view_state`** because every window/panel mount overwrites that one with
  its own projection, which would otherwise make a section's view depend on which surface mounted last.
- `get()` returns that raw view for a section binding; windows/panels keep the old
  `for_window_instance` / `for_panel` behaviour verbatim.
- `remove()` clears `section_view` once no section slot remains.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

- `plugin_mount_surface` (`:28361`) — skips the `for_window_instance` narrowing for a section body key
  (`(None, Some(window)) => narrow, _ => raw`), so the section keeps the shell's own full view.
- `plugin_render_surface` (`:28386`) — a mounted section body key **never reaches `app.render`**; it
  routes to `plugin_render_section` and returns no presence updates.
- `plugin_render_section` (`:28352`) + `canonical_section_json` (`:28339`) — calls the matching accessor
  and serializes it through a `BTreeMap`. The `BTreeMap` is not cosmetic: the accessors return
  `HashMap`s whose iteration order would otherwise change the retained text leaves — and therefore the
  surface hash — on every render without the section changing.
- `crate::app::section_component_tree` (`:420`, exported at the crate root's `pub use app::{…}` list) +
  `section_text_chunks` (`:386`) + `section_leaf` (`:378`) — build the carrier tree.

### 2.4 The carrier, and why it is text

The section value rides as **canonical JSON split into `Component::Text` leaves**, arranged as a
balanced `UI_BUILT_CHILDREN_MAX`(=32)-ary tree of containers whose root's key is the reserved body key.
Host side: depth-first concatenation of every text leaf, then `JSON.parse`.

Alternatives rejected, with the measured reason:

- `Component::Extension { props: UiValue }` — `UiValue`'s owned collections cap at
  `UI_VALUE_MAX_ITEMS` = 256 entries *each* and are credited against **one** live arena page
  (`UI_VALUE_LIVE_PAGES = 1`, `🎬️action.rs:22-60`) that the virtualised panel author needs; a
  measures map would either fail admission or starve that author. `UiValue::Text` alone caps at 512 B.
- `Component::Surface { doc }` — `SurfaceDoc.bytes` is `UiFixedBytes`, a hard 32 KiB ceiling
  (`UI_FIXED_BYTES`) that cannot page, and `SurfaceKind` is a closed set of 15 *product* surface kinds.
- Adding a new `Component` variant — the union is generated (`🤖️generated/📜️ui-contract.ts`) and every
  renderer (React interpreter, wgpu target, tui, conformance corpus) would have to grow a case; the blast
  radius is not justified when text leaves already satisfy every bound.

Because each leaf is its own node, the payload is paged by the existing reconcile mechanism
(`SURFACE_RECONCILE_PAGE_BYTES`) rather than truncated, and it is bounded automatically by
`max_text_bytes` (64 KiB/component), `max_children` (4096) and `max_patch_bytes` (1 MiB).

**No `packValueToExactJson` is involved** — the payload crosses as text, so `JSON.parse` reproduces the
plugin's own numbers with no integer-carrier projection needed.

Bug found and fixed while building this: `try_build()` stamps *every* unparented builder with the same
`"#0"` positional key, so a multi-leaf carrier produced duplicate sibling keys and the tree was rejected
with `duplicate-key`. Leaves now carry explicit `c{index}` ids and page containers `p{generation}-{index}`.

### 2.5 Re-publication and hiding

No new dirty-marking law was needed, and none was added: the shell re-emits `surface-visible` for exactly
the sections its `UiDirtyScope` selects (`buildUiRefreshRequest`'s `uiRefreshWantsFlag(scope, "measures")`
etc.), and `Event::SurfaceVisible` already does mount + `dirty.try_surface` (`⚛️reactor/🔄️turn/🦀️.rs:235`).
A full scope requests all three; a partial scope requests only its flagged ones. `plugin_hide_surface`
drops a section exactly like a window (`SurfaceContexts::remove`, now also releasing `section_view`) —
note that the React shell emits no `surface-hidden` event for *any* surface today (repo-wide grep: zero
producers); session switch tears the whole instance down instead.

## 3. Measured payload sizes (puzzle 3d)

From `reserved_refresh_section_payloads_admit_into_the_retained_section_carrier`
(`✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`), real `Puzzle3dApp`, one `main` window instance:

```
[DEBUG] puzzle3d engagements section payload is 523 bytes
[DEBUG] puzzle3d measures section payload is 10462 bytes
[DEBUG] puzzle3d tools section payload is 2596 bytes
```

All three fit in a single carrier level (32 × 512 B = 16 KiB) — 2, 21 and 6 text leaves respectively —
and are two orders of magnitude below `max_patch_bytes`. Nesting only starts above 16 KiB, and that path
is covered by its own framework test (§4.2).

A second measurement worth recording: with `ViewModel::default()` (no `windowInstances`), `measures` and
`engagements` serialize to `{}` while `tools` still yields 2596 bytes. That is the direct proof that the
section surface must carry the unnarrowed view.

## 4. Verification

### 4.1 Commands run (foreground, `RUSTC_WRAPPER=""`, `CARGO_TARGET_DIR=<scratchpad>/target-p3d`)

| command | result |
| --- | --- |
| `cargo check -p semio-framework-plugin` | `Finished dev profile in 11.91s`, 0 errors, 0 warnings from this wave |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile in 2.23s`, 0 errors |
| `cargo test -p semio-framework --lib ui_refresh_section` | `1 passed; 0 failed` |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib reserved_section` | `3 passed; 0 failed` |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib surface_context` | `6 passed; 0 failed` (3 pre-existing + the 3 touched/added) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib reserved_refresh_section -- --nocapture --test-threads=1` | `1 passed; 0 failed` (payload sizes in §3) |
| `SEMIO_TEST_LEVEL=long bun x vitest run --testNamePattern "reserved refresh sections"` (from `…/📦️packages/🟦️typescript/🎯️targets/⚛️react`) | `2 passed` |
| `SEMIO_TEST_LEVEL=long bun x vitest run "PluginRuntime"` | `72 passed \| 2 failed` — both failures are a peer's in-flight edits, see §5 |

`RUST_MIN_STACK` is required for any puzzle3d test that builds an app and for parts of the plugin lib
suite; the default 2 MiB test stack overflows on pre-existing tests too (`set_fill_count_dispatches_…`,
`instance_lifetime_pending_patch_keeps_scope_after_payload_surface_retires`). Not caused by this wave.

### 4.2 Tests added

- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️ui-refresh-section/{🦀️.rs,🔣️.json}` — the reserved
  identities against the language-neutral fixture, plus "no app-authored body key resolves to a section".
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  - `reserved_section_surfaces_render_accessor_maps_instead_of_app_bodies` — mounts all three sections on
    a real `VcsArtifactApp`, asserts `RENDER_CONTEXT_PROBE` is **never** taken (no section body key ever
    reaches `ArtifactApp::render`), that the root key is the reserved body key, and that the leaf
    concatenation parses back to the accessor maps (`left`/`right` window measures, `fill` tool measures).
  - `reserved_section_carrier_pages_a_payload_past_one_node_of_children` — a payload larger than
    `UI_TEXT_MAX_BYTES × UI_BUILT_CHILDREN_MAX` must nest (depth > 1), every leaf ≤ 512 B, every node
    ≤ 32 children, and the concatenation must be byte-exact.
  - `TestApp::window_measures` / `TestApp::tool_measures` added so the section payloads are non-trivial.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs`
  - `reserved_section_surfaces_keep_the_unnarrowed_view_and_outlive_their_windows` — a later panel mount
    must not steal the section's view; closing every window prunes the window surface but not the sections.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`
  - "mounts one retained surface per requested section …" — reads the same neutral fixture as the Rust
    test, asserts one event per requested section with the reserved body key and the full packed view.
  - "projects a section's chunked text carrier back into the shell's own measures map" — builds a real
    retained surface from two text chunks and asserts `retainedUiRefreshResponse().measures` is
    `{ key: "measures", hash, value: <the measures map> }`, plus the root-key-mismatch guard.
- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the puzzle3d payload sizing/round-trip test in §3.

### 4.3 NOT verified

- **No browser run.** Per the wave brief no wasm component was rebuilt and no server was started, so the
  end-to-end claim (`shellState.windowUi.toolMeasuresByToolId.fill` populated after activating Fill) is
  **not** observed — it is the coordinator's check. See §6 for exactly what to look for.
- **No repo-wide Rust suite run.** `cargo test -p semio-framework-plugin --lib surface` (the wider
  filter) aborts on the unrelated `component::app::mutation_fixture::surface::viewer_rejects_every_
  contract_mutating_verb` destructor panic, and during this wave a peer's in-flight refactor of
  `🏪️store/🦀️.rs` broke `semio-framework-os-kernel` and then
  `🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` for roughly 15 minutes. Only the filters listed
  in §4.1 were run to completion.
- The **wgpu target** is untouched. `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:350-365` still documents the
  same gap and still returns empty maps; it now has a first-class mechanism to adopt.
- `plugin_refresh_ui` (the legacy JSON `RefreshRequest` handler,
  `🔌️plugin/🦀️.rs:~28466`) is **still present**. Its only remaining caller is one Rust test
  (`surface_context_refresh_projects_panels_from_focused_window_state`); no production wire reaches it.
  Deleting it is correct under CLAUDE.md's no-legacy rule but is out of this wave's scope — flagged for a
  follow-up wave.
- The **`labels`** section is untouched: it is dead by design (manifest-resolved `LocalizedLabel` matrix),
  and `UI_REFRESH_SECTIONS` deliberately does not include it.

## 5. Concurrent-peer noise observed (not mine)

- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` — a peer is moving `AppRole`/`AppRef`/`ArtifactDialect` to
  `./🧬️schema/🟦️.ts`. Additive, no conflict with this wave.
- `🔌️PluginRuntime/🟦️.tsx` — a peer replaced the literal `lane > 11` with
  `lane > TYPED_OPERATION_RESULT_LANE_MAX` and reworked `hasRequiredUiPatches` / the continuation batching.
  That is why `validates fixed result page authority …` and `yields the browser event loop while a
  retained surface needs several continuation batches` fail; both are unrelated to sections.
- `🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` fails to import with `self is not defined` from the
  wgpu `plugin-bridge.ts` — pre-existing, untouched by this wave.
- `✏️s/…/🧊️3d/…/🔬️unit/🦀️.rs` tests that build an app overflow the default 2 MiB test stack
  (`set_fill_count_dispatches_…` does too, on a clean filter). `RUST_MIN_STACK=134217728` is required to
  run any of them. Separately, `dispatch(SET_ACTIVE_TOOL_ACTION_ID)` in that suite faults with
  `interactive-job.missing-owned-reducer` — the known bare-bounded-factory defect — which is why the new
  puzzle3d test reads `tool_measures` directly (puzzle3d's `tool_measures` does not gate on
  `active_tool_id`, so the fill measures are present regardless).

## 6. What the coordinator should verify in the browser

After rebuilding the wasm component and booting the React target on puzzle 3d:

1. The refresh cache holds `engagements`, `measures`, `tools` keys alongside `window:*` / `panel:*`.
2. Retained surface ids `<instance>:framework.section.engagements`, `…measures`, `…tools` publish patches
   on first refresh and re-publish on a scope that flags them.
3. `shellState.windowUi.windowMeasuresByWindowId["puzzle3d-main-top"]` holds the projection / vortex / lod
   / grid / select / sun / transform / brush / volume-brush measures (≈10.5 KB of JSON).
4. After activating the Fill tool, `shellState.windowUi.toolMeasuresByToolId.fill` holds the
   `puzzle3d-fill-count` slider (≈2.6 KB), and the Fill panel shows its count slider, cancel row and
   distribution tree via `buildToolTree`.
5. `shellState.windowUi.windowEngagementsByWindowId` is non-empty (≈0.5 KB).

## 7. Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️ui-refresh-section/🦀️.rs` (new)
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️ui-refresh-section/🔣️.json` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
