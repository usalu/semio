# 🔎️ r5 Audit: BIM Mutation Leaves (diff-only mutations)

Scope: 88 mutation leaves under `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`, the diff type `…/🧬️schema/🔺️diff/`, the kit `…/🧬️mutations/🧰️kit/` and the shared helpers `🌊️cascade`, `🧵️elements`, `📍️placement`, `🦉️wall-geometry`, `🧿️horizontal-rules`. Read-only: no repo file was changed, no cargo build was run.

Method: textual gate scan (R8–R16) over non-test leaf code, full read of all 88 `🔺️diff`, `↩️inverse` and `🦠️mutation` sources, shared helper read, fixture and test-file counting, absorb-table check against design §3, catalogue diff against design §4.

## Summary counts

| Item | Result |
|---|---|
| Leaves audited | 88 (design §4 lists 86) |
| Gate hits R8–R16 (`&mut <Snapshot>`, `.apply(`, `apply_diff(`, `ApplyCapability`, `between(`, `apply_to`, `SetSnapshot`/`Restore*`, base clone mutated, `diff(` in inverse body) | 0 hard hits in leaf code. Soft hit: inverses reuse forward helpers (F7) |
| V1-SNAPSHOT-DIFF | 0 |
| V1-GENERIC-DIFF | 8 leaves whole-list replace (F5) plus 14 leaves with unfiltered patches (F4) |
| V2-DIFF-DERIVED-INVERSE | 0 strict; 9 leaves reuse a forward helper (F7) |
| V2-RESTORE-INVERSE | 0 |
| V2-EMPTY-INVERSE | 0 (every empty inverse is a refused or no-op forward) |
| V3-LEAF-APPLY / V3-HAND-MUTATION | 0 (all 88 use `dsl::MutationLeaf`/`dsl::Mutations`, delegate to `diff`/`inverse` modules) |
| V4-LAW-UNTESTED | 0: 88/88 leaves have a test calling `assert_mutation_inverse_sum_law`; 170/170 applied fixtures covered; 302 rejected fixtures, none law-tested; every leaf has ≥1 applied and ≥1 rejected fixture |
| V5-ABSORB | 0 for legal sequences; two invalid-sequence coalesces (F12) |
| Derived values stored | 0 (elevation, height-of-wall, length, area, volume, riser, tread absent from snapshot and payloads; `height`/`elevation` fields are authored) |
| Comments inside function bodies | 0 |
| `todo!`/`unimplemented!`/`[DEBUG]`/legacy/compat | 0 |
| Docstring findings | 17 files with literal `\u{…}`/`¶` escapes (F1); duplicate leading emoji in 6 files (F17) |
| Findings total | 2 High, 5 Medium, 10 Low |

## Findings table

| # | Sev | Leaf / scope | Rule/code | File(s) | Evidence | Suggested fix |
|---|---|---|---|---|---|---|
| F2 | High | create-wall, create-curtain-wall, create-column, create-beam, create-slab, create-roof, create-stair, create-railing, create-space, create-opening, create-grid-line, split-wall (`new_id`) | Integrity (no V-code); `mutation.duplicate-id` only per collection | `➖️create-beam/🔺️diff/🦀️.rs` checks `base.beams.contains_key` only; same pattern in every create leaf | Element ids are assumed globally unique by `elements::exists`, `elements::placement`, `elements::rename`, `cascade::closure::root` (inserts into every collection holding the id) and `properties`/`classifications` keyed by element id. A create of an id already used by a wall makes `delete-elements`/`rename-element` act on both | Every element create/split refuses `DuplicateId` when `elements::exists(base, id)`; add a test per family |
| F3 | High | delete-beam, delete-column, delete-slab, delete-roof, delete-stair, delete-railing, delete-space, delete-opening, delete-curtain-wall, delete-grid-line | Cascade gap (design §2, §4) | `✂️delete-beam/🔺️diff`, `🪦️delete-column/🔺️diff`, `🔻️delete-slab/🔺️diff`, … return a single-collection `ModelDiff` | `delete-wall`, `delete-storey`, `delete-building`, `delete-site`, `delete-elements` cascade `properties` and `classifications` through `cascade::closure`; the single-kind deletes leave orphan property/classification entries behind (comment "orphan-tolerant" in beam/column only). Same element, two policies | Route every single-element delete through `cascade::outcome`, or make `diagnostics` report orphans and refuse. Pick one policy and state it in design §2 |
| F1 | Medium | 17 files: `⬜️create-slab`, `🏔️set-roof-shape`, `🏘️delete-roof`, `🏠️create-roof`, `🔻️delete-slab`, `👣️set-roof-footprint`, `🔸️set-slab`, `🔷️set-slab-boundary` (diff + inverse each), `🧿️horizontal-rules/🦀️.rs` | Docstring corruption (AGENTS: docstrings start with a real emoji) | line 1 of each diff/inverse file; `horizontal-rules` lines 1, 11, 27, 53, 74, 80, 93 | Docstrings contain literal text `\u{1f53a}️`, `↩️`, `¶CreateSlab¶`; the emoji is not rendered and backticks became pilcrows | Rewrite the doc lines with the real emoji and backticks; grep `\\u` as a gate |
| F4 | Medium | set-material, set-wall-type, set-slab-type, set-roof-type, set-column-type, set-beam-type, set-window-type, set-door-type, set-building, set-site, set-project-info, set-grid-line, set-railing, set-space | L1 minimality ("names exactly the fields it changes") | `🖌️set-material/🔺️diff`: `patch(payload)` copies every payload field; `🗺️set-site/🔺️diff`: `SitePatch { name: payload.name.clone(), latitude: payload.latitude, … }`; `🏗️set-building`, `📇️set-project-info`, `🧭️set-grid-line`, `🔧️set-railing`, `🪑️set-space`, `🔅️set-window-type`, `🔑️set-door-type`, `🎚️set-column-type`, `🎞️set-beam-type` do the same | A payload restating one current value puts it into the diff and into the inverse. Only the all-equal case is a no-op. Filter every field with `.filter(\|v\| v != current)` as `set-column`, `set-beam`, `set-curtain-wall`, `set-stair` already do |
| F5 | Medium | set-wall-type, set-slab-type, set-roof-type (layers); set-project-info (phase_names); set-railing (path); set-slab-boundary (boundary, holes); set-site (boundary); set-roof-footprint (footprint) | V1-GENERIC-DIFF (whole-list replace, strict reading) | `🔨️set-wall-type/🔺️diff` `patch(payload)` → `WallTypePatch { layers: payload.layers.clone() }`; `🏕️`, `🟧️` identical; `📇️set-project-info` → `phase_names` | Ruling "collections are diffed per id/index, never as a whole-Vec replacement". Layer stack, phases and path are lists. Geometry loops (boundary, footprint) are one-field geometry and may be sanctioned as a whole-loop replace | Decide the ruling. Preferred: keyed/indexed row diffs (`layer_added`, `layer_removed`, `layer_modified`) for layers and phases. Otherwise add the geometry kinds to a written exception list in design §Rulings |
| F6 | Medium | PropertyValue (snapshot values) and set-element-property | Schema drift (design §2 "PropertyValue = Text \| Real \| Integer \| Boolean \| Length \| Area \| Volume \| Angle (typed)") | `✏️…/📸️snapshot/💠️values/🦀️.rs` ~line 237: `struct PropertyValue { kind, text: Option, number: Option, flag: Option }` | Invalid states are representable (kind Text with number set). `🧾️set-element-property/🔺️diff` re-validates with `typed()`, so the check lives in the leaf, not the type | Make `PropertyValue` a tagged enum (`Text(String)`, `Real(f64)`, …); delete `typed()`; regenerate `🔣️.json`, `.proto`, `.graphql`, `.ts` |
| F7 | Medium | move-elements, rotate-elements, place-elements (`↩️inverse` call `super::diff::changes`); set-curtain-wall (`patch(payload, wall)`); set-material, set-wall-type, set-slab-type, set-roof-type (`super::diff::patch(payload).negate(current)`); split-wall (`diff::plan`, `diff::carried`) | L2 "never derived from the forward diff" (soft: uses forward helper, not `m.diff(base).diff()`) | `🚚️move-elements/↩️inverse/🦀️.rs`, `🎡️rotate-elements/↩️inverse`, `🪧️place-elements/↩️inverse`, `🔆️set-curtain-wall/↩️inverse` (`use super::diff::patch;`, then `patch(payload, wall)`), `🖌️set-material/↩️inverse` | The inverse is still built from payload + base reads, so the law holds (L3 tests pass by construction). But it couples inverse to the forward helper, so a bug in `diff` changes the inverse | Move the shared computation into a neutral module both sides call (`elements::changes`, `patches::negate`), or inline the base reads in each inverse. Keep the leaf's own `🔺️diff` and `↩️inverse` as the only callers |
| F8 | Low | flip-wall `↩️inverse` | L2 preference "absolute setters" | `🪞️flip-wall/↩️inverse/🦀️.rs`: returns `FlipWall` (a relative toggle) | Toggle inverse is correct only because the flip is an involution | Emit `SetWallAxis { axis: wall.axis }` as `split-wall` does |
| F9 | Low | delete-curtain-wall vs delete-wall / delete-storey / delete-elements | Cascade policy inconsistency | `🦖️delete-curtain-wall/🔺️diff`: refuses `TargetReferenced` while openings are hosted; `💥️delete-wall` cascades hosted openings | Two policies for the same host kind | Cascade openings of curtain walls too (same closure), or refuse for walls too. Pick one |
| F10 | Low | create-column, set-column (`🏛️create-column/🔺️diff`, `🎛️set-column/🔺️diff`) `rise()` | Validation silently passes | `!rise(…).is_none_or(…)`: when `compute_storey_levels` lacks the target, `rise()` returns `None` and the check passes | A top constraint to an unresolvable storey is accepted | Return `TargetMissing` when `levels.get(..)?` is `None` |
| F11 | Low | create-column, set-column, placement.rs (`axis_length` from `wall_layout`), `🪞️`, `🧵️elements` | Derived re-computation in leaves | `compute_storey_levels(base)` called from leaf diffs; `placement.rs` imports `wall_layout::axis_length` | Reads of derived values for validation are allowed by design §0, but re-deriving them in leaves duplicates inference logic | Expose one validated query per derived fact from the inference module; leaves call it |
| F12 | Low | `KeyedDelta::absorb` (`🧬️schema/🔺️diff/🦀️.rs`) | V5-ABSORB (invalid sequences only) | `(Some(Deleted), Patched) => Deleted`, `(Some(Replaced \| Patched), Created) => Replaced` | Legal sequences match the design §3 table, so L3 holds. Illegal sequences coalesce silently instead of failing | Refuse or debug-assert on the two invalid pairs |
| F13 | Low | `KeyedDelta::inverse` (`🧬️schema/🔺️diff/🦀️.rs`) | Silent drop | `Patched(patch) => … base.get(id)?` inside `filter_map` | A patch on an absent id drops silently from the inverse | Return an error or keep the entry so the law fails loudly |
| F14 | Low | place-elements, move-elements, rotate-elements (`🧵️elements/placement_diff`, 🧵 lines ~203–221) | L1 minimality (strict) | Every placement field of the kind is written, e.g. Column `rotation` on a pure move, Slab `slope` always `Assigned` | Design says "exactly its placement fields", so this matches the letter of the design but not "fields it changes" | Emit only changed placement fields, or document the kind-record exception |
| F15 | Low | delete-*, cascade `closure` (`🌊️cascade`) | Doc/semantics | `closure` pins only `walls`, `curtain_walls`, `columns`, `stairs` top constraints | Correct today (these are the only `TopConstraint` holders in the snapshot) but implicit | Add a comment-free test or a `TopConstraint` holder list in the design |
| F16 | Low | catalogue vs design §4 | Catalogue drift | 88 implemented, 86 designed: extra `place-elements` (absolute placement, inverse of move/rotate) and `rehost-opening` (split-wall's re-host, split out) | Kind counts match across `🔣️.json`, `.proto`, `.graphql`, `.ts`, `KINDS` (88 each); no kind is missing | Add both kinds to design §4 with their purpose, or fold `rehost-opening` into `split-wall` |
| F17 | Low | docstring emoji uniqueness | AGENTS: "unique and fitting emoji" per docstring within a file | `🦉️wall-geometry/🦀️.rs` 🚫 ×9, 📏️ ×3, ✂️ ×2; `🧵️elements/🦀️.rs` 🔀️ ×3, 📍️ ×2, 🚫️ ×2; `📍️placement/🦀️.rs` ⚠️, 📍️, 📐️, 🔎️, 🚫️ ×2 each; `🌊️cascade/🦀️.rs` 🌊️ ×2, ↩️ ×2; `🦀️.rs` (mutations index) 🧬️ ×2 | Repeated leading emoji within one file | Give each docstring a distinct fitting emoji |
| — | Info | `🦠️mutation/🦀️.rs` apply path | Unverified | `🦀️.rs` (mutations index) `apply_model_mutation` calls `store::apply_mutation`; no local `store` module or import found in that file | Not compiled here; may be an external crate path | Confirm with cargo check: `"$T/🚦️gate.sh" audit -- cargo check -p semio-s-artifact-bim-model` |

## Gate rules R8–R16 per leaf (textual)

- `&mut <Snapshot>` in leaves: none. `&mut` appears only in `🧵️elements/patched(slot: &mut Option<KeyedDelta>)` (a diff slot, not a snapshot) and `🌊️cascade` `Removal::root(&mut self)`. Both allowed.
- `.apply(` / `apply_diff(` / `ApplyCapability` in leaf, kind, diff or inverse code: none. Hits exist only in `🧪️tests` (allowed) and in the editor/framework kits, which are outside this audit.
- `between(` in leaves: none.
- `apply_to`, `SetSnapshot`, `Restore…` variants: none.
- Base `.clone()` mutated in a diff: none (clones are record copies into patches).
- `diff(` inside an inverse body: none as a call on a diff. The soft reuse is listed in F7.

## Per-leaf sparsity (L1) and inverse (L2) status

- Create/delete leaves: 1 entry per id; create carries the full record (allowed); delete inverse carries the full captured record (allowed).
- Single-field `set-*` (`set-storey-height`, `set-storey-level`, `rename-storey`, `set-wall-top`, `set-wall-base-offset`, `set-wall-location`, `set-wall-type-of`, `set-wall-axis`, `set-roof-footprint`, `flip-wall`, `rename-element`): sparse. Inverses absolute.
- Filtered `set-*` (`set-column`, `set-beam`, `set-curtain-wall`, `set-opening`, `set-slab`, `set-stair`, `set-element-classification`, `set-roof-shape`, `set-space` no-op guard only): sparse, but see F4 for the rest.
- Multi-element: `move-elements`, `rotate-elements`, `place-elements` emit placement patches per element (F14); inverses are one `PlaceElements` of exact base placements (concrete, OK).
- `split-wall`: patch on original + create of second wall + sparse opening patches. Inverse: `SetWallAxis` + `DeleteWall` + `RehostOpening`s, storage-reversed correctly.
- `delete-storey`/`delete-building`/`delete-site`/`delete-wall`/`delete-elements`: one sparse `Deleted` entry per removed id via `cascade::closure`; inverse replays one create per removed record and one property/classification setter per data entry (dependants first).

## Cascade coverage (item 7)

- Complete: `delete-site` (buildings, storeys, grids, all contents, openings of removed walls and curtain walls, props, classifs); `delete-building`; `delete-storey` (walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces, openings, props, classifs; refuses if a surviving wall, curtain wall, column or stair still uses the storey as its top constraint, which is the only holder of `TopConstraint`); `delete-wall`; `delete-elements`.
- Gaps: F3 (single-element deletes do not cascade props/classifs), F9 (curtain-wall delete refuses instead of cascading).
- Type and material deletes refuse when referenced: wall/slab/roof/column/beam/window/door types each check their one referencing collection; `delete-material` checks all nine referencing collections (wall, slab, roof types' layers; column, beam, window, door types; curtain walls' panel and mullion; railings). Complete.

## Catalogue (item 9)

- Design §4 kinds (86) all present. No kind missing.
- Extra kinds: `place-elements`, `rehost-opening` (F16).

## Prioritized fix list

1. F2: refuse cross-kind duplicate element ids in every element create and in `split-wall`'s `new_id`; add one test per family. Highest risk: silent wrong deletes and renames.
2. F3: one delete policy for every element: cascade properties and classifications (or make diagnostics report orphans and refuse). Align inverses.
3. F4: filter unchanged fields in the 14 unfiltered `set-*` leaves, so diffs and inverses carry only real changes.
4. F1: rewrite the 17 corrupted docstring files with real emoji and backticks. Add a gate that rejects literal `\u{` and `¶` in `*.rs`.
5. F6: make `PropertyValue` a tagged enum to match design §2; then regenerate the schema mirrors.
6. F5: decide the whole-list ruling; then implement per-row diffs for layer stacks and phases, or write the geometry exception into design §Rulings.
7. F7 and F8: detach inverses from forward helpers; make flip-wall's inverse an absolute `SetWallAxis`.
8. F9 and F10: one cascade policy for curtain walls; refuse unresolved storey references in `rise()`.
9. F12, F13: refuse invalid absorb and inverse pairs instead of coalescing silently.
10. F14, F16, F17, F11, F15: documentation and style items.

Unverified: nothing was compiled or run. The disk on `C:` reported 0 bytes free during the audit, which is why this report is short on line numbers. Confirm the `store::apply_mutation` path and the gate tests with `cargo check` once the disk has space.
