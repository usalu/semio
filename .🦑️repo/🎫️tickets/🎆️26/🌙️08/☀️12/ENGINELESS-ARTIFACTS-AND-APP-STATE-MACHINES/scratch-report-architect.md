# 🏛️architect — artifact `⚙️engine` dissolution

Ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES (#2553).
Crate: `semio-s-plugin-architect`.

## (a) Summary row

| engine dir | LOC before | destinations |
|---|---|---|
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` | 3,600 across 11 files | `🧬️schema/🦀️component.rs` (pure doc helper) · `🧬️schema/💡️inferences/🦀️component.rs` (derived reads) · `🚪️io/🦀️component.rs` (`io_registry`) · `🎛️apps/🏛️architect/🦀️component.rs` (`🔌️Registration` + `🔧️Behavior`) · **deleted** (`ProgramEngine`) |

Concurrency pre-check: last commit touching the engine dir was `fd01661f06`; HEAD at dispatch
`382ace1b27` was 2 commits ahead with neither touching it. Quiescent — proceeded.

## (b) Every source item and where it went

Classification rule applied per item: read-only `fn(&ProgramSnapshot, …) -> Value` → `💡️inferences`;
`&mut ProgramSnapshot` → app `🔧️Behavior`; no-`&mut` doc primitive → `🧬️schema` root.
Several files **split** across two destinations rather than moving whole.

| # | engine file (LOC) | item(s) | destination |
|---|---|---|---|
| 1 | `🦀️component.rs` (233) | flat `pub use` hub re-exports | **deleted** — hub had no content of its own |
| 1a | | `register()`, `register_architect_exports()`, `register_artifact_schema()`, `register_artifact_inference()`, `register_pilot_languages()` | app `//#region 🔌️Registration` — **live**, not dead (see deviation D1) |
| 1b | | `pub struct ProgramEngine` + `new` + `into_snapshot` | **DELETED outright** (rule 1 — see below) |
| 1c | | `pub mod io_registry` (real, `&'static [ComposerEntry]`) | `🚪️io/🦀️component.rs` `//#region 🚪️DerivedIoRegistry` |
| 2 | `↔️adjacency` (180) | `normalize_pair` | `🧬️schema/🦀️component.rs` `//#region 🔖️DocumentHelpers` |
| 2a | | `AdjacencyMatrix`, `AdjacencyCell`, `adjacency_matrix`, `undirected_edges` | `💡️inferences` `//#region 🔀️AdjacencyViews` |
| 2b | | `AdjacencyConflict`, `detect_adjacency_conflicts`, `separation_incompatible` | `💡️inferences` `//#region ⚡️AdjacencyConflicts` |
| 2c | | `set_adjacency`, `clear_adjacency` (`&mut`) | app `🔧️Behavior` `//#region ↔️AdjacencyMutations` |
| 3 | `✅️validate` (513) | `EntityIndex`, `build_entity_index`, `check_ref`, `validate_plugin` | `💡️inferences` `//#region ✅️Validate` |
| 4 | `🎁️outputs` (213) | `OutputKind`, `ProgramOutput`, `build_output` + 20 builders | `💡️inferences` `//#region 🎁️Outputs` |
| 5 | `📄️report` (402) | `ProgramReport`, `ReportSection`, `build_report` + 21 section builders, `timestamp` | `💡️inferences` `//#region 📄️Report` |
| 5a | | `build_report_and_record` (`&mut`, pushes `ReportRecord`) | app `🔧️Behavior` `//#region 📄️ReportRecord` |
| 6 | `📊️status-summary` (185) | `StatusSummary`, `RegisterStatusCount`, `bump_status`, `bump_validation`, `status_summary` | `💡️inferences` `//#region 📊️StatusSummary` |
| 7 | `📐️template` (416) | `TemplateApplyResult`, `apply_template` (`&mut`) | app `🔧️Behavior` `//#region 📐️Template` |
| 8 | `📤️exchange` (538) | `REGISTER_ROW_COLUMNS`, `RegisterCsvRow`, `export_json`, `import_json`, `export_registers_csv`, `export_relationships_csv`, `export_registers_tsv`, `csv_record`, `rows_to_*_snapshot`, `collect_rows`, `header_row` | `💡️inferences` `//#region 📤️ExchangeReads` |
| 8a | | `MergeStrategy`, `import_registers_csv`, `import_registers_tsv`, `import_rows`, `csv/tsv_snapshot_to_rows`, `register_contains`, `remove_register_item`, `upsert_*` (all `&mut`) | app `🔧️Behavior` `//#region 📤️ExchangeImport` |
| 9 | `🔍️search` (219) | `SearchQuery`, `SearchHit`, `search_plugin`, `merge_query`, `push_if_match` | `💡️inferences` `//#region 🔍️Search` |
| 10 | `🔬️analyze` (480) | `AnalysisResult`, `AnalysisMetric`, `run_analysis` + 20 analyzers, `risk_score`, `priority_weight` | `💡️inferences` `//#region 🔬️Analyze` |
| 10a | | `run_analysis_and_record` (`&mut`, pushes `AnalysisRecord`) | app `🔧️Behavior` `//#region 🔬️AnalysisRecord` |
| 11 | `🧭️trace` (221) | `AuditTrail`, `audit_trail`, `resolve_supersedes` (read-only) | `💡️inferences` `//#region 🧭️TraceReads` |
| 11a | | `TraceChain`, `ImpactTrace`, `trace_chain`, `trace_links_for`, `trace_impact`, `add_trace_link`, `embed_requirement_traces`, `follows_kind_chain`, `trace_adjacency` (all `&mut`) | app `🔧️Behavior` `//#region 🧭️Trace` |

### Rule-1 finding: `ProgramEngine` deleted
Verified repo-wide across `✏️s` and `🧰️framework`: `ProgramEngine` had **0 references outside its own
file**, and `trait ArtifactEngine` / `impl … ArtifactEngine for …` had **0 hits in shipped source**
(the only `ArtifactEngine` hit repo-wide is in `🪐️space`, unrelated). No exception raised — struct and
both methods deleted outright as instructed.

## (c) Unqualified paths found and how qualified

The shadowing hazard is **real in this plugin** and was hit exactly as briefed. The artifact root
`🗿️artifacts/🏛️program/🦀️component.rs` carries its own thin `io_registry` whose `entries()` returns
`&'static [&'static ComposerEntry]` — a *different type* from the real registry's
`&'static [ComposerEntry]`.

| bare/stale path | found in | fully qualified to |
|---|---|---|
| `crate::artifacts::program::standards::v1::engine::io_registry as v1` | artifact root `io_registry` shim | `crate::artifacts::program::standards::v1::subsets::any::io::io_registry as v1` |
| `crate::artifacts::program::engine::adjacency::normalize_pair` | artifact root `sample_plugin()` | `crate::artifacts::program::standards::v1::subsets::any::schema::normalize_pair` |
| `crate::artifacts::program::engine::adjacency::adjacency_matrix` | moved report body | `adjacency_matrix` (now same-module in `💡️inferences`) |
| `crate::artifacts::program::engine::adjacency::normalize_pair` | moved exchange `upsert_adjacency_stub` | `…::any::schema::normalize_pair` via module-level `use` |

Every moved `io_registry` reference is fully qualified; no bare `io_registry::entries()` was
introduced anywhere reachable from artifact-root code. A doc note recording the two-types trap was
added above the relocated `//#region 🚪️DerivedIoRegistry`.

### External call sites repointed (14 files)
`🗂️catalog` (`normalize_pair`, `AnalysisResult`, `ProgramReport`), `🎚️config` (`ProgramReport`,
`SearchQuery`), `📌️panels/📄️artifact` (`status_summary`), `🪟️windows/↔️adjacency`
(`adjacency_matrix`, `detect_adjacency_conflicts`), `🪟️windows/🕸️graph` (`undirected_edges`),
`🪟️windows/📄️report` (`build_report`), `🪟️windows/🧭️trace` (`trace_chain`/`trace_impact`/`TraceChain`
→ app behavior; `audit_trail` → inferences — **this file split across both destinations**),
`🎮️commands/🔬️analysis` (`validate_plugin`, `run_analysis`, `build_report`),
`🎮️commands/📐️template` + `🎮️commands/📋️register` (`apply_template` → app behavior),
`🎮️commands/🔍️search` (`search_plugin`, `SearchQuery`), `🎮️commands/📤️exchange`
(`export_registers_csv` → inferences, `import_registers_csv`/`MergeStrategy` → app behavior —
**also split**), app root test (`export_registers_csv`), and both `🔗🧲connect-adjacency`
mutation leaves (`🔺️diff`, `↩️inverse` → `normalize_pair`).

## (d) Assertion count — before vs after

**Before (11 engine files, summed = 45):**

| file | asserts |
|---|---|
| `🦀️component.rs` | 0 |
| `↔️adjacency` | 4 |
| `✅️validate` | 4 |
| `🎁️outputs` | 2 |
| `📄️report` | 5 |
| `📊️status-summary` | 3 |
| `📐️template` | 5 |
| `📤️exchange` | 8 |
| `🔍️search` | 4 |
| `🔬️analyze` | 6 |
| `🧭️trace` | 4 |
| **total** | **45** |

**After (destination files, measured against dispatch commit `382ace1b27`):**

| destination | before | after | delta |
|---|---|---|---|
| `🧬️schema/🦀️component.rs` | 0 | 1 | **+1** |
| `🧬️schema/💡️inferences/🦀️component.rs` | 2 | 31 | **+29** |
| `🎛️apps/🏛️architect/🦀️component.rs` | 47 | 62 | **+15** |
| | | **sum of deltas** | **+45** |

**45 in, 45 out — every assertion survived**, none dropped, none duplicated. Test modules were split
to travel with their code (e.g. `🧭️trace`'s 4 asserts split 1 → `💡️inferences` `tests_trace`,
3 → app `🔧️Behavior` tests; `📤️exchange`'s 8 split 3 → inferences, 5 → app).

## (e) Compiler output and per-error attribution

> **Status: one real run COMPLETED, at commit `62152fabcc`, before the coordinator's stop-waiting
> notice arrived. It is not a never-ran build, so it is reported as observed rather than as
> `UNVERIFIED`. It is, however, a point-in-time result — the tree has moved since (35 concurrent
> cargo processes, shared target dir), so treat it as provisional and superseded by the central
> `cargo check --workspace --all-targets --keep-going`. No further run was started from this lane;
> all my cargo/monitor tasks are stopped.**
>
> The structural checks in the final section need no compiler and were **re-confirmed after** that
> run, on the current tree.

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-architect --all-targets
EXIT=101
error: could not compile `semio-s-plugin-architect` (lib test) due to 34 previous errors; 28 warnings emitted
```

**Pre-flight gate (per coordinator, needs no compiler):** dangling `#[path]` count went **11 → 0**.
See deviation D3 — the initial dissolution left `📦️glue.rs` mounting the eleven deleted files, which
is `os error 2` and made the crate unbuildable; that is now fixed.

### All 34 errors, attributed

| file | count | last commit | mine? |
|---|---|---|---|
| `🧬️schema/🧬️mutations/🦀️component.rs` (lines 340–563) | 31 | `a445617cae` | **No** |
| `🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (lines 38, 44) | 2 | `a445617cae` | **No** |
| `🧬️schema/💡️inferences/🧭topology/🦀️component.rs` (line 170) | 1 | `a46ac1f883` | **No** |

**Zero errors in any file I edited** (verified by grepping every error location against my touched-file
list). Both `a445617cae` and `a46ac1f883` predate even the engine directory's own last commit
`fd01661f06`, so all three files were already broken before this ticket began.

**Foreign error class 1 — 33 × E0433, semantic-mutations lane (#2545).** Test modules reference leaf
slug modules through a `super::` chain that is one level short of where `📦️glue.rs` actually mounts
them (mutations root file is mounted as `mod component` at glue `:89`, so `super` from its
`mod tests` at `:312` resolves to `…::mutations::component`, not `…::mutations`). Verbatim:

```
error[E0433]: cannot find `create_stakeholder` in `super`
   --> .../🧬️schema/🧬️mutations/🦀️component.rs:340:64
340 | let create = ProgramMutation::CreateStakeholder(super::create_stakeholder::mutation::CreateStakeholder { … });
    |                                                        ^^^^^^^^^^^^^^^^^^ could not find `create_stakeholder` in `super`
help: consider importing this module
    | use crate::artifacts::program::mutations::create_stakeholder::mutation;
```

The slug dirs all exist on disk (268 of them) and are all mounted (801 mount lines) — it is purely
the `super::` depth in the test modules. Owned by SEMANTIC-MUTATIONS-OVERHAUL (#2545), not touched.

**Foreign error class 2 — 1 × E0063, inference-schema-family lane.** Verbatim:

```
error[E0063]: missing fields `acoustic_class`, `adjacency_preferences`, `circulation_role` and 9 other fields
  in initializer of `program::…::registers::ProgramElement`
   --> .../🧬️schema/💡️inferences/🧭topology/🦀️component.rs:170:9
170 |         ProgramElement {
```

A test fixture built before `ProgramElement` grew 12 fields. Owned by
INTRODUCE-INFERENCE-SCHEMA-FAMILY, not touched.

**Note on the briefing's expected baseline:** the briefed "~254 errors from a `registers::*` alias typo
in `📦️glue.rs`" is **gone** (another session fixed it), and the briefed foreign error
`ProgramInference::infer not in scope` in `💡️inferences/🦀️component.rs` is **also gone** — that file
now compiles clean with my 29 added assertions in it. Re-derived rather than trusted, as instructed.

## (f) Deviations

**D1 — `register*()` is live, not dead.** The exemplar's `register()` was superseded by
`declaration()`. Architect has **no `declaration()` at all** (`grep -rn "fn declaration"` over the
plugin → 0 hits), and `register_architect_exports` is still wired via
`Plugin::builder(…).setup(crate::register_architect_exports)` in the plugin root. So rule 6's "delete
if dead" did **not** apply; it moved to the app's `//#region 🔌️Registration` as app-scope wiring.
Consequently **the artifact-root `declaration()` `.composers(…)` fix-up in the packet was N/A** —
there is no `declaration()` to fix. Its equivalent, the artifact root's `io_registry` shim, was
repointed to the real `🚪️io::io_registry` instead.

**D2 — behavior parked on app top-level, not in `🎛️apps/🏛️architect/⚙️engine`.** That reserved
directory exists but is **empty**, and `📦️glue.rs` never mounts it (its only two `pub mod engine`
blocks were both in the artifact-tree section). Populating it would have produced dead, uncompiled
code. Per the packet's own fallback, behavior went to the app's `//#region 🔧️Behavior`. The reserved
directory is left empty and untouched. **No state machine was invented** — every relocated item is
the same plain function/struct it was, only moved.

**D3 — `📦️glue.rs` was edited, against the original scope line.** My packet listed `📦️glue.rs` as
do-not-touch. Deleting the engine directory without updating it left 11 `#[path]` mounts pointing at
non-existent files (`os error 2`), which makes the crate unbuildable and reportedly broke
`bun ./📜️script.ts policy` for other sessions. The coordinator issued an explicit mid-task correction
to finish the job. Changes were kept minimal and surgical — `git diff 382ace1b27 HEAD` on that file is
**1 insertion, 30 deletions**, entirely:
1. removed the `pub mod engine { … }` block (11 mounts + wrapper) — every module was a *delete*, not a
   repoint, because all four destination files were already mounted elsewhere
   (`:69` schema, `:88` inferences, `:2520` io, `:2705` apps);
2. removed the legacy shim `pub mod engine { pub use super::standards::v1::engine::*; }`;
3. repointed `pub use artifacts::program::engine::register_architect_exports;` →
   `pub use apps::architect::register_architect_exports;`.

Nothing near the mutations mounts was touched — confirming the 34 errors are not collateral from this
edit. **Caution honoured:** the `🎛️apps/🏛️architect/…` directories sharing names with engine topics
(`↔️adjacency`, `📄️report`, `📐️template`, `📤️exchange`, `🔍️search`, `🧭️trace`) are **pre-existing app
command/window directories, not relocations** — nothing of mine went into them, and no mount was
pointed at them.

## (g) Files touched

**Deleted (11 files + 11 dirs):** the whole
`🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` tree (no `Cargo.toml` under it — verified empty before `rm -rf`).

**Modified (17):**
1. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️component.rs`
2. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
3. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
4. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
5. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗🧲connect-adjacency/🔺️diff/🦀️component.rs`
6. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗🧲connect-adjacency/↩️inverse/🦀️component.rs`
7. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🦀️component.rs`
8. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🗂️catalog/🦀️component.rs`
9. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎚️config/🦀️component.rs`
10. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/📌️panels/📄️artifact/🦀️component.rs`
11. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🦀️component.rs`
12. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️component.rs`
13. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/📄️report/🦀️component.rs`
14. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️component.rs`
15. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🔬️analysis/🦀️component.rs`
16. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/📐️template/🦀️component.rs`, `…/📋️register/🦀️component.rs`, `…/🔍️search/🦀️component.rs`, `…/📤️exchange/🦀️component.rs`
17. `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs` *(deviation D3)*

**Untouched:** `📜️script.ts`, `🔣️taxonomy.json`, all `AGENTS.md`, `✏️s/🔌️plugins/🗄️stdio`,
`🧬️schema/🗄️registers`, `🧬️schema/🧱️kernel`, `🎛️apps/🏛️architect/⚙️engine` (reserved slot),
and all other plugins.

## Structural verification (needs no compiler — re-confirmed on the current tree)

```
find 🏛️architect -path "*🗿️artifacts*" -name "⚙️engine" -type d ....... 0
dangling #[path] mounts in 📦️glue.rs ................................ 0   (was 11)
grep "::engine::|standards::v1::engine|subsets::any::engine" ......... 5   (all out-of-scope, enumerated below)
🎛️apps/🏛️architect/⚙️engine reserved slot ........................... present, empty, untouched
```

### The 5 grep hits, enumerated and categorised
All five are **`stdio_csv::engine::` — real code, but references into `semio-s-plugin-stdio`'s
still-standing engines, which are out of scope for this ticket.** **Zero** hits reference architect's
own dissolved engine.

| # | location | category |
|---|---|---|
| 1 | `🎛️apps/🏛️architect/🦀️component.rs:624` — `stdio_csv::engine::decode_csv_with` | real code → stdio (out of scope) |
| 2 | `🎛️apps/🏛️architect/🦀️component.rs:1082` — `stdio_csv::engine::decode_csv_with` | real code → stdio (out of scope) |
| 3 | `💡️inferences/🦀️component.rs:2186` — `stdio_csv::engine::encode_csv` | real code → stdio (out of scope) |
| 4 | `💡️inferences/🦀️component.rs:2197` — `stdio_csv::engine::encode_csv` | real code → stdio (out of scope) |
| 5 | `💡️inferences/🦀️component.rs:2321` — `stdio_csv::engine::decode_csv_with` | real code → stdio (out of scope) |

Two further stdio references use an aliased import (`…::tsv::standards::iana::engine as
stdio_tsv_engine`, at app `:173` and inferences `:33`) and so do not match the `::engine::` pattern —
also real code into stdio, also out of scope.

Separately, a sweep for **any** mention of architect's own engine (`program::engine`, `⚙️engine`),
including prose, returns only provenance doc-comments I wrote ("Dissolved out of the former
`⚙️engine` …") in the four destination files. One genuinely stale header — `🚪️io/🦀️component.rs:2`,
which still read *"called once from ⚙️engine::register"* — was found during this final sweep and
corrected to `crate::apps::architect::register`.

### Note on the assertion "before" baseline
`git show HEAD:<path>` is **not** a valid before-image here: this repo auto-commits, so HEAD
(`62152fabcc`) already contains my work (it reports 31 asserts in `💡️inferences`, the *after* value).
The before-column was therefore taken from the **dispatch commit `382ace1b27`**.

---

**VERDICT: PASS on structure; compile provisional.**
Artifact `⚙️engine` fully dissolved and deleted (0 dirs, 0 references), `📦️glue.rs` dangling mounts
**11 → 0**, all **45** assertions preserved (45 in / 45 out), reserved app slot intact. The one
completed compile run showed **zero errors in any file this lane touched**; its 34 errors all sit in
three files last modified before this ticket began, belonging to the SEMANTIC-MUTATIONS-OVERHAUL (33)
and INTRODUCE-INFERENCE-SCHEMA-FAMILY (1) lanes — pending confirmation by the central workspace check.
