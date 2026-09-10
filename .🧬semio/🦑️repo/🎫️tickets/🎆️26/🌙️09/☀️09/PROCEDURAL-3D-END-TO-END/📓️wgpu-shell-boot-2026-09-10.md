# wgpu shell boot — plugin-actor isolation and `wgpu-ui.native-owner-required`

Lane: renderer engineer (wgpu shell/worker boot). Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-10.
Live finding under investigation: `📓️runtime-verification-2026-09-09.md` § "07:55 wgpu wasm boot #1" —
`http://localhost:6118/?plugin=generation3d` stalls at `shell-boot 86 %` with

```
wgpu renderer fault: worker-boot-failed: worker-boot-failed: shell-boot: create_app promise failed:
wgpu-ui.native-owner-required — No UI-thread frame fallback was attempted.
```

and, in the same boot, the **flow** plugin's actor (`flow#1`) trapping on `[handler/first-step]` with
`interactive-job.catalog-authority` (sibling lane's artifact defect — untouched here).

## 1 Root causes

### 1.1 The shell booted a foreign plugin's app (why `flow` was instantiated at all)

`ShellState::boot`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3045`
(pre-fix) opened the **first** program in the boot plan:

```rust
} else if let Some(program) = self.plugins.first() {
    let app = program.manifest.apps.iter().find(|app| Some(app.id.as_str()) == resolve_playground_app_id(&self.plugin_filter))
        .or_else(|| program.manifest.apps.first()).ok_or("plugin has no apps")?.clone();
    let instance_id = program.create_app(&app.id).await?;
```

but the frame worker hands it a **dependency-ordered** plan:

- `🎞️frame-worker/🟦️.ts:320` → `resolvePlaygroundBoot(PLUGIN_CATALOG, message.pluginVariant)`;
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2408-2419` — `expandPluginRegistry` pulls the requested plugin's
  contribution closure (`procedural` consumes `flow.extension`; every `flow-extension-*` row contributes it
  and declares `dependsOn: ["flow"]`, `🤖️generated/🧩️plugins.ts:64,85-93`), then `orderPluginRegistryEntries`
  topologically sorts it — "Boot activates in dependency order, not array order".
- `filter_plugins` (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:876`) is a pass-through, so nothing narrows the
  list again on the Rust side.

For `?plugin=generation3d` the plan therefore starts with `flow`, whose manifest has no
`s.procedural.generation3d@1/*#editor`; the `.or_else(first app)` fallback then opened **flow's own editor
app**. That single `create_app` is what ran flow's first step, hit the sibling lane's catalog-authority trap,
and — because its `Err` was `?`-propagated — failed the whole `bootShell()`. The requested `generation3d` app
was never even attempted. `boot.defaultAppId`, which `resolvePlaygroundBoot` computes correctly, was never
passed to the Rust bootstrap.

### 1.2 The failed-open cleanup masked the real cause (why the message said `wgpu-ui.native-owner-required`)

`🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` (pre-fix):

- `createApp` sets `lifecycleByInstance` **before** `lifecycle.open(...)` (line ~672) and
  `uiRouteByInstance` only **after** it returns (line ~677) — a trapped open leaves a lifecycle with no route.
- its rejection path awaited the cleanup and let the cleanup's own rejection escape:
  `if (!retiringInstances.has(instanceId)) await handle.destroyApp(instanceId); throw error;` (line ~688).
- `destroyApp` (line 710) threw `new Error("wgpu-ui.native-owner-required")` for exactly that
  lifecycle-without-route state.

So every guest-side first-step trap reached `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:615`
(`create_app promise failed: …`) wearing the teardown's error, not its own.

### 1.3 There is no UI-thread ownership requirement — the guard was misnamed

`native-owner-required` exists **only** in the wgpu TS bridge (three sites: `accept` line 286,
`requireUiRoute` line 551, `destroyApp` line 710); a repo-wide grep finds it in **no** Rust file — not in
`🖱️ui/…/🎯️targets/🧊️wgpu`, not in the renderer. The owner it names is `WgpuOwnedUiInstanceRoute`, which is
constructed **inside the frame worker** from the shard lifecycle (`new WgpuOwnedUiInstanceRoute(lifecycle)`,
`plugin-bridge.ts:676`) and needs nothing from the UI thread. The banner's trailing sentence
("No UI-thread frame fallback was attempted") is **static copy** in `🚀️browser-boot/🟦️.ts:88`, appended to
every fault — not a diagnosis. Design intent (the class docstring: "Exact WGPU host owner for one captured
guest lifetime") is that the worker owns it; the requirement is therefore *removed at the teardown site*, not
satisfied by relocating an owner.

## 2 Fixes

### 2.1 `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — boot selection + per-plugin isolation

- **`select_boot_program(programs, plugin_filter) -> Option<(usize, AppDefinition)>`** (new
  `//#region 🧯️BootProgramSelection`, above `ShellState`): resolves the program by
  `resolve_registry_plugin_id(filter)` (`generation3d` → `procedural`), falls back to whichever program
  actually declares `resolve_playground_app_id(filter)`, and only then to that program's first app. A variant
  whose plugin is absent resolves to `None` — **never** to another plugin's app.
- **`ShellPluginFault { plugin_id, app_id, detail }`** + `ShellState::plugin_faults: Vec<ShellPluginFault>`.
- **`open_boot_instance`** wraps `create_app`: `Err` → `record_plugin_fault` → `None`, so a trapped actor is
  that plugin's status, not the shell's boot failure.
- **`settle_boot`** is the shared boot tail (`sync_dock`, `sync_session_chrome`, `bootstrap_identity`,
  `refresh_ui`); every early-out path runs it, so the shell paints chrome + status instead of leaving the page
  on the loader. `boot()` now returns `Ok(())` for a plugin fault; only a genuinely broken host program
  (`host program missing`) is still fatal.
- **`plugin_fault_status()`** composes the user-visible line from `shell_chrome_string("plugin.faulted", …)` —
  new EN/DE pair `Plugin unavailable` / `Plugin nicht verfügbar`, no default language — and is re-applied on
  `setLocale` so a language switch re-renders it. It feeds the existing `ShellState::error` chrome slot
  (drawn at `ShellChromeFramePhase::Error`), so no new draw path was needed.
- `use crate::program_bridge::{… resolve_registry_plugin_id …}` added.

### 2.2 `🐚️plugin-bridge.ts` — stop masking the guest fault

- `destroyApp`: `if (!lifecycle || !route)` now joins the existing "nothing owned to retire" branch
  (`registry.cancel(actorId); releaseInstance(...)`). An instance whose `open` never produced a lifetime owns
  no retained UI; `lifecycle.dispose()` is *not* reachable there either (the lease refuses it outside
  `complete`, `📮️shard-client/🟦️.ts:1548`), so cancelling the actor is the whole correct teardown.
- New exported **`settleFailedInstanceOpen(error, cleanup)`**: runs the cleanup, logs a cleanup fault on its
  own `[DEBUG]` line, and always rejects with the **original** error. `createApp`'s rejection path uses it.

Net effect on the live boot: `?plugin=generation3d` now opens `procedural` /
`s.procedural.generation3d@1/*#editor` and never instantiates `flow` at boot at all; if any plugin's actor
does trap, the message that reaches `create_app promise failed:` is the guest's own
(`shard 0 worker fault [handler/first-step] …`) and the shell keeps booting.

## 3 Tests

| test | file |
|---|---|
| fixture table (5 cases incl. the live `flow`-before-`procedural` closure) | `🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs` + `🧑‍🎨engine/🧫️fixtures/🔬️wgpu-shell-boot-selection/🔣️.json` |
| `boot()` with the requested plugin absent settles `Ok`, opens no session, records exactly one fault (`procedural` / `s.procedural.generation3d@1/*#editor`) and mirrors it into `error` | same |
| per-plugin status in EN **and** DE, carrying the guest cause verbatim | same |
| `settleFailedInstanceOpen` rejects with the original error even when the cleanup throws `wgpu-ui.native-owner-required`, and still runs the cleanup | `🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` |
| live oracle: `resolvePlaygroundBoot(PLUGIN_CATALOG, "generation3d")` really is dependency-first (`flow` before `procedural`, `plugins[0] !== "procedural"`), and the Rust fixture's first case matches it, `expected.appId === boot.defaultAppId` | same |

The last one is the language-agnostic pairing: the same fixture case is driven by the Rust selection law and
validated against the real TypeScript kernel resolver that produces the ordering in production.

Also repaired while here: `🔬️wgpu-shell-input/🦀️.rs` asserted the retired app ids (`puzzle3d-play` …) and had
been failing against the regenerated `🖥️hosts.rs` table; it now asserts the live ids plus the
`generation3d → procedural` mapping this lane depends on.

## 4 Checks and tails

All with `CARGO_TARGET_DIR=$S/target-wgpu`, `RUSTC_WRAPPER=""`.

```
cargo check -p semio-framework-os-renderer-wgpu --keep-going
  → Finished `dev` profile [unoptimized] target(s) in 1.51s   (18 warnings, identical to the pre-edit baseline)

cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --keep-going
  → Finished `dev` profile [unoptimized] target(s) in 51.74s  (14 warnings)     [🗑️generated/wgpuboot-wasm-check.txt]

cargo test -p … --lib shell_boot_isolation
  test shell::shell_boot_isolation_tests::boot_selection_opens_the_requested_variant_across_the_fixture_table ... ok
  test shell::shell_boot_isolation_tests::boot_without_the_requested_plugin_settles_with_a_per_plugin_status ... ok
  test shell::shell_boot_isolation_tests::plugin_fault_status_reads_in_both_languages ... ok
  test result: ok. 3 passed; 0 failed
cargo test -p … --lib shell::shell_input_tests
  test result: ok. 14 passed; 0 failed                                          [🗑️generated/wgpuboot-rust-tests.txt]

bun ./📜️script.ts test-preview-generated   → Test Files 1 passed (1) · Tests 20 passed (20)
bun ./📜️script.ts test-browser-worker      → Test Files 2 passed (2) · Tests 35 passed (35)
bun ./📜️script.ts check-browser-worker     → exit 0 (browser-boot + frame-worker bundles current)
                                                                                [🗑️generated/wgpuboot-ts-tests.txt]
```

**Pre-existing failures NOT from this lane** (they fail on the untouched code paths and none of them calls
`ShellState::boot` — only the new isolation test does):
`async_boundary_tests::{native_binary_owns_exactly_one_entrypoint_driver (counts `drive_entrypoint(` in
💾️binary source: 3 vs 2), presenter_ack_retirement_source_mutations_are_denied,
raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete,
renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length (SIGABRT in
`WorldAssetFetchOwner::drop`)}`, `shell::{chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments,
command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss,
panel_anchor_model_tests::{panel_layout_round_trips_through_prefs_store,
persist_panel_layout_if_changed_is_idempotent_when_nothing_changed},
shell_document_retirement_tests::{shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation,
shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement}}`. `shell::` overall:
166 passed / 6 failed. Native tests need `RUST_MIN_STACK=536870912` (otherwise the panel-anchor module
overflows its stack before asserting anything).

## 5 Bundle

Rebuilt — the coordinator only needs to **restart `trunk serve` on 6118**; no rebuild step is owed.

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust
CARGO_TARGET_DIR=$S/target-wgpu-boot RUSTC_WRAPPER="" SEMIO_RENDERER=wgpu bun ./📜️script.ts build
  → Finished `dev` profile … in 53.50s · INFO applying new distribution · INFO ✅ success
  → trunk built wgpu renderer -> .🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu     [🗑️generated/wgpuboot-bundle.txt]
```

Proof the fixes are in the shipped artifacts (dist stamped 2026-09-10 08:26–08:27):

```
strings …/🧊️wgpu/semio-framework-os-renderer-wgpu_bg.wasm | grep -m1 'wgpu shell boot: plugin'
  →   [DEBUG] wgpu shell boot: plugin
… | grep 'Plugin unavailable' → …Plugin unavailablePlugin nicht verf…
grep -c settleFailedInstanceOpen …/🧊️wgpu/🎞️frame-worker.js → 2
```

`🎞️frame-worker.js` and `🚀️boot.js` (generated browser isolates) were regenerated via
`bun ./📜️script.ts generate-frame-worker`; `check-browser-worker` confirms both are current.

## 6 What is still in the way of geometry on 6118

Not this lane's: the flow artifact's `interactive-job.catalog-authority` trap still fires whenever a **flow**
app instance is actually opened (the sibling lane owns it). This lane only guarantees it can no longer take
the `generation3d` boot down with it — and that when it does surface, it surfaces with its own message.

## 7 Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔬️wgpu-shell-boot-selection/🔣️.json` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` (generated)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🚀️boot.js` (generated)
