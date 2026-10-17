# Compliance Fix (cpf)

Slug: COMPLIANCE-FIX, prefix `cpf`. Executor: Sonnet. 2026-10-17 20:25 to about 21:30. Logs `🗑️generated/cpf-*`, tools `cpf-tools/` (`env.sh` sets `$G`/`$T`; `edit_lib.py` atomic edits). Input: `AUDIT-WAVE7-COMPLIANCE.md` (every finding re-verified before acting).

## 0. Verdict

DONE for all five rules, with the deferrals in section 7. Static evidence only: no compiler ran (the launcher refuses, see `RESULT-REPO-GATES-7.md`); no test ran. Every edited file was re-parsed (TS compiler parse, rustfmt `skip_children`), and the docstring census functions of the repo (`docstringHitsOfText`, `docstringEmojiReuseOfText`) were run over every file that received a docstring (0 hits, 0 first-emoji reuse).

Two audit conclusions did not hold: (a) the "~194 comments inside bodies" are pre-existing HEAD comments that the carves moved (only 2 were added by the fleet, rule 3); (b) the audit's "120 test/assert [DEBUG] hits exempt" contained ~90 real `console.log("[DEBUG] ...")` emitters on minified lines (its regex matched any `expect(` on the line), which rule 1 therefore had to treat.

## 1. Counts per rule

| # | Rule | Found | Fixed | Deferred / owner |
|---|---|---|---|---|
| 1 | [DEBUG] temporaries | 6 source sites + 21 test lines (audit); live re-scan found 132 new tagged lines (not present in HEAD) in 45 files, 17 left | drain diagnostics removed (1 module + 2 tests + 6 fixture keys + 2 schema defs); rust-graph stub + preparation log removed; ~77 tagged channels retagged `[DATA]`; ~76 unconsumed test log statements deleted; 19 recorded statements in `child-group-owner.json` | `Fixed UDRW image` channel (nx graph), HEAD-origin hits, live additions by the Codex peer (section 2) |
| 2 | docstrings / duplicate emoji | 141 files without docstring + 58 with a plain `//` comment (199, wider than the audit's 123: scan covers every added file) ; 2 duplicate sibling groups | 199 files + 2 groups; 5 opening emoji re-picked because the file reused them | `bd8-diag.ts` (peer scratch file, no docstring added) |
| 3 | comments inside bodies | 2,215+ Rust and 1,106 TS comment runs inside bodies/definitions in changed files | 2 (fleet-added) removed; all others are HEAD comments moved by carves | none |
| 4 | launch routes of 4 crates | 4 crates with `tests/`, no route | 24 routes (4 crates x build/check/test x prototype/release) in both launch files | stale ticket routes stay with rg7 |
| 5 | non-testkit cfg features | 856 `cfg(any/all(test, feature=...))` rows in changed files, 296 with a non-`testkit` feature | 1 fix (`testing` -> `testkit` in layout mutations); the other rows verified | include!-hosted files resolved by directory heuristic (section 6) |

## 2. Rule 1 detail

Method: `git grep HEAD` gave every tagged line of HEAD (6,528 lines); the repo census function `runDebugTagCensus` (the acceptance gate `debug-tags`) listed the live hits; a hit counts as new when its trimmed line is not in HEAD. TypeScript sources were handled with the compiler AST (`ts_debug.ts` finds every string/template literal containing `[DEBUG] `, classifies it as an emitter statement or other use, and `ts_debug_apply.ts` edits by AST range; edits re-parsed with 0 diagnostics).

Decision per tag: a tag with a consumer (a test or schema that reads lines starting with `[DEBUG] <tag> `) is a data channel, not a temporary log, so emitter and consumer are retagged together to `[DATA] <tag> ` (no `[DATA]` existed in HEAD). A tag with only emitters is a temporary log and its statement is deleted (an `if(cond)` head is removed with the statement when the condition is pure, else the body becomes `{}`; arguments are checked pure).

- Removed outright: `🧭️routing/📥️invocation/🏃️process/🟦️.ts` drain diagnostics (`drainDiagnostic*` protocol, work/state maps, record + report functions, the `writeSync` import, 3 call sites; `track`/`drain` restored to the HEAD text), its `$defs` in `🧬️schema/🔣️.json` (file now equals HEAD), tests `drain diagnostics` and `drain tracking` and fixture keys `drainDiagnostic*`, `drainDelivery*`, `drainTracking*` in `📦️artifacts/.../📥️parent/`.
- `📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts`: empty `phase` stub, its 4 calls and 4 `*Started` constants; the `[DEBUG] Selected Cargo preparation refused` try/catch.
- Retagged channels: `native-codec-publication=` (4 stdio rust tests + the hub script reader; the `[DEBUG] ` part dropped on both sides), `child-group-owner.json` (19 recorded `eprintln!` strings; the test only asserts their count), `Native dependency fixture receipt` (schema `prefix`, read by producer and consumer through `protocol.prefix`), and about 77 TS literals: the worker result lines (`ProtectedParentWorker`, `MaintenanceNestedWorker`, `SelectedScopeWorker`, ...), the library failure-diagnostic protocols (`Current Tools role`, `Current Tools generation names|authentication`, `Authorizer receipt refusal`, `Dependency sync incomplete`, `Dependency unready rebuild incomplete`, `Current Graph ...`) in production code and their tests.
- Deleted: about 76 unconsumed `console.log/error("[DEBUG] Actual ...")` style statements in process/lease/library tests.

Deferred:
- `Fixed UDRW image` (`🔌️nx/📦️graph/🟦️.ts:212` parses recorded producer logs by that prefix; 15 fixture texts in `🧪️tests/🔣️.json` are pinned by `producerLogSha256` selectors). Changing the prefix means regenerating the pinned logs, and the true producer is unproven (the nx patch in HEAD does not print this tag). Owner: library owner (rg7 / nx graph). Recipe: have the producer return structured data and drop the text prefix coupling.
- HEAD-origin hits (about 110 lines in HEAD files, mostly assertion messages and BD9-restored worker channels) are not new and were not touched; the census still counts them.
- A Codex peer keeps adding fresh `[DEBUG]` lines to library/process tests while this slice runs (new ones appeared between two sweeps). The last sweep of this slice is in section 5; rerun `bash cpf-tools/debug_pipeline.sh --apply` once the peer stops.

## 3. Rule 2 detail

`docscan.py` classifies the first content line of every added/untracked `.rs/.ts/.tsx` (directives `#!`, `/// <reference`, `// @vitest-environment` are skipped); `docfix.py --apply` fixed 199 files:
- plain `//` comment with emoji (58 Rust): the leading comment run became a doc comment, `//!` for `#[path]`-wired modules and `///` for `include!`-wired fragments (wiring found by searching the including file; a first version mis-wired 5 `part-N-x` fragments as modules, caught by checking `include!` sites and fixed to `///`; `rustc` accepts `///` before `use`, rejects `//!` in an `include!`d file).
- no docstring: Rust `//!`/`///`, TS `/** <emoji> <Subject>: <what> */` with the subject from the folder name and the content from test titles or exported names; emoji = the folder's own emoji, else a fitting pool emoji, never one already opening a sibling file or any docstring of the same file.
- 5 files whose author-chosen emoji was reused by later docstrings of the same file got a new opening emoji (`invite-and-index-laws`, `guest-residency-and-codec-laws`, `production-posture-laws`, `agent-delegation-laws`, `snapshot-exchange-laws`).
- Duplicate sibling groups: `🔓️mutable-copy` (test file now `🧪️`) and `✂️clipping` (the Rust file now `🧱️`).
No folder was renamed, so no taxonomy run was needed. Note `🧩️composition/bd8-diag.ts` (plugin registry tests) is a peer's scratch diagnostic file inside the source tree without docstring; left to bd8/bd11.

## 4. Rule 3 detail

A body comment counts as fleet-added only when its normalised text is absent from every comment of HEAD (`cpf-head-comments-{a,b}.bin`, 333k + 303k hashes over `.rs/.ts/.tsx` of HEAD). Detection: a string/comment-aware Rust lexer with an fn/impl/trait/struct/enum brace stack (`rslex.py`, 0 unbalanced files in 10,466) and the TS compiler AST (function bodies, class/interface/enum/type-literal bodies, JSX excluded). Doc comments, `#region` markers, `SAFETY`, `//~`, lint/ts directives are exempt. Result: 2 additions in the whole changed set, both fixed: the `/* padding survived ... */` arm of the las diff algebra (code equality verified before and after with the comment-stripped token text) and the 4-line rationale in `.storybook/main.ts` (moved into the docstring of `buildStorybookAliases`).

## 5. Rules 4 and 5 detail, final sweep

Rule 4: `cpf-tools/register_routes.py` (same text and order bands as the other route scripts: `900.98NNN` build, `922.98NNN` check, `903.98NNN` test; groups `4_build`, `4_gate`, `4_test`; names `🦀️ <crate> <target> <config>`). Crates: `semio-framework-3d-mesh-tests` (after the `3d-testing-support` routes), `semio-framework-plugin-command-page-laws`, `-child-group-wire-laws`, `-surface-carrier-laws` (after `plugin-testing-laws`). The crates have no `📋️project.json`, so the routes address the cargo-only owner row by crate name (the same form the 5 existing cargo-owner routes use); crates without a library use `--tests --offline` for build/check and `--offline` for test, `3d-mesh-tests` (has a lib) `--lib --offline` for build/check. Both files parse (13,535 configs in `launch.json`, 10,345 in the seed, the seed's 89 generated placeholders skipped) and the rg7 `launch_check.py` reports no missing project or target for these four. The 41/34 missing route paths of the audit are stale ticket-project paths: rg7 owns them.

Rule 5: all 296 non-`testkit` rows name a feature declared in the hosting crate except `semio-s-artifact-layout-layout-mutations` (`testing`, fixed to `testkit`: the crate and all its dev-dependents already use `testkit`). No feature is reachable from any `default`, and no crate that is not test-support enables one of them in `[dependencies]`/`[build-dependencies]` or forwards it from a default feature (`cfg_verify.py`: 0 and 0).

## 6. Verification

- TS: 116 + 195 + 12 touched files parse with 0 diagnostics (`parse_ts.ts`); Rust: 64 + 8 + 1 files pass `rustfmt --check` parsing (`parse_rs.py`).
- Docstring census functions on all 203 docstring files: 0 hits, 0 reuse of the opening emoji (2 pre-existing `no-emoji` hits inside `part-6-x` come from a split artefact, not from this slice).
- Debug census (`runDebugTagCensus`, `cpf-tools/debug-census.ts`, oracle disagreements 0): 354 hits in 157 files before, 248 hits in 133 files after; of the 248, 17 are not in HEAD (16 lines of the deferred `Fixed UDRW image` channel and 1 line a peer added during the last sweep), the rest is HEAD-origin.
- Limits: the cfg host-crate resolution uses `[lib]/[[test]]` roots plus the nearest `📦️packages/🦀️rust/Cargo.toml`; an `include!`d test file hosted by a companion crate can be attributed to its directory crate (the one file of that kind, `📨️dispatch`, was checked by hand).

## 7. Deferred

| Item | Owner | Why |
|---|---|---|
| `Fixed UDRW image` channel + 15 SHA-pinned fixture logs | library owner / rg7 | needs structured producer output |
| HEAD-origin `[DEBUG]` lines (assertion messages, restored worker channels) | later sweep | not new, asserted by tests |
| fresh `[DEBUG]` lines the Codex peer adds in library/process tests | the peer | rerun `debug_pipeline.sh` at the end of its work |
| `bd8-diag.ts` scratch file in plugin registry tests | bd8/bd11 | peer's file |
| 105 `🪞️mirror` barrels, `Form*` aliases | MIRROR-UNWIND | out of scope per brief |
