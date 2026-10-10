# 🔎️ R13 Audit — Mutation Laws Across All 173 Leaves (Haiku, read-only)

S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`; paths relative to S.

## Summary
| Law | Result |
|---|---|
| L1 diff purity | No `&mut` snapshot, `.apply(`, `between(` or base clone in any leaf. BUT 3 leaves import `💡️inferences` directly and ~32 reach inference/geometry evaluation through shared helpers (placement, wall-depth, modify, family-rules). |
| L2 concrete inverse | Clean: no generic undo/diff-walking helper; 6 inverses reuse forward-diff helpers (coupling only). |
| L3 sum-law test | 173/173 call `assert_mutation_inverse_sum_law`. |
| L4 fixtures | All leaves have applied + rejected fixtures; **306 of 533 applied fixtures unblessed** (`{}` diff/after) over 101 leaves. |
| L5 verbs | All declared `SEMANTICS.verb` pass `APPROVED_VERBS`; 8 kind names start with a non-listed word (copy, array, align, mirror, trim, offset, flip, place). |
| L6 whole-list | Violations: `set-classification-system.entries`, `set-property-template.properties` (+`applies_to`); borderline `set-area-scheme.usages/zones`. |
| L7 registration | 173 dirs = 173 enum variants = 173 KINDS. |

## L1 violations
1. `🔪️trim-extend-wall/🔺️diff:12,38` → `inferences::wall_layout::axis_length`.
2. `🔡️set-family-parameter`, `🔠️remove-family-parameter` diff+inverse → `inferences::families::formula::parameter_id`.
3. `📍️placement/🦀️.rs:222,233-241` `storey_elevation`/`rise` call `storey_levels::resolve` (doc claims "runs no inference
   engine" — contradicts ruling 7); used by create/set-column, set-element-storey.
4. `📍️placement/🦀️.rs:5-7,26-27,82` imports `opening_frames::resolve_size`, `storey_levels`, `wall_layout::axis_length`
   (13 leaves: openings, railings, stairs, ramps, set-element-storey `overflowing_opening`).
5. `🧗️wall-depth/🦀️.rs:5-6,27,66-69` element_solids profile evaluation + `wall_layout::attach::{edges, find_cycle,
   top_target}` (set-wall-top, create-wall, openings, base slab, sweeps).
6. `🧙️modify/✂️cut:8,10,74,80,103,112`, `🧙️modify/🗺️map:9,300` → `wall_layout::segment_of` + framework geometry
   (split-slab, copy/array/align/mirror, trim-extend, offset, end-join).
7. `🔩️family-rules/🦀️.rs:5,38,…` → `inferences::families::formula` (parse-only).
8. `🧿️horizontal-rules` framework intersection checks (validation only, low).
9. `🦉️wall-geometry/🦀️.rs:235` re-exports `element_solids::columns::MAX_TILT` (constant, low).

## Other
- `PropertyKind` still defined (`💠️values/🦀️.rs:5`, used by `PropertyDef.kind` :688) though ruling 6 retired it.
- Unblessed fixture list per leaf: see the agent transcript summary (101 leaves; e.g. `🔭️set-view` 13, `📑️set-sheet` 9,
  `📔️set-viewport` 9, `🏧️set-curtain-wall-grid` 7, `📈️set-schedule` 7).

## Coordinator rulings (binding for `r13-laws`)
- **R-L1**: a diff/inverse may call only (a) the snapshot's own accessors, (b) pure functions over AUTHORED data that live
  outside `💡️inferences`. Move every pure helper used by diffs (`axis_length`, formula parse/`parameter_id`/
  `references`/`solid_slots`, profile extents/polygon of authored profiles, attach-graph edges/cycle over authored
  references, authored segment geometry used by modify) into a shared authored-geometry module under
  `🧬️schema/` (e.g. `🧬️schema/📏️authored`), imported by both inference and mutations. `storey_elevation` becomes a pure
  sum of authored storey heights/levels (no resolver). `resolve_size` for openings → authored size resolution helper
  (type defaults + overrides), not the inference module. Framework geometry validation of authored loops stays allowed.
- **R-L5**: accepted as is (r9-decisions: closest approved verb, semantic kind name).
- **R-L6**: classification-system entries become a keyed map (code → item) with `set-classification-entry` /
  `remove-classification-entry` kinds; property-template properties become a keyed map (name → def) with
  `set-template-property` / `remove-template-property`; `applies_to` and area-scheme `usages`/`zones` are owned
  reference SETS (BTreeSet), whole-set replace accepted.
- Remove `PropertyKind` (use `PropertyValue` kinds).
- All 306 unblessed fixtures are blessed by `r13-integrate` and verified by the mutation oracle.
