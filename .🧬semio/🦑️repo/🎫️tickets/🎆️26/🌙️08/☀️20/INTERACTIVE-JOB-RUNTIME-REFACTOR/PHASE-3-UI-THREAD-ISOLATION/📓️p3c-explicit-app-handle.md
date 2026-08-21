# Packet P3c — Explicit App Handle and Native Send Gate

Scope: the existing uncommitted Phase-3c changes in renderer-wgpu `📦️glue.rs` and
`🦀️winit_app.rs`. Phase-5 Shell thread-local extraction, UI-WGPU prepared-render seams, and
frame-transaction migration remain deliberately untouched.

## Result

- `AppRuntime::self_weak` is removed. Deferred continuations receive `AppHandle =
  std::rc::Weak<RefCell<AppRuntime>>` explicitly from `OsHost`, the owner of the real
  `Rc<RefCell<AppRuntime>>`.
- Every `frame`, hot-reload, sync-pump, camera/action dispatch, asset-poll, and keyboard continuation
  that previously cloned `self.self_weak` now clones the explicit parameter.
- Dead `PointerCallbacks` construction is removed from `boot_runtime`; its return type and its one
  `WinitApp` caller now agree on `Result<Rc<RefCell<AppRuntime>>, String>`.
- Right-click is deliberately wired through the enqueue-only contract: native right mouse input
  normalizes to lossless `DispatchEvent::PointerDown { button: PointerButton::Secondary }`, drains via
  `dispatch_normalized_event`, maps to DOM button `2`, and calls the canonical
  `AppRuntime::handle_pointer_button`. `ShellState::handle_pointer_button` opens the context menu in its
  first pressed-secondary branch. The redundant callbacks-only `AppRuntime::handle_context_menu`
  wrapper is deleted.
- A native-only `assert_send::<AppRuntime>()` remains mounted beside the struct. Its mounted compiler
  verdict is pending the transitive concurrent compiler blockers below; no reasoning-only success is
  claimed.

## Audit evidence

`rg` over the renderer target found no stored `self_weak`, no stale tuple match for `boot_runtime`, and
no live `handle_context_menu`. Remaining `self_weak` text is historical documentation explaining the
removed field. The only `Weak<RefCell<AppRuntime>>` values are the explicit `AppHandle` alias and the
asset-poll RAII reset guard, which receives a cloned explicit handle rather than being stored in
`AppRuntime`.

The right-click route is covered by `p3c_tests::secondary_pointer_button_uses_context_menu_code`, which
asserts the renderer's `PointerButton::Secondary -> 2` boundary. The host crate already tests
`winit::MouseButton::Right -> PointerButton::Secondary`; the enqueue queue treats pointer-down as a
lossless discrete event.

## Commands and results

### Mounted renderer test/compile

```text
bun nx run @semio-tech/framework-renderer-wgpu:test-quick
```

Exit `1`. Cargo advanced through `semio-framework-ui-host`, `semio-framework-job`, and other prior
dependencies, but did not compile `semio-framework-os-renderer-wgpu`. Concurrent out-of-scope sources
failed first:

- `semio-framework-plugin`: 5 errors, including malformed concurrent edits
  `pub async async fn from_function_pointer` at `plugin/.../component.rs:2334` and
  `dsl_val.awaitue_to_ui_value` at line 6070.
- `semio-framework-os-infinite`: 820 errors, led by `.await` in non-async functions and calls now
  returning futures where Vello shapes/transforms are required.

Therefore this run is dependency-gate evidence, not a verdict for `assert_send::<AppRuntime>()`.

### Target lint

```text
bun nx run @semio-tech/framework-renderer-wgpu:lint
```

Exit `0`: `framework-renderer-wgpu: color-literal lint passed`.

### Dependency ratchet

```text
bun ./📜️script.ts verify dependencies
```

Exit `0`: baseline `238`, current `238`, no new third-party dependencies.

### Interactivity audit

```text
bun ./📜️script.ts verify interactivity
```

Exit `0` in WARN mode: `180` findings (`124` blocking bridge, `6` sync clipboard, `36` sync fs, `6`
sync process, `8` thread pool) plus one stale allowlist entry. Counts match the Phase-3 baseline; this
packet adds no bridge, synchronous I/O, process, or thread-pool finding.

### Formatting parse check

```text
rustfmt --check --edition 2021 --config-path ./rustfmt.toml <glue.rs> <winit_app.rs>
```

Exit `1` because formatting `📦️glue.rs` recursively traverses many `#[path]`-mounted renderer element
files and reports extensive pre-existing formatting differences. The only P3c-attributable blank-line
diff surfaced by the run was fixed. No bulk formatter was run because those mounted files are concurrent
work outside this packet.

## Remaining verification

Once the concurrent `semio-framework-plugin` and `semio-framework-os-infinite` rewrites compile, rerun:

1. `bun nx run @semio-tech/framework-renderer-wgpu:test-quick` for the mounted native assertion and
   unit test.
2. `bun nx run @semio-tech/framework-renderer-wgpu:wasm` for the mounted browser build.
3. The existing Phase-3 standalone snapshot and frame-job verifier crates in debug/release.
