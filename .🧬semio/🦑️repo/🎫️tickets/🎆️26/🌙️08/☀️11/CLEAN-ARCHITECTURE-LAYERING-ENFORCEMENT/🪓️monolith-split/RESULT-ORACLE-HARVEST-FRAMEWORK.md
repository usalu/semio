# 🌾 RESULT: ORACLE-HARVEST-FRAMEWORK (`ohf`)

## 0. Verdict

DONE for the harvestable scope, proof run r3 pending (00:10). Scope since 22:2x: `🧰️framework/🔨️modules/**`, `🧰️framework/🛍️products/**` except the renderer, repo products (hub and teaching went to `ohh`, the renderer to `ohr`).

- Families swept: fw-modules (lib: typeck reached, 346 crates; test: resolution only), fw-os, fw-products and rest (lib, typeck not reached, so resolution rows only), fw-all (1,029 crates, lib resolution-only r1/r2, lib+test r3 running), tt (time-travel, lib typeck reached). wasm configs not run (cost; see below).
- Real errors found and fixed: 45 ledger rows (about 130 call sites): privacy widenings (brep-construction, link codec, RecordSource), stale paths after the value/graph/pack carves, undeclared dependencies (13 Cargo.toml edits), duplicate imports (E0252/E0260 ×6), macro-arm mismatches (hub, ohh scope), a 47-method trait impl split over five files (repo-graphql-fs), cfg-gated imports, the os-kernel facade shadowing its own globs, time-travel cross-crate private fields, a stale callers.json. Proof: m2 (17 keys gone), r2 (302 -> 6 target rows, 0 new), tt (privacy 0).
- Remaining with owners (ledger "Hand-offs"): plugin-program/extension layering inversion (plugin owner, design), `FaultStorage`/`FaultScript` missing from the db storage working tree (peer in flight), `plugin_tests_06` path (pi2/ohp), hub and renderer rows (ohh/ohr), wasm32/wasip2 resolution sweep not run, typeck-level classes only reachable for fw-modules lib and tt (wfcheck errors from stubbed traits stop rustc before bodies in fw-os / rest / fw-all).
- Not verified by a compiler: everything is oracle-level. Cargo.lock updates are needed for 13 manifests (compile window).

## 1. Method

- Oracle: `ohf-tools/pos/pos.py run <family> --cfgs lib,test --no-base` (`POS_PREFIX=ohf`): all crates of a family stitched into one sandbox union, external crates stubbed, `rustc +nightly-2026-07-20 --emit=metadata`. Canary rows (E0308/E0616/E0624 in the stub root) prove typeck was reached.
- Families (`ohf-tools/families/`): fw-modules (348 crates), fw-os (471), fw-products (66), hub-core (54), hub-comp (98), teaching (3).

## 2. Progress

### 2.1 fw-modules lib (done)

- Run m1 (346 crates, canary reached): 340 target-code non-stub rows. 12 real defects fixed (ledger rows 1-12), 17 row keys gone in run m2, 1 new row (E0282 in machine-derive, caused by the `quote!` stub macro added to the sandbox root in between; artifact). The rest are classified as artifacts in the ledger.
- Oracle improvements (all in `ohf-tools/pos/`, sandbox only): stub `macro_rules!` for `quote`, `Token`, `format_ident`, `parse_macro_input`, `parse_quote`, `braced`, `parenthesized` at the union root (900 `cannot find macro` rows less).
- Union size: fw-all (1,029 crates) never leaves rustc's resolver diagnostics (`UsePlacementFinder`, quadratic in error count and AST size; 14 CPU minutes, killed). Use the per-family unions instead. Resolution-fatal macro failures (`app_commands!`, `app_labels!`) also hide typeck; they were real (see ledger section fw-all).

### 2.2 Resumed 22:11 (state verified)

- Verified: frontier include-fragment edit applied, includer keeps the import; repo-graphql-fs `🔌️repo-context-port` merge in place; all earlier edits present (ledger rows 1-20).
- fw-all resolution-only run (`OHF_EXTRA=-Zno-analysis`, 1,029 crates, run r1, 5,008 s for iteration 0 because rustc's `UsePlacementFinder` walks the whole union AST per unresolved name; 326 s for iteration 1): 302 target-code non-stub rows (E0432 159, E0433 92, E0603 45, E0659 5, E0252 1). Triage in the ledger section "fw-all r1".
- Typeck-level classes (E0599/E0061/E0308/E0616/E0624/E0107/E0282) are only available where rustc reaches body type-check: fw-modules (done, lib). fw-os and rest do not reach it (wfcheck errors from stubbed traits stop rustc before bodies).


### 2.3 Fixes so far (ledger rows 1-39)

fw-modules lib: 12 real defects. fw-all first expansion errors: 7 (hub 2, renderer 1, os 4). repo-graphql-fs impl split: 1. fw-all r1 resolution rows in scope: 15 groups (rows 25-39), about 70 call sites. Proof run `r2` (fw-all, same options) started 22:32; the ledger rows move from "pending" to "gone" when it finishes.

### 2.4 Handover (scope change 22:2x)

Hub and teaching crates go to `ohh`, `📺️renderer/**` to `ohr`. This slice keeps `🧰️framework/🔨️modules/**`, `🧰️framework/🛍️products/**` (except the renderer) and repo products. Files this slice already edited in the handed-over areas: hub `🪐️space/…/🕹️space-command/🦀️.rs`, `🗣️terminology/🦀️.rs`; renderer `🧿️ui-engine-tests/🧰️surface-fixtures/🦀️.rs`. Run artifacts for them: `🗑️generated/ohf-fw-all-r1-*`, `ohf-rows-fw-all-r1.tsv` (all 302 rows), `ohf-rest-e1-*`. Tools for them: `ohf-tools/` (see ledger).

### 2.5 State 23:25

- r2 (fw-all lib, resolution-only): 302 -> 6 target rows, 0 new. Ledger rows 1-39 proven.
- m3 (fw-modules test): 375 resolution rows triaged, rows 40-43 fixed (dev-dependency gaps, three missing `pack` aliases). Proof run r3 (fw-all lib+test, resolution-only) started 23:18.
- Remaining real items are hand-offs in the ledger (plugin-program/extension layering inversion, db storage `FaultStorage`, `plugin_tests_06` path).
- Oracle changes (all under `ohf-tools/pos/`, sandbox only): `quote!`-family stub macros, `extern crate X as alias` rewritten to `use crate::__m_X as alias;`, `use alias as y;` rewrite, `OHF_EXTRA`/`OHF_NOHIR` env switches (`-Zno-analysis` makes a 1,029-crate resolution run feasible: rustc's `UsePlacementFinder` makes full-analysis unions quadratic in unresolved names).
