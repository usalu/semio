# 🏗️ W11 — exhaustive fixture cases for all 25 fem3d mutation kinds

Scope: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/*/🧬️schema/🧬️mutations/<kind>/🧪️tests/**`,
the fem3d `#[cfg(test)]` mounts in `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs`, the five fem3d
`🔮️oracle/🔣️.json` catalogs and the fem3d `members-of-tests` rows of
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.

**No compiler was run** (host swap exhausted; the coordinator compiles). Every claim below is
either a static resolution, a `rustfmt` parse, a `jsonschema` validation, or a replay through the
plugin's own Python reference implementation — each named with its command and its output in §6.

Before: 25 kinds × **1** case. After: 25 kinds × **3** cases = **75** vectors, 50 authored here.

---

## 1. The new real-world model — the glulam workshop hall

Every one of the 25 second happy-path cases (`🏗️hall-…`) uses the SAME new `⬅️before`, authored in
`🔨️w11-author-fem3d-cases.py` (`hall_model()`) in the snapshot schema
`🌐️any/🧬️schema/📸️snapshot/🔣️.json`, SI throughout (Pa, m, m², m⁴, kg/m³, N, N/m, Pa):

> A **12 m span × 12 m long two-bay GL24h glulam portal hall** — three portal frames at y = 0/6/12 m,
> eaves at 4.2 m, ridge at 6.5 m, six pinned feet, glulam columns and rafters, C24 purlins along the
> eaves and the ridge, two M24 steel tie rods bracing the first roof bay, standing on a 350 mm
> C25/30 raft slab with a 4 × 4 m machine-pit hole, plus a door-apron slab.
> 17 nodes · 21 elements · 5 materials · 5 sections · 2 solids · 7 supports · 4 load cases ·
> 3 combinations · analysis {6 modes, 4 buckling factors, ×150 deformation}.

It is deliberately a different *structural system* from the existing `🧊️steel-frame.snapshot.json`
(an orthogonal two-storey steel space frame): pitched instead of orthogonal, three materials in one
load path instead of one, a solid with a hole instead of a plain rectangle, and mixed bar/frame
elements.

Material and section values are code values, not toy numbers:

| id | name | E [Pa] | G [Pa] | ν | ρ [kg/m³] | source |
|---|---|---|---|---|---|---|
| `gl24h` | GL24h Glulam | 1.15e10 | 6.50e8 | 0.30 | 420 | EN 14080 `E0,mean` / `Gmean` / `ρmean` |
| `c24` | C24 Solid Timber | 1.10e10 | 6.90e8 | 0.30 | 420 | EN 338 |
| `s355` | Steel S355 | 2.10e11 | 8.10e10 | 0.30 | 7850 | EN 1993-1-1 |
| `c25_30` | C25/30 Concrete | 3.10e10 | 1.292e10 | 0.20 | 2500 | EN 1992-1-1 `Ecm` |
| `gl32c` | GL32c Glulam | 1.35e10 | 6.50e8 | 0.30 | 440 | EN 14080 — reserved spare |

| id | name | A [m²] | Iy [m⁴] | Iz [m⁴] | J [m⁴] |
|---|---|---|---|---|---|
| `sec_col` | GL24h Column 200×400 | 0.08 | 1.06667e-3 | 2.6667e-4 | 7.33e-4 |
| `sec_raf` | GL24h Rafter 180×600 | 0.108 | 3.24e-3 | 2.916e-4 | 9.2e-4 |
| `sec_pur` | C24 Purlin 100×200 | 0.02 | 6.667e-5 | 1.667e-5 | 4.58e-5 |
| `sec_tie` | M24 Tie Rod | 3.53e-4 | 1.03e-8 | 1.03e-8 | 2.06e-8 |
| `sec_strut` | SHS 100×100×5 | 1.84e-3 | 2.79e-6 | 2.79e-6 | 4.45e-6 — reserved spare |

Loads: dead (self-weight on, 2.7 kN/m roof UDL + 2.5 kPa on the raft), snow (4.32 kN/m on both
rafters of the middle frame), wind X (5.4 / 10.8 / 5.4 kN at eaves level), plus a reserved crane
case. Combinations `uls_str` {1.35 dead, 1.5 snow, 0.9 wind}, `sls_char`, and a reserved
`sls_qp_spare`.

**Why the spares.** No `create-` verb in this vocabulary carries an index, so the inverse of a
`delete-` is exact only for a **trailing** record (the same limit the existing fixtures document).
The hall therefore ends every collection with one unreferenced spare — `gl32c`, `sec_strut`,
`tie_spare`, `slab_apron`, `sup_ext_a`, `crane_spare`, `sls_qp_spare`, `n_ext_b` — so each delete
case has an unambiguous target whose inverse round-trips exactly. Verified: all 25 inverse
round-trips hold in the Python reference (§6).

---

## 2. Kind × cases

`(a)` is the pre-existing vector, `(b)` the new hall happy path, `(c)` the new edge branch.

| kind | (a) existing | (b) `🏗️hall-…` | (c) edge | (c) code |
|---|---|---|---|---|
| `create-node` | `📍️appends-the-column-head-node-n3` | `🏗️hall-new-node-b26700` | `🚨️dup-node-id-86f2e1` | `mutation.duplicate-id` |
| `delete-node` | `🚫️removes-the-column-head-056295` | `🏗️hall-cut-node-8350fd` | `🚨️no-such-node-4027a8` | `mutation.target-missing` |
| `create-element` | `➖️appends-a-diagonal-bracing-bar` | `🏗️hall-new-tie-074a69` | `🚨️dangling-start-ab4132` | `mutation.target-missing` |
| `delete-element` | `🚫️removes-the-bracing-be89d2` | `🏗️hall-cut-tie-c268d4` | `🚨️no-such-element-eb788c` | `mutation.target-missing` |
| `replace-element` | `🔄️rolls-the-column-50f732` | `🏗️hall-strut-d0e4b7` | `⏸️same-element-61adb2` | `mutation.no-op` |
| `create-section` | `🔳️appends-a-square-bd0e4e` | `🏗️hall-new-beam-251a92` | `🚨️dup-section-id-a76686` | `mutation.duplicate-id` |
| `delete-section` | `🚫️removes-the-spare-30ecfb` | `🏗️hall-cut-shs-d44b9e` | `🚨️no-such-section-50d29b` | `mutation.target-missing` |
| `replace-section` | `🌀️raises-the-torsion-296ef0` | `🏗️hall-deep-purlin-176fd0` | `⏸️same-section-d1d013` | `mutation.no-op` |
| `create-solid` | `🏠️appends-an-extruded-roof-slab` | `🏗️hall-new-slab-d79da4` | `🚨️dangling-mat-1ebd78` | `mutation.target-missing` |
| `delete-solid` | `🚫️removes-the-roof-slab-f0fb64` | `🏗️hall-cut-apron-6c79d3` | `🚨️no-such-solid-f08d23` | `mutation.target-missing` |
| `replace-solid` | `📚️thickens-the-slab-and-b51ef0` | `🏗️hall-thick-raft-cddc0f` | `⏸️same-solid-8ad12c` | `mutation.no-op` |
| `create-material` | `🪙️appends-an-9fdced` | `🏗️hall-new-steel-0d2572` | `🚨️dup-material-id-1c0787` | `mutation.duplicate-id` |
| `delete-material` | `🚫️removes-the-b7b56a` | `🏗️hall-cut-gl32c-264bca` | `🚨️no-such-material-494b10` | `mutation.target-missing` |
| `replace-material` | `📉️softens-the-2cd183` | `🏗️hall-regrades-8dbc23` | `⏸️same-material-950f90` | `mutation.no-op` |
| `create-support` | `🔒️clamps-the-column-f801c9` | `🏗️hall-new-pin-c033f2` | `🚨️dangling-node-af37e2` | `mutation.target-missing` |
| `delete-support` | `🔓️releases-the-b3ebb0` | `🏗️hall-cut-pin-66d795` | `🚨️no-such-support-edd22a` | `mutation.target-missing` |
| `replace-support` | `🔄️frees-the-three-7783c9` | `🏗️hall-fixes-base-b5aa1b` | `⏸️same-support-bff8b3` | `mutation.no-op` |
| `create-load-case` | `🌬️appends-a-wind-case-a6c267` | `🏗️hall-snow-drift-068d9b` | `🚨️dangling-solid-5e04d9` | `mutation.target-missing` |
| `delete-load-case` | `🚫️removes-the-wind-caeb06` | `🏗️hall-cut-crane-52270d` | `🚨️no-such-case-ef1fde` | `mutation.target-missing` |
| `add-load` | `🏠️lays-an-area-pressure-over-769710` | `🏗️hall-adds-udl-e345cb` | `⏸️dup-load-id-4f4a0a` | `mutation.no-op` |
| `remove-load` | `➖️drops-the-trailing-member-b73b25` | `🏗️hall-cut-wind-6cf528` | `🚨️no-such-load-5bab2d` | `mutation.target-missing` |
| `change-load-case-self-weight` | `⏸️switches-self-7e0cda` | `🏗️hall-crane-sw-978370` | `🚨️sw-no-such-case-bfe5bc` | `mutation.target-missing` |
| `create-combination` | `🔗️appends-a-8ede20` | `🏗️hall-new-acc-4099b2` | `🚨️dangling-term-b9d144` | `mutation.target-missing` |
| `delete-combination` | `✂️removes-the-182f7b` | `🏗️hall-cut-qp-ebd806` | `🚨️no-such-combo-f42cd6` | `mutation.target-missing` |
| `update-analysis-settings` | `🔢️doubles-the-7b5381` | `🏗️hall-more-modes-ecbb5c` | `⏸️same-settings-fdb832` | `mutation.no-op` |

(c) totals: **3** Fatal `duplicate-id`, **16** Error `target-missing`, **6** Warning `no-op`.

### What each (b) case actually does

`create-node` sets out the extension-bay grid point · `delete-node` strikes the unused upper set-out
point · `create-element` closes the first portal with an eaves tie · `delete-element` strikes the
reserved gable tie · `replace-element` converts the M24 rod brace into a compression-capable SHS
strut (a `bar` variant swapped for a `frame` carrying a roll) · `create-section` coins a 200×800
glulam ridge-beam profile · `delete-section` strikes the unreferenced SHS profile · `replace-section`
deepens the purlin 200 → 220 mm · `create-solid` extrudes a plant-room slab · `delete-solid` strikes
the door apron · `replace-solid` thickens the raft 350 → 450 mm, sinks it to keep the top at ±0.00,
and refines the mesh through two layers · `create-material` adds S235 · `delete-material` strikes the
reserved GL32c · `replace-material` re-bases the glulam from mean to 5-percentile stiffness ·
`create-support` pins the free set-out point · `delete-support` releases the reserved clamp ·
`replace-support` upgrades a portal foot from pin to moment base · `create-load-case` opens a
snow-drift case carrying a member UDL · `delete-load-case` strikes the reserved crane case ·
`add-load` hangs services from the ridge purlin inside the dead case · `remove-load` drops the gable
wind node load · `change-load-case-self-weight` switches the crane case's self-weight on ·
`create-combination` opens an accidental combination · `delete-combination` strikes the reserved
quasi-permanent one · `update-analysis-settings` doubles the modal count and halves the display
exaggeration.

---

## 3. Refusal semantics, per kind, read off the code

Refusal lives entirely in each kind's own `🔺️diff/🦀️.rs`. Paths below are relative to
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/`.

| kind | guard | severity | file:line |
|---|---|---|---|
| `create-node` | duplicate `node.id` | Fatal `duplicate-id` | `🕸️mesh/…/⚪️create-node/🔺️diff/🦀️.rs:9` |
| `delete-node` | id not present | Error `target-missing` | `🕸️mesh/…/🕳️delete-node/🔺️diff/🦀️.rs:9` |
| `create-element` | duplicate element id | Fatal `duplicate-id` | `🕸️mesh/…/🧩️create-element/🔺️diff/🦀️.rs:10` |
| `create-element` | `start` / `end` / `materialId` / `sectionId` unresolved, in that order | Error `target-missing` | `…:17`, `:20`, `:23`, `:26` |
| `delete-element` | id not present | Error `target-missing` | `🕸️mesh/…/🗑️delete-element/🔺️diff/🦀️.rs:9` |
| `replace-element` | id not present | Error `target-missing` | `🕸️mesh/…/♻️replace-element/🔺️diff/🦀️.rs:9` |
| `replace-element` | new value equals existing | Warning `no-op`, empty diff | `…:12` |
| `create-section` | duplicate `section.id` | Fatal `duplicate-id` | `🕸️mesh/…/📐️create-section/🔺️diff/🦀️.rs:9` |
| `delete-section` | id not present | Error `target-missing` | `🕸️mesh/…/✂️delete-section/🔺️diff/🦀️.rs:9` |
| `replace-section` | id not present / value equal | Error / Warning | `🕸️mesh/…/📏️replace-section/🔺️diff/🦀️.rs:9`, `:12` |
| `create-solid` | duplicate `solid.id` | Fatal `duplicate-id` | `🕸️mesh/…/🧊️create-solid/🔺️diff/🦀️.rs:9` |
| `create-solid` | `materialId` unresolved | Error `target-missing` | `…:12` |
| `delete-solid` | id not present | Error `target-missing` | `🕸️mesh/…/🚫️delete-solid/🔺️diff/🦀️.rs:9` |
| `replace-solid` | id not present / value equal | Error / Warning | `🕸️mesh/…/🔄️replace-solid/🔺️diff/🦀️.rs:9`, `:12` |
| `create-material` | duplicate `material.id` | Fatal `duplicate-id` | `🧱️material/…/🌱️create-material/🔺️diff/🦀️.rs:9` |
| `delete-material` | id not present | Error `target-missing` | `🧱️material/…/🗑️delete-material/🔺️diff/🦀️.rs:9` |
| `replace-material` | id not present / value equal | Error / Warning | `🧱️material/…/🔁️replace-material/🔺️diff/🦀️.rs:9`, `:12` |
| `create-support` | duplicate `support.id` | Fatal `duplicate-id` | `🛡️boundary/…/🛡️create-support/🔺️diff/🦀️.rs:9` |
| `create-support` | `nodeId` unresolved | Error `target-missing` | `…:12` |
| `delete-support` | id not present | Error `target-missing` | `🛡️boundary/…/🗑️delete-support/🔺️diff/🦀️.rs:9` |
| `replace-support` | id not present / value equal | Error / Warning | `🛡️boundary/…/🔁️replace-support/🔺️diff/🦀️.rs:9`, `:12` |
| `create-load-case` | duplicate `loadCase.id` | Fatal `duplicate-id` | `🏋️load/…/📋️create-load-case/🔺️diff/🦀️.rs:9` |
| `create-load-case` | a carried load's node / element / solid unresolved | Error `target-missing` | `…:18` |
| `delete-load-case` | id not present | Error `target-missing` | `🏋️load/…/🗑️delete-load-case/🔺️diff/🦀️.rs:9` |
| `add-load` | `caseId` unresolved | Error `target-missing` | `🏋️load/…/➕️add-load/🔺️diff/🦀️.rs:9` |
| `add-load` | load id already in that case | **Warning `no-op`**, not a duplicate-id Fatal | `…:13` |
| `remove-load` | `caseId` unresolved / load not in that case | Error `target-missing` | `🏋️load/…/➖️remove-load/🔺️diff/🦀️.rs:9`, `:12` |
| `change-load-case-self-weight` | `caseId` unresolved | Error `target-missing` | `🏋️load/…/⚖️change-load-case-self-weight/🔺️diff/🦀️.rs:9` |
| `change-load-case-self-weight` | flag already at that value | Warning `no-op` | `…:12` |
| `create-combination` | duplicate `combination.id` | Fatal `duplicate-id` | `🏋️load/…/🔗️create-combination/🔺️diff/🦀️.rs:9` |
| `create-combination` | a term's case id unresolved (BTreeMap sorted order) | Error `target-missing` | `…:13` |
| `delete-combination` | id not present | Error `target-missing` | `🏋️load/…/✂️delete-combination/🔺️diff/🦀️.rs:9` |
| `update-analysis-settings` | settings equal to current | Warning `no-op` | `📈️analysis/…/🎛️update-analysis-settings/🔺️diff/🦀️.rs:9` |

### The load-bearing subtlety a rejection fixture must encode

**A refused fem3d mutation still returns `Ok(())` from `apply_fem3d_mutation`.**
`🌐️any/🧬️schema/🧬️mutations/🦀️.rs:96` delegates to `vcs::apply_mutation`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:1192`), which is documented as *policy-agnostic*:
it applies the diff and returns the messages, leaving the merge-policy verdict to the caller. Because
`MutationOutcome::fatal`/`::error` force `diff = D::default()`
(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1146,1152`), the applied diff is a no-op, the
snapshot is unchanged, and the `Result` is `Ok`.

Consequence: the `"rejected"` branch of the boilerplate `declared_outcome_holds` that the existing
fem3d cases carry (`assert!(!applied, …)`) **would fail if any fem3d fixture ever declared
`status: "rejected"`** — it is dead code today because all 25 pre-existing outcomes are `"applied"`.
The 19 rejection cases authored here therefore assert on `MutationOutcome::messages()` (code, level,
target) and on snapshot invariance, never on the `Result`, and each says so in its docstring.

---

## 4. Findings — where the code accepts input it arguably should not

None of these were invented into a fixture; they are recorded here instead, per the brief.

1. **`add-load` does not resolve the load's target.** `create-load-case` resolves every carried
   load's node/element/solid (`📋️create-load-case/🔺️diff/🦀️.rs:12-22`), but `add-load` resolves only
   the case (`➕️add-load/🔺️diff/🦀️.rs:8-14`). A nodal load on a node that does not exist, a member
   UDL on a missing element, or an area pressure on a missing solid is **silently accepted**. Same
   asymmetry for the same payloads through two different verbs.
2. **No `replace-` verb re-resolves foreign keys.** `replace-element` accepts a new element naming
   nonexistent `start`/`end`/`materialId`/`sectionId`; `replace-solid` accepts a nonexistent
   `materialId`; `replace-support` accepts a nonexistent `nodeId`. Only the corresponding `create-`
   verbs check.
3. **A `replace-` can silently RENAME a record.** The payload's `id` selects the target, and the
   `new*` record carries its own `id`; nothing requires them to match.
   `apply_delta` (`🌐️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:88`) rejects only an id that COLLIDES with
   another row — a fresh id is accepted, so `replace-material { id: "gl24h", newMaterial: { id:
   "gl24h_v2", … } }` renames the grade and orphans every member pointing at `gl24h`.
4. **No `delete-` verb cascades or checks referrers.** Deleting a material still referenced by
   elements/solids, a section still referenced by elements, a node still referenced by
   elements/supports/nodal loads, a solid still referenced by an area load, or a load case still
   weighted by a combination all succeed and leave dangling references. For `delete-node` this is
   explicitly the specified behaviour (the pre-existing case name says so); for the other seven it
   is undocumented.
5. **`create-solid` performs no geometric validation.** A degenerate footprint (fewer than three
   outline points, or zero), `height <= 0`, `layers == 0`, `meshSize <= 0`, or a hole outside the
   outline are all accepted; the failure would only surface later in
   `fem3d_engine::meshing::resolve_geometry`. This is why the `create-solid` edge case pins the
   dangling-material refusal instead — there is no footprint refusal to pin.
6. **No physical-plausibility validation anywhere.** Negative or zero `e`/`g`/`rho`, `nu` outside
   (-1, 0.5), negative section `area`/`iy`/`iz`/`j`, and non-finite node coordinates are all
   accepted by `create-material` / `create-section` / `create-node`.
7. **`update-analysis-settings` is unbounded.** Any `modalCount`/`bucklingCount` (including values
   far exceeding the model's DOF count) and any `deformationScale` (zero, negative, non-finite) are
   accepted.
8. **The per-kind `🧬️.schema.json` is wrong for every enum-typed payload.** `add-load`,
   `create-load-case`, `create-element` and `replace-element` describe their `FemLoad`/`FemElement`
   argument as a `oneOf` of objects whose `kind` is the **PascalCase Rust variant name**
   (`"Nodal"`, `"MemberUdl"`, `"Area"`, `"Bar"`, `"Frame"`) and which declare
   `additionalProperties: false` with **`kind` as their only property**. The wire form is camelCase
   (`"nodal"`, `"bar"`, …) and carries the variant's real fields, so **every committed vector of
   those four kinds fails its own schema** — including the three pre-existing ones. 12 failures
   listed in §6. Not fixed here (`<kind>/🧬️.schema.json` is outside W11's ownership and is a
   generated projection of the Rust shape).
9. **The snapshot schema's `$defs` are empty.** `🌐️any/🧬️schema/📸️snapshot/🔣️.json` declares every
   record type as a bare `{"title": …, "type": "object"}`, so snapshot validation passes vacuously —
   it proves the nine members exist and nothing about their contents. (Already noted in the fem3d
   Python references' own docstrings as "a defect in the specification".)
10. **The Python reference under-validates four `create-` verbs.** See §5 — it applies where Rust
    refuses. That is a defect in the reference, not in the Rust.

---

## 5. Why the new cases are NOT wired into the `.feature` Examples tables

The brief asked that new cases be picked up by Rust, Python and the oracle catalog alike. Rust
(mounts) and the catalog are done. **Python cannot be extended without also editing files outside
W11's ownership, and doing half of it would break the feature.** The mechanism, read from the code:

* The subset-level features carry a `spec-vector` `Scenario Outline` whose `Examples` rows name
  `id` / `dir` / `fixture` (e.g. `🧱️material/🧪️tests/🧱️mutate-fem3d-1-material/🥒️.feature:105-113`).
* `materializeScenario`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:456`) expands each
  row to scenario id `` `${baseId}-${row.id}` ``, and line 427 rejects a **duplicate scenario id**.
  So a second row with `id = create-material` is a parse error.
* Giving the new row a distinct `id` (e.g. `create-material-hall`) yields scenario id
  `spec-vector-create-material-hall`, for which the reference registers no handler — each `🐍️.py`
  registers exactly `mutate-<kind>` / `inverse-<kind>` / `spec-vector-<kind>` for `kind in KINDS`
  (`🌐️any/🧪️tests/🧱️mutate-fem3d-1-any-material/🐍️.py:391-400`).
* Independently, the reference's own `observable()` law
  (`…/🐍️.py:248`) fails any vector whose `before == after` — so the 25 edge cases (rejections and
  no-ops alike) **can never** be replayed through `spec_vector_handler` as written.

**Handoff (2 files per subset, additive).** For the 25 `🏗️hall-…` vectors:
1. in each of the five `🪆️subsets/<subset>/🧪️tests/…/🥒️.feature`, add a second
   `Scenario Outline` tagged `@id-hall-vector @level-exhaustive @mode-differential` with the same
   three steps and `Examples` rows `| id | dir | fixture |` naming the `🏗️hall-…` directories from
   §2;
2. in the matching `🌐️any/🧪️tests/…/🐍️.py`, add
   `built = built.oracle("hall-vector-%s" % kind, spec_vector_handler(kind))` to `adapter()`.

This is de-risked: `🔨️w11-replay-fem3d.py` already replays all 25 through the real reference and all
four laws (`equals_committed`, `observable`, `touches_one`, `restores`) hold — see §6.

For the 25 edge vectors a `reject-<kind>` handler asserting refusal/no-op would be needed instead;
they are Rust-side only today, and that is stated in each case's docstring.

---

## 6. Verification — commands and output

All run from the ticket folder. **No cargo, bun, nx or git-mutating command was run.**

### (i) `#[path]` and `include_str!` resolution — `python3 🔨️resolve-mounts.py --tree`

```
── ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs
   checked:    443      resolved: 443      unresolved: 0
── include_str! across ✏️s/🔌️plugins/🏗️fem
   checked:    1314     resolved: 1314     unresolved: 0
TOTAL unresolved: 0
```

(W1 measured 343 / 855 on 2026-09-05; the tree now resolves 443 / 1314 because W10 and W6 are
mounting concurrently. W11's own share is 50 `#[path]` leaves and 231 `include_str!` reads —
5 per happy/no-op bundle, 4 per rejection bundle, which carries no `🔺️diff/🔣️.json`.)

### (ii) Rust parse — `rustfmt --edition 2024 < <file> > /dev/null`

```
rustfmt parse-check: 51 files, 0 failures
```

(the 50 new case files plus the pre-existing `⏸️switches-self-7e0cda`, which the emoji filter also
matched; a separate sweep over all 88 fem3d `🧪️tests/**/🦀️.rs` also parsed 0 failures.)
**This proves the files parse, not that they compile.**

### (iii) Python replay + JSON Schema — `uv run python 🔨️w11-replay-fem3d.py`

The driver loads each subset's own independent Python implementation from
`🌐️any/🧪️tests/*/🐍️.py` (framework host stubbed) and calls its own `validate`, `apply_mutation`,
`inverse_mutation`, `observable`, `touches_one`, `restores`, `equals_committed`.

```
vectors: 75
  agree       50    forward = committed after, touches-one, equals-committed and inverse all hold
  agree-noop   6    reference leaves the model unchanged, as the committed no-op claims
  refused     14    reference raises, as the committed rejection claims
  DIVERGE      5
schema problems: 12
structure problems: 0
```

* the **50 `agree`** are all 25 pre-existing vectors **and** all 25 new `🏗️hall-…` vectors;
* the **5 `DIVERGE`** are exactly the four foreign-key `create-` refusals plus the combination-term
  one — `🚨️dangling-start-ab4132`, `🚨️dangling-node-af37e2`, `🚨️dangling-mat-1ebd78`,
  `🚨️dangling-solid-5e04d9`, `🚨️dangling-term-b9d144`. The Python reference's `apply_mutation`
  checks only id-uniqueness on a `create-`, never a foreign key, so it applies where Rust refuses.
  **Finding 10**: the reference under-validates; the committed vectors follow the Rust, which is the
  subject under test.
* the **12 schema problems** are finding 8, and hit the pre-existing vectors too
  (`🔄️rolls-the-column-50f732`, `➖️appends-a-diagonal-bracing-bar`,
  `🏠️lays-an-area-pressure-over-769710`, `🌬️appends-a-wind-case-a6c267`). Every `⬅️before`/`➡️after`
  passes the snapshot schema (vacuously, finding 9).

### (iv) Structure — same script, `structure_problems()`

0 problems across all 75 vectors, checking: catalog `scenarios[]` ↔ on-disk `🧪️tests` children
**set-equality in both directions** (no `mutation-vector-unregistered`, no phantom); `id` is the
stem of `directoryName`; every `directoryName` registered in taxonomy `members-of-tests`; the
sibling leading-emoji namespace is collision-free inside every `🧪️tests` directory; and every
bundle is the closed 13-node source shape with exactly one of `🔺️diff/🔣️.json` /
`🔺️diff/🚫️.absent`. 8 advisories, all **pre-existing** case names longer than the 28-character
budget this ticket adopted (none authored here; all 50 new names are ≤ 26).

### (v) JSON and NFC

`taxonomy.json` parses; `members-of-tests` 1086 → 1136 names, **0 duplicates**. All five fem3d
`🔮️oracle/🔣️.json` parse. All 402 `🔣️.json` under the fem3d subsets parse. Every new directory
name is NFC.

---

## 7. Files changed

**Created** — 50 case bundles × 6 files = 300 files under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/{🕸️mesh,🧱️material,🛡️boundary,🏋️load,📈️analysis}/🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/`
(`🦀️.rs`, `🦠️mutation/🔣️.json`, `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`,
`🎯️outcome/🔣️.json`, and `🔺️diff/🔣️.json` or `🔺️diff/🚫️.absent`).

**Edited (additive only)**

* `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` — 50 `#[cfg(test)] #[path = …] mod tests_…;` blocks
  inserted beside each fem3d kind's existing mount (150 lines added, 0 removed by W11).
* `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/🪆️subsets/{🕸️mesh,🧱️material,🛡️boundary,🏋️load,📈️analysis}/🔮️oracle/🔣️.json`
  — 50 `scenarios[]` entries.
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — 50 `members-of-tests` rows,
  inserted after `🪙️appends-an-9fdced`.

**Ticket**

* `🔨️w11-author-fem3d-cases.py` — the model, the 50 vectors, the generators, and the three
  anchored registration patches (idempotent; `--cases` / `--patch` / `--manifest`).
* `🔨️w11-replay-fem3d.py` — the verifier of §6 (iii)/(iv)/(v).
* this report.
