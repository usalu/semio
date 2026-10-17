# 🌾 RESULT: ORACLE-HARVEST-FRAMEWORK (`ohf`)

## 0. Verdict

IN PROGRESS (started 17:30). Ledger: `ORACLE-HARVEST-FRAMEWORK.md`. Tools: `ohf-tools/` (copy of the pos typeck oracle with sandbox `/Users/ueli/.cache/ohf/`, `mkfam.py` family writer). Logs: `🗑️generated/ohf-*`.

## 1. Method

- Oracle: `ohf-tools/pos/pos.py run <family> --cfgs lib,test --no-base` (`POS_PREFIX=ohf`): all crates of a family stitched into one sandbox union, external crates stubbed, `rustc +nightly-2026-07-20 --emit=metadata`. Canary rows (E0308/E0616/E0624 in the stub root) prove typeck was reached.
- Families (`ohf-tools/families/`): fw-modules (348 crates), fw-os (471), fw-products (66), hub-core (54), hub-comp (98), teaching (3).

## 2. Progress

### 2.1 fw-modules lib (done)

- Run m1 (346 crates, canary reached): 340 target-code non-stub rows. 12 real defects fixed (ledger rows 1-12), 17 row keys gone in run m2, 1 new row (E0282 in machine-derive, caused by the `quote!` stub macro added to the sandbox root in between; artifact). The rest are classified as artifacts in the ledger.
- Oracle improvements (all in `ohf-tools/pos/`, sandbox only): stub `macro_rules!` for `quote`, `Token`, `format_ident`, `parse_macro_input`, `parse_quote`, `braced`, `parenthesized` at the union root (900 `cannot find macro` rows less).
- Union size: fw-all (1,029 crates) never leaves rustc's resolver diagnostics (`UsePlacementFinder`, quadratic in error count and AST size; 14 CPU minutes, killed). Use the per-family unions instead. Resolution-fatal macro failures (`app_commands!`, `app_labels!`) also hide typeck; they were real (see ledger section fw-all).

### 2.2 In flight

fw-os lib (o1) is running; queued after it: fw-modules test, fw-products, hub-core, hub-comp, teaching (lib+test).
