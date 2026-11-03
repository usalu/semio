# Open-items sweep v2: en1990 and en1997

Nothing compiled (no cargo/rustc, no builds). `freeze.txt` named only `build-en1994` during the work. Rebuild lines appended to `🗑️generated/build-en1990/latest.md` and `build-en1997/latest.md`; requests at the end of `📌️important/requests.md`. No git writes. No normative value invented or changed. Roots: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<⚖️en1990|🌍️en1997>/🏅️standards/🔖️1/🪆️subsets/✳️any/` ("ANY").

## 0. Extra item (coordinator): en1997 io missing dependency
`ANY/🚪️io/💾️binary/🧬️mutations/🦀️.rs` uses `semio_s_artifact_norm_en1997_mutations_protocol::LEAF` (3 sites) but `ANY/🚪️io/📦️packages/🦀️rust/Cargo.toml` did not list the crate. Added `semio-s-artifact-norm-en1997-mutations-protocol = { workspace = true }` (workspace dependency exists, `🌍️en1997/Cargo.toml:335`). The io-text crate does not use it (its `protocol::` hits are the framework protocol), so unchanged.

## 1. CHECK-HIGHLIGHT ownership law (read every emitted family)
Law: owner after `Owners::file` = subject row of an owning collection, else a row id embedded in the check id, else a row naming the subject; an empty owner needs a documented document-level prefix. Owners must also be scene entities (`ENTITY_KINDS`/`rows_of` or design cases).

### en1990: clean, no change
| family | owner |
|---|---|
| `str/geo/equ.stab/fat/a14.*/a2.rail-*/a2.transient/a2.traffic-group-(combination|governing)/actions.*/e10-e12` | member (or bridgeSls row) |
| `annex-c.form/partial-factors`, `annex-d`, `override.*`, `a2.traffic-group.<row>`, `integrity.duplicate.*` | analysis / test series / factor override / traffic-group row / duplicated row |
| `table-2.1.*`, `a2.working-life`, `a2.traffic-groups`, `members`, `annex-b.*` (rc-vs-cc, dsl, il, kfi, kfi-applied), `annex-c.beta`, `input.*`, `integrity.*` | document-level (reason: project subject or no entity) |
All 12 `OWNERS` collections are scene entities (`🎬️scene/🧰️kit rows_of`: members, permanents ... trafficGroups, cases). Dangling refs (`integrity.effects.*`, `integrity.bridgeSls.*.memberId`) have a missing id as subject and are cleared by the `integrity.` prefix.

### en1997: one orphan found and fixed
`en1997.design.selection` (subject `""`, `designApproach`) had no owner and no prefix (`gk.category` was listed, this sibling was not). Added the document-level prefix `en1997.design.` in `ANY/🧬️schema/💡️inferences/⚖️checks/🧮️units/🦀️.rs` (OWNERS). Reason: concerns the whole project's approach/situation.
Other empty-subject families: `gk.category`, `*.na` (6), `governing.situation` (prefix `governing.`), all listed; `2.4.2.investigation.<footing>` embeds the footing id; `6.6.tilt.*` already points at the first placed footing; `integrity.duplicate/link.*` rows or `integrity.` prefix; catalogue results are retargeted to their entity (`retarget`).

## 2. 3D, grips, outliner, diagrams (audit false negatives corrected)
Outliners render `ENTITY_KINDS` first then every other entity list, so all collections have outliner entries.
### en1990
- seismics: grip exists (`action_handles`, `leaf_of` -> `aEk`); testSeries: grips via `analysis_handles` (mean/required bars); false negatives.
- Gap closed: `bridgeSls` grips `deflection` and `deflection-limit` (`🎬️scene/🕹️handles/🦀️.rs`, `bridge_handles`, writes `bridgeSls[id].bridgeDeflection|bridgeDeflectionLimit`, exaggeration as the deflection line; field-meta rows and `change-bridge-deflection*` kinds already exist). bridgeSls diagrams exist (`rail-frequency-limits`, `rail-resonance`).
- Still without a grip (by design/reason): `effects` (nested influence rows, no geometry), `trafficGroups` (the active group is stepped by the group selector grip `combination:group`, rows edited in properties).
### en1997
- slopes, anchors, pileGroups, layerWaters were already gripped (audit said diagrams only).
- Gap closed: `upliftCases` grips `<id>:stabilising`, `:destabilising-g`, `:destabilising-q` (N, gain 100 kN/m) and `:pore-pressure` (Pa, 10 kPa/m); `labTests` grip `<id>:stress-increment` (Δσ). `Spec.handles(ctx.handles_of(..))` added in `🎬️scene/💧uplift` and `🧩️entities` (lab tests) so the entity advertises them; `SLAB_THICKNESS` is now `pub`. Edit rules and field-meta rows exist for all written leaves.
- Diagram added: `pore-pressure:<layerWater>` (hydrostatic u(z) below the piezometric level, level marker, table twin) in `🎬️scene/📈️diagrams`. 
- Not done, blocked: slope circle-slip diagram (needs the public `slices()`/governing-layer lookup of `⛰️slopes`, open request "vis-en1997 -> norm-completion" line 52), anchor-layout and pile-group diagrams (no existing quantity series beyond the 3D layout; low value), uplift balance diagram (weight/pressure areas of UPL need the check's governing situation, normative factors unverified).

## 3. Oracle adapters
Script check of every `check` in the oracle case files against adapters: en1990 all wired (`C.beta`, `form-linear`, `form-nonnormal`, `6.4.combinations`, `D.7`, `mechanics.beam-span-envelope`, `fatigue.miner`; `form-nonnormal-rf` is a nested reference label, not a check). en1997: the only unwired check was `en1997.8.anchor-elongation`. Done: ANC-04 gets optional inputs `e_t`, `a_t` and the informative outcome `elongation` (δ = A_d l_free/(E A_t), `📇️catalogue/⚓️walls-and-anchors`), adapter `anchor_elongation` in `ANY/🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring` (registry 19). Other gaps listed in `oracle-wiring-en1997.md` (phase relations, anchor-resistance, pile-group, oedometer) were already wired.

## 4. Choice-token casing scan (field-meta vs wire tokens)
Script over all artifacts (kebab/snake vs camelCase against quoted strings of the artifact). en1990 and en1997: no mismatch (en1990 tokens are snake_case strings matching the checks' vocabularies, en1997 camelCase matches the schema). Others reported in requests.md: en1995 `not-imposed`, `snow-high` vs wire `notImposed`, `snowHigh` (confirmed in schema json); en1996 `calciumSilicate`, `nonLoadBearing`, `ringTie` vs `calcium_silicate`, `non-load-bearing`, `ring-tie` in the corpus (verify); en1992 eight kebab tokens not found anywhere else (verify).

## 5. Open items in scope
### en1990 (20 listed)
Items 1-19 closed in `en1990-v9.md`; unchanged here. Open: item 20 (inferences dev-dependency cycle), blocked: needs the first nx result of the rebuild. Normative: all gamma/K_FI/A2 coefficients unverified (no printed EN 1990).
### en1997 (7 listed)
| item | state |
|---|---|
| positioned pile geometry, per-layer skin friction (264) | open: `piles[].positions[]` and per-layer skin friction are new snapshot leaves (model, twins, diff, io, fixtures); placements exist |
| groundhog N_gamma note (385) | informational, closed (check uses Vesic - 4 tan phi) |
| `include_depth` depth factor outside Annex D (579) | informational, open to the norm owner: the Annex D comparison calls with `include_depth: false` |
| `CaseRowDelta` lane (652) | closed in earlier waves (cases lane in snapshot/diff) |
| 8 oracle cases without entry point (723) | closed: all adapters present, last one `anchor-elongation` added now |
| taxonomy generator skips `change-structure-id` (1454) | open, blocked: generator-side (tool owner), descriptor identity mismatch |
| `piles[].concreteClass` enum leaf (1620) | open, blocked: string enum needs model, four twins, diff lane, SQLite, ~420 fixtures; `concrete_modulus_pa` is ready |
Unverified normative values introduced: none (the elongation formula is elastic mechanics; grip gains are UI scales).

## 6. Files
en1990: `ANY/🎬️scene/🕹️handles/🦀️.rs`. en1997: `ANY/🚪️io/📦️packages/🦀️rust/Cargo.toml`, `ANY/🧬️schema/💡️inferences/⚖️checks/{🧮️units,📇️catalogue/⚓️walls-and-anchors,🧪️tests/🎯️oracle-wiring}/🦀️.rs`, `ANY/🎬️scene/{🕹️handles,📈️diagrams,💧uplift,🧩️entities}/🦀️.rs`.

## 7. Likely first-compile fix-ups
en1997 handles: `v3(-axis.x, -axis.y, -axis.z)` (Vec3 field access/`v3` import from kit), `super::uplift::{SLAB_DEPTH, SLAB_THICKNESS}`, `super::layout::UPLIFT_SIDE`; `Outcome::info`/`QuantityKind` via the catalogue glob; en1990 `bridge_handles` uses `mul`, `along`, `exaggeration` already imported. Tests that count handles per entity or diagrams per document (goldens) may move.
