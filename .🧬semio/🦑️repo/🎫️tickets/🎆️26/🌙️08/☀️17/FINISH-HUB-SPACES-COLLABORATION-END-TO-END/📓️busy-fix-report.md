# Busy-fix repair lane report (draft — being finalized, see bottom for status)

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

## Status at time of writing

Cargo checks green (native target):
- `semio-framework-plugin`: compiles, only 2 pre-existing dead-code warnings.
- `semio-s-plugin-playbook`: compiles clean.
- `semio-s-plugin-cad` + `semio-s-plugin-imperative`: compile clean (`🧪️busy-fix-cad-imperative-check.txt`).
- `cargo test -p semio-s-plugin-space --lib`: **205 passed, 0 failed** (`🧪️busy-fix-space-test.txt`).

**Blocked on an unrelated concurrent session**, not on this lane's changes: the full-catalogue wasm
rebuild (`bun 🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts plugin s`, `🧪️busy-fix-plugin-build.txt`) is
resilient (continues past a single crate's failure) but *every* plugin crate it has reached so far —
`animate`, `block`, `cad`, `cad-extension-aec-building*`, `dag`, `energy`, … — fails at the same root
cause: the shared `semio-framework-os-kernel` crate
(`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`, **not in this lane's lease**) currently fails to
compile — `error[E0432]: unresolved imports crate::os_spr::wire::PresencePoint, PresenceViewport` /
`error[E0422]: cannot find struct ... PresencePoint/PresenceViewport in this scope`, sourced from
`🧰️framework/🔨️modules/📡️spr/🦀️component.rs`. `git status` shows
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` staged-modified (`M`) right now — a live,
in-progress edit by another session on shared presence/SPR wire types (consistent with this ticket's
own "Context you must not break" note about concurrent `encodePresencePeer`/presence work). Per this
project's standing guidance ("Concurrent Cargo Workspace Churn": repo-wide failures traced to another
session's in-progress refactor can run 30–90+ minutes; poll rather than chase), this is not this lane's
bug to fix and is out of lease regardless. Waiting/retrying for it to clear, then rebuilding just the
`playbook`/`cad`/`imperative` wasm targets and completing the Playwright browser verification
(`#s-space-create-artifact` → an `[data-row-id^="artifact:"]` row, no canonical-id panic, no
`plugin instance busy` fault). **This paragraph and the Verify section below will be replaced with
real results once the shared-kernel breakage clears — do not treat this report as final while it's
still here.**
