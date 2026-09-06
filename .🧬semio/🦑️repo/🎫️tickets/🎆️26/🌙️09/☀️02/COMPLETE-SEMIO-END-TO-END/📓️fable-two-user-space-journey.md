# 🚻️ Fable — Two-User Space Journey (`fable-two-user-space-journey`)

Ticket `26/09/02/COMPLETE-SEMIO-END-TO-END`. Packet:
`📓️fable-explore-two-user-journey-readiness.md` §1/§2, with `📓️fable-directory-command-receipt.md`,
`📓️fable-space-administration.md` and `📓️terra-directory-event-page-two-process-journey-p0.md`
consulted for the eight acceptance laws.

This lane adds **one registered gate** — `os-hub:space-journey-{source,process}-check` — whose process
mode drives the whole two-user collaboration journey against **one real SQLite `os-hub` child process**
through the **real authenticated routes only**, and whose source mode proves the journey contract from
a language-neutral fixture with independent oracles and no Rust involved.

---

## 1. What landed

| File | Change |
|---|---|
| `🌎️hub/🧪️fixtures/📇️directory/🚻️space-journey-v1/🔣️.json` | **New.** Neutral journey vectors: 20 ordered steps, 5 honest skips, both session-binding domains, the socket hello schema, the two identities, the space/document literals, the closed privacy substring sets and the 9 routes the journey is allowed to touch. |
| `🌎️hub/🧪️fixtures/📇️directory/🚻️space-journey-v1/🧬️.schema.json` | **New.** JSON-Schema 2020-12 for the fixture: closed `expect` grammar (`receipt` / `status` / `administration-page` / `event-page` / `socket-open` / `wake` / `close`), required EN+DE label pair on every step and skip, `additionalProperties:false` throughout. |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | **New `//#region 🔖️SpaceJourney`**: `DirectorySpaceJourneyFixture`, `SPACE_JOURNEY_ORDER`, `spaceJourneyFixtureLaws`, `spaceJourneyHostiles`, `spaceJourneyAdministrationBinding`, `spaceJourneySocketHelloFrame`, `proveDirectorySpaceJourneyV1` (source oracle), `openLiveDirectorySocket` / `waitForLiveDirectorySocket` / `closeLiveDirectorySocket`, `readLiveDirectoryHistory`, `verifyLiveSpaceJourneyReceipt`, `fetchLiveSpaceAdministrationPage`, `proveDirectorySpaceJourneyV1Process`, `SpaceJourneyCheckScript`; registered as `space-journey-check`. |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `space-journey-source-check`, `space-journey-process-check`. |
| `.vscode/🧩️launch.seed.jsonc` | `⚖️gate🚻️space-journey📐️source` (order 411.09993) and `⚖️gate🚻️space-journey🌎️process` (411.09994), in the existing `4_gate` group beside the sibling directory gates, with a lane-local `SEMIO_TEST_ARTIFACT_DIR`/`CARGO_TARGET_DIR`. `launch.json` regenerated with the plugin-registry generator — never hand-edited. |

No existing runner was replaced: `startLocalHub`, `issueLocalCredential`, `waitForReadiness`,
`finishLocalHub`, `liveDirectoryEventPageUser`, `fetchLiveDirectoryEventPage`,
`postLiveDirectoryCommand`, `createdLiveDirectorySpace` and `hubBinaryPath` are all reused verbatim,
exactly as `proveDirectoryEventPageV1Process` uses them.

---

## 2. The journey the process mode actually drives

One real `os-hub` child (SQLite directory + FS storage via `isolatedSecuritySmoke`), two identities
minted through the private bootstrap pipe, then, over real HTTP/WebSocket:

| # | Step id | What it proves |
|---|---|---|
| 1 | `create-space` | A creates one private space through `POST /directory/commands` as a sealed `DirectoryCommandRequestV1`; the `DirectoryCommandReceiptV1` is `accepted` with exactly 2 durable events and `result.kind = none`. |
| 2 | `create-space-redacted-retry` | The byte-identical retry of the same `requestId` returns `previously-accepted` with **zero** events and no result, and A's whole visible history still contains exactly **one** `space.created`. |
| 3 | `author-reads-administration-page` | `GET /directory/spaces/{id}` returns the `author` variant with `invites` + `capabilities` and 1 member row; its `receiptSha256` and `sessionBindingSha256` are both recomputed independently. |
| 4 | `nonmember-administration-page-denied` | B (no membership) gets `404` with a zero-byte body — the private space does not enumerate. |
| 5 | `author-adds-b-as-spectator` | A admits B through the same receipt route (1 event). |
| 6 | `member-reads-administration-page` | B's page is the `member` variant with 2 member rows and **structurally no** `invites` and no `capabilities` key. |
| 7 | `author-promotes-b` | A promotes B to `author` (1 event). |
| 8 | `announce-document` | A announces one `DocumentDescriptor`, so an exact `DocumentScope` exists for the scoped socket. |
| 9 | `b-creates-private-space` | B creates an unrelated private space; every later privacy law is checked against its id and name. |
| 10 | `open-global-sockets` | A and B each `POST /directory/socket-grants` and dial `/directory/socket/v1` with the grant as the second subprotocol plus the exact tag-7 hello frame. A dials from its own read frontier, never from 0. |
| 11 | `open-scoped-socket` | B issues and consumes a membership-bound grant for the document scope and dials `/directory/spaces/{s}/documents/{d}/socket/v1`. |
| 12 | `member-renames-space` | B renames the shared space as an author (1 event). |
| 13 | `author-observes-dirty-wake` | A's live socket carries B's rename **exactly once** (re-counted after a further 750 ms), and the whole raw frame log contains neither B's private space id nor its name. |
| 14 | `author-reads-ordered-event-page` | A's paged visible history is strictly increasing, duplicate-free, contains the peer rename, and leaks nothing of B's private space. |
| 15 | `author-removes-b` | A removes B (1 event). |
| 16 | `scoped-socket-revoked` | B's scope-bound socket closes with **4401** within the authority tick. |
| 17 | `removed-member-page-denied` | B's page read is `404` with an empty body. |
| 18 | `author-self-revokes` | `DELETE /auth/sessions/me` returns `204`. |
| 19 | `self-revoked-read-denied` | A's stale bearer gets an empty `401`, and A's own global socket closes **4401**. |
| 20 | `restart-preserves-history` | The hub is stopped and a second child started on the **same** data root; a fresh A session reads the space under its renamed name with 1 member row, a **different** session binding, and a durable history still carrying the rename and the `member.removed`. |

Every receipt is verified three ways: an independent `node:crypto` recomputation of both
`commandSha256` and `receiptSha256` over canonical bytes, the repository's own
`parseDirectoryCommandReceiptV1` bound to a `sealDirectoryCommandRequestV1` request (as a third
implementation, never the oracle), and the fixture's declared outcome/event-count/result class.
Every administration page is verified by independent receipt digest **and** an independent rebuild of
the domain-separated session binding from the fixture's declared domain.

### Honest skips (recorded in the fixture, emitted in the trace, never silent)

| Skip id | Why |
|---|---|
| `document-open-plan-and-presence` | No document open plan is obtainable without a trusted catalog profile. The runner **probes** `POST /spaces/{s}/documents/{d}/open-plan` with a well-formed `semio.hub.document-open-intent/v1` and asserts the answer is one of the fixture's admissible honest denials `[400, 401, 403, 404, 409, 503]`; a `2xx` deliberately **fails** the gate, because that would mean the skip is stale. No document-scoped presence socket is dialled and **no roster is claimed**. |
| `document-content-edit` | No landed app content-mutation factory is wired to the document socket (Flow's `addWidget` is still `BatchOnlyPendingRewrite`). Zero content commands are issued. |
| `rendered-surface` | No browser and no WGPU renderer is driven. No pixel, no surface. |
| `invite-token-leg` | Membership is established with the direct `upsert-member` command, exactly as `terra-three-pillar` recommends; the invite-secret delivery/redemption legs belong to a separate packet. |
| `alternate-directory-backends` | SQLite only. PostgreSQL and Neo4j are neither compiled nor executed here. |

### Trace output

The process runner writes, under `SEMIO_TEST_ARTIFACT_DIR`:
`space-journey-trace.json` (schema `semio.hub.directory-space-journey-trace/v1`: per-step outcome,
duration, detail, both hub child exit codes, the journey wall time, the **`os-hub` binary SHA-256**,
and every skip with its reason) and `space-journey-trace.txt` (one block per entry carrying **both**
the English and the German label — no default language). Before either file is written, the whole
serialized trace is checked against the fixture's `forbiddenTraceSubstrings`
(`Bearer `, `capability`, `inviteToken`, `secretDigest`, `passwordHash`, `ssoSubject`, `channelKey`),
so a secret cannot reach the artifact.

---

## 3. Source-mode oracle (no Rust, no hub, no compiler)

`bun ./📜️script.ts space-journey-check --source` → **`checks=149 phase=source steps=20 skips=5`**.

- **AJV 2020 (strict, allErrors)** over the published fixture schema — the third-party oracle.
- **`node:crypto` rebuild** of the administration session binding, including a **length-prefix
  aliasing probe** (`session-journey-0` + `1user-journey-01` vs `session-journey-01` + `user-journey-01`:
  same concatenation, different digest) proving the domain separation is not forgeable.
- **Byte-exact rebuild of the tag-7 socket hello frame** from the fixture's declared schema, checked
  against the shared client's own literal encoding (`b"semio.directory.events/v1"` and
  `&[0, 7, 1, 1, schema.len() as u8]`).
- **Cross-language constant equality**: the fixture's `commandRequestBytes` / `responseBytes` /
  `administrationPageBytes` must equal the *arithmetic* of the shared Rust constants
  (`8 * 1024`, `64 * 1024`, `48 * 1024`) as declared in `📇️directory/🧬️schema/🦀️.rs`.
- **Second implementation of the descriptor admission law**: the fixture's document descriptor is
  re-validated with an independent transcription of `validate_document_descriptor`'s hash/frontier
  rules, and the hub's own function is fenced by name and by its `commit_seq > head_seq` clause.
- **Route ownership fence**: every one of the 9 routes the journey names must appear as a registered
  `.route("…"` in `🚀️bin.rs`, and every step must name a route from that set.
- **Runner-reference fence**: every step id and skip id must appear literally inside
  `proveDirectorySpaceJourneyV1Process`'s own body — a step cannot be declared and then never driven.
- **Session-binding domain fence**: both NUL-terminated domain separators must be the hub's own.
- **EN/DE law**: every step and skip carries a non-empty, trimmed, *distinct* English and German label.
- **10 hostile mutations**, each proven rejected by its own law (verified by printing each rejection
  message during development, then removing the temporary log):

  | Hostile | Rejected by |
  |---|---|
  | `reordered-steps` | `space journey step order differs from the runner's declared order` |
  | `repeated-skip-id` | `space journey skip ids repeat or collide with a step id` |
  | `skip-collides-with-step` | same collision law |
  | `default-language` | `entry 'create-space' has no distinct English and German label` |
  | `untrimmed-label` | `entry 'create-space' label is not trimmed` |
  | `unreferenced-skip` | `runner never references entry 'never-referenced-skip'` |
  | `unregistered-route` | `names route '/directory/forged' which the hub does not register` |
  | `foreign-step-route` | `step 'create-space' names an unregistered route` |
  | `skip-admits-success` | `skip 'document-open-plan-and-presence' is not an honest denial` |
  | `unexplained-skip` | same honest-denial law (reason-length branch) |

---

## 4. Verification — exact commands and results

| # | Command | Result |
|---|---|---|
| V1 | `bun ./📜️script.ts space-journey-check --source` (cwd `🌎️hub/📦️packages/🦀️rust`) | **PASS** — `space-journey-check: checks=149 phase=source steps=20 skips=5` |
| V2 | `bun ./📜️script.ts space-journey-check source` (bare phase, same cwd) | **PASS** — identical line; both spellings accepted |
| V3 | `bun ./📜️script.ts space-journey-check native` | **Rejected as designed** — `space-journey-check accepts --source or --process` |
| V4 | `bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache` (repo root) | **PASS** — `.vscode/launch.json regenerated`; both new gate entries present at lines 5820 and 5831 |
| V5 | `bun nx run os-hub:space-journey-source-check --skip-nx-cache` (repo root) | **PASS** — `checks=149 phase=source steps=20 skips=5`, `Successfully ran target space-journey-source-check for project os-hub` |
| V6 | process mode | see §5 |

---

## 5. Process mode — build status

Lane-private target dir: `…/scratchpad/fable-two-user-space-journey-target`, seeded at **23:44** by an
APFS clone (`cp -c -R`, 32 s, 6.0 GB) of the coordinator's warm `fable-coordinator-hub-target` — taken
only after `lsof …/debug/.cargo-lock` reported the lock free (it had been held continuously from
before 22:40 by two successive coordinator `cargo check -p semio-hub --bin os-hub --tests --features
sqlite` runs, pids 51063 then 22821).

**W1 — `cargo check` on the warm clone**, started 23:46:

```
cd 🌎️hub/📦️packages/🦀️rust
CARGO_TARGET_DIR=<lane-private> CARGO_BUILD_JOBS=4 RUSTC_WRAPPER="" \
  cargo check --manifest-path Cargo.toml -p semio-hub --bin os-hub --features sqlite --message-format=short
```

Progress observed directly from the lane-private target dir's own `rustc` children:

| Time | Crate under compilation | Meaning |
|---|---|---|
| 00:03 | `db` | The crate carrying the Sol fleet's WAL-writer-permit rewrite |
| 00:08 | `semio_framework_plugin`, `semio_framework_plugin_host`, `semio_framework_os_infinite` | `db` **passed** — no diagnostic was emitted for it, so the 21:05 `E0308 expected &WalWriterPermit` blocker is cleared as of this build |
| 00:23 | `semio_s_plugin_stdio` | the `native-artifact-execution` default feature's heaviest dependency |
| 00:46 | `semio_s_plugin_stdio` (32 min elapsed) | still the same crate — see the starvation measurement below |

Machine load during the run: `load averages: 138.55 130.81 113.07` at 00:12, with ~20 peer `cargo`
invocations live — this build is CPU-starved, not stalled.

**Expected failure ahead (external, attributed):** `semio-hub`'s `default = ["sqlite",
"native-artifact-execution"]` and `native-artifact-execution = [… "dep:semio-s-plugin-vcs"]`
(`Cargo.toml:22,30,47`), and the coordinator reported at 00:10 that a sibling lane linked
`semio-s-plugin-vcs` into the hub's default build while that crate carries 18 compile errors from a
framework migration, with `fable-vcs-compile-repair` fixing it. Any `os-hub` build started after
~23:50 therefore fails inside `semio-s-plugin-vcs`. See §7 for the retry record.

*(result recorded below)*

---

## 6. Honest nonclaims

1. **No document content edit.** No application mutation is issued; nothing is written into a
   document.
2. **No rendered surface.** No browser, no WGPU, no pixel. This is a hub-process proof.
3. **No invite-token leg.** No invitation is minted, delivered or redeemed by this gate.
4. **No PostgreSQL and no Neo4j.** SQLite only; neither alternate backend is compiled or executed.
5. **No presence roster.** The document open plan is unobtainable in the isolated security-smoke
   profile, so no document-scoped presence socket is opened and no roster or lease expiry is claimed.
   The 15 s `PRESENCE_LEASE_TTL_MS` law remains unexercised by this lane.
6. **The global directory socket is bound to the session, not to a membership.** Removing B from the
   space is terminal for B's **scope-bound** socket (4401) but does **not** by itself close B's
   **global** `/directory/socket/v1`, because that socket's authority is the session binding with no
   space qualifier. The gate therefore asserts 4401 on the scoped socket and merely *records* the
   global socket's state at that moment; the global 4401 is asserted only where it is actually
   caused — after A's own session revocation. This is a finding about the current design, recorded
   rather than papered over.
7. **The hub bin is built with `--features sqlite`, not `--all-features`.** This is deliberate and
   matches the `alternate-directory-backends` skip: `semio-hub`'s `default = ["sqlite",
   "native-artifact-execution"]`, so this is the shipped configuration the journey actually exercises,
   and `--all-features` would additionally compile the PostgreSQL and Neo4j drivers plus `test-support`
   that this gate explicitly does **not** run. Consequently this gate is **not** evidence that the
   `postgres`/`neo4j` feature combinations compile. Nothing here compiles any
   `#[cfg(target_arch = "wasm32")]` code either.
8. **Terra's eight acceptance laws are only partly covered.** This gate covers the privacy law (7),
   the no-duplicates law (8) across pages/replay/restart, and the disconnect law's 4401 half (5). It
   does **not** cover the receipt-ACK law (1), the frontier law (2), the gap-injection law (3), the
   retry law (4) or the rebootstrap law (6), because those require a landed retained-Home ACK owner
   in a target runtime, which this lane deliberately did not build.

---

## 7. Peer interactions and external blockers

- **Shared cargo build-directory contention.** The coordinator's warm `fable-coordinator-hub-target`
  was held continuously by `cargo check -p semio-hub --bin os-hub --tests --features sqlite` (pid
  51063, 4 h 44 min elapsed at 23:38, then pid 22821) until **23:44**. No clone was taken while the
  lock was held, and this lane never used the collision-prone shared `hub-target` name — its target
  dir is lane-qualified (`fable-two-user-space-journey-target`).
- **The `db` crate's live WAL-writer-permit refactor.** `📓️fable-coordinator-repairs.md` (21:05)
  recorded `🛢️db/🗜️compact/🦀️.rs:1249 E0308 expected &WalWriterPermit, found &ArtifactId`, and the
  coordinator reported the Sol fleet was still editing those files at 23:15. W1 in §5 compiles `db`
  as part of the `os-hub` graph and is this lane's own direct measurement of whether that is still
  true; its result is recorded verbatim in §5.
- **No peer file was reverted or overwritten.** This lane touched only `📜️script.ts` (a new
  append-only `//#region 🔖️SpaceJourney` plus one router registration line), `📋️project.json` (two new
  targets), `.vscode/🧩️launch.seed.jsonc` (two new entries in the existing `4_gate` group), the new
  fixture folder, and this report. `.vscode/launch.json` was regenerated with the plugin-registry
  generator, never hand-edited; that regeneration also picks up seed entries other lanes added, which
  is the intended behaviour of the generator.
- **No `📋️master-plan.md` or `✅️acceptance-matrix.md` edit**, no ticket open/close/reopen, no
  git-modifying command, and no other lane's `🗑️generated` subfolder was read or written.
- **Repo MCP** was unreachable for the whole lane (`repo` and `semio` MCP servers both timed out at
  connect), so the ticket folder was managed directly on disk, as the project memory prescribes.
