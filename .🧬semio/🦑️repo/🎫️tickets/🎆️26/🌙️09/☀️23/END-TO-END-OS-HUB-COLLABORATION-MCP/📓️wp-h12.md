# WP-H12 — Hub Performance + Operations (successor of H10)

Slice: H12 (session 13, 2026-09-26 19:2x). Coordinator = main chat. Ports: hubs 8160–8169, serves 6660–6669.
Private cargo target: `.tmp-ticket/wp-h12/target`. Captures: `wp-h12/generated/` (expendable). Durable data/logs:
`.🧬semio/🌐hub/s13-h12-*`. Handovers: [📓️wp-h10.md](📓️wp-h10.md), [📓️audit-s13-hub.md](📓️audit-s13-hub.md) §4 P1-2/P2-3,
[📓️wp-h9.md](📓️wp-h9.md) rows M/V. Neighbours (not duplicated): H11 (hub correctness), LA (H10 Q1/Q2/codec-app landing),
Z3 (Docker/devcontainer/cross-platform), DB1 (db throughput), W3 (all-package catalog, 7800).

## Session 13

| # | Item | Status |
|---|------|--------|
| 1 | Residency at scale (audit P1-2): law + live on B2 (many kinds/documents, RSS bounded by capacity, evicted guest reloads with same results, no thrash under round-robin > capacity); again on the all-package catalog | CODE WRITTEN, not compiled (waiting for < 10 rustc, rule 22): operator budget `OS_HUB_GUEST_RESIDENCY_BYTES` (schema default/min/max), per-operation use counting + frequency admission (anti-thrash), unadmitted guest lives exactly as long as its operation, counters; 6 laws written |
| 2 | Boot + readiness: `/readyz` per-package phases/bytes/steps schema-first; cold/warm boot on B2; compiled-guest pipeline cached; lazy codec interpretation | CODE WRITTEN, compiling: `TrustedCatalogLoadProgressV1` (per-package phase/bytes/rows, counts only) in `/readyz` `startup.catalog` + observability; **lazy row verification** (load reads + digests only; rows no memory pinned are verified in the background after the hub serves, smallest first, or by the package's first codec call — no call ever runs on an unverified row; a mismatch refuses that package only). Laws: progress law, trust-failure law adapted, readiness law +1 body. Measurement waits for low load |
| 3 | Session mint + first-creation latency per package after LA's Q1 + codec-app; next bottleneck; laws with bounds | IN PROGRESS: found + fixed (code, not compiled) the creation's duplicate guest validation — genesis validated the identical pair twice (`Input` then `Output`), each a full guest `print-mirror` run (≈ one app-bundle construction each, H10) → one validation, ≈ 1/3 of a creation's guest work saved; law pins one validation + the stage sequence. Q1/Q2/codec-app not landed by LA yet → re-measure after |
| 4 | Generative/fuzz hostile-input law over every hub HTTP/WS route + frame (deterministic seeds, bounded time, shrinking); fix findings | CODE WRITTEN, compiling: `🌎️hub/🧪️tests/🎲️hostile-generative/🦀️.rs` (SHA-256 counter draws, 7 mutation kinds, schema-driven bodies, shrinking, directory + document socket frame sequences); fixture `generative` section; node:crypto oracle for the draw vectors **PASS** (TS 4/4) |
| 5 | Observability: structured logs + metrics (per-route latency, admission waits, residency hits/evictions, db I/O census) on an admin route, en + de | CODE WRITTEN, not compiled: `HubObservabilityV1` schema + Rust projection (`🌎️hub/📊️observability`), per-route table (route_layer, templates only), residency + DB I/O census in `/admin/api/observability`; fixture + 4 Rust laws + Ajv oracle (**PASS 3/3**) + 1 bin law written; DB1's admission-wait counters (`admissionWaits/WaitMicros/Refusals`, per kind + totals) wired; admin UI tab **Observability** (en + de, 5 s refresh): admin tsc clean for my files, admin vitest **18/18** (long) |

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
