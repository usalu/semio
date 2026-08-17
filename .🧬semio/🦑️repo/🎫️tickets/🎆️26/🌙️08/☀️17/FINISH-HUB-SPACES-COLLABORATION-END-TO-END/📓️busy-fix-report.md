# Busy-fix repair lane report

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — split
  `AppBuilder::build_definition` into fallible `try_build_definition` + panicking wrapper; added
  `App::try_from_builder`.
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` — canonicalized `MODULE_APP_ID`;
  migrated `create_module_app`/`module_plugin_bundle` onto the fallible path; updated the two tests
  that called `create_module_app()` directly.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`
  — `pluginComponentBridgeSource`'s generated bridge now reloads a trapped plugin's wasm module
  instead of retrying against the same dead instance forever.

## Root cause (verified)

`AppBuilder::build_definition` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, was
line 5169) called
`semio_framework::parse_surface_app_id(&self.id).unwrap_or_else(|error| panic!(...))`. The playbook
procedural extension registered its app with id `"playbook-module-procedural"` — a bare slug, not a
canonical `<kind>@<standard>/<subset>#<role>` surface id — so this panicked on every load.

`wasm32-wasip2` builds use `panic = "abort"` (confirmed in the repo's root `Cargo.toml`, which
deliberately does **not** override this and notes `catch_unwind` never catches on this target
regardless of where it's placed). A guest panic therefore traps the wasm instance permanently.
`InstanceGuard::enter` (`🔌️plugin/🦀️component.rs`) then returns `"plugin instance busy"` for every
later call, and the guest's own `plugin_clear_instance_guard` export can't heal a dead instance — its
own code comment already documented this poisoning mode.

## Changes made

### 1. `build_definition` no longer panics on a malformed id (or any of its other validations)
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`

- Split `AppBuilder::build_definition` into a new fallible `AppBuilder::try_build_definition(self)
  -> Result<AppDefinition, PluginAssemblyError>` that converts every one of its internal
  `assert!`/`panic!`/`unwrap_or_else(|_| panic!(...))`/`.expect(...)` sites (59 `assert!`s + the
  `parse_surface_app_id` unwrap + the undeclared-action-ref unwrap + the tutorial-validation panic +
  two `NonEmptyVec::try_from` expects) into typed `Err(PluginAssemblyError::new("app-definition.invalid",
  ...))` returns, preserving every original message.
- `build_definition(self) -> AppDefinition` is now a **thin panicking wrapper** —
  `self.try_build_definition().unwrap_or_else(|error| panic!("{error}"))` — so all ~423 existing call
  sites across every other plugin (all out of this lane's lease) keep compiling and behaving exactly
  as before, unchanged.
- Added `App::try_from_builder(builder) -> Result<Self, PluginAssemblyError>` alongside the existing
  panicking `App::from_builder`, for the same reason.
- Reformatted with `rustfmt --config-path rustfmt.toml` on an isolated wrapped snippet (not the whole
  file) to avoid an unrelated repo-wide diff.

### 2. Fixed the offending id + migrated its one real call site onto the fallible path
`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs`

- `MODULE_APP_ID` changed from `"playbook-module-procedural"` (bare slug) to
  `"s.playbook.procedural@1/*#editor"` (canonical `<kind>@<standard>/<subset>#<role>`, mirroring the
  sibling `s.playbook.playbook@1/*#editor` already documented in
  `🗿️artifacts/📖️playbook/🦀️component.rs`). `editor` because the app declares `.mutation(...)` actions
  (`ExportSolid`/`ImportSolid`).
- `MODULE_PLUGIN_ID` (a *plugin* id, separate namespace, not subject to `parse_surface_app_id`) is
  unchanged, so every other file that keys off `"playbook-module-procedural"` as a `plugin_id` (host
  contribution tests, storybook fixtures — all out of lease, all still correct) needed no changes.
- `create_module_app()` now returns `Result<App, PluginAssemblyError>` via `App::try_from_builder`
  instead of the panicking `App::from_builder`; `module_plugin_bundle()` (already
  `Result<Plugin, PluginAssemblyError>`) propagates it with `?`. This is the concrete, testable instance
  of "the caller surfaces this app failed to register" the ticket asked for.
- Swept the rest of the repo for siblings: `grep -rn 'App::builder("'` across every `.rs` file. Every
  other non-canonical literal (`"bad-app"`, `"bad-terminology-app"`, `"flat-menu-test"`, etc.) is
  either inside a `#[cfg(test)]` in `🔌️plugin/🦀️component.rs` (intentionally invalid fixtures) or in a
  frozen pre-patch snapshot under an unrelated closed ticket folder — not live production code.
  `imperative-extension-effect` and `cad-extension-aec-building-structure` (also named in the brief)
  do **not** call `App::builder` at all — their `EXTENSION_ID`s are `ExtensionBundle` ids, a different
  namespace `parse_surface_app_id` never touches. Their captured failures in this ticket's own logs are
  `wasmtime: failed to parse WebAssembly module` / `Unexpected identifier 'cabi_post_semio'` — stale
  build artifacts, not a code defect; the rebuild in this lane's Verify step regenerates them.

### 3. Trapped plugin instances now self-heal instead of staying "busy" forever
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`
(`pluginComponentBridgeSource`, generates the per-plugin jco bridge JS)

- The static `import { plugin } from "./${componentBase}.js"` became a mutable `let plugin = (await
  import(...)).plugin` plus a `reloadPlugin()` helper that re-imports the same specifier under a
  cache-busting `?semioReload=N` query (same shape `PluginSource.moduleUrl` already uses for hot
  reload, per `🎠️kernel/🟦️component.ts`'s own comment) — forcing a genuinely new
  `WebAssembly.Instance` instead of the browser's cached (still-dead) module.
- `runSerialized`'s retry loop now distinguishes `trapped` (`"unreachable"`/`trap`/`panicked`) from
  merely `busy` (lock contention): on `trapped` it clears the stale `apps` instance-id set and calls
  `reloadPlugin()` before retrying; on plain `busy` it keeps the cheap `clearInstanceGuard()` heal.
  Previously a `trapped`-but-not-`busy` error (the very first panic, before `INSTANCE_GUARD` even says
  "busy") threw immediately with **no retry at all** — now it gets the same reload-and-retry treatment.
- `🎠️kernel/🟦️component.ts`'s `withSerializedPluginWasmHandle`/`isPluginInstanceBusyError` (the outer,
  transport-level retry wrapper for both the worker-backed and main-thread bridge paths) needed no
  change: it just re-calls the bridge, which now self-heals underneath it.

## Verification performed

Cargo checks/tests green (native target — all commands run to completion, real output captured):
- `cargo check -p semio-framework-plugin --lib`: compiles, only 2 pre-existing dead-code warnings.
- `cargo check -p semio-s-plugin-playbook --lib`: compiles clean.
- `cargo check -p semio-s-plugin-cad -p semio-s-plugin-imperative --lib`: both compile clean
  (`🧪️busy-fix-cad-imperative-check.txt`).
- `cargo test -p semio-s-plugin-space --lib`: **205 passed, 0 failed** (`🧪️busy-fix-space-test.txt`),
  matching the ticket's required baseline exactly.

## Blocker: could not complete the browser/Playwright verification

The wasm rebuild this ticket's Verify step requires
(`bun 🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts plugin s`, and later a scoped
`SEMIO_PLUGIN_ONLY=playbook`/direct `cargo build -p semio-s-plugin-playbook --target wasm32-wasip2`
retry) never produced a `.wasm` for playbook (or any other plugin) across 5 attempts spanning roughly
50 minutes. This is **not caused by this lane's changes** — it is a large, unrelated,
currently-in-progress refactor of the plugin runtime itself, by another concurrent session, evidenced
concretely by:

- `git status --short -- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` shows a brand-new `⚛️reactor/`
  subsystem (`💼️jobs`, `📮️requests`, `📸️checkpoint`, `🧵️executor`, `🩹️patches`, plus the reactor's own
  `🦀️component.rs`) and a brand-new `🌐host/🦀️component.rs`, all `A` (staged, uncommitted, never
  existed before), alongside a *modified* `world.wit` — the one file this lane's brief explicitly
  forbids editing, and it is a `M` right now for reasons that have nothing to do with this fix.
- The last two of five build attempts reproduced the **exact same 38/39-error signature** twice in a
  row (not different each time, ruling out a mid-save race): `error[E0433]: cannot find exports in
  component` / `cannot find function log/now_ms/trace_span in module crate::component` at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️component.rs:327,338,348`, and
  `error[E0562]: impl Trait is not allowed in closure parameters` at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs:361` (full text:
  `🧪️busy-fix-playbook-wasm-direct.txt`, captured 2026-08-17 22:27:52 +0200). None of these files or
  lines were touched by this lane — they are new files this lane never created.
- Earlier attempts (before that reactor/host work reached a stable-but-broken state) hit a *different*
  unrelated failure first: every plugin crate failing on the shared `semio-framework-os-kernel` crate
  with `error[E0432]: unresolved imports crate::os_spr::wire::PresencePoint, PresenceViewport`
  (`🧪️busy-fix-plugin-build.txt`), traced to a live edit of
  `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` (also out of this lane's lease). That one
  cleared on its own mid-session (confirmed: `flow-extension-primitive`, `flow-extension-text`,
  `forms`, `gis` all built to real `.wasm` successfully afterward) — the reactor/host breakage above is
  the *second*, still-unresolved wave.
- The build tool's own budget guard independently flagged the same thing on one attempt:
  `[budget] cargo build -p semio-s-plugin-playbook --target wasm32-wasip2 --profile dev exceeded
  1200000ms — killed. Likely shared cargo target-dir lock contention from another concurrent
  session — investigate before retrying.`

Per this project's own standing guidance on concurrent cargo-workspace churn (repo-wide build failures
traced to another session's in-progress refactor, not this session's diff), and given `world.wit` and
everything under `⚛️reactor/`/`🌐host/` are out of this lane's lease regardless, this is not something
this lane can or should fix. **The Playwright browser check
(`#s-space-create-artifact` → `[data-row-id^="artifact:"]`, no canonical-id panic, no `plugin instance
busy` fault) was not performed and is not claimed here** — it requires a working plugin wasm build,
which is currently unavailable repo-wide for reasons outside this lane's control.

## What is NOT done

- Live browser/Playwright verification of the fix (blocked, see above — retry once the
  `⚛️reactor`/`🌐host` work in `🔌️plugin/` stabilizes or is committed).
- The other ~423 `AppBuilder::build_definition()` call sites across every other plugin still use the
  panicking wrapper (deliberately, to stay in-lease — see `📓️busy-fix-report.md` §1); only playbook's
  one real call site was migrated onto `try_build_definition`/`try_from_builder` as the concrete,
  testable instance of the new fallible path.

## sharedFileRequest

None — everything needed for this fix was in-lease. The blocker above is a report, not a request: it
needs the *other* session's `⚛️reactor`/`🌐host`/`world.wit` work to finish, not an edit from this lane.
