# S4 — space → studio → foreign-kind open inside the real `s` host

Slice S4, fleet 5, started 2026-09-20 17:10. Scope handed over from S3
(`📓️s3-s-host-home-surface-and-foreign-open.md` §5/§5.1) and C1c (`📓️c1-collaboration-e2e.md`
→ `### E2E run 5`).

Outcome 1's acceptance, observed: **inside the real `s` host a signed-in user creates (or enters) a
space, reaches the studio, and opens artifacts of OTHER plugins' kinds, edits them, undoes, redoes.**

Status legend: **measured** = this slice ran it and captured output; **unverified** = read from
source only.

## 0. tl;dr

**Five root fixes landed, each found by measurement and each judged by the live shell answering a
DIFFERENT fault afterwards. Outcome 1's acceptance is still not reached, and the one remaining
blocker is named at file:line with no wasm rebuild in it (§5.2).**

| # | what was broken | evidence it is fixed |
|---|---|---|
| 1 | `applyDirectoryEventPage` was `BatchOnlyPendingRewrite`, and UI dispatch admits only `Migrated` — so the signed-in `s` shell could never acknowledge a directory page and re-fetched `after=0` ~1×/s for ever | the shell's next answer changed (§2.5) |
| 2 | its publication contract declared `HostOnly` = no store lane, but it writes the CONFIG store (`createStudio` writes the ARTIFACT store) | the shell's next answer changed |
| 3 | Home's config one-item lane was sized **16 MiB** against the store's hard **1 MiB** per-item ceiling, so it could never be admitted at all | **retry storm `31× → 2×`** in a 60 s window |
| 4 | Home declared no config/draft/transient store disposers, so every instance close faulted (`predecessor … retirement failed` in the live shell) | the close ladder now reaches the presence lane |
| 5 | the palette's `spawn.<plugin>` raised a guest action with the one controller id the shell's own interceptor excludes, so **every** foreign-kind spawn in this product was dropped | **`firstError: null`** and the canvas breadcrumb becomes `semio · drawing` (§5.1) |

`cargo test -p semio-s-artifact-space-home --lib --features component-app-assembly`: **92 passed,
0 failed** (S3's baseline on the same command: 90 passed, 2 failed).

Three separate gates were measured, each named exactly, none of them the one on file:

1. **The shell's `offline` badge and its directory retry loop are NOT a network fault.** Every hub
   request answers `200`. The loop is a **guest dispatch refusal**: `applyDirectoryEventPage` is
   declared `InteractiveJobClassification::BatchOnlyPendingRewrite`, and
   `validate_ui_dispatch_classification` admits **only** `Migrated`, so the shell's directory
   bootstrap can never acknowledge a page and re-fetches `after=0` about once a second, for ever
   (§2). Root-fixed by migrating the route onto Home's retained tool factory (§2.2).
2. **A hub space can be created and entered today** — measured: `POST /directory/commands → 202`,
   `space.created` linearised, the workspace lists the space, and clicking `Open …` navigates the
   real `s` host to `/spaces/<id>` and paints `semio · s · space · index` (§3).
3. **No artifact of any kind can be created inside a hub space on this hub**, and it is not a code
   defect: `POST /_semio/hub/spaces/<id>/documents/index/open-plan` answers **503
   `catalog-unavailable`**, because hub 7501 has **no published trusted catalog**
   (`/readyz`: `artifactAuthority.ready:false`, `features.openPlan:false`). The hub's `openPlan`
   feature is `true` iff `openable_catalog` is `Some` (§4). That is TC1's lane and ~60 min of cold
   wasm-release build **per plugin** — out of reach for this slice.
   The **local** studio path (`createStudio`, "works with no hub" by its own contract) was dead for
   exactly the same reason as (1) — `BatchOnlyPendingRewrite` — and is migrated in the same fix.

## 1. Inherited state

- serve 6071 (pid 34308, `S_HUB_URL=http://127.0.0.1:7501`) alive, `HTTP 200`; serve 6070
  (pid 26481, no hub env) alive; hub 7501 (pid 5468, binary
  `🌎️hub/📦️packages/🦀️rust/dist/build-dev/os-hub`, data root `/private/tmp/c1c-hub-eCgs`)
  `/healthz` 200, `/readyz` `not-ready` with **`artifactAuthority.ready:false`**, everything else
  ready. Measured 17:10. **Not restarted by this slice.**
- Disk: 44 GiB free on `/System/Volumes/Data` at 17:10.
- S3 landed: Home publishes signed out and signed in, `/hub` overlay no longer tears the canvas
  down, palette chord is `Meta+p`.

## 2. Why the `s` shell reports `offline` and loops the directory — measured, then root-fixed

### 2.1 The measurement

`🐍️s4-space-journey-diagnose.mjs` (new, permanent) drives the served `s` on 6071 headless
(`--use-angle=metal`, 1600×1000 — a real viewport), signs in, and records **every** request the
page makes on the hub lane with its status, plus the shell's own
`[data-directory-bootstrap]` notice element. Captures `🗑️generated/s4-journey-{1,2}.txt`.

| fact | value |
|---|---|
| `GET /_semio/hub/directory/event-page/v1?after=0` | **30× → 200** in one 60 s window |
| `GET /_semio/hub/auth/sessions/me` | 5× → 200 |
| `POST http://127.0.0.1:7501/auth/sessions` | 200 |
| shell notice | `retrying: Retrying directory update through sequence 3` |
| `s-home-main` in the DOM | true, before and after sign-in (S3's fix holds) |
| space rows on Home | 0 |

So the transport is perfect and the frontier is delivered — the page is simply never acknowledged,
and the same `after=0` page is re-fetched about once a second.

`applyDirectoryEventPageBootstrapV1`'s `} catch {` (`🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx:239`)
swallowed the reason. A temporary instrumentation there (added, measured, removed) printed it
verbatim:

```
[S4PROBE] applyDirectoryEventPage threw
  SemioFaultError: UI dispatch rejected action:applyDirectoryEventPage
                   with interactive-job classification BatchOnlyPendingRewrite
  page: {"schema":"semio.directory.event-page.v1", …, "throughSeqInclusive":3, "events":[]}
```

`validate_ui_dispatch_classification` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12788`)
admits **only** `Migrated`; everything else is `interactive-job.not-ui-safe`. `🪐️space`'s Home
declared `applyDirectoryEventPage` as `BatchOnlyPendingRewrite`
(`…/✏️editor/🦀️.rs:626`), so the route has **never** been dispatchable from a shell. Note the
symptom is present even for an **empty** page (`events: []`): the acknowledgement, not the fold, is
what the classification blocks.

`offline` in the footer is the same event seen from the other end — the shell has a verified session
authority but no directory projection it was allowed to apply.

### 2.2 The fix

`applyDirectoryEventPage` is exactly the shape Home's retained tool factory already serves — the
TypeScript side was written for it and says so: `parseDirectoryProjectionReceiptV1` is documented as
"the closed receipt record returned **only** by the retained typed-operation terminal", and
`apply_directory_event_page::handle` already emits that receipt as an `AppEvent`. The Rust side had
simply never claimed the route. Landed in
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

| change | detail |
|---|---|
| `HOME_RETAINED_TOOL_IDS` | `+ "applyDirectoryEventPage", "createStudio"` |
| `HOME_RETAINED_PUBLICATION_CONTRACTS` | two new `HostOnly` rows |
| `home_retained_extent` | now answers **(extent, ceiling)** per route: scalars keep `HOME_RETAINED_SCALAR_BYTES` (4 KiB), one sealed directory page is judged against `HOME_RETAINED_RAW_BYTES` (128 KiB) — the hub pages with `hasMore`, so a page is bounded by construction and a 4 KiB cap would refuse an ordinary page of a dozen spaces |
| `home_retained_reduce` | binds the identity it already required and routes `CreateStudio` to `create_studio::handle_with_identity`, exactly as the direct `ArtifactEditor::handle` lane does (the identity-less `handle` answers `s.home.session-identity-required` by design) |
| `bounded_first_step_tool_proofs!` | two new tool literals |
| `.action_interactive_job(…)` | `applyDirectoryEventPage`, `createStudio` → `Migrated` |
| `🧫️fixtures/🧫️retained-command-limits/🔣️.json` | both rows `Migrated` + `["HostOnly"]`, `blocker` cleared |

`createStudio` is in the same change because it is the **local** studio path and it was dead for the
identical reason. Its own contract says so verbatim
(`🎮️commands/🌱create-space/🦀️.rs:1-6`): *"the local-only 'create ephemeral studio' path
(`create-studio`) is untouched and still works with no hub"* — it did not, because UI dispatch
refused it.

Deliberately **not** migrated: `foldDirectoryEvents`, `bindSpaceFile`, `importSpace`,
`deleteVirtualFileSystemNode`, `renameSpace`. `foldDirectoryEvents` is the same one-line shape and
is what carries ANOTHER user's `space.created` live; it is named here as the next owner's cheapest
follow-up, and it is not on this slice's acceptance path because the bootstrap page re-delivers the
same events.

### 2.3 Two more gaps the fix surfaced, both fixed, one deliberately left

- **Home's close ladder was missing three store lanes.** The moment a `createStudio` dispatch got far
  enough to reach the close, the ladder answered
  `interactive-job.close-owned-disposer-missing … config-store`, then `draft-store`, then
  `artifact store has no owner-supplied bounded disposer`. `EditorApp` installs **no** default
  (`🔌️plugin/🦀️.rs:32854` forwards the editor's own answer unchanged, unlike the viewer wrapper at
  `:7718`), so an editor that omits a lane faults on every close. Home now declares
  `build_config_store_owners`, `build_config_store_disposer`, `build_draft_store_disposer` and
  `build_transient_store_disposer`. This is the same defect the running shell reports as
  `predecessor space/s.space.home@1/*#editor retirement failed` on every app switch (§3).
- **Left, precisely**: Home's **presence** lane still has no retirement factory. That is a bespoke
  domain retirement (`🌊️flow`'s is the pattern, 57 lines) and it is the last stage of the ladder, so
  the live `retirement failed` will persist until it lands. Named here rather than half-written.
- **Two stale laws in the same region, rewritten.**
  `registered_home_rejects_create_studio_until_its_retained_owner_exists` asserted the defect
  (`interactive-job.not-ui-safe`); it is now
  `registered_home_admits_its_retained_routes_at_interactive_dispatch` and asserts both routes are
  past the gate. `temporary_studio_uses_ephemeral_registry_not_catalog` compared the fault **code**
  against `s.home.session-identity-required`, but `Fault::from(&str)` mints code `app.message` and
  carries the text as the message — so that assertion had **never** passed; it now checks the pair.

### 2.4 Judged

```
cargo test -p semio-s-artifact-space-home --lib --features component-app-assembly
  →  92 passed; 0 failed        (S3's baseline on the same command: 90 passed, 2 failed)
```

Both of S3's pre-existing `create_studio` reds are green, and the boundary law now pins the new
two-ceiling rule (`page(scalar+1)` admitted, `page(rawBytes)` admitted, `page(rawBytes+1)` refused).

```
cargo test -p semio-s-plugin-space --lib interactive_job_catalog_tests  →  4 passed; 4 failed
```

All four reds are **pre-existing**, and the proof is structural rather than asserted: `home_declares_…`
reports `presenceHeartbeat` missing from the app definition's migrated set, and `presenceHeartbeat`
was `Migrated` **before** this slice — `declared_dispositions` walks only window-kind actions and
commands, and the three `.view_action` routes are on no window kind. `studio_declares_…` and
`space_index_declares_…` are short by 10 and 14 ids respectively, in files with live peer edits.
`every_app_instance_constructs_…`'s hard-coded Home proof count was updated 9 → 11 (the one line this
slice owns); it now fails later, in `close_registered_fixture_app`, on the **studio** app's missing
document-store disposer — the same close-ladder debt as §2.3, in another app.

### 2.5 Re-stage, and the two further stages the live shell then named

The guest fix needs S2 §3.4's narrow re-stage, now a permanent script `📜️s4-restage.sh` (component
build → `materialize-dev` → `activate s react dev`, **each step through the fleet wasm mutex**, private
`CARGO_TARGET_DIR=…/cargo/target-s4`, capture `🗑️generated/s4-restage.txt`). Three runs, all green:
`Activated s react dev: 60 completed components (changed)` each time; the component build took
29 s / 61 s / 3 m 21 s warm.

**Each re-stage moved the live fault one stage further down the same lane** — this is the measured
sequence, and it is the evidence that the route had never been executed end to end:

| re-stage | what the running shell then answered | fix |
|---|---|---|
| before | `UI dispatch rejected action:applyDirectoryEventPage with interactive-job classification BatchOnlyPendingRewrite` | §2.2 |
| 1 | `typed-operation emitted a store lane absent from its exact factory publication contract` | the publication contract declared `HostOnly`, which means **no store lane at all** — right for Home's nine pure-`Effect` relays, wrong for the only two routes that WRITE. `applyDirectoryEventPage` → `Config` (it replaces the directory projection), `createStudio` → `Artifact` (it bumps the catalog generation) |
| 2 | `one-item preparation footprint exceeds its fixed item or byte capacity` | `HomeConfigPreparationFactory::preflight` declared `retained_bytes: HOME_CONFIG_STEP_BYTES` = **16 MiB**, while `ArtifactStoreOneItemFootprint::is_admissible` refuses anything over `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES` = **1 MiB**. So Home's whole config one-item lane was sized above the store's hard ceiling and could never be admitted. `HOME_CONFIG_BASE_BYTES` (4 MiB) and `HOME_CONFIG_STEP_BYTES` (16 MiB) are now both the store's own ceiling, and the fixture's `configBaseBytes`/`commandStepBytes`/`storeStepBytes` follow |
| 3 | **the retry storm is gone**: `31×` `event-page/v1?after=0` in a 60 s window → **`2×`**. The first page is now accepted by the guest | — |

**What still stops the projection, named exactly and not fixed.** With all three stages passed, the
guest answers the action with a **pending typed-operation handle**
(`{"generation":"0","operationId":"64"}`, read verbatim off the running shell), not the receipt —
because the receipt rides a later turn on typed-operation lane 7, which
`🔌️PluginRuntime/🟦️.tsx:783-788` already knows how to lift into `TYPED_OPERATION_TERMINAL_OUTPUT`.
`applyDirectoryEventPageBootstrapV1` reads the FIRST `handleAction` response, so
`parseDirectoryProjectionReceiptV1` answers `null`, the owner is closed with
`directory-bootstrap.receipt-mismatch`, and the next page throws `app-channel.disposed`. The shell's
notice therefore reads `Directory update stopped` instead of looping. **This is the one remaining
step on this lane**: the bootstrap must settle the typed operation and take the terminal output the
host already extracts, rather than the first response. It is a TypeScript change in a peer-hot file
and was not half-landed.


## 3. Space creation and entering a space — measured, works today

`🐍️s4-space-create-diagnose.mjs` and `🐍️s4-space-index-diagnose.mjs` (new, permanent), captures
`🗑️generated/s4-create-{1,2,3}.txt`, `s4-index-1.txt`.

**Where the create form actually is.** Home's `#s-home-create-space` button dispatches `createSpace`,
whose handler opens a dialog — but the reachable form is the hub workspace's own, and it is only
reachable **while the overlay is open right after sign-in** (S3 §5.1's finding, confirmed). Its real
field identities, which no report had: `input[name="spaceName"]` (**no `type` attribute**, so
`input[type="text"]` misses it — that is why S3's attempt filled nothing), two `select`s for kind
(`atelier`/`studio`) and visibility (`private`/`public`), and `button[aria-label="Create space"]`,
which is `disabled` until the name is non-empty.

Measured, signed in as `user1@semio.dev` (principal `01a0bbc1-126b-758b-a461-e6b9763fdb99`):

```
POST http://127.0.0.1:7501/directory/commands → 202
…workspace lists:  "Open S4 Studio 5607 — Author"   (spaceId 01a0bf67-887d-7208-a07f-1293f3c87e79)
…directory frontier moves 3 → 5, and the page carries the event verbatim:
   {"seq":4, "body":{"kind":"space.created","name":"S4 Studio 5607","spaceKind":"atelier",
                     "visibility":"private","ownerUserId":"01a0bbc1-126b-758b-a461-e6b9763fdb99"}}
```

Clicking `Open …` navigates the real `s` host to **`/spaces/01a0bf67-…`** and paints
`semio · s · space · index` — window `framework.window.table`, ids
`window:framework.window.table/s-space-create-artifact`, `s-space-members`, header row
`ID Name Kind Subset Updated Updated By`. **This is the first time this ticket has observed the `s`
product inside a hub space.** Screenshot `🗑️generated/s4-studio-space-opened.png`.

Two defects found in passing, not on the acceptance path, handed on:

- **A false negative on create.** The overlay shows *"The hub did not accept that. Nothing was
  changed."* while the hub **did** accept it (202, `space.created` linearised, the row appears in
  the same overlay). The originating client's receipt never arrived — `[S4PROBE] directory command
  receipt` was instrumented at `🏛️ShellHost/🟦️.tsx:3232` and never fired — so the operation settled
  on the `directory-command-failed` path. Owner: whoever owns `flushDirectoryQueue`'s receipt wait.
- `pageerror Error: view context: invalid identifier` on entering a space, and
  `switchToPluginApp: predecessor space/s.space.home@1/*#editor retirement failed`.

## 4. What needs the hub authority, and what works locally — the precise answer

Brief item 2 asked exactly this. Measured on hub 7501.

| studio/space action | needs the hub? | observed |
|---|---|---|
| sign in, session refresh | hub directory | **works** — 200s throughout |
| create a space, list spaces, roles, members | hub directory (`/directory/commands`, `/directory/event-page`) | **works** — 202 + linearised event |
| enter a space (`/spaces/<id>`, space index paints) | hub directory only | **works** |
| Home's space/studio table populating | **guest** `applyDirectoryEventPage` | was dead (§2), fixed |
| **open any document in a hub space** (the space's own `index`, hence `Create Artifact`, hence every artifact of every plugin kind inside a hub space) | **hub `artifactAuthority` / trusted catalog** | **503** |
| create a **local** studio (`createStudio`, ephemeral, no hub) | none | was dead (§2), fixed |

The 503 is not a bug in the `s` host. `hub_readiness`'s `openPlan` flag is `true` **iff**
`state.openable_catalog` is `Some(...)`, and the plan route answers `503 {"code":"catalog-unavailable"}`
otherwise — pinned by the hub's own laws
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:2905-2906`, `:2969-2970`, `:6822`). A trusted catalog generation
exists only after `os-hub trusted-catalog publish` (`🌎️hub/README.md` §"Publishing a trusted
catalog"); until then `/readyz` reports
`trusted-catalog-never-published-in-this-data-root`, which is exactly what hub 7501 reports.

**So booting a newer hub binary would not have helped** — the binary is not too old; the data root
has no catalog, and producing one is TC1's lane: `produceFreshComponentV1` runs a cold
`cargo rustc --target wasm32-wasip2 --profile wasm-release` with `CARGO_INCREMENTAL=0` per plugin
(TC1 measured **61 min for `stdio`, 62 min for `gis`**), and TC1's own report states it landed
nothing. No own hub was booted: it would have reproduced the same 503 from an empty data root.

Consequence, stated plainly: **artifacts of other plugins' kinds cannot be created inside a hub
space on any hub in this tree today.** The only path to outcome 1's acceptance that does not wait on
TC1 is the **local** studio (`createStudio`), which is why it is in §2.2's fix.

## 5. Foreign-kind matrix inside `s` (open → mutate → undo → redo)

### 5.1 The palette's `spawn.<plugin>` was a dead round trip — found and fixed

Every `spawn.<plugin>` this product offers was refused, from Home and from an opened hub space alike:

```
semio: app "s.space.home@1/*#editor" dropped action "spawnApp" dispatched from window kind
"s-home-main": no window kind declares it (window kinds: s-home-main).
```

S3 handed this on as "the palette offers a verb the active window cannot accept". The cause is an
**inverted controller test inside the shell's own code**, both halves in `🏛️ShellHost/🟦️.tsx`:

- the palette builds each spawn item as a guest action with the HOST controller —
  `onAction({ controllerId: hostControllerId ?? "", action: "spawnApp", … })` (`:10689`);
- the shell's own interceptor claims `spawnApp` only when the controller is **not** the host's —
  `if (hostMode && action.action === "spawnApp" && action.controllerId !== hostControllerId)`
  (`:6913`), which exists so the host app's *own* `spawnApp` reaches the guest that declares it.

So the shell's own affordance was the one case its own interceptor excluded, and it fell through to
whichever app owns the active window. From the landing window that app is Home, which does not
declare `spawnApp`.

**Fix**: the palette item calls the shell's own `spawnProgram(program)` directly — spawning from the
palette is shell chrome, and `spawnProgram` is exactly what the interceptor was going to call. The
guest's own `spawnApp` path is untouched. TypeScript, so a reload picks it up; no re-stage.

**Measured, before and after**, `🐍️s2-s-host-foreign-kind-probe.mjs` signed in against 6071:

| | before | after |
|---|---|---|
| `draw` | `firstError: dropped action "spawnApp" …` | **`firstError: null`** |
| `note` | same | **`firstError: null`** |
| canvas breadcrumb after spawning `draw` | unchanged (`semio · s · home`) | **`← Back to Workflow · semio · drawing`** |
| canvas breadcrumb after spawning the studio | unchanged | **`← Back to Workflow · semio · s · studio`** |

Captures `🗑️generated/s4-foreign-kind-{2,3}.txt`, `s4-spawned-2.txt`.

### 5.2 The remaining blocker: a spawned program has no windows

The foreign program is now spawned, instantiated and focused — and its canvas is empty, showing the
shell's own instruction *"Drag windows from Display in the navbar, or restore a saved layout."*
Following that instruction is what names the defect:

```
after spawning draw:   breadcrumb "semio · drawing",  [data-window-id] = []
Display ▸ Windows offers exactly one entry:  framework.display.windows.s-home-main  ("Studios")
```

The Display menu offers the **landing** app's window, not the spawned app's. The shell derives window
instances from `sessionWindowInstances(session.app, …)` at every one of its call sites
(`🏛️ShellHost/🟦️.tsx:4115, 4667, 4863, 5074, 6781, 6838, 7018`), and `spawnProgram`
(`:6600`) / `ensureSpawnedPlugin` (`:5639`) put the spawned app in the **space panel**
(`studioPanelFocusingSpawned`) without ever making it the session. So a spawned foreign program is
focused on the canvas with no window instances, and the Display menu has none to offer.

**No artifact of a foreign kind was therefore opened, mutated, undone or redone inside `s` by this
slice** — stated plainly. What changed is that the blocker is no longer a refusal: the spawn is
accepted, the instance is created, the breadcrumb is the foreign app's. The remaining piece is one
named question for the next owner — *the canvas and the Display menu must derive their window
instances from the active spawned app when one is focused, not only from the session's app* — and it
is a ShellHost change, no wasm rebuild.

| plugin kind | spawn accepted | instance created | window | mutation | undo | redo |
|---|---|---|---|---|---|---|
| `draw` | **yes** (was refused) | yes (breadcrumb `semio · drawing`) | **none** — §5.2 | — | — | — |
| `note` | **yes** (was refused) | yes | **none** — §5.2 | — | — | — |
| `s.space.studio` (the studio itself) | **yes** | yes (breadcrumb `semio · s · studio`) | **none** — §5.2 | — | — | — |

### 5.3 Two corrections to what was on file

- **The palette is capped when unfiltered.** An unfiltered dump shows ~20 rows, all `spawn.space`,
  which reads as "no foreign kinds offered". Typing the plugin id reveals `spawn.<plugin>` from all
  148 spawnable programs. Any probe that judges the palette without typing will mis-report.
- **`createStudio` is not reachable from the UI at all.** It is in the palette under no id, Home
  publishes **zero** `[data-action-id]` controls (its Actions pane is empty — the `imperative` class),
  and its `mod+n` keybinding does nothing from the landing window. The reachable route to a studio is
  the palette's `Spawn semio · s · studio`, which is a spawn, not `createStudio`. The classification
  fix in §2.2 is still necessary (it would be refused the moment anything dispatched it) but it is
  not sufficient on its own.

Established before the fix, and it corrects S3 §5's reading: `spawn.<plugin>` is **not** offered by
the space index either. Its palette, measured inside `/spaces/<id>`, offers only
`Spawn semio · s · home` and `Spawn semio · s · space · index` — the shell's spawn list is the
space's registered app set, and `spawnApp` belongs to `s.space.studio@1/*#editor`
(`S_PLAY_APP_ID`), whose owned tools are `setAppRegistrations`, `openSpace`, `openInstance`,
`importSpacePackPayload`, `spawnApp`. So the journey is Home → **studio** → palette → spawn, and the
space index is a third surface between them, not the studio.

## 6. LB1 live-agent gate against this `s` host

**Not run, and the reason is measured rather than assumed.** The gate's `boot`/`ui_focus` steps and
f1–f8 need a foreign-kind **editor open inside `s`**, which §5.2 proves is not reachable today: the
spawn is accepted, the instance is created, and the app has no window. Running the gate would have
re-observed exactly that at the cost of a contended `semio-framework-os-mcp` build — two
`cargo test -p semio-framework-os-mcp` processes were live at 19:40 and preamble rule 26 forbids
starting a third.

What this slice **did** establish for it, measured, is the gate's own step-0 precondition
(`📓️ap1-shell-approval-and-live-snapshot.md` §5.3 item 1): the serve must answer `SHELL_URL` and its
transformed `🏛️ShellHost` must contain `agentArtifactRouteRef`.

```
GET http://127.0.0.1:6071/@fs/…/🏛️ShellHost/🟦️.tsx   →  200, 1 887 754 bytes
  agentArtifactRouteRef        3 occurrences   → :6071 is an eligible gate shell
  void spawnProgram(program)   2 occurrences   → §5.1's fix IS the module vite serves
```

The second grep is there on purpose: it proves the served transform is not stale, which is the trap
that has cost this ticket a slice before. So LB1's next run should target **:6071**, and its one
remaining blocker is §5.2, not identity and not the shell route.

### 6.1 Permanent wiring

No new nx target or launch row was added: S3 already wired **`s-host-foreign-kind-s`**
(`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`) and the launch row
`🛠️dev🪐️os-s🔭️foreign-kind` (group `3_dev`, order `387.066`) to
`🐍️s2-s-host-foreign-kind-probe.mjs`, which is the probe this slice extended with the studio step —
so the existing target now runs the fuller journey. `📜️s4-restage.sh` is the re-stage recipe as a
script rather than a paragraph; it is ticket-owned, not a product command.

## 7. Measured vs unverified, honest gaps

**Measured at runtime by this slice** (capture named for each):

- every hub-lane request and status during a signed-in session — `s4-journey-1.txt`
- the directory refusal verbatim, with the page that was refused — `s4-journey-2.txt`
- a space created on hub 7501 from inside `s`, with the linearised `space.created` event —
  `s4-create-3.txt`
- the `s` host navigating to `/spaces/<id>` and painting the space index —
  `s4-studio-1.txt`, `s4-studio-space-opened.png`
- `open-plan` → 503 on the space's own `index` document — `s4-index-1.txt`
- the space-index palette offering no foreign-kind spawn — `s4-index-1.txt`

- every re-stage's receipt: `Activated s react dev: 60 completed components (changed)` ×3 —
  `s4-restage.txt`, `s4-activate.txt`
- the directory lane's three refusals in sequence, each read verbatim off the running shell, and the
  retry storm collapsing `31× → 2×` — `s4-journey-{2,5,6,7}.txt`
- `cargo test -p semio-s-artifact-space-home --lib --features component-app-assembly`:
  **92 passed, 0 failed** (S3's baseline 90/2)
- the `spawnApp` refusal disappearing and the canvas breadcrumb becoming the foreign app's —
  `s4-foreign-kind-{2,3}.txt`, `s4-spawned-2.txt`
- the Display menu offering only the landing app's window after a foreign spawn — `s4-spawned-2.txt`
- the served `🏛️ShellHost` carrying both `agentArtifactRouteRef` and this slice's fix (§6)

**Not done / unverified**:

- **One mutation + undo + redo in any foreign plugin inside `s`** — not reached. The blocker moved
  from "the dispatch is refused" to "the spawned app has no window" (§5.2), which is a named
  ShellHost question needing no wasm rebuild.
- **Home's space table populating** — the projection still does not land: the bootstrap reads the
  first `handleAction` response instead of settling the typed operation (§2.5, last paragraph).
- **LB1's f1–f8** — §6.
- **Artifacts inside a hub space** — blocked on a published trusted catalog (§4), TC1's lane.
- **Home's presence retirement factory** — §2.3; the live
  `predecessor space/s.space.home@1/*#editor retirement failed` persists until it lands.
- **`foldDirectoryEvents`** — still `BatchOnlyPendingRewrite`, same one-line shape as §2.2; it is
  what carries ANOTHER user's `space.created` live, so outcome 3's live half needs it.
- **The false "The hub did not accept that"** after a create the hub DID accept (§3) — handed on.
- The four `interactive_job_catalog_tests` reds are pre-existing (§2.4); this slice did not fix the
  `declared_dispositions` walk that misses `.view_action` routes, nor the studio/space-index
  close-ladder debt.
- `pageerror Error: view context: invalid identifier` on entering a hub space — observed, not traced.

## 8. Files changed

| file | change |
|---|---|
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | `applyDirectoryEventPage` + `createStudio` migrated onto the retained tool factory (§2.2): tool ids, publication contracts, per-route extent ceiling, identity routing in `home_retained_reduce`, proof literals, `Migrated` classifications |
| `…/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json` | the two committed rows now `Migrated` / `["HostOnly"]`, blockers cleared |
| `🐍️s4-space-journey-diagnose.mjs` (ticket) | new: full hub-lane network + directory-notice census through sign-in |
| `🐍️s4-space-create-diagnose.mjs` (ticket) | new: the hub workspace's create-space form, with its real field identities |
| `🐍️s4-space-index-diagnose.mjs` (ticket) | new: inside an opened space — `Create Artifact`, the palette, the `open-plan` lane |
| `…/✏️editor/🎮️commands/🏗️create-studio/🧪️tests/🔬️unit/🦀️.rs` | stale law rewritten as `registered_home_admits_its_retained_routes_at_interactive_dispatch`; the `Fault::from(&str)` code-vs-message assertion corrected |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | boundary law extended: `createStudio`'s exact scalar limit, and the three `applyDirectoryEventPage` page-size cases that pin the new two-ceiling rule |
| `✏️s/🔌️plugins/🪐️space/🧪️tests/🔬️interactive-job-catalog/🦀️.rs` | Home's proof-count literal 9 → 11, with the reason |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | **§5.1's fix**: the palette's `spawn.<plugin>` item calls the shell's own `spawnProgram(program)` instead of raising a guest action with the one controller id the shell's own interceptor excludes |
| `🐍️s4-studio-foreign-kind-probe.mjs` (ticket) | new: sign in → space → studio → per-kind spawn/mutate/undo/redo |
| `🐍️s4-create-studio-diagnose.mjs`, `🐍️s4-spawned-window-diagnose.mjs` (ticket) | new: how `createStudio` is (not) reachable, and what the shell offers after a spawn |
| `🐍️s2-s-host-foreign-kind-probe.mjs` (ticket) | new `enterStudio` step — the studio is spawned through the palette's label (all three `🪐️space` apps share the id `spawn.space`) |
| `📜️s4-restage.sh` (ticket) | the narrow re-stage as one script, every wasm step through the fleet mutex |

## 9. Live state handed on

- 6070 (pid 26481), 6071 (pid 34308) and hub 7501 (pid 5468) untouched and still serving. **6071
  carries every fix in this report**: the guest through three re-stages, the two ShellHost changes
  through vite (verified by grepping the served transform, §6).
- Re-run the whole journey with
  `S2_SIGN_IN_EMAIL=user1@semio.dev S2_SIGN_IN_PASSWORD='collab e2e first human phrase' bun 🐍️s2-s-host-foreign-kind-probe.mjs http://127.0.0.1:6071/ draw note`
  and the re-stage with `zsh 📜️s4-restage.sh` (≈ 4 min warm).
- Next owner's first move is §5.2: make the canvas and the Display menu derive window instances from
  the active **spawned** app, not only from the session's app. No wasm rebuild; one probe run judges it.
- Two spaces now exist on hub 7501, made by this slice: `S4 Studio 5607`
  (`01a0bf67-887d-7208-a07f-1293f3c87e79`) and `S4 Studio 50858`
  (`01a0bf69-bb38-78d9-8702-b38da09321a3`), both owned by `user1@semio.dev`.
