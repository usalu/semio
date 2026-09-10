# Contributions Re-Arm — Why A Late Registry Install Left The Evaluation Faulted

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "contributions re-arm". Session ⚪9f5f6952
(Fable 5.1, Opus 5). Repo MCP was down all session (`invalid initialize params`); ticket bookkeeping
is on disk and no ticket was opened, closed or reopened.

Answers the LIVE FINDING in `📓️runtime-verification-2026-09-09.md` boot #11: the app booted with
both windows and no traps, yet 2.5 minutes later the preview still read
`phase: faulted — Geometry extension unavailable (flow.extension-not-contributed, brep)` and the
flow status still read `extrusion-axis: computing`, with no user action able to change it.

**Hypothesis (a) held. Root cause, fix, laws and gates below. Restage required: yes.**

---
## 1. TL;DR — which hypothesis held

**(a) held; (b) held as its consequence; (c) is refuted.**

The push DOES run and the pages DO install. What never happened is anything AFTER the install:

> `sync_host_flow_extension_contributions_page`'s last page rebuilt the process-wide registry and
> **published nothing**. The retained `FlowEvalSession` still held the miss it had already cached —
> in its neural cache, in its incremental baseline (`previous_snapshot`/`previous_channels`), in the
> published `eval_json`/`status_json`, and in the tessellation ledger the preview projects as
> `phase: "faulted"` — and its `flowEvalTick` chain had already given up. A registry install is not
> a store lane, so no refresh followed, so `pending_effects` was never asked again, so nothing ever
> looked at the new registry. The surface therefore reads `faulted` **forever**, exactly as boot #11
> observed for 2.5 minutes.

Hypothesis (c) is refuted on evidence, not on argument: `pageCount` **is** declared for
`setContributions` on all three receiving apps in BOTH the tracked descriptor
(`✏️s/🔌️plugins/🌀️procedural/🔣️.json`) and the dev-served copy
(`…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/🔣️.json`, rebuilt
09:41), and `resolvedTargetViewState` (`🏛️ShellHost/🟦️.tsx:3925`) carries the full
`windowInstances` roster into the push, so the gating the live boot meets is correct. A law now pins
that, and a diagnostics line now makes the decision observable instead of inferable.

## 2. Root cause

| | |
|---|---|
| **file:line** | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` — the handler ended at `sync_host_flow_extension_contributions_page(…)?; Ok(Emit::default())`, its `_session` parameter deliberately unused ("touches nothing in it") |
| **why it looked right** | `Generation3dContributionsWork`'s own docstring (`…/✏️editor/🦀️.rs:797`) states the design: "Runs against the app instance's RETAINED session for symmetry with every other retained route, but touches nothing in it: the registry the page assembles into is process-wide". True of the REGISTRY; false of the SESSION, which caches results computed against that registry. |
| **why nothing else caught it** | `Generation3dPlayApp::pending_effects` re-arms whenever the fixture has pending nodes — but it is only reached through `refreshUi`, and `ShellHost.refreshUi` runs `buildUiRefreshRequest` first and skips the whole call when nothing changed. A `HostOnly` install changes nothing. |
| **the native law that passed anyway** | `host_pushed_contribution_pages_install_the_registry_the_served_chain_needs` installs the pages BEFORE it drives any evaluation, so its session had nothing stale to invalidate. The served shell runs the opposite order. |

## 3. The reproduction, in the served order

`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` —
`a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`:

```
boot ── setActiveExample(hexagonal-mushroom-column) ── drain the chain ── PUSH 31 PAGES ── drain
        ▲ registry empty                                ▲ faulted            ▲ nothing else runs
```

The hard part of reproducing it natively is that a `--lib` binary **links** both extension packs
(`🔬️flow-operators`), and `build_flow_extension_registry` applies every linked installer before any
contributed manifest, while `register_contributed_manifest` skips an operator the registry already
carries. So emptying the contribution table alone still evaluates the whole graph — the first
attempt at this law read `meshes = 3` where it expected `0`. `UnlinkedFlowExtensions` (an RAII guard
in the law's own file) therefore retires both linked installers for the length of the law through
the new `semio_framework_os_flow::unregister_linked_flow_extension_installer`, and puts them back —
panic or not — with two replacements, because the installer table is only read while a registry is
BUILT and re-pushing an unchanged closure would rebuild nothing.

That reaches the served guest's exact shape: nothing linked, everything contributed. The measured
before/after:

```
[STATS] before the install: {"height":{"status":"ok"},"radius":{"status":"ok"},"sides":{"status":"ok"},
  "profile":{"status":"error","message":"unknown kind: brep.curve.polygon"},
  "extrusion-axis":{"status":"error","message":"unknown kind: math.vector"},
  "extrude":{"status":"error","message":"unknown kind: brep.solid.extrude"},
  "column-preview":{"status":"ok"}}          preview phase=faulted meshes=0
[STATS] late install: pages=31 page-bound=4096
[STATS] late install re-armed 5 ticks and painted meshes=3     profile/extrusion-axis/extrude/column-preview = ok
```

**The law is discriminating, measured, not argued.** With the invalidation and re-arm removed from
`set_contributions::apply` and nothing else changed, the same law fails at exactly the right line:

```
assertion `left == right` failed: the run's last page owes exactly one re-arm per attached preview window
```

Note what the law does NOT do: after the pages land it never calls `pending_effects`, never
dispatches a tick by hand, and never renders before it asserts. `drain_armed_flow_eval_ticks_from`
is started from the install receipt's OWN effects, so "the eval re-runs without any user action" is
the thing being measured.

## 4. The fix

### 4.1 The invalidation key is the registry generation (framework)

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`

| what | where |
|---|---|
| `FlowEvalSessionState.flow_extension_generation` | the replacement of the process-wide registry every result the session still holds was computed against; seeded at `FlowEvalSession::new()` from `flow_extension_registry_generation()`, so a fresh session is current by construction |
| `FlowEvalSession::flow_extension_generation()` | readable, so a surface can state the key instead of inferring it |
| `FlowEvalSession::invalidate_for_flow_extension_registry(generation) -> bool` | no-op and `false` when the generation is unmoved; otherwise sweeps the neural cache (`begin_epoch` + `sweep`, evicting through the cache's OWN retirement) and runs the existing `set_eval_json(String::new())` ladder — which is already the one place that retires the incremental baseline, the published `eval_json`/`status_json`, every preview mesh, every pending/partial tessellation and its progress row, and clears `tick_scheduled` so a new chain may be armed |

Using `set_eval_json`'s ladder rather than a second one is the point: a session's retirement
discipline is not something to re-derive per call site.

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` gains
`unregister_linked_flow_extension_installer`, the missing inverse of
`register_linked_flow_extension_installer` — registration was one-way, and a process that LINKS an
extension pack had no way to reach the state of one that only receives it as a host contribution.

### 4.2 The route re-arms what it invalidated (plugin)

`…/🧊️generation3d/…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` splits the same way
`delete_selection` does, and for the same reason — `app_commands!`'s per-row
`handle(payload, doc, cfg, ctx)` signature is framework-fixed and carries no attached-window roster:

- `apply(payload, doc, cfg, session, preview_window_ids)` — installs the page; asks
  `flow_extension_registry_generation()`; if `invalidate_for_flow_extension_registry` answers
  `false` it emits nothing; otherwise it emits one `flow_eval_tick::rearm(window_id, 105)` per
  attached preview window, through the SAME addressed self-dispatch every other hop of the chain
  uses.
- `handle(payload, doc, cfg, session)` — the macro row, `apply(…, &[])`.

`Generation3dContributionsWork::step` reads the roster off `input.context.view_state` — the shell's
own trusted `ViewModel`, the same one `pending_effects` arms the first chain from — via the existing
`generation3d_preview_window_ids`. `resolvedTargetViewState` already carries the full
`windowInstances` roster into `handleCommand`, so this is live data, not a test-only path.

`…/🌀️generation2d/…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` is the twin, minus the window
addressing: that editor's tick publishes on the app-wide `HostOnly` lane, so its re-arm names no
window. Its `flow-eval-tick` gains a `rearm(req)` helper, and the two places that previously spelled
that effect literal (the tick's own re-arm and `pending_effects`) now read it from one declaration.

**The generation3d viewer deliberately gets nothing.** `Generation3dViewCommandWork::step` builds a
FRESH `FlowEvalSession` per command and retires it in the same work — it retains no results across
turns and arms no chain, so it has nothing to invalidate and nothing to re-arm.

### 4.3 Observability and the descriptor law (hypothesis (c))

`🏛️ShellHost/🟦️.tsx`'s push loop had four independent gates and a silent `continue` on each, so a
boot where the closure never reached the guest was indistinguishable from one where it did. Behind
`runtimeDiagnosticsEnabled()` (the existing `SEMIO_RUNTIME_DIAGNOSTICS` switch, `1`/`true`/`on`/`yes`
via `VITE_…` or `localStorage`) it now prints, per plugin, at the decision point:

```
[DEBUG] contributions push {"plugin":"procedural","app":"s.procedural.generation3d@1/*#editor",
  "active":true,"takesPageRun":true,"pageCount":73,"chars":293642,"skipped":null}
```

`skipped` names which gate closed (`app-owns-no-setContributions` / `handle-cannot-command` /
`no-bound-instance`) or is `null` when the run actually goes out. A normal boot pays nothing.

`appOwnsCommand` / `appCommandTakesPageRun` are now exported so a law can run the REAL gate rather
than a copy of it, and `🧪️tests/🔬️engine-contract/🟦️.ts` gains
`describe("contributions push declaration")`: it reads the tracked descriptor
`✏️s/🔌️plugins/🌀️procedural/🔣️.json` off disk (1 MB — read, never imported into the test bundle),
asserts the set of apps that own `setContributions` is exactly
`{generation2d#editor, generation3d#editor, generation3d#viewer}`, and asserts
`appCommandTakesPageRun` answers `true` for each. The dev-served copy under
`…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/🔣️.json` is a
gitignored build artifact, so it is asserted only when a dev tree has one — which is precisely how a
STALE served copy (the failure mode hypothesis (c) named) is caught.

## 5. Tests

| # | gate | result |
|---|---|---|
| 1 | `cargo test -p semio-framework-os-flow --lib -- flow_eval_session_invalidates` | **1 passed / 0 failed** — the new framework law |
| 2 | `cargo test -p semio-framework-os-flow --lib` (full) | **79 passed / 127 failed** — the documented `final Dictionary ownership must be explicitly retired or owned by a cold boundary` gate (`🧠️neural/⚙️engine/🦀️.rs:101`), unchanged from the 78/127 baseline plus this lane's one new pass |
| 3 | `cargo test -p semio-s-artifact-procedural-generation3d … --lib -- contribu` | **7 passed / 0 failed** (the three `set_contributions` route laws, `contributions_route_declares_a_reachable_wire_ceiling`, `extension_invocations_address_…`, `host_pushed_contribution_pages_…`, and the new `a_late_contributions_install_re_arms_…`) |
| 4 | `… --lib -- re_pushing_an_unchanged` | **1 passed / 0 failed** — `[STATS] re-push generations=2 -> [3, 3] rearms=[1, 0]` |
| 5 | `… --lib -- a_late_contributions_install_re_arms` with the fix REMOVED | **FAILED** at `the run's last page owes exactly one re-arm per attached preview window` — the law discriminates |
| 6 | `… --lib` (full, `--test-threads=1`) | **333 passed / 5 failed** |
| 7 | the same, `--skip two_instances_converge_disjoint_widget_moves --skip vcs_artifact_app_non_empty_retained_maintenance_swap_…` | **334 passed / 2 failed** — see §5.1 |
| 8 | `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib` (full) | **229 passed / 2 failed** — the same two pre-existing rows |
| 9 | `cargo check -p semio-s-plugin-procedural --keep-going` (native) | **Finished, 0 errors** (2 m 08 s) |
| 10 | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going` | **Finished, 0 errors** (2 m 03 s) |
| 11 | `SEMIO_TEST_LEVEL=long bunx vitest run "🔬️engine-contract"` | **507 passed / 0 failed**, including the new `contributions push declaration` |
| 12 | `bunx tsc --noEmit` (react target) | 826 errors, none in this lane's code — an earlier draft of the push loop DID add one (`TS2722` at the `handleCommand` call, narrowing lost to an indirection) and it was fixed; the remaining ShellHost rows (`baseDispatchViewState.windowInstances`, `{action: string}` vs `{args?}`) are peers' in-flight edits in the same file |

All Rust commands from the repo root with a private `CARGO_TARGET_DIR` seeded from `target/debug`,
`RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`, `--keep-going`. Raw logs in `🗑️generated/rearm-*.txt`.

### 5.1 The five reds in the full generation3d run, and which are this lane's

**None.** Four are previously attributed (`📓️close-ladder-2026-09-10.md` §7.3 lists exactly these
four as "the four reds, none this lane's and all previously attributed"):

| test | owner |
|---|---|
| `two_instances_converge_disjoint_widget_moves` | `module.vcs … remote snapshot merge is fail-closed` |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | envelope-load lane |
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` | hot-path / JSON-512 |
| `refresh_pending_effects_arms_flow_eval_tick_chain` — `typed operation did not retire within 30 seconds` | hot-path |

The fifth, `host_pushed_contribution_pages_install_the_registry_the_served_chain_needs`, is a
**cascade victim, measured as such**: it passes alone, it passes with every other contributions law
in one process, and it passes in the full run once the two panicking VCS rows are skipped (gate 7) —
the same 4-slot publication-lease strand `📓️contributions-delivery-2026-09-10.md` §5 already
documents.

`refresh_pending_effects_arms_flow_eval_tick_chain` and `generation_preview_…` were additionally
checked against this lane's only always-on framework change (the `FlowEvalSession::new()` generation
seed): with that seed reverted to `0` and nothing else changed, **both still fail identically**, and
nothing else in this lane is reachable without a `setContributions` dispatch, which neither test
makes.

## 6. Restage

**Restage required: YES.** Every Rust change here lives in the guest — `semio-framework-os-flow`
(the session's invalidation key and the registry's new unregistrar) and `semio-s-plugin-procedural`
(both editors' `setContributions` routes) — so the served `procedural` component must be rebuilt
before a browser boot can observe any of it. The lane did NOT run `activate` on the shared target:
the poll-leak lane owns the next restage, and these changes ride the one after it.

The TypeScript changes (`🏛️ShellHost/🟦️.tsx`'s diagnostics line and the two exports) are picked up by
the dev server's own rebuild and need no restage.

### 6.1 What to look for on the next boot

With `SEMIO_RUNTIME_DIAGNOSTICS=1` (localStorage or `VITE_…`), boot the generation3d editor and
expect, in order:

```
[DEBUG] contributions push {"plugin":"procedural","app":"s.procedural.generation3d@1/*#editor","active":true,
  "takesPageRun":true,"pageCount":73,"chars":293642,"skipped":null}
```

then the preview leaving `phase: faulted` and the flow status leaving `extrusion-axis: computing`
**without any user action**. If the `[DEBUG]` line never appears, the push loop is not reached at all
and `skipped` will say why on whichever plugins it does reach; if it appears with
`"skipped":"no-bound-instance"`, the contributor instance is not bound yet and the guard that
swallowed that silently is now named.

## 7. What is still not delivered

- **The generation3d viewer is untouched on purpose.** `Generation3dViewCommandWork::step` builds a
  fresh `FlowEvalSession` per command and retires it in the same work, so it retains nothing to
  invalidate and arms no chain. Its `setContributions` still installs and returns
  `ViewEmit::default()`, which is correct for it and would become wrong the moment that app retains
  a session across turns.
- **`cad`, `process3d`, `forms` and `playbook` still declare a single-`json` `setContributions`** —
  unchanged from `📓️contributions-delivery-2026-09-10.md` §6. The new TS law asserts the receiving
  set is exactly the three procedural apps, so the day one of those four gains a paged route the law
  fails and has to be updated deliberately.
- **`pending_effects` is still only reachable through a `refreshUi` the shell may skip.** The fix
  here closes the contributions case specifically, by making the installing route re-arm what it
  invalidated. Any OTHER host-pushed, publication-free state change would have the same shape and
  would need the same treatment; the general form would be a host-side "the guest asked for a
  refresh" channel, which this lane deliberately did not invent.

## 8. Files changed

### Framework

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` | `FlowEvalSessionState.flow_extension_generation` (seeded in `new()`), `FlowEvalSession::flow_extension_generation()`, `FlowEvalSession::invalidate_for_flow_extension_registry()` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` | `unregister_linked_flow_extension_installer` — the missing inverse of the linked-installer registrar |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | **new law** `flow_eval_session_invalidates_only_when_the_flow_extension_registry_generation_moves` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | the `SEMIO_RUNTIME_DIAGNOSTICS`-gated `[DEBUG] contributions push` line at the push decision point, with a named `skipped` reason per gate; `appOwnsCommand`/`appCommandTakesPageRun` exported so a law can run the real gate |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | **new** `describe("contributions push declaration")` |

### Plugin

| file | change |
|---|---|
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` | `apply(…, preview_window_ids)` installs, invalidates on a moved registry generation and re-arms one addressed `flowEvalTick` per attached preview window; `handle` is the roster-less `app_commands!` row |
| `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` | `Generation3dContributionsWork::step` reads the attached preview windows off `input.context.view_state` and calls `apply` |
| `✏️s/…/🌀️generation2d/…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` | the twin, unaddressed |
| `✏️s/…/🌀️generation2d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` | **new** `rearm(req)` — the one declaration of that editor's self-redispatch |
| `✏️s/…/🌀️generation2d/…/✏️editor/🦀️.rs` | `pending_effects` reads the re-arm from `flow_eval_tick::rearm` instead of respelling the effect literal |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | `drain_armed_flow_eval_ticks_from` — the drain started from effects another route armed |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | **new laws** `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted` and `re_pushing_an_unchanged_closure_re_arms_nothing`, the `UnlinkedFlowExtensions` restore guard, and the `preview_phase`/`preview_mesh_count`/`flow_status_json` readers |
