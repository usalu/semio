# S7 — the spawned-refresh lag at its root, per-instance progress, and deterministic agent targeting

Slice S7, fleet 6, started 2026-09-20. Scope handed over from S6
(`📓️s6-all-plugin-kinds-inside-s.md` §2.4 the named suspect, §2 "two dispatches behind", §5.4
`ReadArtifact` has no rule among several instances, §3.4 three kinds with no palette entry).

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read from
source only.

## 0. tl;dr

| item | result |
|---|---|
| **the two-dispatch lag** | **root found, fixed, measured before and after.** It is none of the delivery queues: the in-page hook shows the spawned instance's `OperationCompleted` arriving on every dispatch, with its `historyPatch`. The shell kept **one history projection for the whole window**, so a spawned program's patches were admitted against ANOTHER document's cursor and silently discarded (`patchCursor 1, currentCursor 3, applied false`, twice). One projection **per program** — `applied:false` → the ledger now reads 1 row after dispatch 1 and 2 after dispatch 2, and `#s-checkin` reads `(1)`/`(2)` (§2) |
| **the sweep, re-run** | **35/35 kinds spawn (72 windows), 35/35 publish their own Actions rail, 34/35 dispatch a document verb, and 22/35 show the COMPLETE mutate → undo → redo** — against S6's `3 / 35`. Same probe, same serve, same verbs (§4) |
| **the 3 kinds with no palette entry** | **they had one — it was unreachable.** `spawn.<pluginId>` was emitted once per APP (so one id named up to five programs) and the only searchable text was a breadcrumb that, for an aggregate plugin, names a DIFFERENT plugin's artifact. Unique ids + the plugin id in the searchable description; all three now spawn inside `s`, and `mathematical` passes the full round trip (§5) |
| **per-instance progress** | `subscribeOperationProgress` / `subscribeSpawnedJobProgress` now run per spawned program (§3) — and the shell's own agent artifact route was the last session-only site of all (§6.3) |
| **agent targeting** | **the live agent gate inside `s` is 21 / 21, exit 0** — up from S6's 11/21, with every `(f)` step driving the SPAWNED `note` editor (instance 3), not the landing app. Three faults, each named by the gate after the previous fix: the gateway had no rule for a capability-less command (`ShellArtifactChannel::for_plugin`, ambiguity refused **by name with its candidates**), the shell's OWN artifact route still bound the session alone (the last session-only site), and the routed channel could not pin a plugin at construction (`pin_plugin` per exchange). `data-semio-artifact-id` now names the focused program (§6) |
| **laws** | `🪟️spawned-program-session` **20 → 33 tests, all green**, including the REGRESSION that reproduces the shared-projection drop, plus **2 new gateway laws** (`shell_channel::` 18/18) (§7) |

## 1. Inherited state (measured)

| thing | state at slice start |
|---|---|
| serve `:6070` | `HTTP 200` |
| serve `:6071` (the real `s` host, hub 7501) | `HTTP 200` |
| serve `:6092` (S6's, hub 7611) | `HTTP 200` |
| serves `:6190` / `:6191` | `HTTP 200` |
| hub `:7611` | **down** (`000`) — HS1 owns its restarts, not contended for |
| hub `:7501` | alive (`404` on `/`, the route no probe uses) |
| predecessor S7 work | none — no `📓️s7-*`, no `🗑️generated/s7-*` |

## 2. The two-dispatch lag — which queue actually lags. **Root found, fixed, measured**

S6 §2.4 named three candidates and could not separate them from the console buffer (which survives
reloads). `🐍️s7-history-hook-probe.mjs` publishes a structured in-page hook instead — one record per
`dispatch-response`, per `spawned-completion` and per `applyHistoryPatch` admission decision, each
carrying the cursor the projection held at that moment and whether the patch was admitted — and
reads it against the DOM ledger after every dispatch.

### 2.1 The measurement (`🗑️generated/s7-history-hook-draw-before.txt`)

Spawned `draw`, `addLayer` from its own rail, inside `s` on `:6071`, signed in as `user1@semio.dev`:

```
dispatch 1: rows=3 checkin="Check In"
  spawned-completion draw#4 operation=64  patchCursor=1 upserts=1
  patch              patchCursor=1  currentCursor=3  applied=false
  dispatch-response  draw#4 addLayer patchCursor=null   (session = space#2)
dispatch 3: rows=3 checkin="Check In"
  spawned-completion draw#4 operation=128 patchCursor=2 upserts=1
  patch              patchCursor=2  currentCursor=3  applied=false
```

That is decisive on all three candidates:

| candidate | verdict |
|---|---|
| the typed-operation drain's re-arm (S6 §2.4's named suspect) | **ruled out** — the terminal completion arrives on the dispatch itself, twice, with its `historyPatch` |
| the per-spawned-instance completion subscription (S5's fix) | **ruled out** — it fires, `draw#4`, with the right operation ids |
| the History/check-in projection | **this is it** — every patch it delivers is DROPPED |

### 2.2 The root, one layer below where S6 left it

It is not that the projection "reads the SESSION's instance". It is that **there was only ONE
projection for the whole window**, and the cursor rule that decides whether a patch is a step
forward (`historyPatchShouldApplyV1`) was taken against **another document's cursor**. The studio
(`space#2`) stood at cursor 3; `draw`'s own document started at 1. So the host's cursor acts as a
FLOOR under the spawned document's, and every patch below it is silently discarded until the spawned
document's own cursor climbs past it. That is why it looked like a fixed "two dispatches behind"
(the floor was 2–3 entries high), why 12 s of quiet changed nothing, and why no host-side refresh
could move it: nothing was late, the patches were being thrown away.

Both stale surfaces are that one state: the History panel's rows and `#s-checkin`'s uncommitted
count (`uncommittedEditCount`, derived from `historyProjection.entries`).

### 2.3 The fix

One history projection **per program**, keyed `${pluginId}#${instanceId}`:

| file | change |
|---|---|
| `🏛️ShellHost/🪟️spawned-program/🟦️.ts` | new `🧾️ProgramHistory` region — `programHistoryKeyV1`, `programHistoryProjectionV1`, `programHistoryProjectionsAfterPatchV1` (which owns the one decision that was missing: WHOSE cursor the admission rule is taken against), `programHistoryProjectionsRetainedV1` |
| `🏛️ShellHost/🟦️.tsx` | `historyProjection` → `historyProjectionByProgram`; `applyHistoryPatch(patch, replace, owner)` at all 10 call sites; `refreshHistorySnapshot(owner)` resolving the handle from the OWNER's plugin (it resolved it from the session's, so a spawned instance id was addressed through the host plugin's handle); the per-program staleness order; the derived `historyProjection` = the FOCUSED program's; a first full snapshot when a spawned program takes the canvas; eviction when a program closes |

### 2.4 After, same probe, same shell, same session (`🗑️generated/s7-history-hook-draw.txt`)

```
dispatch 1: rows=1 checkin="Check In (1)"   spawned-completion draw#4 op=64  patchCursor=1
dispatch 3: rows=2 checkin="Check In (2)"   spawned-completion draw#4 op=128 patchCursor=2
```

N dispatches on a spawned instance → N settled receipts → **N ledger rows after each**, on the
dispatch itself, never N−2. (Dispatches 2 and 4 are the probe's own missed rail clicks: they produce
only a `noteShellCommand` on the STUDIO, and the studio's ledger is now correctly a different one.)

The temporary hook was removed after the second run; the tree carries none of it.

## 3. Per-spawned-instance progress subscriptions

`subscribeOperationProgress` / `subscribeSpawnedJobProgress` were wired for `session.instanceId`
alone (S6 §2, last paragraph) — one more of the session-only decision sites S5 started on. A
spawned editor's running operation (a tool run's trace, a mounted analysis's provisional pieces, a
fill plan's progress) reached no window at all: a frozen canvas until the terminal completion landed.

Landed in `🏛️ShellHost/🟦️.tsx`: the same lane per spawned program, keyed on the spawned roster, each
with its own `spawnedProgramViewStateV1` target so the refresh routes through `refreshSpawnedUi`.
Verified by type-check and by the laws in §7; a live mid-operation trace on a spawned program is
**not** separately measured by this slice.

The agent census S6 wrote inline is now the owned unit's `spawnedBridgeCensusV1` (§6).

## 4. The sweep, re-run — **35/35 spawn, 22/35 full round trip**

S6's `🐍️s6-all-kinds-sweep.mjs` is permanent and unchanged; re-run against the same serve `:6071`,
same sign-in, same `S6_VERBS` map (reconstructed from S6's own captures), in the same four chunks
plus a fifth for the three kinds §5 unblocked: `🗑️generated/s6-sweep-s7{a,b,c,d,e}.txt`.

```
                            S6           S7
spawned windows inside s    32 / 35      35 / 35   (67 → 72 windows)
Actions rail published      32 / 32      35 / 35
document verb dispatched    24 / 32      34 / 35
FULL round trip             3  / 35      22 / 35
zero fault lines            23 / 35      27 / 35
```

| kind | windows | rail rows | verb | moved | undo/redo lane | redo ≠ undo | faults | verdict |
|---|---|---|---|---|---|---|---|---|
| `animate` | 1 | 26 | `addTile` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `architect` | 4 | 33 | `setAdjacencyKind` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `block` | 1 | 22 | `addHandleKind` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `cad` | 4 | 37 | `addNode` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `dag` | 2 | 24 | `addNode` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `demonstrator` | 1 | 11 | `changeSchema` | **yes** | rail / rail | **yes** | 1 | full round trip; the fault is the probe's own `replace-text` attempt |
| `draw` | 1 | 17 | `addLayer` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `energy` | 4 | 34 | `set-surface-property` | **yes** | — / — | no | 0 | |
| `fem` | 2 | 34 | `addNode` | **yes** | — / — | no | 0 | |
| `flow` | 2 | 27 | `addWidget` | **yes** | — / — | no | 1 | |
| `forms` | 2 | 35 | `addStep` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `gis` | 1 | 31 | `addFeature` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `imperative` | 2 | 23 | `addStep` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `layout` | 2 | 19 | `addPage` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `lowpoly` | 1 | 59 | `addPrimitive` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `mathematical` | 2 | 16 | `nodeGraphEdit` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** (was unreachable) |
| `norm` | 2 | 12 | `setSnapshot` | **yes** | — / — | no | 2 | |
| `note` | 2 | 19 | `addBlock` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `playbook` | 1 | 21 | `addStep` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `playbook-module-procedural` | 2 | 11 | `importSolidGeometry` | **yes** | — / — | no | 1 | was unreachable; verb refused `BatchOnlyPendingRewrite` |
| `procedural` | 2 | 23 | `nodeGraphEdit` | **yes** | — / — | no | 0 | |
| `process` | 1 | 19 | `addStep` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `puzzle` | 3 | 30 | `addNode` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `raster` | 2 | 15 | `addLayer` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `reasoning` | 1 | 18 | `addNode` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `remodel` | 2 | 40 | `addStream` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `sequence` | 3 | 29 | `addStep` | **yes** | — / — | no | 2 | |
| `shooting` | 2 | 47 | `addShot` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `sourcing` | 4 | 16 | `stockFromCatalogue` | **yes** | — / — | no | 0 | |
| `space` | 1 | 24 | — | — | — / — | no | 4 | none of its rail rows moved the ledger |
| `stdio` | 1 | 10 | `paste` | **yes** | — / — | no | 2 | |
| `trinity` | 6 | 22 | `patchNodes` | **yes** | — / — | no | 1 | |
| `vcs` | 2 | 19 | `incrementCounter` | **yes** | rail / rail | **yes** | 0 | ✅ **PASS** |
| `wfc` | 2 | 25 | `set-input-pixels` | **yes** | — / — | no | 0 | |
| `writer` | 1 | 16 | `paste` | **yes** | — / — | no | 0 | |

**What moved, and why.** Eight kinds that S6 scored "none of its rail rows moved the document"
(`block`, `cad`, `playbook`, `procedural`, `reasoning`, `norm`, `vcs`) now dispatch a verb, and
nineteen more kinds now show the complete mutate → undo → redo. That is §2's fix: S6's `no` meant
"not demonstrated, because the reading is two dispatches behind", and the readings were being
DISCARDED. With each program's ledger its own, the same probe, the same rail rows and the same undo
lane answer.

**What is still `no` (13 kinds).** They fall into two groups the captures name:

- a verb moved the document but the undo lane did not answer — `energy`, `fem`, `flow`, `norm`,
  `procedural`, `sequence`, `sourcing`, `stdio`, `trinity`, `wfc`, `writer`,
  `playbook-module-procedural`. Their `edits` arrays read `[0,0,0,0]` with the applied-row count
  frozen after the verb, which is a DIFFERENT shape from the one §2 fixed (there the rows arrived,
  just against the wrong cursor). Several carry their own named refusal —
  `playbook-module-procedural`'s is `interactive-job classification BatchOnlyPendingRewrite`, a known
  hard-dead classification — and several are the 3d/simulation kinds whose verb starts a job rather
  than an edit. Not chased by this slice.
- `space`: none of its 24 rail rows moved the ledger, with 4 fault lines. Unchanged from S6.

## 5. The three kinds with "no palette entry" — they had one; it could not be reached

`🐍️s7-palette-census.mjs` dumps every `spawn.*` palette item instead of typing a plugin id at it.
Measured (`🗑️generated/s7-palette-census.txt`):

```
spawn.demonstrator               | Spawn semio · cad
spawn.playbook-module-procedural | Spawn semio · forms
```

Both entries existed all along. Two defects kept them unreachable, and both are now fixed in
`🏛️ShellHost/🟦️.tsx`'s palette block:

1. **The id named a plugin, not a program.** `spawn.${program.pluginId}` was emitted once per APP, so
   `spawn.space` was five different items and `spawn.stdio`/`spawn.cad`/`spawn.note` two each — a DOM
   id that names several programs, which is exactly what every probe and the live agent gate address.
   The first program of a plugin keeps the bare `spawn.<pluginId>` id (so nothing that already
   addresses it breaks); its siblings take `spawn.<pluginId>.<appId>`.
2. **The searchable text was the breadcrumb alone**, and an aggregate plugin's breadcrumb resolves to
   ANOTHER plugin's artifact (`demonstrator` declares apps whose ids are `s.cad.cad@1/*#editor`,
   `s.gis.gismap@1/*#editor`, …). `ShellSearch` ranks `label + description + category`, so the
   description now carries `<pluginId> · <appId>`.

After, same probe:

```
typed "demonstrator"               → spawn.demonstrator | Spawn semio · playground  (+9 siblings)
typed "mathematical"               → spawn.mathematical | Spawn semio · equation
typed "playbook-module-procedural" → spawn.playbook-module-procedural | Spawn semio · forms
```

and the sweep's fifth chunk (`s6-sweep-s7e.txt`) drives all three inside `s`: `demonstrator` opens
`demonstrator-4::framework.window.text` with an 11-row rail and completes `changeSchema` → undo →
redo; `mathematical` opens two windows and PASSES outright; `playbook-module-procedural` opens two
windows and its verb moves the document.

**Home's space rows, for DB1:** `0` in the same run (`s7-palette-census.txt`, `home.treeRows: []`,
`tableRows: []`) — so DB1's symptom is still live on `:6071` after this slice's fix, which is the
honest answer to the coordinator's question: it is a DIFFERENT fault (a mounted operation parking
inside its second `await settle()`), not the one measured in §2.

## 6. Agent targeting — one instance, deterministically

S6 §5.4 left the gate at 11/21 with a named next fault:

```
INTERNAL: `note` refused ReadArtifact (plugin.unavailable): this command carries no capability to
resolve an instance from and the attached shell has 3 open instances — prepare an action against
the artifact first so the route binds
```

`AppCommand::ReadArtifact` is a UNIT variant — it carries no capability and no artifact id — so
`ShellArtifactChannel::resolve_instance_id` had nothing at all to resolve from and refused outright
as soon as the census held more than one row. The census growing (S6's own fix) is what exposed it.

### 6.1 The handle already knows its plugin — it was just never passed

Every caller that opens a shell artifact channel resolves the artifact's owning plugin FIRST
(`open_session_artifact_channel(plugin_id)`, `read_session_artifact_bytes(plugin_id)`) and then threw
that knowledge away. Landed in `🌉️mcp/🐚️channel/🦀️.rs` + `🏠️workspace/🦀️.rs`:

- `ShellArtifactChannel::for_plugin(plugin_id)` pins the plugin whose artifact the channel drives;
  both workspace construction sites now pass the id they already hold.
- `resolve_instance_id`, when the command carries no capability: filter the census to that plugin —
  **exactly one** open instance binds (and is remembered for the rest of the saga); **several**
  refuse **by name, listing the candidates** (`this artifact handle names plugin `note`, which has 2
  open instances … candidates: note:s.note.note:4, note:s.note.note:9`); **none** refuses with the
  same sentence the capability lane already used. Nothing is ever picked silently.

`cargo check -p semio-framework-os-mcp --lib`: 0 errors (§7).

### 6.2 The shell half: focus is the census's order, and the DOM says so

`data-semio-artifact-id` was `agentBridgeInstances[0]?.artifactRef` — correct while the census was
the session alone (WR4 §4's single-instance case), and the LANDING app's id the moment a spawned
editor is open. The census is now built by `spawnedBridgeCensusV1` and published **focused-first**,
so entry 0 — and therefore the DOM attribute (f5) asserts — is the program that owns the canvas.

### 6.3 The last session-only decision site — the shell's OWN artifact route

With §6.1 landed the gate named a new fault, which is how each of these was found:

```
SIDE_EFFECT_REJECTED: `note` rejected ReadArtifact (plugin.unavailable):
this shell's live instance is `1`, not `3` — the agent addressed an instance this shell has closed
```

Instance 3 was exactly the spawned `note` editor the human was looking at. `agentArtifactRouteRef`
(`🏛️ShellHost/🟦️.tsx:10618`) bound `sessionRef.current` and refused every other instance by name —
the LAST of the session-only sites S5 started on (five), S6 continued (the census) and §3 finished
(progress). It now resolves the request's instance against the session **or any spawned program**,
and an instance the shell really does not hold is refused listing what IS open.

One more leg then refused: `action_prepare` travels `ShellRoutedArtifactChannel`, which serves every
plugin of a session and so cannot pin one at construction. It now names the owner per exchange the
way the headless lane already does (`RoutingArtifactChannel::plugin_id_for` — the command's
capability, else the instance slot `prepare_action` minted) and hands it to the shell channel
(`ShellArtifactChannel::pin_plugin`), so both lanes answer "which instance is this?" identically.

### 6.4 The gate — **21 / 21, exit 0**

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust
S_OS_MCP_LIVE_SHELL_URL="http://127.0.0.1:6071" S_OS_MCP_LIVE_PLUGIN=note S_OS_MCP_LIVE_SPAWN=note \
  bun ./📜️script.ts live-agent-loop-check
```

| run | gateway | shell | result | capture |
|---|---|---|---|---|
| 1 | binary of 22:44 (pre-fix) | this slice's TS | **11 / 21** — same as S6, `(f1)` refused `no capability to resolve an instance` | `s7-agent-gate-s.txt` |
| 2 | rebuilt with `for_plugin` | same | **11 / 21**, `(f1)` now refused by the SHELL: `live instance is 1, not 3` | `s7-agent-gate-s2.txt` |
| 3 | same | route resolves spawned programs | **11 / 21** with **`(f1)` PASS** (and one flaky `boot`), `(f2)` refused on the routed channel | `s7-agent-gate-s3.txt` |
| 4 | `pin_plugin` per exchange | same | **21 passed, 0 failed, 0 skipped of 21** | `s7-agent-gate-s4.txt` |

```
PASS (f3) action_invoke changes the head :: status=SUCCEEDED
     before={"artifactId":"note:s.note.note@1/*#editor:3","cursor":"0","headEditId":""}
     after ={"artifactId":"note:s.note.note@1/*#editor:3","cursor":"1","headEditId":"apply"}
PASS (f5) the live shell shows the same artifact :: agent id live-agent-loop-muacadjm →
     shell route ref note:s.note.note@1/*#editor:3, carried by [data-semio-artifact-id] in the live DOM
PASS (f8) artifact_export :: contentBase64=1476 char(s)
os-mcp-live-agent-loop: 21 passed, 0 failed, 0 skipped of 21
```

Instance **3** throughout: every (f) step drove the SPAWNED `note` editor inside `s`, not the
landing app — which is outcome 4's acceptance, an MCP agent editing a spawned editor inside the real
`s` host. Preamble rule 26 was checked before each build: `ps … grep -c "cargo test -p
semio-framework-os-mcp"` answered **0**.

## 7. Laws

`🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx` — **20 → 33 tests, all passing**:

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts 🪟️spawned-program-session
 Test Files  1 passed (1)
      Tests  33 passed (33)
```

The thirteen added:

| law | what it pins |
|---|---|
| `N dispatches on a spawned instance leave N rows in ITS ledger, after each one` | the §2 contract: `[1,2,3,4]` rows after dispatches 1…4, each patch admitted |
| `REGRESSION: one shared projection drops every one of those patches` | the live defect, reproduced: with the host at cursor 3 the admissions are `[false,false,true]` |
| `the host's own ledger is untouched while a spawned program is edited` | the host keeps cursor 3 while the spawned one reaches 1 |
| `a program nothing has been read for projects nothing` | an unread program shows the EMPTY projection, never another program's rows |
| `drops the projections of programs that are no longer open` | eviction, and identity preserved when nothing is dropped |
| `is the session PLUS every spawned program` / `marks exactly ONE row focused` / `does not confuse two instances of the same plugin` / `is published focused-first` | the agent census (§6) |
| `subscribes operation PROGRESS per spawned instance too` | §3 |
| `applies every history patch to the program that produced it` | the owner argument at the completion and dispatch sites |
| `reads a history SNAPSHOT through the owner's own plugin handle` | the handle-resolution bug §2.3 names |
| `shows the FOCUSED program's ledger` | the derived projection |

Scoped `tsc` (`🔣️s5-scope.json`): the **same 6 pre-existing errors before and after** this slice's
edits, none in a hunk it wrote — `🗑️generated/s7-tsc-1.txt`, `s7-tsc-2.txt` (`ShellHost:5121
Property 'consumes'` is the one S5 and S6 also recorded; the other five are in `👥️PresenceBar`,
`🪪️WasmSessionLoader`, `🧪️space-artifact-creation-owner` and `♻️mit-bestand`).

`cargo check -p semio-framework-os-mcp --lib` and `--all-targets` after the gateway change:
**0 errors** (`🗑️generated/s7-mcp-check.txt`). Two laws added in
`🌉️mcp/🐚️channel/🧪️tests/🔬️quick/🦀️.rs` —
`a_capability_less_command_resolves_the_one_open_instance_of_its_own_plugin` and
`an_ambiguous_artifact_handle_is_refused_by_name_with_its_candidates`:

```
CARGO_TARGET_DIR=…/⚡️cache/cargo/target-s7 cargo test -p semio-framework-os-mcp --lib shell_channel::
test result: ok. 18 passed; 0 failed
```

Preamble rule 26 honoured at every build: `ps -axo command | grep -c "cargo test -p
semio-framework-os-mcp"` answered **0** each time, and rule 25's private target dir was used for the
test run.

## 8. Measured vs unverified, honest gaps

**Measured at runtime by this slice** (capture named for each):

- which of the three candidate queues lags, by in-page hook, before AND after the fix —
  `s7-history-hook-draw-before.txt`, `s7-history-hook-draw.txt` (§2)
- N dispatches on a spawned instance → N ledger rows and `Check In (N)`, on the dispatch itself (§2.4)
- the sweep re-run inside the real `s` host, all 35 kinds (§4), captures `s6-sweep-s7{a,b,c,d}.txt`
- `🪟️spawned-program-session` **33 passed**; scoped `tsc` the same 6 pre-existing errors;
  `cargo check -p semio-framework-os-mcp --lib` and `--all-targets` 0 errors (§7)
- the palette census before and after the fix (`s7-palette-census.txt`, §5)
- **the live agent gate inside `s`, 21 / 21, exit 0** — four runs, each capture kept (§6.4)
- the two new gateway laws, `cargo test -p semio-framework-os-mcp --lib shell_channel::` →
  **18 passed / 0 failed** under a private target dir (rule 25) — `s7-mcp-laws.txt`

**Not done / unverified**:

- **One flaky `boot` step** — gate run 3 reported `boot :: ready=null error=s` while every later
  step against that same page passed, and run 4 booted clean. Not chased; if it recurs it is a boot
  beacon race in the gate's own step 0, not a route fault.
- **`(e1)`/`(e2)` passed only in run 4.** They were failing on the approval affordance in runs 1–3
  (`timed out waiting for the approval affordance after 30000ms`) and passed once the (f) chain
  stopped cascading. The gate is green as a whole; those two steps were not independently bisected.
- **A spawned program's mid-operation progress** (§3) is type-checked and pinned by a source law;
  no live trace was driven through it.
- **DB1's Home-lists-spaces symptom** — see §5; separate lane, separate fault (a mounted operation
  parking inside its second `await settle()`), not the one measured here.
- **The wgpu twin** — untouched. `🧊️wgpu/🐚️plugin-bridge/🟦️.ts` keeps its own `drainTypedOperations`
  and its own history lane; whether it shares the single-projection defect was not measured.

## 9. Files changed

| file | change |
|---|---|
| `…/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪟️spawned-program/🟦️.ts` | new `🧾️ProgramHistory` region (`programHistoryKeyV1`, `programHistoryProjectionV1`, `programHistoryProjectionsAfterPatchV1`, `programHistoryProjectionsRetainedV1`) and new `📇️SpawnedBridgeCensus` region (`BridgeProgramRefV1`, `spawnedBridgeCensusV1`) |
| `…/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | per-program history projection (state, `applyHistoryPatch(…, owner)` at every call site, owner-resolved `refreshHistorySnapshot`, per-program staleness order, the focused-program derivation, a spawned program's first snapshot, eviction); per-spawned-instance `subscribeOperationProgress`/`subscribeSpawnedJobProgress`; the agent census through the owned unit, published focused-first; `agentArtifactRouteRef` resolves a spawned program as well as the session; unique `spawn.*` palette ids with the plugin id in the searchable description |
| `…/🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx` | +13 laws (§7) |
| `🌉️mcp/🐚️channel/🦀️.rs` | `ShellArtifactChannel::for_plugin` / `pin_plugin`; a capability-less command resolves the pinned plugin's single open instance and refuses an ambiguous handle by name, listing the candidates |
| `🌉️mcp/🐚️channel/🧪️tests/🔬️quick/🦀️.rs` | +2 laws: the single-instance resolution and the by-name ambiguity refusal |
| `🌉️mcp/🏠️workspace/🦀️.rs` | both direct shell-channel construction sites pass the plugin id they already resolved; `ShellRoutedArtifactChannel::exchange` names the owner per exchange through `RoutingArtifactChannel::plugin_id_for` |
| `🐍️s7-history-hook-probe.mjs` (ticket) | **new** — the in-page hook that separated the three candidate queues (§2) |
| `🐍️s7-palette-census.mjs` (ticket) | **new** — every `spawn.*` palette item with an EMPTY query, plus Home's space rows (§5) |

Added and removed deliberately: a temporary `__s7Hook` publication in `applyHistoryPatch`, the
spawned completion subscription and the dispatch-response path — three lines, present for exactly two
probe runs (§2.1, §2.4), removed before the final type-check. The tree carries none of it.
