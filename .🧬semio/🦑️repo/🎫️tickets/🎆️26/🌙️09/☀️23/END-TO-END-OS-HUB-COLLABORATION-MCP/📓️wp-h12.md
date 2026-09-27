# WP-H12 — Hub Performance + Operations (successor of H10)

Slice: H12 (session 13, 2026-09-26 19:2x). Coordinator = main chat. Ports: hubs 8160–8169, serves 6660–6669.
Private cargo target: `.tmp-ticket/wp-h12/target`. Captures: `wp-h12/generated/` (expendable). Durable data/logs:
`.🧬semio/🌐hub/s13-h12-*`. Handovers: [📓️wp-h10.md](📓️wp-h10.md), [📓️audit-s13-hub.md](📓️audit-s13-hub.md) §4 P1-2/P2-3,
[📓️wp-h9.md](📓️wp-h9.md) rows M/V. Neighbours (not duplicated): H11 (hub correctness), LA (H10 Q1/Q2/codec-app landing),
Z3 (Docker/devcontainer/cross-platform), DB1 (db throughput), W3 (all-package catalog, 7800).

## Session 13

| # | Item | Status |
|---|------|--------|
| 1 | Residency at scale (audit P1-2): law + live on B2 (many kinds/documents, RSS bounded by capacity, evicted guest reloads with same results, no thrash under round-robin > capacity); again on the all-package catalog | **LIVE on B3 (15:43)**: 64 MiB budget → 3 resident guests / 57.2 MB every round, compiles per round 16/10/10, admitted 2/0/0, released 1/0/0 (stable set, no thrash), 57/57 creations + open plans, RSS 335.7 → peak 634.9 → settled 53.3 MiB. The watch's `pairsAgree` FAIL is a harness defect (pairs carry two id-derived digest fields: 40 bytes differ between two same-name creations, `pair-diff-*.txt`); guest determinism shown by the probe instead (15/15 identical answers B2 → B3). 6 residency laws PASS. All-package catalog: not published yet |
| 2 | Boot + readiness: `/readyz` per-package phases/bytes/steps schema-first; cold/warm boot on B2; compiled-guest pipeline cached; lazy codec interpretation | **LIVE on B3 (15:0x)**: cold `/readyz` 200 in **13.7 s** (H10 before 363.9 s, H10 after2 153.9 s on B2), every package pinned in the background at 98.0 s, 0 refused; warm **12.3 s** (13.5–14.5 s); SIGTERM → exit 0.53 / 0.11 s; `hub-boot` acceptance PASS (load 40–50). Laws PASS |
| 3 | Session mint + first-creation latency per package after LA's Q1 + codec-app; next bottleneck; laws with bounds | **LIVE on B3**: mint p50 **49 ms** (p95 297, load 45 + background verification; 28 ms on 8160 at load 16) vs 261–264 ms before; creation per kind measured over 3 rounds: puzzle 2d **98–256 s**, 3d **40–217 s**, 5d 49–108 s (H11 before: 749 / 579 s); others 1–47 s. **Next bottleneck, measured:** 99.4 % of a B3 puzzle codec call is the guest's one-time bundle assembly (521.5 M of 524.7 M fuel; the op ≈ 3.2 M), paid per call because the hub instantiates a fresh instance per call → fix = one post-assembly checkpoint per compiled guest, restored per call (proposal in the log; not made, source freeze) |
| 4 | Generative/fuzz hostile-input law over every hub HTTP/WS route + frame (deterministic seeds, bounded time, shrinking); fix findings | **DONE**: law PASS (3 seeds, 1 188 requests + 24 socket sequences, 4.1 s, 0 findings) after fixing its 2 real findings (extension-asset `%00` → 500; document socket dropped without close frame → TCP reset); bin laws 11/11 PASS 10:42. Was: `🌎️hub/🧪️tests/🎲️hostile-generative/🦀️.rs` (SHA-256 counter draws, 7 mutation kinds, schema-driven bodies, shrinking, directory + document socket frame sequences); fixture `generative` section; node:crypto oracle for the draw vectors **PASS** (TS 4/4) |
| 6 | Coordinator top item (09-27 05:1x): puzzle creation 749 s / 579 s (H11) → root-cause genesis, fix at the root, law with a bound, before/after | **LANDED + measured on B3**: codec calls fuel −37 % puzzle (832 → 524.7 M), −77 % note, −75 % draw, −82 % wfc; **all 15 answers byte-identical B2 → B3** (4 plugins, 5 kinds); puzzle creation 749 / 579 s → 98–256 s / 40–217 s at load 40–55. Laws 4/4 PASS. The remaining 521 M is puzzle's bundle assembly per fresh instance (item 3) |
| 5 | Observability: structured logs + metrics (per-route latency, admission waits, residency hits/evictions, db I/O census) on an admin route, en + de | **DONE (live)**: `HubObservabilityV1` schema + Rust projection, per-route table (templates only), residency + catalog + DB I/O census incl. DB1's admission waits on `/admin/api/observability`; admin tab **Observability** (en + de). Laws: Rust lib + 5 bin laws **PASS** (11:15), Ajv oracle 3/3, admin vitest 21/21. Live on 8160 (11:05): body **Ajv-valid**. The live run exposed a blind spot — the address-bucket limiter ran outside routing, so its 429s never reached the route table — fixed (limiter now a `route_layer` inside the metrics) + law `the_observability_route_counts_a_limiter_refusal_under_its_route` **PASS** |

### Session 13 log

- 19:27 start. Read AGENTS.md, preambles 13 + 12, `📓️wp-h10.md`, `📓️audit-s13-hub.md`, `📓️wp-h9.md` (status + S12 handoff),
  `📓️wp-h11.md`, `📓️wp-la.md`, `📓️wp-db1.md`, `📓️fleet-13-agents.md`. Load 99, 17 rustc, 109 GiB free. Ports 8160–8169 /
  6660–6661 free. 7800 ready on B2.
- 19:3x design (item 1): B2's 9 components total 240.7 MB < the 256 MiB default budget, so the eviction path never fired; a
  live test at B2 scale needs a configurable budget. Pure LRU thrashes a round-robin over more guests than fit (every use
  recompiles — shown by a model in the law), and counting every codec call as a use makes a creation (genesis + two
  validations = 3 calls) look like 3 uses. Chosen: (a) `TrustedCatalogGuestResidencyV1.residentComponentBytes` is now an
  operator setting (`OS_HUB_GUEST_RESIDENCY_BYTES`, schema `default` 256 MiB, `minimum` 0, `maximum` 16 GiB; boot refuses
  anything else); (b) every `OperationContext` carries a process-unique serial and owns what its calls must share, so one
  operation counts one use and a guest the ledger does not admit lives exactly as long as the operation that compiled it
  (all its calls share one compile); (c) admission (TinyLFU-style, exact per guest): a compile that does not fit stays only
  when its uses before this operation outnumber those of every least-recently-used unheld guest it would release; use
  counts are capped at 15 and halve together every 16 × registered uses (`accessCountCeiling`, `accessCountAgingPerGuest`
  const in the schema); (d) counters hits/compiles/admitted/bypassed/released/compileMicros
  (`TrustedCatalogGuestResidencyStateV1`). Files: trusted-catalog `🦀️.rs` (ledger), `🧬️schema/{🔣️.json,🦀️.rs}`,
  `🗿️artifact-authority/🦀️.rs` (`OperationContext::serial`/`retain`), `🏗️bootstrap/🦀️.rs` (env → `configure_guest_residency`).
- 19:5x laws written (trusted-catalog unit): `guest_residency_bounds_are_the_declared_schema_values` (rewritten: default/min/max/
  const, env parsing incl. refusals, state fields = schema), `a_compiled_guest_that_fits_stays_resident_and_idleness_releases_nothing`,
  `one_operation_counts_one_use_and_an_unadmitted_guest_lives_exactly_as_long_as_its_operation`,
  `held_guests_are_never_released_and_a_zero_budget_keeps_nothing_resident`,
  `a_round_robin_over_more_guests_than_fit_keeps_a_stable_resident_set` (12 guests, budget ≈ a third, 60 rounds × 3 calls:
  stable set after round 1, exactly the non-fitting guests compile per round, charge ≤ budget, answers = own source digest;
  the same trace thrashes a pure-LRU model 12/12 per round), `a_workload_that_moved_on_displaces_the_guests_it_left` (≤ 2
  aging windows), `concurrent_operations_share_one_compile`. Replaces H10's `compiled_guests_are_released_least_recently_used…`.
- 20:0x item 5 written: `🌎️hub/📊️observability/{🦀️.rs, 🧬️schema/🔣️.json, 🧪️tests/🔬️unit/🦀️.rs}` (lib `semio_hub::observability`),
  fixture `🌎️hub/🧫️fixtures/📊️observability-v1/🔣️.json`, Ajv oracle `🌎️hub/🧪️tests/📊️observability/🟦️.ts` (added to the os-hub-ts
  vitest config), bootstrap: `route_metrics_middleware` as `route_layer` (template keys only; the framework router merged
  beside it is not covered), `HubState.route_metrics`, `observability_view` → `HubObservabilityV1::read`; bin-unit: two
  `HubState` literals + the identity-field law updated, new law `the_observability_route_reports_the_routes_this_hub_answered_by_template`.
- 20:05 coordinator rule 22 (memory): nothing of mine runs (no hub, serve, browser, cargo). 22 rustc, swap 21.7/22.5 GB →
  compile waits for < 10 rustc. Headline latency/boot measurements wait for the coordinator's low-load signal.
- Peer compile break noted (blocks my trusted-catalog laws): `🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs:1494` E0502 (T12 open-kinds
  hunk, `terrain_target.profiles[0]` borrowed mutably and immutably in one `push`) — H11's 19:35 build hit it; I fix it (one
  line) only if it is still there when I compile.
- 20:1x item 2 written. Why lazy: a cold boot interprets every guest row's `pack-schema-hash` before the hub serves
  (H10: 153.9 s on B2 at load ~28; the all-package catalog and every engine change — Q1 lands a new engine identity — make
  it far longer). Now the load only reads, digests and pins what a native codec or this engine's memory already pins; the
  rest is pinned per package by `GuestArtifactComponent::verified` (one `OnceCell`: cancellation/stall leaves it for the
  next caller, a mismatch or compile failure refuses every call of that package and marks it `refused`), driven by a
  background pass spawned after `hand_over` (smallest component first, `server.catalog.publication` record at the end) or
  by the package's first codec call, whichever is first. `TrustedCatalogLoadProgressV1` (schema: trusted catalog) is
  reported through a new `AuthorityOperationControl::catalog_progress` port into the hub's `StartupProgressCellV1`, shown in
  `/readyz` `startup.catalog` while starting (local-bootstrap schema `startupProgress.catalog` → cross-module `$ref`) and in
  `/admin/api/observability` `catalog` afterwards. H10's `verify_guest_rows` + slot types are gone (one path).
- 20:1x item 4 written (see table). HTTP oracle = H9's (typed signed refusal, no 5xx but typed 503, no dropped
  connection) + no success without a usable credential on a non-public route + `/healthz` after every case; a fresh
  member session per request (a drawn sign-out cannot leak); sockets: close frame with a declared code or stay open,
  never a bare disconnect. TS oracles run: `bun ./📜️script.ts test hostile observability` → **7/7 PASS**.
- 20:20 first `cargo check -p semio-hub --tests` launched (pid 65793, `s13-h12-logs/check-1.txt`, 8 rustc at start); it
  re-checks the framework from scratch (landing slices changed framework crates) — long at load ~75.
- FYI from H11 (20:1x): 2d.puzzle "accepted forever" on a current-tree hub = CPU-bound guest genesis at 5 % CPU under swap,
  not a residency release (LRU fix holds); waits for Q1 + codec-app (LA) and LC's creation progress.
- 20:2x coordinator: law that no request is served from an unverified row, incl. after eviction + reload → written:
  `no_codec_call_is_served_from_a_row_its_component_has_not_answered` runs the PRODUCTION interpreter on a real owned-ABI
  guest built byte by byte in the test (`owned_test_guest`: one memory page, the 14 owned exports; `pack-schema-hash`
  answers a chosen 32-byte hash, `genesis` a fixed pair): row pending at load → first call verifies then serves; zero budget
  → the second operation compiles again and answers the same pair, the row stays verified once; component bytes tampered
  after verification → never compiled, never served; a guest answering a different hash → every call refused,
  `packagesRefused` 1; background pass → ready before any call.
- 20:37 rule 25: check-1 had no rustc child for 16 min (lock convoy) → stopped (my pid 65798, exit 143). check-2 relaunched
  20:43 (pid 87351/87361), waiting in the convoy too.
- 20:4x DB1 landed `DbIoCensusV1::admission_{waits,wait_micros,refusals}` + totals (compiles, DB1 20:52) → observability
  `dbIo` carries them per kind and in total (schema, Rust, TS twin, fixture, laws, admin page). TS oracles **8/8**.
- 20:4x harnesses (rule 21, extending V1's instead of forking): `residency-watch` gains `--rounds` (round-robin), the
  hub's residency after every creation, per-round deltas (compiles/hits/admitted/bypassed/released/compile ms) and a
  pair-content digest per creation (canonical pair data records only, own document id replaced) → `pairsAgree`;
  new verb `boot-watch` (nx `os-hub-ts:boot-watch`): fresh root from a published catalog, cold + N warm boots on
  an explicit `--port`, `/readyz` `startup.catalog` until serving, admin `catalog` until every package pinned,
  acceptance result en + de. `hubSeedTrustedCatalog` moved from the backup drill into the shared integration harness (one
  copy). Found: the harness' `OS_HUB_ADMIN_TOKEN` is read by no hub code (0 hits in bootstrap) — boot-watch uses an admin
  subject + session instead. Hub TS typecheck: my files clean (the remaining errors are LD's in-flight wire change and
  peers' files). Launch rows: left to V1 (owns harness launch rows).
- 21:0x item 3: `materialize_selected_genesis` (`🌱️creation/🦀️.rs`) called `validate_pair` twice on the SAME genesis pair (stage
  Input, then Output). For a guest-only package each call is a full owned `print-mirror` interpretation, and H10 measured
  that every codec call spends ~99.9 % constructing the bundle's apps — so a creation ran the guest three times (genesis +
  2 × print-mirror) where two carry the whole information. Now one validation (stage Output, the genesis's product),
  progress 4 units (Preflight, CatalogResolved, OutputValidated, Derived). Law: the creation unit law's exact case asserts
  one validation and that stage sequence (`validations` counter on its fixture codec). LC's stage mapping ignores both
  validation stages, so its creation-progress set is unaffected (told main; its vitest-config anchors need re-deriving
  because of my observability oracle line).
- 21:12 rule 26: check-3 in `build-fleet-b` (cold) launched; 21:3x cut by the usage limit (check-3 died at session end, last line
  `Checking scopeguard`). LC re-derived its two anchors (dry run 9/9); V1 relay sent 21:1x.
- 04:58 (09-27) RESUME, rule 28. Reconciled: every H12 edit is intact and was auto-committed at 22:00 (`40a2736e661`); the one
  staged change on top in bootstrap is a peer's correct follow-up (`admin_observability` now answers
  `Json<HubObservabilityV1>` instead of `serde_json::Value`). Machine: load 9, 155 GiB free. 05:03 check-4 (`build-fleet-b`,
  `w2-detach.py`, pid 92860, `s13-h12-logs/check-4.txt`).
- 05:1x coordinator top item: puzzle creation root cause. H11's sample (2d genesis): interpreter dispatch + LEB + ControlFrame
  allocs dominate (host-side, Q1 addresses) — but WHAT the guest interprets is the question. Code reading: a codec call builds
  the owning editor (`plugin_artifact_codec_app` → `create_app` → `EditorApp::with_registry` → `initial_snapshot()`), and
  genesis calls `initial_snapshot()` a second time (`artifact_app_genesis_pair`); the creation's validation (print-mirror)
  constructs the app once more. So `initial_snapshot()` runs 3× per creation — and puzzle's did far more than its document:
  - 2d `initial_snapshot()` → `set_active_example::warm_examples()` forced EMPTY + CONCRETE_FOREST + NAKAGIN: two example
    documents rendered to JSON and parsed into `Puzzle2dSnapshot`, then discarded (the snapshot is the EMPTY fixture);
  - 3d `initial_snapshot()` converted the Concrete Forest fixture (its document), then built a scene from it and synced a
    full precompute session (`sync_precompute_session`: every mesh fallback, then the collision scene) into a
    `Puzzle3dPlayApp::default()` that is dropped on return — every rendering app syncs its own session anyway
    (`with_puzzle3d_app_for`), so that work reached no one; 5d only converts its document.
  Fix (guest, `🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d}/…/✏️editor/🦀️.rs`, `🛍️set-active-example/🦀️.rs`): the snapshot is its document
  only; `warm_examples` deleted (the example load already forces its own `LazyLock`). Laws (editor unit tests):
  `the_initial_snapshot_costs_only_its_document` ×2 — this thread's heap peak (`semio_framework_trace::HeapWitness`, now also
  installed in the 2d test binary) while building the snapshot ≤ peak of converting the document itself + 4 KiB.
  Census (all `✏️s/🔌️plugins` `initial_snapshot` bodies): 18 non-trivial, all build their own default document (remodel demo,
  cad forest scene, wfc examples, forms spec, fem3d boot snapshot) — no other hidden work. Found on the way (not mine):
  fem3d editor + viewer `initial_snapshot` print `[DEBUG] …boot snapshot` lines.
- 05:20 combined check (hub + puzzle 2d/3d `--tests --features …/component-app-assembly`, `build-fleet-b`, `--keep-going`,
  detached pid 4193, `s13-h12-logs/check-6.txt`). check-4 had failed on a PEER's in-flight `🌱️value/🔁️codec` edit (fixed by
  them 05:04); check-4's orphaned cargo (mine) held locks and blocked check-5 → stopped.
- 05:2x LA handed over (its words: "go ahead and land it yourself") the root fix below its codec-app resolution:
  `VcsArtifactApp`'s five codec methods never read `&self`, yet every codec call constructed the owning app
  (`EditorApp::with_registry` + store around `initial_snapshot()` + registries), then drained and closed it. Now
  each registered app records an `ArtifactCodecTableV1` (fn pointers monomorphized per app type: pack-schema-hash,
  genesis, print-mirror, apply-ops, replay-envelopes; `artifact_codec_table::<A>()`) on `SurfaceDeclaration` and
  `AppFactory` (all four registration sites incl. `🏗️builder/🦀️.rs` ×3); `plugin_artifact_*` answer through
  `artifact_codec_owner` → `Plugin::app_codec`. Deleted: the five `PluginApp::artifact_*` trait methods (only callers were
  the codec runtime; the `#[dyn_enum]` dispatch follows), their `VcsArtifactApp` impls, `plugin_artifact_codec_app`,
  `close_artifact_codec_app`, `retain_unclosed_artifact_codec_app` and their constants. Script (applied, dry-run clean
  first): `wp-h12/patches/codec-table.py`. Per puzzle creation this removes 3 app constructions (genesis, validation,
  and the second `initial_snapshot()` inside construction): the validation no longer re-parses the Concrete Forest
  fixture in a fresh instance at all.
  Laws: plugin `codec_calls_construct_no_app` (LA's law rewritten: every app factory replaced by one that PANICS, and
  pack-schema-hash + genesis + print-mirror still answer for all three fixture schemas); puzzle plugin
  `guest_codec_tables_answer_like_the_declared_native_codecs` (2d/3d/5d: table answers vs each kind's declared native
  codec `ArtifactCodec::of::<Snapshot, Mutation>` — same pack-schema hash, byte-identical mirror of the genesis pair);
  hub `guest_codec_tables_answer_like_the_linked_native_codecs` (gis + vcs vs their linked native receipts; stdio's
  `plugin()` is configured out in the hub's feature set, so the third plugin is puzzle's own law).
  Coverage gap noted (not fixed): `🖥️host/🧪️tests/🔬️owned-instance-open` laws described codec calls as the sweep that
  constructs and closes every app of a staged bundle (disposer proof); since LA's resolution (one app) and now (none)
  that sweep proves nothing about disposers → needs its own explicit construct-and-close law.
- 05:35 T13: the three `🏗️builder` `AppFactory` initializers were missing `codec` (E0063, native too) → fixed at once;
  T13's wasm32-wasip2 check through the plugin crate then compiled (05:40/05:43).
- 05:4x check-8 → hub bin test HRTB `Send is not general enough` at the Check In spawn (my `verify_rows` streamed
  closure-built futures through `buffer_unordered`): restructured into `verify_once` / `verify_rows` (collected futures
  of `verify_row`) → gone. check-9 (05:46, build-landing): plugin lib + lib test, puzzle 2d/3d/5d lib, 2d/3d lib test,
  hub lib + lib test + bin: compiled (warnings present = type-checked). Only red: hub bin TEST
  `🔬️bin-unit/🦀️.rs:7528` `messages.code/level` on `&u8` — a peer's in-flight `ApplyOutcome::Rejected` wire change (not in
  my diff; LD/H11 area).
- 05:47 guest laws running (pid 30279, `laws-guest-1.txt`); wasm32 check queued in the mutex behind wg9 (05:38).
- 06:19 laws run 1 (build-landing): puzzle 2d + 3d `the_initial_snapshot_costs_only_its_document` **PASS**. Two laws
  failed on wrong expectations, not implementation: the plugin fixture's snapshots are hand-written packs (no record spec
  → pack-schema-hash answers the declared refusal), and a guest print-mirror renders `dsl` as the store's document text
  while a kind's native codec renders its own DSL — so parity compares hash + op log + that the native codec decodes the
  table's genesis pair. The failing run itself showed the table path works: puzzle 2d's hash matched the native codec's and
  genesis + both mirrors answered with every factory unreachable. Corrected 06:20; run 2 killed at the 06:35 cut (EXIT 143).
- 06:04 wasm32-wasip2 check of plugin + puzzle 2d/3d (`component-app-assembly`) through the wasm mutex: **EXIT 0**
  (`s13-h12-logs/wasm-1.txt`). Landing row added to `📓️landing.md` (06:3x).
- 06:35–09:5x cut (usage limit). 09:55 RESUME: the codec-table patch is fully applied (the script writes only after every
  anchor matched; verified symbol by symbol), compiled native (check-9) and wasm32 (wasm-1) before the cut. Rule 30: no guest
  edits; laws re-run through the native mutex in `build-fleet-b` (`laws-guest-3.txt`, queued behind ld/lb/la/r9/g11).
- 10:12 guest laws run 3 (native mutex, build-fleet-b): **4/4 PASS** (`laws-guest-3.txt`). Landing row updated. Hub laws (items
  1/2/4/5/6: trusted catalog incl. residency + progress + no-unverified-row + gis/vcs parity, observability, creation genesis,
  readiness, hostile incl. generative) running now in the same lane (`laws-hub-1.txt`).
- 10:12 hub laws run 1 (`laws-hub-1.txt`, native mutex): lib 44/60 — the 16 failures are one cause outside my code: the
  trusted-catalog fixture carried `appChannelVersion: 17` while a peer's wire change moved `CHANNEL_VERSION` to 18; the peer
  updated the fixture at 10:12:40, after my test build → re-run. Bin: readiness laws 2/2, observability laws 4/4,
  `every_route_answers_hostile_input…` PASS, draw-vector law PASS; `the_hostile_input_fixture_covers_every_registered_route`
  red on a PEER's new route `DIRECTORY_PREFERENCE_PAGE_PATH_V1` (no hostile vectors) → row added to the fixture.
- **Generative hostile-input law, first run: 1170 requests + 24 socket sequences in 4.2 s, 21 findings (shrunk):**
  1. 11 × `OPTIONS` (+ absent/forged/empty/lower-case credential) → 2xx: the CORS preflight answers before any
     credential is read — by design; the oracle now allows a preflight success that discloses nothing (empty body).
  2. **`GET /🧩️extension-modules/%00/module.mjs` → 500** (real): a NUL path segment reached `tokio::fs::read`, whose
     `InvalidInput` mapped to 500. Fix: `extension_asset_path` accepts only an id plus normal, control-free path components
     (no root/prefix/`.`/`..`/NUL/backslash); read errors map NotFound/IsADirectory/NotADirectory → 404,
     InvalidInput/InvalidFilename → 400, anything else → typed 503.
  3. **9 × document socket: bare `Connection reset by peer`** (real): a socket whose hello was not a valid hello got an
     error frame and was DROPPED without a close frame while the client's frames were unread → TCP reset; the directory
     socket already closed with 4401. Fix: `close_socket` (Close 4401 after the error frame, then the closing handshake)
     on both hello refusals, and every document socket now drains the client's remaining frames for its close reply
     (`drain_closing_socket`, ≤ `SOCKET_CLOSE_DRAIN` 1 s) before it is dropped.
- 10:2x LC's `AuthorityProgressStage::GuestCompiling` now reported by `GuestArtifactComponent::compiled` (0/1 before, 1/1
  after the compile) → a creation shows `compiling-guest` while its guest compiles. Hub laws run 2 queued (`laws-hub-2.txt`).
- 10:18 **before-measurement, B2 puzzle (old guest), owned interpreter of this tree (Q1), release probe, load ~20:**
  `codec.genesis` of `puzzle.2d.fixture` = **829 481 432 fuel, 17.4 s** — refused at the end (the probe's document id was not
  server-minted), i.e. nearly all of it is spent BEFORE the genesis logic runs: constructing the bundle's apps. Probe fixed
  (server-minted id), rebuild queued.
- 10:30 hub laws run 2 (`laws-hub-2.txt`): lib **59/60** — every H12 trusted-catalog law PASS (6 residency laws incl. the
  round-robin scale law, catalog progress, `no_codec_call_is_served_from_a_row_its_component_has_not_answered` on a real
  owned-ABI guest, `guest_codec_tables_answer_like_the_linked_native_codecs` gis + vcs, the rewritten missing-provider
  law, `all_trust_failures…`); the 1 red is `every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule`
  (committed descriptors predate the rebuild's describe; not mine). Bin 10/11: generative law **1 finding left** (of 21).
  H11 (10:2x) flagged the missing-provider law; answered + rewritten (a provider binding nothing defers the row to its
  component — never served while unanswered — instead of failing the load).
- 10:3x the remaining finding is deterministic (seed 1 socket case 1, re-run 3/3 from the built test binary in 0.5 s): after
  a VALID hello every early `return` of the document socket (document not announced, schema-hash mismatch, storage…) sent
  only an error frame and dropped the socket. Fix at the root: `handle_ws` is now the session (`serve_document_socket`)
  followed by ONE closing step for every exit — a `1000 session-ended` close (refused and harmless when the session already
  sent its typed close) and the closing handshake (`close_socket` → `drain_closing_socket`). Run 3 queued (`laws-hub-3.txt`).
- 10:33 **BEFORE (B2 puzzle component, old guest), release probe with this tree's interpreter, load ~20:**
  | schema | genesis | print-mirror | pack-schema-hash |
  |---|---|---|---|
  | puzzle.2d.fixture | 832 622 870 fuel, 20.3 s | 831 613 376, 19.7 s | 830 300 195, 17.2 s |
  | puzzle.3d | 832 415 740, 14.2 s | 831 548 018, 10.7 s | 830 227 621, 11.3 s |
  Every call costs ≈ 830 M fuel of which the operation itself is 1–2.4 M: the rest is the old guest constructing all six
  apps of the bundle (3 editors incl. 2d's example parses and 3d's precompute sync, 3 viewers). A creation made 3 such calls
  (genesis + 2 validations) ≈ 2.5 G fuel → H11's 749 s / 579 s on a debug hub at 5–40 % CPU. AFTER = the same probe on B3's
  puzzle component (codec-app resolution + no app construction + lean `initial_snapshot`) and 1 validation per creation.
- 10:34–10:38 my refactor left os-hub red (E0596, `&mut sender` on a `&mut` binding, 2 sites) → fixed at once, main told.
  10:41 `cargo check -p semio-hub --bins --tests` **green** (native lane). 10:42 bin laws **11/11 PASS**: readiness ×2,
  observability ×4, hostile fixture coverage, hostile typed refusals, draw vectors, **generative law: 1 188 requests + 24
  socket sequences, 0 findings** (`check-hub-1.txt`). Socket regression guard (every bin law naming socket/presence/
  welcome/close) queued (`laws-hub-socket-1.txt`).
- 10:52 socket regression guard: **45/45 PASS** (every bin law naming socket/presence/welcome/close, `laws-hub-socket-1.txt`).
- 10:5x admin: 3 `ObservabilityPage` laws (fed the language-neutral observability fixture body) → admin vitest **21/21 PASS**.
- 10:5x more BEFORE rows (B2 old guests, same probe): note.document genesis 37.4 M / print-mirror 36.6 M / hash 35.3 M fuel;
  drawing.document 35.7 / 34.9 / 33.3 M; s.wfc.wfc2d 178.1 / 178.2 / 175.1 M (`wp-h12/generated/probe-b2-*.txt`). Same
  shape everywhere: the operation is 1–3 M, the rest is constructing every app of the bundle per call.
- 11:05 live on my hub 8160 (the chain's 11:04 current-tree `os-hub` build-dev, copied + signed to
  `.🧬semio/🌐hub/s13-h12-bin/os-hub-1104`; fresh root WITHOUT a catalog; started via `w2-detach.py`, pid 29209):
  **session mint p50 28 ms, p95 62 ms** (8 sequential after the sign-in rate window; the first probe's 20-in-a-row hit the
  per-address limiter: 10 × `429`, visible as `server.rate-limit` refused 22 in observability). `/admin/api/observability`
  live as admin (`observability-8160-1.json`): `POST /auth/sessions` 19 answers p50 22.9 ms / p95 33.1 ms, `GET /readyz` 119
  (503: no catalog), residency + catalog `null`, DB I/O census 3 tasks, 0 admission waits; the body is **Ajv-valid**
  against `HubObservabilityV1`. Hub stopped 11:12 (stop script, SIGTERM, 162 ms).
- 11:1x finding from that live body: `server.rate-limit` refused 22 while `POST /auth/sessions` showed `rateLimited` 0 —
  the address-bucket `rate_limit_middleware` was a `.layer` OUTSIDE routing, so its refusals never reached the per-route
  `route_layer`. Fix: the limiter is now a `route_layer` inside the metrics layer (router is flat, no nest/fallback; every
  limiter class names a registered route, so nothing unrouted loses a limit). New bin law
  `the_observability_route_counts_a_limiter_refusal_under_its_route` (burst × 401 then a 429 → requests burst+1,
  clientRefusals burst, rateLimited 1). 11:14 `cargo check -p semio-hub --bins --tests` **green** (warnings shown = typed),
  bin laws **19/19 PASS** 11:15 (readiness, observability ×5, hostile incl. generative, every credential sign-in law incl.
  the lockout, refused origin, proxied cleartext) — `check-hub-2.txt`.
- 11:18 the after-measurements are one detached chain (rule 28), `wp-h12/b3-measure.sh` (pid 38195, log
  `s13-h12-logs/b3-measure.txt`): waits for W3's `b3-publish.rc` → owned-probe chain with the BEFORE rows' binary (puzzle 2d/3d,
  note, draw, wfc) → waits for 7800's open-plan probe (≤ 90 min) → hub binary = the chain's post-publish build-dev (has the
  limiter fix) → `boot-watch` cold + 1 warm on a B3 copy (8162) → hub 8161 on a B3 clone with `OS_HUB_GUEST_RESIDENCY_BYTES`
  64 MiB → mint ×8 → `residency-watch --rounds 3` (per-kind creation ms, compiles/hits/evictions per round, pair digests)
  → observability (`wp-h12/obs-read.ts`, Ajv) → stop. Hubs at NI 0, one at a time.
- 12:1x README: the `GET /admin/api/observability` row now documents the full `HubObservabilityV1` body (routes by template,
  limiter 429s counted, `null` residency/catalog without a catalog, `dbIo`).
- 12:1x tried a B2 cold/warm ready measurement at load ~5 with the 11:04 current-tree hub (8163, B2 clone like H10's):
  **the hub refuses B2 at boot** (exit, both boots): `decoded package descriptor identity does not exactly match its trust
  record` — B2 predates CHANNEL_VERSION 18, so B2 cannot serve any current-tree hub (B3 is the only after-catalog). The
  diagnostic named neither the package nor the field → fixed: `validate_descriptor` names the package and every differing
  field, and a catalog of another app channel is refused as such ("package X was published for app channel 17 but this hub
  speaks app channel 18: publish the trusted catalog again…"); law `descriptor_projection_rejects_…` extended (package +
  field; stale channel names both channels); README boot section says so. My poller was stuck on the dead hub (a 600 s
  tool timeout moved it to the background) → killed its python + loop shell (mine); no hub left running.
  12:20 `cargo check -p semio-hub --bins --tests` **green** (warnings shown), bin laws 17/17 PASS (`check-hub-3.txt`);
  12:22 lib laws **59/60** — the red is still T12's `every_committed_editor_…_one_rule` (awaits the describe), both
  descriptor laws + `all_trust_failures…` PASS (`laws-hub-4.txt`).
- 12:32 the coordinator paused the slice ("only the coordinator running now"): no further edits, nothing new started.
  `b3-measure.sh` (pid 38195) stays armed; it is waiting on `b3-publish.rc`. Observed at 12:32: W3's `b3-rebuild-all.txt`
  ends in `rebuild-all 6/11 check (registry)` → `error: script "nx" exited with code 1` (no `b3-publish.rc` yet). No hub
  of mine is running.
- 14:5x RESUME: B3 published 13:54 (generation `e3c0c98e…`, the same 9 packages as B2, components 233.1 MiB); 7800 READY on
  B3 13:57. My armed chain had stopped 13:18 on the first failed publication (rc=1). Adapted `b3-measure.sh`: no chain
  wait; hub binary = a copy (cp + ad-hoc re-sign) of the one 7800 runs, `s13-w3-bin/s13-w3-hub-7800-b3/os-hub` (it has
  the 11:14 limiter fix + the 12:20 channel refusal); relaunched detached 14:57 (pid 24676). Load 40–50 throughout.
- 15:0x **AFTER, owned-probe fuel, same probe binary as the BEFORE rows (10:31 build of this tree's interpreter), B3 vs B2:**
  | package / schema | op | BEFORE B2 fuel | AFTER B3 fuel | Δ |
  |---|---|---|---|---|
  | puzzle.2d.fixture | genesis / print-mirror / pack-schema-hash | 832.6 / 831.6 / 830.3 M | 524.7 / 523.8 / 522.4 M | −37 % |
  | puzzle.3d | genesis / print-mirror / pack-schema-hash | 832.4 / 831.5 / 830.2 M | 524.6 / 523.8 / 522.3 M | −37 % |
  | note.document | genesis / print-mirror / pack-schema-hash | 37.4 / 36.6 / 35.3 M | 8.75 / 7.88 / 6.61 M | −77 % |
  | drawing.document | genesis / print-mirror / pack-schema-hash | 35.7 / 34.9 / 33.3 M | 9.10 / 8.36 / 6.77 M | −75 % |
  | s.wfc.wfc2d | genesis / print-mirror / pack-schema-hash | 178.1 / 178.2 / 175.1 M | 31.7 / 31.9 / 28.7 M | −82 % |
  **Every one of the 15 outputs is byte-identical B2 ↔ B3** (FNV-1a of each answer equal: e.g. puzzle 2d genesis
  `027e9858…`, 3d `25320c5a…`, note `1069f648…`, draw `a3b28e7c…`, wfc `91a08f42…`) — the codec-table change answers exactly
  as the app-constructing path did, now live over 4 plugins (5 kinds).
  **Next bottleneck, measured (warm probe, one instance, `probe-b3-puzzle-warm.txt`):** the first call on a fresh puzzle
  instance costs 521.5 M fuel, every later call on the same instance 0.08 M (a refused genesis) — i.e. **99.4 % of a B3
  puzzle codec call is the guest's one-time bundle assembly** (`__semio_ensure_plugin_runtime` → `puzzle::plugin()`:
  3 `declare_artifact` + 6 mutation rosters + `try_build`; `install_plugin_bundle` itself only stores it), the operation
  itself ≈ 3.2 M. The hub instantiates a fresh instance per codec call (`OwnedRuntime::codec_call`), so every call pays it.
  Root fix proposal (not made — measurements-only window, and H11 is inside `codec_call` for cancellation now): keep ONE
  post-assembly checkpoint per compiled guest (instantiate → a cheap export that runs the ensure, e.g. `pack-schema-hash`
  → `checkpoint()`), and restore from it for each codec call; it lives and is released with the compiled guest in the
  residency ledger (its bytes counted). Expected per-call cost ≈ the operation (1–3 M fuel), i.e. a puzzle creation from
  minutes to about a second; results stay identical because every call starts from the same state. Also worth a guest-side
  look at why puzzle's assembly alone is 521 M while note/draw assemble in ≈ 6 M.
- 15:0x **AFTER, boot on B3** (`boot-watch-b3.txt`, fresh temp root, the 7800 binary, load ~40–50): cold `/readyz` 200 in
  **13.7 s**, every package pinned in the background at 98.0 s, 0 refused, SIGTERM → exit 0.53 s; warm restart ready in
  **12.3 s** (all rows from verification memory at 12.3 s), SIGTERM → exit 0.11 s. BEFORE (H10, B2, load 28–60): cold
  363.9 s → 153.9 s (H10 after2), warm 13.5–14.5 s. `hub-boot` acceptance PASS.
- 15:09 **AFTER, mint on B3** (hub 8161, B3 clone, 64 MiB residency, ready 9 s after start): 8 sequential sign-ins p50
  **49 ms**, min 26, p95/max 297 ms — measured under load ~45 while the hub's background verification interpreted puzzle
  rows; on the idle-catalog hub 8160 (load 16) p50 was 28 ms. BEFORE (H10): p50 261–264 ms.
- 15:43 **AFTER, residency round-robin on B3** (`residency-8161-b3.txt`; hub 8161, `OS_HUB_GUEST_RESIDENCY_BYTES` 64 MiB vs
  the 233 MiB of components, 19 creatable kinds × 3 rounds, load 40–55): **57/57 creations ready + open plan 200**; the
  budget holds **3 resident guests / 57.2 MB** every round; per round compiles **16 / 10 / 10**, hits 23 / 24 / 24,
  admitted 2 / 0 / 0, released 1 / 0 / 0 — a stable resident set, no thrash (a thrashing LRU would recompile on every
  creation that follows a different package). RSS 335.7 MiB baseline → peak 634.9 → **settled 53.3 MiB** (581.6 MiB
  released). Creation per kind (rounds 1/2/3, ms): 2d.puzzle 219 737 / 256 155 / 98 292; 3d.puzzle 217 471 / 122 280 /
  39 690; 5d.puzzle 108 195 / 48 879 / 49 750; s.stdio.md 159 350 / 30 727 / 47 359; wfc kinds 4–32 s; gis 11–38 s;
  block/drawing/note/text/animate 1–10 s. BEFORE (H11, B2): 749 s / 579 s per puzzle creation.
  The watch's acceptance says FAIL only on **"pairs differ" for every kind — a harness defect, not a residency one:**
  15:5x `wp-h12/pair-diff.ts` (hub restarted on the same root) created 2d.drawing and s.note.note three times each in one
  space (names A, A, B): two creations with the SAME name differ in exactly 40 bytes (a 4-byte field right after the
  schema string and a 36-byte field at the end of the pair) and nowhere else, with the document id already replaced
  (`pair-diff-{drawing,note}.txt`). Those are digests over id-bearing content (all bytes differ at random — a hash, not
  a clock), so two documents' pairs can never be byte-equal and `pairsAgree` cannot pass by construction. The
  guest-determinism evidence is the probe instead: the same document id yields byte-identical genesis/print-mirror/hash
  answers even across two different components (B2 → B3, 15/15). Harness fix (not made, source freeze 15:45):
  compare per kind the guest's `print-mirror` DSL of each created pair, or strip those two digest fields, or re-validate
  the round-1 pair through the recompiled guest each round. Hub 8161 stopped 15:5x (686 ms); no hub of mine running.
