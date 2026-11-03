# Typed enums v3 (wave 2)

No cargo/rustc/build was run; proof level is Python only (generator `--check`, selftest, git read-only comparison). The stalled v3 had already done most of the work; this pass verified it, fixed one regression and wrote this report. en1994 was not touched (freeze).

## a) en1995 strengthClass fixtures
- New audit `🧰️scripts/typed-enums/en1995_strength_audit_flat.py` maps every flattened fixture back to its former deep path and compares the ordered `strengthClass` tokens with HEAD/index (read-only git).
- Result: 154 fixture files identical to the committed version, 0 differ, 273 have no committed version.
- Those 273 were recovered exactly (not by position pattern) by `en1995_strength_recover.py`: members and connections come from the committed base snapshot (GL28h, `gl28h`), panels from the entity spec default `strength_class = "C24"` (`en1995-collections/spec.py`), the `change-panel-strength-class` scenario changes it to the generator's next option (`c27`). The uncommitted profile is `gl28h` x248, panel-bearing `c24+gl28h` x21, `c24` x2, `c27` x1, `c27+gl28h` x1: consistent with that rule. The check-outcome documents (8) and `catalogue/demo-document` carry `gl28h` (verified against the independent oracle by v3).
- Undeterminable: none by rule; the only residual uncertainty is that the 273 values rest on generator semantics, not on a committed copy.

## b) snow_high
`ImposedCategory::SnowHigh` exists in all layers (model, wire JSON/TS/GraphQL/proto, sqlite codec rs/ts, editor choice `snow-high`, timber `psi_factors` -> registry row `ndp.en1990.psi.snow_high`, python oracle) and is in `specs/en1995.json`. Tests: `snow_high_category_reads_its_own_table_a1_1_row`, `sls_frequent_uses_psi1_and_de_snow_high_diverges`, fixture `check-outcome/snow-high-pass`.

## c) change-reinforcement-ductility
Leaf payload is `DuctilityClass` in rs, diff, inverse, schema json/graphql/proto/ts, text codec and edit rule; fixture value `"c"`.

## d) Generator guard
`convert.py`: a value that already is a variant is never rewritten; unmapped legacy values reach the default only for String-shaped legacy data; conflicts print `lossy:`; spec drift (model enum variants vs spec) prints `lossy:`; `--check` exits 1 on `manual|unmapped|lossy`. `convert_selftest.py` passes (`convert_selftest ok`).

## Regression fixed in this pass
en1992 `.proto` files (root and snapshot) referenced `BarPosition`, `CementClass`, `ColumnPosition`, `SlabFireSystem`, `ActionKind`, `ActionSource`, `BarBend`, `PsiCategory` without declaring them. Re-applied the en1992 spec (`--apply`): only the two proto files changed; `--check` now exit 0. `--check` exit 0 also for en1991, en1993, en1995 (en1999 produced no non-zero rows).

## Priority 2 (not done)
en1990/1996/1997/1998 (and en1994: frozen) enum fields were not converted. Census files are in `🗑️generated/typed-enums-v3/census-*.txt`; `specs/en1998.json` (4 enums) exists and its dry run touches 11-16 files per enum with 7 residual string consumers each, which need a compiler loop; applying without one would risk uncompilable code in an artifact another agent owns.

Ready for rebuild lines appended to `🗑️generated/build-en1995/latest.md` and `build-en1992/latest.md`.
