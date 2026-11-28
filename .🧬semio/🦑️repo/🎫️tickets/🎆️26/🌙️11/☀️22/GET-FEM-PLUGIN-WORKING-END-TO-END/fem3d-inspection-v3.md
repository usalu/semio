# 🧊️ FEM 3D matrix: inspection, tree and delete failures on the 10:29 wasm

Own headless Chromium against http://127.0.0.1:6087/ (outputs `🗑️generated/ie/run1` full set, `run2` re-runs; both removed or removable, drivers `matrix-v3/ie-driver.mjs`, `ie-vitest.config.mjs`). Run1 (`--only edit.,inspect.,tree.,del.`, 78 checks): 74 PASS, 1 SKIP (`element-section`, only one section exists), 3 FAIL; the 3 were fixed afterwards and re-run green in run2 (`dof`, `delete-term`, `add-term`).

## Verdict

One product defect (framework TS, live in vite, no Rust, nothing to publish). Everything else was harness: state leaks from check order, wrong selectors, wrong expectations.

| Check | Kind | Root cause | Fix |
|---|---|---|---|
| `edit.combination-delete-term` / `-add-term` | PRODUCT (TS) + harness | The Add/Delete Term controls are SELECTS (`combination.add-term`, `combination.remove-term`), not buttons (the check looked for buttons). Behind that: the interpreter's `SelectView` mapped an unset value `""` to `undefined`, which makes the `Select` uncontrolled; it then displays its FIRST option as if it were chosen, and picking that option is "no change", so no event. The first term could never be deleted or added (Add Term shows the placeholder only because it has no options in the Demo). Same in 2D (shared `SelectView`) | `✍️input-views/🟦️.tsx` passes the value as is (controlled, unset shows the placeholder, every option incl. the first is a change). Law `🧱️elements/🔽️Select/unset-value.test.tsx` (2 vitest tests green: unset controlled select shows the placeholder and commits its first option; undefined value shows the first option, the behaviour the old mapping caused). Live: Delete Term `Dead Load` (first option) 2 -> 1 terms, Add Term 1 -> 2. Checks now use the selects; the Demo's only combination already holds both cases, so Add Term first drops a term (3D Delete Term, 2D factor 0) |
| `edit.element-roll` | harness | Roll exists on Frame elements only (`element.roll` row is pushed for `FemElement::Frame`); `edit.element-kind` had turned `col_20_l` into a Bar before | check order: roll before kind (design is right, Bars have no Roll) |
| `edit.load-nodal-dof` | harness x2 | label is "Degree of Freedom" (terminology), not "Dof"; and `load-nodal-node` ran first and moved the load off `@ n20_l1` so the row regex missed | label fixed, dof before node |
| `edit.load-member-wx/wy/wz` | harness x3 | `addMemberUdl` args `elementId` and `caseId` are TEXT inputs, the check typed `#1` (an option index), so the action silently created nothing; labels are `Wx/Wy/Wz` (case-sensitive exact name); the row ends `N/m @ <element>`; the Demo has no member UDL, so the precondition is needed | precondition now passes `elementId=col_00_u, caseId=dead, wx=100, wy=200, wz=-1500` (config `memberTarget`/`memberCase` in `apps.mjs`), polls for the `N/m @` row (re-checked, not a context flag that survived reloads), edits `Wx|Wy|Wz` |
| `inspect.loadcase-pick-load` | harness | load pick buttons are named `l1 · Solid · sol1 800` (id · kind · detail), the regex wanted `@\|Load$`; form labels are capitalised | regex `^l\d+ · `, `Wx\|Wy\|Wz` |
| `tree.select-loadNodal` | harness x3 | (a) clicking a Load Case row toggles it collapsed (`aria-expanded=false`), so the load rows under it vanish for later checks; (b) `del.inspection-loadNodal` had deleted both nodal loads; (c) `edit.load-nodal-node` renamed it | `Ctx.openCases()` expands collapsed case rows when a load row is not found; `tree.groups-expand` starts fresh |
| `del.inspection-element` | harness | selects `col_20_u`, which `del.key-tree` had already deleted (click on a Delete button that no longer exists) | `element3 = col_02_u` |
| `del.inspection-support` | harness | selects `ss_0` on node `sc0`, which `del.inspection-node` had removed with its dependents | `supportB = ss_2` |
| `del.refusal-de` | stale row | no longer in the harness (dropped by the previous agent, the DE switch from a check does not take effect); the row in the coordinator's `matrix-v3/3d` is from an older run | none; German is asserted by the native law |

2D keeps the old row names (`element3`/`supportB` fall back to `element2`/`support`, the 2D ids were not verified live here; `memberTarget "e5"`, `memberCase "dead"` are guesses). The 2D add-term check uses the 2D select by label; 2D has no Delete Term select (factor 0 removes a term, `del.combination-term-zero`).

## Observations, not changed

- Poisson Ratio is a SLIDER (0 to 0.49). The harness slider path sets only Home/End, so `edit.material-nu` reads `0.3 -> 0` for "wanted 0.25" and still passes (the pass condition for sliders is "changed"). Weak assertion, not a product defect.
- Order effects remain by design: `del.*` run before `inspection/tree` (RANK); the first rank-3 checks are now `fresh` (`del.palette-remove-selection`, `del.combination-term-zero`, `focus.entity`, `tree.groups-expand`), so edits start from a clean Demo.

## Rust fixes needing the coordinator's publish

None. The only product change is framework TS (live in vite 6087/6086).

## Files

Product: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/✍️input-views/🟦️.tsx` (SelectView), law `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/unset-value.test.tsx`.
Harness: `matrix-v3/checks-model.mjs`, `checks-core.mjs` (`element3`, `supportB`, `focus.entity` fresh), `apps.mjs` (`memberTarget`, `memberCase`, `element3`, `supportB`), `lib.mjs` (`openCases`, used by `selectRow`), `ie-driver.mjs`, `ie-vitest.config.mjs`.
Cleanup: `C:\t\ie*` not created (no cargo run), my driver on 6391 stopped.
