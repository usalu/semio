# Terra Audit — Admin Live SQLite Post-Stack Frontier

## Verdict

**SOURCE-CLOSED / COORDINATOR-RUNTIME-WITNESSED / NOT INDEPENDENTLY RERUN.** The diagnosed retained `[u64; 4096]` task/result owner is absent from the current tree. `DbIoU64List` now retains only `Option<Box<[u64]>>`, and the task admission, rejected-admission rollback, result close, and lost-owner retirement paths preserve the bounded ledger.

The existing Sol report records a current-source registered journey (`30455`, exit 0) that reached the real loopback hub, protected local-bootstrap relay, shipped SPA, and Chromium EN/DE flow. This audit did not start a cold hub build, Docker, or a competing Cargo run, so that terminal is coordinator evidence, not an independent Terra runtime result.

The qualified boundary is narrowly honest:

- Directory authority is SQLite; the journey intentionally selects filesystem artifact/document storage plus SQLite directory storage.
- It proves local loopback bootstrap and an `admin-relay` session, not production OIDC.
- It does not prove PostgreSQL or Neo4j. The separate all-feature admin gate keeps its PostgreSQL law terminal and requires Docker; absence must remain a failure, never a skip.

There is no current source-deterministic blocker inside that narrow SQLite admin journey. The next acceptance defect is **coverage**, not a demonstrated product failure: `admin-live-journey-check` does not itself select the direct DB heap/ledger law that established the stack repair. It was run separately before the reported final journey. A fresh combined gate needs that exact selector before this boundary can be upgraded from coordinator-witnessed to independently qualified.

## Inputs and method

Read-only review on 2026-09-04 of current source, the prior reports, target/launch registration, and Cargo metadata. No compilation, hub process, browser, Docker, source, test, plan, or matrix mutation was performed.

The historical `📓️terra-hub-admin-live-bilingual-audit.md` is superseded for its former static-token, generic-command, tokenless-stream, and startup-red findings. Its scope predates the local-bootstrap relay, typed intent route, and current registered process journey. It remains useful only as historical provenance; it is not current failure evidence.

`cargo metadata --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml --no-deps --format-version 1` completed successfully. It reports `semio-hub` defaults to `sqlite`, with separate `postgres` and `neo4j` features.

## Root-cause recheck

| Boundary | Current evidence | Assessment |
| --- | --- | --- |
| Retained list shape | `DbIoU64List { values: Option<Box<[u64]>>, len, result_handback }` at `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:1841-1846`; no current `[u64; 4096]` in the hub/DB source census. | Closed. The 32 KiB list no longer travels inside a task/result/retirement owner. |
| Allocation and capacity | `ensure_backing` uses `try_reserve_exact(DB_IO_LIST_ITEMS)`, then converts the vector to boxed storage (`:1857-1865`); `push` indexes the exact backing and rejects the max-plus-one item (`:1868-1876`). | Closed for the original inline owner. Allocation failure is a typed `Unavailable`, not an implicit stack allocation. |
| Admission ordering | List tasks pre-reserve task plus two list backings (`:2172-2179`), only then allocate the first backing (`:2182-2187`). Backend/page-admission failures release an unstarted list backing before detaching its aggregate credit (`:3290-3304`). | Closed. There is no uncredited admitted-list interval on these rejection paths. |
| Result/Drop retirement | The list closes each element then result handback (`:1891-1907`); `Drop` transfers an unfinished owner to the bounded lost-owner retirement path and restores it if parking fails (`:1974-1984`). `DbIoResult::List` delegates that close step (`:2303-2326`). | Source-closed. The exact terminal handle is retained until return. |
| Direct law | `db_io_list_keeps_exact_capacity_off_worker_stacks_and_in_the_ledger` verifies empty initial backing, two-backed credit, reserve/release, max-plus-one, ledger restoration, and task/result/lost-owner size ceilings (`:8180-8221`). Sol records its exact run as one pass in the existing journey report. | Strong source law, coordinator-runtime witnessed. It does not by itself drive a real worker or browser. |

## Remaining stack census

None of these is the removed `[u64; 4096]` owner. They are current watchpoints, not evidence of a repeat crash.

1. `DbIoPageArenaState::new()` still constructs 1,024 page slots, 1,024 free indexes, and 2,048 retirement entries in a `OnceLock` initializer (`storage/🦀️.rs:351-380`). `DbIoOperationLedger::new()` similarly constructs 128 ledger slots plus free indexes (`:123-142`), and `DbIoBackendRegistry::new()` constructs 64 registry entries (`:2411-2450`). They are global bounded registries after initialization, but their lazy constructors have no current source size/first-touch-on-default-worker law. The reported full process journey is reassuring runtime evidence, not a proof of all first-touch placements.
2. `DbIoPageWriter`/`DbIoPages` retain 64 page-lease options (`:520-532`, `:1470-1481`). The current direct law bounds task/result/lost-owner shell sizes, while page bytes live in static page backings (`:330-340`), not a task stack.
3. Production codec scratch buffers are 4 KiB, not retained 32 KiB lists: snapshot descriptor text at `📸️snapshot/🦀️.rs:422-440` and WAL protocol-field decoding at `🔄️sync/🦀️.rs:89-99`. The hub's 4 KiB raw HTTP chunk is test-only (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5871-5904`). No current evidence connects these bounded scratch buffers to the earlier admin startup abort.
4. The large page and platform byte arrays are process-static backing stores, not stack owners (`storage/🦀️.rs:330-340`, `:1218-1232`). They should not be “fixed” by enlarging worker stacks.

## Current runtime frontier

### What the registered journey actually proves

`os-hub:admin-live-journey-check` is non-cached in `🌎️hub/📦️packages/🦀️rust/📋️project.json:95-101` and is registered as `⚖️gate🎭admin-live-journey🌎️hub` in `.vscode/launch.json:4422-4429`.

The script:

1. validates the versioned admin and idle-admission fixtures using independent Bun/AJV/Buffer checks, including exact frame-prefix/payload equality, EN/DE inventory, and 8 KiB intent bounds (`📜️script.ts:2124-2144`);
2. discovers exactly one FQN before executing each selected Rust law (`:3117-3129`);
3. builds the SPA and production `os-hub` binary (`:3130-3132`);
4. starts a direct child with inherited fd 3 and an isolated loopback profile (`:584-680`), waits for directory readiness, issues an `admin-relay` credential, and starts a private relay (`:2164-2177`);
5. uses Chromium to consume and clear the one-use URL proof (`:2186-2191`), then asserts a bounded SQLite overview, durable `create-space`, operation acceptance/poll/cancel, EN/DE labels, bounded 64 KiB API reads, and the persisted `directory.db` (`:2193-2262`).

The loopback profile explicitly sets `OS_HUB_STORAGE_BACKEND=fs` and `OS_HUB_DIRECTORY_BACKEND=sqlite` (`:608-620`). That is correct for the stated admin/directory SQLite claim, and it also places the repaired generic filesystem DB I/O bridge on the process path. It is not an all-storage-SQLite claim.

`read_admitted_frame` leaves an idle listener cancellable, starts the 15-second deadline only at first admitted byte, and bounds remaining prefix/body (`🌎️hub/🔐️🏗️local-bootstrap/🦀️.rs:826-859`). The exact Rust law covers idle past the old deadline, late partial frame failure, and cancellation (`:994-1043`). This supersedes the historical startup timeout finding.

### The exact coverage gap

The registered law array contains only:

```text
local_bootstrap::tests::local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded
directory::sqlite::tests::projection_rebuild_preserves_live_credential_invite_and_session_bindings
```

at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:3120-3128`. It omits:

```text
db_storage::db_io_retained_fixtures::db_io_list_keeps_exact_capacity_off_worker_stacks_and_in_the_ledger
```

whose source is at `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8180-8221`. Sol’s report records that law as separately green before the full journey. This is sufficient historical coordinator evidence, but it is not a single registered current-tree proof after any later DB refactor.

### Outside the SQLite boundary

`admin-backend-check` runs six portable exact laws before its PostgreSQL law, then intentionally executes the latter (`📜️script.ts:3084-3113`). The source records that PostgreSQL runtime requires Docker (`:2296-2303`). Therefore a missing Docker socket is an honest external terminal for the broader all-feature boundary, not a reason to mark the local SQLite journey red or to skip the PostgreSQL law. No PostgreSQL, Neo4j, or OIDC pass is claimed here.

## Smallest dependency-ordered packet

### P0 — make the repair part of the registered SQLite evidence

Extend `AdminLiveJourneyCheckScript` rather than adding a parallel ad-hoc command:

- Add the fully qualified DB heap/ledger law to its exact-one list-and-run loop. Keep the existing exact-line check and `--exact --test-threads=1` behavior; do not use a substring filter or a larger `RUST_MIN_STACK`.
- Add a focused DB law that first-touches the operation ledger, page arena, backend registry, and a real `WalList` task through the ordinary default WorkerPool. It must return/close the 4,096-item output and show the pre/post aggregate ledger equality. If the test exercises a small default thread, join it and fail on panic. It must not hide the test behind a custom larger stack.
- If this first-touch law reproduces a stack issue, move only the fixed registry state arrays to exact heap-backed fixed-capacity owners, retaining their capacity and atomic admission semantics. Do not change worker-stack configuration. If it does not reproduce, retain the source watchpoint and avoid speculative migration.

### P1 — run the one honest acceptance command when the Cargo lane is free

Run the registered launch command without cache:

```sh
bun nx run os-hub:admin-live-journey-check --skip-nx-cache
```

Acceptance requires all of the following terminal evidence from one unchanged tree:

- neutral AJV/Buffer oracle, including invalid fixture rejection and EN/DE/size checks;
- exact-one selection and execution of bootstrap, SQLite projection-rebuild, and DB heap/ledger laws;
- SPA build and `cargo build --bin os-hub` with default `sqlite` feature;
- actual direct-child protected local bootstrap, actual relay cookie/proof boundary, and actual headless Chromium flow;
- readiness, SQLite `directory.db`, overview, durable typed create, bounded page response, operation progress/cancel terminal, EN and DE selection, and bounded shutdown.

### P2 — retain explicit nonclaims

Do not fold the Docker-dependent PostgreSQL law into P1, and do not add an environment-based skip. Do not claim Neo4j or production OIDC. A later dedicated backend/identity packet must run those real adapters separately.

## Acceptance classification

| Surface | Classification | Reason |
| --- | --- | --- |
| Original `[u64;4096]` task/result cause | Source-closed | Heap-backed `DbIoU64List`, exact credit, rollback, close/Drop, and direct source law. |
| SQLite directory bilingual local journey | Coordinator-runtime witnessed | Existing registered session `30455` is recorded exit 0; current source still maps to that gate. This audit did not re-run it. |
| Registered post-stack proof completeness | Source-RED / small P0 | The journey omits its own direct DB heap/ledger law and does not first-touch-test remaining lazy registry constructors on a normal worker. |
| PostgreSQL / Neo4j / OIDC | Out of scope / unaccepted | PostgreSQL is Docker-dependent and terminal by design; the local path has no OIDC claim. |

