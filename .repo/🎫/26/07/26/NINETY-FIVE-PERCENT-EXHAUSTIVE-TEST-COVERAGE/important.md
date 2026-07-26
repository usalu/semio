# Session conclusion: Phase A closed as "best achievable", Phase C wave 1 launched

**Decision: stopped chasing a 100% clean full-repo run after run 20 (of 20 attempts).** The last new failure
(`dsl_derive` proc-macro hitting `E0004: non-exhaustive patterns` on `FieldKind::VecBlockStatements`/
`MapField`, which did not exist in earlier runs) is from another concurrent Claude Code session actively
adding enum variants *right now* — this repo is under continuous live multi-session development, so a
zero-diff clean run is a moving target, not a fixed problem. Runs 1–20 fixed a long tail of real, dead
(non-moving) pre-existing bugs; see below and the historical section for the full list. Best snapshot: **138
coverage files (112 Rust, 5 Go, 21 JS), 53.90% partial repo-wide** (`aggregate-now.ts` output) — up from the
45.94%/98-file snapshot two sessions ago in this same ticket.

**Bugs fixed in this final push (beyond the ones already logged below):**
- `compose/client/lib/query/rs/lib.rs`: 7 more `operation`/`operator` rename-fallout sites (same bug class as
  everywhere else, 4th–5th files found) — `compose_query` now 4/4 tests passing.
- `ui/wgpu/rs/lib.rs`: 6 separate `pub mod` blocks (`paint`, `reconcile`, `events`, `shell`, `engine`,
  `widgets`) each independently missing `use crate::IconName;` — Rust modules don't inherit sibling imports,
  and this was invisible to a bare `cargo check` since most of these are `#[cfg(feature = "engine")]`-gated.
  Plus 3 further type-mismatch bugs the missing imports had been masking. `cargo test -p ui_wgpu --tests
  --features engine` now 164/164 passing. (Note: `framework/renderer/wgpu/rs`, a DIFFERENT/larger crate that
  depends on `ui_wgpu`, still has its own 20 separate errors — own follow-up task, not fixed.)
- `draw/rs/lib.rs`: `DocumentDsl` imported twice (once privately, once via an intentional `pub use` with its
  own doc comment explaining why) — `E0252` duplicate definition. Removed from the private-use list.
- `process/3d/rs/lib.rs`, `writer/rs/lib.rs`: same "trait impl'd but not imported at the call site" pattern as
  the `ui_wgpu` fix — `use vcs::DocumentDsl;` missing at two more call sites. Both now passing (26/26 and 7/7).
- `norm/core/script.ts`, `norm/plugin/script.ts` (too many `../`) and all 10 `norm/en/199X/script.ts` files
  (too few `../`) had wrong relative-depth imports to `repo/lib/js/index.ts` — a **pre-existing, real, silent
  breakage**: every `norm/*` project's test command was failing with "Cannot find module" before this fix,
  for reasons entirely unrelated to coverage. This alone likely blocked norm-* testing for a long time.
- Three `go.mod` files (`repo/lib/go`, `compose/client/lib/go`, `repo/server/coordinator/go`) were pinned to
  `go 1.25.0` while `go.work` and every other module already moved to `go 1.25.5` — caused a
  `"compile: version go1.25.5 does not match go tool version go1.25.0"` failure. Bumped all three to match.
- A stale local Go build cache (`go clean -cache`) was independently contributing to the same symptom for
  `repo-go-lib` in earlier attempts — real, but a machine-state issue, not a code bug (documented for
  awareness, not something to "fix" in the repo).

**Final exclusion list** (all have real, spawned follow-up tasks — see task chips): `math-polynomial`,
`math-cas` (performance + correctness), `puzzle-2d-rs`, `repo-cli-rs` (own test-logic bugs, not chased down
this session, see run 9's important.md notes below), `os-hub`, `compose-py` (fixture data), `framework-
renderer-wgpu` (the *other*, larger wgpu crate), `compose-rs`/`compose-hub` (same fixture data issue as
compose-py, plus `KitSnapshot`/`ComposeWireOperation` Serialize/Deserialize trait-bound errors that were
intermittently reproducible — worth re-checking, may itself be a moving-target symptom), `repo-lib` (one
genuine failing assertion on a hostname config value, needs domain knowledge to fix correctly).

**Phase C wave 1 launched** (Workflow `wf_b0832394-0fe`, "coverage-phase-c-wave-1"): 15 agents writing tests
for the highest-value real, cleanly-resolved under-covered Rust crates from the worklist (`architect_program`
1386 uncovered lines, `ui_wgpu` 1295, `energy_engine` 1273, `remodel_video` 1217, `mathematical_sampling`
1203, `kernel_3d_brepkit` 990, `mathematical_graph_dsl` 648, `ui_tui` 605, `draw` 602, `semio-framework-core`
556, `kernel_3d_mesh` 543, `mathematical_fuzzy` 476, `puzzle_3d` 431, `framework_editor` 389, `kernel_3d_scene`
376). Each agent extends the existing in-source `#[cfg(test)] mod tests` block only, verifies locally, then a
separate verify-stage agent independently re-measures. Check `/workflows` or this ticket's transcript dir for
progress; results were not yet in when this note was written.

**Worklist note**: `build-worklist.ts` v2 walks raw per-tool coverage output with provenance (not the flat
merged summary, which can't disambiguate same-named files across bundles) — `worklist.json` in this folder is
the live, re-runnable artifact for picking the next wave. It currently shows 5 "unresolved" groups (files it
couldn't map to an owning `script.ts` bundle) and a few duplicate-slug artifacts from multi-package cargo
invocations (e.g. `animate_core` appears 3× under different combined slugs, 2 of which show a spurious 0% —
likely because that particular multi-package test invocation's `animate_core`-specific tests didn't actually
run) — worth a closer look before trusting those specific rows, everything else is solid.

---

# Follow-up resolved: framework-core/wgpu icon-refactor breakage (2026-07-26, later session)

The "framework-core/wgpu likely mid-refactor breakage" follow-up mentioned below is now resolved — the
compile-time-icons + window-kind-icons refactor (`.cursor/plans/compile-time_icons_26e07fc6.plan.md`,
`.cursor/plans/window_kind_icons_acc26d72.plan.md`, both `completed`) had landed but left two regressions:

1. **`ui/wgpu/rs`** (package `ui_wgpu`, a dependency of both `framework-core` and `framework-renderer-wgpu`,
   no standalone nx test target of its own) — NOT actually fixed by the refactor as originally believed
   (correcting the note below): `#[path = "../../../ui/asset/icon/generated/icon_name.rs"] mod icon_name_gen;`
   + `pub use icon_name_gen::IconName;` at the crate root were correctly wired, but 6 separate `pub mod` blocks
   (`paint`, `reconcile`, `events`, `shell`, `engine`, `widgets`) each independently need their own
   `use crate::IconName;` — Rust modules don't inherit sibling imports — and none of them had it (only visible
   once the `engine` feature is enabled, which a bare `cargo check` doesn't do by default, hiding most of these
   behind `#[cfg(feature = "engine")]`). Fixed all 6, plus 3 further genuine type-mismatch bugs the missing
   imports had been masking (`render_toggle` called with `&IconName` instead of `IconName` at two call sites;
   one test constructed `IconName::Sparkles` where `UiIconSelectNode.value: String` expected). `cargo test -p
   ui_wgpu --tests --features engine` now 164/164 passing.
2. **`semio-framework-core`** `ui::app_document_tests` — 5 genuine regressions from `WindowKindDefinition.
   icon_id`/`UtilityDefinition.icon_id`/`ToolDefinition.icon_id` becoming required `IconName` (closed catalog):
   test fixtures used a fake `"icon.brush"` id and bare `"brush"`/`"fill"` strings (not real catalog names),
   and one JSON literal omitted the now-required `iconId` field entirely. Fixed in `framework/core/rs/lib.rs`
   by using real catalog names (`paintbrush`, `paint-bucket`, `pen-tool`) and adding `"iconId":"pen-tool"` to
   the JSON fixture — no production code changed, test-only. `cargo test -p semio-framework-core --lib` now
   57/57 passing.
3. **`framework/renderer/wgpu/rs`** (package `semio-framework-renderer-wgpu`, the actual nx project named
   `@semio-tech/framework-renderer-wgpu` — a DIFFERENT, much larger 28,659-line crate that depends on
   `ui_wgpu` but has its own separate bugs) — still broken, 20 compile errors (13 more IconName-enum-vs-
   String mismatches in the same family as #1, plus 4 unrelated bugs: a missing `dock_tab_content_width`
   function, a missing `Color` type import, two mutable-borrow errors on `atlas`). New follow-up task spawned
   for this specifically — NOT fixed this session, still excluded from the baseline run.

The `nx --exclude` for `semio-framework-core` can be dropped going forward. `@semio-tech/framework-renderer-
wgpu` still needs to stay excluded until the follow-up above lands.

---

# Follow-up: exhaustive `operation`/`operator` mismatch sweep (post session-conclusion)

The closed rename ticket `.repo/🎫/26/07/25/RENAME-OP-ABBREVIATION-TO-OPERATION` never ran `cargo check`/`tsc`
to verify its own rename, per its own summary. The partial baseline runs above already caught and fixed 5
trivial instances of the resulting `operation`/`operator` field mismatch in-line (`compose/client/lib/js/
index.ts` 6 sites, `mathematical/cas/rs/src/{assume,fmt,solve}.rs` 4 sites, `infinite/board/port/directed/
dag/rs/lib.rs` 2 misplaced `//!` doc comments — different bug class, same commit —, `puzzle/2d/rs/lib.rs` 2
sites, `puzzle/3d/rs/lib.rs` 1 site). A dedicated exhaustive sweep (Rust/TS/JS/Go/Python/C#, cross-checked via
per-package `cargo check` and a full-repo `tsc --noEmit`) found and fixed **7 more confirmed mismatches**:

1. **`note/plugin/rs/lib.rs:299`** — real compile error (`E0560`), `NoteDiff { operator: ... }` vs the struct's
   declared field `operation`. Fixed; `cargo check -p note-plugin` now passes clean.
2. **`framework/renderer/react/index.tsx`** — 23 `InkCanvasEvent` construction sites used `operator:` against
   the type's declared `operation` discriminant (every ink-canvas draw/erase/move/paste gesture failed to
   type-check). Fixed all 23.
3. **`framework/renderer/react/index.tsx`** — 4 silent runtime bugs: `dispatch(nodeGraphActions.edit, {
   operations: [{ operator: ... }] })` payloads (delete-selection default, 2× setFixture-after-reorganize,
   node-drag-stop move) were built with `operator` instead of `operation`, so 7 Rust plugins reading `.get(
   "operation")` silently dropped them via their `_ => {}` fallback (drag-to-move, reorganize-fixture-sync,
   delete-selection all silently no-op'd). Fixed all 4, verified against each plugin's match arms. Left one
   sibling site (`onSliderChange` → `{ operator: "setSlider", ... }`, ~line 15774) untouched — no plugin
   handles `setSlider` under either name; looks like dead/unfinished code, not this bug class.
4. **`compose/client/lib/js/index.ts`** — the earlier in-line fix only corrected the *reply-reading* direction
   (`m.operation` → `m.operator`, matching `kit-store.worker.ts`'s `post({ operator: ... })` convention) but
   missed the *request-sending* direction entirely: `init()`'s `postMessage({ operator: "init" })` plus
   `execute()`/`subscribe()`'s `postMessage({ operator: "execute"/"subscribe" })` all need `operation:` to match
   the worker's `self.onmessage` check (`msg.operation === "init"/"execute"/"subscribe"`). Also `init()`'s own
   reply listener still checked `m.operation === "ready"/"error"` instead of `m.operator`. Net effect before
   this fix: every worker-backed session `open()` hit the 30s init timeout and silently fell back to inline
   WASM — worker transport was completely non-functional. Fixed all 5 remaining sites in this file.
5. **`compose/client/lib/sketchpad/js/index.ts:17528-17554`** — 3 near-identical VCS-handler factories
   (`createComposeDesignAppVcsHandler`, `...Type...`, `...Kit...`) had `(doc, operator) => ({ id: operation.id
   })` — parameter named `operator`, body referencing an undefined `operation`, a hard `tsc` error (TS2304) in
   all 3. Fixed (body now reads `operator.id`).
6. **`kernel/2d/js/index.ts`** — `booleanPathsClient`/`booleanPathsViaWasm` (lines 368, 443) both take a
   `operator: DrawBooleanOperation` param but their bodies referenced an undefined `operation` — hard `tsc`
   errors (TS2304), and the boolean-path (union/difference/intersection) fallback + WASM bridge didn't compile.
   Fixed both.

Investigated and correctly left alone (not this bug class): `trinity/jack/lsp/js/worker.ts` (incoming
`operation`, outgoing `operator` — this split is the *same* convention as `kit-store.worker.ts`'s
request/reply asymmetry, i.e. internally consistent, not a mismatch); GraphQL AST `OperationDefinition.
operation` in Go/TS; math CSG/algebraic "operator" concepts; C# `Operation.Operator` in `Compose.cs`. All
verified as legitimate uses, not stale-rename fallout.

No further `operation`/`operator` mismatches found after this pass — the sweep is now believed exhaustive
across Rust, TS/JS, Go, Python, and C#.

---

# Final Phase A status (session conclusion) — see bottom for the decision

**A clean 100% full-repo exhaustive run was not achieved this session, and that's a genuine repo-state finding,
not a coverage-tooling gap.** 9 full-repo attempts (runs 1–9, logs in this folder) progressively fixed real,
already-committed bugs (contention, 5 instances of an incompletely-verified rename, a doc-comment syntax error,
a slow-under-instrumentation crate) and got measurably further each time, but kept surfacing *new*, unrelated,
pre-existing breakage in parts of the codebase nothing has run a full test pass over before (this repo's CI
only exercises the "fundamental" test level — see `.github/workflows/`). Three follow-up tasks are spawned
for what's left (math-polynomial/math-cas performance+correctness, framework-core/wgpu likely mid-refactor
breakage, and a bundle of remaining test-logic/derive bugs). The infrastructure itself — coverage
instrumentation, aggregation, the 95% gate — is proven correct end-to-end against real data; getting a fully
clean baseline is now blocked on those follow-ups landing, not on anything in this ticket.

**Final partial baseline this session:** 98 files, 57,545/125,249 lines = **45.94%** (`phase-a-partial-
summary.json`, produced by run 9 before it hit the compose-rs Serialize/Deserialize compile error). This
covers most of `mathematical/*` (excluding polynomial/cas), several Go modules, `main.py`, and 4 JS projects.
It excludes math-polynomial, math-cas, framework-core, framework-renderer-wgpu, and everything downstream of
compose-rs (a large fraction of the JS/Go/Python/.NET side) — see the follow-up tasks for why.

---

# (historical, in chronological order) JS coverage is flaky (not fully broken), plus real Phase A findings

**UPDATE after a full-repo Phase A run:** the "vitest coverage is non-functional" conclusion below (from an
isolated minimal repro) was too pessimistic. In the real `bun ./script.ts test exhaustive` run, 4 of 20 JS
projects produced real, populated LCOV (`framework/renderer/react` — 9,513 `DA:` records, 2,047 hit;
`repo/server/coordinator`, `cad/machine/stately`, `framework/product/os/dev`), while 16 produced empty files —
including `mathematical/graph/dsl/core`, which passes 8 real tests but consistently gets 0 coverage records
across every attempt (isolated repro and full pipeline alike). So this is **flaky/inconsistent, not uniformly
broken** — likely a race in vitest's coverage-provider startup, not the sandbox-wide block first suspected.
Needs follow-up (not resolved this session): compare a working vs. non-working project's config/environment
line-by-line, or file an upstream vitest issue if no repo-side cause is found.

**Phase A run 1** (shared `target/`, no isolation): failed after 38 min — SIGTERM cascade across dozens of
crates. Root cause: several *other* concurrent Claude Code sessions were actively running their own `cargo`
builds against the same shared `target/` dir at the time (visible in `ps aux`:
`claude-501-lowpoly-wave2-cargo-target`, `claude-501-mathematical-wave3-cargo-target`, etc.) — not caused by
this ticket's changes.

**Phase A run 2** (isolated `CARGO_TARGET_DIR=/private/tmp/claude-501-coverage-baseline-cargo-target`): the
wasm prerequisite build succeeded in 26m52s, but the mandatory `bunx tsc --noEmit` build gate for
`compose/client/lib/js` failed on a **real, pre-existing, already-committed bug**: `execute()`/`subscribe()`
in `compose/client/lib/js/index.ts` read `m.operation` on a worker-message object whose type declares
`operator` — so those branches could never match at runtime (the methods would hang until whatever caller
timeout). Fixed (6 call sites, lines ~414–446) by correcting the field name to `operator`, matching the type
and the `postMessage` senders. `init()`'s message handler (lines 379/382) uses a separate, internally-consistent
`{ operation?: string }` type and was deliberately left alone.

**Phase A run 3** (same isolated target dir, now warm + the fix above): ran ~70 minutes and reached real
per-project test execution across 181 projects before failing on two more pre-existing issues, unrelated to
each other and to coverage instrumentation specifically:
1. `infinite/board/port/directed/dag/rs/lib.rs` had two **already-committed** `E0753` errors — inner `//!` doc
   comments placed mid-file (after other items) under `//#region 🔖Dsl` / `//#region 🔖OpText` headers, which
   is invalid Rust syntax anywhere but the top of a file. This is a plain compile error blocking *any*
   `cargo build`/`test`/`check` of that crate and everything depending on it (a large chunk of the ~55
   "Failed tasks" in that run's cascade) — nothing to do with coverage. Fixed: changed both blocks from `//!`
   to `//` (matching rustc's own suggested fix; they're region-descriptive comments, not real crate/item docs).
2. `mathematical_polynomial`'s `algebraic`/`factor`/`roots` test modules (e.g.
   `cbrt2_times_cbrt4_equals_2`, `wilkinson_like_small_case_root_count`) ran past the combined
   `buildBudgetMs + TEST_LEVEL_BUDGET_MS.exhaustive` = 2,100,000ms budget and got killed. `cargo-llvm-cov`
   builds an **unoptimized** (`test` profile) binary — these algebraic-number/root-isolation algorithms are
   apparently expensive enough that instrumentation overhead pushes them over budget even though they likely
   pass comfortably under a normal `--release` `cargo test`. This is real, expected instrumentation overhead —
   exactly the risk the plan's "900s budget overflow" section anticipated, just triggered at the crate's
   *build* budget rather than mid-suite. Not fixed this session — candidate fixes for Phase B/C: raise this
   crate's budget via `SEMIO_TEST_BUDGET_MS`/`SEMIO_BUILD_BUDGET_MS`, move the slowest cases to a lazier
   assertion style, or accept a longer per-crate override.

A 4-file aggregation script (`aggregate-now.ts`) run against whatever `.repo/coverage/` had accumulated from
run 3 (before the mathematical_polynomial timeout killed the overall command) shows the pipeline itself is
correct end-to-end: 97 files, 56,736/124,432 lines = **45.60%** partial baseline (see
`phase-a-partial-summary.json`) — best files were `mathematical/entropy`/`mathematical/graph/traversal`/
`mathematical/spatial` Rust crates at 97–100%, worst were entirely-untested `repo/server/coordinator` API
routes and Go `main.go` entry points at 0–11%. This is NOT a full-repo baseline (only 97 of ~230+ files/
projects), just proof the tooling works.

**Next Phase A attempt** should retry `SEMIO_COVERAGE=1 CARGO_TARGET_DIR=/private/tmp/claude-501-coverage-
baseline-cargo-target bun ./script.ts test exhaustive` now that both blockers above are fixed — expect it to
get further, but likely not all the way through on the first try given the sheer number of projects; treat
each new failure the same way (check whether it's a real pre-existing bug vs. a coverage-instrumentation
budget/overhead issue, fix or note accordingly, retry).

---

# (historical) vitest v8/istanbul coverage is non-functional in this Claude Code sandbox

**Status:** the JS/TS coverage wiring (`runVitest` coverage flags, `coverage.include` in ~29 vitest configs,
`@vitest/coverage-v8`) is fully implemented and matches the design, but could not be runtime-verified in this
session — `@vitest/coverage-v8` (and, tested as a control, `@vitest/coverage-istanbul`) produce an empty
`coverage-final.json` (`{}`) for every run in this execution environment, regardless of:

- vitest version (tried 3.2.4, 4.0.17, 4.1.7 — all matched to their coverage package)
- Node version (tried 22.23.1 and 24.15.0 via homebrew)
- sandbox mode (tried with and without `dangerouslyDisableSandbox`)
- repo involvement (reproduced in a bare scratch project fully outside the monorepo, zero ancestor configs)

Root-caused as far as is possible from here: a raw `node:inspector/promises` `Session.post("Profiler.
takePreciseCoverage")` call **does** return real per-file coverage data when called directly in the same
process (verified — captured the calling script's own function). But the same V8 Profiler data never reaches
vitest's coverage report, for both the v8 provider (inspector-based) and the istanbul provider (instrumentation-
based, no inspector at all) — meaning whatever is broken is shared plumbing inside vitest's coverage pipeline in
this environment, not a provider-specific inspector issue and not a version regression.

**Action needed:** re-run the smoke test (`SEMIO_COVERAGE=1 bun ./script.ts test quick` in e.g. `cad/core` or
`mathematical/graph/dsl/core`) in the actual devcontainer/CI environment, where this sandbox restriction likely
does not apply. Check `.repo/coverage/js/**/lcov.info` for non-empty `SF:`/`DA:` records. If it's still empty
there, this needs upstream investigation (vitest/coverage-v8 issue tracker) before Phase A of the workforce can
trust any JS coverage numbers — until then, treat repo-wide coverage percentages as Rust/Go/Python/.NET-only.

**What IS verified working end-to-end in this session:**
- Rust: `cargo-llvm-cov` on `mathematical_number` — real LCOV with populated `DA:` hit counts (72.60% on a
  first run), confirmed via `.repo/coverage/rust/*.lcov`.
- The aggregation pipeline itself (`parseLcov`/`mergeLcov`/`summarizeCoverage` in `repo/lib/js/index.ts`) —
  verified against the real Rust LCOV output, produces correct per-file and repo-wide percentages.
- `bun install` dependency resolution for `@vitest/coverage-v8` (root `package.json`).
- The `test-exhaustive` nx-target gap closure (0 offenders remain, verified via a full repo scan).
- `runVitest`'s pre-existing latent bug (fixed as a side effect): `bun x vitest` resolves bunx's own globally
  cached vitest version rather than the workspace's locally installed one — silently drifted to 3.2.7 while
  the workspace was pinned to `^4.0.17`/resolved 4.1.7. Fixed by invoking `node_modules/vitest/vitest.mjs`
  directly. Coverage runs additionally need to run under plain `node`, not bun — Bun's `node:inspector` shim
  does not implement the V8 Profiler coverage APIs (`Session.post` on `Profiler.startPreciseCoverage` throws
  "Coverage APIs are not supported").
