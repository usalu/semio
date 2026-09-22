# HC1 — the fresh gis component's genesis, and creating a document on a hub

Slice HC1 of ticket 26/09/18, session 8 (2026-09-22, started 17:21). Inherits C8 §2.6: a catalog
published from today's tree loads on hub **7671** with `features.mcpWorkspace: true`, and every
`POST …/artifact-creations` on it dies in 32 s with
`genesis materialization failed: trusted artifact codec Input failed: epoch deadline exceeded`.

Brief: (0) reproduce that outside the hub in the owned interpreter / wasmtime A/B and root-fix it,
(1) the hub creation path's bounds (wall-clock deadline, recovery-sweep ownership, `credential set`
safety), (2) prove a gis AND a note document created live.

## 0. THE FINDING FOR TC3e — **the guest is not hanging; a wall clock is cutting it**

**TC3e's 3-package bootstrap does NOT need a different component.** C8 §2.6 read the fault as "the
guest never returns", on the evidence that the hub burned `0.0 %` CPU for the whole 32 s window.
That reading rests on the wrong process: C8's hold is a `bun` supervisor (`c8-hub-pid.txt` → pid
99743) that spawns the hub as a child, and the `bun` holder is the process that sits at 0.0 %.

HC1 re-ran C8's own `🐍️c8-create-diagnose.ts` against the SAME live hub 7671, sampling the process
that actually **listens on 7671** (`lsof -nP -iTCP:7671 -sTCP:LISTEN` → pid **99749**) every 2 s
(`📜️hc1-create-sampled.sh`, capture `🗑️generated/hc1-create-sampled-1.txt`, 17:24:13 → 17:24:45,
machine load 14.73):

```
HUB PID 99749  started 17:24:13  load 14.73 21.40 37.59
CPU 33.3 142.0 135.8 140.8 64.7 120.5 133.3 145.2 132.1 140.6 134.5 129.5 129.7 129.5 142.6 131.2
WALL 32s  ended 17:24:45
```

**130–145 % CPU for the entire 32 s**, then `phase: "failed"`. The guest is running flat out. What
kills it is a bound, and the deterministic 32.0 s C8 measured at loads 120, 33 and 21 is the
signature of a bound, not of a hang: `GUEST_CODEC_BUDGET`
(`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:240`) arms `deadline_ms: 30_000`, and
`OwnedRuntime::codec_call` spent it as `OwnedDeadline::TotalWall` — total wall clock, charged for
every millisecond the machine spends elsewhere.

That is the **identical defect** CE2/CE3 already fixed one call site over, for `describe`: the
`OwnedDeadline` docstring in `🔌️plugin/🖥️host/🦀️.rs` records `🀄️wfc` and `🧩️puzzle` dying
`DeadlineExceeded` "while still progressing normally", and `describe_observed` was moved to
`OwnedDeadline::NoFuelProgress` for it. `codec` was left on the old bound, with the comment
"`Poll`/`codec` therefore keep `TotalWall`".

**So: rebuild nothing on account of this.** The component C8 published (and the one TC3e's bootstrap
builds) is fine so far as this fault goes; the host bound was wrong. Fix landed in §2.

**For TC3e specifically.** Your catalog `76d111513c1bb4697487578009181f492a4873ad4f0cbffd8896dbe5fe5718d1`
on hub 7651 reproduces the fault exactly (`accepted → failed` in 32 s at machine load 5–7,
`🗑️generated/hc1-create-7651-1.txt`) — and the SAME catalog, held by a hub built from this
slice's tree on HC1's own port 7681, creates documents: `s.gis.gismap` **ready in 132 s**,
`s.note.note` **ready in 22 s** (§4). 7651 was never restarted and never written to; HC1 works
on an APFS clone of its data root. Your catalog is good — the hub binary holding it is what
needs rebuilding.

## 1. The reproduction outside the hub — the guest completes

A new permanent law in the plugin host's own suite,
`owned_codec_genesis_answers_the_biggest_staged_component_under_the_hub_s_own_budget`
(`🔬️owned-instance-open/🦀️.rs`), drives `codec.genesis` + `codec.pack-schema-hash` on the freshest
staged `semio_s_plugin_gis.wasm` under `HUB_GUEST_CODEC_BUDGET` — the hub's own `GUEST_CODEC_BUDGET`
copied verbatim (4 G fuel, 30 000 ms) — so the law runs the CALLER's budget instead of one the test
chose for itself.

The law is **green**: `owned_codec_genesis_answers_the_biggest_staged_component_under_the_hub_s_own_budget
... ok <1631.759s>`, `1 passed; 0 failed` (`🗑️generated/hc1-codec-laws-4.txt`, machine load 38).
Both calls ANSWER on the real release component; the run that timed them
(`🗑️generated/hc1-codec-laws-3.txt`, machine load ≈ 40, `--report-time`) reports:

```
codec.genesis(gis.map)          840.703220583 s
codec.pack-schema-hash(gis.map) 895.51642 s
finished in 1741.68s
```

Those are interpreter seconds on a loaded machine, not hub seconds — the same component's genesis
inside the hub took **132 s** (§4) — but they settle the question this slice was sent to answer:
**the gis guest returns**. It never returned under a 30 s total-wall bound because 30 s was never
enough, at any load.

Two traps this law fell into first, both now closed in the law itself:

- its first run picked the **`wasm-dev`** gis a peer rebuilt at 21:52 — 215 827 660 B against the
  release build's 47 969 633 B — because `plugin_wasm` takes the newest mtime across both profiles.
  A hub stages the RELEASE component, so the law now names its profile (`plugin_wasm_in_profiles`).
- it asserted the two calls finish inside 600 s, which is the very judgement this slice took out of
  the product. That assertion is gone; the law asserts that the calls ANSWER, and the durations ride
  in its failure messages.

The decisive run is the existing sweep `owned_codec_answers_every_call_on_every_staged_component`,
re-run with the fix in place (`🗑️generated/hc1-codec-laws-1.txt`, `--report-time`):

```
test …owned_codec_answers_every_call_on_every_staged_component ... FAILED <1453.074s>
3 staged components swept, 3 failed:
semio:note: codec.print-mirror(note.document) lost the minted identity
semio:gis: codec.print-mirror(gis.map) lost the minted identity
semio:stdio: codec.pack-schema-hash(stdio.txt): Guest(Fault { … "artifact codec schema has no
  structural record specification" })
```

Read it for what it is: `codec.genesis(gis.map)` and `codec.genesis(s.gis.gismap)` both **answered**
— the sweep runs pack-schema-hash → genesis → genesis-by-kind → print-mirror → apply-ops and only
reached `print-mirror` because everything before it returned a pair. **The gis guest does not hang.**
The three failures above are a different, pre-existing matter and belong to the codec-sweep owner
(TC3e); they are named in §5.

## 2. Root cause and fix — six copies of one wall clock

The `epoch deadline exceeded` was the FIRST of six independent absolute-deadline gates a single
creation has to pass. Each one charges an operation for the machine's other work, and the biggest
staged component needs ~130 s of honest guest interpretation, so every one of them had to go.

| # | file | what it was | what it is |
|---|---|---|---|
| 1 | `🔌️plugin/🖥️host/🦀️.rs` `OwnedRuntime::codec_call` | `OwnedDeadline::TotalWall` on `budget.deadline_ms` | `OwnedDeadline::NoFuelProgress` — the ceiling is the caller's `budget.fuel`; a guest that stops consuming fuel still dies |
| 2 | same, new `codec_genesis_observed` | `codec.genesis` had no progress observations | the same per-25 M-fuel / per-5 s observations `describe_observed` already had |
| 3 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` | `GuestArtifactCodecBinding::genesis` ran blind | takes the `OperationContext` and reports the guest's fuel progress into it as checkpoints |
| 4 | `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs` | `execute`/`recover_prepared` ran under `OperationContext::new(intent.deadline_ms, …)` | `OperationContext::stall_bounded(ARTIFACT_CREATION_STALL_BOUND_MS, …)` (HT16's shape) |
| 5 | `🌎️hub/🏗️bootstrap/🦀️.rs` | `ArtifactCreationHttpControlV1` reported `is_cancelled` 30 s after accept — an overrun read as the AUTHOR cancelling | cancellation and shutdown only; the bound lives in the context |
| 6 | `🌎️hub/🏗️bootstrap/🦀️.rs` | the 1 s recovery sweep closed any uncommitted key past its deadline | it asks `ArtifactCreationHttpTaskOwnerV1::owns_live_execution` first and skips keys a reservation or an unfinished task owns |
| 7 | `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs` | `ArtifactCreationPreparedV1::validate` refused a checkpoint `published_at_ms >= intent.deadline_ms`; `fold` refused a Prepared fact `recorded_at_ms >= deadline_ms` | both keep only their ORDER clauses; a late Prepared on a key the sweep closed is refused by the phase machine, which is where that decision belongs |
| 8 | `🪶️sqlite`, `🐘️postgres`, `🌐️neo4j` `append_artifact_creation_fact` | refused a `Prepared` append when `observed_now >= intent.deadline_ms` | refuses only a fact stamped in the future |
| 9 | the same three + `📇️directory/🦀️.rs` `validate_document_genesis_append_v1` | refused the genesis publication past the deadline | same |
| 10 | `…/service-v1/🦀️.rs` `publish_prepared` | `Err(AuthorityError::Publication(_))` returned `Indeterminate` and dropped the sentence | records it as a `server.artifact.creation` Failed fault; this is what named gate 9 at runtime |

`ARTIFACT_CREATION_DEADLINE_MS` keeps its value and changes its meaning, stated in its own
docstring: it is the bound on an **abandoned** key — the hub that accepted it died — and the sweep
asks it only of keys no live execution owns. A creation that is running is bounded by the new
`ARTIFACT_CREATION_STALL_BOUND_MS` and by `AuthorityLimits`.

Laws (both new, permanent):

- `creation_genesis_is_bounded_by_stalling_and_not_by_the_calendar`
  (`🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🔬️unit/🦀️.rs`) — on a scripted clock, a genesis
  that spends 4 × `ARTIFACT_CREATION_DEADLINE_MS` while checkpointing is materialized; one that
  reaches no checkpoint for the stall span is refused with `AuthorityError::Stalled`.
- `artifact_creation_recovery_never_closes_a_key_a_live_execution_owns`
  (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`) — ownership starts at the reservation, survives the running
  task, and is released when it finishes.

## 3. The hub suite — 332/332

`📜️hc1-hub-run.sh 1` (HC1's copy of `📜️ht16-hub-run.sh`, private `CARGO_TARGET_DIR=target-hc1-hub`,
`CARGO_INCREMENTAL=0`), machine load 25.5 at 21:37:

```
Summary [  31.524s] 332 tests run: 332 passed, 0 skipped
PASS [ 0.024s] (  6/332) semio-hub artifact_authority::creation::tests::creation_genesis_is_bounded_by_stalling_and_not_by_the_calendar
PASS [ 0.018s] (223/332) semio-hub::bin/os-hub tests::artifact_creation_recovery_never_closes_a_key_a_live_execution_owns
```

HT16's baseline was **330/330**; the two rows above are this slice's, so 332 is 330 + 2 with no
regression. `cargo check -p semio-hub --all-targets` exit 0 with **358 warnings**
(`🗑️generated/hc1-hub-check-1.txt`) — the warning count is the proof the tree really type-checked.
Captures: `🗑️generated/hc1-hub-nextest-{full,latest}-1.txt`, `hc1-hub-build-{1..5}.txt`.


## 4. LIVE: a gis document AND a note document created on a hub

Hub **7681**, HC1's own port, holding an APFS clone of TC3e's data root
(`.🧬semio/🌐hub/hc1-boot` ← `tc3e-hub`, so the catalog is TC3e's fresh three-package publication
`76d111513c1bb4697487578009181f492a4873ad4f0cbffd8896dbe5fe5718d1` from today's tree), running
`⚡️cache/cargo/target-hc1-hub/debug/os-hub` built from this slice's tree. 7651 was never restarted
and never written to; `features.mcpWorkspace: true`.

| kind | phases | wall | artifact id | capture |
|---|---|---|---|---|
| **`s.gis.gismap`** | accepted → **preparing** (132 s) → **ready** (134 s) | ≈ 256 s end to end incl. the 122 s the first probe spent | **`artifact-c3a3ac50f12865f707f4b2add56ec42c`** | `🗑️generated/hc1-creation-poll-gis5.txt`, `hc1-create-7681-gis5.txt` |
| **`s.note.note`** | accepted → **ready** | **22 s** | **`artifact-4445c8289f5a59d432ee74d598ba7327`** | `🗑️generated/hc1-create-note-1.txt` |

The hub's log carries **no** `server.artifact.creation` record for either — nothing failed.

The 22 s / 132 s split is the whole story of why this looked like a note-vs-gis difference: note's
genesis fits inside the old 30 s wall clock and gis's does not.

### 4.1 The same creation on the same catalog, before the fix

For the record, on TC3e's hub **7651** at machine load 5–7 (`🗑️generated/hc1-create-7651-1.txt`,
20:52:45): `accepted → failed` in **32 s**, hub pid 68878 at **94–218 % CPU** throughout. Same
catalog generation, same component, unfixed binary. That is the control for §0.

### 4.2 The four failures on the way, each one a distinct gate

| attempt | binary | terminal | the hub's own sentence |
|---|---|---|---|
| gis 1–2 | stall bound only | `failed` 180 s / 176 s | `conflict: artifact creation prepared pair differs from its accepted intent` (gate 7) |
| gis 3 | + gate 7 | `failed` 130 s, phase reached `preparing` | `conflict: artifact creation transition is outside its live server clock` (gate 8) |
| gis 4 | + gate 8 | `failed` 136 s | `genesis publication is indeterminate: conflict: genesis publication is outside its live server deadline` (gate 9, named only because of fix 10) |
| gis 5 | + gate 9 | **`ready`** | — |

### 4.3 Attach — the document opens in a browser, and the mount fails somewhere NEW

One chromium context on a `gis2d` serve bound to 7681 (port **6192**, `📜️c2-serve.sh`, already-staged
tree, no re-activation), driven by C4's own actor-reason probe against the gis document created above
(`🗑️generated/hc1-actor-probe-gis.txt`, `hc1g-actor-reason-user1.{json,png}` + console):

```
WHO user1 card=true attach=ok
BOOTSTRAP Restoring document: 81038 of 81038 bytes 81038/81038
STAGE 51884ms … actor-decode 147456/47916634 … 38191104/47916634
DIAGNOSTIC uiPatch.receipt: invalid bytes
ALERT The document component could not be verified. Reopen the document.
ACTIVE false (verifying=true rendererUnavailableSeen=false cleared=false canvas=1587x907)
```

Three things are new here and all three matter to the mount lane:

1. **A hub document created from a fresh catalog attaches in a browser** — `attach=ok`, the whole
   81 038-byte pair restored, the catalog's own 47 916 634-byte browser actor decoded.
2. **C7's defect 5 is gone.** C8 §3 measured `cold-pair.frontier` on the 2026-09-21 catalog; on
   TC3e's catalog that refusal does not appear at all.
3. **What refuses now is `uiPatch.receipt: invalid bytes`** — a different fault, after the cold pair,
   in the UI-patch receipt rather than the frontier. It is not genesis, not creation and not a hub
   bound, so it is not this slice's to fix; it is the next thing between here and a live edit.

**No edit round-trip is claimed**: `ACTIVE false` means there is no editor to author with.

## 5. `credential set` against a live data root — could NOT be reproduced

C8 §5.4 reported that `os-hub credential set` against the data root of a RUNNING hub left the hub
unreachable (`ConnectionRefused`, holder and child gone). Driven deliberately, twice, it does not:

| run | hub | root | result | capture |
|---|---|---|---|---|
| not-ready hub (catalog still opening) on 7682 | pid 26239 | a clone of `hc1-boot` | `credential set user3@semio.dev` exit **0**, user id `01a0caaf-84fa-7c93-961b-d04305e921df`, hub **alive** | — |
| **ready** hub on 7681 | pid 11917, `/readyz` **200** before | `hc1-boot`, live | exit **0**, user `01a0ca76-992f-7281-94d8-fb0865691519`, hub **alive and still 200** | `🗑️generated/hc1-credential-live-root.txt` |

And the credential it wrote works end to end: signing in as that user and creating a document on the
same live hub reached `ready` in **88 s** (`artifact-fe8d6ec7118235370bd5cef43bd91721`, same
capture). So no refusal was added: guarding a route that is observably safe would break the
documented provisioning order for a defect that is not there.

What IS true about the shape: `credential set` opens the hub's own `directory.db` as a second writer
(`SqliteDirectory::connect`, rollback journal, `busy_timeout` 2 s,
`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:648`), so under write contention a live hub's own transaction can
take `SQLITE_BUSY`. That is a latency hazard, not the process death C8 saw — and C8's own §5.3 found
a readiness waiter that kills the hub it waits for, which is a better fit for what they observed.

## 6. Honest gaps

- **The gis `codec.genesis` law costs 27 minutes of machine time** (green at `ok <1631.759s>` under load 38, `🗑️generated/hc1-codec-laws-4.txt`). It is a real oracle and it is far too expensive to sit in a default suite run; whoever owns the plugin-host suite should decide whether it belongs behind a profile. Its first two attempts are recorded in §1 because each failed for a reason worth keeping.
- **Three codec-sweep failures are open and are NOT this slice's**: `codec.print-mirror` "lost the minted identity" on note AND gis, and `codec.pack-schema-hash(stdio.txt)` faulting `artifact codec schema has no structural record specification` (§1). They are downstream of genesis, they reproduce on all three staged components, and the codec sweep belongs to TC3e. A hub only calls `print_mirror` for a package whose Rust codec it does not link, which is why creation succeeds despite them.
- **A 48 MB component's genesis costs ~130 s of owned interpretation** and nothing here makes it faster. The compiled `WasmtimeRuntime` codec path exists (M10 §3.3.1) and the hub does not use it; whether a JIT compile of 48 MB pays for itself per creation is unmeasured.
- **No browser MOUNTED either document**, and no edit round-trip was performed. The gis document attaches and restores (§4.3) and then refuses with `uiPatch.receipt: invalid bytes`; the note document was never opened in a browser at all.
- **The two `s.note.note` documents and the `s.gis.gismap` document live in `hc1-boot`**, a 190 MB APFS clone of TC3e's root. 7651 itself was never restarted, never republished into and never written to by this slice.
- **`trusted-catalog-load-stalled-before-it-finished`**: the scratch hub on 7682 never reached ready while a `cargo test` ran, because HT16's startup stall bound fired. The catalog load's checkpoints are coarse enough that a loaded machine trips a 30 s no-progress span — the same class of defect this slice fixed one layer down, still live at hub startup. Not fixed here; hub 7681 was started before that load and was unaffected.
- **Postgres/Neo4j lanes were not run.** `📜️hc1-hub-run.sh` runs the default nextest profile; DB4's serial `live-database-lanes` group needs the live containers, which were not started. My three backend edits are identical in shape across sqlite/postgres/neo4j and only sqlite was executed at runtime.
- **The recovery sweep's ownership predicate is proven by a unit law, not at runtime.** What the live runs show is the ordering that follows from it: in the gis 3/4 attempts the execution task's own fault is logged BEFORE the sweep's close, i.e. the sweep did not take the key from a live execution.

## 7. Files changed and captures

Product source:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` — `codec_call` stall-bounded + a progress parameter; `codec_genesis_observed`; the `OwnedDeadline` docstring carries the measurement
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` — `GuestArtifactCodecBinding::genesis` reports guest fuel progress into the caller's context
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs` — `ARTIFACT_CREATION_STALL_BOUND_MS`; `ARTIFACT_CREATION_DEADLINE_MS` re-documented; the two calendar clauses in `ArtifactCreationPreparedV1::validate` and `fold` removed
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs` — stall-bounded `execute` / `recover_prepared`; the indeterminate publication now names itself
- `🌎️hub/🏗️bootstrap/🦀️.rs` — the control loses its wall clock; stall-bounded accept and execution contexts; `owns_live_execution` and the sweep's ownership check
- `🌎️hub/📇️directory/🦀️.rs` — `validate_document_genesis_append_v1` keeps order, drops the deadline
- `🌎️hub/📇️directory/{🪶️sqlite,🐘️postgres,🌐️neo4j}/🦀️.rs` — the `Prepared` append and the genesis publication drop their deadline clauses

Laws:

- `🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🔬️unit/🦀️.rs` — `creation_genesis_is_bounded_by_stalling_and_not_by_the_calendar`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — `artifact_creation_recovery_never_closes_a_key_a_live_execution_owns`
- `🧰️framework/…/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs` — `owned_codec_genesis_answers_the_biggest_staged_component_under_the_hub_s_own_budget`, `HUB_GUEST_CODEC_BUDGET`, and `codec_budget()` given a finite fuel cap (a stall-bounded call with `u64::MAX` fuel has no ceiling at all)

Ticket folder, new and permanent:

- `📜️hc1-create-sampled.sh` — a creation driven against a live hub while the process that LISTENS on the port is CPU-sampled
- `📜️hc1-hub-run.sh` — HC1's copy of the hub build + suite runner, private target dir
- `🐍️hc1-creation-poll.ts` — polls one creation to its terminal phase however long it takes
- `🐍️hc1-create-kind.ts` — creates one document of a NAMED kind and follows it to terminal
- `📓️hc1-fresh-component-genesis-and-creation.md` — this report

Captures in `🗑️generated/`: `hc1-create-sampled-1.txt`, `hc1-create-7651-1.txt`,
`hc1-create-7681-gis{2,3,4,5}.txt`, `hc1-creation-poll-gis{2,3,4,5}.txt`, `hc1-create-note-1.txt`,
`hc1-credential-live-root.txt`, `hc1-hub-7681.txt`, `hc1-hub-pid.txt`, `hc1-hub-check-1.txt`,
`hc1-hub-build-{1..5}.txt`, `hc1-hub-nextest-{full,latest}-1.txt`, `hc1-codec-laws-{1,2,3,4}.txt`,
`hc1-actor-probe-gis.txt` + `hc1g-actor-reason-user1.{json,png,-console.txt}`, `hc1-serve-gis2d-pid.txt`.

### Infrastructure HC1 owns

| what | value |
|---|---|
| hub port | **7681** (holder pid in `🗑️generated/hc1-hub-pid.txt`, log `hc1-hub-7681.txt`) |
| data root | `.🧬semio/🌐hub/hc1-boot` — an APFS clone of `tc3e-hub`, never shared |
| binary | `⚡️cache/cargo/target-hc1-hub/debug/os-hub` (this slice's tree) |
| users | `user1@semio.dev` / `gm1-local-dev-pass-1`, `user2@semio.dev` / `gm1-local-dev-pass-2` |
| documents created | gis `artifact-c3a3ac50f12865f707f4b2add56ec42c`, note `artifact-4445c8289f5a59d432ee74d598ba7327`, note `artifact-fe8d6ec7118235370bd5cef43bd91721` |

