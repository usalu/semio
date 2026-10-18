# 🌾 RESULT: ORACLE-HARVEST-HUB (`ohh`)

## 0. Verdict

IN PROGRESS (started 22:17, updated 00:10). Ledger: `ORACLE-HARVEST-HUB.md` (rows 1-17 plus artifacts). Tools: `ohh-tools/` (copy of the `ohf-tools` pos typeck oracle; sandbox `/Users/ueli/.cache/ohh/`, `POS_PREFIX=ohh`; `run.sh <family> <label> [cfgs]`, `mkfam.py`, `rows.py`, `privdefs.py`, `traitscope.py`, `undecl.py`, `adddep.py`, `closeloop.py`). Logs: `🗑️generated/ohh-*`.

## 1. Method

- Families (`ohh-tools/families/`): `hub-core` (55 crates of the `🌎️hub/` workspace outside compositions; 54 build a lib), `hub-comp` (98 crates under `🧩️compositions`, 97 build), `teaching` (3). Each runs in two modes:
  - stub mode (`hub-core`, `hub-comp`): only the own crates are real, everything else is stubbed. This is the only mode that reaches body type-check (canary E0308/E0616/E0624 reached for lib and test), so it harvests privacy, trait-scope, arity and local-name rows between hub crates.
  - context mode (`hub-core-ctx` 459 crates, `hub-comp-ctx` 508, `teaching-ctx` 56): the direct dependencies plus the `pub use` re-export closure of the framework (`mkfam.py reexport_closure`, follows crate alias names too) are real, so hub imports of moved framework APIs resolve against real definitions. It never reaches type-check (framework macro errors stop rustc), so only resolution-level rows (E0425/E0432/E0433/E0412/E0603) are available. Only rows in own crates count.
- Oracle fixes made in `ohh-tools` (sandbox only): the union builder now creates every crate's manifest directory physically (an `include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../…"))` otherwise cannot be opened under `..`); `privdefs`/`pos2widen plan` read the `*-priv.err` files because `*-final.err` is cut at 30 MB (the test configs lose later privacy rows there).
- Proof rule: a row is closed only when the next run's digest no longer has it and shows no new row.

## 2. Findings so far

Real defects found and fixed (ledger rows 1-17), by class:

1. Manifests (rows 1-7): the carve of `semio-hub` dropped `semio-framework-server` (lib name `server`, 8 crates plus one test crate), `pack`, `semio-framework-value`, four layout editor/viewer crates, `plugin-host` (bim, note tests), two draw crates (space tests) and the `integration-fixtures` feature on the fixtures crate. The generator derives crate heads from `semio_*` names, so renamed lib names (`server`, `pack`) are invisible to the undeclared-dependency check; the oracle finds them as E0433.
2. Privacy (52 + 15 one-line widenings in 23 files, `pos2widen`): `pub(crate)`/`pub(super)`/private methods and fields that new crate boundaries or moved law modules now cross.
3. Trait scope and locals (rows 8-15): traits imported in a parent module but used in a child module, the missing `use std::fmt`, a stale call arity after the loader gained a progress cell, a lost local in a moved test.
4. Framework drift (row 17): `SnapshotRetirementStep`/`close_owned_step(1, MAX)` was replaced on 2026-10-09 by `RetainedCloneGrant`/`RetainedCloneStep`; six hub call sites migrated.
5. One framework context row (row 16): `value-retirement` cfg(test) imported the `FactoryPayloadRetirement` derive twice (E0252).

Artifact classes are listed at the end of the ledger.

## 3. Remaining / open

Filled in at the end of the run.
