# W3-C report — `verify collab` two-browser hub e2e

Lane 3-C. `bun ./📜️script.ts verify collab` (from the dev package) boots the real hub plus two
independent `s` react dev servers (one per user) and drives them as two Playwright browser contexts
through the ticket's 8-step collaboration scenario. **The harness runs end to end today and produces a
real, reproducible 0/8** — every step failed for a concrete, evidenced reason, not an infra crash. This
report is the truth as of the run captured in `🧪️3-c-collab-e2e.txt` (28786-ish lines, `REAL_EXIT_CODE=1`).

## Reproduction

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript
bun ./📜️script.ts verify collab
```
Or via nx: `bun nx run @semio-tech/framework-os-dev:collab-e2e`.

Port/env knobs (contract §C0): `S_COLLAB_HUB_PORT` / `S_COLLAB_USER1_PORT` / `S_COLLAB_USER2_PORT`
override the auto-scanned 7400–7498 pool; everything else (temp `OS_HUB_DATA`, `OS_HUB_ADMIN_TOKEN=e2e-
admin`, `S_USER=user1@semio.dev`/`user2@semio.dev`, separate `S_DATA_DIR`s) is internal to the harness.
Budgets: `COLLAB_E2E_HUB_BOOT_BUDGET_MS` (default 300s), `COLLAB_E2E_PREBUILD_BUDGET_MS` (default 1800s,
now rarely needed in full — see below), `COLLAB_E2E_DEV_BOOT_BUDGET_MS` (default 300s).

Full log: `$T/🧪️3-c-collab-e2e.txt`. Earlier iteration logs kept for the record: `🧪️3-c-collab-e2e-
run2.txt` (hit the pre-existing `semio-s-plugin-space` wasm/tokio break, before lane 4-E's fix landed),
`🧪️3-c-collab-e2e-run3.txt` (space compiled — a concurrent lane had already fixed it by then — but hit a
corrupted local Playwright browser install). Screenshots: `🧪️3-c-step1-user1.png`, `🧪️3-c-step1-
user2.png`, `🧪️3-c-step5-user1.png`, `🧪️3-c-step5-user2.png`.

## Per-step result

| # | Scenario step | Result | Evidence |
|---|---|---|---|
| 1 | user1 creates a public studio space from Home; user2's Home shows the same row | **FAIL** | `command palette has no item matching /create space/i for search "create space"` — see Defect 1 |
| 2 | user1 shares the space with user2 as author; user2 opens `/spaces/{id}` | **FAIL** (skipped) | `no space id from STEP 1` |
| 3 | user1 creates a writer artifact; row appears in both tables, opens an editor for user1 | **FAIL** (skipped) | `no space id from STEP 1` |
| 4 | user2 opens the same artifact; user1 types, user2 sees the text | **FAIL** (skipped) | `no artifact id from STEP 3` |
| 5 | `#s-presence-peers` shows 2 peers in both shells | **FAIL** | `#s-presence-peers does not exist in the React shell` — see Defect 2 |
| 6 | user1 checks in with a message; history + space table `updated` column move for both | **FAIL** (skipped) | `no space/artifact id from earlier steps` |
| 7 | admin: `/admin/api/connections` lists both connections; `/admin` returns HTML | **FAIL** | `/admin/api/connections does not mention user1: []` — see "Step 7 assessment" below |
| 8 | hub restarts against the same `OS_HUB_DATA`; user2 reloads, data survives | **FAIL** (skipped) | `no space/artifact id from earlier steps` |

**Summary: 0/8 steps passed.** Steps 2, 3, 4, 6, 8 never independently ran their own logic — they are
downstream of step 1's failure and were never separately exercised. **Nothing here was weakened to
manufacture a pass** — every assertion is exactly what the worker-brief specified (the frozen §C0
id/row grammar), and every FAIL is a real, reproduced condition, not a harness bug (see "What changed
in this lane" for the infra issues that were real harness/environment bugs, all now fixed, all logged
separately from the scenario's own FAILs above).

## Defect 1 — `createSpace` is unreachable from any UI surface today (blocks STEP 1, and by extension 2/3/4/6/8)

The Home editor's `createSpace` command (lane 2-A, `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/
🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:212-231`) is registered as `.shell_action("createSpace", …)`
plus a `.dialog(...)` and is listed in `window_kind_action_refs(S_HOME_WINDOW, …)`. It is **not exposed
by any DOM element or interaction path** that a real user (or this harness) can reach:

1. **The contract-frozen `#s-home-create-space` id does not exist anywhere.** `grep -rn
   "s-home-create-space"` across the whole repo returns zero hits (Rust, TS, everything). Compare: `#s-
   space-share`/`#s-checkin`/`#s-sync-status` DO exist as real DOM ids (confirmed by direct source
   reading, see "Verified ids that DO exist" below) — `#s-home-create-space` and `#s-space-create-
   artifact` are the two frozen ids from contract §C0 that were never actually wired to a button.
2. **The command palette (`Meta+p` / `Meta+n`-adjacent modal, `[data-slot="command-input"]`) never
   opens for Home at all.** Live-verified in this lane's own manual browser session (same build, same
   port pattern, separate from the automated harness): after `Meta+p`, `document.querySelectorAll('[data-
   slot="command-input"]').length === 0`. This is the SAME modal `SpaceE2eVerify`'s pre-existing test
   (`🔖️SpaceE2eVerify` region, this file) successfully opens for the OLD studio shell surface
   (`.semio-node-graph-host`) — it is either not wired for Home's `TableWindowKit` surface, or the
   keybinding itself isn't bound outside a studio window context.
3. **The footer "Command" panel's "App" category lists exactly one item: "Record Tutorial".** No
   `createSpace`/`deleteSpace`/`renameSpace`/`shareSpace`/`copyInviteLink` anywhere in any of the visible
   category tabs (Layout / App / Appearance / Language).
4. **Right-clicking the table surface opens a "Workspace Menu" that still only lists the PRE-TICKET VFS
   commands**: `Set Cell`, `Create Studio` (mod+n), `Bind Studio File`, `Import Studio` (mod+o), `Open
   Studio`, `menu.group.history`, `menu.group.actions`, `Delete File System Node`. None of lane 2-A's 8
   new commands (`createSpace`, `deleteSpace`, `renameSpace`, `shareSpace`, `copyInviteLink`,
   `foldDirectoryEvents`, `presenceHeartbeat`, `setClient`) appear in this menu either — this looks like
   a hardcoded/legacy context-menu item list that was never updated to include the new directory
   commands, distinct from `window_kind_action_refs` (which DOES list them, per the manifest source).

**Net read**: the manifest-level plumbing (2-A's own unit tests, which assert on the `AppCommand`
dispatch layer, not the DOM) is real and correct — `createSpace` dispatches fine if something calls it.
The GAP is entirely in the render/interaction layer that would let a human (or Playwright) actually
click, palette-search, or right-click their way to it. This is squarely the kind of "known gap" 2-A's
own report flagged ("real per-row clickable action buttons... blocked on `TableWindowKit`'s current flat
shape") extended one level further: even after 3-F's row-id/row-action fix (which DID land — the
`Demo Studio` row correctly shows a working `open` button, confirmed in the same screenshot/DOM dump),
nothing analogous exists for a WINDOW-level "create" action that isn't tied to any existing row.

**What this harness could NOT determine**: whether the intended fix is (a) wiring `createSpace` into the
existing "Workspace Menu"/command-palette machinery, or (b) adding a real toolbar button carrying
`id="s-home-create-space"` per the frozen contract grammar. Both are legitimate designs; picking one is
a product decision for whichever lane picks this up, not something to guess at from a test harness.

## Defect 2 — `#s-presence-peers` does not exist in the React shell (blocks STEP 5)

`grep -rln "s-presence-peers"` finds it only in: `👥️PresenceBar`'s own component/doc comment (both
Rust and TS twins — the element accepts a caller-supplied `id` prop, unused by default), the WGPU
`Shell/🧊️component.rs`, and ticket planning docs (`📋️master-plan.md`/`📋️contract-freeze.md`). It is
**never imported, never referenced, never rendered** in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
(confirmed: zero matches for `PresenceBar` or `s-presence-peers` in that file). `📓️w2-d-report.md`
confirms lane 2-D wired presence (`id="s-presence-peers"`, per-peer `data-row-id="peer:<actor>"`) into
the **wgpu** shell (`ui_wgpu::wgpu::build_presence_bar`) only — the wgpu renderer is a separate, still-
broken target per this ticket's own "Reality check" section, so that wiring is currently unreachable
from any browser session either way. **Nobody has wired `PresenceBar` into the React `ShellHost`.**

This is a clean, scoped, well-defined follow-up: `PresenceBar` (and its `PresencePeer[]` prop) already
exist and are exported from `@semio-tech/ui-react`; `ShellHost` already tracks live peer presence data
somewhere internally to compute `presenceClientIdentity` calls (per lane 2-C's own work) — the missing
piece is purely "render `<PresenceBar id="s-presence-peers" peers={...} />` somewhere in the chrome,"
not new data plumbing.

## Page error (both users): `TypeError: Cannot read properties of undefined (reading 'length')`

Observed via Playwright's `page.on("pageerror", ...)` on BOTH `user1` and `user2` pages in every run
that got a shell to boot (this run and the two earlier iterations). **Capture limitation, stated
honestly**: this harness's error listener only records `String(err)` (the error's message), not a
stack trace — `Error.prototype.toString()` in V8 does not include the stack, and Playwright's
`pageerror` payload wasn't captured with `.stack` in this version of the harness. So this report cannot
hand over a file/line. It fires consistently (both users, every run), is NOT one of the 8 scenario
assertions (informational only, logged separately — see `[collab-e2e] page errors observed` in the
log), and did not visibly break rendering (the screenshots show a fully-formed, interactive Home table).
Flagging precisely rather than guessing at a cause: whoever picks up Defect 1/2 should open the same
build in a headed/devtools session and reproduce this directly for a real stack trace — this harness as
it stands cannot supply one. (A follow-up to this harness itself, not a scenario defect: add
`err.stack` to the capture.)

## Step 7 assessment: consequence, not an independent defect

`GET /admin/api/connections` returning `[]` is consistent with — and, on the evidence available, best
explained by — the fact that no browser session in this run ever got far enough to open a real document
WebSocket: STEP 1 never created a space, so STEP 2/3 never opened `/spaces/{id}` for real (the harness's
own STEP 2/3 blocks were skipped), so no `sync_session` ever attached to any document, so the admin
connections list has nothing to show. **This was not independently verified as broken** — lane 3-E's own
hub-only e2e test (`os-hub-ts`, `📓️w3-e-report.md`) already proves `/admin/api/connections` correctly
lists real connections when they exist, against the same hub binary, so the mechanism itself is not
under suspicion here. Re-run this exact harness once Defect 1 is fixed; if `/admin/api/connections`
is STILL empty after a real document connection opens, that would be a genuine, separate, newly-
discovered defect — this run gives no evidence either way beyond "nothing ever connected."

## What changed in this lane (harness-level fixes, not scenario defects)

Three real infrastructure/environment issues were hit and fixed while building this harness — logged
here for transparency, distinct from the four scenario-level findings above:

1. **`buildPlugins` (non-streaming) is not resilient** — an unrelated, pre-existing `semio-s-plugin-
   animate` compile break (attributed below) aborts the WHOLE catalog build under the strict `plugin`
   subcommand. Fixed by using the resilient, per-crate-try/catch path, and — per the coordinator's own
   guidance — narrowed further to build ONLY the two crates this scenario needs (`"s"`/space and
   `"writer"`), verified via `preparePluginBuildTargets` + a direct `buildPlugin` call per target, gated
   by a hard existence check on each target's own `.core.wasm` artifact path afterward
   (`collabPluginArtifactPath`). This turned a 20-40 minute full-catalog build into a sub-2-minute one.
2. **`semio-s-plugin-space` could not target `wasm32-wasip2`** (blocked the ENTIRE scenario — neither
   Home nor Space can load without it). Root-caused precisely: `semio-s-plugin-space`'s own Cargo.toml
   (`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:37`) requests `semio-framework-os = { features =
   ["os-host-full"] }` unconditionally; `os-host-full` (`🖥️host/📦️packages/🦀️rust/Cargo.toml:62`) turns
   on `semio-framework-os-kernel/sync`, which turns on `tokio/net` — not one of the five tokio features
   (`sync,macros,io-util,rt,time`) supported on any wasm target. Confirmed pre-existing (not this
   ticket's introduction): `git diff HEAD -- ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` shows
   only lane 1-F's unrelated `user_ports` line; the `os-host-full` request itself is unchanged since
   commit `19b970280` (2026-08-11). **Fixed by lane 4-E during this session** (target-gated, per the
   coordinator) — reconfirmed here independently: `cargo check -p semio-s-plugin-space --target
   wasm32-wasip2` now exits clean, and the harness's own build produced a real `plugin-modules/s/
   semio_s_plugin_space_component.core.wasm` (44.8 MB). Not this lane's fix; just independently
   reproduced and re-verified.
3. **A separate, still-standing, unrelated defect**: `semio-framework-os-flow`'s wasm build
   (`flow_core_bg.wasm`) fails — its Cargo.toml pulls `semio-framework-ui` with `features = ["wgpu",
   "wgpu-engine"]`, which pulls `wgpu`/`vello_encoding`/`hayro-interpret`, which pull `getrandom` 0.3.4
   with no crate enabling its `wasm_js` feature for `wasm32-unknown-unknown` (the `.cargo/config.toml`
   `--cfg getrandom_backend="wasm_js"` compiler flag alone is not sufficient). Confirmed standing, not
   transient churn (`git status`/`git log --date=iso` show none of `🌊️flow/**`, `◻2d/**`, `🖱️ui/**`
   Cargo.toml mid-edit). **Not fixed here** — worked around via `FLOW_CORE_SKIP_WASM_BUILD=1` (flow-
   core's own pre-existing escape hatch), which is safe for this scenario specifically because the React
   renderer's only reference to `@semio-tech/flow-core` is a lazy `import()` inside `createFlowSession`,
   never called by Home/Space/Writer. Flow/DAG functionality itself remains unverified and broken;
   this is a real, still-open gap for whoever owns `🌊️flow`/`🖱️ui`'s wasm build.
4. Local Playwright install was corrupted (`chrome-headless-shell` binary missing under `chromium_headless_
   shell-1234/`, despite an `INSTALLATION_COMPLETE` marker) and, separately, the harness didn't pin
   `PLAYWRIGHT_BROWSERS_PATH` so it fell back to the OS-default cache (`~/Library/Caches/ms-playwright`,
   holding an older revision). Fixed: harness now sets `PLAYWRIGHT_BROWSERS_PATH` to the repo-scoped
   `node_modules/.cache/ms-playwright` (matching `SetupScript`'s own convention) before launching
   chromium; the corrupted local cache entry was reinstalled once, out-of-band, for this session.

`animate`'s own break (encountered, NOT fixed, confirmed out of scope): `error[E0063]: missing fields
chunk_sample_counts and metadata in initializer of Mp4Track` at `✏️s/🔌️plugins/🎞️animate/📦️packages/
🦀️rust/./././../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/
🦀️component.rs:1240`. Never built by this harness after the narrowing in fix #1 above (only `"s"` and
`"writer"` are ever attempted), so it costs nothing here — flagged only because the coordinator asked
for its attribution: `git log --date=iso -- <that file>` shows it last touched under the `animate`
plugin's own history, unrelated to this ticket's lanes.

## Verified ids that DO exist (contract §C0), for contrast with Defect 1

Confirmed by direct source reading (not assumed):
- `[data-row-id="space:<id>"]` / `[data-row-id^="artifact:"]` — real, lane 3-F, with working per-row
  `open`/`rename`/`share`/`delete` (Home, hub-origin rows) and `open`/`delete` (Space) buttons.
- `#s-checkin` / `#s-checkin-message` / `[data-tab-id="s-sync-status"]` — real, inside the React
  `ShellHost`'s History footer tab (`framework.panel.history`), gated on an editor session actually
  being open.
- `#s-space-share` — real, but is the members panel's "Copy Invite Link" button (spectator-role invite
  link), not a generic "share" affordance; the Home row's `share` button (title="share", no fixed id —
  per-row buttons cannot share one global id) opens the actual by-email/by-role `shareSpace` dialog this
  scenario's STEP 2 needs.
- `#s-presence-peers` — does NOT exist in React (Defect 2).
- `#s-home-create-space` / `#s-space-create-artifact` — do NOT exist anywhere (Defect 1, and its Space-
  app analogue — STEP 3 never got far enough to independently re-confirm the Space-side gap, but the
  identical "no toolbar button, table-only window" shape was directly confirmed by reading `✏️s/🔌️plugins/
  🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/
  🦀️component.rs`, which is table-only, same as Home).

## Changed files (this lane)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` — new region
  `//#region 🔖️CollabE2e` (after `🔖️SpaceE2eVerify`, before `class VerifyScript`): port scanning, hub
  spawn/wait, targeted plugin prebuild + artifact gate, two dev-server spawns, DOM helpers (command
  palette, dialogs, Radix select, row-id polling), the 8-step scenario, hub-restart step, and the
  top-level orchestrator (`runCollabE2eVerify`). `VerifyScript.run` gained a `segments[0] === "collab"`
  branch calling it. Neither `🔖️SpaceE2eVerify` nor the parity region (`🔬️ParityScript`) were touched.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json` — new
  `collab-e2e` target (`bun ./📜️script.ts verify collab`, `cache: false`), matching lane 1-F's already-
  wired `⚖️gate🌎️collab-e2e` launch entry (`bun nx run @semio-tech/framework-os-dev:collab-e2e`).

## What was NOT verified, and why

- **Steps 2, 3, 4, 6, 8's own logic** — never independently exercised; all downstream of STEP 1's real
  failure. Once Defect 1 is fixed, re-running this exact harness (unchanged) will exercise all of them
  for real.
- **The Space app's own `#s-space-create-artifact` gap** — inferred from source (table-only window, same
  shape as Home) rather than independently reached via the browser, since STEP 3 never got that far.
- **Flow/DAG functionality** — deliberately unbuilt this session (`FLOW_CORE_SKIP_WASM_BUILD=1`); still
  broken, per fix #3 above.
- **wgpu shell** — out of scope per the brief (React-only lane) and confirmed still broken by the
  coordinator's own message this session.
- **The page-error's root cause** — flagged with the evidence this harness could capture, not chased
  further (no stack trace available from this harness's current error listener).

Ticket not closed (coordinator owns that). Harness is committed, runnable today, and — once Defect 1 is
fixed — should progress meaningfully further on a re-run with no changes needed to this lane's own files.
