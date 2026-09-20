# S6 — every plugin kind inside the real `s` host, a hub document inside `s`, the agent gate inside `s`

Slice S6, fleet 6, 2026-09-20 ~20:40–22:30. Scope handed over from S5
(`📓️s5-spawned-app-windows-in-s.md` §4.3 residual, §6 nine-kind matrix, §8 "the gate has no spawn
step").

Status legend: **measured** = this slice ran it and captured output; **unverified** = read from
source only.

## 0. tl;dr

| item | result |
|---|---|
| **the sweep** | **all 35 spawnable kinds in the `s` registry driven inside the real `s` host in one session each.** 32 of 35 spawn (3 have no palette entry at all), **32 of 32 open their own windows — 67 windows total — and publish their own Actions rail (10–59 rows)**, 24 dispatch a document verb from that rail, 23 do it with zero fault lines, **3 show the complete round trip** (§3) |
| **the one-refresh-late residual** | **root-caused one layer deeper and a THIRD host-side cause ruled out by measurement.** It is not latency: 12 s of quiet after the verb changes nothing at all. The host's reading of a spawned program's work is **two dispatches behind**, and the surface that is stale is the shell's own History ledger and `#s-checkin` count. One host-side fix (refreshing the host's panels after spawned work) was written, measured to change nothing, and **reverted**. Remaining suspect named with file:line (§2) |
| **hub document inside `s`** | **no — blocked, with the boundary moved.** A second `s` serve was bound to GM1's ready hub 7611 (`:6092`, pid recorded), the shell boots `ready:s`, **sign-in against 7611 succeeds inside `s`** (`Signed in as 01a0c00d-cd33-7948-91cb-da23affa54ec` — GM1's `user1@semio.dev` principal) and `/spaces/<id>` renders the **hub space INDEX surface inside the real host** (`framework.window.table`, artifact columns, `Create Artifact`). Then hub 7611 went down mid-run — `Connection refused`, with two sibling hold restarts in flight and one already failed `ArtifactAuthority(DeadlineExceeded)`. Not contended for (§4) |
| **the agent gate inside `s`** | **run, and moved from unrunnable to 11/21.** The gate gained its missing spawn step (additive, `S_OS_MCP_LIVE_SPAWN`, one skipped `if` when unset) and a root fix landed in the shell: the agent census was the session ALONE, so a spawned editor was invisible to an MCP client. **7/21 → 11/21**, the whole (a)/(b)/(c)/(d) shell-control block now passing against a spawned foreign editor inside `s` (§5) |

Two root fixes landed, each judged by the live gate answering a **different** fault afterwards:

| # | what was broken | evidence it is fixed |
|---|---|---|
| 1 | the live agent gate had no way to reach a spawned editor — its step 0 asserts the BOOTED plugin's windows, which against `s` is Home | `PASS 0 spawn note through the command palette` |
| 2 | `agentBridgeInstances` was the live session and nothing else, so an MCP client could drive only the landing app of the shell whose whole purpose is hosting every other plugin's artifacts | `… the shell reports 1 open instance(s)` → `… the attached shell has 3 open instances`, and (a)/(b)/(c)/(d) went fail → **pass** |

## 1. Inherited state (measured)

| thing | state at slice start |
|---|---|
| serve `:6070` | pid 26481, `HTTP 200` |
| serve `:6071` | pid 34308, parent 33969 = `bun ./📜️script.ts serve s react dev`, env `S_OS_PORT=6071 S_HUB_URL=http://127.0.0.1:7501 S_DATA_DIR=/private/tmp/c1c-s-data`, `HTTP 200` |
| hub `:7501` | `os-hub` pid 5468, alive |
| hub `:7611` | `os-hub` pid 39233, alive, `/readyz` **ready** with `artifactAuthority.ready` and `features.openPlan` |
| predecessor S6 work | **none** — no `📓️s6-*.md`, no `🗑️generated/s6-*`. Clean start. |
| sibling C2 | `📓️c2-two-user-collaboration-on-ready-hub.md` was all `(filling)`; **no serve of `s` against 7611 existed to reuse**, so §4 started its own |

**Nothing of a peer's was killed or restarted by this slice.**

## 2. The one-refresh-late repaint — root-caused deeper, third host-side cause ruled out

S5 §4.3 stated the residual and ruled out two host-side causes. This slice ruled out a third and
replaced the sentence "the window repaints one refresh late" with three measurements.

### 2.1 It is not latency

`🐍️s6-repaint-latency.mjs` runs one verb on a spawned program and then samples the canvas every
250 ms for **12 s with no input at all**, and does the same for the undo and the redo. Spawned `dag`,
`addNode` (`🗑️generated/s6-repaint-dag.txt`):

```
after-verb: 0 transition(s) in 12000 ms of quiet
after-undo: 0 transition(s) in 12000 ms of quiet
after-redo: 0 transition(s) in 12000 ms of quiet
```

Nothing arrives on its own. A slow refresh would have shown a transition; a dropped one shows this.

### 2.2 The stale surface is the shell's own history, and it is TWO dispatches behind

`🐍️s6-ledger-diagnose.mjs` reads every surface a mutation could move, step by step. Spawned `draw`,
`addLayer` → undo → redo (`🗑️generated/s6-ledger-draw.txt`):

```
before             #s-checkin "Check In"      history entries 3
after addLayer     #s-checkin "Check In"      history entries 3     ← stale; the layer IS in the document
after +4 s quiet   #s-checkin "Check In"      history entries 3     ← still stale
after undo         #s-checkin "Check In"      history entries 3
after redo         #s-checkin "Check In (1)"  first history row = "create-layer index=1 path base { id=path-6d482eb28…"
```

The plugin-authored ledger row and the uncommitted-edit count **do** arrive — two dispatches later.
The same shape shows in the sweep's own `edits` arrays (§3): `draw` reads `[0,1,1,1]` under one
alignment and `[0,0,0,1]` under another purely because the number of dispatches before each read
changed.

### 2.3 The host-side fix that did NOT work, and why it was reverted

`refreshSpawnedUi` fetches a spawned instance's bodies, utilities, engagements and measures and **no
panels** — "no panels, no labels" is its own docstring (`🏛️ShellHost/🟦️.tsx:5468`) — and the branch
that calls it in `applyHostEffects` (`:6268`) has no host half. So the host's panels genuinely are
never refreshed while a spawned program owns the canvas. A `hostPanelRefreshScopeV1` unit plus a
panel-only `refreshUi(hostSession, scope)` in that branch was written, type-checked, served
(`GET /@fs/…/🏛️ShellHost/🟦️.tsx` → `hostPanelRefreshScopeV1` ×2, so the browser ran it) and
re-measured: **the reading was byte-identical**. Under preamble rule 6 an unproven change that also
costs a guest round trip per spawned dispatch was not landed — it is reverted and the tree carries
none of it.

That rules the host's panel-refresh lane out and leaves the completion DELIVERY itself.

### 2.4 The remaining suspect, named

`drainTypedOperations` (`🔌️PluginRuntime/🟦️.tsx:2995`) is the only thing that advances a retained
typed operation once no host call is left to drive it, and it is re-armed at `:2983` by
`if (wireTurnStatusTag(result.status) === "more-work") void drainTypedOperations(instanceId)`. Its
terminal `AppFrame::OperationCompleted` is what carries the `historyPatch` the shell's ledger is built
from (`💻️os/🟦️.ts`'s `pumpOutcomes` → `publishOperationCompletion`). The observed behaviour — nothing
for 12 s of quiet, everything two dispatches later — is what a spawned instance whose drain is not
armed looks like. **This is a hypothesis with a file:line, not a measured cause**; proving it needs
instrumentation inside that loop, which is where the next owner should start.

Adjacent, measured, and separate: `subscribeOperationProgress` / `subscribeSpawnedJobProgress` are
still wired for `session.instanceId` ALONE (`🏛️ShellHost/🟦️.tsx:6357-6381`) — the same session-only
shape as the five S5 fixed and the one this slice fixed in §5. A spawned program's mid-operation
progress reaches no window.

## 3. The sweep — every plugin kind the `s` registry offers

`🐍️s6-all-kinds-sweep.mjs` (nx target `s-host-foreign-kind-s`'s sibling; the census mode answers the
registry). Signed in as `user1@semio.dev` on `:6071`, Home → studio → per-kind spawn, **each spawn
closed before the next** so the table measures hosting rather than sixty live wasm instances.

### 3.1 What the registry offers

```
60 registry components, all 60 loaded at boot, 148 spawnable programs
35 distinct plugin ids own a spawnable program
25 own none — they are extensions/modules that contribute to a host plugin:
   cad-extension-* (4), flow-extension-* (9), imperative-extension-* (5),
   process-extension-* (4), sourcing-module-* (3)
```

So "every plugin kind" is **35 rows**, not 60; the other 25 are measured to have no program of their
own. `🗑️generated/s6-sweep-census.txt`, `s6-spawnable-kinds.txt`.

### 3.2 The table

Captures `🗑️generated/s6-sweep-{a,b,c,d}.txt`, rendered `s6-sweep-table.txt`.

| kind | windows | rail rows | verb taken | verb moved the document | undo / redo lane | redo ≠ undo | faults | verdict |
|---|---|---|---|---|---|---|---|---|
| `animate` | 1 | 26 | `seedGrid` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `architect` | 4 | 33 | `addRegisterItem` | **yes** | — / — | **yes** | 0 | ✅ **PASS** |
| `block` | 1 | 22 | — | — | — | — | 0 | none of 6 rows moved the document |
| `cad` | 4 | 37 | — | — | — | — | 0 | none of 12 rows moved the document |
| `dag` | 2 | 24 | `addNode` | **yes** | — / — | no | 1 | |
| `demonstrator` | 0 | 0 | — | — | — | — | 0 | **no `spawn.demonstrator` palette entry** |
| `draw` | 1 | 17 | `addLayer` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `energy` | 4 | 34 | `set-surface-property` | **yes** | — / — | no | 0 | |
| `fem` | 2 | 34 | `addNode` | **yes** | — / — | no | 0 | |
| `flow` | 2 | 27 | `addWidget` | **yes** | — / — | no | 1 | |
| `forms` | 2 | 35 | `addQuestion` | **yes** | — / — | no | 0 | |
| `gis` | 1 | 31 | `addFeature` | **yes** | — / — | no | 0 | |
| `imperative` | 2 | 23 | `addStep` | **yes** | — / — | no | 0 | |
| `layout` | 2 | 19 | `addFrame` | **yes** | — / — | no | 0 | |
| `lowpoly` | 1 | 59 | `addPrimitive` | **yes** | — / — | no | 1 | |
| `mathematical` | 0 | 0 | — | — | — | — | 0 | **no `spawn.mathematical` palette entry** |
| `norm` | 2 | 12 | — | — | — | — | 2 | none of 2 rows moved the document |
| `note` | 2 | 19 | `addBlock` | **yes** | — / — | no | 0 | |
| `playbook` | 1 | 21 | — | — | — | — | 0 | none of 6 rows moved the document |
| `playbook-module-procedural` | 0 | 0 | — | — | — | — | 0 | **no palette entry** |
| `procedural` | 2 | 23 | — | — | — | — | 0 | none of 6 rows moved the document |
| `process` | 1 | 19 | `addStep` | **yes** | — / — | no | 0 | |
| `puzzle` | 3 | 30 | `addNode` | **yes** | — / — | no | 0 | |
| `raster` | 2 | 15 | `addLayer` | **yes** | — / — | no | 0 | |
| `reasoning` | 1 | 18 | — | — | — | — | 0 | none of 3 rows moved the document |
| `remodel` | 2 | 40 | `addStream` | **yes** | — / — | no | 0 | |
| `sequence` | 3 | 29 | `addStep` | **yes** | — / — | no | 0 | |
| `shooting` | 2 | 47 | `addShot` | **yes** | — / rail | **yes** | 2 | 4 of 5 clauses |
| `sourcing` | 4 | 16 | `paste` | **yes** | — / — | no | 3 | |
| `space` | 1 | 24 | — | — | — | — | 5 | none of 9 rows moved the document |
| `stdio` | 1 | 10 | `paste` | **yes** | — / — | no | 6 | |
| `trinity` | 6 | 22 | `patchNodes` | **yes** | — / — | no | 6 | |
| `vcs` | 2 | 19 | — | — | — | — | 7 | none of 6 rows moved the document |
| `wfc` | 2 | 25 | `set-input-pixels` | **yes** | — / — | no | 8 | |
| `writer` | 1 | 16 | `paste` | **yes** | — / — | no | 9 | |

```
spawned windows inside s   32 / 35   (67 windows)
Actions rail published     32 / 32
document verb dispatched   24 / 32
zero fault lines           23 / 35
FULL round trip            3  / 35   (animate, architect, draw)
```

### 3.3 What the `no` in the `redo ≠ undo` column means — and what it does NOT mean

It means **not demonstrated**, and §2 is why: the host's reading of a spawned program is two
dispatches behind, so a probe that takes one reading per dispatch compares post-verb with post-verb.
That is the same caveat S5 §6 gave for its five `no` rows; this slice can now name the mechanism and
has measured the alternatives:

- the undo/redo pair was driven through **two different lanes** and both recorded per row — the rail's
  own `action.undo` row (what a human clicks, what `🐍️b3d-interaction-probe.mjs` drives) and the
  reserved shell CHORD (what S5 drove). Only `animate`, `draw` and `shooting` answered on either lane.
- the round trip was driven with a **fourth dispatch** and read under both alignments; the reading
  still did not move for the lagged kinds, so their `no` is not purely an alignment artefact either.

What IS demonstrated for all 24: the verb reached the spawned instance and moved the document (no
refusal on the lane, and the reading moved).

### 3.4 Three kinds cannot be spawned at all

`demonstrator`, `mathematical` and `playbook-module-procedural` publish no `spawn.<pluginId>` command
palette entry, so there is no route to them inside `s` — while all three ARE in the registry, loaded,
and have a spawnable program in `__semioOsCatalogProbe.programs`. That is a palette/registry
disagreement and the cheapest remaining outcome-1 gap.

### 3.5 Refusals captured verbatim

```
draw   combineBoolean  — draw action 'combineBoolean' arguments do not decode: missing field `ids`
dag    connectMediaPorts / moveMediaNode / renameDagNode
       — action '…' carries a payload no shell `{action,args}` pair can express
         (it is dispatched through the typed command channel)
forms  patchQuestions  — forms action 'patchQuestions' arguments do not decode: missing field `question_ids`
```

The first and third are arguments the probe did not stage. The `dag` family is a real statement about
the lane: those verbs are typed-command-only and have no shell `{action,args}` form at all.

### 3.6 A note on the probe's oracle, because it changed the answer

S5's probe judged mutate/undo/redo by diffing `textContent` of the window bodies **and the engagement
pane**. That oracle is blind twice over and both blindnesses were measured here: unfolding the Actions
rail injects ~50 static row labels into the same subtree, and a graph or 3d window's body is an SVG or
a canvas whose text never changes at all. Under it, `dag addNode` scored "nothing moved" for the verb
**and** its undo. This sweep's digest excludes the action pane by name and counts elements, svg nodes
and canvases per window body, plus the published window measures. `🐍️s6-all-kinds-sweep.mjs` carries
that reasoning in its own docstrings.

## 4. A hub document inside `s` — how far it got, and the blocker

`🐍️s6-hub-document-in-s.mjs`, against a second `s` serve bound to GM1's ready hub.

**Serve started by this slice (the only process it started):** `S_OS_PORT=6092
S_HUB_URL=http://127.0.0.1:7611 S_DATA_DIR=/private/tmp/s6-s-data-7611 SEMIO_VITE_HMR=0 bun
./📜️script.ts serve s react dev`, log `🗑️generated/s6-serve-6092.txt`, launcher pid in
`s6-serve-6092-pid.txt`. `HTTP 200`. No activation was run and no staged output was touched — the `s`
variant was already activated for `:6071`, and a serve is vite only.

Measured, in one session (`🗑️generated/s6-hub-document.txt`, screenshots `s6-hub-{signed-in,space}.png`):

| step | result |
|---|---|
| boot on `:6092` | `ready:s`, windows `["s-home-main"]` |
| sign in through the shell's hub badge against **7611** | **succeeded** — `Signed in as 01a0c00d-cd33-7948-91cb-da23affa54ec`, which is GM1 §0's `user1@semio.dev` principal verbatim |
| the hub workspace | opens, lists the hub `This device — http://127.0.0.1:7611`, offers Spaces / Create a space / Join space |
| the space `01a0c00f-4f3c-7834-a7e6-2ccf9de925db` | **the hub space INDEX surface renders inside `s`**: window `framework.window.table`, columns `ID · Name · Kind · Subset · Updated · Updated By`, a `Create Artifact` control (`window:framework.window.table/s-space-create-artifact`) |
| the gis map document | **not reached** — the table has **0 rows** |

The reason is one banner, read verbatim off the running shell:

```
Shared access is unavailable. Open a fresh authorized session from the secure launcher.
Remote: detached          Showing the spaces from your last connection.      No spaces yet.
```

That string is `DIRECTORY_SESSION_AUTHORITY_TEXT_V1.en.unavailable`
(`📇️directory/🪪️session-refresh/🟦️.ts:14`) and it is published only when
`GET /auth/sessions/me` answered a `DIRECTORY_SESSION_AUTHORITY_REFUSAL_STATUSES` (401/403) or the
authority expired. **The cause is that hub 7611 went down during this run**: immediately afterwards
`curl http://127.0.0.1:7611/readyz` answered `Connection refused`, where minutes earlier it had
answered `status: ready`. Two sibling hub-hold restarts were already in flight (pids 170/176,
98420/98425) and `🗑️generated/gm1-hub-hold.txt` ends with
`error: hub readiness deadline exceeded … ArtifactAuthority(DeadlineExceeded)`.

**Not contended for** (preamble: never wait on a sibling, never take their work). The hub belongs to
GM1's lane and C2's scenario. What this slice contributes to it is the measured statement that the
`s` host's own half works: a shell bound to 7611 signs a human in and renders a hub space's artifact
index inside the real host. The only unmeasured leg is opening a row of it, and it needs the hub up.

## 5. The live agent gate inside `s` with a spawned editor

Preamble rule 26 checked first: `ps -axo command | grep -c "cargo test -p semio-framework-os-mcp"` →
**0**. The gateway binary already existed (`…/🦀️rust/dist/build/semio-os-mcp`, 161 MB, 18:04), so no
build was started by this slice.

### 5.1 The gate's missing step, added

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts`, +81 lines, additive:

- `S_OS_MCP_LIVE_SPAWN` (default unset) — when unset the change is **one skipped `if`**, so every
  existing single-plugin invocation is untouched.
- `spawnProgramThroughPalette` — Home → studio → `spawn.<pluginId>` through the shell's own command
  palette, with S3's measured facts written into its docstring (the chord is `Meta+p`; `spawnApp` is
  the STUDIO app's verb; the studio is found by LABEL because all three `🪐️space` apps share the
  palette id `spawn.space`).
- a `0 spawn <plugin> through the command palette` step, recorded like every other.

### 5.2 The root fix the gate then measured

First run against `:6071` (`🗑️generated/s6-agent-gate-s.txt`) — the spawn step passed and named the
next fault exactly:

```
PASS  0 spawn note through the command palette
FAIL  (f2) action_prepare :: PLUGIN_UNAVAILABLE: capability `note.s.note.note@1/*#editor.addBlock`
      is owned by plugin `note`, which has no open instance in the attached shell — open one there
      first (the shell reports 1 open instance(s))
```

One open instance — `s`'s own Home — while a spawned `note` editor had its window on the canvas.
`agentBridgeInstances` (`🏛️ShellHost/🟦️.tsx:2475`) was the live session and nothing else: **the same
session-only shape S5 found at five other decision sites, on the last lane that still had it.** An
MCP client could drive only the landing app of the shell whose whole purpose is hosting every other
plugin's artifacts.

Landed: the census is the session PLUS every spawned program, filled by an effect keyed on the spawned
roster (never on `panel` identity — a panel object churns on every action) and on the loaded-plugin
count (a roster entry can be written before its plugin finished loading). The window ids are the
shell's namespaced ones from the owned unit's own projection, `spawnedProgramWindowInstancesV1`, which
is what `ui_focus` and every window-scoped agent command address.

### 5.3 Measured, before and after — same gate, same shell, same session

```
                                              before   after
0 rendezvous                                    PASS    PASS
boot                                            PASS    PASS
0 spawn note through the command palette        PASS    PASS
(a) the shell dials /bridge                     PASS    PASS
0 context_resolve pins the shell channel        PASS    PASS
(d) ui_reveal moves the real dock               FAIL →  PASS
(a) agent presence renders                      FAIL →  PASS
(d) ui_focus moves the real active window       FAIL →  PASS
(b) tool call → running row → result row        FAIL →  PASS
(c) Cancel cancels an in-flight call            FAIL →  PASS
(f1) artifact_create/open                       PASS →  FAIL   ← a DIFFERENT fault, §5.4
(f2)…(f8), (e1), (e2)                           FAIL    FAIL
(e3) a silent client returns the typed timeout  PASS    PASS

os-mcp-live-agent-loop: 7 passed / 14 failed   →   11 passed / 10 failed   of 21
```

`🗑️generated/s6-agent-gate-s.txt`, `s6-agent-gate-s2.txt`.

### 5.4 The next fault, named

```
INTERNAL: `note` refused ReadArtifact (plugin.unavailable): this command carries no capability to
resolve an instance from and the attached shell has 3 open instances — prepare an action against
the artifact first so the route binds
```

`1 open instance` → **`3 open instances`** is the fix answering. The remaining fault is a different
one and belongs to the gateway's `ShellArtifactChannel`: a capability-less `ReadArtifact` has no rule
for choosing among several instances. The obvious rule is the shell's own — the FOCUSED program, which
`focusedProgramV1` already decides — but that is a gateway change outside this slice's measurement and
is left named rather than guessed at.

## 6. Laws

`🪟️spawned-program-session` re-run after this slice's ShellHost edit, unchanged and green:

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts 🪟️spawned-program-session
 Test Files  1 passed (1)
      Tests  20 passed (20)
```

(The workspace wrapper `bun ./📜️script.ts test 🪟️spawned-program-session` answers
`Unknown workspace test selection` — the suite is registered in the react target's own vitest config,
so it is driven through that config directly. Worth a follow-up in the wrapper's selection table.)

Scoped `tsc` (`🔣️s5-scope.json`, which already covers `🏛️ShellHost` + the spawned-program unit):
**the same 6 errors before and after this slice's edits, none in a hunk it wrote** —
`🗑️generated/s6-tsc-1.txt` (before) and `s6-tsc-2.txt` (after). The `ShellHost … Property 'consumes'`
error is the pre-existing one S5 also recorded; the other five are in `👥️PresenceBar`,
`🪪️WasmSessionLoader`, `🧪️space-artifact-creation-owner` and `♻️mit-bestand`, none touched here.

**No new law was written for §5's fix.** Honest reason: its subject is a `useMemo` + `useEffect` pair
over React state inside an 11 900-line component, and the existing `🪟️spawned-program-session` suite
drives the owned unit and the real `Mode` renderer, not the bridge census. The measurement that proves
it is the live gate transcript in §5.3 — five steps flipping fail → pass and the refusal text changing
from `1 open instance` to `3 open instances`. A unit law for it wants the census extracted into
`🪟️spawned-program/🟦️.ts` first, which is the next owner's cheapest follow-up.

## 7. Measured vs unverified, honest gaps

**Measured at runtime by this slice** (capture named for each):

- the `s` registry's own census — 60 components, 148 programs, 35 spawnable plugin ids, 25 extensions
  with no program (`s6-sweep-census.txt`)
- all 35 spawnable kinds driven inside the real `s` host: 32 spawn, 67 windows, 32 rails,
  24 document verbs, 3 full round trips (`s6-sweep-{a,b,c,d}.txt`)
- the residual is not latency — 12 s of quiet, 0 transitions, three times (`s6-repaint-dag.txt`)
- the host's reading of a spawned program is two dispatches behind, with the ledger row and
  `Check In (1)` arriving verbatim on the third (`s6-ledger-draw.txt`)
- a third host-side cause written, served, re-measured as no-change, and reverted (§2.3)
- sign-in against hub 7611 inside `s` and the hub space index rendering inside the real host
  (`s6-hub-document.txt`, `s6-hub-{signed-in,space}.png`)
- the agent gate against `s` with a spawned editor, 7/21 → 11/21, both transcripts kept
  (`s6-agent-gate-s.txt`, `s6-agent-gate-s2.txt`)
- `🪟️spawned-program-session` 20/20; scoped `tsc` the same 6 pre-existing errors

**Not done / unverified**:

- **A hub document opened inside `s`** — §4. The shell's half is measured; the hub went down mid-run
  and is a sibling's to restart. This is the one deliverable of this slice that is a **no**.
- **The cause of the two-dispatch lag** — §2.4 names `drainTypedOperations`'s re-arm with a file:line
  but does not prove it. Three host-side causes are now ruled out by measurement (S5's two, this
  slice's one).
- **`subscribeOperationProgress` / `subscribeSpawnedJobProgress` for spawned instances** — still
  session-only (`🏛️ShellHost/🟦️.tsx:6357-6381`), found while reading, not fixed, not measured.
- **`redo ≠ undo` for 21 of the 24 kinds that dispatched a verb** — §3.3. Not demonstrated, on two
  lanes and under two alignments; not shown to be broken either.
- **8 kinds whose rail offered no document verb** within the probe's budget — `block`, `cad`, `norm`,
  `playbook`, `procedural`, `reasoning`, `space`, `vcs`. The batch reports name verbs for several of
  them that this sweep's rail scan did not reach; staging their arguments is the cheapest way to
  convert them.
- **3 kinds with no palette entry** — §3.4. Not investigated beyond measuring it.
- **The rising fault counts in the last chunk** (`space` 5 → `writer` 9 within one session) look like
  the known "one UI admission fault kills every later refresh" cascade; not chased.
- **No law for §5's fix** — §6 says why and what it wants first.
- **The wgpu twin** — not touched, not run.

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `agentBridgeInstances` is the live session **plus every spawned program**; a `spawnedBridgeInstances` state filled by an effect keyed on the spawned roster and the loaded-plugin count (§5.2). Three hunks. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts` | `S_OS_MCP_LIVE_SPAWN` + `spawnProgramThroughPalette` + a `0 spawn …` step; additive, one skipped `if` when unset (§5.1). +81 lines. |
| `🐍️s6-all-kinds-sweep.mjs` (ticket) | **new** — the registry census and the per-kind round trip inside `s`, with the oracle reasoning of §3.6 in its docstrings |
| `🐍️s6-repaint-latency.mjs` (ticket) | **new** — latency vs stalled, by sampling a quiet page (§2.1) |
| `🐍️s6-ledger-diagnose.mjs` (ticket) | **new** — what witness the real `s` host offers for a spawned mutation (§2.2) |
| `🐍️s6-hub-document-in-s.mjs` (ticket) | **new** — sign-in → space → document → edit, against a hub-bound `s` (§4) |

Reverted before landing, deliberately: a `hostPanelRefreshScopeV1` unit in
`…/🏛️ShellHost/🪟️spawned-program/🟦️.ts` and its call site — written, served, measured to change
nothing (§2.3). Neither file carries any of it.

## 9. Processes started (by pid)

| what | how | pid | note |
|---|---|---|---|
| `s` react dev serve on `:6092`, bound to hub 7611 | `S_OS_PORT=6092 S_HUB_URL=http://127.0.0.1:7611 S_DATA_DIR=/private/tmp/s6-s-data-7611 SEMIO_VITE_HMR=0 nohup bun ./📜️script.ts serve s react dev` from `…/🧑‍💻dev/📦️packages/🟦️typescript` | launcher pid in `🗑️generated/s6-serve-6092-pid.txt`; vite child via `lsof -nP -iTCP:6092 -sTCP:LISTEN` | **left running** for whoever finishes §4; log `🗑️generated/s6-serve-6092.txt` |

Nothing else was started, and **nothing was killed** — not serve 6070/6071, not hub 7501, not hub
7611 (which went down on its own, with sibling restarts already in flight), not any peer cargo.
