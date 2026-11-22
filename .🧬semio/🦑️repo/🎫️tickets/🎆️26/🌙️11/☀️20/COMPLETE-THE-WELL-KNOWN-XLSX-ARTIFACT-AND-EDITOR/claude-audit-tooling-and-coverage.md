# 🔍️ Tooling Usability and XLSX Coverage Audit

Read-only audit. No source file was edited and no git command was run. Snapshot taken 2026-11-22 around 19:59 local time.

Abbreviations: `ART` = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx`, `SUB` = `ART/🏅️standards/🔖️ecma-376/🪆️subsets`.

---

## A) Shared repo tooling

Commands run from the repo root with a 180 s bound: `repo doctor`, `repo fleet status`, `repo leases status`, `repo capacity status`.

### Verdict

Usable for local, selection-scoped work. Not healthy for whole-workspace queries. `repo doctor` reports **degraded** (0 errors, 8 warnings, 1 note). Nothing is deadlocked, but the graph has never been published and several fleet workers are stale.

### Graph

- **No project graph generation is published.** `repo doctor`, `repo fleet status` ("Generation: none published") and the graph probe all agree. Whole-workspace questions cannot be answered from a sealed graph. Starting the first read is `repo graph --graph-policy fresh`, which takes hours on a cold checkout and should be started once, outside a task.
- Graph loads are **not stuck on the local host**. The graph probe reads in 18 ms and "Graph: no builder; 0 computer(s), 0 waiting".
- Graph-related debris: 2 dead per-process publications (`p-<pid>`) hold 155 521 024 bytes. These are never collected by the optional quota. Safe repair is `repo doctor --fix --only=caches`. **Not run** (read-only audit).

### Memory and capacity

- Host: 6 734 of 16 384 MB free, 906 MB above the 2 048 MB floor. Pressure **normal**, swap 785 MB, load 11.49 on 10 cores.
- Capacity grants: 2 granted, 0 waiting. Scheduler has 0 queued.
- **Over-reservation:** pid 12026 (`repo cargo ... test --manifest-path ✏️s/🔌️plugins/🗄️stdio…`, which is this XLSX artifact) holds a 4 174 MB reservation but observes only 394 MB. Pid 12342 (`repo run @semio-tech/s-dev-composition:check-stdio-format-claims`) waited 8 s for capacity behind it.
- Fleet-wide: g1-m02 and g1-m03 report `pressure warn` and refuse work for lack of headroom. g1-m07, g1-m08 and g1-m11 refuse for memory or queue order.
- Storage: host free 61.1 GiB (floor 13.5 GiB). Machine budget 58.5 of 198.3 GiB used.

### Leases

- 29 leases held by four processes, 0 waiting, 0 deadlock cycles, 10 recent order violations.
- pid 9235 (`repo dev mcp stdio os`, a long-running dev MCP server) holds 21 leases for about 150 s under `cargo:artifact-plan`. This includes an exclusive `cargo-target-recipe-v1` and five `semio.compiler.cpu/v1/*` slots.
- pid 12294 (`bun`) holds native-fixed-image, rustup and cargo locks for about 13 s.
- pid 12431 (`repo synchronize --manifest …/📕️xlsx/Cargo.toml`) holds rustup and cargo-preparation locks for about 8 s.
- **Lock-order violations (20 total):** `semio.compiler.cpu-picker/v1` (rank 3) is requested while `artifact:…/.cache/b/p/m/infinite/prototype/fonts` (rank 6) is held shared. The tool itself flags this as deadlock-prone under load.

### Fleet and workers

- 11 machines reporting. Workers: 4 ready, 5 working, 0 idle, 1 silent, 2 offline, 2 without a record.
- g1-m04 (linux): offline 4.8 h in stage `dependencies`. It needs 6 144 MB free in `/home/deck/semio/.cache` but only 5 762 MB is free. Stale.
- g1-m05 (windows): offline 32 min, backoff in `dependencies`. Stale.
- g1-m03: silent 95 s in backoff. Failing with exit 1.
- g1-server and g1-m09: enrolled, no readiness record, never seen alive.
- g1-m12 (workspace stage) and g1-m13 (dependencies stage) are in retry backoff after exit-status-1 failures.

### Dependencies, caches, tools

- 2 unfinished dependency generations of dead builds. The oldest is 6 150 s old. Retire with `repo cache gc`, which also runs on idle; not run.
- Tools not provisioned on this machine: cmake, docker, ffmpeg, gh, ninja. Fix with `repo tools setup all`; not run.

### Processes holding leases (summary)

| pid | role | leases | age | note |
|---|---|---|---|---|
| 9235 | `repo dev mcp stdio os` | 21 | 150 s | cargo:artifact-plan, about 3 min left |
| 12294 | `bun` | 5 | 13 s | session a231diuifvm |
| 12431 | `repo synchronize` (xlsx Cargo) | 6 | 9 s | session a231diuifvm |
| 12026 | `repo cargo test` (xlsx) | 0 leases, 4 174 MB grant | 25 s | blocks 12342 |
| 12342 | `repo run check-stdio-format-claims` | 0 | 14 s | waiting for capacity |

---

## B) XLSX feature coverage

Legend: ✅ present, ⚠️ partial or weak, ❌ absent.

Columns:
1. Fixture = language-agnostic JSON fixture or xlsx before/after pair.
2. Oracle = third-party cross-check actually executed.
3. UI = editor or viewer surface in TS or Rust, with en/de strings.

### Declared surface

- `ART/📜️artifact-definition.json`: 27 mutations, all `status: implemented` but `executable_registration: false`. Standard `ecma-376` is `unverified` with `normative_source: null`. `support_ledger` read/write/lossless/canonical are all `opaque`. `conformance_suites` entry `calamine-oracle` is `unimplemented`. `resources.external-references` is `unimplemented`. Profiles: `any` (`implemented`). Dialects: `strict` and `transitional` in `representations`.
- `ART/🧩️composition/🔣️.json`: 6 apps (base and strict and transitional, editor and viewer each), 3 playgrounds, 3 mutation descriptors.
- `SUB/🔣️.json`: subsets `base`, `strict`, `transitional`.

### Mutation matrix

| # | Subset | Mutation | (1) Fixture | Feature row | (2) Third-party oracle | (3) UI and en/de |
|---|---|---|---|---|---|---|
| 1 | base | set-cell | ✅ `🧱️base/🧪️tests/🧫️fixtures/✍️set-cell-applied/{⬅️before,➡️after}.xlsx`, `🧬️mutations/✍️set-cell/🧮️widens/📸️snapshot/*.json` | ✅ Examples, mode-differential | ✅ calamine read plus rust_xlsxwriter write (`🔮️oracles/🦀️.rs`, `🧮️mutation-dispatch`) | ✅ base editor (`✏️editor/🦀️.rs` line ~305/385). ⚠️ en/de: locale-aware number parse, tests in `✏️editor/🧪️tests/🔬️unit/🦀️.rs:508-563` |
| 2 | base | insert-cell | ⚠️ JSON only: `🧬️mutations/➕️insert-cell/🧾️wire-witness/**/*.json`, no xlsx pair | ❌ no Examples row (`mutate-xlsx-ecma-376/🥒️.feature`) | ❌ `mutation-dispatch` has no arm, falls through to "no oracle implementation" | ⚠️ base editor uses `InsertCell` (line ~394). en/de only for the 3 generic labels |
| 3 | base | remove-cell | ✅ `🧽️remove-cell-applied/*.xlsx` | ✅ | ✅ calamine + rust_xlsxwriter | ❌ no remove UI |
| 4 | base | insert-sheet | ✅ `➕️insert-sheet-applied/*.xlsx` | ✅ | ✅ | ❌ no sheet UI |
| 5 | base | remove-sheet | ✅ `➖️remove-sheet-applied/*.xlsx` | ✅ | ✅ | ❌ |
| 6 | base | rename-sheet | ✅ `🏷️rename-sheet-applied/*.xlsx` | ✅ | ✅ | ❌ |
| 7 | base | set-shared-string | ✅ `🔤️set-shared-string-applied/*.xlsx` | ✅ round-trip and property only | ❌ own `🧵️shared-string-pool` module, not third-party | ❌ |
| 8 | base | insert-shared-string | ✅ `📥️…-applied/*.xlsx` | ✅ round-trip and property only | ❌ same | ❌ |
| 9 | base | remove-shared-string | ✅ `📤️…-applied/*.xlsx` | ✅ round-trip and property only | ❌ same | ❌ |
| 10 | base | set-content-type | ✅ 4 pairs under `📇️set-content-type-*` | ⚠️ not in Examples; only oracle unit test `🔮️oracles/🧪️tests/🔬️unit/🦀️.rs:183` | ⚠️ own plumbing on `quick-xml`, not an independent XLSX reader | ❌ |
| 11 | base | remove-content-type | ✅ `🧺️remove-content-type-*` (2 pairs) | ⚠️ as above | ⚠️ as above | ❌ |
| 12 | base | set-relationship | ✅ 4 pairs under `📎️set-relationship-*` | ⚠️ as above | ⚠️ as above | ❌ |
| 13 | base | remove-relationship | ✅ `🧷️remove-relationship-*` (3 pairs) | ⚠️ as above | ⚠️ as above | ❌ |
| 14 | strict | set-main-namespace | ✅ `🌐️set-main-namespace-applied/*.xlsx` | ✅ | ✅ quick-xml 0.42 + zip 6 (`🔒️strict/🔮️oracles/🦀️.rs`) | ❌ strict editor has only SetCell |
| 15 | strict | set-relationships-namespace | ✅ `🔗️…-applied/*.xlsx` | ✅ | ✅ | ❌ |
| 16 | strict | set-relationship-base | ❌ no xlsx pair | ✅ | ✅ | ❌ |
| 17 | strict | set-conformance-attribute | ✅ `✅️…-applied/*.xlsx` | ✅ | ✅ | ❌ |
| 18 | strict | remove-conformance-attribute | ✅ `🚫️…-applied/*.xlsx` | ✅ | ✅ | ❌ |
| 19 | strict | insert-vml-part | ✅ `✒️insert-vml-part-applied/*.xlsx` + `🧬️mutations/✒️insert-vml-part/🧾️owned-document/🔣️.json` | ✅ | ✅ | ❌ |
| 20 | strict | remove-vml-part | ✅ `🧹️remove-vml-part-applied/*.xlsx` | ✅ | ✅ | ❌ |
| 21 | strict | set-worksheet-content-type | ✅ `🏷️…-applied/*.xlsx` | ✅ | ✅ | ❌ |
| 22 | transitional | set-main-namespace | ✅ `🌐️…-applied/*.xlsx` | ✅ | ✅ quick-xml + zip (`🌉️transitional/🔮️oracles/🦀️.rs`) | ❌ transitional editor has only SetCell |
| 23 | transitional | set-relationships-namespace | ✅ | ✅ | ✅ | ❌ |
| 24 | transitional | set-relationship-base | ❌ no xlsx pair | ✅ | ✅ | ❌ |
| 25 | transitional | set-conformance-attribute | ✅ | ✅ | ✅ | ❌ |
| 26 | transitional | remove-conformance-attribute | ✅ | ✅ | ✅ | ❌ |
| 27 | transitional | set-worksheet-content-type | ✅ `🏷️…-applied/*.xlsx` | ✅ | ✅ | ❌ |

Notes on the matrix:
- Rows 14 to 27 are the OOXML conformance-class mutations. They exist only as ECMA-376 class stamps, so UI for them is arguably not needed. Still, they are not reachable from any editor surface.
- Feature row for transitional has scenario name `@id-mutate-stamp-strict-class` (`🌉️transitional/🧪️tests/🔀️mutate-xlsx-ecma-376-transitional/🥒️.feature`). This is a copy from strict and reads oddly in the transitional subset.
- Every fixture in `testEvidence` for strict and transitional is `class: handcrafted`. Base has a mix of `third-party-generated` (xlsx) and `handcrafted` (shared-string).

### Oracle inventory

| Oracle entry | File | Actually executed? |
|---|---|---|
| `xlsx-ecma-376-mutate` (calamine 0.36 + rust_xlsxwriter 0.96) | `SUB/🧱️base/🔮️oracles/🔣️.json`, `🔮️oracles/📦️packages/🦀️rust/Cargo.toml` | Yes, for 7 base kinds. Not for insert-cell, shared-string, or plumbing |
| `openpyxl-xlsx-ecma-376-mutate-reader` (openpyxl 3.1.5) | three `🔮️oracles/🔣️.json` files | **No.** Declared in JSON only, grep finds no code call site |
| `jszip-xlsx-ecma-376-opc-plumbing-reader` (jszip 3.10.1) | `SUB/🧱️base/🔮️oracles/🔣️.json` | **No.** Declared only |
| quick-xml 0.42 + zip 6 pairing | `SUB/🔒️strict/🔮️oracles/🦀️.rs`, `SUB/🌉️transitional/🔮️oracles/🦀️.rs` | Yes, for all 14 conformance kinds. No openpyxl cross-check |

### Editor and viewer surfaces

- Base editor (`SUB/🧱️base/✏️editor/🦀️.rs`, 446 lines): mutations used are `SetCell` and `InsertCell` only. The labels block (`app_labels!`, lines 55-61) has three strings in en and de: Row, Column, and "Unknown body". The number-format helpers (`xlsx_decimal_separator`, lines 72-140) are locale-aware and tested for de.
- Base viewer (`SUB/🧱️base/👁️viewer/🦀️.rs`, 157 lines): same three labels in en and de (`app_labels!` at line 39).
- Strict and transitional editor and viewer (`SUB/🔒️strict/✏️editor/🦀️.rs` and `🌉️transitional/✏️editor/🦀️.rs`, 285 and 287 lines, plus viewers at 117 lines each): no `app_labels!`, no German strings, no `Locale::De` tests. Their TS files are 10-line re-export twins only.
- TS files (`✏️editor/🟦️.ts`, `👁️viewer/🟦️.ts`) are re-exports, not UI components. No TS UI component carries en or de strings.

---

## Gaps and blockers, prioritised

### Tooling (repo)

1. **No graph generation is published.** Whole-workspace queries are blocked until `repo graph --graph-policy fresh` runs once (hours). `repo doctor`, `repo fleet status`.
2. **Capacity over-reservation.** Pid 12026 reserves 4 174 MB but uses 394 MB, which blocks pid 12342 for 8 s. `repo capacity status`.
3. **Lock-order violations (20).** `semio.compiler.cpu-picker/v1` (rank 3) is requested under `artifact:…/fonts` (rank 6). Deadlock risk under load. `repo leases status --violations=20`.
4. **Stale and dead fleet workers.** g1-m04 offline 4.8 h (disk short by 382 MB), g1-m05 stale 32 min, g1-m03 silent, g1-server and g1-m09 never reported. `repo fleet status`.
5. **Dead cache publications and unfinished dependency generations.** 155 MB held by dead processes, 2 dead dependency generations (6 150 s). Fix with `repo doctor --fix --only=caches` and `repo cache gc`. Not run.
6. **Unprovisioned tools.** cmake, docker, ffmpeg, gh, ninja missing. `repo tools setup all`. Not run.

### XLSX artifact

7. **insert-cell has no third-party oracle and no xlsx fixture or feature row.** `SUB/🧱️base/🔮️oracles/🧮️mutation-dispatch/🦀️.rs` has no arm for it. Only JSON wire-witness exists under `SUB/🧱️base/🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-cell/`.
8. **Shared-string kinds (set, insert, remove) have no independent oracle.** The base oracle uses its own `🧵️shared-string-pool` module. Feature marks them round-trip or property only, so no differential evidence exists.
9. **Declared oracles that never run.** `openpyxl` (three entries) and `jszip` (base) appear in `🔮️oracles/🔣️.json` files, but nothing imports them. Either wire them in or remove the declarations.
10. **Set and remove relationship and content-type use plumbing, not an independent reader.** `SUB/🧱️base/🔮️oracles/🔌️plumbing-and-projection/🦀️.rs` edits rows through quick-xml. Its fixtures are labelled `third-party-generated`, which overstates the independence. The `calamine-oracle` conformance suite in `ART/📜️artifact-definition.json` is still `unimplemented`.

Further gaps, lower priority, not in the top ten:
- `set-relationship-base` has no xlsx before/after pair in strict or transitional (`SUB/🔒️strict/🧪️tests/🧫️fixtures/`, `SUB/🌉️transitional/🧪️tests/🧫️fixtures/`).
- Transitional has no JSON fixtures at all.
- No UI for remove-cell, sheet operations, shared strings, relationships, content types, or conformance kinds. Base editor only sets and inserts cells.
- en and de coverage is three generic labels in base editor and viewer. Strict and transitional have none.
- `ART/📜️artifact-definition.json` marks all 27 mutations `implemented` while `executable_registration` is false for every one. Standard is `unverified` with no normative source, and the support ledger is all `opaque`.
- Transitional feature has a `@id-mutate-stamp-strict-class` scenario copied from strict.

---

## Commands run

- `repo doctor` (from repo root, 180 s bound): completed, degraded.
- `repo fleet status`: completed.
- `repo leases status`: completed.
- `repo capacity status`: completed.
- Read-only inspection of `ART` and `SUB` files with `find`, `grep`, `sed`, and a Python JSON reader. Temporary listing written only to the session scratchpad.

No repo command that modifies state was run. No file outside this report was written.
