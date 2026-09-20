# C2 — two users, one shared document, on the READY hub (7611)

Slice C2 (session 6, 2026-09-20). Outcome 3: **two signed-in humans in two browser contexts editing
ONE shared document over the hub.**

Inherited: GM1's hub on `http://127.0.0.1:7611`, data root `.🧬semio/🌐hub/gm1-boot`, PUBLISHED
trusted catalog — `/readyz` `status: ready`, `artifactAuthority.ready: true`, `features.openPlan:
true` (re-measured at 18:5x, runId `c2b570b1b8808c01f6af5137f89560e1`), space
`01a0c00f-4f3c-7834-a7e6-2ccf9de925db`, gis map document
`artifact-2fb248125b8b2b4d56de25933d30ed21`, surface `s.gis.gismap@1/*#editor`, window kind
`gis2d-main`, humans `user1@semio.dev` / `gm1-local-dev-pass-1` and `user2@semio.dev` /
`gm1-local-dev-pass-2`.

## 0. Headline

**The hub document socket opened for the first time in this repo** —
`ws://127.0.0.1:7611/spaces/01a0c00f-…/documents/artifact-2fb2…/socket/v1?surface=s.gis.gismap@1/*#editor`,
after `open-plan 200` → `execution-target/{manifest,component,descriptor} 200` → `socket-grants 200`,
from a signed-in human in a real browser (`🗑️generated/c2-attach-diagnose.txt`). Five product defects
had to be fixed to get there, each found by running the thing (§2).

**Seconds later the hub aborted**: `thread 'semio-pool-worker-4' has overflowed its stack / fatal
runtime error: stack overflow, aborting`, immediately after its own
`server.document.socket outcome=ok` line. It aborted a **second** time on the next run, on a
different pool worker, right after `server.auth.session.read`
(`🗑️generated/c2-hub-crash-gm1-hold.txt`, `c2-hub-crash-2.txt`). That is a hub-binary defect and
needs the coordinator's hub build — it is the single thing between here and the per-step matrix.

**Two humans, two browser contexts, one shell, both signed in against the live hub: OBSERVED**
(`STEP boot: PASS`, `STEP sign-in: PASS`, `🗑️generated/c2-shared-document.txt`). No document-level
step (live edit, undo, loss/catch-up, convergence, presence colours, restart) was observed: the
socket's first session is what crashes the hub.

The hub is **back up and ready** at hand-off (§8).

## 1. Shells bound to hub 7611 — measured

Serve only, no activation, of already-staged outputs:

| variant | port | pids | evidence |
|---|---|---|---|
| `s` (60 plugins incl. `gis`, `space`, `writer`; receipt 18:24 today) | 6190 | wrapper 42162 / vite 42307 | `/_semio/hub/readyz` → 200 with the hub's OWN `runId` |
| `gis2d` playground | 6191 | wrapper 66282 / vite 66294 | same `runId`; `/?plugin=gis2d` boots `data-semio-os-ready=gis2d` |

`📜️c2-serve.sh <variant> <port> <hubUrl>` (new, permanent) is the recipe: it exports
`S_OS_PORT` + `S_HUB_URL` and calls `bun 📜️script.ts serve <variant> react dev` — no activation, so
nothing of a peer's staging is touched.

## 2. Five product defects, found by running and fixed

### 2.1 The first-run introduction covered the very button it introduces

Every probe died on `TimeoutError: click` at the sign-in submit —
`<div data-slot="introduction-veil"> … intercepts pointer events`.

Measured (`🐍️c2-veil-diagnose.mjs`, `🗑️generated/c2-veil-diagnose.txt`): the button carried
`data-introduction-elevated` and `z-index: 10001` against the veil's `10000`, and
`document.elementFromPoint` at its own centre still answered the **veil** — a stacking-context trap
that `useIntroductionElevation`'s doc comment already names, which is why it elevates the nearest
`[data-slot="mode-dock-stack"]` or `[data-elevation-root]` ancestor. The hub workspace overlay
(`🏛️ShellHost/🟦️.tsx:11701`, `absolute top-workbench left-1/2 z-50`) is exactly such a root and was
not marked as one.

Fix: `data-elevation-root=""` on that overlay. After it, `elementFromPoint` answers
`DIV[button-group]` inside the workspace and the overlay itself carries `z-index: 10001`
(`c2-veil-diagnose-after.txt`).

### 2.2 The harness's "close the workspace" click replayed the first-run tour

`collabSignIn` ended with `[data-semio-hub-workspace] button[aria-label]` `.first().click()`. The
first such button is `os.hub.firstRun.replay` ("How this works"), so every run **re-opened the tour**
instead of closing the workspace — which is why §2.1's veil was on screen at all (screenshot
`c2-recon-user1.png` from the first run shows the tour box at step "Sign in 2 / 5").

Fix: the cancel control gained its own id (`os.hub.signIn.cancel`, matching `os.hub.signIn.submit`),
and `collabSignIn` clicks that id and waits for the workspace to hide.

### 2.3 The directory event page was refused by the browser on every hub that holds an artifact

Signed in, Home rendered `s-home-main` with zero rows and a red `Directory update stopped`. The fault
code was surfaced nowhere, so `DirectoryBootstrapStatusNotice` now also emits
`data-directory-bootstrap-code` (permanent, diagnosability). It read
`directory-bootstrap.invalid-page`.

The shell's OWN parser against the live page (`🐍️c2-event-page-parse.ts`,
`🗑️generated/c2-event-page-parse.txt`):

```
event-page status=200 bytes=4298
PARSE refused: directory-event-page.invalid-index
```

`validDocumentIndexEntryV1` (`📇️directory/🧬️schema/🟦️.ts`) validated the dialect's `subset` with the
identity grammar `/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/`, and the indexed gis map carries
`dialect.subset = "*"` — the product's canonical any-subset coordinate (same `*` as in
`s.gis.gismap@1/*#editor`, in the trusted catalog's open targets and in the hub's `parentDialect`).
One refused event refuses the whole page and `invalid-page` is **not retryable**, so on any hub that
has ever indexed a document no shell could ever list spaces or artifacts.

The **Rust twin of this law already admits it** — `📇️document-index-v1/🦀️.rs:24`,
`let subset_identity = |value: &str| value == ANY_SUBSET || identity(value)`, whose docstring says
rejecting it "would drop the presentation row of practically every indexed document". The TypeScript
side had never been given that clause. Fixed by mirroring the twin
(`DOCUMENT_INDEX_ANY_SUBSET_V1` + `subsetIdentity`). Re-measured: `PARSE ok events=5 through=8
kinds=space.created,member.upserted,document.announced,document.indexed,artifact.checkpoint-published`.

### 2.4 A `remote://` hub binding never named its surface

`attachSyncBackbone` built `[{ kind: "hub", baseUrl, spaceId }]` and nothing else. The store worker
refuses a hub binding carrying neither `requestedSurfaceId` nor an installed target
(`👷️worker/🟦️.ts:6085`) — observed live as `Error: socket actor unavailable
(installed-target-unavailable)` in both browser contexts. So C1c's `remote://` decode fix could never
have opened a hub document even with `openPlan` true.

Fix: the adapter supplies `requestedSurfaceId = canonicalSurfaceId(app.dialect, app.role)` — the same
expression `openDocument` uses when it resolves bindings itself. After it, the shell reached
`POST …/open-plan → 200` and fetched manifest + component + descriptor (all 200).

### 2.5 `undefined` crosses the worker wire as `null`, and the view-context law refused it

Next live refusal, in both contexts: `Error: view context: invalid identifier`, stack
`parseResolvedPluginViewState ← parseBrowserActorViewStateRequest ← decodeBackboneWorkerRequest ←
worker.onmessage`.

Reproduced outside the browser (`/tmp` round trip through the product's own
`encodeBackboneWorkerRequest`/`decodeBackboneWorkerRequest`): `activeUtilityId: undefined` and
`activeToolId: undefined` arrive as **`null`** — `encodePackValue` has no `undefined`. The law tested
`row[key] !== undefined` and so handed `identifier()` a `null`, refusing **every** view context that
has no active utility or tool, i.e. the ordinary case. The Rust twin's fields are `Option<String>`
(`🛂️manifest/🦀️.rs:4691`), which decode `null` to `None`, so the wire's spelling of "absent" is
`null` and the TypeScript twin was the odd one out.

Fix: `parseResolvedPluginViewState` drops null-valued entries before validating, with the reason in
the comment. Verified by the same round trip and then live (the error is gone).

### 2.6 The manual attach passed a chrome breadcrumb where the hub expects the artifact schema

With 2.4 and 2.5 fixed the open still stopped after the descriptor fetch, silently, with the shell
showing "The document target changed. Reopen the document." — the `"stale"` execution-target status,
i.e. `documentOpenPlanAuthority` threw. Temporary instrumentation in that function (added, measured,
removed) named it exactly:

```
C2PROBE authority projected=ok planSchema="gis.map" configSchema="semio.gis.2d"
        requested="s.gis.gismap@1/*#editor" installedSurface="s.gis.gismap@1/*#editor"
        sameFields=true rendererTarget=wasm leaseFields=present configPackHash=null
```

Everything matched except the schema: `attachSyncBackbone` passed
`targetSession.app.breadcrumb.join(".")` — the chrome breadcrumb ("semio · s · gis") — as the
document schema, while the hub's plan states `gis.map`. Harmless for a local document (nothing
compares it), fatal for every hub document.

Fix: the adapter states the app's own `io.artifactSchema` when it has one, breadcrumb only as the
fallback. **Immediately after it the socket-grant and the document socket opened** (§0).

## 3. The path used, and why it is not `collabRunScenario`'s

`collabRunScenario` drives everything through the Home and Space tables. Those tables are fed by the
directory projection, and with §2.3 fixed the projection advances to `Updating directory through
sequence 8` and then stops at `directory-bootstrap.receipt-mismatch`. Measured cause (temporary
instrumentation, removed): the guest's reply to `applyDirectoryEventPage` is the typed-operation
**admission**, not the receipt —

```
output   = {"generation":"0","operationId":"64"}
expected = {sessionBindingSha256 …, authorizationGeneration: 1, throughSeqInclusive: 8, receiptSha256 …}
```

`applyDirectoryEventPageBootstrapV1` (`🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx:222`) parses the
receipt straight out of `response.output`; `invocationFromFrames`
(`🔌️PluginRuntime/🟦️.tsx:3619-3622`) only substitutes the guest's receipt when the typed operation's
TERMINAL page is drained in the same invocation; and `applyDirectoryEventPage` is declared
`InteractiveJobClassification::Migrated` in the guest
(`🪐️space/…/🏠️home/…/✏️editor/🦀️.rs:678`), i.e. job-routed — it answers on a later turn through
`OperationCompleted`, which carries `operation/revision/ui_scope/history_patch` and **no output**. The
law that covers this path (`🧪️tests/📇️directory-home-bootstrap/🟦️.tsx:58`) fakes a handle that
returns the receipt from `handleAction`, so it has never exercised the real admission. **Not fixed**:
either the completion must carry the terminal output, or the verb must stop being job-routed — a
runtime/guest contract decision plus a wasm rebuild.

So the shared document was reached the other way, which needs no table: the shell's own sync card ▸
Remote ▸ `127.0.0.1:7611/<spaceId>/<documentId>` ▸ Attach — C1c's `remote://` path, which this slice
made actually work (§2.4, §2.6). Probe `🐍️c2-shared-document.mjs` (two contexts) and
`🐍️c2-attach-diagnose.mjs` (one, verbose).

### E2E baseline on the ready hub

| # | step | verdict | measured |
|---|---|---|---|
| 1 | two contexts boot the shell on the hub-bound serve | **PASS** | `user1:ready=gis2d user2:ready=gis2d`, no page errors |
| 2 | two DIFFERENT humans signed in at once, real form, real hub | **PASS** | both contexts lose `[data-semio-hub-sign-in]`; hub log mints two principals |
| 3 | open the SAME hub document through the open plan | **PASS for one human** | `open-plan 200` → `execution-target/manifest,component,descriptor 200` → `socket-grants 200` → document socket opened with `?surface=s.gis.gismap@1/*#editor` |
| 3b | the same, for the second human concurrently | **NOT REACHED** | the hub aborts (stack overflow) during/right after the first socket session |
| 4 | live edit A→B | not run | needs 3b |
| 5 | live edit B→A | not run | needs 3b |
| 6 | per-user undo | not run | needs 3b |
| 7 | connection loss on B + catch-up | not run | needs 3b |
| 8 | two-writer convergence (snapshot hashes) | not run | needs 3b |
| 9 | presence roster, distinct colours | not run | presence beats ride the document socket |
| 10 | mid-edit restart on an own second hub | not run | needs a surviving document session first |

Between step 3 and step 3b the shell reports `The verified document component is ready, but this
renderer is unavailable.` (`emitExecutionTargetStatus(…, "renderer-unavailable")`, emitted right after
`lease.admitBrowserActor`) and `Remote: backoff`, while every reopen attempt answers `open-plan 500`
— the hub process is already gone by then.

## 4. The hub crash — the one thing the next owner must fix

Reproduced twice, on two different pool workers, on the GM1 binary
(`⚡️cache/cargo/target-gm1/debug/os-hub`), same data root:

```
{"event":"server.document.socket","outcome":"ok","space":"01a0c00f-…","artifact":"artifact-2fb2…","durationUs":4782}
{"event":"server.auth.session.read","outcome":"ok",…}

thread 'semio-pool-worker-4' (23088553) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

and, on the next run, after a `server.auth.session.mint` that itself took **18.2 s**:

```
thread 'semio-pool-worker-3' (23430030) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

Captures: `🗑️generated/c2-hub-crash-gm1-hold.txt`, `c2-hub-crash-2.txt`. This needs a coordinator hub
build (rule 26 forbids this slice from building `-p semio-hub`); a pool worker's stack size is the
obvious first suspect, and the first crash's context is the **gis map document socket** — the first
document session this catalog has ever served.

**Recovery, measured:** after the abort, `📜️gm1-hub-hold.sh` could not bring the hub back — the child
printed nothing and the holder died on `hub readiness deadline exceeded — no /readyz answer was ever
received`, four times in a row. The fix is C1 §4's budget knob: with
`SEMIO_BUILD_BUDGET_MS=1800000` the same data root came up **ready in 20 s**
(`📜️c2-hub-restart.sh`, capture `🗑️generated/c2-hub-restart.txt`) — a crash-recovery boot simply
outlives the default readiness deadline. Two holder processes whose `os-hub` child had died (GM1's
39232 and this slice's 78946) were killed by pid first; nothing else of GM1's was touched and the
published catalog survived untouched.

## 5. Permanent target wiring

Not added. The scenario this slice can run end to end today stops at step 3 for one human, so an nx
target + `.vscode/launch.json` row would pin a red path as a gate. The probes are permanent and
parameterised (`🐍️c2-shared-document.mjs <shellUrl> <hubHostPort> <spaceId> <documentId>`), and the
harness change in §2.2 is inside the real `🤝️collaboration/🟦️.ts` so `collab-e2e` inherits it. Wire
the target once the hub survives a document session — the honest gate is step 3b, not step 1.

## 6. Honest gaps

- **No document-level collaboration step was observed.** Steps 4-10 above were not run, and the
  reason is one hub abort, not a scenario result. Nothing in this report claims a behaviour that was
  not printed by a live process.
- **Presence colours: not observed.** No `c2-presence-*.png` exists, deliberately.
- **The mid-edit restart on a second hub was not attempted** — restarting a second hub proves nothing
  while the first document session kills the first hub.
- **§3's receipt defect is not fixed**, so Home/Space tables stay empty and the 13-step scenario still
  cannot reach step 1.
- **Two ShellHost fixes (§2.4, §2.6) are observed; §2.1 and §2.2 are observed; §2.3 and §2.5 are
  observed both by unit round trip and live.** No fix here is claimed on reading alone.
- Nothing was rebuilt: every fix is TypeScript, picked up by vite (proved by the behaviour change on
  the same serve pids).

## 7. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts` | `validDocumentIndexEntryV1` admits the canonical any-subset `*`, mirroring its Rust twin; new `DOCUMENT_INDEX_ANY_SUBSET_V1` (§2.3) |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | `parseResolvedPluginViewState` treats a wire `null` as an absent optional field (§2.5) |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `data-elevation-root=""` on the hub workspace overlay (§2.1); `attachSyncBackbone` names `requestedSurfaceId` (§2.4) and passes the app's `io.artifactSchema` (§2.6) |
| `…/🧱️elements/🔗️HubConnection/🏛️workspace/🟦️.tsx` | the cancel control gained `id="os.hub.signIn.cancel"` (§2.2) |
| `…/🧱️elements/🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx` | the fault notice reports `data-directory-bootstrap-code` (§2.3) |
| `…/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` | `collabSignIn` closes the workspace by id instead of replaying the tour (§2.2) |

Ticket folder (not product code): `📜️c2-serve.sh`, `📜️c2-hub-restart.sh`, `🐍️c2-recon.mjs`,
`🐍️c2-veil-diagnose.mjs`, `🐍️c2-veil-rules.mjs`, `🐍️c2-directory-diagnose.mjs`,
`🐍️c2-event-page-parse.ts`, `🐍️c2-sync-card-diagnose.mjs`, `🐍️c2-attach-diagnose.mjs`,
`🐍️c2-shared-document.mjs`; captures `🗑️generated/c2-*.txt`, `c2-*.json`, screenshots `c2-*.png`.

## 8. State at hand-off

| what | pid | note |
|---|---|---|
| hub 7611 on GM1's data root, published catalog | `os-hub` **83162** | `status: ready`, `artifactAuthority.ready: true`, `features.openPlan: true`, runId `529f4b6f…`. Restarted with `📜️c2-hub-restart.sh` after the aborts of §4. |
| `s` react dev serve, port 6190, hub 7611 | 42162 / 42307 | serve only |
| `gis2d` react dev serve, port 6191, hub 7611 | 66282 / 66294 | serve only |

Every peer serve and every other hub was left alone. Two dead holder processes (39232, 78946) whose
`os-hub` children had already aborted were killed by pid — nothing that was serving anything.

**Request to the coordinator:** a `semio-hub` build/run that fixes the pool-worker stack overflow of
§4. With it, steps 3b-10 are reachable on the shells already serving, by re-running
`bun 🐍️c2-shared-document.mjs http://127.0.0.1:6191 127.0.0.1:7611` — no activation, no rebuild.
