# Norm mutation vectors, 2026-11-25

Status: in progress. en1992 ran once in the private copy (compiled, 0 passed 1 failed, remaining breaches classified and fixed in the real tree, not yet rerun). Other families not run. Verified: wire-twins `repo test-file` 11929 pass, 0 fail.

## Incident

`pkill -f "lane.py repo cargo"` at about 10:29 killed other agents' queued runs (carve agent core run, en1994 scene and checks runs). Never kill by pattern; only recorded pids.

## Findings per family (from the 2026-11-24/25 logs `mv-all-<family>.log`)

| Family | Finding | Where fixed |
|---|---|---|
| din4108 | 7 case leaves (`change-case-*`, `insert-case`, `remove-case`) declared `no-op`; diff code never emits it | descriptors (as en1990) |
| en1991 | 324 binary op encode failures: codec `kind_of` read a `"mutation"` key the wire no longer has; 223 descriptors disagree with dispatch | codec in `💾️binary/🧬️mutations/🦀️.rs` (kind from descriptor); descriptors |
| en1992 | 155 descriptors; 1 fixture `newLapShare: 2` not canonical (`2.0`); 98 produced diffs differ; 12 committed diffs do not reach the after-snapshot | descriptors, fixture, 98 diff.json regenerated (stale duplicated rows); awaiting rerun |
| en1998 | same stale `kind_of` codec as en1991 | codec fixed, not compiled |
| en1993 | codec already reads the first key | none |
| din18599 | `🟰noop` bundles commit no diff file and use the old `"mutation": "name"` wire; will fail the law | OPEN, not run |

## Descriptor and oracle justification

The log is circular (descriptor, oracle outcomes and law come from the same run). Independent rule, `derive-outcome-classes.py`: class `no-op` iff the leaf's diff source contains `"mutation.no-op"`, `rejected` iff it contains `MutationOutcome::error(` or `fatal(`. It sets descriptor and oracle `outcomes` from source. Result: en1991 0 descriptors changed (324 match), din4108 0 (36 match), en1992 1: `change-member-order` gains `no-op` (source: from == to after clamping). Oracle entries now follow the source rule instead of the log.

`change-member-order` got a `🟰noop` bundle (indices 0,0; hand-written diff: every `En1992Diff` key null, `cases` the empty `CaseRowDelta`, as `cases` is not an `Option`).

## en1992 run (private copy, `cp-mv-en1992.log`, 0 passed 1 failed) and the diff classes

Remaining breaches: 98 "produced diff is not the committed diff", 12 "committed diff does not turn before into after", 8 inverse. No descriptor, codec or wire breach is left in that run.

Class (all 98, classified with `classify-diff-mismatch.py`): the committed `members.modified` holds the same member row up to 8 times (90 leaves; in the remaining 8 the first row is real and rows 2..8 carry empty patches), plus the member patch lacks the newer keys (`bearingWidth`, `punchingBetaMethod`, `punchingMomentC1/C2`, `openings`, `tightness`, `clearTightness`, `actions`, `bars`, ...). After dropping nulls, empty deltas and duplicate rows, committed and produced are equal for 98/98. The produced single row is the one the diff source builds (`En1992MembersRows::modification(id, patch)` yields one row); the mutation code is right, the committed data is stale authoring output (rows cloned by the earlier fixture scripts). Regenerated all 98 `diff.json` with `regenerate-mutation-vector-diffs.py` from `cp-produced-en1992`.

The 12 insert/remove leaves are among the 98: applying the committed diff replays the same insertion or removal 8 times, which the applier refuses (`mutation.apply.duplicate-target`) or which shifts rows. My earlier hand application read only the first modified row, which is why it looked correct. After regeneration each holds one row, which by hand yields the committed after for section point, action and bar layer.

## Scan of all families (`scan-vector-defects.py`, read-only)

Repeated `modified` row ids in committed diffs: 0 in every family (en1992 after regeneration). `no-op` bundles without `diff.json`: only din18599, 21 bundles; each now has a hand-written empty diff (every `Din18599Diff` key null; all its fields are `Option`), after checking before == after.

din18599 wire tag, decided from `cp-mv-din18599.log`: the first committed vector fails to decode with ``unknown `mutation` variant `insertzone` ``, so flat-lowercase is not the Rust wire; `Din18599Mutation` declares `tag = "mutation", rename_all = "camelCase"` like en1990/en1999/en1997. Moved together: 84 `mutation.json` fixtures, the 25 leaf `🧬️schema/🔣️.json` consts, the 25 leaf `🟦️.ts` readers and the aggregate `🟦️.ts` parse table to camelCase (variant name with lowercase first letter). The twins generator (`schema twins generate --under ✏️s/🔌️plugins/📕️norm`) reports identical=8321 drift=0 before and after, i.e. it does not cover these din18599 files, so the TS files were edited by hand. Verified: `repo test-file …/🧪️wire-twins/🟦️.ts` 11956 pass, 0 fail (no lane).

Side effect found by that test: the 98 regenerated en1992 diffs were not canonical for the TS twin, because the Rust runtime serialises `En1992MembersPatch` in a different key order (`useFem` at position 14) than the committed schema and TS twin (`stirrupsDiameter` at 14). The fixtures were reordered to the schema order (same 52 keys). OPEN (not fixed, a schema vs runtime disagreement the Rust law cannot see): the member patch key order in the en1992 diff schema/twins versus the Rust serialiser.

## en1997 (`cp-mv-en1997.log`, 0 passed 1 failed)

Only 3 breaches: `change-geotechnical-category`, `change-groundwater-level`, `change-investigation-depth` declare `rejected` (their sources emit `MutationOutcome::fatal`) but no committed vector is rejected. Added `⛔range` bundles (before = after, fatal `mutation.invariant`, no diff) for category (4) and investigation depth (-1.0). Not added for groundwater level: its refusal is `!is_finite()`, which the JSON wire cannot spell, so the descriptor class is unreachable from committed vectors; decision needed (drop the dead guard in the mutation source, which belongs to the en1997 work of another agent, or accept the breach). The mutation source was not touched.

## en1994 (`cp-mv-en1994.log`, first failing vector `insert-column/✅apply`)

- Cause of the panic: `insert-column`'s diff source had no duplicate-id guard (its siblings `insert-beam` etc. have one), so the law's re-application of the mutation to the after-snapshot produced a second `col-new` and the applier refused with `mutation.apply.duplicate-target`. Fixed in the mutation source: `➗️insert-column/🔺️diff/🦀️.rs` now returns `mutation.duplicate-id` like `insert-beam`. Not compiled yet.
- Row scan (`scan-vector-rows.py`, `inserted`, `removed`, `moved`, `modified` ids in every committed `diff.json` of all families): 0 repeats everywhere, so the en1992 defect (duplicated `modified` rows) is specific to en1992.
- 24 produced diffs: 20 equal the committed ones after dropping nulls, empty row deltas and repeated rows (committed lacks the newer keys `cases`, `placements`, `studs`, `reliabilityClass`, `annex`, ...: stale authoring form). 4 differ: `change-beam-adjacent-span-m` (committed patch leaves `adjacentSpanM` null although the committed after moves 0.5, so the committed diff never reached its after: wrong; produced `0.5` right) and `insert-slab`, `remove-slab` (x2) (committed `reliabilityClass: "RC2"` where before and after are both RC2: the stale class of en1999).
- Decision executed (coordinator): en1994 `cases` is a plain required `CaseRowDelta`. Changed: `🧬️schema/🔺️diff/🦀️.rs` field, `🔁️apply-and-inverse/🦀️.rs` (absorb, commit_onto, is_empty, inverse), the 7 case mutation diff sources (`cases: Some(x)` becomes `cases: x`), and the schema JSON (`"required":["cases"]`, as en1992). The twins generator is one-way schema JSON to Rust/TS/proto/GraphQL; it first overwrote my Rust edit back to `Option` until the schema carried `required`. After the schema edit it rewrote TS, proto and GraphQL and a second run reports `identical=8321 drift=0`.
- The 24 produced diffs became the fixtures (with `cases` as the empty delta, in schema order via `reorder-diffs-to-schema.py`); the other 109 committed en1994 diffs were filled mechanically with `fill-diff-form.py` (every schema key present, `null` or empty, required arrays `[]`). Validation of that fill: applied to the committed form of the 24, it equals the produced diff for 2 of 24 exactly, and for 20 of the other 22 after dropping repeated rows (the 4 real differences above). So the mechanical fill is unverified for the 109 until the next en1994 run reports any stale content (for example `reliabilityClass` of a class).
- wire-twins `repo test-file`: 11956 pass, 0 fail.
- `insert-column` guard is a real production bug fix (production accepted a duplicate column id), not only a test fix. Check of `insert-*` in all families for a missing `duplicate` guard: only en1990 `insert-effect` lacks one, and its rows (`MemberEffect`: memberId, actionId, influence) carry no key, so there is nothing to duplicate.

## Key order of `En1992MembersPatch` (decision: schema is canonical), investigated, NOT changed

The patch struct is generated by `nested_patch!` with groups `set`, `within`, `optional`, `rows` in that order, so serialisation follows the groups; the schema and TS twin follow the model order (`stirrupsDiameter` right after `bucklingLength`, late additions `punchingBetaMethod`, `punchingMomentC1/C2`, `openings` at the end). Fields of different groups interleave in the schema, so no reordering of the macro arguments can reproduce it; it needs a macro that keeps the declared order. The proto twin numbers fields in schema order (`stirrups_diameter = 15`, `use_fem = 29`). Not yet checked: pack/binary op encoding, DSL order, SQLite columns, committed binary fixtures. Nothing reordered in Rust (see the law below).

## en1997 groundwater (`!is_finite()` guard) precedent

Scan of every leaf whose only refusal is a guard containing `is_finite`: din18599 (3 leaves) and en1997 investigation depth combine it with a range (`<= 0.0`) and are reached by a range vector (`🚫rule`, value -1.0). The pure finite guards without a range (en1997 `change-groundwater-level`; en1994 `change-steel-fy-pa`, `change-insulation-thickness-m`) have no rejected vector anywhere: no precedent that makes the class reachable. JSON cannot spell a non-finite number, so no committed vector reaches it; the binary op codec test (`binary_breaches`) round-trips the committed mutation only and does not feed a NaN either. Descriptor left as the source rule gives it; the breach stays open for en1997 groundwater and, on their first en1994 run, for the two en1994 leaves.

## en1995 and en1996 (`classify-outcome-breaches.py`)

- en1995 (206 breaches): 203 leaves declare `no-op` and 3 `rejected` that their sources cannot emit (no `"mutation.no-op"` / no `error(`/`fatal(`): classes dropped, descriptors and oracle derived from source (`derive-outcome-classes.py`). No missing vector: every class the source can emit is reached.
- en1996 (116 breaches): the source rule gives the declared set plus `no-op` for 103 leaves whose vectors already reach it (the declaration was missing a reachable class: added), and drops `no-op` from 13 whose source cannot emit it. Descriptors and oracle derived from source.
- en1996 five vectors authored with a mutation value equal to the base value (`change-tie-position` 1.0, `change-flange-position` 0.0, `change-lintel-position` 0.6, `change-lintel-span` 1.4, `change-confined-depth` 0.24): before equals after yet marked `applied`, so production correctly raised `mutation.no-op`. Wrong side: the fixtures. New values 1.5, 0.5, 0.8, 1.6, 0.3 in mutation, after and diff. `change-wall-web-spacing-m`: committed diff had an empty patch although the after moves `webSpacingM` 0.0 to 1.5; produced diff (patch `webSpacingM: 1.5`) right, regenerated. Wire-twins still 11956 pass.

## Key order law and macro

Law added in `✏️s/🧪️tests/🧪️mutation-vectors/🦀️.rs`: when produced and committed diff are equal as values, `key_order` (all object keys in document order) must be equal too, so a Rust serialiser that drifts from the committed (schema and TS-checked) order is a breach for every family at once. Unit test `the_key_order_reads_every_object_key_and_no_string_value` added. Not compiled. Dependency check for the declared-order `nested_patch!`: the binary diff form (`💾️binary/🔺️diff`) is an opaque JSON payload (ksy: `payload size-eos`), the text diff is JSON, the proto twin is numbered from the schema (already schema order); no SQLite or binary committed fixture holds a diff. The only dependence on order is JSON text. The macro itself was not changed: copies of `nested_patch!` exist per family diff crate and need compile feedback.

## en1994 and din18599 descriptors after the coordinator's source comparison

`derive-outcome-classes.py` on en1994 changed 33 descriptors (and their oracle outcomes) and on din18599 1, to the source rule: 13 en1994 leaves drop `no-op`/`rejected` their source cannot emit (`remove-column`, `insert-slab/beam/column/case`, `remove-slab/beam/case`, `change-case-*`, `change-case-entry-id`), 10 become `[applied, rejected]` (`insert/remove-beam-action`, `insert/remove-column-action`, `insert/remove-slab-action`, `insert/remove-placement`, `insert/remove-stud`), and 7 gain `rejected` (`change-beam-action-xm`, `change-slab-action-xm/ym`, `change-placement-x/y/yaw/z`), plus din18599 `change-element-u`. `change-structure-kind`, `change-fire-rating`, `change-fatigue-detail` drop `rejected`.

Which vector reaches each class: every class gained has a payload addressing field (`id`, `beamId`, `slabId`, `columnId`, `elementId`, `index`), so the law's stray-target step (`id` becomes an unknown id, `index` becomes `u32::MAX`) reaches `rejected` from the existing `✅apply` vector, provided the source refuses a missing target with `error(`/`fatal(` (the same code the rule found). `no-op` of the scalar leaves is reached by re-applying the mutation to the after-snapshot. Therefore no vector was added; this stays unproven until the en1994 and din18599 runs, and a leaf whose stray target is not refused will show as a breach naming the class.

## The 109 mechanically filled en1994 diffs

Produced by `fill-diff-form.py` (kept in the ticket folder): every schema key present as `null`/empty, required arrays `[]`, keys in schema order. AGENTS.md discourages fixture scripts; the script is not the proof, the next en1994 `mutation_vectors` run is (it compares each fixture with the produced diff). Check done here: for all 109 files the fill changed nothing beyond null or empty keys and key order (compared with `git show HEAD:` after dropping nulls and empties): 0 leaves. The 24 diffs with a produced twin come from `cp-produced-en1994`, not from the fill.

## Checks covering the hand-edited din18599 TypeScript

The twins generator reports din18599 files as out of its scope (identical=8321 before and after the edit). Covered by: `repo test-file …/🧪️wire-twins/🟦️.ts` (the aggregate reader parses every committed `mutation.json` and the diff and snapshot fixtures of din18599 and compares canonical bytes), and, when it runs, the Rust `mutation_vectors` wire law (`committed mutation is the canonical Rust wire`).

## Open

en1992/en1991/din4108 real runs; en1998, din18599, en1997, din16798, iso16757, en1993, en1994, en1995, en1996 never run; the schema crate sweep; TS oracle and wire-twin tests per family.
