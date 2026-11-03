# Tools hardening v2 (wave 2)

Method: read the partial results (round0, s1-s5, f1/f2 matrices) instead of rerunning; no scratch copies were made. Only `--check` modes were run against the real tree.

## Status of the listed generators
| generator | before | now |
|---|---|---|
| leaf-kinds/en1992_run.py | removed lines in an early round | Files merge (additive), tags after max; last proof (f1): 84 modified, 0 removed, 2nd run 0; `--check` works |
| leaf-kinds/en1992_diff.py | library | no entry point, nothing to fix |
| kind-convention-v3/en1992_tightness_kinds.py | hard-coded tags 153/154 (tree has 152/153), overwrote files | FIXED this wave: tags read/allocated max+1, create-only/grow-only writes, taxonomy dedupe, `--check` (verified: 0 files on the real tree, rc 0) |
| en1995-collections schema/kinds/enums | non-idempotent | incremental, tags after max, proven ok in s5/f1 |
| typed-enums/convert.py | wrote by default | FIXED: writes only with `--apply`; `--dry`/`--check` unchanged; re-run safe by rule shape. Real-tree `--check`: en1991/en1993 clean, en1992 has 4 pending conversions (spec drift, not applied) |
| demo-values/demo_gen.py | overwrote demo values | native union + `--check`; matrix ok |
| showcase/*.py | overwrote the showcase fixture | FIXED: additive merge, `--check`. Builders en1993/1994/1998/1999 fail validation on the current schemas (stale, write nothing) |
| placements-adder/*, field-adder/field_adder.py, new_collection.py | duplicated fields, lost lines | via schema generator + `safe_run.py` (all-or-nothing); f2 matrix all ok. padd en1998 refuses (would lose 1 line) |
| leaf-kinds (rest) | various | s5 all ok; `en1990_run.py` r1 2nd-run diff predates the Files rewrite (01:49 vs 02:28), re-proof pending |

## Retired (deleted, unreferenced)
- field-adder/new_collection_en1998.py, _v4, _v5, _v6 (guarded stubs; schema generator owns twins). Their 5 cases removed from tools-hardening/cases.json and guarded.txt.
- en1992-diff-patch-fields.py, en1992-gen-member-field-leaves.py (superseded by leaf-kinds/en1992_run.py + en1992_diff.py).
- compact-schema-json.py was already gone.

## Open
- Re-prove `leaf-kinds/en1990_run.py --apply` twice in a sandbox (last record is pre-fix).
- Re-prove tightness and convert.py `--apply` in a sandbox (only `--check` was run).
- Showcase builders need updates for current schemas; en1992 typed-enum spec has drift.

Docs: 🧰️scripts/README-generators.md (purpose, inputs, safety, invocation, superseded, inventory).
