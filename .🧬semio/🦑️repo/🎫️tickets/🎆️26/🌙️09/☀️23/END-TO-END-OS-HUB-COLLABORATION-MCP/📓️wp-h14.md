# WP-H14 — Hub Performance + Operations (successor of H12)

Slice: H14 (session 14, 2026-09-27 18:3x). Coordinator = main chat. Rules: `📓️session-14-preamble.md`. Ports: hubs 8160–8169,
serves 6660–6669 (keeps H12's hub 8161, pid 79070). Private cargo target: `.tmp-ticket/wp-h14/target` (build-dir
`build-fleet-b`, native lane only). Captures: `wp-h14/generated/` (expendable). Durable data/logs: `.🧬semio/🌐hub/s14-h14-*`.
Handovers: [📓️wp-h12.md](📓️wp-h12.md), [📓️wp-h10.md](📓️wp-h10.md), [📓️wp-db1.md](📓️wp-db1.md),
[📓️acceptance-s13.md](📓️acceptance-s13.md) §2. Neighbours: H13 (hub correctness/security, fuzz row 2.13, interpreter
cancellation), R10 (launch generator, harness productization), W4 (chain, 7800).

### Session 14b

Successor agent, 2026-09-28 12:1x (predecessor cut ~20:45 by the usage limit, mid-way through the P1 db credit fix).
Chain launched 12:02:46 (GUEST FREEZE on). `semio-framework-os-kernel-db` has exactly one normal-edge dependent, `semio-hub`
(`cargo tree -i … --workspace -e normal --target all`), so it is chain-compiled only through os-hub (hub prewarm + the
post-publish os-hub build), never through a guest / rebuild-all.

| # | Item | Status |
|---|------|--------|
| 0 | Reconcile the predecessor's in-flight db edit | predecessor's WAL + artifact hunks (20:35–20:41) were auto-committed at 21:54 (`5bcb2da23da`) **red**: `PlannedEntries` undefined + replay still called the removed `apply_one` → chain `hub-prewarm rc=1` 12:03 (`s14-w4-logs/final-hub-prewarm.txt` l.222–676; predecessor's own `lane-hub-1.txt` 21:10 saw the same 4 errors). **Finished forward 12:2x**: `struct PlannedEntries` + replay `plan_one → commit_one`; native check queued (priority stamp) → `generated/lane-db-1.txt` |
| 1 | P1 db credit: a declared-legal batch never refused for capacity | **DONE, native green** (coordinator: kernel-db is hub-closure-only → hub rule): 4 capacity refusals + the replay trap removed, 1 P0 found by the max-batch law and fixed (WAL reader held 64 frames → a committed batch of ≥ 64 envelopes left its document unopenable); db laws 85/85 + 210/210, hub check EXIT 0, hub bin laws 2/2 — see log + landing |
| 2 | Trusted-catalog fixtures → channel 19 + bundle check green (coordinator 13:0x–13:5x; H14 = trusted-catalog owner this session) | **DONE**: 3 fixtures bumped; stdio-gis generation re-derived twice by the hub script's own encoding (→ `bda0b90f…` after ST2's svg pin fix; relayed); compiled-dependencies rawCases re-derived by the Pack codec (15 → 19); census sees Pack-hex + numeric `executionProtocol` versions, 2 fixtures registered, two-author fixture 17 → 19 via `generate`: `channel-version check` **0 findings**; resolver: `trustedBootstrapResolveClosure` (foreign/self/cycle/linked-profile edge) at the root; 3 stale source anchors aligned; `trusted-stdio-gis-bundle-check --source` **rc 0**; hub TS typecheck rc 0 |
| 3 | Rust hub `trusted_catalog` laws 54/57 → root-cause the 3 reds | **DONE**: all three panicked at `plugin().expect(..)` (unit l.940/l.1895) = stdio/gis/vcs assembly failing on the peer's svg `📡️.protocol.semio` change with a stale pinned SHA-256 (fixed by ST2 13:5x); rerun **3/3 PASS** 14:26 (`generated/lane-tc-1.txt`) |
| 4 | Item 7: remove the db artifact "deprecated-in-spirit extension seam" | **DONE 14:28**: `wp-h14/h14-remove-authz-seam.py` applied inside one lane hold (auto-revert armed): `AuthzHook`, `AllowAll`, `SecurityAuthzHook`, `Database::open_with_authz` and the generic `A` of `ArtifactEngineConfig`/`ArtifactEngine`/runner/`Database` removed (9 files); `submit` authorizes through `security` only; db `--lib --tests` EXIT 0, semio-hub `--lib --bins --tests` EXIT 0, db laws **210/210** (`generated/lane-seam-1.txt`); root `📜️script.ts` P1w caller census drops `open_with_authz` (its 16 other findings on the real engine source are pre-existing, identical before/after); TS self-test PASS |
| 5 | Predecessor's unrun hub laws (residency footprint, observability, pair-content Rust twin) | **DONE**: hub lib laws **68/68** (incl. footprint + pair-content twin), bin observability/readiness **6/6** 14:30 (`generated/lane-hub-2.txt`); landing row written for the predecessor's 27th hub set |
| 6 | Post-assembly codec origin (`OwnedRuntime::codec_call`, plugin host = frozen) | prepared patch `wp-h14/h14-codec-origin.py` **dry-run clean on the live tree 14:3x** (plugin-host + owned-instance laws + hub residency hunk: all `replace`, i.e. unchanged since capture); the predecessor's overlay law build died on an overlay-sync gap (`validate_fragment_array_structure`, 27th 20:57) → **window 3**: apply, native check plugin-host + hub, run the laws (no overlay rebuild now: machine reserved for the chain) |
| 7 | ALL catalog on 7800: idle-release/LRU under real pressure (row 2.7), boot/readyz timings | waits for 7800 on ALL (chain in rebuild-all at 14:3x) |
| 8 | `shutdown-drill` / backup-restore / README-metrics parity (row 4.9) | done by the predecessor (drill LIVE PASS 27th, parity law 7/7, spec relayed to R10); backup-restore stays permanent |

#### Session 14b log

- 12:1x read preamble 14 (rules 1–21 + 14b), AGENTS.md, fleet tail, this report. db module: no working-tree diff; the
  predecessor's P1 work = commit `5bcb2da23da` (10 files: `📝️wal/🦀️.rs` commands appended from their own encoded bytes
  inside the one transaction — `SegmentWriter::append_command`, `ArtifactWal::{preflight_submit, submit}(commands, records)`;
  `🗿️artifact/🦀️.rs` submit split into a side-effect-free plan (`plan_one`/`plan_entries`, batch-local last-writer map) →
  WAL preflight → commit (`commit_one`/`apply_entries`), so a WAL refusal leaves state untouched and commands take no DB I/O
  page credit; call sites in cli/cluster/tests pass `&[]`).
- 12:2x finished forward (not reverted — revert would have to restore 10 files from `6b8089dcb21`): `PlannedEntries` struct
  beside `DocumentState`; `open`'s replay → `plan_one(&envelope, &batch_ids, &mut HashMap::new())` then `commit_one`
  (same semantics: replay applies one envelope at a time, conflicts ignored); doc reference `apply_one` → `plan_one`.
  Main told (SendMessage). Native lane: `wp-h14/h14-lane-db.sh` (check db lib+tests, then hub lib+bins+tests),
  `FLEET_TICKET_STAMP=20260928120000`, capture `generated/lane-db-1.txt`; lane held by WG11 from 12:18.
- 12:3x–12:4x hold 0 (`lane-db-1.txt`, priority stamp): db lib EXIT 0; lib-test red on one stale `apply_entries` call (fixed);
  semio-hub red only on ST2's vcs xlsx break (`XlsxSnapshot.workbook`, relayed). Contract `💻️os/📦️packages/🦀️rust/📜️script.ts`
  asserted the old `preflight_submit(&self, records…)` signature → updated.
- 12:4x **P1 root causes** (C12: the post-cut outbox arrives as ONE batch, refused → rolled back → keystrokes lost). A
  declared-legal batch (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_*` = 8192 envelopes / 256 KiB) met four capacity refusals + one replay trap:
  (a) DB I/O per-operation credit when commands were staged as `WalBytes` (predecessor's WAL change: commands appended from their
  own bytes); (b) `DbLimits::default().max_batch_commands` 4 096 < 8 192 → now `protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES`;
  (c) the hub's per-socket DoS budget 240 tokens charged per envelope (a 241-envelope flush refused, permanent) → capacity = the
  declared maximum, whole batch charged at once (never partially), refill 60/s, and an empty budget is `DbError::Unavailable` →
  H13's transient `hub.unavailable` (client resends); (d) the replay guards (hub socket + engine) RECORDED ids at admission, so the
  resend of a batch refused after admission was refused as a replay (permanent) → `ReplayGuard::{check, record}` split,
  `SecurityGate::admit_commands` (batch) + `record_committed` after commit; the engine skips already-committed envelopes of a
  batch (idempotent merged resend) and refuses one id twice in one batch explicitly.
- 13:0x hold 1 (`lane-p1-2.txt`): db check EXIT 0 (226 pre-existing warnings), hub `--lib --bins --tests` EXIT 0 13:16, hub bin laws
  `a_document_socket_admits_a_declared_maximal_batch_and_readmits_an_uncommitted_one` + `a_transiently_refused_batch_…` **2/2 PASS**;
  db laws (security/wal/artifact/engine/sync/cluster) **330 pass / 2 fail**: (1) my `a_declared_legal_byte_maximal_batch_commits_as_one_transaction_and_replays`
  → `ArtifactEngine::open` refused the reopen → **P0**: `WalTransactionGate` (recovery + committed replay) held
  `[Option<WalRecordFrame>; 64]` while the writer bounded a transaction only by the readable segment → a committed batch of ≥ 64
  envelopes left its document unopenable (reachable since `5bcb2da23da`; before it, ≥ 63 envelopes hit the per-op credit instead).
  Fix: `WAL_TRANSACTION_RECORDS_MAX` = declared envelopes + 1 (frontier); `preflight_submit` refuses above it before any I/O; the
  gate is a bounded `Vec<WalRecordFrame>`; WAL law extended (maximal 8 192 × 32 B transaction reopens via `ArtifactWal::open` and
  replays byte for byte; one record over is refused, segment untouched); os contract assertion updated. (2)
  `fs_commits_and_reopen_storms_stay_within_their_throughput_bounds` (process-isolated child) → rerun alone in hold 2.
- 13:3x coordinator item: trusted-catalog fixtures `👥️two-package`, `🧬️stdio-gis-bootstrap`, `🧱️generation-stage` → `appChannelVersion`
  19 (2/2/3 literals). stdio-gis `profile.generationId` derived with the hub script's OWN `trustedBootstrapProfileEncoding` over
  `projectTrustedBootstrapCodecsV1` (loaded from a rewritten copy, `wp-h14/h14-bootstrap-generation.ts --write 19`): 0ca2d589… →
  76d1a92f… (4 occurrences: generationId, rotation initial/current/stale-issued); the derivation only admits the current version, the
  old id was already stale before 18. `channel-version check`: pin 19, 28 consumers, **0 findings** (`generated/channel-version-check-1.txt`).
  Oracles: `trusted-stdio-gis-bundle-check --publication-source` rc 0 (generation-stage 33 cases, publication; `fixture-oracle-2.txt`);
  full stdio+GIS bootstrap oracle **PASS** via `h14-bootstrap-oracle.ts` skipping only `proveTrustedCompiledDependenciesFixture`, which
  is red outside this fixture (`🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json` violates its schema; `fixture-oracle-1.txt`).
  RELAY ST2: `cx1-apply.py` BOOTSTRAP_GENERATION_OLD → 76d1a92f….
- 13:5x coordinator: (1) lease fixture `descriptorHex` → H13 (fixed by H13, census 0 findings 14:0x); (2)+(3) mine.
  (2) `trustedBootstrapResolveDependencies` was generalised to N packages on 09-21 (`50c97b20513`, deliberate) and lost four
  refusals the hub still enforces at load (`validate_bundle`: selected identity = its package record, topological order refuses
  cycles incl. self, profile fence `local-stdio-gis-open-v1` = gis → stdio exactly). Root fix in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`:
  per-package self-dependency refusal + new `trustedBootstrapResolveClosure(profileId, claims, candidates)` (identity = own
  descriptor, acyclic, linked-profile edge set), used by `materializeTrustedCatalogBundle` and `trustedBootstrapPreflightDescriptorsV1`
  (both hold every selected package's claims; `requests` = `selection` 1:1); the compiled-dependencies oracle resolves the whole
  two-package closure → 25/25 cases (probe `wp-h14/🗑️generated/probe-cases.ts`: 0 non-throwing refusals). Further reds of the same
  check, all stale source anchors vs deliberate 09-21/09-26 changes: fresh-component describe docstring (`Shared implementation` →
  `The ONE describe route`), Rust N-target messages (`exactly one` → `at least one` + count equality), `export async function
  materializeTrustedCatalogBundle`; `🌎️hub/🧫️fixtures/🤝️two-author-shell-v1` held `"executionProtocol": 17` unregistered → census
  pattern `"executionProtocol": N` + consumer row, `channel-version generate` wrote 19. **`trusted-stdio-gis-bundle-check --source`
  rc 0** 14:0x (`generated/fixture-oracle-9.txt`, `-11.txt` after the re-derivation); hub `typecheck` rc 0 (`hub-ts-typecheck-2.txt`).
- 14:0x ST2 fixed svg's pinned protocol sha (stdio receipts changed) → `h14-bootstrap-generation.ts --write 19`: 76d1a92f… →
  **bda0b90f3ef9a8e253c4ee54c6e7ea2595b8377330d4044f7e9d6400abbead51** (4 occurrences); full bootstrap oracle PASS with nothing
  skipped (`fixture-oracle-10.txt`); relayed to ST2 (`cx1-apply.py` OLD).
- channel-version vitest: Pack-hex + literal + generator laws 3/3 PASS; the repo-census law's 50 s whole-repo `git grep --untracked`
  (48.8 s old pattern vs 51.0 s new at load 44) exceeds the vitest timeout under load — pre-existing cost, the CLI census is the
  measurement (`channel-version check` 0 findings).

## Session 14

| # | Item | Status |
|---|------|--------|
| 1 | Post-assembly checkpoint per compiled guest (`OwnedRuntime::codec_call`), law: outputs byte-identical (H12's 15 FNV-1a hashes) | plugin-host part WRITTEN in overlay `s14-h14-overlay` (window 3), overlay law build queued (overlay lane #6); hub part (measured residency charge + `footprintBytes`) in tree, native lane queued (#6) |
| 2 | residency-watch "pairs differ" harness defect (id-derived digests) | H12's TS fix (16:00, live PASS 16:15) + my Rust twin under the production decoders; TS oracles **8/8 PASS** 19:07; Rust twin compile+run queued (native lane) |
| 3 | ALL catalog: residency/idle-release under memory pressure (row 2.7) + permanent RSS-watch harness; cold/warm boot + `/readyz`; creation latency per package | waits for 7800 on ALL (chain not launched yet) |
| 4 | Permanent commands: backup/restore drill (row 2.11) + graceful-shutdown drill (📜️script.ts verb + nx target + generated launch row, with R10); fuzz (row 2.13) | backup/restore ALREADY permanent (os-hub-ts `backup-restore-drill` + nx + launch row); **graceful-shutdown drill WRITTEN + LIVE PASS 19:5x** (os-hub-ts verb `shutdown-drill`, acceptance `hub-graceful-shutdown`; SIGTERM → exit **1 025 ms, code 0** with a 2d.puzzle creation interpreting + open socket + edit in flight, load 115); nx target + launch row spec → R10 (rule 17); fuzz = H13 item 6 |
| 5 | Coordinator 18:4x (audit-s14-state row 4.9): os-hub README / metrics-vocabulary parity vs the running hub (`/metrics`, `/readyz` schema), live diff on 7800 + own hub, fix at the root, permanent law or nx target | **live diff done** (7800 + 8161: `/metrics` 404 on both = README's "no scrape endpoint"; liveness/readiness/observability keys = schemas) → 5 README defects + 1 code-first body fixed; **permanent law** `hub README parity` (4 cases, os-hub-ts vitest) **7/7 PASS** 19:4x; liveness schema bin law queued (native lane) |

### Session 14 log

- 18:3x start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet), `📓️wp-h12.md`,
  `📓️wp-h10.md`, `📓️wp-db1.md`, `📓️acceptance-s13.md` §2, `📓️fleet-13-agents.md` from 14:00, `📓️wp-h11.md` (items K/S),
  `📓️wp-h13.md`, `📓️wp-r10.md`. Load 73 (peers' greps + builds), 4 rustc. H12's hub 79070 is alive on **8161**
  (`os-hub-b3`, B3 clone root `s13-h12-hub-8161-b3`, 64 MiB residency).
- Reconciled H12 after its last report line (15:5x): at 16:00 H12 landed the pair-harness fix in the tree (hub TS only):
  `🌎️hub/🧪️tests/🧠️residency/🟦️.ts` (`pairContentDigest` = `semio.hub.pair-content/v1`: pack whole, SPR frame by frame
  without each frame's CRC-32C/`back_len` and a commit's chain hash, own id → `<document>`), new oracle
  `🌎️hub/🧪️tests/🪞️pair-content/🟦️.ts` + fixture `🌎️hub/🧫️fixtures/🪞️pair-content-v1/🔣️.json` (4 captured pairs), vitest config
  entry. Live rerun on 8161 16:15: `[acceptance] hub-residency PASS — 38/38 creations opened … pairs agree`
  (`wp-h12/generated/residency-8161-b3-2.txt`). No landing row. The TS law names a Rust twin
  `the_pair_content_digest_is_the_fixtures_under_the_production_decoder` (`🛰️lag-rebootstrap`, production decoders + `sha2`)
  that does not exist (`git grep` 0 hits) → item 2 = write it + run both.
- 18:4x coordinator: + item 5 (row 4.9); rule 16 (relay via main) + rule 17 (harness contract, reuse os-hub-ts verbs, no
  project.json/launch edits, send R10 the target spec). H13 (via main): no edits in OwnedRuntime / trusted-catalog residency /
  `interpret_off_worker` — mine.
- 18:5x **item 1 design.** `CompiledHandle.owned` becomes `OwnedCompiledGuest { artifact, codec_origin: OwnedCodecOriginCell }`.
  `codec_call` = get-or-assemble the origin, then run the operation on a CLONE of the origin instance (`OwnedSemioInstance`
  is `Clone`: memories, tables, globals; no serialization). Assembly = `codec.pack-schema-hash` for the schema `""` (no app
  declares it: the export runs `__semio_ensure_plugin_runtime`, answers its refusal) on a fresh instance, kept after its
  Deallocate. One assembly per compiled guest: a concurrent caller waits on a Condvar, relays the assembler's fuel as its own
  progress (a hub stall bound sees it move, and sees it stop), observes its own `GuestCallCancellation`; a failed / cancelled /
  panicking assembly leaves the cell `Absent` (drop guard) and a waiter takes over. The call's `budget.fuel` covers assembly +
  operation; progress reports both as one count. `CompiledHandle::codec_origin_bytes()` = origin memory + diagnostics.
  Laws (overlay, owned-instance-open tests, real note component): `codec_calls_answer_from_the_assembled_origin_exactly_what_a_fresh_instance_answers`
  (genesis/pack-schema-hash/print-mirror/apply-ops each == a fresh instance's answer; first call's fuel == assembly + second
  call's fuel exactly; second < fresh; genesis A, B, A → A unchanged; origin shared by clones; foreign schema refused),
  `a_call_waiting_for_another_calls_assembly_relays_its_fuel_honours_its_cancellation_and_takes_over`; third-party oracle:
  `wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter` now also compares wasmtime's fresh instance against the
  owned origin path. Overlay = tracked files + 2 589 gitignored generated inputs cloned (`wp-h14/h14-overlay-sync.py`, APFS
  clonefile); first overlay build 18:56 died on a missing generated file (`🔤️tokens/🦀️.rs`, gitignored) → extras added, relaunched
  19:14 (`wp-h14/generated/overlay-host-laws-2.txt`, overlay lane position 6).
- 19:0x **item 1 hub part (tree, rule 35, hub-only):** the residency ledger charges what a guest holds, not only its
  registration: `GuestResidentFootprintV1` (value-measured bytes; `CompiledHandle` → 0 today, → `codec_origin_bytes()` in window 3),
  slot charge = registered + footprint (admission, `configure`, `state`), `GuestResidencyV1::remeasure(value, context)` after
  every hub codec call (genesis, print-mirror, apply-ops, replay, row verification): a resident guest that grew past what is left
  displaces LRU unheld guests only if its uses outnumber theirs (the same TinyLFU rule), else becomes held (serves its
  operation, dropped with it), and one grown past the whole budget is never resident. Schema-first: `TrustedCatalogGuestResidencyStateV1`
  + required `footprintBytes` (JSON schema, Rust, TS twin, observability fixture ×8, observability unit law, admin page figure
  "Codec origins"/"Codec-Ursprünge", residency-watch round report). Law `a_guest_that_grows_in_a_call_is_charged_what_it_holds`.
- 19:0x **item 2:** Rust twin written: `🛰️lag-rebootstrap` unit law `the_pair_content_digest_is_the_fixtures_under_the_production_decoder`
  (production `decode_canonical_checkpoint_pair` + replication `format::FrameCursor`, own SHA-256; asserts frames == fixture
  `sprFrames`, digest == `contentDigest`, `sameContent` groups share one digest). The hub has no `sha2` dev-dependency (adding one =
  Cargo.toml/lock edit, frozen), so the third-party oracle is the TS law's node:crypto SHA-256; the TS header claimed `sha2` →
  corrected. **TS oracles 8/8 PASS** 19:07 (`bun ./📜️script.ts test long pair-content observability`, pair-content 5 + observability
  Ajv 3 incl. the fixture with `footprintBytes`; `wp-h14/generated/ts-oracles-2.txt`; the `quick` level's 15 s budget killed the
  first run at load 99: `ts-oracles-1.txt`).
- 19:07 native lane hold queued (`wp-h14/h14-lane-hub.sh`, `generated/lane-hub-1.txt`: check hub lib+bins+tests, lib laws
  trusted_catalog/observability/lag_rebootstrap, bin observability/readiness), position 6 behind t14/sh2/h13/lb2/g12.
- 19:2x **item 5, live diff** (`wp-h14/generated/{healthz,readyz,metrics}-{7800,8161}*`, `observability-8161.json` via
  `wp-h14/h14-obs-read.ts`, credentials from env only): `/healthz` 200 `{schema, status, runId, uptimeMs}` both hubs; `/readyz`
  200, keys = `LocalBootstrapReadinessV1` (ready: no `blockedBy`/`startup`); `/metrics` **404 on both** (README: "no Prometheus
  exposition … no scrape endpoint" — holds); `/admin/api/observability` (8161) top-level, `rows[]`, `routes[]` keys = README list =
  `HubObservabilityV1`; 25 `declaredEvents` incl. presence join/expiry/leave, boot, shutdown, catalog publication (README claim holds).
  **Defects found + fixed:** (1) README said `blocked_by`, the body/schema say `blockedBy`; (2) the startup object omitted its
  `catalog`; (3) the Known-gaps list pointed at a section "Health and readiness" that does not exist (→ "Health endpoints");
  (4) `SEMIO_TRACE_LEVEL`/`SEMIO_TRACE_SINK` are read by the hub (`Tracer::from_environment`) but were missing from the env tables
  (prose only) → rows added; (5) `/readyz` documented no field list → full list with `?` for `blockedBy`/`startup`, schema links;
  (6) **the liveness body was code-first** (`HubLivenessV1` struct, no schema) → `LocalBootstrapLivenessV1` in
  `🚀️local-bootstrap/🧬️schema/🔣️.json`, one `hub_liveness()` for the boot server and the router (was two copies), fixture value
  `live` (🚇️pipe-v1) + Ajv checks in the 🤝️integration quick contract, bin law `the_served_liveness_body_is_the_declared_liveness_schema`.
  Also: `OS_HUB_GUEST_RESIDENCY_BYTES` row now says what the budget charges (component + footprint).
  **Permanent law** (existing vitest file `🌎️hub/🧪️tests/📊️observability/🟦️.ts`, no new directory): `hub README parity` —
  every documented health route is registered (`.route("…")` in bootstrap) and its `{…}` field list == its schema's properties
  (`?` exactly on optional ones; observability `rows[]`/`routes[]` by `$ref`); the startup notation == `startupProgress`; the router
  registers no metrics/prometheus/scrape route and the README says so; README trace levels == `TraceLevel::as_str`; env tables ==
  every `"OS_HUB_*"` a non-test hub source reads + the trace/db-driver constants. Served ↔ schema stays with the Rust laws
  (liveness new, readiness + observability existing). Run: vitest verbose **7/7 PASS** (`generated/ts-oracles-4.txt`),
  `test long observability integration` **18 passed / 1 skipped** (`ts-oracles-3.txt`), os-hub-ts `typecheck` EXIT 0,
  admin vitest **21/21** (`admin-vitest-2.txt`, ObservabilityPage laws read the fixture with `footprintBytes`). Red-before for the
  parity law not run (would need the pre-fix README in place; the positive run + each assertion naming a fixed defect is the evidence).
  Noted, not changed: `OS_HUB_ADMIN_TOKEN` / `OS_HUB_TRUSTED_CATALOG_{BUNDLE,PROFILE}` are still scrubbed/passed by 4 TS launchers
  although no Rust reads them (README calls them vestigial) — legacy plumbing, candidate cleanup (not in the parity law's table set).
- 19:5x **item 4, graceful-shutdown drill** (rule 17: existing verb module + existing directory): `runShutdownDrill` in
  `🌎️hub/🧪️tests/💾️backup-restore/🟦️.ts` beside the backup drill (shared `bootHub`, now also answering exit code/signal + output),
  verb `shutdown-drill` in `🌎️hub/📦️packages/🟦️typescript/📜️script.ts` (`withAcceptanceRecord` + `publishAcceptanceCheckResult`,
  check `hub-graceful-shutdown`, en + de summary, `blocked` on missing precondition), probe sessions gain `ended()` (close code +
  clean) in `🌎️hub/🤝️integration-harness/🟦️.ts`. Round: fresh root + catalog copy, cold boot, one document with 20 acknowledged
  edits, socket left open with a 21st edit in flight, a heavy-kind creation in flight → SIGTERM → pass iff exit code 0 within
  `SHUTDOWN_EXIT_BOUND_MS` (10 s = the db shutdown deadline), `server.shutdown … database=closed` recorded, socket ended with a
  close frame (≠ 1006), restart on the same root, head ordinal ≥ 20, next edit accepted, interrupted creation answers typed (< 500).
  **LIVE PASS** 19:5x (`OS_HUB_BINARY` = copy of the 16:08 build-dev `os-hub` = `.🧬semio/🌐hub/s14-h14-bin/os-hub-1608`, catalog B3,
  load 115): SIGTERM → exit **1 025 ms code 0** with the **2d.puzzle creation interpreting**; socket close **1012** (service restart)
  but `wasClean: false` (the hub exits before the closing handshake completes — noted); 7/7 checks (`generated/shutdown-drill-1.txt`).
  The observability read at SIGTERM answered non-200 in development mode (packages-verifying count unknown) → the drill now records
  the status. Hub 8161 (H12's, pid 79070) stopped 19:5x (SIGTERM → 2 871 ms); no hub of mine runs.
