# O2 — closing O1's open items: activation reason, wasm32 lazy install, install chrome

Slice O2, worker-owned. Scope: items 1–4 of the O2 brief, i.e. `📓️o1-multi-plugin-hub.md` §6 gaps 1–4.
`PluginRuntime/🟦️.tsx` and `🏪️store/🔄️sync/🦀️.rs` (worker C1) were not touched — see §5 for the one
two-line handoff that leaves. The peer `WGPU-RENDERER-REACT-PARITY` session edited
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` throughout this slice; every edit here was made against a fresh read and
kept to the four seams named below. No `🗑️generated` folder was swept; no git-modifying command was run.

## 0. Decisions taken

1. **The activation reason is ONE string in the declaration grammar, not a tag plus a payload.**
   `ActivationReason` was `"on-artifact-kind" | …` — a tag with the kind thrown away, which is why
   nothing could be sent to a guest even after `activationReasonForAppId` existed. It is now the same
   dash-separated spelling `📓️design-abi.md` §2 and the catalog's own `activationEvents` rows already
   use (`on-artifact-kind:s.cad.cad`). Consequence: every existing `"manual"` call site still typechecks
   unchanged (22 of them), and the catalog row and the runtime reason are literally the same string
   instead of two vocabularies that have to be kept in step.
2. **`manual` reaches no guest, deliberately.** `📜️.wit`'s `activation-event` variant has six cases and
   none of them is `manual` — the host's own trigger is not a declared activation event. So
   `activationEventEnvelope("manual")` is `undefined` and no `activate` event is queued. Inventing a
   seventh case (or reusing `on-startup-finished`) would have made the guest's `activate` lie.
3. **The native Rust producer derives the reason from the app id, at the activation site.** Both native
   activation sites (`run.rs`'s `open`, the wgpu renderer's `create_app`) already receive the canonical
   surface app id the opening resolved, and that id carries the artifact kind by construction
   (`surface_app_id`). Deriving there is the exact twin of the TS `activationReasonForAppId` rule and
   needs no new parameter threaded through `ParallelRuntime`/`NativeKernelRuntime`/`install_actor`.
4. **`Event::Activate` rides the instance's FIRST turn, ahead of `Event::InstanceOpen`.** The kernel
   mailbox path (`Payload::Event` + `Kernel::submit`) would also work, but the renderer's `create_app`
   drives that first turn directly through `run_turn` rather than waiting on a `tick_and_dispatch`, so
   prepending is the delivery that is actually deterministic on both native hosts.
5. **wasm32 lazy install is a frame-Worker door, and `switch_to_app` is NOT its caller.**
   `handle_open_artifact_relay`/`switch_to_app`/`open_document` are `#[cfg(not(target_arch = "wasm32"))]`
   in the committed tree (verified at `HEAD`, not introduced by O1), so the browser wgpu shell has no
   artifact-open relay at all to hang a lazy install off. Un-gating that whole relay is a separate,
   much larger packet. What IS now real on wasm32 is the module source the relay will need plus a live
   caller: `install_plugin`/`cancel_plugin_install`/`ShellPluginInstall` compile and run on both targets,
   and `switch_to_managed_app` (both targets) installs before it looks the host program up — which also
   fixes a real fault, since `mountPluginHandles` skips a plugin whose module load faulted and a boot
   that lost the HOST plugin that way answered `host program missing` for every later switch forever.
6. **Cancellation settles the caller, not the fetch.** Nothing can abort a module fetch already in
   flight, and killing it would discard work the next request repeats. Both the browser door and the
   React band reject/abort the AWAIT immediately and let the load finish into the loader's own cache.

## 1. Activation reason reaches the guest (brief item 1)

### TS

- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2134` — `ActivationReason` is now the payload-carrying
  declaration grammar (`on-artifact-kind:${string}` … plus `on-startup-finished` and `manual`).
- `…:2150` `activationEventEnvelope(reason, instance = 0)` — the whole declared→WIT bridge on this side:
  `{kind:"activate", payload:{instance, reason:{tag, val}}}`, the `{kind, payload}` envelope
  `🏗️materialization/🟦️.ts:883`'s `lifecycleEvent` lifts into jco's `{tag, val}` variant. `manual` ⇒
  `undefined`.
- `…:1279` `activationReasonForAppId` now returns `on-artifact-kind:<kind>` with the kind the surface id
  carries (it previously returned the bare tag).
- `…:1251` `ON_EXTENSION_REQUEST_ACTIVATION_PREFIX`.
- `…:2427` `ActivationRegistry.activate(pluginId, actorId, reason)` — `_reason` is gone; the reason is
  queued to the freshly resident actor at `…:2434` via `enqueueTurn(actorId, "Maintenance", [envelope])`.
- `…:2472` — the extension cascade activates each child `on-extension-request:<parent>`, the declared
  event that names exactly what pulled it up.
- `🧊️wgpu/🐚️plugin-bridge/🟦️.ts:1646` already passed `activationReasonForAppId(appId)` (O1); it now
  carries the kind, so that call site became real without an edit.

### Rust

- `🔌️plugin/🖥️host/🎠️activation/🦀️.rs:16` `activation_event_for_app_id(app_id)` →
  `Some(ActivationEvent::OnArtifactKind { kind })` for a canonical surface id, `None` for a bare
  landing/host app id.
- `…:25` `activation_turn_event(app_id)` → `Some(Event::Activate { reason })`. This is the one producer
  of `Event::Activate` in the tree; `kernel_event_to_wit` (`🖥️host/🦀️.rs:2637`) already marshalled it
  onto `activate(activate-event)` and had no producer at all before.
- `🖥️host/🦀️.rs:1630,2637` — the `wit_*` alias block and `kernel_event_to_wit` are now `pub(crate)`, so
  the marshal is assertable from a sibling module's test instead of only from inside `component`.
- `🔨️modules/🏃️run/🦀️.rs:2016` (headless native host) and
  `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:7041` (rendered native host) — the first turn is now
  `[Activate?, InstanceOpen]` instead of `[InstanceOpen]`.

### Tests

**Rust — `🔌️plugin/🖥️host/🎠️activation/🧪️tests/🎠️activation/🦀️.rs:68`** (new, in the host activation
module the brief names): derivation for both spellings, `None` for a landing id and for a coordinate
without a role suffix, and the marshalled value the guest's `activate` actually receives.

```
$ cargo test -p semio-framework-plugin-host --lib --message-format short -- activation
running 2 tests
test component::activation::tests::artifact_kind_app_ids_reach_the_guest_as_the_declared_activation_event ... ok
test component::activation::tests::neutral_activation_failures_retire_the_exact_kernel_and_guest_owners ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 222 filtered out
```

**TS — `🧑‍🎨engine/🧪️tests/🎬️activation-owner/🟦️.ts` (3 tests appended to O1's suite)**: the reason derived
from the app id the hub's install-then-open actually resolves; the exact WIT-shaped envelope for
`on-artifact-kind`/`on-extension-request`/`on-startup-finished` and `undefined` for `manual`; and a REAL
`ActivationRegistry` over a real `ShardClient` with a recording fake worker, asserting the guest-bound
`turn` message carries exactly
`[{kind:"activate", payload:{instance:0, reason:{tag:"on-artifact-kind", val:"s.beta.sheet"}}}]`, and that
a `manual` activation sends no turn at all.

## 2. wasm32 wgpu lazy install (brief item 2)

- `🎞️frame-worker/🧩️lazy-install/🟦️.ts` (new) — `createLazyPluginInstallDoor({moduleUrl, mount, progress})`:
  per-phase progress (`resolving`/`loading`/`mounted`/`cancelled`/`failed`), request de-duplication (a
  second request for the same plugin joins the first fetch), immediate withdrawal on cancel, and
  `plugin-install.*`-prefixed refusals. Injected seams, so it is testable with no `Worker` global.
- `🎞️frame-worker/🟦️.ts:737-752` — the door wired to the real catalog and to the SAME
  `loadPluginModule` + `pluginHandleForBridge` pair `mountPluginHandles` uses, under the same
  `monitoredSuspension` watchdog declaration, and published as `globalThis.semioWgpuInstallPlugin` /
  `semioWgpuCancelPluginInstall` (the `semioWgpuHostIo` idiom).
- `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:1063,1068,1080,1094` — `js_plugin_install_door_available`,
  `install_js_plugin` (awaits the door's promise and admits the handle through the existing
  `ProgramBridgeEntry::from_js`, so there is ONE handle shape for eager and lazy mounts) and
  `cancel_js_plugin_install`.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `ShellPluginInstallPhase` (`:611`), `ShellPluginInstall` (`:621`),
  the `plugin_install` field and its constructor init are no longer native-only; `install_plugin`
  (`:9207`) and `cancel_plugin_install` (`:9254`) are now on both targets, with the two target-specific
  steps split out as `plugin_install_precondition` (`:9164` native / `:9170` wasm32) and
  `acquire_plugin_program` (`:9180` native / `:9189` wasm32). The native precondition still refuses
  BEFORE any record is minted, so O1's `installing_without_a_resident_program_is_refused` law is intact
  byte for byte. `cancel_plugin_install` also withdraws the browser request.
- `…:9003` — `switch_to_managed_app` installs the host plugin before looking it up (see §0.5).

**Test — `🧑‍🎨engine/🧪️tests/🎬️wasm-plugin-install/🟦️.ts`** (new, 6 tests, registered at
`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts:61`): catalog url → mount, every phase reported; unknown plugin
refused without mounting; second request joins the first; cancel settles the caller immediately while
the fetch finishes and a re-request mounts warm; a mount fault reports `failed`, not `cancelled`;
cancelling an idle plugin is inert.

## 3. Chrome for `plugin_install` (brief item 3)

### wgpu

A new `ShellChromeFramePhase::PluginInstall` (`:18073`), painted directly after `TransientNotice` — the
shell's own existing banner channel — with the same stepped-cursor painter, the same `Theme` tokens and
the same severity tone map:

- `:16493` `PLUGIN_INSTALL_CANCEL_CONTROL_ID` (`shell.plugin-install.cancel`),
  `:16502` `plugin_install_banner_text`, `:16519` `plugin_install_tone`, `:16528` `plugin_install_rect`,
  `:16537` `plugin_install_action_rect`, `:20474` `render_plugin_install_step`.
- The control is a real cancel while the install runs (`ShellState::cancel_plugin_install`) and a clear
  once it settled; a `Cancelled`/`Failed` record does NOT auto-dismiss, because a refused open that
  vanished after 4 s is exactly what `ShellPluginInstallPhase::Failed`'s message exists to prevent.
- English first, German through the existing `shell_chrome_string` table (`:22174…`):
  `plugin.install.resolving|loading|cancelled|failed|cancel`.

### React

- `🛠️ShellHelpers/🟦️.tsx:1769` — `loadPluginModuleResilient` takes an `AbortSignal` and races it
  explicitly, so a cancel settles the await now; a cancelled load logs as a cancel, not a fault.
- `…:1810,1814` — `pluginInstallBandTextV1`/`pluginInstallCancelTextV1`, EN/DE through the file's own
  `frozenLabelText` idiom (no `ui.common.*` key covers "loading a plugin").
- `🏛️ShellHost/🟦️.tsx:2784` `pluginInstallAbortsRef`, `:3673` the signal handed to the load, `:3708`
  `installingPluginIds` derived from the `"installing"` status the install path ALREADY dispatches (no
  second notion of "an install is happening"), `:3713` `cancelPluginInstalls`, `:11019` the band itself —
  rendered immediately after the transient notice, `data-semio-plugin-install` /
  `data-semio-plugin-install-cancel` for probes.

## 4. Verification

Commands run from the repo root, one cargo at a time, foreground.

```
$ bun nx run @semio-tech/framework:test
 Test Files  2 passed (2)
      Tests  141 passed (141)          # includes the kernel's ActivationRegistry in-source block

$ SEMIO_TEST_LEVEL=long npx vitest run --root …/⚛️react/📦️packages/🟦️typescript \
    --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts \
    🎬️activation-owner 🎬️wasm-plugin-install 🚪️opening 🧯️router-plugin-faults 🫀️plugin-load-progress
 Test Files  5 passed (5)
      Tests  35 passed (35)

$ (cd 📇️registry && bun ./📜️script.ts test 🧪️tests/🎬️host-activation)
 Test Files  1 passed (1)
      Tests  6 passed (6)

$ cargo test -p semio-framework-plugin-host --lib --message-format short -- activation
test result: ok. 2 passed; 0 failed; 222 filtered out

$ cargo test -p semio-framework-os-renderer-wgpu --lib --message-format short -- plugin_install
test result: ok. 5 passed; 0 failed; 933 filtered out

$ cargo check -p semio-framework-plugin-host --lib --message-format short
    Finished `dev` profile [unoptimized] target(s) in 54.78s

$ cargo check -p semio-framework-os --lib --message-format short
    Finished `dev` profile [unoptimized] target(s) in 59.67s      # warnings present ⇒ expansion ran

$ cargo check -p semio-framework-os-renderer-wgpu --lib --message-format short
    (no errors)

$ cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format short
    Finished `dev` profile [unoptimized] target(s) in 40.09s
```

The wasm32 check is the one that matters for §2 — a native check never compiles `cfg(target_arch =
"wasm32")` code. It failed twice mid-slice with five `no field agent on ShellState` errors at
`🐚️Shell/…/🦀️.rs:6714-6744`; those were the peer's in-flight `ShellChromeBuildState.agent` move (the
file's mtime advanced between my two runs and the peer then repaired the call sites to
`self.chrome_build.agent`), never this slice's code, and the run above is clean.

A final re-check of `semio-framework-os-renderer-wgpu` at the end of the slice failed in
`semio-s-artifact-stdio-pdf` (`🚪️io/🦀️.rs`: `cannot find Lexer in cos`, six errors) — a peer's in-flight
PDF work in the shared crate graph, untouched here and unrelated to any file this slice edited. The
clean native and wasm32 runs quoted above were taken minutes earlier against the same edits.

`npx tsc --noEmit` over the react package's tsconfig reports 1067 pre-existing errors repo-wide; the
per-file set for every file this slice touched is byte-identical before and after its edits, and
`🧩️lazy-install`, `🎞️frame-worker/🟦️.ts`, `🎬️activation-owner` and `🎬️wasm-plugin-install` report none.
That tsconfig is not a clean gate, so it is evidence of "no new type errors", not of a green typecheck.

### Live `dev s` probe — NOT run, and why

`dist/<profile>/🔌️plugin-modules/` for the `s` hub **does not exist**
(`find … -path "*dist/dev/*plugin-modules*"` → 0 hits; `🧑‍💻dev/dist/plugin-modules` holds only
`_vendor` + `cad`). The staged `🧑‍💻dev/🔌️plugin-modules/` tree has 61 plugin folders but is the
activation-chain staging dir, not what `served` reads, and its wasm artifacts date from 09-06.
Per the brief's own fallback this slice relies on tests and no server was booted. Two further reasons
the probe would not have measured item 2 anyway: `📜️script.ts:275` forces `SEMIO_RENDERER=react` for
`served`, and the browser wgpu artifact-open relay is native-gated (§0.5), so the wasm32 lazy install
has no artifact-open caller to exercise yet. `🐍️o2-hub-open-foreign-kind-probe.mjs` was therefore not
written rather than written and left unrun.

## 5. Honest gaps / follow-ups

1. **React's own two `registry.activate` call sites still pass `"manual"`** —
   `🔌️PluginRuntime/🟦️.tsx:2633` (the extension request actor, correctly `manual`) and **`:3047`, inside
   `createApp(appId)`, which is the React DOM host's real artifact-open activation**. That file is worker
   C1's and was not edited. The one-line handoff for C1: at `:3047` replace
   `"manual" satisfies ActivationReason` with `activationReasonForAppId(appId)` (already exported from
   `@semio-tech/framework`); `:2633` stays `"manual"`. Everything under it is done — the registry
   delivers whatever reason it is given. Until that line lands, the kind-derived reason reaches the guest
   on native (both hosts) and through the wgpu browser bridge, but not through the React DOM host.
2. **The guest still ignores `Event::Activate`.** `⚛️reactor/🔄️turn/🦀️.rs:801` matches it into `{}`. The
   event now genuinely arrives with the real reason; making a guest DO something with it (lane choice, a
   kind-specific warm-up) is a guest-SDK decision, not host plumbing, and was out of scope.
3. **wasm32 has no artifact-open relay.** §0.5. Un-gating `handle_open_artifact_relay` / `switch_to_app` /
   `open_document` for the browser is its own packet (they pull in persistence bindings, the hub
   transport and `system_fs`); the module source and the install state machine they will need are in
   place and compile for wasm32 today.
4. **The wgpu install band is compile- and unit-verified only.** No wgpu boot ran in this slice, so the
   band's geometry is code review plus the shared painter it reuses, not a screenshot.
5. **No cold `dev s` timing** — still C1's, unmeasured here.

## 6. Files changed

```
🧰️framework/🔨️modules/🎠️kernel/🟦️.ts                                                     (reason grammar, envelope, activate delivery)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs                               (pub(crate) wit aliases + marshal)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🦀️.rs                  (reason derivation + Activate producer)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🧪️tests/🎠️activation/🦀️.rs (new law)
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs                                         (first turn carries Activate)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs (first turn carries Activate)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs (wasm32 install door)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs (both-target install + band)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts (door wiring)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🧩️lazy-install/🟦️.ts (new)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx (abort signal + band copy)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx  (install band + cancel)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎬️activation-owner/🟦️.ts (3 laws appended)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎬️wasm-plugin-install/🟦️.ts (new)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts (suite registration)
```

## 7. Verified at runtime vs. by tests only

| Claim | How |
|---|---|
| a surface app id derives `OnArtifactKind` with its own kind, native | Rust test, real `parse_surface_app_id` |
| the WIT `activate` the guest receives carries that kind | Rust test through `kernel_event_to_wit` |
| `ActivationRegistry.activate` queues that envelope to the guest, and nothing for `manual` | TS test, real `ActivationRegistry` + real `ShardClient` + recording worker |
| both native hosts prepend the event to the instance's first turn | **compile + code review only** — needs a live guest turn |
| the browser lazy-install door's phases, dedup, cancel and faults | TS test over injected seams |
| the wasm32 shell install path compiles and resolves its door | `cargo check --target wasm32-unknown-unknown`, clean |
| the wgpu install band paints | **compile only** — no wgpu boot in this slice |
| the React install band paints / cancels | **compile + code review only** — no React mount test written |
| the hub opens a foreign kind in a live browser | **not run** — no `dist/<profile>/🔌️plugin-modules/` (§4) |
