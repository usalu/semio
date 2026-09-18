# Audit — one `os s` frontend hosting ALL plugins/artifacts: architecture, seam-by-seam state, plan

Read-only audit, no files edited, no cargo builds run (grep/read/bun-nx-show-projects only). Builds on
`📓️audit-os-frontend.md` §0/§1/§2/§6 (read first, per the task) — this audit goes one level deeper and, on
several points, **corrects/refines** that audit's §0 tl;dr and P0 item, which had not traced the code this
far. Prior audit's headline claim — "`dev s` boots exactly one plugin (`space`)" — is **only true for the
`wgpu` renderer's plugin set; for the underlying build graph, session data, and the React renderer it is
false**: the repo already has a real, consistently-wired "host boots every registered plugin" design. The
gap is narrower and more specific than "not built" — see §2 for the precise seams.

## (a) Intended end-state architecture (as actually encoded in the repo, not just doc comments)

The repo already encodes, at **four independent layers**, the same rule: *a plugin whose Cargo manifest
declares `[package.metadata.semio].host` (only `space` does — `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:17`,
`host = { landing = "home", shell = "studio" }`) is a **hub**: its playground variant's plugin set is not
"itself + its dependency closure" but **the entire registered catalog** (all 60 `🔌️plugins.json` rows), and
any artifact of any registered kind is meant to open inside it by lazily instantiating that kind's owning
plugin's wasm module on demand, not by rebuilding/rebooting the shell.

1. **Nx build-graph layer** — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs:29`
   `runtimeComponentClosure(components, roots)`: normal closure walks `dependsOn`/`consumes↔contributes`
   from `roots` (`[playground.pluginId]`, e.g. `["space"]`), **but** line 29:
   `if (component.host) for (const hostId of byId.keys()) pending.push({ id: hostId, shallow: false });`
   — if the root component declares `host`, the closure is force-expanded to **every** component id ever
   discovered from a `Cargo.toml` (`playgroundPreparationTargets`, same file `.mjs:983-997`, feeding
   `📚️library/🟨️.mjs:1018-1035`'s generated `activate-<variant>-<renderer>-<profile>` / `prepare-*` Nx targets'
   `dependsOn: […selected].sort().map(id => materialize-<profile>)`, `.mjs:1010,1019`). **For `dev s`, this
   means the generated Nx graph really does make `prepare-s-{react,wgpu}-{dev,release}` depend on
   `materialize-<profile>` for every one of the ~60 registered plugin/extension crates**, not just `space`.
2. **Session-generation layer** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts:38-56`
   `buildPlaygroundSession(variant)`: `hostMode = projectedHostPluginFilter(projection, variant)` (host
   detection: `🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts:23-28` — "a variant whose crate declares
   `[package.metadata.semio].host`") → `entries = filterProjectedPluginRegistry(projection, hostMode ?
   undefined : registryPluginId)`. `filterProjectedPluginRegistry` (`📖️catalog-view/🟦️.ts:31-36`): "if no
   filter **or the plugin is a host**, return every projected registry row unfiltered." The generated
   `PLAYGROUND_SESSION.plugins` artifact for variant `s` therefore lists all ~60 plugins with `moduleUrl`s,
   not one.
3. **Kernel/runtime-resolution layer** — `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:461-462`
   `expandPluginRegistry(plugins, primaryPluginId, hostMode=false)`: `if (hostMode || !primaryPluginId)
   return plugins;` — again, host mode = "no filtering, return everyone." `PlaygroundBootPlanner`
   (`🎠️kernel/🟦️.ts:2988-3014,3076`, `resolvePlaygroundBoot`) computes `hostMode =
   resolvePluginHostConfig(catalog, variant) !== undefined` from the SAME `PluginHostConfig`
   (`landingAppId`/`hostAppId`) table (`🎠️kernel/🟦️.ts:3073-3093`; the doc comment at :3074-3076 names
   `space`'s home/studio pair explicitly) and, when a `PLAYGROUND_SESSION` for the same variant is passed
   in, just reuses `session.plugins` (`🎠️kernel/🟦️.ts:3010-3013`) — the runtime layer trusts the
   already-all-inclusive session artifact rather than recomputing.
4. **Client-runtime layer** — `ActivationRegistry` (`🎠️kernel/🟦️.ts:2228-2483`) is a genuine
   manifest-only, lazy, LRU-suspending microkernel: `registerCatalog(catalog)` seeds a
   `{pluginId → moduleUrl}` manifest for **every** plugin+extension without touching wasm
   (`🎠️kernel/🟦️.ts:2318-2331`, doc at :2071-2079 "no worker/module is touched until an actor actually
   activates"); `activate(pluginId, actorId, reason)` fetches+instantiates on demand
   (`🎠️kernel/🟦️.ts:2345-2352`). `ActivationReason`/the WIT ABI
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:668`, `on-artifact-kind(string)`) and the
   registry's declared `activationEvents` (`🔌️plugins.json`, e.g. `"space":
   ["on-artifact-kind:space.shome","on-artifact-kind:space.sspace"]`) all agree this is meant to be
   triggered by "the artifact the user just opened names this kind."

**Intended end-to-end flow**: `dev s` builds+serves the whole catalog once (expensive, §c) → the space hub
boots with a manifest naming every plugin but wasm-instantiating only what's actually opened → user opens
(or creates) an artifact of any registered kind from the hub's `home`/`studio` apps → the host resolves
`{pluginId, appId}` for that artifact's dialect (`ArtifactKindChoice`/`artifactKindChoices`, added by ticket
`26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`, replacing a hardcoded
`KNOWN_ARTIFACT_KINDS` list that no longer exists in the tree — confirmed by
`grep -rl KNOWN_ARTIFACT_KINDS` returning zero hits) → the shell lazily activates that plugin's wasm module
if it isn't resident yet and opens the document inside the SAME hub session, no reboot.

## (b) Seam-by-seam current state

### Seam 1 — `dev multi` (root `📜️script.ts:416-425` → `@semio-tech/framework-os-dev:dev -- multi`)

**Broken, and redundant with the design above.** `"multi"` has no matching `Cargo.toml`/registry row (root
script's own comment at `📜️script.ts:417-420` says so), so it can never resolve through
`resolveFrameworkOsPlaygroundPlugin`/the catalog — the branch bypasses that entirely and shells straight
into the Nx `dev` target. That target (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:127-140`)
is **hardcoded**: `dependsOn: ["@semio-tech/framework-os-dev:activate-s-wgpu-dev"]` and
`command: "bun ../../../📺️renderer/…/wgpu/🌐️server/📜️script.ts serve s dev"` with `forwardAllArgs: true`.
Forwarding `"multi"` appends it as a 4th positional arg to `ServeScript.run([variant, profile, ...args])`
(`…/wgpu/🌐️server/📜️script.ts:8-26`), where `variant` is already the literal `"s"` and the `args` loop
only understands `--port`/`--host` pairs (`:13-17`) — `"multi"` fails that check and the script `throw`s
`"Unknown browser server option: multi"` before Vite even starts. **Verdict: `dev multi` is dead/crashing
code and, given §(a)'s host-mode fan-out already makes `dev s` do the "boot every plugin" job, it should
either be deleted or repointed to alias `dev s` — not "implemented" as a separate mechanism** (contra the
prior audit's P0 framing that it "needs implementing").

### Seam 2 — build/materialize graph (Nx) for `dev s`

**Present, correct in design, expensive in practice.** §(a).1 shows `prepare-s-{react,wgpu}-{dev,release}`
already depends on `materialize-<profile>` for all ~60 crates via `runtimeComponentClosure`'s host
fan-out. This was **not exercised** by this audit (no cargo run, per instructions) and its cost is real:
every one of the ~60 plugin/extension wasm crates has to compile at least once before `dev s` can serve. No
evidence was found of a smaller "declared-reachable-kinds-only" fan-out (e.g. only kinds with an
`on-artifact-kind:*` activation event some particular demonstrator cares about) — it is all-or-nothing.
Nx caching means a *warm* `dev s` is cheap; a *cold* one (fresh checkout / after a registry-wide
regeneration) is not. No evidence this specific cost has ever been measured end-to-end (no ticket found
reporting a full `dev s` cold-boot wall-clock time); it is inferred from the generator logic, not observed.

### Seam 3 — session/registry data reaching the client

**Present and correct.** §(a).2/.3: `PLAYGROUND_SESSION.plugins` for `s` really does list every registered
plugin+extension with a `moduleUrl`; `🧑‍💻dev/🟦️.ts:20` (`resolvePlaygroundBoot(PLUGIN_CATALOG,
variant, PLAYGROUND_SESSION)`) reuses it verbatim; `🧑‍💻dev/🟦️.ts:57-64` passes `boot.plugins` (plural, all
60) into `bootFrameworkOs({ plugin: pluginFilter, plugins, … })`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:1218-1233`), which mounts
`<FrameworkOsShell … plugins={options.plugins} …/>`.

### Seam 4a — React `ShellHost`: lazy cross-plugin activation on artifact-open

**Present and working, landed by the 26/08/17 "universal artifact creation" ticket.** This is the
concrete answer to the prior audit's P0 question ("can the hub load another plugin's wasm at runtime").
- `ActivationRegistry`/`registry` prop = the full host-mode plugin list (§(b) Seam 3).
- `installPlugin(pluginId, rebuiltAt)` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3644-3680`):
  looks `pluginId` up in `registry`; if present but not yet loaded, fetches+instantiates
  `pluginSource.moduleUrl(pluginId, rebuiltAt)` via `loadPluginModuleResilient`
  (`🛠️ShellHelpers/🟦️.tsx:1758-1776`) → `loadPluginModule` (wgpu-bridge-style adapter,
  `🔌️PluginRuntime/🟦️.tsx` region header at :4-18 names this exact "microkernel-pooled-actor" design).
- `openArtifactWithAppRef(target: AppRef, dialect, role, …)` (`🏛️ShellHost/🟦️.tsx:7867-7893`): "installs
  the target plugin first if it isn't loaded yet" (doc comment at :7864-7867) — `if (!plugin) { const
  outcome = await installPlugin(target.pluginId); … }`, then `handle.createApp(app.id)`, then
  `publishPreparedArtifactOpening`.
- Wired to the actual UI action: `os.open-artifact`/`os.open-artifact-with`
  (`🏛️ShellHost/🟦️.tsx:5717-5729`) → `resolveArtifactOpeningRelayRef.current(actionId, argsRecord)` resolves
  `{app, dialect, role}` → `openArtifactWithAppRefRef.current(...)`.
- `artifactKindChoices(manifests, roles)` (`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1379`, consumed at
  `🛠️ShellHelpers/🟦️.tsx:2781`) resolves which kinds/apps are choosable from the LOADED manifests — this is
  the "host-resolved manifest control" that replaced the hardcoded `KNOWN_ARTIFACT_KINDS` list the
  `SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION` ticket (`.🧬semio/…/🎆️26/🌙️08/☀️17/
  SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION/🎫️ticket.json`) describes fixing; confirmed
  gone from the tree (`grep -rl KNOWN_ARTIFACT_KINDS` → 0 hits) and its landing lane's own report
  (`📓️1-a-finish-report.md:150-161`) confirms the Rust half (`ActionArgControl::ArtifactKind`/`SurfaceApp`,
  `PluginBuilder::editor/viewer` schema stamping) was already committed by the time that finisher ran.

**Caveat found nowhere fixed**: every real call site of `ActivationRegistry.activate`/`registry.activate`
across the whole tree (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:1518,1645`
and its generated frame-worker mirror `…/🎞️frame-worker/🤖️generated/🟨️.js:26394,26533`) passes the literal
reason `"manual"` — never `"on-artifact-kind"`. The `on-artifact-kind:<kind>` activation events declared per
plugin in `🔌️plugins.json` and the matching `ActivationEvent`/WIT variant are **type-level plumbing with no
production caller that actually keys off an artifact's kind automatically** — activation is always driven
by an explicit UI action (`os.open-artifact`) that already resolved `{pluginId, appId}` up front via
`artifactKindChoices`, not by the host inferring "this artifact's kind belongs to plugin X" from the
activation-event string at the moment of open. Functionally this doesn't block the flow (the UI-resolved
path works), but it means the declared `activationEvents` field is presently **catalog/documentation
metadata only** — exactly what the prior audit's §2 flagged as unconfirmed, now confirmed on both the TS
and Rust sides (see Seam 5).

### Seam 4b — wgpu `Shell`: NOT the same mechanism, no lazy cross-plugin activation

**This is the real, concrete gap**, and it is narrower/more precise than "wgpu↔react parity is broken" —
it's a specific missing fallback:
- `bootFrameworkOsWgpu` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts:139-163`)
  eagerly `Promise.all`s `loadPluginModule` over **every** entry in `options.plugins` at boot
  (`:161-163`) — no lazy per-open loading exists on this path at all; wgpu's design is "load everything up
  front," which is consistent with §(a)/(b) Seam 2's cost but means wgpu genuinely needs every registered
  plugin's wasm instantiated (not merely built) before first paint, whereas React can defer.
  Its **only non-test caller** in the tree is a Storybook story
  (`🧰️framework/🛍️products/💻️os/📖️stories/🧭️coordination/🟦️.tsx:149`) passing a single-plugin array — so it is
  not even confirmed to be the function the real `dev s` wgpu native/browser entrypoint uses; the real wgpu
  boot path (`⌨️native-entrypoint/📜️script.ts`, `🎞️frame-worker/🟦️.ts`) was not fully traced in this pass and
  is a follow-up (see plan §(c)).
- `switch_to_app(&mut self, plugin_id, app)` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9046-9075`):
  `self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned().ok_or("program missing")?` — if
  the target plugin isn't already in `self.plugins` (populated once, at construction/boot), this **returns
  an error, full stop**. There is no `installPlugin`-equivalent lazy-fetch fallback anywhere in this
  function or its caller (`open_artifact_relay_target`/the `os.open-artifact` dispatch at
  `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8153,8430,8442`, which resolves `OpenArtifactRelayTarget{plugin_id,
  app_id,…}` at `:504-557` and then calls into this same session-switch path). The ticket description that
  named this bug ("the wgpu relay … never switched plugin") is now only *half* fixed: the relay correctly
  parses/carries a target `plugin_id` and a real generic `switch_to_app` exists (replacing the old
  `switch_to_s_app` per the comment at `:8882`), but it still can't switch to a plugin that wasn't eager-
  loaded at boot. Given wgpu eager-loads its full `options.plugins` set (previous bullet), this is
  self-consistent **only if** the real wgpu boot path genuinely passes the full host-mode plugin list — a
  claim this audit could not confirm end-to-end (see gap above).

### Seam 5 — Rust OS host (`🖥️host/🎠️activation/🦀️.rs`) and native wgpu renderer activation

**Refines the prior audit's "no hits" finding.** `ActivationEvent` (imported from `semio_framework_actor`,
`🖥️host/🎠️activation/🦀️.rs:7,78`) IS used — but only as a generic parameter threaded into
`KernelActivationRequest` (`:82`) for the scheduler's own bookkeeping, not matched against `on-artifact-kind`
anywhere in that file. The native wgpu renderer's own `.activate(...)` call sites
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:7127,8354`) pass
`ActivationEvent::Manual`/`ActorActivationTrigger::Manual` exclusively — the same "declared-but-unused for
its named purpose" pattern as the TS side (Seam 4a). **Conclusion for the prior audit's P0: the
`activationEvents`/`on-artifact-kind` field is real, typed, cross-language (WIT + TS + Rust) plumbing, but
confirmed — now on both language sides — to have zero production call sites that key activation off an
opened artifact's kind automatically.** Every real activation in the tree is either (i) boot-time
(`SEMIO_PLUGIN`/host-mode fan-out, eager for wgpu) or (ii) explicit-UI-action lazy install (`"manual"`,
React only).

### Seam 6 — `space` plugin itself

Confirmed via `Cargo.toml:17` (`host = { landing = "home", shell = "studio" }`) and
`✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs` (`S_PLAY_APP_ID`/`"studio"` app,
`.document(["semio","s","studio"])`) plus `🗿️artifacts/🏠️home/🦀️.rs` (`component_kind: "home"`,
`HomeSpaceRow` directory listing). `space` does **not** hardcode other plugins' kinds any more (§(a) —
`KNOWN_ARTIFACT_KINDS` deleted); it dispatches the generic `os.open-artifact`/`os.open-artifact-with`
actions (Seam 4a) and lets the shell resolve `{pluginId, appId}` via `artifactKindChoices` against
whichever manifests are loaded. `space`'s own declared kinds (`space.shome`, `space.sspace` per
`🔌️plugins.json`) are handled by its own `home`/`studio` apps exactly like any other plugin's kind would be
by that plugin's own app — no special-casing found beyond the `PluginHostConfig` landing/shell role table.

## (c) Implementation plan

1. **Delete or alias `dev multi`** (`📜️script.ts:416-425`) — it is dead, always-throwing code once you
   accept that `dev s`'s existing host-mode fan-out already IS "boot every plugin." Either remove the
   branch (call out in a CHANGELOG-style comment why) or make it `runFrameworkOsPlaygroundDev("s",
   segments.slice(1))` so `bun ./📜️script.ts dev multi` is a harmless alias instead of a crash. Cross-check
   the `@semio-tech/framework-os-dev:dev` Nx target string too
   (`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:127-140`) since it hardcodes `serve s dev` regardless
   of the segment — no separate `multi` target exists to alias against, which is further evidence "multi"
   was never wired past the `📜️script.ts` comment layer.
2. **Give wgpu a lazy per-artifact plugin-install fallback that mirrors React's `installPlugin`.** Exact
   seam: `switch_to_app` (`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9046-9048`) — when
   `self.plugins.iter().find(...)` misses, instead of `ok_or("program missing")?`, resolve the plugin's
   `moduleUrl` from the same catalog the boot path already has (`PLAYGROUND_SESSION`/host-mode plugin list)
   and call the WGPU equivalent of `loadPluginModule`/`ActivationRegistry.activate` before retrying the
   `find`. This needs a corresponding async-capable call site change at
   `open_artifact_relay_target`/its caller (`:504-557`, `:8153-8450`) since plugin instantiation is async
   and the current relay path assumes synchronous lookup. This is the single highest-value fix for wgpu
   parity on this specific axis (separate from, and narrower than, the general WGPU-RENDERER-REACT-PARITY
   ticket's chord/camera/pane-body work).
3. **Confirm (or fix) that the real wgpu native/browser boot path — not just the Storybook-only
   `bootFrameworkOsWgpu` — actually receives the full host-mode plugin list for `dev s`.** Trace
   `⌨️native-entrypoint/📦️modules/📜️script.ts` and `🎞️frame-worker/🟦️.ts:446`'s `monitoredSuspension`
   call chain back to where `PLAYGROUND_SESSION`/`resolvePlaygroundBoot` output actually reaches the wgpu
   renderer's plugin list; this audit found the LRU/suspension machinery (`monitoredSuspension`,
   `suspensionLedger`) but did not confirm it's fed by the full 60-plugin session for variant `s` specifically
   — that confirmation (or the fix if it isn't) is the prerequisite for item 2 above to matter in practice.
4. **Wire `on-artifact-kind` activation into an actual automatic-resolution path**, if the product intent is
   "opening a bare artifact file/id with no prior `{pluginId, appId}` should resolve its owner plugin from
   the kind string" (as opposed to always going through a UI action that already knows `{pluginId, appId}`
   via `artifactKindChoices`). If the existing "UI resolves `{pluginId,appId}` up front, then installs
   lazily" flow (Seam 4a) is considered sufficient, this item can be closed as "declared metadata, correctly
   unused" rather than a bug — needs a product decision, not just an engineering one; flag to whoever owns
   this ticket's collaboration/AI scope.
5. **Measure and budget the cold-build cost.** No evidence found that anyone has run a cold `dev s` and
   recorded wall-clock/OOM behavior. Given `📌️ project-semio-build-budget-and-oom.md`'s documented ~20-minute
   cargo budget and swap-thrash risk, and that `dev s` cold now provably fans out to ~60
   `materialize-<profile>` targets (§(a).1/(b) Seam 2), this should be run once, in isolation (no concurrent
   cargo), with `--keep-going`-style per-target accounting, before anyone treats `dev s` as a fast local
   loop. The existing receipt/materialize-gate machinery (`component-<profile>`→`materialize-<profile>` per
   crate, Nx-cached) is already the right reuse point — no new gate needed, just a first real cold
   measurement and, if too slow, a decision about whether `dev s` should default to `served` mode
   (`📜️script.ts:255-265`'s doc comment: skips the activation chain, serves whatever's already in
   `dist/<profile>/🔌️plugin-modules/`) for everyday dev loops with a separate CI/nightly job doing the cold
   all-60 build.

## (d) Tests — existing and needed

**Existing, and confirmed NOT wired into the real CI gate**: `expandPluginRegistry`'s host-mode branch has
an `import.meta.vitest` test block inside `🎠️kernel/🟦️.ts` itself (referenced by
`.🧬semio/…/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION/📓️1-a-finish-report.md:216-221`:
"`🎠️kernel/🟦️component.ts`'s pre-existing `describe("expandPluginRegistry", ...)`… blocks… never run under
the real `bun nx run @semio-tech/framework:test` gate" because
`🧰️framework/📦️packages/🟦️typescript/🧪️vitest.config.ts`'s `include`/`includeSource` only names
`"🟦️glue.ts"`). This is a live, still-open `sharedFileRequest` from that ticket (item 3 in
`📓️1-a-finish-report.md:226-236`) — **the exact host-mode logic this report's §(a).3 depends on is
currently untested in CI**, not because no test was written but because the test file isn't in the vitest
include list. Fixing that include list (or moving the tests into `🟦️glue.ts`) is a prerequisite for trusting
future changes to `expandPluginRegistry`/host-mode.

**Existing but not host/cross-plugin-specific**: `🧑‍💻dev/🧪️tests/🧪️multi-shell-harness/🟦️.tsx` mounts several
independent `FrameworkOsShell` instances on one page for `ShellScope` testing — this is "many shells," not
"one shell with many plugins," and does not exercise `installPlugin`/`openArtifactWithAppRef`.
`🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts` drives a Playwright/Studio e2e smoke but neither file's grep for
`installPlugin`/`hostMode`/`openArtifactWithAppRef` returned any hits.

**Needed** (none of these were found anywhere in the tree):
1. A unit test exercising `installPlugin`/`openArtifactWithAppRef` in `🏛️ShellHost/🟦️.tsx` for the
   specific "hub session, plugin B not yet loaded, open an artifact of B's kind, plugin B gets installed
   and the artifact opens in the same session" path — the core claim of §(a)'s end-state.
2. The wgpu-side equivalent, once plan item 2 lands — a test that opens an artifact whose plugin isn't in
   `self.plugins` and asserts it gets installed rather than `Err("program missing")`.
3. A cheap, CI-safe (no live cargo) test asserting `buildPlaygroundSession("s").plugins.length` equals the
   full registry count (`🔌️plugins.json` row count) — a regression guard against `filterProjectedPluginRegistry`
   or `projectedHostPluginFilter` accidentally narrowing host mode back down.
4. An Nx-graph assertion (similar to the existing `⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts` and
   `⚡️cache-contracts/🟦️.ts:869-874` patterns, which already assert `activate-<variant>-<renderer>-<profile>`
   `dependsOn` shapes for *non-host* variants) extended to assert the **host** variant's `dependsOn`
   actually includes `materialize-<profile>` for every registered plugin — currently only the non-host case
   is asserted by name in those two files.
5. A real, once-off cold-build timing run per plan item 5, captured as a ticket artifact (not a repeatable
   CI test — too expensive — but a documented baseline).

## Evidence index (file:line citations already inlined above; no raw-capture dump was produced this audit —
all findings were verified by direct file reads/greps listed inline, not by running the code)
