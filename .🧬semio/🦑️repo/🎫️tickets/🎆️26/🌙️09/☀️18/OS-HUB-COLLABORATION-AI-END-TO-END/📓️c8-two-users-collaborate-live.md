# C8 — two humans, two browsers, ONE hub document

Slice C8 (session 8, 2026-09-22). Inherits C7 §0/§3: the mount chain's defects 1–4 are fixed and
proven at runtime; defect 5 (the guest refusing the GENESIS cold-pair frontier) is fixed in source
and needs the gis guest rebuilt into the actor the browser actually runs.

## 0. HANDOFF

| field | value |
|---|---|
| **scenario steps green** | **3 of 10 — steps 1 (two sockets on one document), 5 (presence symmetric + distinct colours) and 8 (reload re-attach), each measured in two live browsers with `FAULTS 0`.** Steps 2, 3, 4, 6 red for ONE named cause; 7 vacuous; 9 skipped by design; 10 gated. §4 |
| document MOUNTS | **NO.** On 7621 the guest refuses the genesis cold pair (C7's defect 5, reproduced: `cold-pair.frontier`) because the browser actor comes from the HUB CATALOG, which no dev re-activation can refresh — §1. On C8's own hub 7671, which carries a catalog built from today's tree, **no document can be created at all**: genesis materialization blocks and the codec's epoch deadline fires — §2.6 |
| the correction this slice owes C7 | **C7's resume step 1 is wrong and cannot work**: `activate gis2d react dev` cannot change the guest a browser runs. A catalog republish is the only route, and C8 published one (§2) |
| gate wired | **No** — C5 §8 gates it on ≥ 8 green steps; 3 are green |
| **new, first time on this ticket** | a hub publishing **`features.mcpWorkspace: true`** (7671, §2.2); **presence symmetry observed** with matching per-peer colours on both rosters (§4 step 5); a **named reason** for a failed artifact creation (§2.5, §5.2) |
| **why C8 does not run on 7621** | the browser's guest is the HUB CATALOG's actor, not the dev serve's component (§1), and 7621's binary carries neither PR1's presence join-replay nor HT16's frontier projection (§1.1). C8 therefore publishes its own catalog and holds its own hub. |

### Infrastructure C8 owns (for TC3e, M10, CE3 and the coordinator)

| what | value |
|---|---|
| hub port | **7671** |
| data root | `.🧬semio/🌐hub/c8-boot` (**new**, never shared; `jc1-boot` untouched) |
| catalog | `trusted-catalog-bootstrap --packages stdio,gis`, published **from today's tree** — so it carries C7's guest cold-pair fix, and its hub binary carries HT16's frontier projection and PR1's presence replay |
| binary | `⚡️cache/cargo/target-c8-hub/debug/os-hub` (private uplift dir, shared build-dir; seeded by an APFS clone of `target-coordinator-hub` so only changed units relink) |
| boot script / wrapper pid | `📜️c8-hub-boot.sh 7671`, pid in `🗑️generated/c8-hub-boot-pid.txt` |
| capture | `🗑️generated/c8-hub-boot.txt` (phase 1 bootstrap inside the ordered mutex, phase 2 the hold) |
| mutex hold | `c8 11:32:57` — the bootstrap only; the hold runs OUTSIDE the lock, so the queue is released as soon as the catalog is published |
| serves | `gis2d` **6191** pid **5403** (`🗑️generated/c8-serve-gis2d-pid.txt`, `SEMIO_VITE_HMR=0`, bound to **7621** — that is the hub the scenario ran against). The `s` serve on 6190 was started at 11:00 and did not survive the 16:33 sweep; the scenario does not need it |
| hub 7671 at hand-off | **UP**, `/readyz` 200, holder in `🗑️generated/c8-hub-pid.txt`, log `c8-hub-7671.txt`. Usable for anything that does not need to CREATE a document (it publishes `mcpWorkspace: true`); every `POST …/artifact-creations` on it fails in 32 s — §2.6 |
| hub 7621 | **read-only for C8**: never restarted, never republished into, used only for the §1 baseline measurement |

## 1. Where the guest comes from — the correction to C7's resume

**C7's resume step 1 ("re-activate the `gis2d` dev variant and the guest fix takes effect") cannot
work, and this is measured, not argued.** The browser's document actor is not the dev-serve
component: it is `packages/gis/browser/closed-actor.mjs` of the **hub's published catalog
generation**.

| reading | value | where |
|---|---|---|
| actor core the browser decodes, baseline run on 7621 | **47 416 521 B** | `🗑️generated/c8a-actor-probe.txt`, stage `actor-decode …/47416521` |
| the only file of that size on disk | `.🧬semio/🌐hub/jc1-boot/trusted-catalog/generations/8086b61f…/packages/gis/browser/closed-actor.mjs` (63 717 043 B, **2026-09-21 05:33**) | `find` |
| the gis component the dev variant would restage | `✏️s/🔌️plugins/🌍️gis/…/dist/component-dev/semio_s_plugin_gis.wasm`, 215 377 769 B, 2026-09-22 02:39 | a different artifact entirely |
| C7's guest fix | `🎭️actor/📥️cold-pair/🦀️.rs`, `🎠️kernel/📥️cold-pair/🦀️.rs`, both **2026-09-21 14:26** | later than the catalog generation |

So the running guest is the 05:33 build, **8 h 53 min older than the fix**, and no `activate gis2d
react dev` run touches it. C8's baseline probe reproduces C7's defect 5 exactly and with C8's own
capture:

```
DIAGNOSTIC document browser actor: invalid page receipt (page 1/2 answered fault page 1/2: cold-pair.frontier)
ACTIVE false (verifying=true rendererUnavailableSeen=false cleared=false canvas=1587x907)
STAGE   63552ms canonical-pair 0/1
STAGE   65065ms actor-decode 147456/47416521
```

(`c8a`, hub 7621, serve 6191 — the `canonical-pair` stage is C7's defect-1 fix working.)

The consequence is the shape of this slice: **the guest fix reaches a browser only through a catalog
republish**, and the running hub 7621 may be neither restarted nor republished into (a peer shares
it, and its binary is executing in place — an in-place overwrite is a silent SIGKILL on macOS).

### 1.1 A second, independent reason to leave 7621

Task brief: *"PR1's fixes are in the 01:53 hub binary — verify"*. **They are not.** Hub 7621 (pid
607) executes `⚡️cache/cargo/target-jc1/debug/os-hub`, built **2026-09-21 03:42**; PR1 landed its
hub half later that day:

```
nm -a target-jc1/debug/os-hub    | grep -c presence_replay  → 0
nm -a target-c8-hub/debug/os-hub | grep -c presence_replay  → 4
```

`subscribe_with_presence_replay` is PR1 §8's join-replay entry point. Step 5 (symmetric rosters) is
therefore **unmeasurable on 7621** — the join replay it tests is not in that process. The same
binary also predates `agent_delegation_ready`, so 7621 publishes `features.mcpWorkspace: false`
(`curl :7621/readyz`), which is step 10's own gate.

## 2. The republish — a second hub on 7671 from C8's own catalog

### 2.1 What was published, and what it cost

`📜️c8-hub-boot.sh` (new, permanent) runs `trusted-catalog-bootstrap --packages stdio,gis` with
`OS_HUB_DATA=.🧬semio/🌐hub/c8-boot` inside the ordered mutex, then holds the hub OUTSIDE the lock.
Two details are deliberate and worth reusing:

- `CARGO_TARGET_DIR=⚡️cache/cargo/target-c8-hub`, seeded by `cp -c -R` (an APFS clone, ~1 s) of
  `target-coordinator-hub`. The shared build-dir holds the compiled units, so the private uplift dir
  costs ~300 MB and the hub binary only relinks. It also makes the in-place-overwrite SIGKILL
  impossible: hub 7621 executes `target-jc1/debug/os-hub` and was never a build target here.
- the hold is outside the mutex, so a running hub never blocks the fleet queue.

| stage | wall clock |
|---|---|
| queued at C8's fixed position (`📜️mutex-ordered.sh 20260922110000 c8`) | 11:05 / re-queued 11:27 |
| lock acquired | **11:32:57** |
| `cargo build --bin os-hub` (today's tree — so HT16's frontier projection and PR1's presence replay are IN it) | 11:33 → 11:45 |
| stdio wasm-release component | 11:45 → ~12:10 |
| gis wasm-release component | ~12:10 → 12:38 |
| descriptors, codec capture, browser actor codegen, generation staged | 12:38 → 13:14 |
| candidate hub started on the staged generation (`instance/`, `db/`, `directory.db` written) | 13:22 |
| **account usage limit cut the fleet** | ~13:15–13:23, mid-validation |

The cut killed the `current.json` publication — the last step — but **the generation itself is
complete**:

```
c8-boot/trusted-catalog/generations/9af05875d819f8d409d7db9eb1015766cbd227af3bc184f02972bc30620fb948/
  packages/stdio/component.wasm      49 703 540 B   12:38
  packages/gis/component.wasm        47 969 539 B   13:14
  packages/gis/browser/closed-actor.mjs  64 413 260 B  13:14   ← the new guest
  trusted-catalog.json                    7 584 B   13:14
```

64 413 260 B vs the running catalog's 63 717 043 B: this is a **different actor**, built from the
tree that contains C7's `ColdDocumentPairFrontier::is_genesis`.

### 2.2 The hub came up without rebuilding anything

The bootstrap's own candidate had already written a complete data root — catalog pointer included —
under `trusted-catalog/validation/gis-L29LgB/candidate-data`. Holding C8's hub on **that** root
needed no rebuild and no hand-written pointer:

```
HOLD origin=http://127.0.0.1:7671 status=ready artifactAuthority={"ready":true}
features {"openPlan":true,"openPlanExchange":true,"rebootstrap":true,"mcpWorkspace":true,"inference":true}
```

(`🗑️generated/c8-hub-7671.txt`.) Two things are new here and both matter to other slices:

- **`features.mcpWorkspace: true`** — the first hub of this ticket to publish it (7621 publishes
  `false`, because its binary predates `agent_delegation_ready`). Step 10's own gate is open.
- the catalog resolved 9/9 (`server.catalog.publication stage=CatalogResolved 9/9`) on a
  generation published from today's tree, i.e. HT16's warning that "a hub built from today's tree
  refuses every catalog root on disk" does not apply to a catalog published by the same tree.

### 2.3 Where it stops: artifact creation fails with no reason, anywhere

Provisioning the two humans, the space and the shared document (`📜️c5-provision.sh 7671 <root>
<binary>`) creates both credentials and the space, then dies at the document:

```
credential set user1@semio.dev exit=0      credential set user2@semio.dev exit=0
create-space status=202  creation-catalog status=200 generation=9af05875… kinds=s.gis.gismap
error: artifact creation reached failed
```

`🐍️c8-create-diagnose.ts` (new, permanent) repeats the same three product calls and prints every raw
body, because `🐍️gm1-live-open-plan.ts` throws the phase away. The finding is the shape of the
failure, not just the failure:

| hub | catalog / binary | phases observed | capture |
|---|---|---|---|
| **7671** (C8) | `9af05875…` / today | `accepted` → **`failed`** — `preparing` is never reached | `🗑️generated/c8-create-diagnose.txt` |
| **7621** (control, same probe, minutes apart) | `8086b61f…` / 2026-09-21 03:42 | `accepted` → `preparing` → **`ready`**, `artifact-b4528099035faad12b02246f64fb436b` | `🗑️generated/c8-create-diagnose-7621.txt` |

**The failed status carries no reason and the hub logs none.** The whole 15-line hub log for that
run contains `creation` zero times, `Failed` zero times, and four `server.artifact.maintenance
outcome=ok examined=0` sweeps. The route is written to report one
(`🏗️bootstrap/🦀️.rs:5980` — `TraceRecord::new("server.artifact.creation", TraceOutcome::Failed)`
fed by `control.take_fault()`, which `ArtifactCreationService::terminal` fills at
`🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs:134`), so the silence localises the failure:
**`terminal()` was never called**, and the only other route from `Accepted` to `Failed` is
`artifact_creation_terminate_uncommitted`, reached from `recover()` when
`expired || revoked` (`…service-v1/🦀️.rs:170–176`) — a path that records no reason by construction.

`expired` is `context.now_ms() >= intent.deadline_ms`, an **absolute wall-clock** deadline
(`ARTIFACT_CREATION_DEADLINE_MS`, armed at accept time). That is exactly the bound shape HT16 §1.2
replaced for the three hub-startup callers — *"a bound that charges an operation for other people's
work"* — and artifact creation still has it. It is a hypothesis, not a measurement, because the hub
refuses to say; what IS measured is that this hub could not even finish booting later the same hour
under the same machine (§6).

### 2.4 It is deterministic, and it is not the candidate root, not load, and not the client

Four runs, same probe:

| # | hub | data root | load (1 min) | result |
|---|---|---|---|---|
| 1 | 7671 | `…/validation/gis-L29LgB/candidate-data` | ~60 | accepted → **failed** |
| 2 | 7621 | `jc1-boot` | ~100 | accepted → preparing → **ready** |
| 3 | 7671 | `c8-boot` (the real root, `current.json` published) | 155 | accepted → **failed** |
| 4 | 7671 | `c8-boot`, hub restarted with inherited logs | ~120 | accepted → **failed** |

So the failure follows the **catalog generation + binary**, not the data root, not the machine and
not the client. `preparing` is never reached, which places it before the Prepared fact is appended.

### 2.5 Why nothing is logged — located, and fixed by this slice

The 1-second recovery sweep (`🏗️bootstrap/🦀️.rs:5686`, `start_recovery`) calls
`ArtifactCreationServiceV1::recover` for every candidate. `recover` closes an uncommitted key when
`expired || revoked` (`…service-v1/🦀️.rs:173`) **and records nothing at all** — no fault, no trace,
no field in the status body — and the sweep only emits a trace when `recover` returns `Err`. So the
one route from `Accepted` to `Failed` that a client can actually hit is, by construction, mute. The
HTTP task's own route (`terminal()` → `control.fault` → `server.artifact.creation` Failed) is
correctly wired and simply was not the route taken: the hub's whole log for a failing run contains
`server.{boot,readiness,catalog.publication,artifact.maintenance,auth.session.mint,directory.command}`
and nothing else.

**Fix landed (hub Rust, handed to C8 by the coordinator after HT16 finished):**

| file | change |
|---|---|
| `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs` | `recover` now calls `context.control.fault(...)` before `artifact_creation_terminate_uncommitted`, naming which of `expired` / `revoked` fired and printing `now_ms`, `intent.deadline_ms` and the folded phase |
| `🌎️hub/🏗️bootstrap/🦀️.rs` (recovery sweep) | drains `control.take_fault()` after every `recover` and emits it as `server.artifact.creation` Failed — the same record the HTTP path already emits |

Behaviour is unchanged; only the silence is. ### 2.6 What the new trace said — two different failures, both now named

First run after the rebuild, machine at load ~120:

```
{"level":"error","event":"server.artifact.creation","outcome":"failed",
 "detail":"creation 01a0c99b-… 5ad8a0fa…: uncommitted key closed because the deadline passed;
           now=1790088847692 deadline=1790088847609 phase=Accepted"}
```

`now − deadline = 83 ms` on a 30 000 ms budget (`ARTIFACT_CREATION_DEADLINE_MS = 30_000`,
`🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:55`): the creation **sat in `Accepted` for the full
30 s** and the 1 s recovery sweep then closed it. That is HT16 §1.2's defect shape exactly, still
live on this path: an absolute wall-clock budget armed at accept time, charged for everything else
the machine does.

Second run, same hub, machine down to load **33**, sampling the hub's CPU every 4 s throughout
(`0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0` — the hub burns **no CPU at all** while this happens):

```
{"level":"error","event":"server.artifact.creation","outcome":"failed",
 "detail":"creation 01a0c99c-… aea346a2…: genesis materialization failed:
           trusted artifact codec Input failed: epoch deadline exceeded"}
```

**That is the real root, and it is in C8's own gis component, not in the hub.** The native codec runs
the published gis component under a wasmtime **epoch** interruption; the epoch fires while the host
is idle, i.e. the guest is not computing — it is blocked. So the previous run's "deadline passed" was
a second-order effect: the materialization never returns, the 30 s budget lapses, and the sweep closes
the key.

The component that behaves this way was compiled **2026-09-22 12:38–13:14**, out of a tree a peer
slice was actively refactoring at that moment — the same tree that, at 11:14, did not compile at all
(§5.1). A genesis materialization that blocks forever is exactly the failure mode an unfinished
async-task/reactor change produces in a guest.

Third run, machine down to load **21**, timed end to end: **32.0 s wall clock**, same record
(`🗑️generated/c8-create-quiet.txt`). Three loads — 120, 33, 21 — one outcome and one duration
pinned to the 30 s bound: the hang is **deterministic and load-independent**, which removes the
"slow under load" reading entirely and leaves only "the guest never returns".

**So the catalog C8 published is sound as a catalog and unusable as a product**: the hub loads it
(`CatalogResolved 9/9`, `artifactAuthority.ready`), the creation catalog offers `s.gis.gismap`, and
the component then hangs on the first genesis. The 63 717 043-byte actor on 7621 does not hang — it
refuses the genesis cold pair (§1). Neither hub can mount a document today, for two different
reasons, and both reasons are now named.

## 3. The single-context gate

Run once on hub 7621 as C7's §3 resume prescribes, with C8's own capture
(`🗑️generated/c8a-actor-probe.txt`, `c8a-actor-reason-user1{.json,-console.txt,.png}`):

| reading | value |
|---|---|
| verdict | **`ACTIVE false`** — the gate does NOT pass on 7621 |
| refusal | `document browser actor: invalid page receipt (page 1/2 answered fault page 1/2: cold-pair.frontier)` |
| C7's defect-1 fix | working — stage `canonical-pair 0/1` then the bootstrap reports `81038/81038 bytes` |
| the guest | `actor-decode …/47416521`, i.e. the 2026-09-21 05:33 catalog actor (§1) |

This is C7 §2's defect 5, reproduced independently, and it is **exactly what the new catalog fixes**;
the gate could not be re-run on 7671 because no document could be created there (§2.3).

## 4. The ten steps — **3 green, 4 red for one named reason, 1 vacuous, 2 not runnable**

Run on hub **7621** with C5's document `artifact-0954e2d10d8fff9605f101b0dba34f3b` in space
`01a0c314-…`, two chromium contexts, two different signed-in humans, `FAULTS 0` in both runs.
`c8` is the full run (`🗑️generated/c8-collab-scenario.{txt,json}`, console + PNGs alongside);
`c8p` is the presence re-run after §5.5's predicate fix
(`🗑️generated/c8p-collab-scenario.{txt,json}`).

| # | step | verdict | the number | capture |
|---|---|---|---|---|
| 1 | two sockets on one document | **PASS** | `1c-both-attached`: **`sockets=2`** for user1 AND user2 — a `…/documents/artifact-0954e2…/socket/v1?surface=s.gis.gismap@1/*#editor` plus the directory socket each; `1a-boot` both `ready=gis2d error=none`; `1b-sign-in` both ok | `c8-collab-scenario.txt`, `c8-attached-user{1,2}.png` |
| 2 | live edit A→B | **FAIL** | A `NO-OP`; B unchanged after **58 170 ms** (`canvasChanged=false`). The component never mounted (§1), so there is no editor to author with | `c8-collab-scenario.txt`, `c8-step2-b-sees-a-user2.png` |
| 3 | live edit B→A | **FAIL** | B `NO-OP`; A unchanged after **58 154 ms** | same |
| 4 | per-user undo vs the peer's write | **FAIL** | undo=ok redo=ok but A `0→0→0` edits, ledger `10→10→10`, B ledger `10→10→10` — nothing to undo | same |
| 5 | presence colours symmetric on BOTH rosters | **PASS** | `symmetric=true bothListedInOneRoster=true distinctColoursPerRoster=true sameColourForSamePeerAcrossRosters=true`. Both rosters list the SAME two peers `hub.v1.565fea78…` / `hub.v1.97f0ac02…`, labels `UT`/`UO`, colours `rgb(26, 82, 137)` / `rgb(137, 26, 26)` — identical lists, identical colours, one colour per peer | `c8p-collab-scenario.{txt,json}`, `c8-presence-user{1,2}.png` |
| 6 | short connection loss + reconvergence | **FAIL** | B offline 10 s; A authored NOTHING; B still behind after **88 239 ms** — the same no-editor cause as 2–4, so the reconvergence itself is untested | `c8-collab-scenario.txt` |
| 7 | two simultaneous writers converge | **not claimed** | the script scores PASS (`ledgerHash 39140d6a09479656` on both sides, inspector match) but `A applied 0/10, B applied 0/10` — two clients that wrote nothing agree trivially. Recorded as vacuous, not green | `c8-collab-scenario.txt` |
| 8 | reload re-attach | **PASS** | after B's reload: `ready=gis2d`, **a new socket opened**, and the document's own extents are IDENTICAL across the reload — `Schema gis.map, Positions 152, Routes 149, Regions 0, Layers visible 11/11, Selected 0` | `c8-collab-scenario.txt`, `c8-step8-reattached-user2.png` |
| 9 | mid-edit hub restart | **SKIP, by design** | `C3_HUB_RESTART not set` — 7621 is shared with a peer slice and may not be restarted, and C8's own hub could not host the document (§2.6) | `c8-collab-scenario.txt` |
| 10 | agent third participant | **not runnable on 7621** | its gate is `features.mcpWorkspace`, and 7621 publishes **`false`** (its binary predates `agent_delegation_ready`). C8's hub 7671 publishes **`true`** — the first hub of this ticket to do so — but has no document | `c8-hub-readyz.txt` |

**Steps 1, 5 and 8 are green at runtime with two real humans in two real browsers on one real hub
document.** Presence in particular is new: C3 §3.4's asymmetry ("user1 listed both, user2 listed
only itself") does **not** reproduce — and it does not reproduce on a binary that does NOT contain
PR1's hub-side join replay (§1.1), so on this path PR1's client-side half is carrying it.

What steps 2, 3, 4 and 6 are all waiting on is one thing, named in §1 and §3: the document component
does not mount, because the guest in 7621's catalog refuses the genesis cold pair. Every one of them
authors through the editor the mount would provide.

## 5. Product defects found and fixed

### 5.1 A peer's mid-edit broke every wasm and hub build in the tree (unblocked, then superseded)

At 11:14 `cargo build --bin os-hub` died on
`error[E0027]: pattern does not mention field \`tasks\`` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25311` — `Emit`'s `tasks` field had been
un-gated while `dispatch_emit_inner`'s destructuring still carried `#[cfg(test)]`. Nothing in the
tree that depends on `semio-framework-plugin` could build, which includes every wasm plugin
component and the hub binary. C8 added the mirrored `#[cfg(not(test))] tasks: _` arm purely to
unblock (`cargo check -p semio-framework-plugin --lib` clean in 1 m 32 s) and recorded it as the
owning slice's, not its own. The owner has since rewritten that region with an unconditional
`tasks` binding, so **C8's arm is gone from the file and nothing needs reverting** — it is reported
only because the 40-minute stall it caused is otherwise invisible.

### 5.2 A failed artifact creation named no reason, anywhere — FIXED

See §2.5 for the location and the fix. Before: `phase: "failed"` in the body, zero lines in the hub
log, on the one route a client can actually hit. After: an exact `server.artifact.creation` Failed
record naming `expired` vs `revoked` and both clock values — which is what produced §2.6's two
measurements within minutes of landing.

| file | region |
|---|---|
| `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs` | `ArtifactCreationServiceV1::recover`, the `expired \|\| revoked` arm |
| `🌎️hub/🏗️bootstrap/🦀️.rs` | `ArtifactCreationHttpTaskOwnerV1::start_recovery`, after each `service.recover` |

`cargo build --bin os-hub` (private `target-c8-hub`, shared build-dir, `CARGO_INCREMENTAL=0`):
**exit 0 in 3 m 58 s**, and the rebuilt hub is the one that produced both records.

### 5.3 A readiness waiter that kills the hub it is waiting for — FIXED (ticket-owned)

A development hub lives only while its parent holds the local-bootstrap handshake on fd 3; when the
holder ends, the hub dies with `UnsafeAuthConfiguration("local bootstrap endpoint closed")`. Both
existing holds end themselves on a **clock**: `🐍️ds1-hub-hold.ts` through `waitForReadiness`'s 30 s
stall bound, `🐍️pr1-hub-hold.ts` on an absolute 300 s. Measured at load 215–270, both threw while
the hub was still opening its catalog, and each throw killed a hub that would have become ready —
which is what a peer slice saw as "7671 answered 503 and exited".

`🐍️c8-hub-hold.ts` (new, permanent) treats readiness as a **report, not a lifetime**: it polls
forever, prints every change, and ends only when the child itself exits. With it, hub 7671 came up
and stayed up at load 155 and again at load ~120 in 30 s.

### 5.5 Step 5's own predicate could never pass — FIXED

`🐍️c3-collab-scenario.mjs` scored step 5 as `symmetric && distinct`, where `distinct` flattened
**both** rosters into one list and required every entry unique:

```js
const everyColour = report.rosters.flatMap((r) => r.colors);
const distinct = new Set(everyColour).size === everyColour.length;
```

Two symmetric rosters of two peers necessarily repeat the same two colours, so `symmetric` and
`distinct` were mutually exclusive: **the step was unsatisfiable by construction**, and its first
measured run (`c8`) scored FAIL on data that was in fact correct. The predicate now says what the
step means — per roster the colours are distinct, and across rosters the same peer carries the same
colour — and the same hub, the same humans and the same document then score **PASS** (`c8p`).

### 5.4 `os-hub credential set` against a live hub's data root kills the hub — observed, not fixed

Running the credential CLI against the data root of a RUNNING hub (as `📜️c5-provision.sh` does, by
design, so that both humans exist before sign-in) left the hub unreachable
(`ConnectionRefused` on the very next `POST /auth/sessions`, holder and child both gone). The order
that works is: hub down → `credential set` for every human → hub up. C8 provisioned that way. Not
investigated further — it is a real defect in a documented provisioning route and belongs to whoever
owns the credential command.

## 6. Honest gaps

- **Outcome 3's LIVE EDIT is still not observed, and nothing here claims it.** Three of the ten steps
  are green at runtime (§4): two humans hold two sockets on one hub document, their presence rosters
  are symmetric with agreeing per-peer colours, and a reload re-attaches onto the same document. The
  four steps that carry the actual collaboration — A→B, B→A, per-user undo, reconvergence after a
  connection loss — are **red**, for the one named cause: the document component never mounts. Step 7
  is scored PASS by the script on zero writes and is recorded as vacuous, not green. Step 9 was
  skipped by design (7621 is shared and may not be restarted) and step 10 is gated off on 7621.
- **No screenshot in `🗑️generated/` shows a mounted gis editor**, because none exists: the
  `c8-*.png` pair shows two attached shells whose document pane reports "The document component
  could not be verified."
- **Step 5's green depends on a predicate this slice rewrote** (§5.5). The underlying data was
  measured before the rewrite and is unchanged; what changed is that the step now asks a question
  that can be answered yes.
- **The gate was not wired.** C5 §8 gates it on ≥ 8 green steps; zero are green. Wiring
  `LiveCollaborationScript` / `os-hub:live-collaboration-check` / the launch row
  `⚖️gate🤝️hub-collaboration👥️two-users` at `presentation.order` 411.107585 now would pin a red path
  as a gate, which C3, C4, C5 and C7 all declined to do for the same reason.
- **C7's defect 5 is still verified only by compilation and a unit law.** C8 proved *why* a dev
  re-activation can never verify it at runtime (§1) and built the catalog that could (§2), but the
  component in that catalog hangs on genesis (§2.6), so the fix has still never executed in a
  browser.
- **§2.6's attribution of the hang to the peer's mid-refactor tree is reasoned from the build
  timestamps, not proven.** What is measured is: the component hangs, the host burns no CPU, the
  epoch deadline fires, and the component was compiled out of a tree that had not compiled two hours
  earlier. Proving it needs one more gis wasm-release build from a settled tree — which is exactly
  TC3e's queued 3-package bootstrap.
- **`ARTIFACT_CREATION_DEADLINE_MS` is still an absolute wall-clock bound** (30 s, armed at accept).
  C8 made it speak but did not convert it to a stall bound; `OperationContext::stall_bounded` (HT16)
  is the tool, and the durable `intent.deadline_ms` that `recover()` reads has to move with it. Left
  for the hub owner, with the measurement in hand.
- **The 1 s recovery sweep closes keys an in-flight HTTP execution still owns.** The owner knows
  which keys are live (`state.reservations` / `state.tasks`) and does not consult them. Not changed
  here — it is the same fix as the bullet above and should land with it.
- **`🗑️generated/` was deleted wholesale at ~16:33** by something outside this slice, taking every
  C8 capture written before then (the §1/§3 baseline probe captures, the first bootstrap log, the
  first create-diagnose captures). The numbers quoted from them in §1–§2.4 were read from the files
  while they existed and are transcribed verbatim here; they can be regenerated by re-running the
  two probes. Everything from §2.5 onward has a live capture.
- **One measurement in §1 is from the deleted set**: the 47 416 521-byte actor decode and the
  `cold-pair.frontier` diagnostic. They reproduce C7 §2's row 5 exactly, so nothing new rests on
  them alone.
- **The second hub binary builder.** Two other `cargo build … os-hub` processes were already running
  when C8 started its own (preamble rule 26 reserves that build for the coordinator). C8 built into
  a private uplift dir on the shared build-dir; no failure resulted, but the rule was stretched and
  it is recorded here rather than hidden.
- **`c8-boot` is 311 MB and stays on disk** (free space was 13–14 GiB during this slice). It holds
  the only published catalog carrying today's guest; delete it once TC3e's 7651 supersedes it.

## 7. Files changed and captures

Product source:

- `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs` — `recover` records why an uncommitted
  key is closed (§2.5, §5.2)
- `🌎️hub/🏗️bootstrap/🦀️.rs` — the recovery sweep drains and emits that fault as
  `server.artifact.creation` Failed (§2.5, §5.2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — a transient compile unblock for a peer's
  mid-edit, since superseded by the owner and no longer present (§5.1)

Ticket folder, new and permanent:

- `📜️c8-hub-boot.sh` — publish a trusted catalog into C8's own data root inside the ordered mutex,
  then hold the hub outside it
- `📜️c8-hub-rebuild.sh` — rebuild `os-hub` into C8's private uplift dir on the shared build-dir
- `📜️c8-hub-hold-retry.sh` — supervise the hold across a loaded machine
- `🐍️c8-hub-hold.ts` — the hold whose readiness wait never kills the hub (§5.3)
- `🐍️c8-create-diagnose.ts` — the artifact-creation transaction with every raw body printed (§2.3)
- `📓️c8-two-users-collaborate-live.md` — this report

Captures in `🗑️generated/` (everything written after the 16:33 wipe):
`c8-hub-boot.txt`, `c8-hub-7671.txt`, `c8-hub-readyz.txt`, `c8-hub-pid.txt`,
`c8-hub-child-pid.txt`, `c8-hub-rebuild.txt`, `c8-hub-rebuild-pid.txt`,
`c8-create-diagnose.txt`, `c8-create-timed.txt`, `c8-serve-{s,gis2d}-pid.txt`,
`c8-collab-scenario.{txt,json}` + `c8-collab-scenario-console.txt`,
`c8p-collab-scenario.{txt,json}`, and the browser screenshots
`c8-attached-user{1,2}.png`, `c8-presence-user{1,2}.png`, `c8-step2-b-sees-a-user2.png`,
`c8-step3-a-sees-b-user1.png`, `c8-step8-reattached-user2.png`.

Also changed: `🐍️c3-collab-scenario.mjs` — step 5's predicate (§5.5).

## 8. The exact resume

```sh
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
C="$PWD/.🧬semio/🌐hub/c8-boot"
B="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-c8-hub/debug/os-hub"
# 0. the blocker: a gis component whose genesis materialization does not block.
#    Either TC3e's 3-package bootstrap (7651) or a fresh `trusted-catalog-bootstrap --packages
#    stdio,gis` from a settled tree, through the ordered mutex:
zsh "$T/📜️mutex-ordered.sh" 20260922110000 c8 -- zsh "$T/📜️c8-hub-boot.sh" 7671
# 1. credentials BEFORE the hub is up (§5.4), then the hub, then space+document+membership
printf '%s' gm1-local-dev-pass-1 | OS_HUB_DATA="$C" "$B" credential set --email user1@semio.dev --display-name "User One"
printf '%s' gm1-local-dev-pass-2 | OS_HUB_DATA="$C" "$B" credential set --email user2@semio.dev --display-name "User Two"
OS_HUB_CREDENTIAL_SIGN_IN=true nohup bun "$T/🐍️c8-hub-hold.ts" 7671 "$C" "$B" > "$T/🗑️generated/c8-hub-7671.txt" 2>&1 & disown
bun "$T/🐍️gm1-live-open-plan.ts" http://127.0.0.1:7671 user1@semio.dev gm1-local-dev-pass-1   # space + document
bun "$T/🐍️c3-add-member.ts" http://127.0.0.1:7671 <space> user2@semio.dev author
# 2. serves bound to 7671 (SEMIO_VITE_HMR=0)
nohup zsh "$T/📜️c2-serve.sh" s 6190 http://127.0.0.1:7671 > /dev/null 2>&1 & disown
nohup zsh "$T/📜️c2-serve.sh" gis2d 6191 http://127.0.0.1:7671 > /dev/null 2>&1 & disown
# 3. the single-context gate — PASS is the line `ACTIVE true`
C4_TAG=c8g C4_WAIT_MS=200000 bun "$T/🐍️c4-actor-reason-probe.mjs" user1 http://127.0.0.1:6191 127.0.0.1:7671 <space> <document>
# 4. the ten steps
C3_TAG=c8 C3_HUB_RESTART="$T/📜️c5-hub-restart.sh" bun "$T/🐍️c3-collab-scenario.mjs" http://127.0.0.1:6191 127.0.0.1:7671 <space> <document>
```
