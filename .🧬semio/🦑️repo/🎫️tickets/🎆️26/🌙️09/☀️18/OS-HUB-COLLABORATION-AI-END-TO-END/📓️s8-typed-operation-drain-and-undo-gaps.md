# S8 — the typed-operation drain stall, and the 13 kinds without undo/redo inside `s`

Slice S8, session 6, 2026-09-20. Inherited from `📓️db1-directory-bootstrap-receipt.md` §4.3 (the
mounted operation that never publishes a terminal) and `📓️s7-spawned-refresh-lag-and-agent-targeting.md`
§4 (22/35 full round trip, 13 kinds with `edits [0,0,0,0]`).

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read from
source only.

## 0. tl;dr

| item | result |
|---|---|
| **the "drain stall"** | **there is none.** Measured with an in-page hook on the brief's own pair (serve `:6190` → hub `7611`): the drain polls fine, ~14.5 s per settle, both schedulers idle. DB1's "parks in its second `await settle()`" and its `UserVisible`-starvation suspicion are both **disproved** (§2.1) |
| **the real root** | **guest-side, found and fixed.** Home's config preparation answered `Blocked` unless it was granted 1 MiB in one step, while the publication ladder grants 4 KiB per unit — so `applyDirectoryEventPage` sat in `Publishing` for ever, silently, keeping the actor in `MoreWork` (§2.4–2.6) |
| **the runtime law** | a `Publishing` operation that has not moved for 4 096 units is now terminated with `interactive-job.publication-stalled` instead of spinning, and a bounded stall trace names the stage and the ownership flags on the way past (§2.3, §2.6.2) |
| **two more, exposed by the fix** | the receipt's real carrier is a `publish-event` (not a lane-7 page) and must be decoded through the pack projection; and the terminal output has TWO carriers, only one of which was read (§2.7) |
| **Home lists spaces** | **no.** The bootstrap now reaches `idle` — receipt matched byte-for-byte, ACK published — but the table still renders "No studios yet" while the hub holds one studio and the identities match exactly. The next stage is named at file:line (§3) |
| **the 13 undo/redo gaps** | clustered and attributed: **3** already complete the round trip and are lost to the `#s-checkin` oracle, **8** fail because the sweep cannot fill a staged form (6 of them PASS on their own serve), **2** carry named refusals — one host-side (`norm`), one hard-dead (`playbook-module-procedural`) (§4) |
| **the sweep** | re-run for six kinds with corrected verbs: **0/6**. The ≥ 30/35 target is **not** reached and no 35-kind re-sweep was run (§4.4) |

## 1. Inherited state (measured)

| thing | state at slice start |
|---|---|
| serve `:6070` | `HTTP 200` |
| serve `:6071` (the real `s` host, hub 7501) | `HTTP 200` |
| serve `:6092` | `HTTP 200` |
| serve `:6190` / `:6191` | `HTTP 200` |
| hub `:7501` | alive (`404` on `/`) |
| hub `:7611` | alive (`404` on `/`) — up again since DB1's §4.1 hand-off |
| predecessor S8 work | none — no `📓️s8-*`, no `🗑️generated/s8-*` |

## 2. The typed-operation drain stall — **it is not a stall, and it is not the host**

### 2.1 DB1's "parks inside its second `await settle()`" is disproved, measured

`🐍️s8-drain-hook-probe.mjs` (new, permanent) signs in on the brief's own pair (serve `:6190` → hub
`7611`, both alive at slice start) and reads an in-page hook the drain publishes: ONE record per poll
stage (`poll-begin`, `call-open`, `thunk-start`, `submit-done`, `settle-done`, `call-done`,
`poll-end`, `yield-begin`, `yield-end`, `drain-outcome`, `drain-error`), each carrying the plugin-turn
scheduler's and the command-ingress thunk scheduler's state for that actor at that instant. Capture
`🗑️generated/s8-drain-hook-2.txt`:

```
t=24868 poll=1 poll-begin    turnBusy=false turnPending=0 thunkBusy=false thunkPending=0
t=24868 poll=1 thunk-start   thunkBusy=true
t=24981 poll=1 submit-done (more-work)   turnBusy=true
t=39512 poll=1 settle-done  (more-work)            ← +14.5 s
t=39512 poll=1 poll-end · yield-begin · yield-end
t=39512 poll=2 poll-begin … t=54105 poll=2 settle-done (more-work)   ← +14.6 s
t=54106 poll=3 poll-begin … t=68626 poll=3 settle-done (more-work)   ← +14.5 s
t=68627 poll=4 poll-begin …
```

Read exactly:

- **the drain is not parked.** It completes poll after poll (4 polls in 44 s) and re-arms itself; the
  `yield` resolves in under a millisecond; `drainTypedOperationTurns` never exceeds its budget;
- **neither scheduler is starved.** `turnPending` and `thunkPending` are 0 throughout, so the
  `lane: "UserVisible"` starvation DB1 suspected (its own §4.3, "unverified") **does not happen**;
- **each settle costs ~14.5 s** and returns `more-work` — that is `settlePluginTurn` running its
  zero-progress ladder to `PLUGIN_UI_QUIESCENT_CONTINUATIONS` and quiescing, i.e. ~128 shard round
  trips in which the guest published nothing, acknowledged nothing and emitted nothing;
- **no result page ever arrives** (`frames=0`, leftover empty), so no terminal, so no completion.

DB1 saw only poll 1 because its probe waited 30 s and its instrumentation logged per poll; the drain
was advancing the whole time. **The host-side defect DB1 handed on does not exist.**

### 2.2 The guest's own more-work streak names the lane

Re-run with the guest's runtime diagnostics armed (`localStorage SEMIO_RUNTIME_DIAGNOSTICS=1`, which
`🎭️actor/🩺️diagnostics/🟦️.ts` stamps onto the shard-worker URL and the guest reads through
`wasi:cli/environment`) — capture `🗑️generated/s8-drain-hook-4.txt`:

```
[DEBUG] plugin_exchange actionId=applyDirectoryEventPage branch=catalog
[actor] [DEBUG] reactor more-work streak=2 seen=10 sources=["typed_operation"] contended=false effects=2
  patches=[slots=[2:s-home-main#g7:--R:ack1/rev1:outNone, …all six --R:ack1/rev1:outNone]
  ready=[] terminals=[] producer_terminals=[] deferred=[] rejected=0 unadmitted=0 closing=0
  output_fault=none reserve_refusal=none …] pending=[slots=[] handback_empty=true exhausted=false closing=0]
[actor] … streak=3,4,5,6,7 … sources=["typed_operation"] effects=0   (unchanged, for ever)
```

`sources=["typed_operation"]` is the **only** source, every retained-surface slot is clean
(`--R:ack1/rev1:outNone`), nothing is pending, nothing is closing, and `process_pool` is absent from
the source set. So the spin is the typed-operation publication lane alone, inside `🪐️space`'s Home
instance, and it is neither the reconcile tracker (the `🧠️` family of
`📓️…reconcile-tracker-more-work-spin`) nor the worker pool.

### 2.3 The instrument the runtime was missing

A mounted operation that never publishes its terminal is **silent by construction**: every arm of
`advance_typed_operation_publication_one` that cannot advance returns `Ok(())` (a completion that is
not ready, `publish_mounted_typed_operation_unit`'s `completion.take()? else return Ok(())`,
`🔌️plugin/🦀️.rs:27708`), the host's drain keeps polling, and the only thing any instrument says is
`more-work`. Landed a permanent, self-limiting diagnostic in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

| addition | what it does |
|---|---|
| `TYPED_OPERATION_STALL_WITNESS_FLOOR` (64) | consecutive unchanged units before anything prints |
| `typed_operation_stall_witness` | the three facts that decide which ladder arm runs: selected operation, its stage, and the eight ownership flags (`session`, `session_rejected`, `completion`, `publication`, `pending_artifact_publication`, `terminal_outcome`, `terminal_seen`, `result_page`) |
| `typed_operation_stall_streak` | streak of unchanged witnesses; answers only on a power of two ≥ 64, so one stalled operation costs a handful of lines however long it spins |
| `advance_typed_operation_publication_one` | now a wrapper around the renamed `…_unit`, printing `[DEBUG] typed-operation <verb> advanced N units with no change: operation=… stage=… flags=0b…` |

`cargo check -p semio-framework-plugin --lib` → **0 errors, 39 warnings** (warnings are the proof the
expansion ran), capture in this section's log.

### 2.4 What the instrument named, first run (`🗑️generated/s8-drain-hook-6.txt`)

Re-staged through `📜️s8-restage.sh` (S4's recipe on S8's private target dir; component dev →
`space-plugin:materialize-dev` → `activate s react dev`, each step through the fleet wasm mutex;
`Activated s react dev: 60 completed components (changed)`):

```
[DEBUG] typed-operation applyDirectoryEventPage advanced  64 units with no change: operation=64 stage=1 flags=0b01011100
[DEBUG] typed-operation applyDirectoryEventPage advanced 128 units with no change: operation=64 stage=1 flags=0b01011100
[DEBUG] typed-operation applyDirectoryEventPage advanced 256 units with no change: operation=64 stage=1 flags=0b01011100
```

`stage=1` is `Publishing`; `flags` reads (LSB first) `session=0 sessionRejected=0 completion=1
publication=1 pendingArtifactPublication=1 terminalOutcome=0 terminalSeen=1 resultPage=0`. So the job
ran, its completion was installed, its emit was staged, and the operation was sitting on its **store
publication** — `PendingArtifactStorePublication::Config` — which never advanced.

### 2.5 The root: a guest owner that demands its whole envelope in one granted page

`🔌️plugin/🦀️.rs:27874` maps `ArtifactStoreOneItemAdvance::Progress(_) | Blocked => Ok(())`, and the
ladder grants exactly `ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes:
TYPED_OPERATION_RESULT_PAGE_BYTES }` — **4 096 bytes**. Home's config preparation answered:

```rust
// ✏️s/🔌️plugins/🪐️space/…/🏠️home/…/✏️editor/🦀️.rs:301   (before)
if !grant.permits_one() || grant.maximum_bytes < HOME_CONFIG_STEP_BYTES || self.cancelled { return Ok(Blocked); }
```

`HOME_CONFIG_STEP_BYTES` is `store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES` = **1 MiB**, so
`4096 < 1048576` was true on every unit and the preparation answered `Blocked` **for ever**. This is
the second law of `project-fixed-operation-registry-shared-close-cursor` exactly: *an owner whose step
returns `Blocked` unless it is handed its whole declared envelope never advances in production, and
only its hand-rolled test passes.* S4's fix (4 MiB/16 MiB → 1 MiB) made `is_admissible` accept the
declared footprint but left the per-step demand at the same 1 MiB, which no pump ever grants. The
grant's own contract is "consume at most one semantic unit", not "here is your whole budget".

`close_step` (`:358`) had the identical demand and additionally reported `released_bytes:
HOME_CONFIG_STEP_BYTES`, above any grant.

### 2.6 The three landed fixes

| # | file | change |
|---|---|---|
| 2.6.1 | `✏️s/🔌️plugins/🪐️space/…/🏠️home/…/✏️editor/🦀️.rs` | `advance` and `close_step` demand only `grant.permits_one()`; the close releases ONE retained owner per granted page and reports `released_bytes: grant.maximum_bytes` |
| 2.6.2 | `🔌️plugin/🦀️.rs` | the stall instrument of §2.3 **plus the runtime law**: `fault_stalled_typed_operation_publication` terminates a `Publishing` operation that has not moved for `TYPED_OPERATION_STALL_FAULT_CEILING` (4 096) units with `interactive-job.publication-stalled`, cancelling its lease and queueing a real fault page. Only `Publishing` is bounded — a `Worker`-stage operation's progress is its own job's business, and a long tool run legitimately leaves the witness unchanged |
| 2.6.3 | `🔌️PluginRuntime/🟦️.tsx` | two receipt-delivery defects the guest fix then exposed (§2.7) |

### 2.7 Two further defects, each measured, each fixed

With the publication unblocked the operation settled **inside its own command turn** (the drain never
armed again: `drain records: 0`), and the lane moved to `directory-bootstrap.receipt-mismatch`. Two
facts, both read off the live wire:

1. **The receipt's real carrier is a `publish-event`, not a lane-7 result page.**
   ```
   [S8PROBE] completion instance=1 operation=64 completionTags=[]
     turn=[{"tag":"publish-event","val":{"topic":"semio.space.home.directory-projection-receipt.v1","payload":{…}}},
           {"tag":"typed-operation-terminal-seen"}]
   ```
   `consumeTypedOperationEffects` only lifted the receipt out of a `TYPED_OPERATION_LANE` 7 page, so
   the terminal output was always `undefined`. It now also recognises a `publish-event` whose `topic`
   is the receipt schema, decoding the payload through `decodeWirePack` — the pack projection, because
   a raw `decodePackValue` answers integer CARRIERS and `parseDirectoryProjectionReceiptV1`'s
   `Number.isSafeInteger` guards read those as `[object Object]` (measured: `gen=[object Object]
   seq=[object Object]`, the trap `project-pack-integer-carriers-need-exact-json-projection` names).
2. **The terminal output has two carriers, and only one was read.** An operation the drain finished
   parks it in `pendingCompletionEffects`; an operation that finished inside its own command turn
   parks it in `pendingTurnEffects`. New `takeTypedOperationTerminalOutputV1(instanceId)` takes it
   from whichever holds it and removes it from the invocation's carrier, so the completion delivers it
   exactly once and the admitting reply's `output` stays the operation HANDLE (which is DB1's law).

### 2.8 Measured after, on serve 6190 → hub 7611

| | before this slice | after |
|---|---|---|
| the drain | 1 poll per 14.5 s, for ever, no terminal | never arms — the operation settles inside its command turn |
| the reactor | `more-work streak` → thousands, `sources=["typed_operation"]` | streak ends |
| the notice | `pending` "Updating directory through sequence 9", for ever | **`[]` — `idle`: the receipt matched and the ACK was published** |
| the receipt | never delivered | `terminal=3afdaf10…67e8 gen=1 seq=9` = `pageReceipt=3afdaf10…67e8 pageGen=1 pageSeq=9` |
| Home's rows | 0 | **still 0** (§3) |

Laws green after the change (`🗑️generated/s8-bootstrap-laws.txt`):
`directory-home-bootstrap-oracle: checks=33 clean`, **14/14** bootstrap laws, **2/2** PluginRuntime
completion-delivery laws. The oracle's runtime clause was rewritten to pin the stronger contract (both
carriers, the `publish-event` receipt, the pack projection) instead of the exact old expression.

## 3. Home lists spaces — **no, and the next stage is named**

Measured on serve `:6190` → hub `7611`, signed in as `user1@semio.dev`, 95 s settle
(`🗑️generated/s8-journey-2.txt`):

```
bootstrap: []                      ← idle: receipt matched, ACK published
windows:   ["s-home-main"]
rows:      []
body:      "… Studios  Create Space  No studios yet. Create one from the navbar. …"
```

The hub is **not** empty and the identity **does** match — read off the shell's own requests
(`🐍️s8-directory-body-probe.mjs`, capture `🗑️generated/s8-directory-body.txt`):

```
GET /directory/spaces        → [{"access":"author","space":{"id":"01a0c00f-4f3c-7834-a7e6-2ccf9de925db",
                                 "name":"GM1 Shared Map 7161c795","kind":"studio",
                                 "ownerUserId":"01a0c00d-cd33-7948-91cb-da23affa54ec","documentCount":1,…}}]
GET /auth/sessions/me        → {"userId":"01a0c00d-cd33-7948-91cb-da23affa54ec","displayName":"User 1",…}
GET /directory/event-page/v1 → throughSeqInclusive 9, seq 4 = space.created(GM1 Shared Map …, studio)
```

So the frontier carries the space, the shell's `userId` is byte-identical to the directory's
`ownerUserId`, the receipt for that exact page is accepted, and the ACK is published — and Home's
table still renders its empty case.

**The next stage, named at file granularity and NOT fixed here.** Home's table is
`crate::home_space_rows(&directory, &identity.user_id)`, called from
`✏️s/🔌️plugins/🪐️space/…/🏠️home/…/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:214-218`,
where `directory` is `cfg.directory()` off the CONFIG the operation published. Everything before that
call is now proven live; what is not proven is that the published config reaches that render. The two
candidates, in the order the next owner should test them, are (a) the `ReplaceDirectoryProjection`
config mutation landing on an instance whose surface is not the visible one — the probes show
`applyDirectoryEventPage` running on instance 1 AND instance 2 within one session, with the first
owner dying `directory-bootstrap: owner retired before its operation settled` and the second timing
out once before the third succeeds; and (b) `home_space_rows`' own membership filter over the folded
projection. Both are one `[DEBUG]` line away now that the operation settles, which it did not before.

### 3.1 Three consecutive page offers, three different outcomes

Worth recording because it bounds how long the lane takes to converge
(`🗑️generated/s8-drain-hook-13.txt`, `-15.txt`):

| offer | outcome |
|---|---|
| 1 | `directory-bootstrap: owner retired before its operation settled` |
| 2 | `directory-bootstrap: typed operation 64 published no terminal receipt within 30000 ms` |
| 3 | receipt matches, ACK published, notice clears |

So the ACK lands ~60–70 s after sign-in rather than immediately — a 35 s journey probe reads the
pre-ACK state. Attributing offers 1 and 2 is the same instance question as (a) above.

## 4. The 13 kinds without undo/redo — clustered into three shapes, two of them the probe's

S7's 13 (`energy`, `fem`, `flow`, `norm`, `procedural`, `sequence`, `sourcing`, `stdio`, `trinity`,
`wfc`, `writer`, `playbook-module-procedural`, plus `space`) read `edits [0,0,0,0]`. Parsing S7's own
five captures per row (`🗑️generated/s6-sweep-s7{a..e}.txt`) and re-running six of them on `:6071`
with the verbs each artifact's single-plugin bar report PROVED
(`🗑️generated/s6-sweep-s8a.txt`, tag `s8a`) separates them into three shapes. The decisive extra
field is `lastLedgerRow`, which S7's table does not show.

### 4.1 Shape 1 — the round trip HAPPENS; `edits` is the wrong oracle (3 kinds)

```
flow        verb=addWidget  applied=[1,3,6,9]  last="framework.history.entry.3:Add Widget↶"
sequence    verb=addStep    applied=[2,4,6,9]  last="framework.history.entry.4:Add Step↶"
procedural  verb=generate   applied=[1,3,3,3]  last="framework.history.entry.3:show-mode value=generate↶"
```

The applied ledger grows on the verb and again on undo and redo, and the verb's own entry carries the
`↶` undone marker — that IS the undo witness `🐍️b3d-interaction-probe.mjs` uses against the
single-plugin serves. What reads `0` is `edits`, the **uncommitted-edit count** the sweep takes from
`#s-checkin`, and since S7 §2.3 that count is derived from the **FOCUSED** program's projection while
the sweep reads it with the studio owning the canvas. So the sweep's undo verdict asks the wrong
surface for these three. **Named, not fixed**: the fix is one line in the sweep (judge undo from the
`↶` marker on the verb's own entry, as the single-plugin probe does) or one in `🏛️ShellHost` (publish
`#s-checkin` per program rather than for the focused one). `procedural` additionally needed its verb
corrected — S7 drove `nodeGraphEdit`; the offered, working verb is `generate`.

### 4.2 Shape 2 — the verb emits nothing, and the row the probe scored is the SHELL's (8 kinds)

```
energy   known=rename-zone OFFERED, tried, applied=[5,5,5,5]  → no edit; scored on set-surface-property
         last="framework.history.entry.5:Activate Window"
fem      verb=addNode  applied=[0,1,1,1]  last="framework.history.entry.1:Activate Window"
```

and, from S7's captures, the same `Activate Window` last row for `sourcing`, `wfc`, `trinity`, `norm`,
and `applied=[0,0,0,0]`/`lastLedgerRow: null` for `stdio` and `writer`. The sweep's `mutated` predicate
is "the applied ledger grew", and the shell's own `Activate Window` command grows it — so `mutated:
true` here means the window was focused, not that the document changed. **This is a probe oracle
defect, and it is why S7's table reads "verb dispatched and moved the document" for kinds that moved
nothing.**

Underneath it, the real reason these verbs emit nothing is that each needs an argument or a selection
the sweep never supplies. The bar reports drive them with staged forms —
`energy` `rename-zone zone=1 newName=ProbeZone` (b3d, PASS), `sourcing` `curationSetCount` with a Pool
stepper cell `delta +1` (b3f, PASS 5/5), `wfc` `change-seed` (b3b, PASS ×3), `trinity` `patchNodes` /
`setParameter` (b3b, PASS ×2), `fem2d` `addNode x=3.5 y=4.5` (b3f, PASS 5/5) — while the sweep's
`runVerb` recorded `filled: []`, `submitted: "absent"`. Passing `S6_ARGS` for `energy.rename-zone` and
`sourcing.curationSetCount` this slice was **not** enough: the row is clicked, the staged form opens,
and nothing fills it. Two of the eight are known dead ends rather than probe gaps: `writer` has **no
mutating verb in its Actions pane at all** (F1 measured exactly this: only `formatDocument`, inert on
an unedited document, `lintDocument` on the transient lane, and `setActiveExample`), and `fem3d`'s
`addSupport` was already measured as a no-op twice (b3f, 4/5).

Also corrected here, and worth carrying into the sweep's map: S7's `S6_VERBS` spelled `wfc`'s verb
`changeSeed`; the artifact's own id is **`change-seed`**, which is why it was never offered.

### 4.3 Shape 3 — two named refusals, one host-side and one hard-dead (2 kinds)

| kind | refusal, verbatim from the capture | side |
|---|---|---|
| `norm` | `input #91 setSnapshot refused: dispatch-failed (user window=norm-10::norm-din16798-inputs) — action 'setSnapshot' is not a framework-reserved action (history/clipboard/revert/fil…)` | **host** — `setSnapshot` is norm's PROVEN verb (B2c.1: "norm — din4108 boot→mutate→undo→redo ✅ full bar, 0 faults" after the `NormOneItemPreparationFactory` root fix), so the `s` shell routing it down the framework-reserved lane is a dispatch-routing defect inside `s`, not a guest one |
| `playbook-module-procedural` | `UI dispatch rejected action:importSolidGeometry` | **guest** — the `BatchOnlyPendingRewrite` interactive-job classification, hard-dead in the app by construction (the same class S4 cured for `applyDirectoryEventPage` by declaring `Migrated`) |

`space` is the fourteenth row and is §2/§3's own lane: its Home retained routes are exactly the ones
this slice unblocked.

### 4.4 The sweep, re-run — measured, and honestly short of the bar

`🗑️generated/s6-sweep-s8a.txt` (tag `s8a`, serve `:6071`, six kinds, corrected verbs and two staged
arg maps): **0/6 PASS**, and the six rows are the evidence above. `norm` did not spawn at all in this
chunk (no row fields, 0 faults), so its §4.3 refusal is S7's measurement, not this one's.

**The ≥ 30/35 target is NOT reached and no full 35-kind re-sweep was run by this slice.** What is
delivered instead is the attribution the target needs: three of the thirteen already pass on the
ledger and are lost to the `#s-checkin` oracle (§4.1), eight fail because the sweep cannot fill a
staged form (§4.2) — of which at least six are PASS on their own single-plugin serve — and two carry
named refusals, one host-side and one hard-dead (§4.3). Fixing §4.1's oracle and §4.2's form filling
in `🐍️s6-all-kinds-sweep.mjs` is what moves the number, and neither is a framework defect.

## 5. Honest gaps

- **Home still lists 0 spaces.** Everything up to and including the ACK is now proven live; the last
  hop (the published config reaching the visible Home's render) is named in §3 with its two
  candidates, not guessed at.
- **The space index was never reached**, so the gis map document on hub 7611 was not listed. It is
  behind Home's row.
- **No 35-kind re-sweep.** Six kinds were re-run (§4.4). The other seven of the thirteen are
  classified from S7's captures, which is reading, not re-measuring.
- **§4.1's and §4.2's fixes are named but not landed** — both are in `🐍️s6-all-kinds-sweep.mjs`, the
  shared permanent probe, and changing its oracle mid-ticket would invalidate S6's and S7's numbers
  without a full re-run to replace them. The next owner should change the oracle and re-run all 35 in
  one pass.
- **`norm`'s host-side refusal is named, not fixed** (§4.3). It is a real `s`-host dispatch-routing
  defect and the cheapest remaining point on the sweep.
- **The `interactive-job.publication-stalled` law is not observed firing.** By construction it needs
  4 096 unchanged units of a stalled `Publishing` operation, and the only known instance of that class
  is the defect this slice removed. It is verified by `cargo check` and by reading, not at runtime.
- **The stall trace IS observed** — it is what named the stage and flags in §2.4.
- Hub `7611` was used read-only (sign-in as `user1@semio.dev`) and never restarted; no serve was
  killed. One `space` re-stage was run twice through the fleet wasm mutex.

## 6. Files changed

| file | change |
|---|---|
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | `HomeConfigPreparation::advance` and `::close_step` demand only `grant.permits_one()` instead of the whole 1 MiB envelope no pump grants; the close releases one owner per granted page and reports `released_bytes ≤ grant` (§2.5, §2.6.1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `TYPED_OPERATION_STALL_WITNESS_FLOOR`/`_FAULT_CEILING`/`_PUBLISHING_STAGE`; `typed_operation_stall_witness`, `typed_operation_stall_streak`, `fault_stalled_typed_operation_publication`; `advance_typed_operation_publication_one` is now the tracing wrapper around the renamed `…_unit` (§2.3, §2.6.2) |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `consumeTypedOperationEffects` lifts the receipt off its real `publish-event` carrier through `decodeWirePack`; new `takeTypedOperationTerminalOutputV1` reads the terminal output from either carrier and removes it from the invocation's, so the completion delivers it once and the admission keeps carrying the handle (§2.7) |
| `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` | the bootstrap oracle's runtime clause pins the stronger contract (both carriers, the `publish-event` receipt, the pack projection) — still 33 checks, clean |

Ticket folder (not product code): `🐍️s8-drain-hook-probe.mjs` (the in-page drain hook + guest
diagnostics arming), `🐍️s8-directory-body-probe.mjs` (what the hub's directory actually holds),
`📜️s8-restage.sh`; captures `🗑️generated/s8-drain-hook-{1..15}.txt`, `s8-directory-body.txt`,
`s8-journey-{1,2}.txt`, `s8-bootstrap-laws.txt`, `s8-{space-component,materialize,activate}.txt`,
`s6-sweep-s8a.txt`; screenshots `s8-drain-home.png`, `s8-drain-aborted.png`.

Temporary instrumentation added, measured with, and removed: the `__s8Drain` per-poll hook in
`drainTypedOperations`, the completion-tag log in `subscribeOperationCompletions` and two
`[S8PROBE]` lines in the directory bootstrap — `grep -c S8PROBE` and `grep -c s8Mark` are **0** in
all three files.
