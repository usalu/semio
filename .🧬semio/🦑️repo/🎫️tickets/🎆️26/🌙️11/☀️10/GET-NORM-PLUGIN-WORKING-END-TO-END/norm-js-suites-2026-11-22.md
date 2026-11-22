# `@semio-tech/norm-js:test` remaining suites (2026-11-22)

Result: `NATIVE_EXEC_TARGET=@semio-tech/norm-js:test repo native-exec.ts ... test` exits 0, 12051 pass, 0 fail (50 files), `norm wire twins` included (green after the sibling agent's fixture work and the en1991 fixture fix below).

## schema sync (checker and byte-for-byte twin gate)
Cause (schema is the source):
- en1991/en1998 `Diff`: delta members (`fatigueModels`, `supportCollisions`, `localFires`, `snowGuards`; en1998 `cases`) were missing from the diff schema `required`, unlike their sibling deltas. Fix: added to `required`, twins regenerated (Rust stays non-optional).
- en1994 `Diff.cases`: the diff is all-optional, so the generator now makes it `Option<CaseRowDelta>` (the field is unused elsewhere).
- 9 mutation payloads (en1992 `change-stud-row-count`, en1993 anchor/bolt/plate/shear/forming counts): `minimum: 0` without `maximum` vs Rust `u32`. Fix: `maximum: 4294967295` in the schemas; Rust unchanged (`u32`), proto now `uint32`, GraphQL `UInt32`.
- en1997 artifact: `designSituation`/`designApproach` were plain strings in the schema while Rust and the snapshot use `SituationChoice`/`ApproachChoice`. Fix: `$ref` both enums (added `ApproachChoice` def); TS/proto/GraphQL regenerated; Rust unchanged.
- en1992 snapshot: `ConcreteGrade`, `ReinforcementGrade`, `PrestressSteel` annotations lacked `CanonicalJsonTree`/`canonical_json` that the Rust carries. Fix: schema annotation brought in line, Rust unchanged.
- en1998 twin drift (graphql/proto/rust): regenerated; the `cases` table now carries `#[dsl(table)]` from the schema.
Result: `schema twins check` identical=8321 drift=0; checker mismatches=0.

## field-meta choices
Cause: din18599 `USE_CLASS` table order differed from the schema enum (two new `Residential*Dwellings` members). Fix: `🐍️generate.py --artifact din18599`. Result: 285 choices, 0 mismatches.

## schema typecheck
Cause: din4108 hand-maintained TS twin (projection off) used `normWireRef` for `CaseRow` without importing it. Fix: import added. Result: zero diagnostics.

## insert-bindings
Cause: two new `insert-case` rules (din4108, en1998) absent from the closed corpus. Fix: added to the corpus, census 234 to 236 (test, schema, feature). 

## payload-bindings
Cause: (a) the test keyed payload schemas by `$id` matching `/s/norm/`, stale since ids carry the component (`norm-energy`, `norm-eurocode*`); (b) 12 en1998 mutation `$id` segments are abbreviated while kinds are long (path length); (c) counts din4108 49, en1998 368. Fix: payloads keyed by descriptor `semanticKind` plus `payloadSchema`; counts updated.

## catalogue
Cause: the din4108 and en1998 oracle catalogues were not in native `KINDS` order (case kinds first) and en1998 lacked the seven case kinds. Fix: reordered; en1998 got vectors, manifest entries and seven new `✅apply` fixtures (insert/remove/change number/text/label-en/label-de/entry-id), derived with `norm-en1998-case-fixtures.py`.

## wire twins (side effect)
Making the en1991 diff deltas required required the four empty deltas in 304 en1991 `🔣️.diff.json` fixtures (script inline, canonical bytes kept).

## Not touched
en1998 mutate adapters (`🧪️tests/🫨️mutate-en1998-1` Rust/Python) still list 288/294 kinds (already stale before) and do not cover the case kinds; they are outside `norm-js:test`.

## Changed files (areas)
- schemas: en1991/en1994?/en1998 diff schema, en1992 snapshot schema, en1992/en1993 nine mutation payload schemas, en1997 artifact schema, plus regenerated twins (ts/proto/graphql/rs) beside them
- din18599 editor field-meta `🦀️.rs`; din4108 schema `🟦️.ts`
- tests: insert-bindings (ts, feature, schema, cases), payload-bindings (ts, cases)
- oracles: din4108 and en1998 `🔮️oracles/🔣️.json`; en1998 seven new case fixture folders; 304 en1991 diff fixtures

## Browser-guest impact
Rust/schema/DSL-facing changes: en1998 artifact Rust (`#[dsl(table)]` on `cases`) and en1991/en1994 diff Rust (en1994 `cases` now Option), din18599 field-meta. Rebuild: norm-eurocode (en1991, en1997, en1998 TS/Rust twins), norm-eurocode-structural (en1992, en1993, en1994), norm-energy (din18599 field-meta). norm-eurocode-materials and norm: no source change.
