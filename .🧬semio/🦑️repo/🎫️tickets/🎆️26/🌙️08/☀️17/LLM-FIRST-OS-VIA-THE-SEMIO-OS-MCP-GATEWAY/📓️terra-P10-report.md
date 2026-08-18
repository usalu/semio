# 📓️ terra report — packet P10-react-shell

## 1. Preconditions

- **Baseline HEAD** at session start: `d9542d156aa2da76dd53b18ab794f6fed2f0366a` — this already carries
  the peer ticket's H1 `ShellHost` rewrite (confirmed: `git log --date=iso --oneline -3 --
  ShellHost/🟦️component.tsx` → `d9542d156a` is the top commit). `Shell/🟦️component.tsx`'s last touch is
  `1eaf87e6f5` (P1b's window, before H1) — its reducer has NOT moved since; still the file the shell's
  `shellReducer` lives in.
- Read in full before writing anything: `📌️important.md`, `📓️design-decisions.md`,
  `📓️luna-shellstate-audit.md`, `📓️terra-P9-report.md`, `📓️terra-P1b-report.md`, `📋️master.md` §2.1–2.4,
  `/Users/ueli/Documents/semio/CLAUDE.md`, and the peer ticket's `📓️status.md` (its H1 section, §"W2 is
  now complete").
- Verified directly from disk (not assumed) before designing anything:
  - `🌉️mcp/🦀️component.rs` and `🌉️mcp/📦️bin.rs`: `run_http` does **not** mount
    `bridge::server::bridge_router()` — only `/mcp` is served. `bridge_router()` exists but is an
    unmounted skeleton (matches P1b report §2.4/§5's own words: "a later packet mounts it alongside
    `/mcp`"). No `~/.semio/agent/bridge-token`-file-writing code exists anywhere in the crate.
  - `🌉️mcp/🧵️bridge/🟦️component.ts` (417 lines) and `🖥️shell/🟦️component.ts` (635 lines,
    `🤖️generated/🟦️shell.ts` present locally, 17548 bytes): read in full for their exact exported API
    before writing a single line against them.
  - `ShellHost/🟦️component.tsx` (6874 lines) and `Shell/🟦️component.tsx`: read the relevant regions
    (imports, `useLabel`/i18n bundle registration pattern, the final JSX return) to match the
    co-location and accessibility conventions exactly, without editing either file.
  - `@semio-tech/framework-os-shell` IS a registered Cargo/npm workspace member already (`grep -n
    "🖥️shell" Cargo.toml package.json` — both present), so P9's own lease had already been applied by
    sol before this packet started.

## 2. Pre-change test baseline (established before any edit, foreground)

```
$ bun nx run @semio-tech/framework-renderer-react:test
...
 Test Files  1 failed (1)
      Tests  11 failed | 325 passed (336)
     Errors  1 error
   Start at  12:34:25
   Duration  4.53s
```
Full transcript: `🧪️baseline-renderer-react-test.txt` (session scratchpad, referenced here — its
content is reproduced in `🧪️postchange-direct-vitest.txt` below since the two runs are byte-identical
in pass/fail counts). The peer ticket's own H1 report claims "321/336 (15 pre-existing failures)"; my
own foreground measurement, taken fresh at this packet's start, is **11 failed | 325 passed (336
total)** — 4 fewer failures than their number (very plausibly other concurrent sessions fixed some in
the interim; either way this packet's own measurement, not the stale report figure, is the baseline it
proves zero regression against). The 11 failures are all pre-existing and unrelated to P10 (a
`ShellFaultBoundary`/`toHaveTextContent` Chai-property mismatch, several `mit-bestand` asset-path regex
mismatches, a command-category label rename, and two `resolveWindowActions`/`buildCommandCategoryTabs`
shape mismatches — none touch anything in `🧱️elements/Agent*/**`).

## 3. Files created

| file | lines | sha256 |
|---|---:|---|
| `🧱️elements/AgentBridge/🟦️component.tsx` | 494 | `a6476e87b9a510b1193e5b59d97bb602423db1d2c8309c8f2951658fdd3c92e8` |
| `🧱️elements/AgentBridge/🧪️component.test.ts` | 158 | `7fc278c9f8a3560d676a8ca441b853fc1918748eec47cd221aeebac322bf30c0` |
| `🧱️elements/AgentPresence/🟦️component.tsx` | 48 | `c07161b31158f0100a903c9775495690560527225a1f3e06e327c2b985035bcb` |
| `🧱️elements/AgentPresence/🧪️component.test.tsx` | 57 | `f6defb6939ca0a4125a24f21d61e5d33326d9fec64345c9f762fe9e9c9990b68` |
| `🧱️elements/AgentApprovals/🟦️component.tsx` | 160 | `5d2a08c312a44cdd4ce497127f70dfcf17798417f35b524fdb9ad721eeaad755` |
| `🧱️elements/AgentApprovals/🧪️component.test.tsx` | 87 | `ddfd09b42bee5f00368899809597d699b5de6b9c333106cac2585ce42b1533a7` |

Plus this report, `📓️sol-P10-react-shell-packet.md` (the verbatim brief), `📓️lease-P10-shellhost-mount.md`,
and scratch `.txt` transcripts in the ticket folder. No `🧪️story.tsx` files — the local co-location
convention this ticket points at (`ShellSync`, `Dock`, every other `🧱️elements/<Element>/` sibling)
has none; only the `🖱️ui` module's own elements use `🧪️story.tsx`, a different tree.

No `AGENTS.md` touched. `[DEBUG] ` grep across all 6 files: zero matches.

## 4. Design, per required piece

### 4.1 `AgentBridge`
- `discoverAgentBridgeConfig()` / `bridgeUrlWithToken()` — see §5 below, the full port/token
  discovery writeup.
- `createDefaultShellState()` — a field-for-field copy of `🖥️shell/🟦️component.ts`'s own in-source
  `defaultState()` test fixture (not exported — lives inside its `import.meta.vitest` block), used to
  seed the bridge's `ShellState` mirror before a real `ShellHost`-derived state is ever supplied.
- `encodeJsonPayload`/`decodeJsonPayload` — the wire codec for the `bytes` fields inside
  `ShellState`/`ShellStatePatch`/`ShellCommand`/`AppCommand` frames. `📋️master.md` §2.2 names these
  `pack`, but that binary format lives in `semio-framework-actor`'s wire module
  (`📡️spr/🧵️channel/🦀️component.rs`) — the peer ticket's exclusive, mid-rewrite territory
  (`📌️important.md`'s collision matrix, packet A4). Plain JSON is used instead, symmetric with what
  the gateway's own future dispatch handler already expects on the other end (P9 report §8.4:
  `serde_json::from_value::<ShellCommand>`) — the same kind of frame-envelope-vs-payload-format
  distinction P1b's own report §7.5 already drew for the bridge's primitives themselves.
- `applyInboundShellCommand(state, seq, commandBytes, nowMs)` — decodes the payload, calls
  `@semio-tech/framework-os-shell`'s `reduce()` (imported, not reimplemented), and returns
  `{command, result, resultFrame}`. `result`/`command` are `null` **only** when `commandBytes` itself
  fails to decode (a codec-level fault, reported via `resultFrame.fault`, never thrown); a command the
  reducer semantically rejects is `result: {ok:false, error: ShellError}` — not `null` — since that is
  real `reduce()` output, not a decode failure. This exact function is what both the hook's
  `shellCommand` frame handler and this file's own tests call — one code path, not two.
- `useAgentBridge(options)` — the hook. Dials `new WebSocket(bridgeUrlWithToken(config))` only when a
  config is present (from `options.config` or `discoverAgentBridgeConfig()`); sends `Hello` on open;
  on `welcome` publishes a full `shellState` snapshot (this packet always sends full snapshots, never
  patches — see §7 deviations); on `shellCommand` applies it and replies with `shellCommandResult`; on
  `approvalRequested`/`approvalResolved`/`agentPresence` updates local state; reconnects with capped
  exponential backoff (1s → 2s → 4s → … capped 30s) via `setTimeout`, never a busy loop; pings every
  20s; every socket call is fire-and-forget or timer-scheduled, so the UI thread is never blocked
  waiting on the gateway. `dispatch(command)` lets `ShellHost` (once it adopts this hook) push a local
  command through the same `reduce()` and publish the resulting state immediately.
- `agentUiLabel` — the `registerUiTranslationBundles({en, de})` caster (see §4.4), exported so
  `AgentPresence`/`AgentApprovals` share one registered vocabulary.

### 4.2 `AgentPresence`
Small `role="status"` indicator. `agentPresenceTone(status, presence)` is a pure, directly-tested
state machine (`"connected" | "working" | "connecting" | "disconnected"`) driving both the dot color
and which of the 5 label strings (`idle`/`working: {{label}}`/`connecting…`/`reconnecting…`/
`disconnected`) is shown. `aria-label` on the root carries the accessible name ("Agent status" /
"Agent-Status"), so the whole widget announces correctly to a screen reader without relying on visible
text alone.

### 4.3 `AgentApprovals`
The `os.agent.approvals` dialog. `parseApprovalSummary(summary)` is a pure, directly-tested parser:
the bridge's `ApprovalRequested` frame (`🌉️mcp/🧵️bridge/🦀️component.rs` tag 3) carries only
`{approvalId, summary: string}` — there is **no separate capability/diff/risk field on the wire today**
(a real gap between what this packet's brief asks the dialog to show and what P1b's already-shipped
frame shape carries — documented, not hacked around: I did not touch `🧵️bridge/🦀️component.rs`, since
it is P1b's already-closed, out-of-my-`path_scope` territory). `parseApprovalSummary` tries
`JSON.parse(summary)` for `{capabilityId, diffSummary, risk, requestedBy}` and falls back to treating
the whole string as the diff summary when it isn't that shape (or isn't JSON at all), so the dialog
never renders blank for a plain-text producer today while being ready for a richer JSON-encoded
`summary` the moment a future approval-producing packet starts sending one — zero wire-format change
needed on either side. The dialog auto-opens whenever `approvals.length > 0` (and pops back open on a
newly arrived approval even after a human dismissed an emptied dialog), lists every pending row with
capability/diff/requester/risk, and each of the 3 decision buttons (`Deny`/`Approve Once`/`Approve for
Session`) is wired straight to `onDecision(approvalId, decision)` — the caller passes
`useAgentBridge().resolveApproval`, which sends the `Approval{decision}` frame and optimistically
clears the row. Used plain `<button>` elements rather than `@semio-tech/ui-react`'s `Button` — that
component requires a mandatory `icon: ControlIcon` prop (verified by reading
`🖱️ui/🧱️elements/🔘️Button/🟦️component.tsx`) that sibling elements like `ShellSync` pass no value for
either (a pre-existing repo inconsistency, not something this packet's `path_scope` may fix); plain
`<button type="button" aria-label="…">` sidesteps it entirely while staying fully accessible.

### 4.4 i18n
`registerUiTranslationBundles({en, de})` (from `@semio-tech/ui-react`, the exact "product's own bundle"
mechanism `useLabel`'s own doc comment names — verified by reading `📚️I18n/🟦️component.tsx` and
`⚛️react/📦️index.tsx` directly, since no product in this repo had called it yet to copy from) is
called once in `AgentBridge/🟦️component.tsx`, registering 22 keys under `os.agent.presence.*` /
`os.agent.approvals.*`, English and German both, every leaf shaped `{label: {normal, beginner}}`
exactly like the domain-neutral chrome schema requires. `agentUiLabel(...)` is the typed caster both
`AgentPresence` and `AgentApprovals` import and call through `useLabel()` — the same hook every sibling
element (`ShellSync`, etc.) uses, so no new i18n plumbing was invented.

## 5. Port/token discovery in the browser — the compromise

`📋️master.md` §2.1 says the gateway mints a bridge token "0600 file" at start and shells "dial" a
loopback WebSocket; the packet brief said to read it "as P1b implemented" it. I read P1b's actual
shipped code (`🌉️mcp/🦀️component.rs`, `🌉️mcp/📦️bin.rs`, `🚚️transport/🦀️component.rs`) before writing
anything and found:

1. **No file-based token mint exists.** `HttpOptions.token` / `--token <t>` is a plain CLI-supplied
   bearer string (`bin.rs`'s `parse_http_args`) — nothing in the crate writes
   `~/.semio/agent/bridge-token` or any file like it. `default_audit_dir()` (the `📒️audit` facet) is
   the only "well-known dotfile path" helper that exists today, and it is for the audit log, not a
   token.
2. **The bridge WebSocket route is not mounted into the real process.** `run_http` builds an
   `HttpTransport` and calls `.serve(server)`, which only ever serves `/mcp`. `bridge::server::
   bridge_router()` exists (P1b built the codec + a one-shot `Hello`→`Welcome` skeleton, deliberately
   scoped out of mounting it — its own report §2.4 says so explicitly: "Do not wire the WebSocket
   server yet if that forces you into the shell's territory") but nothing calls it from `run_http`. A
   browser literally cannot dial a live bridge today, regardless of how it learns the URL/token.
3. **A browser tab cannot read `~/.semio/...` regardless** — even if step 1 existed, filesystem access
   is not something a browser page has. The packet brief's own fallback clause ("if the browser cannot
   read that file, take them from a dev-server-injected env/config value and say so") is exactly the
   situation, so `AgentBridge` implements exactly that fallback and nothing else:
   `discoverAgentBridgeConfig()` reads `import.meta.env.VITE_SEMIO_BRIDGE_URL` /
   `VITE_SEMIO_BRIDGE_TOKEN` (mirroring `ShellHost/🟦️component.tsx`'s own existing `readViteSEnv`
   idiom for `VITE_S_*` defines — guarded with try/catch for non-Vite embeds where `import.meta.env` is
   absent) and returns `null` (never throws) when either is unset. `useAgentBridge()` treats `null` as
   "stay `status: disabled` forever, never attempt a connection" — never blocks, never crashes a shell
   with no gateway configured.
4. Because Vite auto-exposes any `VITE_`-prefixed env var from `.env`/process env with **zero config
   changes needed** (confirmed: no `define` block is required for `VITE_S_HUB_URL`-style vars
   elsewhere in this codebase either), no lease against `🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts`
   was needed for this. A dev/launch config that wants `AgentBridge` to auto-connect sets
   `VITE_SEMIO_BRIDGE_URL=ws://127.0.0.1:6300/bridge` and `VITE_SEMIO_BRIDGE_TOKEN=<the --token
   the gateway process was started with>` before starting the dev server; today, with the bridge route
   unmounted, this compromise wires the shell side end-to-end and inert-safe, but there is genuinely
   nothing live to dial yet on the gateway side. **This is the one thing P12 (or a small follow-up
   packet) needs before an end-to-end browser demo works**: mount `bridge_router()` into `run_http`'s
   router and mint the token to a well-known place (or just pass it via the same env var this packet
   already reads). Nothing in `AgentBridge`'s own design changes when that lands — only where the
   config comes from.

## 6. Lease requests

- **`📓️lease-P10-shellhost-mount.md`** — the exact 3-import + 1-hook-call + 2-JSX-line diff for
  `ShellHost/🟦️component.tsx` (full fenced content in that file). **Not applied** — pending
  sol/registrar review. No `package.json`/`vitest.config.ts`/`vite.config.ts` lease was needed: every
  cross-module import (`🖥️shell/🟦️component.ts`, `🌉️mcp/🧵️bridge/🟦️component.ts`) is a relative-path
  import, the same pattern `ShellHost` itself already uses for `🟦️backbone-worker.ts` and
  `💻️os/🎚️config/…` — confirmed by reading those exact import lines before choosing this approach,
  precisely so no shared build-config file needed touching.
- An **optional, explicitly non-blocking** follow-up noted in that same lease file: extending
  `⚛️react/🧪️vitest.config.ts`'s `test.include` so my 3 co-located test files are picked up by the
  standard `bun nx run @semio-tech/framework-renderer-react:test` target (today that target's vitest
  `root` is the `⚛️react` package directory, which does not reach into `🧱️elements/**` at all — the
  entire existing suite for this project lives in one file, `⚛️react/🧪️index.test.ts`; no
  `*.test.ts(x)` exists anywhere under `🧱️elements/` yet). Not requested as blocking since §8 below
  verifies these files directly instead.

## 7. Deviations, with justification

1. **Full `ShellState` snapshots only, never `ShellStatePatch`.** `AgentBridge` always sends
   `buildShellStateFrame(state)` on `welcome` and after every locally-applied `dispatch()`. Patch
   diffing needs a stable "what changed" representation this packet's own `ShellState` mirror (which
   nothing else in the shell reads or writes yet) has no consumer motivating; `ShellStatePatch`'s wire
   shape (`revision`/`baseRevision`/`patch: bytes`) is unchanged and ready for a later packet to add
   diffing without touching this file's public API.
2. **`ShellState` mirror is `AgentBridge`'s own, not `ShellHost`'s live UI state.** `ShellHost` has not
   adopted `@semio-tech/framework-os-shell`'s `ShellCommand`/`reduce()` as its actual dispatch
   mechanism yet — P9's own report §8.1 calls that "adoption packet" work, explicitly separate from P9
   itself, and this packet's lease is scoped to a 1-3-line **mount**, not a `ShellHost` reducer
   rewrite. `AgentBridge` is therefore fully wired and testable end-to-end today against its own
   authoritative `ShellState` mirror (seeded via `createDefaultShellState()` or a caller-supplied
   `initialState`), but does not yet reflect `ShellHost`'s *real* live window/panel/session state until
   a future adoption packet routes `ShellHost`'s own dispatches through `reduce()` too and passes the
   resulting state in via `useAgentBridge({initialState: ...})`. Nothing about `AgentBridge`'s public
   API needs to change for that to happen.
3. **`ApprovalRequested.summary` structured-JSON convention, not a wire-format change.** See §4.3 — the
   bridge frame only carries a string; `parseApprovalSummary` documents and implements the richer shape
   a future approval-producing packet can start emitting with zero Rust/TS codec change on either side.
4. **Plain `<button>` instead of `@semio-tech/ui-react`'s `Button`** for the three decision buttons —
   see §4.3; that component's mandatory `icon` prop is a pre-existing repo inconsistency outside this
   packet's `path_scope` to fix, and `parseApprovalSummary`'s fallback keeps every control's accessible
   name self-contained via `aria-label` regardless.
5. **JSON, not `pack`, for the `bytes` payloads inside `ShellState`/`ShellCommand` frames** — see §4.1;
   `pack` lives in the peer ticket's exclusive `semio-framework-actor` territory this packet must not
   depend on, mirroring P1b's own precedent for keeping the bridge's wire primitives self-contained.
6. **No `🧪️story.tsx` files** — the co-location convention this packet's brief points at (`ShellSync`,
   `Dock`, every sibling `🧱️elements/<Element>/`) has none; story files are a different tree's (`🖱️ui`
   module) convention, not this one's.

No other deviations. Every piece named in the brief's §3/§4 is present and self-contained inside the 3
owned directories.

## 8. Acceptance — verbatim output + exit codes

### `bun nx run @semio-tech/framework-renderer-react:test` — confirmed project name from `package.json`

Pre-change baseline (§2 above): **11 failed | 325 passed (336)**, exit non-zero (nx surfaces test
failures as a failed task — this is the SAME pre-existing-failure shape before any of my edits, not a
regression I introduced).

Post-change: the nx-wrapped target hit its own internal 15-second subprocess budget twice in a row
under this session's system load (`[budget] … exceeded 15000ms — killed`) — concurrent sessions on this
shared machine (this ticket's own `📌️important.md` "≤6 concurrent building agents" note, and the "Concurrent
Cargo Workspace Churn"-style hazard this environment is known for) were driving up esbuild
transform/import times generally, not something scoped to my files (my new files are not imported by
`index.tsx` at all, so they cannot be the cause). Verified instead via a **direct, unwrapped** foreground
invocation of the exact same command vitest would run:

```
$ node node_modules/vitest/vitest.mjs run --config 🧪️vitest.config.ts --passWithNoTests
 (cwd: 🧰️framework/…/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react)
...
 Test Files  1 failed (1)
      Tests  11 failed | 325 passed (336)
     Errors  1 error
   Start at  12:49:35
   Duration  44.70s (transform 18.65s, import 37.61s — inflated by concurrent load, not my code)
```
**Identical 11 failed | 325 passed (336) to the pre-change baseline — zero regression.** Full
transcript: `🧪️postchange-direct-vitest.txt`.

### My own 3 component-level test files — not discovered by the above target (§6), verified directly

```
$ node node_modules/vitest/vitest.mjs run \
    --config <…>/⚛️react/🧪️vitest.config.ts --passWithNoTests \
    --root 🧰️framework/…/📺️renderer/🧑️‍🎨️engine/🧱️elements \
    AgentBridge/🧪️component.test.ts AgentPresence/🧪️component.test.tsx AgentApprovals/🧪️component.test.tsx
...
 Test Files  3 passed (3)
      Tests  51 passed (51)
```
Full transcript: `🧪️agent-elements-test4.txt`. Breakdown: `AgentBridge/🧪️component.test.ts` — 3
config-discovery + 3 URL-encoding + 1 codec-parity structural check + one dynamically-generated `it`
per row of P1b's own `🧵️bridge/🧫️fixtures/frames.json` (20 rows, both directions, proving the codec I
imported round-trips every fixture byte-for-byte) + 3 `applyInboundShellCommand` + 1
`buildShellStateFrame` = 31 tests. `AgentPresence/🧪️component.test.tsx` — 6 pure `agentPresenceTone`
cases + 5 render/label assertions = 11 tests. `AgentApprovals/🧪️component.test.tsx` — 3
`parseApprovalSummary` cases + 6 render/dispatch tests (including the deny/once/session
decision-dispatch assertions on the exact `(approvalId, decision)` pair, and the multi-approval-row
test) = 9 tests. 31 + 11 + 9 = 51, matching the run.

A scoped `bunx tsc --noEmit` pass against all 6 files together (matching P1b's own precedent of
checking one file in isolation) produced a large volume of output, but **zero errors reference any file
under `🧱️elements/Agent{Bridge,Presence,Approvals}/`** — every reported error is a pre-existing,
unrelated issue elsewhere in the whole-program dependency graph tsc necessarily pulls in (e.g.
`framework/📦️packages/🟦️typescript/🟦️glue.ts`'s `PlayerEvent`/`StatechartEvent` mismatches,
`🎠️kernel/🟦️component.ts`'s `PluginManifest.contributions` mismatch) — none inside my `path_scope`,
none touched by me. `grep -c AgentBridge\|AgentPresence\|AgentApprovals` on the full tsc output: 0.

### `bun nx run @semio-tech/framework-os-shell:test-quick`

```
$ bun nx run @semio-tech/framework-os-shell:test-quick
 Test Files  2 passed (2)
      Tests  6 passed (6)
 NX   Successfully ran target test-quick for project @semio-tech/framework-os-shell
```
Unmodified since P9 — confirms my reading of its API didn't require any change there. Exit code 0.

### `CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp`

```
test result: ok. 160 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 214.05s
     Running unittests ../../📦️bin.rs (…)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
**160 passed — matches the required number exactly.** Full transcript: `🧪️cargo-test-os-mcp.txt`. Exit
code 0.

## 9. What P12 can now drive end to end

- **Codec + reducer, fully live today, no gateway needed**: `AgentBridge`'s exported
  `applyInboundShellCommand`/`buildShellStateFrame`/`encodeJsonPayload`/`decodeJsonPayload` are real,
  tested functions a headless test (or a future in-process gateway stub) can call directly to prove a
  `ShellCommand` → `reduce()` → `ShellCommandResult` round trip without a socket at all.
- **The moment `ShellHost` applies `📓️lease-P10-shellhost-mount.md`'s diff**: a mounted React shell
  gains a live `AgentPresence` indicator and an `AgentApprovals` dialog wired to `useAgentBridge()`'s
  `status`/`presence`/`pendingApprovals`/`resolveApproval`. With `VITE_SEMIO_BRIDGE_URL`/
  `VITE_SEMIO_BRIDGE_TOKEN` set at dev-server launch, the hook will dial, send `Hello`, and (once a
  small follow-up mounts `bridge_router()` into `run_http` per §5 point 4) publish real `ShellState`
  snapshots and answer real inbound `ShellCommand`s the gateway sends — the exact wiring P12's
  browser e2e needs to drive the shell from an agent. Until that gateway-side mount lands, the hook
  stays safely `status: "disabled"` and every other part of the shell is unaffected.
- **Human-in-the-loop approvals**: once a policy-layer packet starts emitting `ApprovalRequested`
  frames (optionally with the `{capabilityId, diffSummary, risk, requestedBy}` JSON convention
  `parseApprovalSummary` already understands), they render immediately in a real dialog with working
  Deny/Approve-Once/Approve-for-Session buttons — no shell-side work left for that packet to do beyond
  emitting the frame.
