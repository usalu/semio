# Fable Explore — Residual Graph Refresh (as of 2026-09-05 ~23:30)

Lane `fable-explore-residual-graph-refresh`. Read-only: no cargo/nx/bun run, no edit, no
subagent. Every **[verified]** tag below means I read the exact current source line myself in
this exploration, today, at ~23:30. Every **[report]** tag means I am relaying another lane's own
claim from its report file, not independently re-executed. Every **[inference]** tag is my own
synthesis/judgment from combining reports, not a direct observation of either kind. Line numbers
move under concurrent editing; treat them as "true at ~23:30", not eternal.

Inputs reconciled: `📓️terra-three-pillar-current-residual-execution-graph.md` (01:40, the seed
graph), `📓️fable-explore-two-user-journey-readiness.md` (21:49) and
`📓️fable-explore-inference-readiness-path.md` (21:51), every `📓️sol-*`, `📓️root-*`, `📓️terra-*`,
`📓️fable-*` file modified after 12:00 today (84 files — full inventory, not a sample), the
`✅️acceptance-matrix.md` tail (21:30) and `📋️master-plan.md` head/tail, plus the sibling ticket
`26/09/05/S-END-TO-END`'s `📋️plan.md` coordinator log (through 23:20) and its newest
`📓️opus-space-studio-migration.md` (21:40, summarized in the coordinator log at 21:45).

## 0. Headline verdict — what moved since 01:40

The 01:40 graph's verdict ("none of the three user journeys is yet end-to-end executable") is
**still true**, but every P0 node has moved, and two new systemic blockers (the WAL-writer-permit
refactor, and a live plugin-identity rename) now dominate the critical path more than the original
graph's P0-A/B did. Concretely:

- **P0-C (execution-target lease)** went from "narrow packet, not started" to **schema+browser+hub
  routes fully landed** (`📓️fable-execution-target-lease.md`), with its 3 native laws **written,
  registered, and now actually unblocked** (the plugin-host compile blocker that prevented them
  from ever running was fixed at 18:10 — see §5). This is the single highest-leverage "run it now"
  item in the whole ticket.
- **P0-D (two-user directory journey)** is reconciled by `fable-explore-two-user-journey-readiness`
  as "fixed at every RED verdict the 01:40/03:26 graphs listed, at source, but zero native/process
  reruns since." Invite-redemption atomicity, scoped-socket revocation, presence-lease TTL, and the
  directory-event-page ingestion path **[report, cross-checked by that lane against source]** are
  all now implemented — this closes 4 of the 01:40 graph's "Still RED" rows from §"Reconciled status
  of previous findings" **at source**, none natively re-verified.
- **P1-C/D (AI Map proposal → approval)** moved the furthest of all three pillars: `HubState` now
  owns `InferenceJobLedgerV1`, four authenticated routes exist and are wired
  (`inference/gis-map/jobs{,/events,/cancel,/approval}`), `features.inference` gates them
  **[verified]** (`🚀️bin.rs:1978,2002,6335-6338,6596`). The 01:40 graph's "Still RED... no route" for
  this pillar is **closed at source**. What remains RED, unchanged from 01:40 in substance: the
  actual *typed durable publication* on approval. `UnavailableGisMapApprovalCommitterV1` is still
  the sole registered committer **[verified]**, `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6591`, so every
  otherwise-valid approval still terminates `503 approval.commit-unavailable`.
- **A new, larger blocker replaced P0-B/P0-A as the dominant cross-cutting risk**: the Sol
  WAL-writer-permit migration (~15 packets, all source-complete, essentially zero of them re-run as
  one coherent native group since the API broke) and, in Pillar 1, a live plugin-identity rename
  (`space` plugin: `s`→`space`) that is mid-flight and has already broken two Wave-2 rebuilds today.
  Both are detailed in §5.
- **P0-A (public MemberFactory Open)** and **P0-B (trusted stdio+gis materialize/candidate/process)**
  are **unchanged in kind** from 01:40: still source/native-prerequisite work with no candidate or
  process run completed by anyone today. `sol-hub-headless-native-artifact-provider.md` (19:22)
  advanced the *architecture* around P0-B (native codec provider is now an optional, feature-gated
  port so headless Hub no longer compiles Stdio/GIS/VCS), which is real progress but does not
  advance the trusted-bundle materialization itself, which `fable-explore-inference-readiness-path`
  independently confirms **"has in fact never completed in any of today's reports."**

## 1. Residual slice table

Legend for **Evidence level**: `source` = written and internally consistent, no build attempted;
`focused laws` = a package/module-scoped Bun/AJV oracle ran green; `native laws` = a real `cargo
test`/`cargo check` executable ran the registered Rust selectors; `process` = a real multi-process
journey (two authenticated sessions, restart, etc.) ran; `browser` = a real headless-Chromium/JCO
run. **Blocked on compile evidence only** = yes means the *only* thing stopping promotion to
native/process is getting a clean, current-tree compile through the shared Cargo lock — no design
work remains.

| # | Residual slice | Owner (lane) | Evidence level | Exact blocker | Compile-evidence-only? |
|---|---|---|---|---|---|
| 1 | WAL writer-permit non-cloneable authority across Memory/FS/SQLite/CLI/compaction/cluster | Sol (multiple packets, this ticket) | native laws (per-primitive groups: writer7/writer8/shutdown3/deferred-wake3/maintenance11/compaction2 all individually green) | The *combined* 26-30-law `wal-writer-authority-native-check` group has never passed as one run since the API broke; Postgres/Neo4j remote guards are source-only (no live DB in this env); `db/compact.rs` had an `E0308 &WalWriterPermit vs &ArtifactId` mismatch at 21:05 that a re-read at 21:40 no longer confirms — current compile state genuinely unknown | **No** — remote-guard laws need infra (`SEMIO_TEST_POSTGRES_URL`), and several Terra P0/P1 design gaps remain open (see §5) |
| 2 | Database concurrent first-document-mount race | Sol (`📓️sol-database-document-mount-single-flight.md`, 23:20 — **newest file in the whole ticket**) | source-complete, 3 native laws registered, unrun | Native run pending the root-owned warm target | **Yes** |
| 3 | Actor-owned WAL compaction | Sol (`📓️sol-compaction-actor-owned-wal.md`) | source-complete; native RED once on a `Vec` over-allocation bug, now source-fixed | Rerun pending; separately, VCS Store lifecycle abort on the live-actor law forwarded to `/root/complete_wgpu_home` (a *different* agent outside this ticket's fleet) | **Partially** — the Vec fix is compile-only, the VCS lifecycle fix is design work owned elsewhere |
| 4 | GIS Map durable three-member group commit (parent+drawing+value) | **Unowned as a whole** — Sol built the Store-side codec/coordinator (`📓️sol-map-durable-group-decision-codec.md`, 12/12 native laws green), but no DB-side journal sink or per-document actor exists | Store-contract native laws green; zero DB integration | No `DurableOwnedGroupJournalCommitV1` impl, no per-`DocumentScope` typed actor owning the 3 Stores; `terra-gis-map-approval-committer-ownership-bridge` (22:49, newest on this topic) specifies exactly what's missing | **No** — this is unstarted design+implementation, not a compile block |
| 5 | GIS approval → real committer wiring | Unowned (route/ledger side is Fable's `fable-ai-map-proposal`, already closed) | route/schema/ledger native-pending; committer is a stub by design | `UnavailableGisMapApprovalCommitterV1` still registered **[verified]**; needs #4 first | **No** |
| 6 | Presence normalization | Sol/Root (`📓️sol-presence-peer-bounded-exact-codec.md`, `📓️root-hub-presence-normalization.md`) | source green (17 vectors); 2 of 6 native laws have a known test-fixture bug | `terra-hub-presence-plan-fixture-headless-current-audit` (19:48) identifies the exact fix: install a descriptor-bound, readiness=true catalog *before* the plan helper runs, in two test setups | **Yes** — this is a ~10-line test-helper fix, not a design change |
| 7 | Admin removal + presence + execution-target SQLite recovery (composed journey) | Root (actively iterating, `📓️root-hub-presence-admin-native-current.md`) | native RED at first member-socket Welcome, cause undiagnosed | Root is mid-debug; `terra-hub-admin-removal-presence-target-sqlite-recovery-packet` flags 2 separate test-design gaps (consumed-grant reuse, missing durable-authority direct checks) on top of the runtime bug | **No** — live debugging in progress |
| 8 | Directory command receipt transport | Fable (`📓️fable-directory-command-receipt.md`) | TS 267/267, 90 AJV assertions; **Rust never compiled once** | Needs a clean `cargo check -p semio-hub --bin os-hub --tests --features sqlite` | **Yes** |
| 9 | Space administration page/pane | Fable (`📓️fable-space-administration.md`) | TS 259/259 ×3, kernel compiled 0 errors with new schema/client; hub route/WGPU laws never run | 4 hub route laws, 2 WGPU laws unrun | **Yes** |
| 10 | Execution-target lease | Fable (`📓️fable-execution-target-lease.md`) | schema+browser fully green; 3 native laws registered **and now unblocked** (see §5, blocker resolved 18:10) | Simply needs to be run | **Yes — and it is the best "run it now" candidate in the ticket** |
| 11 | GIS Map inference UI port (15th Shell command, kernel effect, ShellHost panel) | Fable (`📓️fable-gis-map-inference-ui-port.md`) | kernel 0 errors, plugin-host 0 errors (56 min build), browser 9/9; GIS/WGPU crate tests + wasm reactor arm unverified | Needs a wasm32-wasip2 plugin rebuild (shared with Pillar-1 Wave 2, see §7) | **Partially** — needs a real wasm build, not just a native check |
| 12 | MCP↔hub inference bridge | Fable (`📓️fable-mcp-inference-bridge.md`) | `--process` 6/6 against the real binary, end-to-end 12/12 with 26-tool census, `--lib` 0 errors | No route callable against a live hub (`features.inference` is `false` without a trusted profile — depends on P0-B); unrelated stack-overflow in `artifact::quick` blocks the full `--lib` suite | **No** (blocked on P0-B, a design/materialization dependency, not a compile issue) |
| 13 | VCS second native openable provider | Fable (`📓️fable-vcs-native-provider.md`) | 2 Bun oracles green; Rust receipt laws unrun; descriptor pair stale | Needs the registered `describe` wasm build (shared `target/debug` lock contention, 5h+ waits recorded) | **Yes**, modulo lock contention |
| 14 | Closed browser component factory → actor bundle → GIS executor handoff | Root (`📓️root-closed-browser-component-factory.md`) + Terra audits (`📓️terra-browser-closed-actor-bundle-review.md` 22:48, `📓️terra-browser-per-activation-wasi-jco-contract.md` 23:13, `📓️terra-browser-wasi-host16-current-audit.md` 23:23, `📓️terra-verified-gis-editor-executor-handoff-current-packet.md` 21:33, `📓️terra-jco-closed-gis-browser-factory-current-audit.md` 22:18) | Node-hosted JCO/Wasm proofs green (host lifecycle, actor factory, WASI-fence, stream bounds); zero browser-Worker or real GIS-guest runs | No first-party per-activation Preview2 resource module yet; no browser-catalogued bundle descriptor/route; `renderer-unavailable` remains the correct current terminal | **No** — substantial, well-specified design work remains (Root is the active, sole owner; this is the single most-audited thread today, 5 Terra reports in 90 minutes) |
| 15 | Lossless dynamic Pack integer codec (`TAG_UINT`/`TAG_INT`) | **Unowned** | Bug identified with an ordered 5-step repair plan (`📓️terra-pack-dynamic-integer-lossless-current-packet.md`, `📓️terra-artifact-replay-numeric-shape-current-risk.md`) | No implementation started | **No** — real correctness gap, self-contained package, nobody has claimed it |
| 16 | Filesystem WAL directory-entry durability (power-loss barriers) | Unowned, audited only (`📓️terra-wal-fs-directory-entry-durability-current-packet.md`) | source review only | Needs 4 new platform laws (macOS/Linux/Windows) plus fault-injection harness | **No**, and collides with active Sol WAL-storage edits (see §5) |
| 17 | Postgres/Neo4j WAL remote writer guards | Unowned, audited (`📓️terra-wal-writer-guard-sqlite-postgres-neo4j-current-packet.md`) | source review only | No live Postgres/Neo4j in this environment; laws are specified but cannot execute | **No** — environment gap, not a code gap |
| 18 | WAL retained transaction gate / committed cursor fairness (fuel/yield bugs) | Unowned, audited (`📓️terra-wal-retained-transaction-gate-current-audit.md`) | source review, 3 concrete P1 findings with exact law specs | None — ready to implement | **No**, but shares files with active Sol WAL work (medium collision risk) |
| 19 | Pillar 1 — OS frontend catalog (all 59 plugins/artifacts openable in the React shell) | **Sibling ticket `26/09/05/S-END-TO-END`**, separate Fable/Opus/Sonnet fleet | Wave 1 fully done (catalog-smoke harness, descriptor producer, stdio census); Wave 2 mid-flight | See §7 — currently blocked on a live plugin-identity rename (`space` plugin) | **No** — active design/rename work, not a compile wait |
| 20 | WGPU wasm/native parity tier (Wave 3 of Pillar 1) | Sibling ticket, not yet dispatched | `📓️explore-wgpu-tier-readiness.md` (19:05) is an audit only | Depends entirely on #19 landing first; `check-frame-worker` RED, wgpu `lint` RED (3 raw colour literals), ship `BuildScript` points at a nonexistent path | **No** |

## 2. Refreshed dependency graph (same P0/P1 notation as the 01:40 terra graph)

```text
P0-A  public MemberFactory Open (UNCHANGED — still an external prerequisite; no lane
       touched it today; native acceptance still blocked by unrelated compilation)
       │
P0-B  fresh trusted stdio+GIS materialize → candidate readiness → atomic current publication
       (ARCHITECTURE MOVED: sol-hub-headless-native-artifact-provider made the native codec
       provider an optional/feature-gated port — headless Hub no longer needs Stdio/GIS/VCS
       compiled at all. The bundle MATERIALIZATION ITSELF is UNCHANGED — still never
       completed by any lane today; a cold wasm32-wasip2 build of this closure is
       independently estimated at "on the order of hours" by fable-explore-inference-readiness-path)
       │                                                  │
       │                                                  └─ no client execution claim (UNCHANGED)
       ▼
P0-C  immutable execution-target lease + byte verification  ***MOVED FROM "NOT STARTED" TO
       "SCHEMA+BROWSER+HUB DONE, NATIVE LAWS READY TO RUN NOW"*** (fable-execution-target-lease;
       blocker that prevented native laws from ever running — plugin-host non-exhaustive match —
       RESOLVED at 18:10; not yet re-run since)
       │
       ├─ P0-D  two-user GIS Map read-only open/reconnect/revocation journey
       │          ***MOVED: every named RED verdict fixed AT SOURCE per fable-explore-two-user-
       │          journey-readiness; zero native/process reruns since the fixes landed***
       │          │
       │          ├─ P0-E  retained Home/Space directory event page  ***DONE AT SOURCE***
       │          │          (applyDirectoryEventPageBootstrapV1 wired into ShellHost;
       │          │          fable-directory-command-receipt closed the idempotency half)
       │          └─ P0-F  request-id DirectoryCommand + Home/Space admin outcome UI
       │                     ***DONE AT SOURCE*** (fable-directory-command-receipt +
       │                     fable-space-administration); Rust NEVER COMPILED ONCE
       │
       └─ P1-A  first Flow provider/open target + Flow addWidget retained child factory
                  (UNCHANGED — explicitly deferred by fable-explore-two-user-journey-readiness
                  as step 7, "deliberately last")
                  │
                  └─ P1-B  atomic parent/child publication + global composition history routing
                            (UNCHANGED for Flow; but the ADJACENT GIS three-member group-commit
                            problem — slice #4 in §1 — is the SAME shaped problem and is now the
                            critical path for P1-D below, via terra-gis-map-approval-committer-
                            ownership-bridge)

P0-D + P0-C ───► P1-C  deterministic, private Map proposal job
                           ***MOVED FROM "DISCONNECTED ON BOTH SIDES" TO "ROUTES LIVE, LEDGER
                           CONSTRUCTED IN HubState, PREVIEW PROJECTION IMPLEMENTED"***
                           (fable-ai-map-proposal, sol-inference-preview-scope-acceptance)
                           │
                           └─ P1-D  explicit approval → atomic typed GIS publication → B observes
                                     ***STILL BLOCKED, BUT THE BLOCKER NARROWED***: was "no route,
                                     no ledger, no committer" at 01:40; is now exactly ONE missing
                                     piece — a real GisMapApprovalCommitterV1 (slice #4/#5 above).
                                     terra-gis-map-approval-committer-ownership-bridge gives 5
                                     executable acceptance laws for it.

Parallel reliability P1: invite consume transaction + presence lease/expiry
       ***BOTH CLAIMED FIXED AT SOURCE*** by fable-explore-two-user-journey-readiness
       (invite: conditional UPDATE with accepted_at/accepted_event_id/revoked_at/expires_at guard;
       presence: PRESENCE_LEASE_TTL_MS=15_000 + socket_live_id fencing) — native/process unverified.

*** NEW NODES not in the 01:40 graph, now on the critical path ***

P0-G  WAL writer-permit non-cloneable authority (Sol, ~15 packets) — gates EVERY native law in
       P0-D/E/F/P1-C/D above, because they all compile through `semio-hub --features sqlite`,
       which transitively depends on `framework-os-kernel`'s db module where the permit migration
       lives. This is why "Rust never compiled once" recurs across almost every Fable slice above.

P0-H  Plugin identity rename (`space` plugin `s`→`space`) — Pillar-1-internal, but blocks the same
       wasm rebuild that P1-C's UI port (#11) and VCS provider (#13) need, because all three draw
       on the same `target-s-e2e*` wasm32-wasip2 build lane in the sibling ticket.
```

## 3. Top five unowned packets an Opus implementer could start right now

Ranked by (value × readiness) ÷ collision risk. All five avoid the two systemic blockers (§5) or
are small enough to route around them.

### 1. Presence-normalization test-fixture repair (tiny, highest confidence)

**Files:** `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` (test-only helper near lines 7984–8000 and the two
failing law bodies at ~9983–9985 / ~10061–10065). **Size:** ~10–30 lines, test-only.
**Prerequisites:** none. **Registered gate to extend:** `os-hub:presence-normalization-check --
--native`.

`terra-hub-presence-plan-fixture-headless-current-audit` (19:48) diagnosed exactly why the reported
native RED (`exact-cargo-laws-UkC7ve/00`) failed before any presence logic ran: the test installs a
descriptor-bound catalog *while* `open_plan` readiness is still `false`, so `issue_document_open_plan`
fails closed on `503/CatalogUnavailable` before the socket/lease code is ever exercised. The fix is a
paired test-only helper that sets readiness only after installing the catalog — explicitly *not* a
change to `test_state`'s default `openPlan: false` (that default is itself load-bearing for other fail-
closed route tests) and *not* a synthesized plan/grant. No later report claims this fix has landed. This
one is close to risk-free: it touches only test scaffolding in a file whose production code paths are
otherwise stable, and unblocks a 6-law native gate that has been "source green, native pending" all day.

### 2. Run the execution-target-lease native laws to completion

**Files:** none expected to change — this is (mostly) execution, not code. **Size:** near-zero code;
possibly a few lines if the run surfaces a stale selector. **Prerequisites:** a warm/available Cargo
target and the plugin-host fix already landed (confirmed **[verified]**: the `Effect::
RequestInferenceProposal` match arm exists at `🔌️plugin/🖥️host/📥️imports/🦀️.rs:544`).
**Registered gate:** `os-hub:execution-target-lease-check -- --native`.

This was **[report]** blocked all day by a compile error in a completely unrelated crate
(`semio-framework-plugin-host`'s non-exhaustive match). `fable-explore-two-user-journey-readiness`
independently confirmed at ~21:40 that the match arm now exists in source. Nobody has re-run this
gate since. If it passes clean, it closes acceptance-matrix's "P0-C execution-target lease" row
outright; if it doesn't, the diagnostic is cheap and immediately actionable by whoever owns the
next fix.

### 3. Narrow, parent-only `GisMapApprovalCommitterV1`

**Files:** `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` (new impl, e.g.
`SingleDocumentGisMapApprovalCommitterV1`); no expected change to
`🌎️hub/💡️inference/🧾️wal/🦀️.rs` (`InferenceWalVerifierV1::verify` is already generic).
**Size:** ~100–200 lines (wraps the existing single-document `ArtifactEngine`/`ArtifactWal` submit
path). **Prerequisites:** none beyond what's already landed. **Registered gate to extend:**
`gis-map-proposal-check`'s route-law block in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`.

This is `fable-explore-inference-readiness-path`'s own dependency-ordered item #3, rated "low-medium"
collision risk by that report. It deliberately does **not** attempt the full three-member
parent+drawing+value group commit (slice #4 in §1, which needs the durable-group DB integration that
is genuinely unowned and large) — it wires *only* the parent `CreateRegion` mutation through the
existing per-document `ArtifactWal`, which is real, single-document durability, just not the full
Map/drawing/value atomicity the product ultimately needs. It gets the approval pipeline off `503` and
onto a real committed effect for the first time, which is a meaningful, demoable milestone even before
the full group-commit lands. Re-check `🏃️runtime/🦀️.rs` for fresh edits before starting — it was
"live" as of `fable-ai-map-proposal.md` (21:59) but that lane's own report is now hours old and shows
no further pending edits.

### 4. Lossless dynamic Pack integer repair (`TAG_UINT`/`TAG_INT`)

**Files:** the dynamic `Shape::Value` codec in `🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs` plus its
TypeScript twin and schema; `Cargo.toml` comment fix noted by the audit. **Size:** medium-large —
5 ordered steps (schema+law, Rust grammar change, `PackInteger`+BigInt LEB128+browser corpus, retained
scene decoding upgrade, descriptor-JSON safe-projection conversion). **Prerequisites:** none; this is
a self-contained package. **Registered gate to extend:** a new exact-variant native law, per
`terra-pack-dynamic-integer-lossless-current-packet.md`'s own ordering.

This is a real correctness bug (the current encoder round-trips every dynamic number through `f64`,
so `u64`/`i64` values beyond `2^53` silently corrupt on replay — independently reproduced by
`terra-artifact-replay-numeric-shape-current-risk.md`), fully diagnosed with an exact implementation
order, and touches no file any active Sol/Fable/Terra lane is currently editing. The honest caveat:
"no compatibility path" means both Rust and TypeScript sides land together, so it's not trivially
splittable across two people, and it does affect the Map durable-group decision codec's own numeric
handling (slice #4), so whoever picks up #3/#4 above should be aware this fix, once landed, will
change the acyclic hash inputs those codecs already pinned.

### 5. WAL retained-transaction-gate fairness/fuel fixes (three P1s, one file)

**Files:** `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs` (the committed-cursor/replay-gate
region, not the writer-permit region). **Size:** small — three isolated bugs each with a concrete law
already specified: (a) one-unit-fuel cursor can never open a segment, (b) a long run of physical
commits has no fairness yield, (c) poison/close coverage doesn't exercise a live decoded record.
**Prerequisites:** none. **Registered gate:** the existing `wal-committed-transactions-native-check`
group (add 3 laws).

Flagged with an honest caveat: this file is in the same module family (`db/📝️wal/🦀️.rs`) that Sol's
writer-permit migration is actively rewriting (`sol-wal-open-retained-rejection-design.md`,
`sol-wal-writer-fs-sqlite-integration.md`). The specific regions this fix touches
(`prepare_transaction_step`, `wal_next_page_frame`, `wal_committed_cursor_unfinished_borrow_poison_and_
cancelled_close`) are the *replay/read* path, while Sol's writer-permit work is the *write/acquire*
path — they are structurally distinct, but both land in the same 2,700+ line file, so **re-read the
region immediately before editing** and expect a rebase if Sol's migration lands first. If collision
risk is a concern, slice #4 above is the safer alternative of similar size.

## 4. Systemic blockers

### WAL writer-permit refactor churn (Sol, this ticket)

**Owner:** Sol (implementer), Terra (auditor) — both active fleet roles in this ticket, not an
individual named lane; ~15 separate `📓️sol-wal-*`/`📓️sol-database-*`/`📓️sol-worker-*` reports today,
cross-audited by ~20 `📓️terra-wal-*`/`📓️terra-writer-*`/`📓️terra-compaction-*` reports. **What it
is:** every mutating WAL operation (`create`, `append`, `sync`, `seal`, `truncate_tail`,
`delete_segment`) is being migrated from an unguarded `WalStorage` trait method to one gated by a
non-cloneable `WalWriterPermit`, across Memory/FS/SQLite/CLI/compaction/cluster/testkit, to close a
real production data-corruption race (two stale openers can append the same repair suffix). **Why it
blocks nearly everything above:** this module sits underneath `framework-os-kernel`'s `db` tree, which
`semio-hub --features sqlite` depends on transitively; almost every "Rust never compiled once" line in
§1 traces back to this file family being mid-rewrite (confirmed concretely once today at
`db/🗜️compact/🦀️.rs:1249`, though a re-check 35 minutes later no longer showed that exact pattern —
the tree moves faster than any one compile). **Evidence that would show it's resolved:** a single
green run of the *combined* `wal-writer-authority-native-check` group (all ~26-30 laws, not the
per-primitive subsets that have passed individually) followed by a clean
`cargo check -p semio-hub --bin os-hub --tests --all-features` with no diagnostic in any db/wal/writer
file. Neither has happened today.

### Trusted stdio+GIS bundle materialization never completes (P0-B)

**Owner:** unclear/rotating — referenced by Sol, Root, and multiple Fable lanes, but no single lane
owns *finishing* it; `sol-hub-headless-native-artifact-provider` owns the architecture around it.
**What it is:** producing the immutable, atomically-published trusted catalog bundle (26 stdio + 2 GIS
+ 1 VCS receipts) that `OS_HUB_TRUSTED_CATALOG_BUNDLE`/`PROFILE` point at, which is the only thing
that can ever flip `features.inference` to `true` in a real deployment. **Why it's stuck:** it requires
a cold, sccache-bypassed `wasm32-wasip2` build of the whole stdio+gis closure plus a full native
`--bin os-hub` build in the same gate invocation — independently estimated by
`fable-explore-inference-readiness-path` as "on the order of hours," and it has literally never
completed end-to-end in any report from today. **Evidence that would show it's resolved:** a terminal
receipt from `os-hub:trusted-stdio-gis-bundle-check -- --process` showing candidate readiness plus a
persistent `current` pointer surviving restart.

### Descriptor re-emission debt for ~40 plugins/extensions (Pillar 1)

**Owner:** sibling ticket `26/09/05/S-END-TO-END`, Wave 2. **What it is:** 21 of the 59 registry rows
have no owner descriptor pair at all, and the ones that do exist are frequently stale relative to
current source (confirmed today: the auto-committed rename of the `space` plugin's Cargo identity
broke `describeBuiltPlugin` twice in one evening — see next item — and the stdio emoji-uniqueness
rename left stale paths in at least 3 other lanes' fixtures today, e.g. `fable-vcs-native-provider`'s
`🧊️obj`→`🗽️obj` path mismatch). **Evidence it's resolved:** `plugin-registry:check` reaching 59/59
valid pairs (currently far short — Lane B's own report records 1/59 valid at the last full check).

### Live plugin-identity rename (`space` plugin `s`→`space`) mid-flight

**Owner:** sibling ticket, Lane K (`space-plugin-identity`), dispatched 21:55, **still in progress**
as of 23:20 (interrupted by an Opus session-limit reset at ~23:10, resumed 23:18). **What it is:**
the space plugin's Cargo component identity was auto-committed to `space` at 03:53 while every other
authority (registry, deployment catalog, host config) still says `s`; the fix direction (decided
21:55) is to make `space` canonical everywhere except the playground/launch variant name. **Current
state, verified just now:** `✏️s/🔌️plugins/🪐️space/🦀️.rs:809,812` still reads
`builder("s")`/`package_id("semio:s")` — **[verified]** — so the rename has *not yet landed in
source* despite being decided 90 minutes ago; the census work is what got interrupted by the Opus
limit. **Why it matters here:** the same `wasm32-wasip2` rebuild lane (`target-s-e2e-wasm`) that this
rename gates is also what P1-C's GIS-inference-UI-port (slice #11) and the VCS provider (slice #13)
need for their own wasm-side verification — a fourth pillar-2 lane starting a wasm rebuild today would
very likely collide with Lane K's in-flight rename. **Evidence it's resolved:** `builder("space")` /
`package_id("semio:space")` in source, a green `describeBuiltPlugin` for the space crate, and
`plugin-registry:check` no longer flagging an identity mismatch for that row.

## 5. Cross-check: no duplicated work found between Root's execution-target-relay and Fable's execution-target-lease

Worth flagging explicitly since it looked, from filenames alone, like two lanes might have built the
same thing twice. **[verified]**: `os-hub:execution-target-relay-check` (Root,
`📓️root-execution-target-relay.md`) and `os-hub:execution-target-lease-check` (Fable,
`📓️fable-execution-target-lease.md`) are two *different* registered gate scripts in
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`, but they **share one law by name**
(`execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body`,
lines 2750 and 3801). This means Root built the byte-transport/relay layer (deadlines, resource caps,
`Content-Length`, cancellation) and Fable built the schema+lease+browser layer on top of it, and both
converged on the same underlying route law rather than reimplementing it — this is layered
contribution, not duplication. No action needed here; noted so the coordinator doesn't dispatch a
"de-duplicate execution-target work" packet that isn't needed.

## 6. Pillar 1 (OS frontend, all plugins/artifacts) — sibling-ticket detail

Owned entirely by `26/09/05/S-END-TO-END`, a separate Fable/Opus/Sonnet fleet, not this ticket's
Sol/Terra/Fable roster. Summarized here only because the prompt asked for all three pillars; **do not
dispatch Pillar-1 work from this ticket's coordinator** without checking that plan's own coordinator
log first (it is current through 23:20 today).

- **Wave 1 — done.** Catalog-smoke Playwright harness (`verify catalog`), descriptor-producer
  contract + registry fail-closed check, stdio native census.
- **Wave 2 — mid-flight, boot now progresses past the fatal shard-worker bug** (root cause: an ASCII
  transliteration of the shard-worker route mismatched the actual published path — fixed, lane G).
  Router now excludes the demonstrator plugin's bad descriptor gracefully instead of dying (lane H).
  All 45 `norm` app actions migrated off `BatchOnlyPendingRewrite` (lane D). Studio/home/space-index
  command classification substantially repaired (lane J: 15/40, 18/18, 14/14 respectively). **Current
  single blocker for the whole wave: the `space` plugin identity rename (§5, "live plugin-identity
  rename")** — three consecutive incremental rebuilds today failed `describeBuiltPlugin` on identity
  mismatches stemming from the same 03:53 auto-commit, each fix exposing the next layer (builder id →
  package_id → artifact-kind-ownership grammar). Lane K is mid-census on the final, most invasive fix
  (rename to `space` everywhere except the playground variant name).
- **Wave 3 (WGPU wasm/native parity) — not dispatched.** `explore-wgpu-tier-readiness` (19:05) is
  audit-only: no trunk artifact exists, `check-frame-worker` is RED on a stale `frame-worker.js`,
  `lint` is RED on 3 raw colour literals, and the ship `BuildScript` points at a nonexistent path.
  This wave shares the plugin cache, shard worker, and JSPI requirement with the React tier and with
  this ticket's browser-actor-bundle work (§1 slice #14) — coordinate before starting either.

## 7. Explicit inference-versus-verified summary

**Verified by direct source read in this exploration (~23:30):** hub inference routes and
`inference_ready`/`features.inference` wiring (`🚀️bin.rs:1978,2002,6335-6338,6591,6596`);
`UnavailableGisMapApprovalCommitterV1` still the sole registered committer; GATEWAY_TOOL_NAMES = 26
in the MCP module; trusted-catalog loader's `open_target_count`/`load`/`load_selected` shape; the WAL
writer-permit module exists at `🧰️framework/…/🔐️writer/🦀️.rs`; the 15th GIS command
(`propose-bounds-region`) is present and counted in `every_command()`/`WIRE_KEYWORDS`; the space
plugin's Cargo identity is still `builder("s")`/`package_id("semio:s")` (Lane K's rename has not yet
landed); `db/compact/🦀️.rs` does not currently show the specific `E0308` line a 21:05 report flagged
(consistent with the tree having moved, not proof the crate compiles); the two
`execution-target-*-check` gate scripts are distinct but share one law (no duplication).

**Verified by report + cross-check (not independently re-executed by me):** every "N checks green" /
"N tests passed" figure in §1's table is taken from the owning lane's own stated command output.
`fable-explore-two-user-journey-readiness` and `fable-explore-inference-readiness-path` did their own
line-level source cross-checks at ~21:40-21:51 for the claims they made; I did not re-derive those a
third time except where the table above adds a fresh `[verified]` tag.

**Inference/my own synthesis, not directly observed by anyone:** the "P0-G/P0-H new critical-path
nodes" framing in §2; the risk ratings and file-collision judgments in §3's top-5 list; the claim that
slices #3 and #4 (approval committer, durable group) are the *same shaped problem* as Flow's P1-B; the
overall verdict that the WAL-writer-permit churn is now the single largest blocker (this is a
synthesis across ~35 Sol/Terra reports, not a number computed by any one of them).

**Not re-verified, explicitly unknown right now:** whether `cargo check -p semio-hub --bin os-hub
--all-features` currently compiles end-to-end. The most recent concrete signal (21:05, a real
`E0308`) says no; a re-read at the exact same line 35 minutes later no longer shows that pattern, but
nothing later in the ticket reports a fresh green compile. Anyone picking up items #2, #6, #8, or #9
from §1 should expect to hit *some* compile error first and should not assume the tree is clean.
