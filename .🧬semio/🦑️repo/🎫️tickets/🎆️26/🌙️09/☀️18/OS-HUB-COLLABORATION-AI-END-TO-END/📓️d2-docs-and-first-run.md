# D2 — Docs and first-run experience (G12 #5, #10, #11, #12)

Slice D2, session 5 (2026-09-20), cargo-free by design (machine load ≈ 150 at launch). Spec:
`📓️g12-deploy-and-onboarding-audit.md` §A/§C/§D + ranked slices #5, #10, #11, #12. Facts cited from
`📓️au2-os-sign-in-and-spaces-ui.md`, `📓️au3-live-sign-in-integration.md`,
`📓️m4-mcp-bridge-approval-binding.md`, `📓️m7-live-agent-bridge-loop.md`,
`📓️p4-hub-and-mcp-production-readiness.md` (sibling, read near the end) — and, for every env var,
default and path, from the source read directly.

## 0. Docs convention (decided, not invented)

`AGENTS.md` says nothing about docs placement (`grep -n -i "docs/|documentation|README" AGENTS.md`
→ 0 hits). The tree's own convention is unambiguous from what is tracked:
`git ls-files | grep -i "readme\.md$"` outside tickets returns **58 files, every one of them a
module-root `README.md`** — `✏️s/🔌️plugins/<plugin>/README.md` (12),
`🧰️framework/🛍️products/🦑️repo/🔨️modules/<module>/README.md` (~30),
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md`, plus the root `README.md`. There is **no
`docs/` tree anywhere** in the tracked repo.

So: no parallel docs tree was invented. Two module-root READMEs:

- `🌎️hub/README.md` — **new**. `🌎️hub` is a top-level module that had no README; every sibling
  module of comparable weight has one.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md` — **extended**, not replaced. G12 §C is
  right that it is a developer README; the end-user material is a new section inside it rather than
  a second file, because a second file for the same module would be exactly the parallel tree the
  slice forbids.

Language: **en only**. The en+de constraint G12 §D documents is a *UI string* constraint
(`["en","de"]` locale lists validated in the shell/manifest/config schemas); no tracked `README.md`
in the repo has a German counterpart, and the root README is en. The first-run tour in §4 is en+de
because it is UI.

## 1. Operator hub deployment doc (G12 #5 + #11)

**Landed:** `🌎️hub/README.md` (new file, 340 lines).

Everything in it was read from source in this slice, not carried over from another report. Capture:
`🗑️generated/d2-env-survey.txt`.

### 1.1 Correction to G12 §A — the hub cannot boot unsupervised, and production mode is unreachable

G12 §A states *"`OS_HUB_BIND` defaults to `0.0.0.0` — i.e. a naive `os-hub` launch already listens on
every interface, not just loopback, with no TLS and no warning."* **That is not what happens.** Read
`validate_auth_startup` (`🌎️hub/🏗️bootstrap/🦀️.rs`) and `main`:

- `HubMode::from_environment(bind)`: no `OS_HUB_MODE` + loopback bind → `Development`; no
  `OS_HUB_MODE` + **non-loopback** bind → `Production`. So the default `0.0.0.0` selects production.
- `Production` requires `verifier.is_some()`. `main` has
  `let identity_verifier: Option<Arc<dyn IdentityAssertionVerifier>> = None;` — a hardcoded `None`,
  the only assignment in the file. **No environment can satisfy it: production mode is unreachable
  in this build.**
- `Production` *also* refuses a non-loopback bind outright
  (`"production cleartext HTTP/WebSocket may bind only to loopback"`), so even with a verifier the
  binary would never serve cleartext off-host.
- `Development` refuses a non-loopback bind too, and additionally requires a
  `LocalBootstrapTransport`, which `🌎️hub/🚀️local-bootstrap/🦀️.rs` obtains from **inherited fd 3**
  (`INHERITED_BOOTSTRAP_DESCRIPTOR: i32 = 3`, then a keyed hello exchange).

A bare `os-hub` started by systemd/docker/a shell therefore exits, in every configuration. The one
launcher that works is `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts`'s `startLocalHub`, which spawns
with `stdio: ["ignore", out, out, "pipe", …]` and forces `OS_HUB_MODE=development`,
`OS_HUB_BIND=127.0.0.1`. The doc leads with this table rather than burying it, and the systemd unit
supervises the **launcher** (`bun nx run os-hub:dev`), not the binary — anything else would be a
unit that cannot start.

### 1.2 Env-var table — 24 variables, not G12's 14

G12 cites G2 §9's count of 14. The full set read in this slice is **24** (`std::env::var` /
`var_os` sites across `🌎️hub/**/🦀️.rs`), with defaults taken from the literal fallbacks:

| group | variables |
|---|---|
| process/mode (10) | `PORT` (8787), `BIND` (`0.0.0.0`), `MODE` (inferred), `DATA` (`./.🧬semio/🌐hub/`), `ADMIN_SUBJECTS` (empty, `provider:subject`, max 64), `ADMIN_DIR` (compile-time admin `📤️dist`), `EXTENSIONS_DIR` (`{DATA}/extension-modules`), `MERGE_POLICY` (`normal`), `ARTIFACT_CAS_SWEEP_EXECUTE` (`false`), `TEST_INFERENCE_CHECKPOINT_FD` (test-only) |
| document store (6) | `STORAGE_BACKEND` (`fs` → `{DATA}/db`), `DB_SQLITE` (`{DATA}/db.sqlite3`), `DATABASE_URL`, `NEO4J_URI`, `NEO4J_USER` (`neo4j`), `NEO4J_PASSWORD` (empty) |
| directory (5) | `DIRECTORY_BACKEND` (`sqlite`), `DIRECTORY_DATABASE_URL`, `DIRECTORY_NEO4J_URI`, `DIRECTORY_NEO4J_USER` (`neo4j`), `DIRECTORY_NEO4J_PASSWORD` (empty) |
| sign-in (3) | `CREDENTIAL_SIGN_IN` (`false`), `SESSION_TTL_SECONDS` (`43200` = `DEFAULT_SESSION_TTL_SECS`, range `60..=31536000`), `PASSWORD_ITERATIONS` (`210000` = `password::DEFAULT_ITERATIONS`, range `1000..=999999999`) |

Two facts the doc records that no prior report did:

- `OS_HUB_STORAGE_BACKEND` is read **twice** — `connect_db` and `connect_artifact_cas` — so the
  document store and the artifact chunk CAS always share one backend.
- The SQLite **directory** path is *not* configurable: always `{OS_HUB_DATA}/directory.db`
  (`connect_directory` joins it unconditionally). Only the *document* store's sqlite path
  (`OS_HUB_DB_SQLITE`) is overridable. H1 §6.2's observed layout is confirmed, and the doc states
  which half an operator can move.
- `OS_HUB_TRUSTED_CATALOG_BUNDLE`, `OS_HUB_TRUSTED_CATALOG_PROFILE`, `OS_HUB_ADMIN_TOKEN` appear in
  the launcher's env-scrub list but are read by **no** Rust code under `🌎️hub` — vestigial, flagged
  as such.

### 1.3 The rest of the doc

- **Data-root layout**: `db/`, `directory.db`, `extension-modules/`, `trusted-catalog/current.json`
  + `generations/<id>/`, with the `sqlite`/`postgres`/`neo4j` variations and which ones the file
  backup does *not* cover.
- **First user**: the `os-hub credential set --email … [--display-name …]` verb with the stdin-only
  password rule, the idempotency + session-revocation semantics and the argument validation, read
  from `🌎️hub/🔐️auth/📤️command/🦀️.rs` (module doc + `selected`). Also records that the account is
  useless over HTTP unless `OS_HUB_CREDENTIAL_SIGN_IN=true`, which AU3's report does not spell out.
- **Trusted catalog**: the second operator verb `os-hub trusted-catalog publish < command.json`,
  which requires an **absolute** `OS_HUB_DATA` (`📤️command/🦀️.rs` filters on `is_absolute`), and the
  `trusted-catalog-never-published-in-this-data-root` readiness reason.
- **Health**: `/healthz` (`semio.hub.liveness/v1`, always 200 while serving) and `/readyz`
  (200/**503**, `blocked_by: [{gate, reason}]`) — read from `get_healthz`/`get_readyz`. States
  plainly that there is no metrics endpoint and no request tracing.
- **Backup/restore runbook**: stop → `tar` the whole `OS_HUB_DATA` → start → `curl /readyz`;
  restore into an *empty* path; the postgres/neo4j caveat; and the explicit "no cross-version
  upgrade path" quoting the `📇️directory/🐘️postgres/🦀️.rs` design comment.
- **Reverse proxy** (G12 #11): Caddy and nginx, both forwarding to `127.0.0.1:8787`, both carrying
  the WebSocket upgrade. The nginx example includes the `map $http_upgrade $connection_upgrade` block
  and the `proxy_set_header Upgrade/Connection` pair with a note naming the three socket route
  families that break without them (`/directory/socket/v1`,
  `/directory/spaces/{space}/documents/{doc}/socket/v1`,
  `/spaces/{space}/documents/{id}/socket/v1` — grepped from the router, not assumed), plus long idle
  timeouts. A "what the proxy does *not* fix" subsection covers the reflect-all-origins CORS posture
  and the fact that the hub does not read `X-Forwarded-For`, so its per-remote-address rate limiting
  sees only the proxy.
- **systemd** (G12 #11): a unit with `ProtectSystem=strict` + `ReadWritePaths`, `KillMode=mixed`,
  `TimeoutStopSec=30s`, and an explicit note that `OS_HUB_MODE=development` is the only mode that
  boots and describes the auth topology, not a debug build.
- **Known gaps, stated plainly**: eight bullets — no standalone daemon, no in-process TLS, no CORS
  allowlist, no signal handler, no metrics/tracing, no upgrade path, no container/CI/publish, and
  release profile never proven.

## 2. End-user semio MCP doc (G12 #10)

**Landed:** a new `## Using it from your own MCP client` section in
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md`, inserted after `## Run it` (≈150 lines).
The existing developer content is unchanged.

### 2.1 Which binary exists today — stated honestly

Re-verified in this slice, **still true, P4 has not changed it yet**:
`🎚️config/🧱️binary-gate.json` is `{"cargoPackage": "semio-framework-os-mcp", "cargoBinary":
"semio-os-mcp", "profile": "debug", …}`, and `📦️packages/🦀️rust/📜️script.ts:27` throws if
`binaryContract.profile !== "debug"`. The build call is
`buildCargoArtifacts(…, ["--package", MCP_CARGO_PACKAGE, "--bin", MCP_BINARY_NAME], …)` — no
`--release`. The doc says so in those words, names the staged path
(`…/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`), records the `SEMIO_OS_MCP_BIN` override, and
states there is no npm/Homebrew/release/signing path, so "installing" means clone + bun + build.

### 2.2 `--hub` cannot be used from a client config — measured, not inferred

G7/M4/M7 report `--hub` as "real code with zero production call sites." This slice found the
mechanical reason, which the doc states: `🌉️mcp/🦀️.rs:740` reaches `open_hub` only via
`claimed_local_hub_credential("mcp")`, whose value comes from
`claim_inherited_local_hub_credential` →
`LocalHubCredential::read_inherited` (`📇️directory/🪪️identity/🦀️.rs:78-80`: *"Claims, seals,
consumes, and closes fd3 before any plugin or renderer activation"*). An MCP client spawns the
server with ordinary stdio and **no fd 3**, so `--hub` is unreachable from any
`.mcp.json`/`claude_desktop_config.json` — not merely unwired. `--folder` is the mode that works.
The doc names this as the single gap between "your assistant edits a folder on your machine" and
"your assistant joins your team's space."

### 2.3 What else the section covers

- **Tool surface in user terms**: a task → tools → effect table over the real loop
  (`capabilities_search`/`describe`/`context_resolve` → `artifact_open`/`create` →
  `action_prepare` (true dry run) → `action_invoke` → `transaction_begin|commit|rollback` →
  `artifact_snapshot` → `history_undo`/`redo` → `inference_run` / `inference_submit|events|cancel` →
  `artifact_export`/`validate`).
- **Approvals** in user terms (client elicitation → in-shell dialog → refusal naming why), plus the
  honest limit that **no descriptor declares `destructive: true`**, so the gate cannot fire and
  `--auto-approve` is the control that actually matters today (M7 §5.4's finding, carried into the
  user-facing doc rather than left in a ticket report).
- **Rendezvous**: `~/.semio/agent/bridge/sessions/` discovery, the `0600` offer in `offers/<pid>.json`,
  what works once bridged (edits visible, `ui_focus`/`ui_reveal`, presence, approval dialogs), the
  typed `PLUGIN_UNAVAILABLE` when no session is live, and the operational consequence: **start semio
  first, then the client**. Same user, same machine.
- **`--scopes`** with the real scope vocabulary grepped from the crate.
- **Two worked configs**: Claude Code `.mcp.json` (the `bun` + `cwd` form, with a note that `cwd`
  matters because `bun ./📜️script.ts` resolves against the repo root) and Claude Desktop
  `claude_desktop_config.json` (both OS paths; absolute-binary form, because Claude Desktop takes no
  `cwd`). Both point `--folder` at the *user's own* directory, explicitly not `.` — G12 §C's
  complaint that every shipped config binds the repo checkout itself.
- **Checking it works**: bare invocation prints the usage line (the parser's own error string),
  `audit --folder <dir>` as a no-workspace no-shell smoke, and how to read the two typed failures.

## 3. README "Products" getting-started

**Landed:** `README.md` §🛍️ Products — the four product blurbs G12 §D calls "aspirational marketing
copy" now each carry a **Getting started** block. Demo images, headings and anchors are unchanged, so
the existing TOC links still resolve; the Grasshopper/Rhino/Wasp/Monoceros/Ladybug entries below them
were not touched (out of slice).

| product | outcome | what was added |
|---|---|---|
| ✏️ sketchpad | 1 — `s` frontend | the real two-step `activate-s-react-dev` then `serve s react dev` (with `SEMIO_RENDERER=react S_OS_PORT=6060`), why activation is slow the first time, why `dev s` is *not* the command to use (it re-activates on every peer edit and takes the server down — the project memory `feedback-dev-variant-watches-all-split-activate-serve`), and the advice to substitute a single plugin name for `s` for a first look |
| 👥️ studio | 2 + 3 — hub, two users | `os-hub:dev-secure-suite` / `os-hub:dev`, linking to `🌎️hub/README.md` and telling the reader to read its opening section before planning a deployment |
| ☁️ cloud | 2 — durable storage | that storage *is* the hub's own store, `OS_HUB_DATA` as the one directory, the four backends, linking to the hub doc for the connection variables |
| 🤖️ assistant | 4 — MCP | that the assistant is the reader's own AI client, not a model this project ships; `@semio-tech/framework-os-mcp-rs:build`, linking to the MCP README's new end-user section |

A status note above the four (`README.md`) states there is no installer, container image, package or
hosted instance, that `.github/workflows/` is empty and the root `publish` map is empty, and points
at `npm run setup` — so the section cannot be read as a claim that any of this is shipped.

Commands are the ones this ticket actually uses (`📜️b3a-activate.sh` / `📜️b3a-serve.sh` shapes,
`os-hub:dev*` from `📋️project.json`), not invented ones.

## 4. First-run hub tour inside `s` (G12 #12)

**Landed**, built on the existing `IntroductionDefinition` mechanism exactly as the slice required —
no second onboarding subsystem. Split the way AU2 split its own work (pure contract ← thin pane), so
the tour is testable without a hub, a transport or a mounted shell.

### 4.1 Contract — `📇️directory/🎓️first-run/🟦️.ts` (new)

Pure: no React, no DOM, no storage handle of its own; the seen flag enters through the same injected
`HubConnectionStorageV1` port AU2's sign-in contract uses.

- **One tour, five steps, stage order**: `welcome → signIn → space → invite → done`. The definition
  is **state-independent** (a function of locale only) and only the *entry step* is derived from live
  hub state (`hubFirstRunStepIndexV1`). That is what lets the tour teach the whole flow while still
  opening on whatever the person has not done — and it makes the contract trivially testable.
- **`hubFirstRunStageV1`**: not signed in → `signIn`; signed in with no space *or* no open space →
  `space`; in a space and allowed to invite → `invite`; in a space but **not** allowed to invite
  (viewer/guest) → `done`, so the tour never teaches a control that would refuse the person.
- **Anchors are the real controls**, read out of `🔐️HubSignIn`/`🏘️SpaceBrowser`: `os.hub.signIn.email`
  (+ password/submit/hub), `os.hub.spaces.createName` (+ createSubmit/list/`os.hub.invite.redeemField`),
  `os.hub.invite.create` (+ role/expiry/copy). The space step declares **two unordered** interactions
  — create *or* redeem — because those are genuinely alternatives.
- **en + de, no default language.** `hubFirstRunTextV1("fr")` throws `hub.first-run.locale-unsupported`,
  matching `hubSignInTextV1`'s refusal rather than silently downgrading to English.
- **Seen flag** `semio.os.hub-first-run.v1`: `hubFirstRunSeenV1`/`markHubFirstRunSeenV1` both swallow a
  throwing or absent store (private mode, blocked origin, full quota) — the cost is replaying the tour,
  which is strictly better than first-run breaking the shell. `hubFirstRunShouldStartV1` additionally
  refuses to auto-start while the hub surface is closed, so it cannot ambush someone working locally.

### 4.2 Pane — `🧱️elements/🎓️HubFirstRun/🟦️.tsx` (new)

~55 lines. Renders `UIIntroduction` and nothing else — the glass, veil, cutout, checklist,
drag-to-move info box, keybindings and ghost-cursor demonstration all come from the shell's existing
introduction surface, so this element adds no chrome of its own and inherits that surface's a11y and
layout behaviour rather than re-implementing it. Props are `locale`, `state`, `onDismiss` and an
optional `initialStepIndex`; no transport, no storage, no hub. The step index is local state seeded
once from hub state, so a peer's invite landing or a session expiring mid-read cannot yank the person
to a different step (there is a law for this).

**a11y and phone width**: the two screen steps (`welcome`, `done`) carry `introduce: null` +
`placement: "center"`, so they render as a centered full-veil box on any viewport — the one shape that
cannot be pushed off a phone screen. The three task steps anchor to live controls, and
`UIIntroduction` degrades an unmounted anchor to a centered screen step, which is what keeps the tour
usable at phone width where the spaces list and the invite pane are never on screen together. Both
properties are asserted (`gives the two screen steps no anchor and centers them, so they survive a
phone viewport`). Everything else — focus handling, the `ui.introduction.{next,back,skip}` controls
and their keybindings — is the shared surface's, not re-implemented here.

### 4.3 Mount — the tour reaches a real person (session 5b)

Mounted in **`🧱️elements/🔗️HubConnection/🏛️workspace/🟦️.tsx`** (`HubWorkspace`), not in `ShellHost`.
That choice is deliberate and is what made the mount safe and cheap:

- `ShellHost` already renders `HubWorkspace` (AU2's `/hub` overlay block), so mounting one level down
  reaches the real shell anyway — **proved live**, §5.4: the probe drives the served shell, clicks the
  footer connection badge, and the tour auto-starts.
- `🏛️ShellHost/🟦️.tsx` was being edited by C1c (sign-in re-assembly) and S2 (`/hub` host-mode gate)
  at the same moment. `HubWorkspace` is an 82-line leaf nobody else was in. Zero conflict, and the
  diff is one import pair plus ~10 lines.

What the mount does:

| requirement | how |
|---|---|
| shown once per user | `useState(() => hubFirstRunSeenV1(port.storage) ? null : "auto")` — decided **once at mount**, never re-derived, so neither a later render nor an expiring session can reopen it |
| when the hub workspace is first reached | the state lives in `HubWorkspace`, which only exists while that surface is open — the tour cannot ambush someone working locally |
| signed-out or without a space | entry step from `hubFirstRunStateFromV1(hub.session.phase, hub.rows, activeSpaceId)` |
| dismissible | `UIIntroduction`'s own Skip (`ui.introduction.skip`); `onDismiss` marks seen and unmounts |
| re-openable from a help affordance | a `graduation-cap` ghost `Button id="os.hub.firstRun.replay"` in the workspace header beside Close; ignores the seen flag and restarts at step 0 |
| persisted per user like other tours | **changed from the first draft** — see 4.4 |
| en + de | new `os.hub.firstRun.replay` key in both halves of the `registerUiTranslationBundles` bundle (both locales are a compile-time requirement there) |
| keyboard + screen reader | a real `<button>` with `aria-label`; the tour's own controls are `UIIntroduction`'s, which already carry `useControlKeybinding` for skip/next/back |
| phone width | header is `flex-wrap`; the two screen steps are centered and anchorless — **measured live at 375 px**, §5.4 check 8 |

### 4.4 Persistence corrected to the shared introduction key space

The first draft invented `semio.os.hub-first-run.v1` / `"seen"`. The coordinator's "persisted the way
other intro tours persist completion" is a real correction: the shell already has
`UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX = "ui.introduction.seen."` with
`readStoredIntroductionSeen`/`writeStoredIntroductionSeen`, value `"true"`.

The contract now writes **`ui.introduction.seen.os.hub` = `"true"`** — the same key space, so
"has this person been introduced to X" and any clear-introductions action find it. It restates the
key rather than importing those helpers (it is pure TS with no React dependency), and **four laws
cross-check the restatement against the real helpers in both directions**, including that it does not
collide with another app's flag. That is what stops the restatement drifting.

### 4.5 The anchors were fiction — found by looking, not by testing

`HUB_FIRST_RUN_ANCHORS_V1` was built from `os.hub.signIn.*` / `os.hub.spaces.*` / `os.hub.invite.*`
strings read out of the two panes. **Those are i18n label keys, not DOM ids.** The panes' real ids are
`useId()`-derived (`${id}-email`). A dedicated probe (`🐍️d2-anchor-probe.mjs`) against the live
workspace answered **0/12 anchors resolve** — the tour was five centered screen steps highlighting
nothing, which is exactly the silent degradation §7.2 warned about, now confirmed as a real defect
rather than a hypothetical one.

Fixed with the mechanism the repo already owns for this: `elementIdSelector` resolves
`[id="x"], [data-element-alias~="x"]`, and `🆔️ElementId`'s own doc calls the alias "the single
logical-id → element resolver every consumer (introductions, tests, tutorials) should use". So every
anchor element got a `data-element-alias` (or an `id` where it is a `Button`, which forwards one):

- `🔐️HubSignIn/🟦️.tsx` — `email`, `password`, `hub` aliases; `id="os.hub.signIn.submit"` on submit.
- `🏘️SpaceBrowser/🟦️.tsx` — `createName`, `redeemField`, `list`, `invite.role`, `invite.expiry`
  aliases; ids on `spaces.createSubmit`, `invite.create`, `invite.copy`.

Re-probed live: **5/12 resolve while signed out — and that is the correct answer.** All four sign-in
anchors resolve, which is the step a signed-out person is on. The other seven are stage-gated: the
spaces and invite panes only mount once signed in, which is exactly when their steps run. The
screenshot shows the email field cut out of the veil with the info box anchored under it.

Both aliased panes are AU2's, so their suites were re-run: **111/111 across HubSignIn (31),
SpaceBrowser (24) and HubFirstRun (56)** — capture `🗑️generated/d2-hub-suites.txt`.

## 5. Verification

### 5.1 vitest — 45 → 56 laws, all green

```
cd …/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
SEMIO_TEST_LEVEL=long NX_DAEMON=false bunx vitest run \
  --config ../../🧪️tests/🎚️config/🟦️.ts 🎓️HubFirstRun --silent=false --reporter=verbose
```

`Tests 45 passed (45)` for the file alone (capture `🗑️generated/d2-firstrun-vitest.txt`, exit 0), and
`Test Files 3 passed (3) · Tests 111 passed (111)` for the three hub suites together
(`d2-hub-suites.txt`, exit 0). Narrowest runs, nx bypassed (rule 16).

Added over the first draft: 4 key-space cross-checks against the real
`readStoredIntroductionSeen`/`writeStoredIntroductionSeen`, 6 `hubFirstRunStateFromV1` projection
laws, and **7 mount laws driving the real `HubWorkspace`** (auto-start on a fresh profile at the
sign-in step; no auto-start for someone who saw it; storage refused → opens once, still dismissible;
dismissal persists into the shared key and survives a remount; help affordance replays from step 0;
accessible name in en **and** de; the affordance is a real enabled `<button>`).

`SEMIO_TEST_LEVEL=long` is required — the config's `include` is `[quickTestSuite]` below that, so a
plain run answers "No test files found".

**Two real failures caught by running, both fixed:**
1. jsdom has no `ResizeObserver`; every pane law died. Fixed with the repo's existing shim pattern.
2. The four mount laws queried `getByText("Sign in")`, which the workspace's own submit button also
   matches — "Found multiple elements". Rewritten to read
   `[data-slot="introduction-info-box-title"]`, the tour's own title node. A text query that
   ambiguous is a law that would have passed for the wrong reason.

### 5.2 Typecheck — 0 errors in D2 files

One foreground `bunx tsc --noEmit -p tsconfig.json` (rule 23b), ~80 s, capture
`🗑️generated/d2-typecheck.txt`: 161 errors total, **0** matching `HubFirstRun`/`first-run`/
`HubSignIn`/`SpaceBrowser`/`HubConnection`. All 161 are pre-existing and in unrelated trees
(`♻️mit-bestand/🧺️demonstrator`, plugin `🧬️schema` document-contract tests).

### 5.3 Live — observed in a real shell against a real hub

**Not test-only.** C1c's hub (7501, `/readyz` answers `mode: development`, `publicSessionIssuance:
true`) and React serve (7502) were both up. `🐍️d2-firstrun-probe.mjs` drove headless Chromium
(`--use-angle=metal`, per the SwiftShader memory) through the real boot: load 7502 → wait for
`[data-semio-hub-connection]` → click the footer badge → `[data-semio-hub-workspace]`.

```
PASS 1 the walkthrough auto-starts in the hub workspace
PASS 2 it opens on the sign-in step (signed out) — "Sign in"
PASS 3 the help affordance is present
PASS 4 the step body rendered
PASS 5 skip closes the walkthrough
PASS 6 dismissal persisted into the shared introduction key — "true"
PASS 7 the help affordance replays from the first step — "What a hub gives you"
PASS 8 the info box fits a 375px viewport
ALL PASS
```

Capture `🗑️generated/d2-firstrun-live.txt` (exit 0). Screenshots:
`🗑️generated/d2-firstrun-live.png` (1440×900, email field cut out of the veil, info box anchored
under it, checklist row "Sign in to a hub", step counter `2 / 5`) and
`d2-firstrun-live-phone.png` (375×812). Check 6 reads `localStorage` in the page, so the shared-key
persistence is proven in a real browser, not only against a fake port. Check 8 measures the info
box's bounding box against a 375 px viewport — the phone-width claim is measured, not asserted.

### 5.4 What is still NOT verified

The **space** and **invite** steps have never been seen with their anchors live: that needs a signed-in
session in a space, which needs a seeded credential on 7501 (`os-hub credential set`) — a hub-side
setup this cargo-free slice did not do. Their anchors are the same `data-element-alias` mechanism the
four sign-in anchors are now proven to use, so the risk is low, but it is untested and says so here.

## 6. wgpu parity debt (for the second coordinator's WG6 lane)

`🎓️HubFirstRun` is **React-only**, as the slice directed ("React now"). Concretely for WG6
(`📓️wg6-wgpu-hub-sign-in-and-spaces.md`):

1. **The contract is already renderer-agnostic and should not be re-authored.**
   `📇️directory/🎓️first-run/🟦️.ts` is pure TS with no React import; a wgpu twin consumes
   `hubFirstRunIntroductionV1(locale)` + `hubFirstRunStepIndexV1(state)` and renders the steps itself.
   Duplicating the text tables would create exactly the en/de drift the locale laws exist to prevent.
2. **There is no wgpu `UIIntroduction`.** React gets the glass/veil/cutout/checklist/demonstration
   surface for free from `🖱️ui/🎯️targets/⚛️react`. The wgpu target has no counterpart, so WG6 must
   either build one or render the tour as a plain retained panel — a real cost, and the reason this
   slice did not attempt both renderers.
3. **The anchors presuppose the React panes.** `os.hub.signIn.*` / `os.hub.spaces.*` / `os.hub.invite.*`
   are ids `🔐️HubSignIn`/`🏘️SpaceBrowser` render. G9 found wgpu has **no hub sign-in/spaces UI at
   all** — so until WG6 lands those panes there is nothing for a wgpu tour to point at, and
   `HUB_FIRST_RUN_ANCHORS_V1` is the list of ids its panes must publish to make the tour work.
4. **Ordering**: WG6's panes first, then the wgpu tour. A wgpu tour built before the panes would be
   five centered screen steps highlighting nothing.

There is a `🎯️targets/🧊️wgpu/🦀️.rs` twin under `🔐️HubSignIn` and `🏘️SpaceBrowser`;
`🎓️HubFirstRun` has **no** such twin, and that absence is this slice's parity debt.

## 7. Honest gaps

1. **No command-palette entry.** The tour is mounted and live-observed (§4.3, §5.3), but the only way
   to it is the hub workspace itself — its auto-start or its help affordance. There is no
   `os.introduceHub` beside `os.introduceApp`, because that command table lives in `🛠️ShellHelpers`
   and the gain over the in-surface affordance is small. A one-row addition whenever someone wants it.
2. **Anchor drift is still silent, not red — and it already bit once.** The laws check anchors against
   `ELEMENT_ID_PATTERN` and the `os.hub.` namespace, not against rendered DOM. That is precisely how
   12/12 anchors came to be i18n label keys that resolved to nothing (§4.5). The live probe
   `🐍️d2-anchor-probe.mjs` is the real guard and it is **not wired into any suite** — someone renaming
   a control in `🔐️HubSignIn`/`🏘️SpaceBrowser` will silently degrade the tour again. Wiring that probe
   into a gate is the honest follow-up.
3. **The space and invite steps' anchors are unobserved** — see §5.4.
4. **No runtime observation of any doc'd hub command.** This slice is cargo-free by design (load ≈ 150).
   Nothing in `🌎️hub/README.md` was executed here: not `os-hub credential set`, not
   `trusted-catalog publish`, not the Caddy/nginx configs, not the systemd unit, not the backup
   runbook. Every claim is a source read, cited to the file it came from. The boot-gate table in
   particular is derived from `validate_auth_startup` + `main` + `startLocalHub`, not from trying to
   boot the binary and watching it refuse.
5. **The reverse-proxy and systemd examples are standard boilerplate adapted to measured facts** (the
   real port, the real socket routes, the real mode requirement, the real drain). They have not been
   loaded by Caddy, nginx or systemd. Treat them as correct-by-construction starting points.
6. **G12 §A's `OS_HUB_BIND` claim was contradicted** by §1.1 above. I did not edit G12 (another slice's
   report); the correction lives here and in the hub README's opening table.
7. **The MCP doc's tool table is user-facing framing, not a verified transcript.** The tool names and
   the approval lanes are read from the crate and its README; M7 proved 8/11 of a live loop, and the
   three unproven steps are the approval ones, which the doc states as its own honest limit.
8. **P4/M6 reconciliation is a moving target.** P4's report was still a skeleton (every item ⬜)
   when read, so landing was verified from **source**, not from its report — see §8.2. Items it lands
   after this report was written will leave the three docs stale in exactly the places §8.2 names.

## 8. Files changed

### 8.1 D2's own

| file | change |
|---|---|
| `🌎️hub/README.md` | **new** — operator deployment doc (G12 #5 + #11). P4 co-owns it now; D2 rewrote the systemd section for production mode in session 5b |
| `…/💻️os/🔨️modules/🌉️mcp/README.md` | new `## Using it from your own MCP client` section (G12 #10) |
| `README.md` | §🛍️ Products — status note + getting-started for the four outcomes |
| `…/📇️directory/🎓️first-run/🟦️.ts` | **new** — contract; shared intro key space, `hubFirstRunStateFromV1` |
| `…/🧱️elements/🎓️HubFirstRun/🟦️.tsx` | **new** — pane |
| `…/🎓️HubFirstRun/🧪️tests/🧩️component/🟦️.tsx` | **new** — 56 laws incl. 7 mount laws |
| `…/🎓️HubFirstRun/📖️stories/🧪️.story.tsx` | **new** — 6 stories |
| `…/🔗️HubConnection/🏛️workspace/🟦️.tsx` | **the mount** — auto-start state, dismiss→persist, help affordance |
| `…/🔗️HubConnection/🟦️.tsx` | `os.hub.firstRun.replay` label in **both** locale halves |
| `…/🔐️HubSignIn/🟦️.tsx` | 3 `data-element-alias` + 1 `id` so the sign-in anchors resolve (§4.5) |
| `…/🏘️SpaceBrowser/🟦️.tsx` | 5 `data-element-alias` + 3 `id` for the space/invite anchors |
| `…/🎯️targets/⚛️react/🟦️.tsx` | `🔖️HubFirstRun` export region (append-only) |
| `🧰️framework/🛍️products/💻️os/🟦️.ts` | `🔖️HubFirstRun` export region (append-only) |
| `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | one line registering the suite — without it the laws never run |

Probes: `🐍️d2-firstrun-probe.mjs` (live walkthrough, 8 checks), `🐍️d2-anchor-probe.mjs` (live anchor
resolution). Captures: `d2-env-survey.txt`, `d2-firstrun-vitest.txt`, `d2-hub-suites.txt`,
`d2-typecheck.txt`, `d2-firstrun-live.txt`, `d2-anchors.txt`, `d2-firstrun-live.png`,
`d2-firstrun-live-phone.png`.

No cargo was run. No server was started (7501/7502 are C1c's, reused read-only). No process was
killed. `📌️important.md`, `🎫️ticket.json` and others' `🗑️generated` entries untouched.

### 8.2 Re-sync with P4 and M6 (verified in source, not from their reports)

| landed | source evidence | doc consequence |
|---|---|---|
| **P4 #9** CORS allowlist | `OS_HUB_ALLOWED_ORIGINS` parsed in `🏗️bootstrap/🦀️.rs` (≤32, serialized origins) | P4 wrote the env row + `## Cross-origin access`; D2's "no allowlist" gap bullet gone |
| **P4 #6** SIGTERM drain | `termination_signal()` over `ctrl_c` + `SignalKind::terminate` | D2 rewrote the backup section: both signals drain, allow ≥30 s, never `SIGKILL` |
| **P4 #3** os-mcp release | `build-release` target; gate json now `profiles: {build: debug, build-release: release}` | D2 rewrote "Which binary exists today" and repointed the Claude Desktop config at `dist/build-release` |
| **P4 production mode is now REAL** | `validate_auth_startup` admits production on `OS_HUB_CREDENTIAL_SIGN_IN=true` (no verifier adapter), and admits a **network bind** under `OS_HUB_ALLOWED_ORIGINS` + `OS_HUB_TRUSTED_FORWARDING=proxy` | P4 rewrote the opening table. **D2 rewrote the systemd unit**: it now runs `/srv/semio-hub/bin/os-hub` directly in production mode with the two network statements commented in place, plus a pre-start `credential set` step — the old unit supervised `bun nx run os-hub:dev` and would have been actively wrong advice |
| **M6** `--credential-file` | `"--credential-file"` in both `parse_stdio_args` and `parse_http_args`; `--credential-fd` refuses 0/1/2 | **D2's `--hub` section was the most stale thing in any of the three docs** — it said `--hub` "cannot be used from a client config today". Replaced with a three-way credential table naming `--credential-file` as the client-config path, a worked `args` block, and the `0600` warning |

My session-5 finding that production mode was unreachable (hardcoded `identity_verifier: None`) was
**correct when measured and is now obsolete** — P4 fixed the cause rather than the symptom: the real
requirement was *an identity authority*, and the hub's own credential sign-in is one. `main` still
passes `None`; the verifier is now the optional external-IdP seam it always should have been.

Final stale-claim sweep across all three docs (`only mode that boots`, `supervises the launcher`,
`cannot be used from a client config`, `No CORS allowlist`, `no SIGTERM`): **0 hits.** Anything P4 or
M6 lands after 06:30 will leave those same places stale.
