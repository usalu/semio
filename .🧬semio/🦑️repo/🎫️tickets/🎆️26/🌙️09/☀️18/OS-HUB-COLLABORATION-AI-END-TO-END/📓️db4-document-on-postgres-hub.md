# DB4 — a real document created, edited by two sessions and re-attached across a restart, on a Postgres-backed hub

Slice DB4 of ticket 26/09/18, session 8 (2026-09-22). Predecessors: DB2
(`📓️db2-postgres-neo4j-live-lanes.md`, directory lanes live), DB3 (`📓️db3-full-hub-on-postgres.md`,
the whole durable hub on Postgres minus documents). Every number below comes from a capture in
`🗑️generated/db4-*`.

Goal: outcome 2 ("working server hub backend — db, presence, auth") observed with a **document**, on
the non-sqlite backends.

## 0. Status (live, updated per landed item)

| item | state |
|---|---|
| 1 Postgres + Neo4j up (DB4's own stack) | **DONE** 12:24 — `🗑️generated/db4-containers.txt`, `db4-container-pid.txt` |
| — hub binary with the two drivers, private `target-db4` | **DONE** 12:33 — `db4-hub-bin-build.txt`, `Finished dev in 8m 45s` |
| — a trusted catalog published **from today's tree** (the ticket-wide blocker) | **DONE** 12:56 — `db4-catalog-fixture.txt`, §1 |
| 2 hub on 7691, Postgres both halves, `/readyz` | **DONE** — **200, every gate open**, first time in this ticket on a non-sqlite backend (§3); same on the Neo4j-directory lane (§4.5) |
| 3 document created, two sessions, an edit crossing, restart, re-attach | **BLOCKED, root-caused and traced** — creation fails for EVERY backend, including in-process sqlite; the catalog's component bytes are synthetic and genesis is the guest component's authority (§4). Harness written and exercised up to that step |
| 4 hub suite with both drivers in `target-db4` | **DONE** — 349 run, **341 passed**, 8 timed out; 7 of the 8 root-fixed in `.config/nextest.toml`, the 8th is a profile/level mismatch (§5) |
| 5 sqlite-only lanes Postgres lacks | **DONE — there are none**, and §6 says why structurally |
| — three hub defects found and fixed | `integration-fixtures` would not compile (D8–D10); a failed creation reported nothing anywhere (D11, now observed at runtime, §4.3b) |

## 1. Inherited state and the blocker DB4 starts from

DB2 proved the **directory** on a live PostgreSQL and Neo4j; DB3 proved the **document store** too
(`DbIoAsyncDriverRuntime` seam) and left one gap as "the single biggest remaining step":
DB3 §8.1 — *no artifact was created on the Postgres hub, and `/readyz` never reached `200`*, because
the artifact-authority gate needs a **published trusted catalog** in the data root.

HT16 §5.1 then measured that every catalog root on disk (`gm1-boot`, `hs1-boot`, `jc1-boot`,
`rb1-prod`) is refused by a hub built from the current tree — commit `50c97b2051` renamed
`TrustedBundleProfileV1::open_target` to `open_targets` under `#[serde(deny_unknown_fields)]`. The
brief pointed DB4 at TC3e's republish for 7651; at 12:20 `.🧬semio/🌐hub/tc3e-boot` was still empty
and nothing listened on 7651, so DB4 did **not** wait for it.

**The unblock DB4 used instead.** The hub already knows how to build a verified catalog from its own
tree without any wasm build, jco step or the fleet's wasm mutex: the `integration-fixtures` feature
compiles `🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs` into the crate, and the bin law
`tests::checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog`
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:1165`) writes a complete data root — `trusted-catalog/current.json`
plus a six-file generation — and then asserts `TrustedCatalogLoader::load_current` accepts it.

**That feature did not compile, in five places, and had not for at least a day** (every file
involved was last touched 09-20/09-21; nothing in the fleet builds `integration-fixtures`, so
`cargo check -p semio-hub --all-targets` with the default feature set never expanded this code).
Fixed here — see §8 D8–D10. After the fix (`🗑️generated/db4-catalog-fixture.txt`):

```
running 1 test
[DEBUG] checkpoint process fixture relocated files=6 packages=2 codecs=28
test tests::checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog ... ok
test result: ok. 1 passed; 0 failed; … finished in 1.59s
```

Generation `7a043f2ae32f798042afd38476a6718ae846f3cd3beabe0a93c129d75f8cef06`, profile
`gis-map-integration-fixtures`, kind `s.gis.gismap` / schema `gis.map`, 2 packages, 28 codecs —
**built from today's tree, so `openTargets` is what it carries.** DB4 copies that root (never shares
it) into its own hub data root.

## 2. The databases

`$T/🔣️db4-compose.yaml` — `postgres:16-alpine` on 127.0.0.1:**5435**, `neo4j:5-community` on
**7690**/7477, named volumes (`db4-postgres`, `db4-neo4j-data`, `db4-neo4j-logs`); bind mounts under
`/Users/ueli/Documents` remain refused by this Docker Desktop (DB2 §8.4). Ports are DB4's own so
nothing collides with DB2's 5433/7688, DB3's 5434/7689 or the stray `db2-probe` on 5499 (still
running, still not ours, still untouched).

Started by this slice at 12:24:25 (`🗑️generated/db4-container-pid.txt`):

| container | image | host pid | ports |
|---|---|---|---|
| `semio-db4-postgres-1` (`6f346cf17f67`) | `postgres:16-alpine` | 83204 | 127.0.0.1:5435→5432 |
| `semio-db4-neo4j-1` (`29ec1f4b40e2`) | `neo4j:5-community` | 83203 | 127.0.0.1:7690→7687, 7477→7474 |

Both were **stopped and their volumes removed at 16:35** (`docker compose -f "$T/🔣️db4-compose.yaml"
down -v`) once the machine went to load 267 with swap full; `📜️db4-postgres-hub-boot.sh` brings the
stack back on its own, and `--fresh` recreates the schema, so nothing in this report needs them to
stay up. Only those two were ever DB4's.

## 3. The hub on 7691 — `/readyz` **200**, every gate open, on Postgres

Binary `⚡️cache/cargo/target-db4/debug/os-hub` (`--features postgres,neo4j`, 329 MB, 12:33), boot
script `$T/📜️db4-postgres-hub-boot.sh`, data root `.🧬semio/🌐hub/db4-pg` (fresh, mode 700, DB4's
own — never shared), `OS_HUB_STORAGE_BACKEND=postgres` **and**
`OS_HUB_DIRECTORY_BACKEND=postgres` on `postgres://…@127.0.0.1:5435/db4`,
`OS_HUB_CREDENTIAL_SIGN_IN=true`, port **7691**.
Capture `🗑️generated/db4-pg-document-lane-run2.txt`:

```
boot first: pid=34779 after 2s
PASS 1 readyz status: 200
PASS 1 readyz says ready: "ready"
PASS 1 every gate is open: [true,true,true,true,true,true]
```

The six gates are `directory`, `storage`, `artifactAuthority`, `artifactCasBarrier`,
`artifactPublication`, `adminAssets`. **This is the first time in this ticket that a hub on a
non-sqlite backend answered `/readyz` `200`** — DB2 and DB3 both stopped at `503` with
`artifactAuthority` blocked by `trusted-catalog-never-published-in-this-data-root`, which is
DB3 §8 gap 1's first half. The catalog of §1 is what opens it.

Two humans then sign in with credentials, create a space and the second is promoted to Author, all
on the Postgres directory (same capture): `PASS 2 both humans signed in [200,200]`,
`PASS 2 create-space status 202`, `PASS 2 the second human is an Author 202`.

One probe line is a **probe defect, not a hub defect**: `1 the loaded catalog is the published
generation` reads `artifactAuthority.generationId` off `/readyz`, which the readiness body does not
publish; the hub's own boot trace states the generation instead. It is left FAILing rather than
silently deleted, and named here.

## 4. The document lane — and the defect that blocks it, which is NOT the database

`$T/🐍️db4-document-lane.ts` drives the seven steps of the brief against the live hub. Steps 1–2
pass (§3). **Step 3 does not: `POST /spaces/{s}/artifact-creations` is accepted and then goes to
phase `failed` within one poll**, every time:

```
PASS 3 artifact-creation accepted: true
creation status body: {"schema":"semio.hub.space-artifact-creation-status/v1",…,"phase":"failed"}
```

### 4.1 It is not Postgres, and it is not this slice's setup

| run | storage | directory | capture | step 3 |
|---|---|---|---|---|
| live hub 7691 | **postgres** | postgres | `db4-pg-document-lane-run2.txt` | `failed` |
| live hub 7692 | **fs** | postgres | `db4-fs-storage-ab.txt` | `failed` |
| **in-process law**, no HTTP, no database container | **fs** | **sqlite** | `db4-inprocess-creation-law.txt` | `failed` |

The third row is the decisive one. `tests::space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed`
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:238`) is the hub's OWN law for this route, on sqlite and the
filesystem, in one process, and it fails at exactly the same place:

```
panicked at …/🧪️tests/🔬️bin-unit/🦀️.rs:312: creation reached an unexpected terminal phase
test result: FAILED. 0 passed; 1 failed; … finished in 2.77s
```

So **artifact creation is broken in the current tree for every backend**, and DB3 §8 gap 1 ("no
artifact was created on the Postgres hub") was never only about Postgres. It has been invisible
because this law is gated on `integration-fixtures`, and that feature did not compile (§1, D8–D10) —
so `cargo check -p semio-hub --all-targets` and the 330-test suite both expand right past it.

### 4.2 The second defect: a creation that fails says nothing, anywhere

Diagnosing 4.1 was blind work, and that is itself a defect of the same class as DB3's D6, one layer
deeper. `ArtifactCreationServiceV1::terminal` (`🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs`)
is reached from four places — a catalog that no longer matches the accepted intent, a catalog that
no longer matches the prepared intent, a failed `materialize_genesis`, and a failed publication —
and **all four dropped their `AuthorityError` on the floor** and wrote a durable `Failed` fact whose
only content is the word `failed`. The HTTP route emitted nothing at all: in
`db4-pg-document-lane-run2.txt` the hub's own trace goes straight from `upsert-member` to silence.

Fixed here (D11, §8): a defaulted `fault(&str)` on the `AuthorityOperationControl` port — the
host-owned observation channel that already carries the clock, cancellation and progress — carries
the sentence, `terminal` states it at all four call sites, and `ArtifactCreationHttpControlV1` keeps
the first one so the detached execution task emits it as a `server.artifact.creation` `Failed` trace
record. Nothing is invented: a control with nowhere to put a sentence keeps the empty default.

### 4.3 The root cause, read out of the code rather than guessed

Creation genesis is **the guest component's authority, always** — stated in so many words at
`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:362`:

> *"Creation authority is the COMPONENT's, always. The `dialect` the caller carries is not passed to
> the guest: the guest stamps its own app's dialect …"*

`VerifiedNativeArtifactCodec::initial_pair` (`:365`) therefore calls `self.guest.genesis(document_id)`
unconditionally — it never consults `self.codec`, the linked NATIVE codec, even when one exists. And
`GuestArtifactCodecBinding::genesis` (`:279`) begins with `self.component.compiled().await?`, i.e. it
compiles and instantiates the package's wasm component.

The `integration-fixtures` profile cannot survive that, and says so itself
(`🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs:8` and `:24`):

> *"Only the component bytes are synthetic, as are the browser actor bytes: this profile never
> executes either, and must never be offered as evidence that one was executed."*
> `const SYNTHETIC_COMPONENT: &[u8] = b"synthetic-gis-component-for-hub-integration-fixtures-profile";`

So the catalog of §1 verifies, loads, opens every readiness gate and offers `s.gis.gismap` as a
creatable kind — and then the first genesis dies compiling 60 bytes of ASCII as a wasm component.
**Every step of the creation route up to the factory call is proven working on Postgres; the factory
call needs a package whose component bytes are real.**

This also dates the break: `space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed`
asserts *"actual native genesis did not become Ready"* — it was written when genesis came from the
linked native codec. When genesis moved to the guest, the law became unsatisfiable with its own
fixture, and nothing caught it because `integration-fixtures` had stopped compiling (§1).

### 4.3b The hub says it itself, now — D11 observed at runtime

With the rebuilt binary (13:21) the same run prints the sentence that did not exist before
(`🗑️generated/db4-pg-document-lane-run3.txt`):

```
{"level":"error","event":"server.artifact.creation","outcome":"failed",
 "detail":"creation 01a0c964-52a3-72ee-86f3-968bf94aba84 111…111: genesis materialization failed:
           trusted artifact catalog failed: semio:gis: plugin: wasm decode:
           expected WebAssembly component version 13.1"}
```

`wasm decode: expected WebAssembly component version 13.1` is the 60 synthetic ASCII bytes of §4.3
being handed to the component decoder. The reading in §4.3 was made from the source before this
build existed; the trace confirms it word for word. The same run's step 2 now also asserts the
loaded generation directly off `GET /spaces/{id}/artifact-creations`:

```
PASS 2 the loaded catalog is the published generation: "7a043f2ae32f798042afd38476a6718ae846f3cd3beabe0a93c129d75f8cef06"
PASS 2 the catalog offers the kind this lane creates: ["s.gis.gismap"]
```

### 4.4 What this means for the document lane

A document on a Postgres hub needs a trusted catalog whose gis (or note) package carries a **real
built component** — which is exactly TC3e's republish, and the brief's plan. At 13:10
`.🧬semio/🌐hub/tc3e-boot` is still empty, nothing listens on 7651, TC3e's last capture
(`🗑️generated/tc3e-hub-dev.txt`) was written at 11:30, and `/tmp/semio-wasm-build.lock/owner` has
said `c8 11:32:57` for 1 h 40 m. DB4 did not take that lane: publishing a second real catalog would
duplicate TC3e's work and contend for the same mutex the whole fleet is queued behind.

**So steps 3–7 of the lane are NOT observed, and this report does not claim them.** The harness that
runs them is written, exercised up to the blocking step and deterministic
(`$T/🐍️db4-document-lane.ts`): point it at a data root holding a real catalog with
`DB4_HUB_DATA=… zsh 📜️db4-postgres-hub-boot.sh --fresh` and the remaining five steps run without
further work.

### 4.5 Both directory backends reach the same point

| run | storage | directory | port | capture | result |
|---|---|---|---|---|---|
| Postgres both halves | postgres | postgres | 7691 | `db4-pg-document-lane-run2.txt` | gates open, steps 1–2 PASS, step 3 blocked |
| documents on Postgres, directory on **Neo4j** | postgres | neo4j | 7693 | `db4-neo4j-directory-lane.txt` | **identical**, same PASS lines, same block |

Both are the first `/readyz` **200** hubs on a non-sqlite backend in this ticket.

## 5. The hub suite with the Postgres and Neo4j drivers compiled in — 349 run, 341 passed

`$T/📜️db4-hub-run.sh` (DB4's own copy of `📜️ht16-hub-run.sh`, never run in place) with
`FEATURES=postgres,neo4j`, `CARGO_INCREMENTAL=0`,
`CARGO_TARGET_DIR=⚡️cache/cargo/target-db4`, started 15:56 at load ≈ 7:

```
Starting 349 tests across 2 binaries
Summary [ 614.791s] 349 tests run: 341 passed, 8 timed out, 0 skipped
```

Captures: `🗑️generated/db4-hub-nextest-full-db4a.txt`, `…-latest-db4a.txt`,
`db4-hub-suite-reds.txt`. Build phases: bin `4m 49s` (`db4-hub-bin-build.txt`), test targets
`2m 14s` (`db4-hub-testbuild.txt`).

| | HT16 (12:02, default features) | DB4 (16:07, `postgres,neo4j`) |
|---|---|---|
| tests | 330 | **349** (+19: the postgres and neo4j directory lanes DB2/DB3 added, which only exist when those drivers are compiled) |
| passed | 330 | **341** |
| failed / timed out | 0 | **8 timed out** |

### 5.1 Seven of the eight are one defect, and it is fixed

All seven are `directory::neo4j::…`, each killed at exactly the `long` profile's `300 s`
`terminate-after`. They are not slow laws: DB2 measured the same four in **76 s** and DB3 the same
seven in **98.6 s** — when the lane ran **by itself**.

The cause is the lane fixture's shape. Each of these tests `docker run`s its **own**
`neo4j:5-community` and polls `connect` for up to 60 s
(`🌎️hub/📇️directory/🌐️neo4j/🧪️tests/🔬️unit/🦀️.rs:28-48`); `Drop` removes it. Run inside the full
suite, nextest starts them concurrently, so seven JVM containers boot at once and none of them
answers inside the budget. Measured side effect: when nextest terminated the seven tests their
`Drop` never ran, and **seven orphaned `semio-hub-neo4j-*` containers were still up afterwards**
(removed by this slice, `docker ps` before/after).

Root fix, landed in `.config/nextest.toml`: a `live-database-lanes` test group with
`max-threads = 1`, applied by a `profile.default` override to
`test(/^directory::(neo4j|postgres)::/)`, so every live-database law runs one at a time in every
profile while the rest of the suite keeps its parallelism. The file parses (verified) and the
installed runner is `cargo-nextest 0.9.140`, well past the 0.9.48 that introduced test groups.
**It has not been executed**: the machine went to load 267 with swap full at 16:2x and the
coordinator's standing rule forbids cargo above load 20, so the re-run that would show
`349 tests run: 349 passed` is owed — see §7.

### 5.2 The eighth is a level/profile mismatch, not a lane defect

`directory::tests::exhaustive::artifact_chunk_cas_opaque_continuation_converges_after_page_overflow_cancel_and_resume`
is an **`exhaustive::`-level law**, and `.config/nextest.toml` gives that level a `900 s`
`slow-timeout`. The coordinator's hub line (and therefore HT16's copy and mine) runs the whole tree
under `--profile long`, i.e. `300 s` — so this law is charged a budget less than a third of its own
level's. HT16 measured it at **65.2 s** on 09-22 at 12:02 and it passed; today, behind seven
starving neo4j containers, it crossed 300 s and was terminated. Nothing about the law changed.
The honest reading is that `--profile long` over a tree that contains `exhaustive::` laws is a
mis-budget that only bites under load; the level-aware verb (`os-hub:test-exhaustive`) is what
gives it its 900 s.

## 6. What sqlite implements that Postgres and Neo4j do not — **nothing**

The brief asked for anything sqlite-only (sagas, checkpoints, rate limits) to be implemented for
Postgres rather than descoped. Measured, there is no such thing, and the reason is structural:

1. **The directory is a closed-set dispatch, not a capability negotiation.**
   `HubDirectories` (`🌎️hub/📇️directory/🦀️.rs:3452`) is one hand-written enum with one variant per
   compiled backend and a match-delegating `impl HubDirectory`. Every method of the `HubDirectory`
   trait — including all the checkpoint methods (`get_artifact_checkpoint`,
   `get_active_artifact_checkpoint`, `append_reserved_artifact_checkpoint`,
   `list_artifact_checkpoint_lineage`, `artifact_checkpoint_count`, the checkpoint-publication
   claim/release pair) and the artifact-creation facts — must be implemented by all three or the
   crate does not compile. `grep -rn "unimplemented!\|todo!\|Unsupported\|not supported"` over
   `🌎️hub/📇️directory/**` (excluding tests) returns **0 hits** on all three backends.
2. **The document store is one closed task set.** `DbIoTask` (`🛢️db/🗄️storage/🦀️.rs:2026`) has 33
   variants, and the exact set of variant names each backend's executor matches on is **identical**:
   `comm -23` between sqlite and postgres, and between sqlite and neo4j, is empty (33 = 33 = 33).
3. **Sagas are not a backend feature.** `drain_instance_sagas` (`🏗️bootstrap/🦀️.rs:555`) drains the
   instance outbox of the four *server-product* stores, which are filesystem-only by design for
   every backend (DB3 §6) — a Postgres hub runs the same saga cadence over the same stores.
4. **The rate limiter is not durable state at all.** `🌎️hub/🔐️auth/🚦️rate-limit/🦀️.rs` is a
   `HashMap` behind a `Mutex` with an injected clock, per process, in front of sign-in, directory
   commands, invite redemption and socket grants. It behaves identically on every backend — and it
   is per-process rather than per-cluster, which is a horizontal-scaling statement, not a Postgres
   gap.

So nothing was descoped, because nothing is missing at the interface. The asymmetry between sqlite
and the other two is **what has been executed**, not what exists — which is the gap DB2, DB3 and
this slice have been closing one lane at a time.

## 7. Honest gaps

1. **The document itself is not observed.** No document was created, no document socket was opened,
   no edit crossed, nothing was re-attached after a restart. §4.3 names the exact reason and §4.4 the
   exact remedy. Everything this report claims about documents is about the route *up to* the
   factory call.
2. **The stale creation law is documented, not repaired.**
   `space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed` now needs a
   profile whose component bytes are real; giving the `integration-fixtures` builder a real
   component means a wasm32 build through the fleet mutex, which is TC3e's lane and was not taken.
   The law compiles again (§8 D8–D10) and fails honestly instead of being invisible.
3. **The `live-database-lanes` test group (§5.1) is landed but not executed.** Its effect —
   `349 tests run: 349 passed` — needs one more suite run, and the machine went to load 267 with
   swap full before it could happen. The config parses and the runner supports it; that is all this
   report claims for it.
4. **The suite's other red is not repaired, only explained** (§5.2): an `exhaustive::`-level law
   charged the `long` profile's 300 s by the coordinator's own hub line. Changing that line is the
   coordinator's call, not a slice's.
5. **`semio-hub` source changed here, so the coordinator's hub binary and suite are stale** by
   D11 + the three `integration-fixtures` compile fixes. A rerun of
   `📜️coordinator-hub-run.sh` is owed. DB4's own build+suite of 15:53–16:07 already contains them.
6. **The probe's step-1 catalog assertion moved.** The first two runs FAIL
   `1 the loaded catalog is the published generation` because `/readyz` publishes no
   `artifactAuthority.generationId`; the check now reads `GET /spaces/{id}/artifact-creations`
   instead, which is the route that actually names the generation. The two captures keep the old
   FAIL line and are not re-run.
7. **DB2 §8's stray `db2-probe` container (`92dfd5a3a0c6`, port 5499) is still running** and is
   still not ours. Untouched, for the third slice running.
8. **Disk was 24 GiB free at 13:10** (rule 30's prune threshold is 45 GiB); this slice added a
   329 MB binary in `target-db4` and ~2 MB of captures.

## 7b. 🚨 `🗑️generated/` was wiped machine-wide at 16:35 — what survives

Between 16:33 and 16:35 the ticket's whole `🗑️generated/` folder went from 882 files to 21
(most of them zero bytes), and free disk went from 13 GiB to 114 GiB: the same machine-wide clean
the preamble's rule 28 records for 2026-09-21 02:30, not a delete by this slice. **Every
`db4-*` capture this report cites was destroyed, along with every other slice's.**

What that does and does not cost:

* **Every number in this report was written into the report as it was measured** (preamble rule 12),
  and the decisive lines are quoted verbatim above — the suite summary (§5), the creation fault
  trace (§4.3b), the readiness and step-2 PASS lines (§3, §4.5), the in-process law's panic (§4.1).
  Those quotes are now the evidence; the files behind them are gone.
* **The catalog fixture was rebuilt immediately after the wipe** from the surviving test binary
  (`⚡️cache/cargo/build/debug/build/semio-hub/993a5db9b52a0e74/out/os_hub-993a5db9b52a0e74`, no
  cargo run needed) and came back with the **identical** generation
  `7a043f2ae32f798042afd38476a6718ae846f3cd3beabe0a93c129d75f8cef06`, 6 files, 2 packages,
  28 codecs — so §1's artefact exists again and the builder is deterministic
  (`🗑️generated/db4-catalog-fixture.txt`, 16:36).
* **Regenerating the rest needs no cargo except the suite**: the document lane is
  `zsh 📜️db4-postgres-hub-boot.sh --fresh` (it brings the compose stack back itself), the Neo4j
  variant adds `--directory-backend neo4j`, and the suite is `sh 📜️db4-hub-run.sh <tag>` once the
  load rule allows.

## 8. Files changed

Source — all in `🌎️hub/**`, whose Rust file set HT16 released when it finished this session:

- `🌎️hub/📦️packages/🦀️rust/🦀️.rs` — **D8**: `test_artifact_root` was `#[cfg(test)]` while
  `trusted_catalog_fixture` (compiled under `integration-fixtures`, a feature whose whole purpose is
  to serve the crate's other targets) calls it. Gate widened to
  `any(test, feature = "integration-fixtures")`.
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3574` — **D9**: the `integration-fixtures` checkpoint gate
  referenced `progress_cursor` while the binding is `_progress_cursor` (the value is genuinely
  unused in a production build, so the underscore is right and the reference was wrong).
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — **D10**: three stale lines in the `integration-fixtures`
  creation law — a missing `SpaceArtifactCreationCatalogV1` import, `ready.document_id` after the
  field was renamed to `artifact_id`, and two `tokio::join!`ed requests borrowing a temporary header
  array (E0716). With these, `--features integration-fixtures` compiles for the first time in at
  least a day.
- `🌎️hub/🗿️artifact-authority/🦀️.rs` — **D11a**: `AuthorityOperationControl::fault(&str)`, defaulted
  to a no-op, beside the existing clock/cancellation/progress ports.
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs` — **D11b**: `terminal` takes the
  reason and the context and states it; all four call sites now name why creation ended
  (catalog-no-longer-matches at accept and at publish, genesis materialization, genesis
  publication).
- `🌎️hub/🏗️bootstrap/🦀️.rs` — **D11c**: `ArtifactCreationHttpControlV1` keeps the first fault and
  the detached execution task emits it as a `server.artifact.creation` `Failed` trace record.

Repo configuration:

- `.config/nextest.toml` — the `live-database-lanes` test group and its `profile.default` override
  (§5.1). Nothing else in that file is touched; the four level profiles are unchanged.

Ticket-local:

- `$T/🔣️db4-compose.yaml`, `$T/📜️db4-postgres-hub-boot.sh`, `$T/📜️db4-hub-run.sh`,
  `$T/🐍️db4-document-lane.ts` — **new**.
- `$T/🗑️generated/db4-*` — every capture named in this report, plus
  `db4-fixture/checkpoint-publication-process-fixture/` (the emitted catalog + payload).

Containers started by this slice: `semio-db4-postgres-1`, `semio-db4-neo4j-1` — **stopped with
`down -v` at 16:35**. Seven orphaned `semio-hub-neo4j-*` containers left behind by the suite's
terminated lane tests (§5.1) were removed at 16:10. Data roots created by this slice:
`.🧬semio/🌐hub/{db4-pg,db4-fs,db4-neo4j}` (≈ 780 KB each, kept as evidence). **No hub, serve or
cargo process of this slice is left running** (checked by port and by pid at 16:35); the only
container left on the machine is DB2's stray `db2-probe`, which is not ours.
