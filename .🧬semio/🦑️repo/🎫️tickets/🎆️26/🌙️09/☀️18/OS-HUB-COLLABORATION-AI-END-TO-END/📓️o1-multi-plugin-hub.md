# O1 — one `dev s` hub hosting every registered plugin

Slice O1, worker-owned. Scope: plan items 1–6 of `📓️audit-multi-plugin-hub.md` §(c)/§(d). The ticket, the
wgpu chrome (peer: `WGPU-RENDERER-REACT-PARITY`), `PluginRuntime/🟦️component.tsx` and `🏪️store/🔄️sync/🦀️.rs`
(worker C1) were not touched. The `dev s` activation chain was NOT run here — C1 owns that cold build.

## 0. Decisions taken

1. **`dev multi` deleted, not aliased.** Host-mode `dev s` already IS "boot every plugin" at all four
   layers the audit names, so a second spelling would be a second mechanism for one behaviour. The `why`
   moved into `runFrameworkOsPlaygroundDev`'s own doc comment instead of a dead branch.
2. **The two artifact-kind namespaces are both carried by the catalog, and the owner is claimed at
   generation time.** A crate declares `ActivationEvent::OnArtifactKind { kind }` with its
   `ArtifactKindSpec.id` (`3d.cad`), while an opening coordinate parses to the dialect's artifact kind
   (`s.cad.cad`). Measured across the real catalog: of 30 crates declaring `on-artifact-kind`, exactly
   **one** (`gis`) declared a kind that matched any of its own app surfaces' dialects. Keying the runtime
   resolution on the declared kinds alone would therefore have resolved 1 plugin out of 30 — declared
   metadata still decorative, just one layer deeper. Both spellings are now emitted as
   `on-artifact-kind:` rows from the **same descriptor** the generator already reads, and a catalog-wide
   ownership pass leaves each kind on exactly one row.
3. **Ownership is the dependency edge, not a naming convention.** `claimOwnedArtifactKinds` strips from
   each row every `on-artifact-kind:` it shares with a transitive `dependsOn` crate. That is the
   build-time twin of `AppRouter.build`'s "dependency-first load order, first claim wins": a contributor
   registering a surface on someone else's kind must declare that owner as a dependency
   (`surface.contribution-not-permitted`), so the dependency edge is exactly what separates owner from
   contributor. Verified on the real catalog: `demonstrator` ends with only `s.demonstrator.playground`,
   `cad` keeps `3d.cad` + `s.cad.cad`.
4. **The Rust `Manual` constants at `🧊️renderer/🦀️.rs:7018,7127,8354` were left alone — the audit
   conflated two types.** `semio_framework_actor::ActivationEvent` (`🎭️actor/🦀️.rs:4705`) is a
   three-variant SCHEDULER trigger (`Manual | WindowOpen | Restart`); the plugin-declared
   `semio_framework::kernel::ActivationEvent` (`🎠️kernel/🦀️.rs:1560`) is the one with `OnArtifactKind`.
   Those call sites pass the scheduler enum, which has no artifact-kind variant to move to. See §6.
5. **Progress + cancellation for the wgpu install is state, not chrome.** The retained
   `ShellState::plugin_install` record (phase + `CancelToken`) mirrors the shell's own `inference_port`
   pattern. Painting it is left to the wgpu chrome owner — that file is under active peer edit and the
   seam I was allowed to touch is `switch_to_app`/the open-artifact relay.

## 1. `dev multi` removed (plan item 1)

- `📜️script.ts:416-426` (old numbering) — the whole `segments[0] === "multi"` branch and its comment are
  gone. The branch shelled into `@semio-tech/framework-os-dev:dev -- multi`, whose target is hardcoded to
  `serve s dev` with `forwardAllArgs: true`, so `"multi"` arrived as a 4th positional and the wgpu serve
  script threw `Unknown browser server option: multi` before Vite started.
- `📜️script.ts:255-271` — `runFrameworkOsPlaygroundDev`'s doc now states why there is no multi variant.
- No dangling references: `grep -n multi` over `🧑‍💻dev/**` (`*.ts`/`*.tsx`/`*.json`) returns only unrelated
  matches (a forms fixture's `kind=multi` question type). The os-dev Nx `dev` target
  (`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:127-140`) never had a `multi` branch to remove.
- `.claude/launch.json` and `.vscode/launch.json` contain no `multi` entry (grep: 0 hits), so neither
  needed editing for this item.
- `bun build --target=bun 📜️script.ts` → `Bundled 860 modules in 384ms`, exit 0.

## 2. Declared `on-artifact-kind` wired into a real resolution path (plan item 4)

### Registry (build time)

- `📇️registry/🔎️discovery/🟦️.ts:258` `ON_ARTIFACT_KIND_PREFIX`.
- `…:267` `descriptorSurfaceArtifactKinds(descriptor)` — the distinct `manifest.apps[].dialect.artifactKind`
  values a descriptor declares.
- `…:385-388` (inside `parsePluginCargo`) — appends each of those as `on-artifact-kind:<kind>` beside the
  crate's own declared events.
- `…:288` `claimOwnedArtifactKinds(entries)` — the transitive-`dependsOn` de-claim pass; applied at
  `…:443` on `generatePluginRegistry`'s return.
- `📇️registry/📽️projection/🟦️.ts:214-225` — the kind → owner map, emitted into the generated Rust host
  module as `PLUGIN_ARTIFACT_KIND_ACTIVATIONS` (`…:274`) plus
  `resolve_artifact_kind_activation_owner` (`…:281`).
- Regenerated: `bun ./📜️script.ts generate` →
  `plugin registry catalog refreshed (60 plugin crates, 65 playgrounds, 54 framework packages)`.
  `🤖️generated/` is gitignored, so this is a local regeneration the activation chain repeats.
  Spot check after regeneration:
  `cad → ["on-artifact-kind:3d.cad","on-artifact-kind:s.cad.cad"]`,
  `demonstrator → ["on-artifact-kind:s.demonstrator.playground"]`,
  `space → [space.shome, space.sspace, s.space.home, s.space.space, s.space.studio]`.

### Kernel (TS)

- `🎠️kernel/🟦️.ts:1217` — `PluginCatalogTarget.activationEvents`.
- `…:1246` `ON_ARTIFACT_KIND_ACTIVATION_PREFIX`, `…:1259` `artifactKindActivationOwner(catalog, kind)`
  (ties resolve to the first `pluginId` ascending, matching the router's own ordering),
  `…:1273` `activationReasonForAppId(appId)`.
- `🔌️plugin/📇️registry/🟦️.ts:15-22` — `activationEvents` passed through into the injected `PluginCatalog`.

### React `ShellHost`

- `🏛️ShellHost/🟦️.tsx:2523` — `resolveArtifactOpeningWithActivationRef`.
- `…:7781-7807` — `installActivationOwnerAndResolve`: sync relay first; on a throw it parses the
  `artifactRef` into a dialect, looks the owner up in `PLUGIN_CATALOG`'s declared rows, `installPlugin`s
  it, rebuilds `AppRouter` over `loadedPluginsRef.current` (the captured `appRouter` memo is one render
  behind `installPlugin`'s dispatch) and resolves once more. One install, one retry; a second miss
  rethrows the original fault.
- Call sites moved onto it: `…:5728` (`os.open-artifact`/`os.open-artifact-with` replay) and `…:7970`
  (space artifact-creation ready-opening). The pre-existing flow — UI already knows `{pluginId, appId}` —
  is unchanged: it succeeds on the first `resolveArtifactOpeningRelay` and never enters the catch.
- Activation reason at a real call site: `🧊️wgpu/🐚️plugin-bridge/🟦️.ts:1646` now passes
  `activationReasonForAppId(appId)` instead of the literal `"manual"`. `…:1519` (the extension request
  actor) stays `"manual"` — it has no app and no kind.

## 3. wgpu lazy per-artifact install (plan item 2)

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`:

- `:610` `ShellPluginInstallPhase` (`Resolving | Loading | Cancelled | Failed(String)`), `:621`
  `ShellPluginInstall { plugin_id, phase, cancel }`, field on `ShellState` beside `inference_port_status`.
- `:633` `plugin_modules_root_of(programs)` — reads `<modules_root>/<plugin_id>/<file>.wasm` back two
  levels off a resident program, the layout `load_wasm_plugins` publishes and the renderer's
  `activate_extensions_of` already relies on. No second copy of a path the renderer owns.
- `:9117` `install_plugin(plugin_id)` — resident is a no-op; one install at a time; phase advances
  `Resolving → Loading → cleared`; the `CancelToken` is checked at the step boundary AFTER the manifest
  read and BEFORE the entry is committed; a miss in the manifest is `Failed`, not a panic.
- `:9167` `cancel_plugin_install()` — fires the token for an in-flight install (phase does not move: only
  the install's own next boundary may report `Cancelled`), and clears a settled record.
- `:9183` `resolve_activation_owner_app(dialect, role)` — the wgpu twin of the React lookup, over
  `crate::program_bridge::resolve_artifact_kind_activation_owner`.
- `:9200` `switch_to_app` now calls `install_plugin` before the `self.plugins.iter().find(...)`, so the
  old unconditional `Err("program missing")` is gone.
- `:8480-8501` `handle_open_artifact_relay` — **this is the behavioural fix.** Before, the relay parsed
  `target.plugin_id`/`app_id` and then threw them away: it called `open_document` against whatever
  session happened to be mounted, so a hub showing `home` opened a cad document into `home`. It now
  switches session to the target's own app (installing the plugin if needed), or, with no explicit app
  ref, to the kind's declared owner, and only then opens the document. The function was already `async`,
  so the relay path needed no restructuring to await the install.
- `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:1067` re-exports `resolve_artifact_kind_activation_owner` and
  `PLUGIN_ARTIFACT_KIND_ACTIVATIONS` through `crate::program_bridge`.

On native this install is real (`ProgramBridgeEntry::from_wasm` compiles nothing until `create_app`, so a
manifest re-read is the whole install). On `wasm32` the whole path is `cfg(not(target_arch = "wasm32"))`
— the browser frame worker eager-mounts every plugin in the boot plan and Rust cannot fetch a module
there; see §4.

## 4. The real wgpu boot paths DO get the full host-mode list (plan item 3) — confirmed, no fix needed

Traced end to end, by reading (no cold build run here):

- **Native.** `⌨️native-entrypoint/📦️modules/📜️script.ts:22-34` builds the runtime manifest from
  `PLAYGROUND_SESSION.plugins` for the variant — for `s` that is host mode, i.e. all 60 rows —
  and `publishNativeRuntime` (`…/📦️modules/🟦️.ts:16`) writes every one into
  `dist/runtime/native/<profile>/<variant>/🔣️runtime.json`.
  `program_bridge::load_wasm_plugins(plugin_filter, modules_root)` passes `plugin_filter` to
  `NativeRuntimeManifest::read` as a **variant identity check only**
  (`⌨️native-entrypoint/📦️modules/🦀️.rs:27` — `value.variant != variant` ⇒ error); it performs no
  per-plugin narrowing, and `filter_plugins` (`🌉️ProgramBridge/…/🦀️.rs:1070`) is the identity function.
- **Browser.** `🎞️frame-worker/🟦️.ts:545-550` — `new PlaygroundBootPlanner(PLUGIN_CATALOG, descriptor.pluginVariant)`
  → host mode → every row; `mountPluginHandles` then eager-loads each with per-plugin fault isolation.
- `bootFrameworkOsWgpu` (`🎬️renderer-boot/🟦️.ts:139`) is the embeddable/Storybook door and is NOT on
  either real path — the audit's caveat about it stands but is not a gap.
- Observed on disk: the only staged native runtime today is `native/dev/puzzle3d` with `modules: 1`
  (correct for a non-host variant). No `s` native runtime is staged yet — C1's build produces it, and
  the count assertion for `s` lives in the session test below rather than in a build artifact.

## 5. Tests (plan item 5)

### The gate that was measuring nothing

`bun nx run @semio-tech/framework:test` printed **`No test files found, exiting with code 1`**. Root cause
is NOT the include list the audit suspected — `🧰️framework/🧪️tests/🎚️config/🟦️.ts` already named the kernel
in `includeSource`. Two real faults, both fixed:

1. `📚️library/🟦️.ts:2679-2692` `vitestRunArguments` passed `--config` **relative**. Vitest 4 resolves a
   relative `--config` against the root it detects (the repo root), not the launched cwd, so
   `../../🧪️tests/🎚️config/🟦️.ts` resolved to `<repo parent>/🧪️tests/🎚️config/🟦️.ts`, esbuild could not load
   it (observed directly: `Could not resolve "/Users/ueli/Documents/🧪️tests/🎚️config/🟦️.ts"`), and the run
   proceeded with no config at all. Now resolved against `bundleRoot`; the relative literals at every
   call site (the shape `runVitestConfigArgumentTokens` scans) are untouched. Blast radius is bounded to
   configs whose path escapes upward — a config path without `../` always resolved correctly, which is
   why the other suites were unaffected.
2. `🧰️framework/🧪️tests/🎚️config/🟦️.ts:12,34-35` — `root` was `📦️packages/🟦️typescript` while every
   `includeSource` entry was `../../…`, and vitest globs `includeSource` against `test.root` with a
   globber that cannot walk upwards. Root moved to the framework root, patterns made downward.

Consequence: `🎠️kernel/🟦️.ts`'s five `import.meta.vitest` blocks (`createTurnOutcomeBroadcast`,
`AppRouter`, `ActivationRegistry`, **`expandPluginRegistry` host mode**, `IoEntryGraph`) now actually run.
The first run exposed one genuine regression they had been hiding: `createDevPluginSource`'s malformed-frame
handler at `🎠️kernel/🟦️.ts:2853-2857` was an **empty `catch`** — the `console.warn` had been stripped, so a
malformed dev watch frame was swallowed silently. Restored.

```
$ bun ./📜️script.ts test          # 🧰️framework/📦️packages/🟦️typescript
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework
 Test Files  2 passed (2)
      Tests  141 passed (141)
```
(before the fixes: `No test files found, exiting with code 1`; after the config fix but before the warn
restore: `1 failed | 140 passed`.)

### New suites

**(3)+(4) `📇️registry/🧪️tests/🎬️host-activation/🟦️.ts`** — 6 tests:
- the host variant's session is the whole registry (`plugins.length === projection.entries.length`,
  `hostMode === true`, id-for-id equality) — the regression guard against `filterProjectedPluginRegistry`
  / `projectedHostPluginFilter` narrowing host mode back down;
- a non-host variant stays strictly smaller;
- every declared artifact kind is claimed by exactly one catalog row;
- `artifactKindActivationOwner` agrees with an independently rebuilt index for every kind, and returns
  `undefined` for `""` and an unknown kind;
- every declaring plugin owns its own kinds;
- **Nx-graph host fan-out**: a hermetic three-crate fixture (one declaring
  `host = { landing, shell }`) driven through `cacheInternals.playgroundPreparationTargets` asserts
  `prepare-hub-{react,wgpu,native}-{dev,release}.dependsOn` contains `<id>:materialize-<profile>` for
  **every** crate, that a non-host variant does NOT inherit the fan-out, and that
  `activate-hub-<renderer>-<profile>` follows its own prepare. (`⚡️cache-contracts/🟦️.ts:888` already
  asserted the `prepare-*-react-*` set equals `buildPlaygroundSession(variant)`'s components for every
  playground including `s`; what was missing — and is now pinned — is that for a HOST variant that set is
  the whole catalog, and that wgpu/native prepare fan out too.)

```
$ bun ./📜️script.ts test 🧪️tests/🎬️host-activation     # 📇️registry
 Test Files  1 passed (1)
      Tests  6 passed (6)
```

**(1) `🧑‍🎨engine/🧪️tests/🎬️activation-owner/🟦️.ts`** — 5 tests, registered at
`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts:60`. Exercises exactly the composition `ShellHost` performs, over
injected state instead of a React mount: a session holding only the host plugin, `os.open-artifact` for
`s.beta.sheet@1/*`, the relay failing, the owner resolved from the declared rows, installed, and the SAME
open routed against the rebuilt router into `beta`'s editor — plus "already loaded installs nothing",
"both kind spellings resolve", "unclaimed kind installs nothing", "uninstallable owner installs nothing".

```
$ SEMIO_TEST_LEVEL=long vitest run … 🎬️activation-owner
 Test Files  1 passed (1)
      Tests  5 passed (5)
```

**(2) `🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs`** — 5 tests, mounted at
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22851-22853`. Covers the generated owner table (one owner per kind, both cad
spellings → `cad`, more than one owning plugin, unknown/empty → `None`), the install refusal when no
resident program can name the modules root, the cancel semantics, and `plugin_modules_root_of(&[]) == None`.

```
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- plugin_install_tests
running 5 tests … test result: ok. 5 passed; 0 failed; 930 filtered out
```

### Compile gates

```
$ cargo check -p semio-framework-os-renderer-wgpu --lib --message-format short
warning: `semio-framework-os-renderer-wgpu` (lib) generated 62 warnings
    Finished `dev` profile [unoptimized] target(s) in 8.60s
$ cargo test -p semio-framework-os-renderer-wgpu --lib --message-format short --no-run
warning: `semio-framework-os-renderer-wgpu` (lib test) generated 109 warnings
    Finished `test` profile [unoptimized] target(s) in 1m 59s
```
Warnings present in both (proof the expansion actually ran, not an aborted check); 0 errors. All 62/109
warnings are pre-existing `unnecessary qualification` / `never used` noise in files I did not touch.

Peers held `cargo`/`rustc`/`trunk` processes throughout; only one cargo command of mine ran at a time and
none were killed.

### Adjacent suites, and what is NOT mine

- `🚪️opening` + `🧯️router-plugin-faults`: pass (23 tests) — the opening relay is unchanged for callers
  that already know `{pluginId, appId}`.
- `🏛️space-administration`: **2 failures, pre-existing/peer-owned** — `announce-document` is not yet in the
  admission fixture; `🔨️modules/📇️directory/🧬️schema/🟦️.ts` and `…/🦀️.rs` are uncommitted peer edits
  (hub-backend slice). Not caused by this slice.
- `@semio-tech/plugin-registry:test` as a whole: 8 pre-existing failures in `✅️trusted-stdio-catalog`
  (cargo target dir unavailable) and `🚀️launch` (a peer added a `flow-browser-package` generator target
  and renamed the repo-mcp launchers), and the full suite exceeds the 15 s `fundamental` budget (80 s).
  My suite is green in isolation and adds ~0.1 s.

## 6. Honest gaps / follow-ups

1. **The guest never receives an activation reason.** `kernel_activation_event_to_wit`
   (`🔌️plugin/🖥️host/🦀️.rs:2692`) maps `ActivationEvent::OnArtifactKind` onto the WIT variant and has
   **zero callers** in the tree. The declared→WIT bridge exists; nothing sends it. Wiring it needs an
   `Activate` event on the instance-open path, which is ABI work outside this slice.
2. **`ActivationRegistry.activate(pluginId, actorId, _reason)`** (`🎠️kernel/🟦️.ts:2345`) still ignores its
   reason parameter. The reason is now truthful at the call site (item 4 above); making the registry do
   something with it (e.g. lane/priority selection) is a separate decision.
3. **wasm32 wgpu has no lazy install.** The browser frame worker eager-mounts the whole boot plan, and
   Rust cannot fetch a module from inside the wasm renderer. A browser-side lazy install needs a JS-side
   `installPlugin` hook on the frame-worker transport — mirrors React's, but is its own packet.
4. **Chrome for `ShellState::plugin_install`.** The phase/cancel state is retained and unit-tested; no
   band paints it yet. The wgpu chrome is under active peer edit (`WGPU-RENDERER-REACT-PARITY`), so I did
   not add a band there.
5. **Cold `dev s` timing** (audit plan item 5) is C1's, not measured here.
6. `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` show a 3-line diff after my registry
   regeneration: a peer's `.claude/launch.json` edit (`repo-mcp-go` → `repo-mcp`) that had not been
   regenerated yet. Left as regenerated — reverting would re-desync the derived file from its source.

## 7. Verified at runtime vs. by tests only

| Claim | How |
|---|---|
| `dev multi` is gone and the root script still bundles | `bun build` on `📜️script.ts`, exit 0 |
| host variant session = whole registry | test, against the real regenerated catalog |
| one owner per artifact kind, both spellings | tests (TS + Rust), against the real catalog |
| host variant Nx fan-out to every `materialize-<profile>` | test, hermetic Nx fixture |
| React hub install-then-open composition | test (injected state, not a React mount) |
| wgpu install/cancel state machine + owner table | Rust `--lib` tests |
| wgpu relay now switches session before opening | **compile + code review only** — needs a live `dev s` wgpu boot (C1's chain) |
| native/browser wgpu boot receives all 60 plugins for `s` | **code reading only** — no `s` runtime staged yet |
| kernel in-source suites run under the real gate | `bun nx run @semio-tech/framework:test`, 141/141 |

## 8. Files changed

```
📜️script.ts                                                              (dev multi deleted, doc)
🧰️framework/🔨️modules/🎠️kernel/🟦️.ts                                      (catalog field, resolvers, warn restore)
🧰️framework/🧪️tests/🎚️config/🟦️.ts                                        (vitest root + includeSource)
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts                    (absolute --config)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts            (activationEvents passthrough)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts (surface kinds + ownership pass)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts (generated Rust owner table)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎬️host-activation/🟦️.ts        (new)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx          (activation-owner install)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs (re-export)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs (install + relay switch)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs (new)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts    (activation reason)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts  (suite registration)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎬️activation-owner/🟦️.ts        (new)
```

No new executable commands were introduced, so `.claude/launch.json` needed no entry (plan item 6).
