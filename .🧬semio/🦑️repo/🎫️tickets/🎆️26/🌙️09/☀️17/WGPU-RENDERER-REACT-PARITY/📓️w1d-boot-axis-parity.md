# 🧭️ W1d — boot-axis parity across the three wgpu doors and React

Packet W1d of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`; closes audit packets 4, 5 and 10 of
`📓️audit-runtime-boot-input.md` (gap-table rows 3, 4, 34).

## What the packet found

React has **one** boot door, `FrameworkOsBootOptions` (`🧱️elements/🐚️Shell/🟦️.tsx`, region `Boot`),
fed by `🧑‍💻dev/🟦️.ts` from `?plugin=&role=&example=` (`🧑‍💻dev/🔗️boot-query/🟦️.ts`) plus
`VITE_SEMIO_*` env. wgpu has **three** — the trunk page, the embeddable library call and the native
CLI — and each carried a different subset of the same axes:

| axis | React | wgpu browser-boot (before) | wgpu renderer-boot (before) | wgpu native (before) |
|---|---|---|---|---|
| plugin | `plugin` | `?plugin=` / `<meta semio-plugin>` | `plugin` | `--plugin` |
| plugins | `plugins` | (planner) | `plugins` | (modules root) |
| rootId | `rootId` | fixed `#root` | `rootId` | n/a |
| appId | `appId` | **absent** | **absent** | **absent** (runner passed `--app`, binary ignored it) |
| appRole | `appRole` | `?role=` | **absent** | `SEMIO_APP_ROLE` env only |
| appMode | — (wgpu-only axis) | `?mode=` | **absent** | **absent** |
| appExample | `defaults.exampleId` | `?example=` | **absent** | **absent** |
| locks | `locks` | **absent** | **absent** | `SEMIO_LOCKED_*` env (native-only by construction) |
| defaults | `defaults` | **absent** | **absent** | **absent** |
| brand | `brand` | **absent** | **absent** | **absent** |
| hub/user/dataDir | (not a boot option) | `?hub=&user=&dataDir=` | **absent** | **absent** |
| `#semio-broker=<64hex>` | `🏛️ShellHost/🟦️.tsx:209` | **absent** | **absent** | n/a (inherited fd) |

A second finding, recorded because the audit asserted the opposite: **React does not update the URL
when the plugin or the example is switched.** `useUIHistory(initialUri, syncBrowser)` is mounted once
(`🏛️ShellHost/🟦️.tsx:3487`) as `useUIHistory("/", hostMode && scope.ownsPage)` and pushes
`pathname + search` for the HOST shell's space/document navigation only; there is no plugin/example
writeback and no hash writeback anywhere. The only `#` route in the whole repo is
`#semio-broker=<64 hex>` — a one-shot local-hub broker proof the hub CLI puts in the URL
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:11229,12027`), which ShellHost reads at module scope and then
`history.replaceState`s away. That — read once, strip immediately — is what "hash routing" means here
and is what browser-boot now mirrors; no per-switch URL writeback was added, because adding one to
wgpu alone would have been a divergence, not parity.

## What was wired

### 1. One descriptor shape, two twins

New `🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts` — the shared vocabulary and the one resolver:

- `WgpuBootDescriptor` = `pluginVariant, appId, appRole, appMode, appExample, brandId, brokerProof,
  locks{exampleId,locale,terminology,themeId,appearance}, defaults{exampleId}, hub{hubUrl,user,dataDir}|null`.
- `resolveWgpuBootDescriptor({search, hash, meta, defaultVariant, overrides})` — one precedence for
  all doors: **explicit override > `?query=` > `<meta name="semio-*">` > built-in fallback**, with
  React's own `"viewer"`-or-else rule for `role` and React's own collapse of `?example=` over the
  per-server default seed into ONE `defaults.exampleId`.
- Caps restated once: `WGPU_BOOT_FIELD_CAPACITY = 2048` per field (the trunk page's old private
  constant) and `WGPU_BOOT_LOCATION_CAPACITY = 8192` for the raw search/hash — the same 8192 as
  React's `BOOT_QUERY_CAPACITY`.
- `readBootBrokerProof` / `stripBootBrokerProof` — the `#semio-broker=` reader and the
  `replaceState` that removes it, byte-for-byte React's.

Its Rust twin is `WgpuBootDescriptor` in `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (region
`🧭️BootDescriptor`, replacing the old `🔖️RoleBoot`/`🎭️ModeBoot`/`📚️ExampleBoot` trio), with
`#[serde(rename_all = "camelCase", default)]`, `WgpuBootDescriptor::validated()` (the Rust half of
`boundedBootField`, refusing by field name), `apply_boot_descriptor()` and accessors
`boot_descriptor / boot_app_id / boot_app_role / boot_app_mode / boot_app_example / boot_brand_id /
boot_broker_proof / boot_locks`.

The four per-axis wasm hooks (`semioWgpuSetAppRole`, `semioWgpuSetBootMode`,
`semioWgpuSetBootExample`) collapsed into ONE — `semioWgpuSetBootDescriptor(json)` — because three
independent setters are exactly how the three doors came to disagree. No legacy shim was kept.

### 2. Trunk page (`🚀️browser-boot/🟦️.ts`)

`bootDescriptor()` is now three lines over the shared resolver. It reads
`?plugin=&app=&role=&mode=&example=&hub=&user=&dataDir=` plus the `<meta name="semio-*">` seeds
(`semio-app-id`, `semio-app-role`, `semio-brand`, `semio-default-example`, `semio-locked-*`), reads
the `#semio-broker=` proof and strips it. `🌐️server/🟦️.ts`'s `wgpu-browser-selection` plugin grew
`bootAxisMetaTags()`, which injects those metas from `SEMIO_*` process env — the wgpu twin of React's
serve baking `VITE_SEMIO_*`. An unset variable injects no tag, so the page never carries an empty pin.

`🚚️browser-frame-transport/🟦️.ts`'s `BrowserFrameWorkerBoot` now carries `descriptor` in place of the
four loose fields, and `🎞️frame-worker/🟦️.ts` forwards it as one `semioWgpuSetBootDescriptor` call.

### 3. Embeddable door (`🎬️renderer-boot/🟦️.ts`)

`FrameworkOsWgpuBootOptions` gained `appId, appRole, appMode, appExample, locks, defaults, brand,
hub` beside the existing `rootId, plugin, plugins, rendererModuleUrl`. It resolves the same
descriptor (overrides only — like React's own library door it reads no `?query=`, but it does read
the page's `#semio-broker=` hash the way ShellHost does), applies it through
`semioWgpuSetBootDescriptor` **before** `semioWgpuMount`, and mounts `descriptor.pluginVariant`.

### 4. Native CLI (`⌨️native-entrypoint/🦀️.rs`)

New flags `--app --role --mode --example --brand --hub --user --data-dir` beside `--plugin`, folded
over the process-env seeds and applied through `apply_boot_descriptor` (a rejected descriptor exits
1 with the field name). `resolve_environment_boot_descriptor` seeds `SEMIO_APP_ID / SEMIO_APP_ROLE /
SEMIO_BRAND / SEMIO_DEFAULT_EXAMPLE / SEMIO_LOCKED_*` — the same names React's serve projects into
`VITE_SEMIO_*`, so locks and brand stay per-server axes on both renderers rather than becoming
per-invocation flags.

`⌨️native-entrypoint/📜️script.ts`'s `RunScript` now forwards `--flag value` pairs the dev spells and
adds the playground row's own `--brand` beside the `--app` it already passed (which the binary had
been silently dropping). Explicit flags are spelled first because `arg_value` reads the first
occurrence.

### 5. The shell honours them (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`)

- `select_boot_program(..., pinned_app_id)` — `appId` pins the anchor app; a pin no manifest declares
  falls back to the variant's own app, the same "a URL is not a place to hard-fail a shell" rule the
  other axes follow. Wired from `crate::boot_app_id()`.
- `env_lock()` now resolves through `crate::boot_locks()` instead of `std::env::var`. Native
  behaviour is unchanged (the descriptor is seeded from the same env), and the **browser build,
  which has no process env and was therefore unlockable, now locks** from `<meta name="semio-locked-*">`
  or the library door's `locks` option. This covers appearance/locale/terminology/theme at all five
  existing call sites (preference load, five persist gates, `resolve_shell_locks`).
- The example picker is removed when `locks.exampleId` is set — React's own
  `exampleOptions.length > 0 && !locks.exampleId` render gate, not a refusing control.
- `apply_boot_example` / `apply_boot_mode` / `resolve_boot_example_id` were already correct and are
  unchanged: `?example=` still reaches the guest through the same `setActiveExample` `ActionDescriptor`
  React dispatches, after `push_contributions` and before the first `refresh_ui`.

### 6. The standing law

`🧪️tests/🧭️boot-axis-parity/🦀️.rs`, declared as
`#[cfg(test)] #[path = "../../../🧪️tests/🧭️boot-axis-parity/🦀️.rs"] mod boot_axis_parity_tests;` in
`🧊️renderer/🦀️.rs` (region `🧭️BootAxisParityTests`) — the same `#[path]`-module idiom the sibling
Rust laws in that folder use; it runs under `cargo test -p semio-framework-os-renderer-wgpu --lib`.

Three laws:

1. `every_boot_door_names_the_same_axes` — parses the four field sets out of their own sources
   (`FrameworkOsBootOptions` from the React `.tsx`, `WgpuBootDescriptor` from the `.ts`,
   `FrameworkOsWgpuBootOptions` from the `.ts`, every `arg_value("--…")` from the native `.rs`),
   canonicalises the spellings, and asserts each door carries the union minus a **commented
   allowlist**. The allowlist is checked in both directions: a stale entry (an axis a door now does
   carry) fails too.
2. `the_descriptor_rust_twin_matches_the_typescript_shape` — `WgpuBootDescriptor` and its three
   nested types must have identical fields in `🧭️boot-descriptor/🟦️.ts` and `🧊️renderer/🦀️.rs`, since
   a field added to one alone would deserialize as a silent default.
3. `every_door_bounds_a_field_at_the_same_capacity` — 2048/8192 in the TS, 2048 in the Rust, 8192 in
   React's `BOOT_QUERY_CAPACITY`.

Allowlisted intentional exceptions:

| door | absent axes | reason |
|---|---|---|
| descriptor | `rootId`, `plugins`, `rendererModuleUrl`, `surfaceSessionFactories` | mount/module addressing, not session axes; a JS closure cannot cross into wasm |
| React | `appMode`, `appExample`, `hub`, `hubUser`, `hubDataDir`, `brokerProof`, `rendererModuleUrl` | `?mode=` has no React reader yet; hub reaches React via the backbone worker; `appExample` is covered by `defaults`; the broker proof is read from `location.hash`, never passed in; no wasm module to point at |
| wgpu library | `surfaceSessionFactories`, `brokerProof` | React-only closure; proof read from the page's own hash |
| native | `rootId`, `plugins`, `rendererModuleUrl`, `surfaceSessionFactories`, `brokerProof`, `locks`, `defaults` | browser-mount concepts; native claims its credential through the inherited fd; locks/defaults are deliberately per-server env on both renderers |

## Option matrix after this packet

| axis | React | browser-boot | renderer-boot | native |
|---|---|---|---|---|
| plugin | `plugin` (env `VITE_SEMIO_PLUGIN`) | `?plugin=` / `<meta semio-plugin>` | `plugin` | `--plugin` |
| plugins | `plugins` | boot planner | `plugins` | `SEMIO_PLUGIN_MODULES` |
| rootId | `rootId` | fixed `#root` | `rootId` | n/a |
| appId | `appId` | `?app=` / `<meta semio-app-id>` | `appId` | `--app` / `SEMIO_APP_ID` |
| appRole | `appRole` / `?role=` | `?role=` / `<meta semio-app-role>` | `appRole` | `--role` / `SEMIO_APP_ROLE` |
| appMode | — (allowlisted) | `?mode=` | `appMode` | `--mode` |
| appExample | `defaults.exampleId` ← `?example=` | `?example=` / `<meta semio-default-example>` | `appExample` | `--example` / `SEMIO_DEFAULT_EXAMPLE` |
| locks (×5) | `locks` ← `VITE_SEMIO_LOCKED_*` | `<meta semio-locked-*>` | `locks` | `SEMIO_LOCKED_*` |
| defaults | `defaults` | `<meta semio-default-example>` | `defaults` | `SEMIO_DEFAULT_EXAMPLE` |
| brand | `brand` ← `VITE_SEMIO_BRAND` | `<meta semio-brand>` | `brand` | `--brand` / `SEMIO_BRAND` |
| hub/user/dataDir | — (allowlisted) | `?hub=&user=&dataDir=` | `hub` | `--hub --user --data-dir` |
| brokerProof | `location.hash`, stripped | `location.hash`, stripped | page `location.hash` | n/a |

`?plugin=cad&example=…` on 6120 and `--plugin cad --example …` natively now resolve to the same
descriptor — audit packet 4's acceptance criterion.

## launch.json

**Unchanged, deliberately.** No existing flag was renamed or removed; `--app`, `--role`, `--mode`,
`--example`, `--brand`, `--hub`, `--user`, `--data-dir` are all additive and optional, and the
existing wgpu entries (`🛠️dev…🧊️wgpu🖥️native` in `.vscode/launch.json`, the `*-wgpu` entries in
`.claude/launch.json`) pass no boot axis beyond the variant. The per-variant defaults a dev would
otherwise have to spell are already supplied by the playground row through
`⌨️native-entrypoint/📜️script.ts`.

## Verification

| check | result |
|---|---|
| `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker` (NX_DAEMON=false) | **green** after `generate-browser-boot` + `generate-frame-worker`. This is the real gate for the TS side: it re-bundles `🚀️browser-boot` and `🎞️frame-worker` through `📽️projection/🟦️.ts`'s schema-owned-import resolver, which *refused* the new module until it was registered in the taxonomy browser profile (see below), and then compares the bytes against the committed artifacts. |
| `tsc --noEmit --strict` on `🧭️boot-descriptor/🟦️.ts` | **green** (standalone, no dependencies to resolve). |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --bins --features native-bin --keep-going` | **NOT COMPLETED — five attempts, every one SIGKILLed (exit 137) by the OOM killer**, each at a different dependency crate (`semio-s-artifact-stdio-*`, `semio-framework-ui`, `semio-framework`), with **0 errors emitted before the kill each time**. Cause is environmental, not this packet: 9–23 concurrent `cargo check -p semio-framework-os-renderer-wgpu` processes from the parallel W1 fleet on one 34 GB machine, with swap pinned at 4.3/5.1 GB used and <5 000 free pages. `-j 1`, `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_DEV_DEBUG=false` did not save it. Log: `🗑️generated/w1d-native-check.txt`. |
| `cargo check … --target wasm32-unknown-unknown` | **NOT RUN** — blocked behind the same saturation. |
| the new law test | **NOT RUN** — same. |
| `rustfmt --edition 2021 --emit stdout` on all four edited `🦀️.rs` files | **parses clean** (syntax only — this is NOT a type check and is reported as such). |
| the law's four parsers + allowlist, re-implemented in Python against the real sources | **all four doors: 0 missing, 0 stale**; `WgpuBootDescriptor` and its three nested types match field-for-field between `🧭️boot-descriptor/🟦️.ts` and `🧊️renderer/🦀️.rs`; React's `BOOT_QUERY_CAPACITY = 8192` present. So the law's *logic* and its input parsing are verified; only its *compilation* is not. |

**What the coordinator must re-run once the fleet quiets** (in this order, alone):

```
cargo check -p semio-framework-os-renderer-wgpu --lib --bins --features native-bin --keep-going
cargo test  -p semio-framework-os-renderer-wgpu --lib boot_axis_parity
cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going
```

Note `--bins` matters: the packet's native-entrypoint changes live in the `semio-wgpu-native` binary,
which `--lib --features native-bin` alone never compiles (the feature only satisfies the bin's
`required-features`).

### One extra registration this packet needed

A new module under `🎯️targets/🧊️wgpu/` is not free: the browser bundles are built by a
schema-owned-input resolver whose authority is
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` →
`generatorContracts.wgpu-frame-worker.packageGeneration.browserProfile.sourceModulePaths` (an explicit,
byte-ordered list; an unlisted import throws `WGPU browser import is not schema-owned`, and a listed
but unread one throws `includes unread inputs`). The new path was inserted there and in the matching
`frameWorkerSources` named input of the wgpu TypeScript `📋️project.json`, both at the byte-ordered
position between `🚪️host-io` and `🫀️boot-liveness`.

## Remaining gaps (honest)

1. **`brand` is an id, not a `ShellBrand`.** The axis is now accepted and threaded to
   `crate::boot_brand_id()`, but the wgpu shell has no brand registry: React's `resolveShellBrandById`
   supplies `windowTitle`, `ephemeral` (which clears durable storage), `replayIntroductionOnLoad`, and
   brand-level `locks`/`defaults` that `mergeShellLockSources`/`resolveShellDefaults` overlay. None of
   that exists on wgpu. Wiring it is its own packet (it needs the brand table ported to Rust).
2. **The hub axis is still read by nobody on wasm.** `BOOT_HUB_ENV` in the wgpu Shell
   (`🦀️.rs:~378`) is written by `semioWgpuSetHubEnv` and has **no reader at all** — the only
   `resolve_identity_env()` is `#[cfg(not(target_arch = "wasm32"))]`, and the browser's directory
   base URL is the fixed same-origin proxy `/_semio/hub`. Native ignores `--hub`/`--user` for the
   same reason (`IdentityEnv::from_process_env` keys off the inherited `S_LOCAL_CREDENTIAL_FD`, not a
   URL). The axis is now uniform across the doors; making it *do* something is hub/identity territory.
3. **`brokerProof` has no wgpu consumer.** It is read and stripped exactly as React does, but React
   hands it to `BrowserBrokerPortClientV1` over a `MessageChannel` to its backbone worker — a seam
   wgpu has no counterpart for.
4. **`surfaceSessionFactories`** cannot exist on wgpu at all (a JS closure cannot cross into the
   renderer wasm); the wgpu equivalent is the plugin bridge. Allowlisted, not deferred.
5. **`?mode=` has no React reader.** The mode axis is currently wgpu-only. Symmetry would mean adding
   `resolveBootQueryModeId` to `🧑‍💻dev/🔗️boot-query/🟦️.ts` and a `mode` field to
   `FrameworkOsBootOptions`; that is a React-side change and was left out of a wgpu packet, with the
   asymmetry allowlisted and named.
6. **Locks still only cover five preferences**, matching React (`driver`/`layout`/`customThemes`
   deliberately stay unlocked on both sides).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts` (new)
- `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts`
- `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `…/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`
- `…/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts`
- `…/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` (new module in `frameWorkerSources`)
- `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `…/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs` (new `select_boot_program` argument)
- `…/🧪️tests/🧭️boot-axis-parity/🦀️.rs` (new)
