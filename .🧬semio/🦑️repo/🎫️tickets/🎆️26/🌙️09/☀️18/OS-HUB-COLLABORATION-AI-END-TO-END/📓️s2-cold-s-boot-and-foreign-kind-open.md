# S2 — Cold `s` boot to a served page, foreign-kind open inside the real `s` host

Slice S2, fleet 4, started 2026-09-20 ~00:55. Scope (from `📓️g9-s-product-space-audit.md` §5 items 1, 3, 4
and `📓️g10-goal-gap-reaudit.md` §D "Outcome 1"):

1. run the real `s` activation pipeline cold to a served page and capture the receipt;
2. live-probe a foreign-kind open (new + example, one proven plugin + one formerly dormant) inside the
   actual `s` host, then hub workspace / `/hub` inside `s`;
3. trace how the React host carries the activation reason to the guest and make it real, with a unit test;
4. make the proving command permanent (nx target + `launch.json` row).

Status legend: **measured** = this slice ran it and captured output; **unverified** = read from source only.

## 0. tl;dr

**`dev s` cold-booted to a served page for the first time on this ticket, with all 60 components staged
and activated and none excluded.** Measured 2026-09-20 06:40:

- `Activated s react dev: 60 completed components (changed)` — `🗑️generated/s2-prepare-activate.txt`
- `Cold boot s react dev: 60 of 60 components staged and activated, 0 excluded` —
  `🗑️generated/s2-cold-boot-check.txt`, via the new `cold-boot-check` verb
- served: **http://127.0.0.1:6070/**, `HTTP 200`, `VITE v7.3.6 ready in 18574 ms`, pid 26173 —
  `🗑️generated/s2-serve.txt`

Getting there took 4 attempts over 6 h and the two blockers were neither of the ones on file. G9 §1.5
blamed `🧱️block`; `block` built and staged cleanly at 02:36 inside this run. What actually stopped it was
(a) the nx daemon timing out on `HASH_TASKS` under fleet load, and (b) a 4 h cargo deadlock in the shared
build dir that the coordinator broke at 06:12. The activation was finished by running the underlying
`prepare`/`activate` verbs directly (preamble rule 16) once the staging root was complete.

One real product fix landed: preparation was **all-or-nothing**, so any single red crate of ~60 made the
whole hub un-bootable — the healthy-set rule (§2.1) now names and excludes a failed component instead,
refusing only when the host's own component is missing. Measured payoff: at 02:40 this tree had `stdio`
unstaged and would have died; it booted.

Two claims this slice was handed are **stale and are corrected with evidence**: the React DOM host does
carry the real activation reason (`🔌️PluginRuntime/🟦️.tsx:3048` — the earlier "no call any more" readings
were a `grep` binary-file artefact, §5), and `HubWorkspace`/`/hub` are **not** host-mode-gated away from
`s` — `s` *is* the host mode, so the `/hub` branch is reachable there and only there (§4).

## 1. Boot timeline

Recipe used (from `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts:331-344`,
the only place in the tree that already scripts a cold `s` stage — activate once via Nx, serve detached via
the `🧑‍💻dev` bundle script, never `dev s` which is a watch target that re-activates on any peer's write):

- activate: `bun nx run @semio-tech/framework-os-dev:activate-s-react-dev`
  (`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `SEMIO_BUILD_BUDGET_MS` deliberately UNSET = 24 h ceiling)
- serve: `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts serve s react dev`

| t | event | capture |
|---|---|---|
| 00:47 | attempt 1 launched detached, pid 35711, machine load 111, 66 GiB free | `🗑️generated/s2-activate.txt` |
| 01:10 | attempt 1 completed 19 of 132 tasks (1 cache hit), `Run duration: 22m 31s`, through `cad-extension-aec-building-rust:component-dev` | same |
| 01:10 | attempt 1 **exited 1**: `NX The daemon timed out while processing HASH_TASKS`, 113 tasks `skipped`, no receipt written | same |
| 01:44 | attempt 2 launched detached, pid 9889, `NX_DAEMON=false`, `--output-style=static`, load 158, 48 GiB free | `🗑️generated/s2-activate2.txt` |

**Measured fault, attempt 1 — the nx daemon, not a crate.** The line that ended the run is
`NX   The daemon timed out while processing HASH_TASKS`, exactly the failure mode the worker preamble's
rule 16 names for this machine under fleet load (load average was 158–175 at the time, ≈ 12 concurrent
cargos on the shared build dir plus a peer session's own test wave). It is not a compile failure: the 19
tasks that ran all printed `✔`, including `@semio-tech/block-plugin:component-dev` and
`@semio-tech/block-plugin:materialize-dev` — **`🧱️block`, the single crate G9 §1.5 named as the one that
stops a cold `dev s`, now builds and materializes**. Attempt 2 therefore resumes from a warm Nx cache for
those 19 tasks and runs with the daemon disabled.

| 02:36 | `🧱️block` staged **inside this run** | `dist/dev/🔌️plugin-modules/🧱️block` mtime |
| 03:00 | fleet cut by the account session limit; wrapper survived (ppid 1) and kept building | — |
| 03:02–05:42 | `stdio`, `puzzle` and the remainder staged while the session was down | staging root mtimes |
| ~02:00–06:12 | **cargo deadlock**: 34 cargos at 0 % CPU for ~4 h, no `rustc` anywhere, all in `prebuild_lock_exclusive` → `flock`; coordinator killed the set at 06:12 | preamble rule 23 |
| 06:13 | attempt 2's `animate` cargo reported `Cargo artifact build failed` after `elapsedMs=15573446` (4.3 h) — it was one of the deadlocked set | `🗑️generated/s2-activate2.txt` |
| 06:23 | attempt 2 advanced past the failure to `architect`; 65 entries in the staging root | — |
| 06:33 | staged set re-measured: **60 of 60 prepared, 0 excluded** | `🗑️generated/s2-staged-table.txt` |
| 06:36 | attempt 2 wrapper (and its 4 descendants) killed by pid — position no longer needed | — |
| 06:36 | `prepare` + `activate` run **directly** (preamble rule 16, the nx wrapper had spent 4 h at task 11 of 132): `Activated s react dev: 60 completed components (changed)` | `🗑️generated/s2-prepare-activate.txt` |
| 06:37 | `cold-boot-check s react dev`: `60 of 60 components staged and activated, 0 excluded` | `🗑️generated/s2-cold-boot-check.txt` |
| 06:38 | serve detached, pid 26173, `S_OS_PORT=6070`, `SEMIO_VITE_HMR=0` → `HTTP 200`, `VITE v7.3.6 ready in 18574 ms` | `🗑️generated/s2-serve.txt` |
| 06:40 | first browser probe: **shell refused to boot** — `Framework OS boot failed Error: configured host app has no panel leaf`, beacon `error:s` | `🗑️generated/s2-foreign-kind-probe.txt` (first run) |
| 06:45 | root fix in `🏛️ShellHost/🟦️.tsx` (§3.1), re-probe: **`beacon: ready:s`, 60 registry rows, 60 loaded, 148 spawnable programs** | `🗑️generated/s2-foreign-kind-probe.txt` |

**Four attempts, and neither published blocker was the real one.** G9 §1.5 named `🧱️block`; block built
and staged cleanly at 02:36 inside this very run. What actually stopped a cold `dev s` was, in order:
the nx daemon's `HASH_TASKS` timeout, a 4-hour shared-build-dir deadlock, and then — once the tree was
complete and served — a host-mode-only React boot throw that no single-plugin playground can reach (§3.1).

The serve also printed the freshness pass working as designed: 42 `[stale] … source-newer` lines
(peers edited plugin sources after those modules were staged) — reported, non-fatal, boot unaffected.

## 2. Per-plugin staged / missing table, and the healthy-set rule

### 2.1 The rule that was missing (fixed here)

Before this slice, preparation was **all-or-nothing**, and that is structurally wrong for `s`
specifically. `🪐️space` is the only plugin in the registry declaring `[package.metadata.semio].host`, and
that one flag fans the Nx closure out to the entire registered catalog (G9 §1.2). So
`♻️activation/🧰️preparation/🟦️.ts`'s loop — `readFileSync(join(directory, "🔣️.json"))` followed by
`throw new Error(\`Incomplete prepared component ${plugin.pluginId}\`)` — meant that **any one of ~60
crates failing made the whole hub un-bootable**, which is precisely how AU3 §5.5's attempt ended
(on `🧱️block`'s missing staged module). A single-plugin playground never hit this, because its closure
is one crate.

Landed: one pure rule, two callers.

- `♻️activation/🟦️.ts` (`//#region 🩺️HealthySet`, node-builtin-only import surface preserved):
  `PreparedComponentFacts` / `PreparedComponentVerdict` / `preparedComponentVerdict` (verdict precedence
  most-fundamental-first: `unstaged` → descriptor unreadable → descriptor names another plugin → no
  bridge → no `.nx-artifact.json`), `healthyPreparedComponents(verdicts, hostPluginId)`, and
  `preparedComponentReportLines` (same line shape as the existing `stagedModuleReportLines`, so one boot
  capture reads as one report).
- `♻️activation/🔍️freshness/🟦️.ts`: `stagedComponentFacts(moduleRoot, pluginId)` — the ONE disk reader
  both callers share, so "prepared" cannot mean two things.
- `♻️activation/🧰️preparation/🟦️.ts`: reports `[excluded] <id>: <kind> — <detail> — run: <cmd>` and
  continues; refuses **only** when the variant's own host component is the excluded one, or when nothing
  prepared at all.
- `♻️activation/🏃️execution/🟦️.ts`: the receipt is now built from `healthy.prepared`, not from every
  session plugin. An excluded component is **absent** from the receipt rather than carrying a digest of
  nothing — which is what makes the existing serve-start freshness pass
  (`reportServeStagedModuleFreshness`) report it `unactivated`, and the shell's own per-plugin install
  isolation (`loadPluginModuleResilient` → `null` → status `failed`) refuse it at open time, instead of
  the host pretending it is there.

The exclusion is explicit, not a swallowed error: every excluded component is named with its verdict and
the command that brings it back, and two facts still refuse the boot (host component missing; nothing
prepared).

**Measured**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts`, new
`describe("healthy prepared set")` — 6 verdict cases + exclude-one-and-still-boot + host-missing-refuses +
nothing-prepared-refuses. `9 passed | 37 skipped` filtered, and `46 passed (46)` for the whole suite
(previously 37), i.e. no regression in the sibling freshness contract. Command:
`bun node_modules/vitest/vitest.mjs run --config 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts`.
All four touched runtime modules were additionally proven to import cleanly under `bun`.

### 2.2 The per-plugin receipt (mid-run snapshot, 02:40)

Measured by running the shipped rule over the canonical staging root
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules`,
which is what `pluginModulesRoot("dev")` resolves to — **not** the `🧑‍💻dev/🔌️plugin-modules` folder of
the same name, which is a second, stale tree and is what an eyeball `ls` lands on first). Capture:
`🗑️generated/s2-staged-table.txt`.

**59 of 60 session components staged; 1 excluded; `refusal=none`** — i.e. the healthy set is non-empty and
the host component `space` is in it, so this tree boots.

Staged (59): animate, architect, **block**, cad, cad-extension-aec-building,
cad-extension-aec-building-energy, cad-extension-aec-building-structure, cad-extension-spatial-shape, dag,
demonstrator, draw, energy, fem, flow, flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text},
forms, gis, imperative, imperative-extension-{control,effect,logic,math,text}, layout, lowpoly, mathematical,
norm, note, playbook, playbook-module-procedural, procedural, process,
process-extension-{concrete,metal,robotic,wood}, puzzle, raster, reasoning, remodel, sequence, shooting,
sourcing, sourcing-module-{beams,slabs,windows}, **space**, trinity, vcs, wfc, writer.

Missing (1): **`stdio`** — `no staged module directory`. Its `materialize-dev` had not run yet at the time
of the snapshot (its `component-dev` cargo was observed live, pid 40316). `stdio` is also the crate DS1 is
separately fixing for the >4 MiB descriptor bound, and S1 classifies it as an I/O codec library with no UI
app and no openable artifact kind — so it is the exact shape the healthy-set rule exists for.

**This is the concrete, measured payoff of §2.1.** On the pre-slice code, this same tree would have died at
`prepare` with `Incomplete prepared component stdio` (in fact with an unhandled `ENOENT` from
`readFileSync(…/🔣️.json)`, since the directory is absent entirely) — one non-UI codec library taking the
whole 60-plugin hub down, which is the same class of failure AU3 §5.5 hit on `🧱️block`. `block` itself is
no longer that failure: it staged at 02:36 **inside this run**.

Two honest caveats: (a) this is a snapshot taken while the activation was still running, so `stdio` may yet
stage before the run ends — the final receipt is what `cold-boot-check-s` prints; (b) "staged" here means
the module directory holds a descriptor naming the right plugin, a bridge and an Nx artifact marker — it
does not mean the plugin opens, which is §3's question.

## 3. Live foreign-kind open probe inside `s`

Probe: `🐍️s2-s-host-foreign-kind-probe.mjs` (new, this ticket folder) — headless chromium,
`--use-angle=metal`, 1440×900, driving the served `s` at `http://127.0.0.1:6070/`. It reuses the
selectors `🧪️tests/🔬️catalog-smoke/🟦️.ts` already proves against this shell (readiness beacon dataset,
command-palette `spawn.<pluginId>` item, `window.__semioOsCatalogProbe`) and adds only the mutate/undo
half. Capture: `🗑️generated/s2-foreign-kind-probe.txt`.

> 🔗️ C1c, 2026-09-20: the `s.home.session-identity-required` this probe stops on is now observed
> **clearing on sign-in** inside `s` — two humans, hub-wired serve of this same staged output on port
> 6071 (pid 33969; 6070 has no `S_HUB_URL`), probe `🐍️c1c-s-host-probe.mjs`, capture
> `🗑️generated/c1c-s-host.txt`. `s-home-main` is still not published afterwards because `refreshUi`
> does not restart the already-stopped actor; the remaining half and its recommended shape are in
> `📓️c1-collaboration-e2e.md` → `### E2E run 5 (session 5b)`.

### 3.1 The host-mode-only boot throw (found live, fixed)

First run: beacon `error:s`, `Framework OS boot failed Error: configured host app has no panel leaf`
(`requiredHostPanelLeafId`, `🏛️ShellHost/🟦️.tsx:715-718`).

Root cause, traced: `establishPrimarySession` (`🏛️ShellHost/🟦️.tsx:3675+`) resolved the LANDING app from
the manifest it is handed — `resolveRequiredHostApps(manifest.apps, hostConfig).landing` — but read the
HOST app from the render closure (`requiredHostPanelLeafId(hostApp)`). `hostApp` derives from
`hostAppsResolution` → `hostPlugin` → `loadedPlugins`, and this callback is invoked by `installPlugin`
immediately after its own `UPSERT_LOADED_PLUGIN` dispatch, so `loadedPlugins` is still the pre-commit
array and `hostApp` is `undefined` → throw → whole boot dead. This is the exact React-commit hazard the
neighbouring `PluginInstallOutcome` doc comment already warns about ("must not infer success from
`loadedPluginsRef`, which only updates after the next React commit").

**Why this was invisible until now**: the branch is guarded by `if (hostConfig)`. A single-plugin
playground has `hostConfig === null` and takes the branch below it, so every prior live proof on this
ticket (AU3's two-browser sign-in, the activation-owner runs, all in `animate`) ran the other path. It
was reachable only from a host-mode shell — i.e. only from `dev s`, which nobody had ever booted.

Fix (one resolution answers both, no closure dependency, `hostApp` dropped from the dep array):
```
const hostApps = resolveRequiredHostApps(manifest.apps, hostConfig);
const sApp = hostApps.landing;
const panelState = buildSpacePanelState([], requiredHostPanelLeafId(hostApps.host));
```
`handle` IS the host plugin on this branch — the `.landing` resolution on the same manifest already
depends on that — so this is strictly more correct, not a workaround.

**Measured after the fix**: `beacon: ready:s`, `shell s: 60 registry rows, 60 already loaded, 148
spawnable programs`. All 60 components report install status `loaded` in the shell's own probe. This is
the first observed `dev s` boot to a ready shell.

### 3.2 What the foreign-kind open did NOT get to prove, and why

All three targets (`draw`, `note`, `layout`) came back `statusAfterOpen: "loaded"`,
`loadedBeforeOpen: true`, `lazyInstalled: false`, `windowId: null`,
`openDetail: "command palette never opened"`. Two facts, honestly separated:

1. **Lazy install could not be exercised as "lazy".** In host mode the shell installs the whole catalog
   at boot (all 60 are `loaded` before any artifact is opened), so `openArtifactWithAppRef`'s
   install-on-open path never had a not-yet-installed plugin to install. The probe's `lazyInstalled`
   column is therefore `false` by construction here, not by failure. Proving the lazy path needs an
   artifact kind deliberately excluded from the boot set — not attempted.
2. **No window opened, because the host app publishes no surface.** The blocking fault is the shell's
   own, captured verbatim:
   `[DEBUG] PluginRuntime: actor space#1 stopped without publishing requested UI surfaces (missing=["1:s-home-main"], status=idle, faults=["plugin.internal: s.home.session-identity-required: current host session identity is required"])`.
   The `🪐️space` `home` guest **refuses to render its main surface until a host session identity
   exists** — i.e. until someone is signed in to a hub. With no window body, the command palette (the
   shell chrome that spawns a program) never opens, so no foreign-kind artifact could be opened,
   mutated or undone.

So **one mutation and one undo in three foreign plugins is NOT proven** by this slice. The gate is
identity — see §3.3, where it was chased signed-in against C1c's hub-backed serve and turned out to sit
one level deeper than anyone had it.

### 3.3 Signed-in run on 6071, and why C1c's diagnosis is necessary but not the operative blocker

Follow-up asked by the coordinator: verify C1c's diagnosis (E2E run 5 check 2 — `s-home-main` never
published even after the identity fault clears, because `refreshUi` talks to an actor that already
stopped), fix it at the root, judge with `🐍️c1c-s-host-probe.mjs` unchanged.

**The fix asked for was made** (`🏛️ShellHost/🟦️.tsx`, §8): the session key is split into a triple key
(`pluginId:appId:instanceId`) and a human key (`userId:displayName`). The triple key keeps the existing
`refreshUi`; the human key gets a new effect, placed after `retireSessionInstance`, that runs
`establishPrimaryWithShardRetry(handle)` and then retires the superseded instance — the same recovery
the killed-boot and worker-loss paths already use, on the grounds C1c gives: every surface assembled
under the old identity is invalid, and a stopped actor cannot be refreshed back to life.

**Judged with C1c's probe unchanged** (capture `🗑️generated/c1c-s-host.txt`, re-run against their serve
on 6071, hub 7501): checks 1, 2-signed-in, 2-fault-cleared, 2-command-palette **PASS**; check 2 surface
row still **FAIL** — `s-home-main` absent. No regression, no improvement.

**Why, measured rather than guessed.** A temporary one-line instrumentation of the new effect (added,
observed, removed) printed exactly twice per run — both times at mount:
`previous=null next=":" session=none`. It **never ran a third time**. The effect's only changing
dependency is the human key, so this says plainly: **`identity` never changes in this shell after
sign-in**, and therefore neither C1c's `refreshUi` nor this slice's re-establish is ever reached. The
recovery is not failing; it is not being invoked.

That relocates the root cause upstream of both:

- Sign-in genuinely succeeds — the badge clears (`signInOffered: 0`), which is driven by
  `verifiedSessionAuthority !== null`, so `onAuthority` (`🏛️ShellHost/🟦️.tsx:3514+`) did fire, and on a
  first authority `authorityChanged` is true so the `if (unchanged && !authorityChanged) return;` guard
  cannot be what skips `setIdentity(resolved)`.
- Yet no render observes a new `identity`. The boot-time refusal is an **uncaught** error — it arrives
  as a playwright `pageerror`, not a caught console error:
  `[DEBUG] PluginRuntime: actor space#1 stopped without publishing requested UI surfaces (missing=["1:s-home-main"], …)`.
  The two mount-time log lines (a mount, then a remount) with no third line are the signature of a
  subtree that stopped applying updates after that throw.

So the sequence is: Home refuses without an identity → the refusal escapes as an uncaught error →
the shell subtree stops updating → the later, genuinely successful sign-in can no longer reach any
re-assembly path, whichever one is wired. **The next owner's target is the uncaught refusal**, not the
refresh-vs-establish choice: either `🪐️space`'s Home must degrade to an empty-but-live surface when
`sessionIdentity` is absent (preferred — it is a landing page, and a hub-configured shell is expected to
boot signed-out), or `PluginRuntime`'s "stopped without publishing" must be raised as a caught,
recoverable window fault instead of an unhandled throw.

The split-key re-establish is kept: it is correct, it is proven wired and inert until an identity
change arrives, and it costs nothing — but this slice makes **no claim that it fixes check 2**, because
the code path it lives on is never entered today.

Incidental, also measured: hub 7501 answers `/healthz` 200 and `/readyz` **503**, with every subsystem
`ready: true` except `artifactAuthority` — DS1's lane. Sign-in and the directory are unaffected by it.

### 3.4 Both halves of the operative blocker, fixed

**(A) HOST — a guest refusal must not escape as an unhandled rejection.** Traced with a stack capture
(`S2_STACKS=1` on the probe): the throw is `settlePluginTurn`
(`🔌️PluginRuntime/🟦️.tsx:1884`), and it escaped because **six** `void refreshUi(…)` call sites in
`🏛️ShellHost/🟦️.tsx` had no `.catch` at all (7848, 7871, 7958 ×2, 10219, 10243). An unhandled rejection
out of a React subtree stops that subtree applying updates — one guest declining to render took the
whole shell with it, silently.

Fix: one funnel, `reportRefreshFault(pluginId, error)` (declared beside `pluginSupervisorByIdRef`, deps
`[]`), which turns any refresh rejection into the shell's existing window fault via `windowFaultFromError`
+ `SET_ERROR`. All six sites now route through it. **Measured**: the `pageerror` is gone from every
subsequent probe capture, and the shell keeps running and re-rendering after the refusal.

**(B) GUEST — signed out is a state, not a fault.** `🪐️space` Home refused in *three* places, not one:
the editor's explore main window, the viewer's view main window, and — the one that actually gated the
surface — `HomeApp::render` itself
(`🗿️artifacts/🏠️home/…/✏️editor/🦀️.rs`), which bailed before ever reaching the window render. All three
now answer an empty space table when `home_session_identity` is absent (a signed-out human owns no
spaces; the table kit already states its own empty case in en+de). The command-side
`require_session_identity` in `handle` is deliberately **kept**: a signed-out human genuinely cannot
create a studio — rendering is not gated on identity, mutating is.

Rebuilt with the narrowest verb, one cargo, no 60-plugin activation:
`… ⚡️caching/🦀️cargo/📜️script.ts native component dev --manifest ✏️s/🔌️plugins/🪐️space/…/Cargo.toml`
→ `nx run @semio-tech/space-plugin:materialize-dev` → `activate s react dev`
(`60 completed components`, receipt re-checked `60 of 60, 0 excluded`).

**Cargo deadlock at 08:30** (rule 23(a)): the second space build sat 20 min with no `rustc` child while
11 rustc ran elsewhere; `sample 8174 1` showed 3 `prebuild_lock`/`flock` frames. Killed by pid and rerun
once — the rerun finished in `16m 07s`. Recorded here as the rule requires.

**Judged with `🐍️c1c-s-host-probe.mjs` unchanged** (capture `🗑️generated/c1c-s-host.txt`):

| check | before this section | after |
|---|---|---|
| 1 hub badge signed out | PASS | PASS |
| 1 "Home refuses its surface without a signed-in human" | PASS | **FAIL — and this is the fix landing** |
| 2 signed in inside `s` | PASS | PASS |
| 2 no `session-identity-required` | PASS | PASS (now also true *before* sign-in) |
| 2 host app surface published | FAIL | **FAIL (unchanged)** |
| 2 command palette tab | PASS | PASS |

Check 1's inversion is the honest signal, not a regression: that check asserts the defect
(`identityFault: true` expected before sign-in) and now reads `identityFault: false` on first paint,
signed out. **C1c's probe needs its check 1 updated to assert the new contract** — Home publishes signed
out — which is C1c's file to change, not this slice's (the coordinator required it be run unchanged).

**Still open, and stated plainly**: `s-home-main` / `.semio-table-host` is *still* absent
(`homeSurface: false`, `tableHosts: 0`) both before and after sign-in. So the refusal is gone, the
uncaught rejection is gone, the tree stays live — and the surface still does not appear. Something
downstream of `HomeApp::render` is not publishing the body; this slice ran out of budget before tracing
it, and did not guess. That is the single remaining item for outcome 1 inside `s`, and it is now a
narrow, well-isolated question rather than three interacting ones.

## 4. Hub workspace and `/hub` inside `s`

**The premise this slice was handed is wrong**: `HubWorkspace` and `/hub` are not "host-mode-gated" away
from `s` — `s` **is** the host mode. Verified in source and then live:

- `HubWorkspace` mounts on plain `useState` (`🏛️ShellHost/🟦️.tsx:2613`), opened by `openHubWorkspace()`
  with no `hostMode` condition; the doc comment at `:9052-9056` says so explicitly ("Signing in to a hub
  is shell chrome, not a host-mode privilege").
- Only the `/hub` **URL** is host-mode-only: `openHubWorkspace` calls `navigateHistory("/hub")` behind
  `if (hostMode)` (`:9059`), and the `applyShellUri` effect that owns the route is guarded by
  `if (!hostMode || loadedPlugins.length === 0) return;` (`:6230`). The route branch itself is
  `if (path === "/hub") { setHubWorkspaceOpen(true); return; }` (`:6148`).
- So the `/hub` address works **only** inside a host-mode shell, and `s` is the only one. G9 §3 read this
  correctly; G10/the slice brief inverted it.

**Measured live inside `s`** (same probe, same session, after the §3.1 fix):
`{"route":"/hub","signInVisible":true,"badgeVisible":true,"workspaceMounted":false,"spacesVisible":false}`,
body text `"… Remote: detached  No one else is here  signed out  Sign in  Settings  Marketplace  Tasks  Command"`.

Read honestly: navigating to `/hub` inside the real `s` host **is** accepted (the URL stays `/hub`, it is
not bounced), the hub connection badge is mounted, the shell reports `signed out` and offers `Sign in`.
That is the first time any hub surface has been observed inside `s` rather than in the `animate`
playground. What is **not** proven: the `HubWorkspace` overlay's own body (spaces list, sign-in form
fields) — the probe's `spacesVisible` is false and the beacon after the `/hub` navigation was
`not-found:s`, because a full page navigation to `/hub` re-boots the shell and the same
`s.home.session-identity-required` refusal from §3.2 leaves it without a rendered host app to overlay.
Reaching the workspace body needs either the in-page badge click (no reload) or a signed-in session. No
hub was up for this slice to sign in against (C1c keeps one; not running at 06:45), so **sign-in →
spaces inside `s` is unverified**.

## 5. Activation reason through the React host

**G10 N1 is stale twice over, and the premise handed to this slice is also wrong.** The brief said "there
is no `registry.activate` call in the React DOM host any more; only `🐚️plugin-bridge/🟦️.ts` has them".
Measured (grep, current tree) — the React DOM host has **two** of them, in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`:

| site | line | reason passed |
|---|---|---|
| `ensureRequestActor` (the handle's inbound-request actor) | `:2634` | `"manual" satisfies ActivationReason` |
| `handle.createApp` (the artifact-open path) | `:3048` | `activationReasonForAppId(appId)` |

Both readings that missed them were the same measurement error, worth recording because it will bite the
next slice: **`grep` classifies that file as binary and silently reports no match unless given `-a`.**
`grep -c createApp <file>` prints nothing and exits 1; `grep -ac createApp <file>` answers 23. Every
"there is no call any more" claim about `🔌️PluginRuntime/🟦️.tsx` in G10 and O2 traces back to this.

So the React DOM host already sends the real reason, and the earlier one-line fix (O2 §5 gap 1,
`"manual"` → `activationReasonForAppId(appId)`) has already landed. The remaining, still-open half is the
one O2 also named and nobody has closed: **the guest drops it**.
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:843` still matches
`Event::Activate { .. } | Event::SuspendRequest | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}`
— the event is decoded (`⚛️reactor/🦀️.rs:1479` builds `Event::Activate { reason: wit_activation_to_kernel(payload.reason) }`)
and then discarded. A repo-wide grep for `activation_reason` / `last_activation` across
`🔨️modules/🔌️plugin` returns **one** hit, a doc comment — there is nowhere in the guest that stores or
answers "why was I activated".

Chain status, end to end:

| leg | state | evidence |
|---|---|---|
| app id → reason | real | `🎠️kernel/🟦️.ts:1279` `activationReasonForAppId` |
| reason → WIT envelope | real | `🎠️kernel/🟦️.ts:2150` `activationEventEnvelope`; `manual` correctly answers `undefined` (no WIT variant) |
| React DOM host → registry | real | `🔌️PluginRuntime/🟦️.tsx:3048` |
| wgpu browser bridge → registry | real | `🐚️plugin-bridge/🟦️.ts:1646` |
| registry → guest turn | real | `🎠️kernel/🟦️.ts:2427` `activate()` enqueues the envelope on `Maintenance` |
| guest consumes it | **dead end** | `⚛️reactor/🔄️turn/🦀️.rs:843` matches to `{}` |

This slice did **not** close the guest leg. It is a `⚛️reactor` change that rebuilds every one of the 60
plugin wasms, and this slice was already holding the machine for the cold activation those same 60 crates
feed; landing it concurrently would have invalidated the run in flight. Scoped for a successor: store the
decoded reason per actor in the reactor and expose it through `plugin_runtime`, so an app can branch on
`on-artifact-kind:<kind>` in its first turn. Nothing downstream asks for it yet, so it is honest
plumbing-before-consumer either way.

## 6. Permanent wiring (nx target + launch row)

The live half of the proving command **already existed and nobody had wired it to the `s` activation**:
`@semio-tech/framework-os-dev:catalog-smoke` (`📋️project.json:275`) runs `verify catalog`
(`🧪️tests/🔬️catalog-smoke/🟦️.ts`), which drives a served `s` page, reads the shell's own
`window.__semioOsCatalogProbe` (`🏛️ShellHost/🟦️.tsx:728`) and spawns every program the shell offers,
reporting per-plugin install status. It assumes a server is already up at `S_OS_PORT` and has no
launch.json row. It is not duplicated here.

What was missing is the **offline half**: after a multi-hour activation, "what did that activation
actually produce, per plugin" had no command at all. Added:

- `bun ./📜️script.ts cold-boot-check <variant> react|wgpu <dev|release>` — `ColdBootCheckScript`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts:146+`, registered in
  the dev router at `📦️packages/🟦️typescript/📜️script.ts`. Prints one `[staged]`/`[missing]` line per
  component the variant's playground session names, cross-checked against the activation receipt, and
  refuses on three distinct facts: no receipt at all, the host's own component missing, or the receipt
  being behind the staged tree.
- nx target `cold-boot-check-s` (`📋️project.json`), `dependsOn: ["activate-s-react-dev"]` — one command
  that performs the cold boot and then states its per-plugin receipt.
- `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` row `🛠️dev🪐️os-s🩺️cold-boot`, group `3_dev`,
  order `387.065` (next to `🛠️dev🤝️os-collab-e2e` at `387.064`).

Measured: run against the tree before the activation finished, the verb refuses correctly —
`error: No activation receipt at …/dist/runtime/react/dev/s/activation — run: bun nx run @semio-tech/framework-os-dev:activate-s-react-dev`.

## 7. Measured vs unverified, honest gaps

**Measured at runtime by this slice** (capture named for each):

- 60 of 60 components staged and activated, 0 excluded — `s2-cold-boot-check.txt`, `s2-staged-table.txt`
- `Activated s react dev: 60 completed components (changed)` — `s2-prepare-activate.txt`
- `s` serving, `HTTP 200`, Vite ready in 18.6 s — `s2-serve.txt` (pid 26173, port 6070)
- shell reaching `ready:s` with 60 registry rows, 60 loaded, 148 spawnable programs — `s2-foreign-kind-probe.txt`
- the `configured host app has no panel leaf` boot throw, before and after the fix — same capture, two runs
- `s.home.session-identity-required` as the reason no window body renders — same capture
- `/hub` accepted inside `s`, badge mounted, shell `signed out` + `Sign in` offered — same capture
- healthy-set rule: 9 new tests green, suite 37 → 46 — vitest run in §2.1

**Unverified / not done**:

- **One mutation + one undo in three foreign plugins** — still blocked, now chased to its real cause
  signed-in against a live hub (§3.3): the boot-time identity refusal escapes as an **uncaught** error
  and the shell subtree stops applying updates, so the later successful sign-in never reaches any
  re-assembly path. Measured by instrumenting the new effect: it runs only at mount, never again.
- **The split-key re-establish (§3.3) is landed but unprovable today** — correct and wired, but its code
  path is not entered, so it does **not** move C1c's check 2, which still FAILs on `s-home-main`.
- **Lazy cross-plugin install as *lazy*** — host mode installs all 60 at boot, so the install-on-open
  path had nothing to install (§3.2 point 1). Needs a deliberately-excluded kind to exercise.
- **Sign-in → spaces inside `s`** — no hub was running at 06:45; `HubWorkspace`'s own body never
  rendered (§4).
- **The guest activation-reason leg** — `⚛️reactor/🔄️turn/🦀️.rs:843` still discards `Event::Activate`.
  Deliberately not touched: it rebuilds all 60 plugin wasms and would have invalidated the activation
  this slice was holding the machine for (§5).
- **`animate`'s `component-dev` is red in Nx** from the deadlock kill (`Cargo artifact build failed`
  after 4.3 h in `flock`). Its previously staged module (02:31) is what the 60/60 receipt counts, so the
  served shell has a working `animate` — but a fresh `nx run activate-s-react-dev` will rebuild it, and
  that build has not been re-run to green by this slice.
- The `cold-boot-check-s` nx target was **not** executed end to end (its `dependsOn` is the same
  `activate-s-react-dev` whose nx wrapper spent 4 h at task 11); the verb it runs was executed directly
  and is green.
- `stdio` was `missing` at the 02:40 snapshot and `staged` by 06:33 — the healthy-set exclusion path was
  therefore proven by unit test and by that snapshot, **not** by a final activation that actually
  excluded something. No component was excluded in the receipt that booted.

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts` | new `//#region 🩺️HealthySet`: `PreparedComponentFacts`, `PreparedComponentVerdict`, `preparedComponentVerdict`, `healthyPreparedComponents`, `preparedComponentReportLines` |
| `…/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts` | new `stagedComponentFacts` — the one disk reader both callers share |
| `…/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts` | all-or-nothing `Incomplete prepared component` throw replaced by the healthy-set report + host-only refusal; summary line now states prepared/total/excluded |
| `…/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts` | receipt built from `healthy.prepared`, excluded components named and left out of the receipt |
| `…/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts` | new `ColdBootCheckScript` (`//#region 🩺️ColdBootReceipt`) + its imports |
| `…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | registered the `cold-boot-check` verb |
| `…/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | new nx target `cold-boot-check-s`, `dependsOn: ["activate-s-react-dev"]` |
| `…/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts` | new `describe("healthy prepared set")`, 9 cases; suite 37 → 46 passing |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | row `🛠️dev🪐️os-s🩺️cold-boot`, group `3_dev`, order `387.065` |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `establishPrimarySession`: host app resolved from the manifest in hand instead of the uncommitted `hostApp` closure (§3.1); `hostApp` dropped from the dep array. **This is the fix that made `dev s` boot.** |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | session key split into a triple key (keeps `refreshUi`) and a human key (new effect: `establishPrimaryWithShardRetry` + retire the superseded instance), §3.3; new `reportRefreshFault` funnel + `.catch` on all six previously-uncaught `void refreshUi(…)` sites, §3.4 (A) |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/…/✏️editor/🦀️.rs` | `HomeApp::render` no longer gated on `home_session_identity` (the command-side gate in `handle` kept), §3.4 (B) |
| `…/🏠️home/…/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs` | absent identity → empty space rows instead of `s.home.session-identity-required` |
| `…/🏠️home/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs` | same, viewer twin |
| `🐍️s2-s-host-foreign-kind-probe.mjs` (ticket folder) | new permanent probe: boot beacon, catalog probe, foreign-kind spawn, mutation+undo, `/hub` step, optional `S2_SIGN_IN_EMAIL`/`S2_SIGN_IN_PASSWORD` sign-in; `error:s` no longer treated as a dead shell |

Rust touched only in `🪐️space` (§3.4 B), rebuilt with one cargo and restaged into the existing `s`
output. The `⚛️reactor` guest activation-reason leg is still deliberately untouched (§5).

## 9. Live state handed on

- `s` react dev is **serving on http://127.0.0.1:6070/**, pid **26173**, detached (`nohup … & disown`),
  `SEMIO_VITE_HMR=0`, log `🗑️generated/s2-serve.txt`. C1c shares this staged output — do not re-activate
  under it; the Vite server picks up `ShellHost` edits on reload.
- Activation receipt: `…/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/s/activation`, 60 plugins.
- Re-run the per-plugin receipt any time with
  `bun ./📜️script.ts cold-boot-check s react dev` (cwd `…/🧑‍💻dev/📦️packages/🟦️typescript`).
- Re-run the live probe with
  `bun 🐍️s2-s-host-foreign-kind-probe.mjs http://127.0.0.1:6070/ draw note layout`.
- Next owner's first move: get a host session identity into the `s` shell (C1c's sign-in), then re-run
  the probe — the mutation/undo columns should fill without any further code change.
