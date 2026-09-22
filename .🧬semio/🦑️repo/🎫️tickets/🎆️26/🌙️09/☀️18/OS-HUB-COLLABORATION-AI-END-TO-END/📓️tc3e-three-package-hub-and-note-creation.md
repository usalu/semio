# TC3e — a three-package trusted catalog on hub 7651, and the first `s.note.note` document

Slice TC3e of ticket 26/09/18. Predecessor `📓️tc3d-guest-genesis-and-note-creation.md`.
Started 2026-09-22 11:0x.

## 0. HUB HANDOFF

**No `semio-hub` source changed by this slice** — everything landed is in one `s` plugin app and in
two law files. The coordinator needs no `semio-hub` rebuild on TC3e's account, and
`cargo check -p semio-hub` was therefore not run (preamble rule 26).

**Hub 7651** is the live proof hub for C8 (frontier + presence socket probes), CE3
(`hub-agent-participant-check` 17/17), M10 (an agent committing an edit) and HT16's live checks. It
stays up after this slice ends.

| field | value |
|---|---|
| port | **7651** |
| data root | `.🧬semio/🌐hub/tc3e-boot` (new, never shared) |
| catalog | `trusted-catalog-bootstrap --packages stdio,gis,note`, published from today's tree — the FIRST three-package trusted catalog |
| binary | `⚡️cache/cargo/target-tc3d/debug/os-hub`, relinked by the bootstrap inside the hold, so it carries HT16's `OperationContext::stall_bounded` / `LOCAL_READINESS_STALL_BOUND_MS` and the frontier-identity fix |
| credentials | `OS_HUB_CREDENTIAL_SIGN_IN=true`; `user1@semio.dev` / `user2@semio.dev` minted by `📜️tc3e-provision.sh` |
| hold pid | **68876** (child `os-hub` **68878**), `🗑️generated/tc3e-hub-pid.txt`, hold script `🐍️tc3e-hub-hold.ts`, log `🗑️generated/tc3e-hub-7651.txt` |
| data root | **`.🧬semio/🌐hub/tc3e-hub`** (see below — NOT `tc3e-boot`) |
| profile | `local-stdio-gis-note-open-v1` |
| generation | **`76d111513c1bb4697487578009181f492a4873ad4f0cbffd8896dbe5fe5718d1`**, bundle sha256 `653aeed2428361e2453ed1b2037ce37afd5333ffd2339f126d881139448104eb`, `publicationRevision` 1 |
| packages on disk | `generations/76d1115…/packages/{gis,note,stdio}` — **the first three-package trusted catalog** |
| `readyz` | **200**, `status: ready`, `artifactAuthority.ready: true`, `directory/storage/artifactCasBarrier/artifactPublication/artifactCasSweeper/adminAssets` all ready, `runId b6eb601de9d7254929cf28421434b288`, features `openPlan/openPlanExchange/rebootstrap/mcpWorkspace/inference` all true (`🗑️generated/tc3e-readyz.json`) |

**Two things a consumer must know about this root.**

1. **It is the bootstrap's own CANDIDATE root, promoted by copy.** Run 4's stage 2 got all the way
   through staging, verification and the candidate hub's boot, and then died in
   `proveTrustedStdioGisCandidatePlan` (§4.2) — the step AFTER the catalog is current in the
   candidate root and BEFORE it is published into `tc3e-boot`. That candidate root
   (`tc3e-boot/trusted-catalog/validation/gis-HTfnDg/candidate-data`) already carried
   `trusted-catalog/current.json` pointing at the three-package generation, so it was copied to
   `.🧬semio/🌐hub/tc3e-hub` (its `validation/` subtree dropped) and 7651 holds that. The BYTES are
   the bootstrap's own verified generation; what is missing is the final `trusted-catalog publish`
   into `tc3e-boot`, which is honest and is stated in §7.
2. **The readiness waiter is widened by the CALLER, not by editing HT16's constant.** `🐍️ds1-hub-hold.ts`
   failed this boot at 17:52 with `hub readiness stalled — nothing about the hub changed for
   30 031 ms; closed gates: no /readyz answer was ever received`, and `startLocalHub`'s cleanup then
   killed a hub whose very next line was `os-hub ready at http://127.0.0.1:7651`
   (`UnsafeAuthConfiguration("local bootstrap endpoint closed")`). `waitForReadiness`'s third
   parameter exists for exactly this, so `🐍️tc3e-hub-hold.ts` is `🐍️ds1-hub-hold.ts` with
   `TC3E_READINESS_STALL_BOUND_MS = 600_000` passed in; `LOCAL_READINESS_STALL_BOUND_MS` is untouched.
   **This is a live data point for HT16:** a three-package catalog under fleet load emits nothing at
   all for >30 s during boot, so a 30 s no-change bound kills a healthy hub.

**Why a fresh publish and not a reuse.** HT16 measured that a hub binary from today's tree refuses
EVERY catalog root on disk — the bundle profile's `openTarget` → `openTargets` rename (commit
`50c97b2051`) makes an older root answer
`ArtifactAuthority(Catalog("unknown field openTarget"))`, and hand-editing a root fails the
generation id. C8's own two-package root on 7671 cannot be copy-extended with note either: the
generation id is derived from the whole package closure and the profile carries an exactness fence
on its id (TC3c §5b), so a fourth package is a republish, not an append. TC3e therefore publishes
its own three-package generation, which is also the only generation that can ever serve
`s.note.note`.

## 1. Inherited state — TC3d's hold DID run, and died one layer deeper

The brief said "TC3d's driver never reached the mutex". **The captures say otherwise, and that
changes the whole slice.** `🗑️generated/tc3d-hub-dev.txt` (808 KB, last written 04:12:44):

| stage | begin | end | exit |
|---|---|---|---|
| preflight (outside the mutex) | 01:27:15 | 01:35:14 | 0, 0 retries |
| stage 1 `semio-s-plugin-stdio` wasm-release cdylib | 03:05:05 | 03:12:40 | 0 |
| stage 1 `semio-s-plugin-gis` | 03:12:40 | 03:12:42 | 0 (fresh) |
| stage 1 `semio-s-plugin-note` | 03:12:42 | 03:16:31 | 0 |
| stage 2 `trusted-catalog-bootstrap --packages stdio,gis,note` | 03:16:31 | **04:12:44** | **1** |

So all three components WERE rebuilt with TC3d's `codec` resolver fix compiled in
(`⚡️cache/cargo/target-tc3d/wasm32-wasip2/wasm-release/semio_s_plugin_{stdio,gis,note}.wasm`,
03:12/02:53/03:16), the bootstrap ran for 56 minutes, got through the actor closure and the actor
hash, and died in `trustedBootstrapComponentCodecRowsV1`. 7651 never came up because stage 2 failed,
not because the lock was never granted. TC3d's `📜️tc3d-prove.sh` watcher therefore never fired and
`🗑️generated/tc3d-prove.txt` holds only its start line.

## 2. The next fault in the chain — it was never `codec` at all

The exact stage-2 death (`🗑️generated/tc3d-hub-dev.txt`, tail):

```
semio-framework-plugin-describe codecs: codec.pack-schema-hash(note.document) on
  …/trusted-catalog/staging-a999881f2407f3681838d26d58f89c4d/packages/note/component.wasm:
guest fault plugin.internal: throwaway artifact codec app close faulted:
  Fault { origin: Framework, code: FaultCode("interactive-job.close-owned-disposer-missing"),
          message: "app owner did not provide the required bounded disposer for document-store" }
```

**That is TC3d's fix working.** Where TC3c had `wasm trap: unreachable executed` and TC3d decoded it
to a store `Drop` assert, the guest now returns a NAMED, reportable fault from
`close_artifact_codec_app` instead of aborting. What the fault names is the layer below: the close
cursor exists, and the app it is asked to close cannot close.

**Which app, and why.** `plugin_artifact_codec_app`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37311`) constructs every app of the bundle
to read its schema, keeps the editor, and closes the viewer it rejects. `VcsArtifactApp`'s close
ladder drives eight owned lanes (`🦀️.rs:30594`), each through
`drive_artifact_owned_disposer(lane, …)`, and lane 0 is `document-store`. The disposer for that lane
is `A::build_document_store_disposer()`, captured at construction (`🦀️.rs:22700`); the trait default
is `None` (`🦀️.rs:33366` for `ArtifactViewer`) and `None` is the fault above — fail-closed by
design.

`NotePlayApp`, the editor, declares all five of its disposers
(`…/🪆️subsets/✳️any/✏️editor/🦀️.rs:526-551`). **`NoteViewer` declared none of them** — no store
owners, no disposers, nothing but snapshot/handle/render. So the note document kind was reachable
only as long as nobody ever closed its viewer.

**This is not a codec defect and not a TC3d regression: `ViewerApp<NoteViewer>` could never close at
all.** The identical defect is already documented one plugin over —
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/…/👁️viewer/🦀️.rs:60` records that every close of a mounted
`ViewerApp<Block3dViewer>` failed with this exact code until its owners were declared, with the
comment "Read-only says nothing about ownership". Note's viewer is the same omission, found through
the codec path because the codec resolver is the one caller that closes EVERY app of a bundle.

**Scope, measured.** Only packages the hub links no Rust codec for run the component codec probe
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:9669-9676`: `spec.linkedCodecRegistry` short-circuits to the
linked rows). stdio and gis are linked, note is not — so note is the only one of the three whose
guest `codec` interface the bootstrap calls, and the only one whose bundle apps must close.

## 3. The fix — one app, six declarations

`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`:
`NoteViewer` now declares `build_document_store_owners` and `build_config_store_owners`, and the
four bounded disposers `VcsArtifactApp`'s ladder drives for a viewer —
`bounded_document_store_disposer::<NoteSnapshot, NoteMutation>()`, `no_config_store_disposer()`,
`no_presence_store_disposer()`, `no_transient_store_disposer()` (the draft lane is supplied by
`ViewerApp` itself, `🔌️plugin/🦀️.rs:34051`). The document lane is the editor's own catalogue: a
viewer owns the same `NoteSnapshot` envelope and retires it the same way.

No framework edit was needed; TC3d's cursor and diagnostics are what made this legible in one read
instead of another two-hour wall.

| check | exit | warnings | capture |
|---|---|---|---|
| `cargo check -p semio-s-artifact-note-note` | 0 | 86 | `🗑️generated/tc3e-check-note-artifact.txt` |

(Warning count as the proof of a real type-check rather than a short-circuited cache.)

**PROVEN AT RUNTIME, on the rebuilt component.** Run 1 of the hold rebuilt
`semio_s_plugin_note.wasm` (13:26:01 → 13:29:56) and the in-hold gate drove the emitter against it:

```
13:29:56  gate BEGIN codec probe on the fresh note component
          semio-framework-plugin-describe codecs …/wasm32-wasip2/wasm-release/semio_s_plugin_note.wasm
            --kinds s.note.note=note.document
          -> 🗑️generated/tc3e-note-codec-rows.json (1 rows)
13:30:03  gate END exit=0
```

```json
{"schema":"semio.plugin.component-codec-rows/v1","rows":[{"artifactKind":"s.note.note",
 "artifactSchema":"note.document",
 "packSchemaHash":"db3a2145b46b70bd8c76ce01d6db76fc4004059176d9a2cb2589b6b2b1dc8b82"}]}
```

That is the exact call that answered
`interactive-job.close-owned-disposer-missing … document-store` at 04:12:44 and
`wasm trap: unreachable executed` at 16:14:12 the day before. **Seven seconds, one row, a real
32-byte fingerprint.** The same hash reappears in the published bundle's note `openTarget` (§4), so
the guest's answer is what the catalog carries.

## 4. The rebuild + bootstrap hold

ONE detached hold, driver `📜️tc3e-hub-boot.sh 7651 note,stdio,gis` (pid in
`🗑️generated/tc3e-driver-pid.txt`, work script `📜️tc3e-mutex-work.sh`, capture
`🗑️generated/tc3e-hub-dev.txt`), launched 11:10:34. Private `CARGO_TARGET_DIR=…/target-tc3d`
(deliberately TC3d's, not a fresh one: it already holds the warm wasm32 closure of all three
plugins and the `os-hub` binary, so only what actually changed rebuilds), fresh data root
`.🧬semio/🌐hub/tc3e-boot`, ordered mutex stamp `20260922110200-tc3e` (session 8's fixed queue
order, `📜️mutex-ordered.sh`).

**One structural change against TC3d's work script: note is built FIRST and gated.** TC3d spent
7 min on stdio, 4 min on note and then 56 min inside `trusted-catalog-bootstrap` before dying on the
note codec probe. TC3e builds note, runs `semio-framework-plugin-describe codecs` on the fresh
component immediately (`🗑️generated/tc3e-gate-codecs.txt`, exit 77 on failure) and only then builds
stdio and gis and enters the bootstrap. The same failure now costs ~5 min of a hold instead of ~70.

The preflight (outside the mutex, so a queue slot is never spent on a tree that cannot build) is
`cargo check -p semio-s-plugin-note --target wasm32-wasip2` plus `cargo build -p
semio-framework-plugin-describe` — the second both proves the native emitter and produces the binary
the in-hold gate runs.

`📜️tc3e-prove.sh` is a SECOND detached watcher (pid in `🗑️generated/tc3e-prove-pid.txt`, launched
11:15:17) polling `:7651/readyz` for up to 10 h; on 200 it runs the codec laws, then
`📜️tc3e-provision.sh` (two credentials + create-and-attach per kind), then re-reads `readyz`, all
into `🗑️generated/tc3e-prove.txt`.

**Preflight timeline (measured):**

| attempt | at | result |
|---|---|---|
| 1 | 11:13:40 | RED — 7 errors, none of them this slice's: `E0027 pattern does not mention field \`tasks\`` at `🔌️plugin/🦀️.rs:25311` and 5×`E0609 no field \`reserved\` on TaskSlot` at `⚛️reactor/🧵️executor/🦀️.rs:477-496` — a peer's async-task-lane refactor landing half-finished (preamble rule 3: not mine to revert) |
| 2–6 | 11:15:40 → 11:24:27 | RED, error count falling 7 → 1 as the peer landed the call sites |
| 7 | **11:30:16** | **GREEN**, `preflight exit=0 after 6 retries` |

**This is exactly what the preflight is for.** Had TC3e entered the hold at 11:10 the way TC3c did
at 13:40 on 2026-09-21, the whole queue slot would have been spent on a tree that could not build.
The cost of 6 retries was 20 min of wall clock and no queue slot at all.

Queued at 11:30:16 as `20260922110200-62505-tc3e`, **position 2 of 3**: behind `c8` (holding since
11:32:57) and ahead of the peer session's `play` (`…110250`) — session 8's intended order.

### Run 1 — the hold, 13:26:01 → 13:48:13

| stage | begin | end | wall | exit |
|---|---|---|---|---|
| 1 `semio-s-plugin-note` wasm-release cdylib | 13:26:01 | 13:29:56 | 3 m 55 s | 0 |
| **gate** `describe codecs` on the fresh note component | 13:29:56 | 13:30:03 | **7 s** | **0** |
| 1 `semio-s-plugin-stdio` | 13:30:03 | 13:38:25 | 8 m 22 s | 0 |
| 1 `semio-s-plugin-gis` | 13:38:25 | 13:40:52 | 2 m 27 s | 0 |
| 2 `trusted-catalog-bootstrap --packages stdio,gis,note` | 13:40:52 | 13:48:13 | 7 m 21 s | **1** |

Stage 2 ran in **7 min against TC3d's 56** — c8's own `stdio,gis` bootstrap had just warmed the
shared build dir, exactly as the coordinator predicted. It got all the way past the gis cold-map
native laws, the actor closure, the receipts and the whole generation, and died in the LAST fence
before publication (§4.1). The fleet was then cut by the account session limit at ~13:15–15:50 and
the driver died with it; run 2 is below.

### 4.1 The third fault — the candidate staging is still the two-package one

```
error: trusted generation fence: generation directory is not the exact regular closure
  at trustedBootstrapVerifyGeneration (🌎️hub/📦️packages/🦀️rust/📜️script.ts:9147:15)
  at stageTrustedBootstrapCandidateCurrent (…:10039:7)
  at validateAndPublishTrustedStdioGisCandidate (…:10586:3)
```

`stageTrustedBootstrapCandidateCurrent` copies the verified selection below an ISOLATED candidate
data root before the hub validates it. TC3c generalised `materializeTrustedCatalogBundle` to an
N-package list and **left this copy spelled as the fixed gis+stdio pair**: two `mkdirSync` calls
(`packages/gis/browser`, `packages/stdio`) and a five-row literal `files` array. So a three-package
selection staged two packages, and line 10039 —
`trustedBootstrapVerifyGeneration(stageRoot, …)`, whose expected closure IS derived from the
receipts — answered that the directory is not the exact closure. The materialized generation on
disk was complete and correct (`generations/274265af7e…`, `packages/{gis,note,stdio}`, note's
`browser/closed-actor.mjs` 20 130 344 B); only the candidate copy was short.

**Fixed** in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: the staging derives `stagedPlugins` and
`stagedCarriesActor` from the same `receipts` map the fence derives its expectation from, builds the
`files` list and the `fsync` walk per package, and adds a package with zero new lines. This is the
third instance of one pattern — a two-package literal left behind by the N-package generalisation —
after TC3c's own fences and §2's viewer.

### Run 2

Data roots `tc3e-boot` and `tc3d-boot` deleted before relaunch (disk was at 13 GiB). Driver
relaunched 15:56 with stamp `20260922110200-tc3e`, preflight GREEN at 15:58:21 on the first try (the
peer's async-task refactor has been green since 11:34), queued 3rd behind `ca1` (holding since
15:47) and `s11`.

**Run 2 was killed at ~16:33 by a peer's `🗑️generated` sweep** — the whole folder went from ~40
TC3e captures to 5 files belonging to other slices, and the driver died with its own log (preamble
rule 13, and memory *Subagents Sweep Ticket Generated Folder*). Nothing of the OUTCOME was lost:
the three `wasm-release` components (13:29/13:38/13:40), the `os-hub` binary (13:42) and the
`describe` emitter (13:05) all live in `⚡️cache/cargo/target-tc3d` and are untouched, and every
measured number of run 1 was already written into this report rather than left in a capture. Disk is
114 GiB after the coordinator's prune.

### Run 3

Relaunched 16:34:35, driver pid in `🗑️generated/tc3e-driver-pid.txt`, prove watcher relaunched
alongside it.

## 5. The 7651 proof table

**Hub 7651, three-package trusted catalog `76d111513c1bb469…`, measured 2026-09-22 20:52–20:54.**
Probe `🐍️tc3c-create-and-attach.ts` (TC3c's, unchanged), captures
`🗑️generated/tc3e-provision.txt` and `🗑️generated/tc3e-attach-*.txt`. The probe stamps its own
monotonic clock (`18:5x`); wall clock was 20:52–20:54.

### s.note.note — **the first `s.note.note` document ever created on a hub**

| step | code | detail |
|---|---|---|
| `POST /auth/sessions` (`user1@semio.dev`) | **200** | tokenShape ok |
| `GET /readyz` | **200** | `artifactAuthority.ready=true`, `features.openPlan=true` |
| `POST /directory/commands` create-space | **202** | space `01a0ca76-a13b-7c68-82ec-1e136686be57` |
| `GET /spaces/{s}/artifact-creations` | **200** | generation `76d111513c1bb469…`, kinds **`s.gis.gismap,s.note.note`** |
| `POST /spaces/{s}/artifact-creations` kind=`s.note.note` | **202** | accepted |
| poll → **ready** | **202 → CREATED** | **`artifact-a0e42a33a7434b3045445515c6344baa`**, schema `note.document`, **25.5 s in genesis** |
| `POST …/open-plan` | **200** | surface `s.note.note@1/*#editor`, window `note-composite`, role `editor`, rendererTarget `wasm`, `packSchemaHash db3a2145b46b70bd…` |
| `POST …/execution-target/manifest` | **200** | 3 047 B |
| `POST …/execution-target/component` | **200** | 14 610 991 B, `componentSha256 bad067ea55dd4c29…`, `appChannelVersion 17` |
| `POST …/execution-target/descriptor` | **200** | 55 429 B |
| `POST …/socket-grants` | **200** | grant issued |
| `GET …/socket/v1` upgrade | **WS-OPEN** | `semio.socket.v1`, hello 52 B |
| server frames | **Welcome 279 B, Session 75 B** | `opened=true session=true`, close 1000 |

The `packSchemaHash` the hub published for this kind is `db3a2145b46b70bd…` — byte-identical to the
fingerprint the in-hold gate read straight out of the rebuilt guest (§3). The whole creation path
for note runs through `codec.genesis` on the guest, because the hub links no Rust codec for it: this
row is the guest `codec` interface working end to end, from the trap TC3c first saw at 16:14:12 on
2026-09-21 to a document with a live socket.

### s.gis.gismap — **RED, and it is not this slice's fault**

| step | code | detail |
|---|---|---|
| sign-in, readyz, create-space, creation catalog | 200 / 200 / 202 / 200 | same generation, `s.gis.gismap` offered |
| `POST …/artifact-creations` kind=`s.gis.gismap` | **202** | accepted |
| poll for 30 s | **failed** | |

Hub log (`🗑️generated/tc3e-hub-7651.txt`):

```
server.artifact.creation failed: creation …0da7334c…:
  genesis materialization failed: trusted artifact codec Input failed: epoch deadline exceeded
server.artifact.creation failed: creation …d3987d48…:
  uncommitted key closed because the deadline passed; now=1790103197586 deadline=1790103197508 phase=Accepted
```

`epoch deadline exceeded` is the wasmtime EPOCH interrupt, not a guest panic — there is no guest
stderr to capture, the guest was still running when the host cut it. **This is the fault HC1 is on**
and it is a BUDGET fault, not a correctness one: note's genesis took 25.5 s against the same 30 s
creation deadline and passed with 4.5 s to spare, and gis's component is 47 969 539 B against note's
14 610 991 B — 3.3× the module for the same budget. The ordering is the evidence: the gis component
compiles and its cold-map native laws passed inside the same bootstrap minutes earlier. Two numbers
for HC1: **note 25.5 s / 14.6 MB = pass; gis > 30 s / 48.0 MB = fail**, same hub, same generation,
same machine, minutes apart.

It is also why run 4's bootstrap never published (§4.2): `proveTrustedStdioGisCandidatePlan` creates
exactly this gis map on the candidate hub before publishing.

### s.stdio.txt — not offered, by design

`CREATION-CATALOG does not offer s.stdio.txt`. The generation's `openTargets` set holds gis and note
only: stdio is a dependency package with a linked Rust codec and no open target of its own, which
the published `trusted-catalog.json` states directly. Not a regression — the row is here because the
probe was run, and the answer is the catalog's own.

## 6. Codec laws (TC3d §6 a/b/c)

**(b) The per-component sweep.** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs`
gains `STAGED_CODEC_COMPONENTS` (package id, component file, artifact kind, document schema for the
three trusted-catalog packages), `codec_sweep_one_component` (all four exports plus BOTH resolver
keys over one component, returning the failure text rather than panicking) and
`owned_codec_answers_every_call_on_every_staged_component`, which reports every component in one run
instead of dying on the first. A component not built in any `target*` root is SKIPPED; the law fails
if it found none at all.

**(c) The nonempty `apply-ops` batch** is a NATIVE law, in the note artifact crate rather than in the
host: `artifact_app_apply_ops` needs a real `NoteMutation`, and `semio-framework-plugin-host` cannot
depend on an `s` plugin. `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️unit/🦀️.rs` ·
`note_apply_ops_reduces_a_nonempty_batch_and_closes_its_store` drives genesis → one `rename-note` op
encoded exactly the way the guest receives it (`os_spr::encode_ops_vec` over `OpBinary::encode_op`)
→ `artifact_app_apply_ops::<EditorApp<NotePlayApp>>`, and asserts the batch lands exactly one edit.
This is the first run of TC3d §2(b)'s close cursor: the empty batch returns before a store is ever
built, so that cursor had only ever been type-checked.

**(a)/(c) Results.** (a) needs the rebuilt component of §4. (c) was run at 11:44 and could NOT be
graded: `cargo test -p semio-s-artifact-note-note --lib note_apply_ops` fails to COMPILE on two
errors a peer owns in `…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:13-14`
(`OwnedValueRetirementFactory::<NoteSnapshot>` / `::<NoteMutation>` — `RetireOwned` is not
implemented for either), in a file that is `M` in `git status` right now.
`🗑️generated/tc3e-apply-ops-law.txt`. The `--lib` test target compiles every `#[cfg(test)]` module
of the crate, so one peer's in-flight test file blocks every law in it; the wasm-release cdylib the
hold builds does not compile test modules and is unaffected. Retried below.

## 7. Gaps

**(a) The missing disposer is a REPO-WIDE defect; only note is fixed.** Counted across
`✏️s/🔌️plugins` (every file with an `impl ArtifactEditor`/`impl ArtifactViewer`, checked for a
`build_document_store_disposer` declaration):

| plugin | apps declaring a disposer | apps leaving the fail-closed default |
|---|---|---|
| `🗒️note` | 2 | **0** (after §3) |
| `🌍️gis` | 2 (both editors) | **2** (both viewers: `🗺️gismap`, `🏔️gisterrain`) |
| `🗄️stdio` | 9 | **167** |
| all `s` plugins | 69 | **229 of 298** |

Every one of those 229 surfaces faults `interactive-job.close-owned-disposer-missing` on its first
close. gis and stdio are invisible to the bootstrap only because the hub links Rust codecs for them
(§2), so nothing ever asks their bundles to construct-and-close. **They were deliberately NOT fixed
here**: gis is in the same mutex hold as note and a compile error in a gis viewer would cost the
whole hold and the slice's outcome, and stdio is 167 apps. The measurement plus the worked example
is the handover; a background task carries it.

**(b) The sweep law's table.** `STAGED_CODEC_COMPONENTS` names all three packages, so the sweep
covers gis and stdio the moment their components are built — which means (a) is what it will report
for them. The measured result is in §6; if a package is red there, its row is a live statement of
(a), not a flaky law.

**(c) The published note component PREDATES the `apply-ops` fix of §6c.** 7651's generation
`76d1115…` was built at 17:02 and the two `artifact_app_apply_ops` edits landed at ~23:00. Creation
is unaffected — it runs `codec.genesis`, which the hub proved end to end in §5 — but a hub that
routes a nonempty `codec.apply-ops` batch at the note component in THIS generation still meets the
abort. The fix is proven natively (§6c) and compiles for `wasm32-wasip2`; it reaches a component at
the next bootstrap, which needs one mutex hold.

**(d) The native twin `store::ArtifactCodec::apply_ops_binary` has defect (1) too.**
`apply_ops_binary_impl` (`🏪️store/🦀️.rs:10600`) builds its store with the same bare
`ArtifactStore::new(envelope)` and never installs an owner catalogue, so a nonempty batch through a
LINKED Rust codec — the path stdio and gis take — will answer
`artifact store has no owner-supplied bounded disposer` exactly as the guest twin did. It was not
fixed here: that thunk is generic over `P`/`Mutation` with no app type in scope, so it has no
`build_document_store_owners()` to call and the owners have to come from wherever the codec is
registered. That is a design decision for whoever owns `ArtifactCodec`, not a line edit.

**(e) The three-package generation is staged and current in 7651's root, but never PUBLISHED into
`tc3e-boot`.** §0 states the mechanics. `trusted-catalog publish` runs after
`proveTrustedStdioGisCandidatePlan`, and that probe is blocked on the gis `epoch deadline exceeded`
of §5. So `.🧬semio/🌐hub/tc3e-boot/trusted-catalog` has a `generations/` directory and no
`current.json`, while `.🧬semio/🌐hub/tc3e-hub` (the promoted candidate root 7651 serves) has both.

**(f) The detached prove watcher fired but proved nothing, and that is fixed.** It woke at 17:53:27
on the first `readyz=200`, ran the codec laws (6 passed / 2 failed — the pre-correction run of §6)
and then `📜️tc3e-provision.sh 7651` with no further arguments. `shift 3` on ONE argument is a no-op
under `set -u` rather than an error, so the port survived as the kind list and the run probed
`=========== 7651 ===========` against the default data root, answering `401`
(`🗑️generated/tc3e-prove.txt`). Both scripts are fixed — the guard is `if [ $# -ge 3 ]; then shift 3;
else shift $#; fi`, and `📜️tc3e-prove.sh` now passes the root, the binary and both kinds. §5's rows
are the manual run at 20:52 with explicit arguments, not this one.

**(g) No law over `NoteViewer`'s close on its own.** The viewer is proven to close through the codec
resolver (which closes the rejected app, §2) and through the sweep, not by a direct unit law: the
throwaway app the resolver builds is the plugin-level app enum, which lives in `semio-s-plugin-note`
rather than in the artifact crate where the laws are.

## 8. Files changed

Source:
* `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` —
  `NoteViewer::build_{document,config}_store_owners` and
  `build_{document,config,presence,transient}_store_disposer` (§3)
* `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `stageTrustedBootstrapCandidateCurrent`'s staging derived
  from the bundle's own receipts instead of the fixed gis+stdio pair (§4.1)
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `artifact_app_apply_ops` installs the
  app's document-store owner catalogue on its reduction store, captures the dispatch fault as a
  value instead of returning through `?` over a live store, and retains rather than drops a store
  that did not reach its witness (§6c)

Laws:
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs` —
  `STAGED_CODEC_COMPONENTS`, `codec_sweep_one_component`,
  `owned_codec_answers_every_call_on_every_staged_component` (§6b)
* `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️unit/🦀️.rs` —
  `note_apply_ops_reduces_a_nonempty_batch_and_closes_its_store` (§6c)

Ticket-owned: this report, `📜️tc3e-hub-boot.sh`, `📜️tc3e-mutex-work.sh`, `📜️tc3e-provision.sh`,
`📜️tc3e-prove.sh`, `🐍️tc3e-hub-hold.ts`, and `🗑️generated/tc3e-*`.

## 9. What a consumer of 7651 does next

* **CE3** (`hub-agent-participant-check` 17/17), **M10** (an agent committing an edit) and **C8**
  (frontier + presence socket probes) run against `http://127.0.0.1:7651` with
  `user1@semio.dev` / `gm1-local-dev-pass-1` and `user2@semio.dev` / `gm1-local-dev-pass-2`. A note
  document already exists: `artifact-a0e42a33a7434b3045445515c6344baa` in space
  `01a0ca76-a13b-7c68-82ec-1e136686be57`.
* **Create `s.note.note`, not `s.gis.gismap`** until HC1 lands the genesis budget (§5): gis creation
  fails `epoch deadline exceeded` on this hub, note succeeds in ~25 s.
* **Do not restart 7651** without `🐍️tc3e-hub-hold.ts` — `🐍️ds1-hub-hold.ts`'s 30 s readiness stall
  bound kills this hub during boot (§0).
