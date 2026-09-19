# U1 — progress, cancel and connection status (ticket 26/09/18)

Slice U1 of `📓️g5-ux-completeness-audit.md`'s ranked fix list: items **1, 2, 4, 5, 8** (+13 assessed).

| # | item | state |
|---|---|---|
| 1 | Mount `🧵️TaskManager` as a real os window `os.task-manager` | **done** (mounted, dockable, in the command palette, live row feed) |
| 2 | Cancel affordance for an in-flight tool call in `💬️AgentChatPanel` | **done** through a new real bridge frame + the gateway's own job registry; one honest residual gap (§2.4) |
| 4 | Localize `🔄️ShellSync` `syncStatusLabel` (en + de) | **done**, plus a genuinely unresolved key found while doing it |
| 5 | Cancel an in-flight plugin install in `📌️ChromePanels` | **already landed** by a predecessor U1 session — verified, one `[DEBUG]` leftover cleaned |
| 8 | Persistent hub-connection indicator in the shell chrome | **done** (footer, `role=status`, icon + text, en + de) |
| 13 | `cause.message` English leak in the rejected-mutation toast | **not changed** — assessed, §13 says why |

---

## 1. `🧵️TaskManager` mounted as `os.task-manager`

The audit's finding was exact: the only central progress + cancel surface for actor-level work existed
in code and was reachable by nobody. Its own header punch-list asked for "a window-kind registration
and a `ShellHost` mount". Both now exist, plus the action-line entry the brief asked for.

**Fix**

| what | file:line |
|---|---|
| panel id `os.task-manager` (the SAME id the row actions' `ActionDescriptor.controllerId` already used) | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx:84` (`FRAMEWORK_TASK_MANAGER_PANEL_ID`) |
| `createFrameworkTaskManagerPanelTab` — built exactly like the chat leaf, so it docks/drags/folds and reaches the mobile stack with every other chrome tab | `…/📌️ChromePanels/🟦️.tsx:1394` |
| tab memo + dock placement (`bottom-right`, beside Marketplace) | `…/🏛️ShellHost/🟦️.tsx:8684`, `…:9591` |
| `os.openTaskManager` palette/action-line command | `…/🛠️ShellHelpers/🟦️.tsx:2565` (`OPEN_TASK_MANAGER_COMMAND_ID`), `…:4620` (`buildOsCommands` entry) |
| command handler (reveals the tab + shows the anchor, the same pair `open-artifact-with-*` uses) | `…/🏛️ShellHost/🟦️.tsx:9462` |
| live row feed — subscribes to `ActivationRegistry.metricsBus`'s `os.runtime.metrics` | `…/🧵️TaskManager/🟦️.tsx` `🔖️LiveFeed` region: `runtimeMetricsRowsV1`, `useRuntimeMetricsRows`, `TaskManagerWindow` |
| optional `activationRegistry` prop on `FrameworkOsShell` | `…/🏛️ShellHost/🟦️.tsx:1216` (props), `…:1990` (inner) |
| i18n `ui.panelToggle.taskManager`, `ui.command.openTaskManager` (en + de) | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:171,278`; values in `🎯️targets/⚛️react/🟦️.tsx` both locale blocks |

**Two design decisions worth stating.**

1. `TaskManagerRow`'s counters became `number | null` and render as an em dash. A web
   `ActivationRegistry` genuinely cannot observe `turns`/`traps`/`wallUsP95`/`mailboxLen`/`restarts`
   — it delegates straight to `ShardClient` and holds no `Kernel` (the kernel module's own
   `runtimeMetricsActorRows` doc says the same). Zero-filling them would read as "nothing has
   happened", which is false. `taskManagerMetricCell`/`metricText` keep the React table and the wgpu
   `TableScene` saying the identical thing.
2. `TaskManagerPanel` distinguishes **"no runtime attached"** from **"the runtime reports no
   actors"** (`runtimeAttached` prop, `data-semio-task-manager-empty="no-runtime" | "no-actors"`).
   An empty table would conflate two very different facts.

**Evidence** — `🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx`, 16 tests, all green (run below). The live-feed
tests drive a **real** `ActivationRegistry` + `ShardClient` (auto-replying fake `Worker`, the one seam
`ShardWorkerLike` exists for), activate a real actor, and assert the row appears; a second actor plus a
dispatched `os.runtime.metrics` event proves the subscription follows. The kernel's
`startRuntimeMetricsPublisher` doc said "no real CONSUMER subscribes to `metricsBus` yet anywhere in
this codebase" — that is no longer true.

**Honest gap.** The React host passes no registry today, because it has none: the only production
`new ActivationRegistry` in the repo is module-private inside
`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:228`, and the React shell's plugin runtime
(`🔌️PluginRuntime/🟦️.tsx`) contains no reference to `ActivationRegistry`/`ShardClient` at all. So in the
React shell the window opens and honestly says "no actor runtime is attached to this shell". Closing
that needs a host that owns a kernel to pass `activationRegistry` — one prop, no change to this
window. That is a pre-existing architectural fact, not something this slice introduced or hid.

## 2. Agent chat: cancel an in-flight tool call

The audit: "no cancel button anywhere in `AgentChatPanel/🟦️.tsx` … despite `inference_cancel`/
`job_cancel`/`action_cancel` existing as real MCP-layer tools". Tracing M2's frame family
(`📓️m2-agent-surface-and-inference.md` §2) showed why: **there was no shell→gateway cancel frame at
all**, and those `*_cancel` tools are agent-facing, not shell-facing. So the wiring had to be built,
not merely exposed.

### 2.1 A real frame (Rust is the SSOT, TS is the twin, the fixture is the anti-drift mechanism)

`ShellToGateway::AgentCancel { invocation_id }`, tag **10**:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs` — variant, `encode`, `decode`,
  `ShellFrameKind::AgentCancel`, `ShellDecodePhase::AgentCancelInvocation` (bounded decode),
  `ShellMaterializePhase::AgentCancelInvocation` (+ tag dispatch and `finish()`): eleven sites, the
  same set `AgentMessage` occupies.
- `…/🧵️bridge/🟦️.ts` — the TypeScript twin (`{ variant: "agentCancel"; invocationId: string }`,
  encoder case, decoder case 10).
- `…/🧵️bridge/🧫️fixtures/📨️frames.json` — one canonical row, `0a05000000696e765f31`; 23 → **24** rows
  (13 shell→gateway / 11 gateway→shell). Both codecs answer that file.
- `…/🚚️transport/🦀️.rs:1317` — recorded like every other shell→gateway observation.

### 2.2 The gateway effect is the REAL cooperative cancel, not a second mechanism

`AgentConversation::begin_tool_call` now opens a job in the one process-wide
`crate::ui::job_registry()` **keyed by the invocation id the panel already renders**, and moves it off
`Pending` (`🧵️bridge/🦀️.rs`, `💬️AgentConversation` region). `BridgeHandle::record` routes an inbound
`AgentCancel` to `job_registry().request_cancel(invocation_id)` — the exact call the `job_cancel` MCP
tool makes — before taking the connections lock, so the two locks never nest.
`finish_tool_call` settles a cancelled call as `Cancelled` rather than as whatever the handler
returned. Consequence: the panel's control, the `job_cancel` tool and `semio://job/{id}` all act on
one record.

### 2.3 The surface

- `useAgentBridge().cancelToolCall(invocationId): boolean` — `false` when no socket is open, the same
  contract `sendAgentMessage` uses, so the panel never shows a cancel that did not leave
  (`…/🧱️elements/🔗️AgentBridge/🟦️.tsx`).
- `AgentConversationEntry.toolCall.state` gained `"cancelling"` — this shell's own optimistic state
  between the frame leaving and the gateway's real `agentToolResult` landing. Cancellation is
  cooperative, so the row says *asked to stop*, never *stopped*.
- `💬️AgentChatPanel/🟦️.tsx` — an optional `onCancelToolCall`; the control renders **only** on a row
  still reported `running` and **only** when a cancel path is attached, carries
  `aria-label`/`title` naming the tool (`os.agent.chat.cancelToolCall`, `"Cancel {{tool}}"` /
  `"{{tool}} abbrechen"`), and is a real labelled button, not an icon alone.
- Three new `os.agent.chat.*` keys in `🔗️AgentBridge/🟦️.tsx`, en + de, through the same
  compile-checked `registerUiTranslationBundles` mechanism M2 used.
- `🏛️ShellHost/🟦️.tsx:8681` passes `agentBridge.cancelToolCall` into the panel.

Slice M4 is adding an approval affordance to the same file; the panel file was re-read immediately
before each edit (md5 `3cadd342…` unchanged across the session) and the change is confined to the
tool-call row's trailing control plus one optional prop.

### 2.4 A real bug found and fixed on the way

`useAgentBridge` computed `shellSessionId` as `options.shellSessionId ?? \`shell-${Math.random()…}\``
**in the render body**, while that same value sat in the socket effect's dependency array. Every state
change the hook made — a tool call arriving, a message echoing, a cancel leaving — therefore tore the
bridge websocket down and redialled it. The `cancelToolCall` test caught it: the frame after
`agentCancel` was a `bye`. Fixed with a `useRef`-minted id (`🔗️AgentBridge/🟦️.tsx`, `🔖️Hook` region).

### 2.5 Honest gap (the one that remains)

`McpServer::handle_tools_call` runs `self.tools.call(...)` **synchronously** on the request thread. A
cancel arriving on the bridge socket can therefore only be *observed*, never *preempt*. Today the
invocation's job flips to cancel-requested and settles `Cancelled`, which `job_get` and
`semio://job/{id}` report — real, and the same guarantee `job_cancel` gives — but no existing handler
polls the *invocation* job id (`inference_run` polls its own `cancellationId`). Making a long tool
actually stop needs the invocation id handed to the handler as an ambient cancellation token; that is
M2's `💡️inference` territory and was deliberately not taken here. **Do not read this as "a running
inference stops when you press cancel" — it does not yet.**

## 4. `ShellSync` status localization

`syncStatusLabel` hard-coded `live`, `connecting…`, `reconnecting…`, `offline`, `saved`, `unsaved`,
`N pending` and the English `peer`/`peers` plural — the literal offline/reconnect status text, in
English for every locale.

**Fix** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🟦️.tsx`):

- `syncStatusLabelV1(status, texts)` — pure, React-free, unit-testable with either locale's real
  bundle values.
- `useSyncStatusTexts` / `useSyncStatusLabel` — the only place the `ui.sync.*` keys are read. Both
  counted keys resolve unconditionally with the live counts (hooks may not run behind a branch) and
  the composer picks which words the state needs.
- The badge is now `role="status" aria-live="polite"` with `aria-label={ui.sync.statusLabel}`.
- Plurals are two explicit keys (`peerOne`/`peerMany`) chosen in code rather than i18next's `_one`/
  `_other` suffixes, which would not be expressible in the compile-checked `UiTranslationSchema`.

Thirteen new `ui.sync.*` leaves in `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:482` and both locale blocks of
`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`. Because the bundle is `satisfies Record<UiLocale, …>`, a key added
to one locale and not the other does not compile — the en/de pair is a type error, not a review item.

**A real bug found while doing it:** `ShellSync` already called `useLabel("ui.sync.browse")`, and
`ui.sync.browse` **did not exist in the schema at all** (grep over `📚️I18n/🟦️.tsx` for `browse`: zero
hits). i18next's default missing-key behaviour echoes the key, so the folder-picker button's
accessible name and tooltip literally read `ui.sync.browse` in both locales. The key now exists
("Browse" / "Durchsuchen"; beginner tier "Pick a folder" / "Ordner wählen").

## 5. Plugin install cancel — already landed, verified

This was already implemented when I arrived, by a predecessor U1 session (the audit describes it as
missing; the code has it):

- `pluginInstallAbortsRef` + one `AbortController` per install
  (`🏛️ShellHost/🟦️.tsx:2783`, and registered inside `installPlugin`).
- `installingPluginIds` derived from the SAME `"installing"` status the install path already
  dispatches — no second notion of "an install is happening" (`…:3710`).
- `cancelPluginInstalls` (`…:3715`) and the install band's cancel control (`…:11041`,
  `data-semio-plugin-install-cancel`), with `role="status" aria-live="polite"` on the band.
- `loadPluginModuleResilient` races the `AbortSignal` explicitly rather than only handing it to
  `loadPluginModule`, "because the point of a cancel control is that the AWAIT settles now"
  (`🛠️ShellHelpers/🟦️.tsx:1811`).

O1's report (`📓️o1-multi-plugin-hub.md` §5, `:9167`) covers the **wgpu** half
(`ShellState::plugin_install` + `CancelToken` + `cancel_plugin_install()`), and its own §273 flags
"Chrome for `ShellState::plugin_install`" as still missing on that side — i.e. React has the chrome,
wgpu has the state machine. Recorded as a parity gap below, not closed here.

**One cleanup:** `🛠️ShellHelpers/🟦️.tsx` logged `"[DEBUG] plugin install cancelled …"` on the cancel
path. The surrounding doc comment says that log is intentional and permanent ("A cancelled load is not
a fault and is logged as such"), so the `[DEBUG] ` marker — which the preamble says must not be left
behind — was removed and the log kept.

## 8. Persistent hub-connection indicator

**Fix** (`🔄️ShellSync/🟦️.tsx`, `🔖️hub-connection-indicator` region):

- `hubConnectionSummaryV1(statuses, session)` — pure. Folds every attached document's `RemoteState`
  into one aggregate, best state first: one `live` document means the hub is reachable, a document
  still dialling outranks one already in backoff, everything detached (or nothing attached) is
  `offline`. `signedOut` outranks all of them, because no transport state means anything without a
  session. `peerCount` is the **max** over live documents, never a sum — a sum would double-count a
  peer who has two documents open, which no reader could interpret.
- `HubSessionPresenceV1 = "signedIn" | "signedOut" | "none"`. `"none"` (no sign-in surface mounted at
  all) is deliberately not the same as `"signedOut"`; only the latter is actionable and only it offers
  the sign-in entry point.
- `HubConnectionIndicator` — `role="status"`, `aria-live="polite"`, accessible name
  `"{hub label}: {state text}"`, and **icon + text**, never colour alone; the tone class is decoration
  on top of a readable word. Mounted in the shell footer beside the presence bar
  (`🏛️ShellHost/🟦️.tsx` `footerItems`), on mobile too — a phone is exactly where a connection drops.
- Source of truth is `shellState.sync.syncStatusByDocumentId` in full (`hubConnectionStatuses`,
  `🏛️ShellHost/🟦️.tsx:8890`), not the active document's status — that is the whole point.
- New `ui.sync.hubLabel` / `ui.sync.hubSignIn` / `ui.sync.signedOut` keys, en + de.

**Coordination with AU2.** `📓️au2-os-sign-in-and-spaces-ui.md` is still all `(pending)` and neither
`🔐️HubSignIn` nor `🔗️HubConnection` is mounted in `ShellHost` yet (grep: their only consumers are
their own stories/tests). So `hubSessionPresence` is a documented constant `"none"` today
(`🏛️ShellHost/🟦️.tsx:8895`) with the exact replacement named in its doc comment: AU2's
`useHubConnection` session state maps to `"signedIn"`/`"signedOut"`, and its sign-in opener becomes
the indicator's `onSignIn`. The indicator is already the natural entry point — it just has nothing to
open yet, and says so by offering no button rather than a dead one.

## 13. Rejected-mutation toast English leak — assessed, not changed

`ShellHost/🟦️.tsx`'s `showMutationRejectedNotice` appends `cause.message`, raw English Rust prose. The
audit already calls it "a known, designed partial gap" and the code's own comment (lines 8168-8171)
documents the rule it protects: *the UI localizes by code, never by parsing the English message
prose*. Fixing it properly means the guest emitting a localizable code + arguments per cause, which is
a wire-contract change in the mutation-outcome vocabulary, not a renderer edit. Changing only the
renderer would either drop real diagnostic detail or start parsing prose — both worse than the
documented tradeoff. **Left as-is deliberately; it needs its own ticket against the guest-side
`causes` wiring the same comment already points at.**

---

## wgpu parity

Checked per item; the shared `UiNode` schema carries a change declaratively only where the surface is
already a generic scene kind.

| item | wgpu status |
|---|---|
| 1 TaskManager | **Partially parity-free by construction.** The pane's scene shape (`buildTaskManagerTableScene`) already mints the generic `TableScene` both renderers parse (`ui_wgpu::wgpu::SurfaceKind::Table`), and my `taskManagerMetricCell` change keeps the two byte-identical (an em dash is a text cell in both). What is NOT done on wgpu is the *window registration*: the wgpu shell's dock/tab list is Rust (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`), so adding an `os.task-manager` tab there plus a `Kernel::runtime_metrics_snapshot` publisher is substantial Rust. **Recorded gap.** |
| 2 agent cancel | The frame and the gateway effect are in the Rust SSOT, so a wgpu shell gets them for free the moment it renders a chat surface. `📓️m3-mcp-tests-and-wgpu-agent-panel.md` owns the wgpu agent panel; the cancel control there is one more retained button bound to the same `AgentCancel` frame. **Recorded gap**, not attempted — it is M3's surface. |
| 4 sync status | wgpu's own status strings live in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` and use `LocalizedLabel` (exhaustive `Locale` match, compile-enforced en/de), so that target never had the English-literal defect. Nothing to port. |
| 5 install cancel | Inverted: wgpu has the state machine (`ShellPluginInstallPhase`, `CancelToken`, `cancel_plugin_install()`, O1 `:9167`) and no chrome; React now has both. **Recorded gap** — O1's own §273 already lists it. |
| 8 hub indicator | Needs a Rust twin of `hubConnectionSummaryV1` plus a footer band in the wgpu shell. The aggregation rule is 10 lines of pure logic and the fold order is specified above precisely so a Rust twin can be byte-identical. **Recorded gap.** |

## Tests run (real counts, all foreground)

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts \
    …/🧱️elements/🔄️ShellSync/🧪️tests/🧩️component/🟦️.tsx
 Test Files  1 passed (1)       Tests  11 passed (11)

$ … …/🧱️elements/🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx
 Test Files  1 passed (1)       Tests  16 passed (16)      # 12 pre-existing + 4 new live-feed laws

$ … …/🧱️elements/💬️AgentChatPanel/🧪️tests/🧩️component/🟦️.tsx
 Test Files  1 passed (1)       Tests  4 passed (4)

$ SEMIO_TEST_LEVEL=long SEMIO_INCLUDE_AGENT_BRIDGE=1 bun x vitest run --config …
 Tests  1 failed | 35 passed (36)        # the 1 failure is pre-existing, see below
```

```
$ cargo check --manifest-path 🌉️mcp/📦️packages/🦀️rust/Cargo.toml --all-targets --message-format short
    Finished `dev` profile [unoptimized] target(s) in 23.38s      # 0 errors; 39 warnings prove expansion completed

$ cargo test --manifest-path … --lib -- --test-threads=1 fixture agent_conversation bridge
test result: ok. 46 passed; 0 failed        # incl. every_fixture_round_trips_through_the_rust_codec (now 13/11 rows)

$ cargo test --manifest-path … --lib -- --test-threads=1 agent_cancel
test ui::quick::a_late_agent_cancel_frame_is_absorbed_without_disturbing_the_settled_job ... ok
test ui::quick::an_agent_cancel_frame_flips_the_tool_calls_own_job_and_settles_it_cancelled ... ok
test result: ok. 2 passed; 0 failed
```

New/extended suites, and all three are **registered** in
`📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`'s `engineTestSuites` — that file's own
comment warns a co-located suite no runner includes is a gate that reads green while measuring
nothing, and `🧵️TaskManager`'s existing suite **was exactly that** (in no include list at all). It is
now included too.

## Honest gaps

1. **The React shell has no `ActivationRegistry`**, so `os.task-manager` opens and says "no actor
   runtime is attached". §1 above has the evidence and the one-prop fix. The window, its dock entry,
   its command, its live subscription and its row actions are all real and tested against a real
   registry — only this host supplies none.
2. **An agent cancel is observed, not preemptive** — §2.5. A running `inference_run` does not stop
   today; its invocation job flips to cancel-requested and settles `Cancelled`.
3. **wgpu parity for items 1, 2, 5, 8** is unclosed — the table above says exactly what each needs.
   Only item 4 needed nothing.
4. **Pre-existing failing test, not mine:** `AgentBridge inference state parity > starts from the
   neutral Shell state…` fails because `createDefaultShellState()` now carries nine `ui*` preference
   fields the fixture `🖥️shell/🧫️fixtures/💡️set-document-inference-port.json` does not
   (`uiAppearance`, `uiCustomDrivers`, `uiCustomThemes`, `uiDriverId`, `uiKeybindingOverrides`,
   `uiLayout`, `uiLocale`, `uiTerminology`, `uiThemeId`). I touched neither
   `createDefaultShellState`, the reducer, nor that fixture. It fails identically with and without my
   `cancelToolCall` addition.
5. **A full `cargo test --lib` on `semio-framework-os-mcp` did not complete** inside 10 min under the
   fleet's load (the `--test-threads=1` run reached no output and was stopped). The targeted filters
   above are what actually ran; the `transport::long::*` socket tests are unverified by me.
6. **I disrupted a peer.** Stopping my own stalled test run, I used
   `pkill -f "semio_framework_os_mcp-…"`, which matched slice **M4**'s concurrently running filtered
   test binary (pid 41644, filters `elicit approval unbound rendezvous coordinator auto_approve`,
   writing `🗑️generated/m4-unit-tests.txt`) and killed it too. No files were damaged — M4 only needs to
   re-run that command. Recorded rather than hidden.
7. **Typecheck**: see the section below. My files are clean under the os scoped tsconfig; 97
   pre-existing errors remain in the program, four of them in files I edited but on untouched lines.
8. **`TaskManagerRow`'s counters became nullable**, which is a public type change. Nothing outside
   this element and its test constructs one (grep: `TaskManagerRow` has no other consumer), so the
   blast radius is zero today — but a future producer must decide `null` vs a real number per field
   rather than defaulting to `0`.

## Typecheck

Scoped the same way T4 scopes its measurements: `🔣️u1-scope.json` (this folder) extends
`💻️os/tsconfig.json` and pins exactly the 13 files this slice touched, so the program is the os
product's real import graph rather than a file in isolation. Capture: `🗑️generated/u1-typecheck.txt`.

```
$ bun x tsc --noEmit -p 🔣️u1-scope.json --pretty false
first run (before my own fixes below):  159 error TS…
final run:                               97 error TS…
```

**Zero of the remaining 97 are in `🔄️ShellSync`, `💬️AgentChatPanel`, `🧵️TaskManager`, the bridge
`🟦️.ts`, or any of the three new/extended test suites.** Four remain in files I edited, all
pre-existing and on lines this slice never touched:

| file:line | error | why it is not mine |
|---|---|---|
| `📌️ChromePanels/🟦️.tsx:667,774` | `ThemePaletteGroup` key map does not satisfy the translation-key union | theme-palette region; my edits are at `:84` and `:1394` |
| `🔗️AgentBridge/🟦️.tsx:205` | `'uiAppearance' does not exist in type 'ShellState'` | inside `createDefaultShellState`, untouched — the SAME `ShellState` drift as the pre-existing failing test in §gaps 4 |
| `🛠️ShellHelpers/🟦️.tsx:656` | `Uint8Array` not assignable to `BlobPart` | download path, untouched |

Three real defects **in my own files** were found and fixed by this run rather than reported around:

1. `TaskManagerTableRow` was `Record<string, TaskManagerTableCell> & { readonly id: string }` — an
   intersection that makes `id` unsatisfiable, so the whole `taskManagerRows` return was untypeable.
   Replaced with an index signature admitting the id string beside the cells.
2. `TaskManagerPanel`'s three row buttons passed `<Icon>` children with no `icon` prop, which `Button`
   requires. Rewritten as `<Button icon="pause|play|square" … />`, the idiom the rest of the codebase
   uses; the 16 tests (which query by accessible name) still pass.
3. The test suite's cell read then needed an explicit narrow past the index signature.

The 159 → 97 drop is larger than those three account for: a peer's fix to `🏛️ShellHost`'s tutorial
types landed between the two runs. I am claiming only that my files are clean, not credit for the
delta.

## Files changed

| file | what |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` | `ui.sync.*` (13 leaves incl. the previously-missing `browse`), `ui.panelToggle.taskManager`, `ui.command.openTaskManager` |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | the en and de values for all of the above |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🟦️.tsx` | localized status composer + hooks; `🔖️hub-connection-indicator` region |
| `…/🧱️elements/🔄️ShellSync/🧪️tests/🧩️component/🟦️.tsx` | **new** — 11 laws |
| `…/🧱️elements/🧵️TaskManager/🟦️.tsx` | nullable counters, em-dash cells, empty states, `🔖️LiveFeed` region, header punch-list updated, row-type and `Button icon` typecheck fixes |
| `…/🧱️elements/🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx` | +4 live-feed laws, fixture/imports updated |
| `…/🧱️elements/💬️AgentChatPanel/🟦️.tsx` | cancel control on a running tool call, `cancelling` state text |
| `…/🧱️elements/💬️AgentChatPanel/🧪️tests/🧩️component/🟦️.tsx` | **new** — 4 laws |
| `…/🧱️elements/🔗️AgentBridge/🟦️.tsx` | `cancelToolCall`, `cancelling` entry state, 3 `os.agent.chat.*` keys (en+de), stable `shellSessionId` fix |
| `…/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts` | +1 `cancelToolCall` law over the real hook and a fake socket |
| `…/🧱️elements/📌️ChromePanels/🟦️.tsx` | `FRAMEWORK_TASK_MANAGER_PANEL_ID`, `createFrameworkTaskManagerPanelTab` |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx` | task-manager tab + dock + command handler, `activationRegistry` prop, hub indicator in the footer, `onCancelToolCall` wiring |
| `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `OPEN_TASK_MANAGER_COMMAND_ID` + its `buildOsCommands` entry; `[DEBUG]` marker removed |
| `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registered the 3 element suites (2 new + the orphaned TaskManager one) |
| `🎫️tickets/…/🔣️u1-scope.json` | scoped tsconfig for this slice's typecheck (ticket-local, kept) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs` | `ShellToGateway::AgentCancel` (11 codec sites), job-registry wiring in `AgentConversation`, `record` routing |
| `…/🌉️mcp/🧵️bridge/🟦️.ts` | the TypeScript twin of the frame |
| `…/🌉️mcp/🧵️bridge/🧫️fixtures/📨️frames.json` | +1 canonical row (23 → 24) |
| `…/🌉️mcp/🧵️bridge/🧪️tests/🔬️quick/🦀️.rs` | fixture count 12 → 13 shell→gateway |
| `…/🌉️mcp/🚚️transport/🦀️.rs` | records `AgentCancel` like every other shell→gateway frame |
| `…/🌉️mcp/🖥️ui/🧪️tests/🔬️quick/🦀️.rs` | +2 cancel laws |
