# Norm example documents versus schema audit (2026-11-22)

## Method
`check-norm-example-schema.py` (this folder; python only, no cargo) walks all 79 authored `🗣️.dsl.semio` assets of the 15 Norm families. It tokenizes the DSL (header-driven tables, nested `[ {..} ]` lists, braced records, flattened `key=sub=value` records) and checks against each family's `🧬️schema/🔣️.json`:
- required top-level fields and required table columns (the DSL decoder rejects a missing non-optional column per row, and a missing non-optional top-level field);
- unknown columns, fields and tables;
- every `ENUM` cell value, using kebab of the schema enum, or an explicit `#[dsl(key = "...")]` from the family's snapshot Rust (for example `gr1a`, `m2_5`), or `_` for an absent optional.

The JSON schema lags the Rust model in places (din4108 `inclination-deg`, `delta-u-*`, layer classes; iso16757 and vdi3805 nest records the JSON does not describe). The checker therefore also accepts names that exist as `pub` fields in the family's snapshot Rust. `check-example-required-fields.py` (top-level required tables) is superseded but still passes (0 files).

## Result per family
| Family | Assets | Defects | Fix |
|---|---|---|---|
| en1990 | 9 | 8 stale table headers: `reliability-analyses` lacked `second-action-distribution/-mean/-sd`, `test-series` lacked `theory-mean` (schema and Rust require them; only the showcase had them) | Headers extended in the 8 assets; tables are empty, so no cell values needed (decoding checks columns per row, so it did not fail yet, but any added row would) |
| din4108 | 2 | none (JSON schema stale versus Rust, not an asset defect) | none |
| din16798 | 3 | none | none |
| din18599 | 5 | none (demo `cooling=lighting=...` is an empty cooling record followed by lighting) | none |
| en1991, en1992, en1993, en1994, en1995, en1996, en1997, en1998, en1999 | 3 to 6 each | none (en1998 `cases` fixed earlier) | none |
| iso16757, vdi3805 | 2 and 7 | none | none |

Final run: 15 families, 79 assets, 0 issues. Checker sanity: before the fix it reported the 32 en1990 findings, so it detects missing required columns.

## Not verified
No decoder, cargo or Play test was run (builds are forbidden on this host). The checker is a structural approximation of the Rust DSL decoder; deep value semantics (units, QTY parsing, nested flattened records of vdi3805 and iso16757) are only checked by key name. Committed binary twins (`🎒️.pack.semio`) exist only where an example test compares them; none were touched.

## Changed files
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/{🏢️accidental-seismic-compliant,🏢️accidental-seismic-failing,🏢️default-office,🏢️fatigue-compliant,🏢️fatigue-failing,🏢️high-consequence-office,🏢️road-bridge-compliant,🏢️road-bridge-failing}/<same name>/🗣️.dsl.semio`
- ticket: `check-norm-example-schema.py`, this report
