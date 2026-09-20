# S7 — the spawned-refresh lag at its root, per-instance progress, and deterministic agent targeting

Slice S7, fleet 6, started 2026-09-20. Scope handed over from S6
(`📓️s6-all-plugin-kinds-inside-s.md` §2.4 the named suspect, §2 "two dispatches behind", §5.4
`ReadArtifact` has no rule among several instances, §3.4 three kinds with no palette entry).

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read from
source only.

## 0. tl;dr

(filling)

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
alone (S6 §2, last paragraph) — the last of the six session-only decision sites S5 started on. A
spawned editor's running operation (a tool run's trace, a mounted analysis's provisional pieces, a
fill plan's progress) reached no window at all: a frozen canvas until the terminal completion landed.

Landed in `🏛️ShellHost/🟦️.tsx`: the same lane per spawned program, keyed on the spawned roster, each
with its own `spawnedProgramViewStateV1` target so the refresh routes through `refreshSpawnedUi`.
Verified by type-check and by the laws in §7; a live mid-operation trace on a spawned program is
**not** separately measured by this slice.

The agent census S6 wrote inline is now the owned unit's `spawnedBridgeCensusV1` (§6).

## 4. The sweep, re-run

(filling)

## 5. The three kinds with no palette entry

(filling)

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

### 6.3 The live gate

**Not re-run by this slice**, and the reason is preamble rule 26, measured rather than assumed:
`ps -axo command | grep -c "cargo test -p semio-framework-os-mcp"` answered **2** at the time of the
gateway change, and `live-agent-loop-check` builds that crate. Starting a third builder is forbidden.
The gate's own recipe (`S_OS_MCP_LIVE_SHELL_URL=http://127.0.0.1:6071 S_OS_MCP_LIVE_PLUGIN=note
S_OS_MCP_LIVE_SPAWN=note …`) is unchanged and the next owner can run it directly. **11/21 stands as
the last measured number**; this slice's §6.1/§6.2 are type-checked, not gate-measured.

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

`cargo check -p semio-framework-os-mcp --lib` after the gateway change: **0 errors**, 15 warnings,
3m49s (`🗑️generated/s7-mcp-check.txt`). Preamble rule 26 honoured: `ps` showed 2 peer
`cargo test -p semio-framework-os-mcp` runs, so no third test build was started by this slice.

## 8. Measured vs unverified, honest gaps

**Measured at runtime by this slice** (capture named for each):

- which of the three candidate queues lags, by in-page hook, before AND after the fix —
  `s7-history-hook-draw-before.txt`, `s7-history-hook-draw.txt` (§2)
- N dispatches on a spawned instance → N ledger rows and `Check In (N)`, on the dispatch itself (§2.4)
- the sweep re-run inside the real `s` host, all 35 kinds (§4), captures `s6-sweep-s7{a,b,c,d}.txt`
- `🪟️spawned-program-session` **33 passed**; scoped `tsc` the same 6 pre-existing errors;
  `cargo check -p semio-framework-os-mcp --lib` and `--all-targets` 0 errors (§7)

**Not done / unverified**:

- **The live agent gate was not re-run** — preamble rule 26: two peer `cargo test -p
  semio-framework-os-mcp` builds were running, and `live-agent-loop-check` builds that crate. 11/21
  (S6) stands; §6.1/§6.2 are type-checked and check-compiled, not gate-measured.
- **No Rust law for the new `ReadArtifact` resolution.** A `#[cfg(test)]` law in
  `🐚️channel/🧪️tests/🔬️quick` would be compiled and RUN by the two peer test builds already in
  flight, and an untested assertion of mine failing inside a sibling's gate is worse than none. It is
  the next owner's first ten minutes, with the crate free.
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
| `…/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | per-program history projection (state, `applyHistoryPatch(…, owner)` at every call site, owner-resolved `refreshHistorySnapshot`, per-program staleness order, the focused-program derivation, a spawned program's first snapshot, eviction); per-spawned-instance `subscribeOperationProgress`/`subscribeSpawnedJobProgress`; the agent census through the owned unit, published focused-first |
| `…/🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx` | +13 laws (§7) |
| `🌉️mcp/🐚️channel/🦀️.rs` | `ShellArtifactChannel::for_plugin`; a capability-less command resolves the pinned plugin's single open instance and refuses an ambiguous handle by name, listing the candidates |
| `🌉️mcp/🏠️workspace/🦀️.rs` | both shell-channel construction sites pass the plugin id they already resolved |
| `🐍️s7-history-hook-probe.mjs` (ticket) | **new** — the in-page hook that separated the three candidate queues (§2) |
| `🐍️s7-palette-census.mjs` (ticket) | **new** — every `spawn.*` palette item with an EMPTY query, plus Home's space rows (§5) |

Added and removed deliberately: a temporary `__s7Hook` publication in `applyHistoryPatch`, the
spawned completion subscription and the dispatch-response path — three lines, present for exactly two
probe runs (§2.1, §2.4), removed before the final type-check. The tree carries none of it.
