# W3-EN1994-STUDS: persisted per-stud collection (en1994)

Nothing compiled or run by me (no cargo; `twins generate` never got the heavy slot, my queued job was stopped because W3-EN1994b queued the same `--under en1994` run). Python oracles, spec-trace codegen and handle-paths ran.

## 1. `xM = spacingM*i` does NOT match the old station layout
The old scene put station `k` at `(k+0.5)*L/S`, `S = floor(L/spacing)`, `lanes = clamp(countPerRib,1,3)`; the checks counted `totalCount/2` per half regardless of position. The spec was changed (`schema-evolve/specs/en1994-studs.json`, tool got `get`/`round` in `l_convert.CALLS`): stations `S = ceil(N/lanes)`, row `i` in station `i // lanes`, `xM = round(origin + span/2 + (k-(S-1)/2)*spacing, 9)` in the beam frame (origin = neighbouring span for interior span/cantilever), so the standard 8 m / 40 / 0.2 m fixtures reproduce the old stations exactly. 295 objects became 11 847 rows (apply ran; the post pipeline died at `twins generate`, exit -15).
Post-step `🧰️scripts/en1994-studs/fit_pitch.py`: where `S*spacing > span` and the spacing is legal (<= s_max) the pitch is compressed to `span/S` so every declared stud stays inside the span (old checks counted all declared studs); 6 oracle documents. `beam-stud-spacing-fail` (80 @ 0.9 m) keeps its illegal spacing. Reproduced old committed values: prd 0.290898, etamin 0.715652, 1.26249 (fy fixture). Changed consequence: studs are persisted, so `change-beam-span-m` no longer moves them (feature row pass-01 0.3007 -> 0.3168 for span 10 with studs left at 0.1..7.9 m).

## 2. MANUAL list worked through
- Model: `HeadedStuds`, `CompositeBeam.studs`, `HeadedStuds::typical` removed (Rust + snapshot/artifact schema JSON + diff JSON `studs*` beam leaves); `Stud::typical`, `Stud::typical_row`; example doc carries 52 studs.
- SQLite: sub-table `en1994_beam_studs` removed (DDL, Rust codec, TS codec, admission: beam record indices >= 11 shift -1, actions 28 -> 27, 15 authored tables), tool bugs fixed (three-argument `add`, shadowed `stud_rows`, missing `stud_groups`, duplicate `studRows`/`Stud` import, `Exact<Placement,Stud,..>`, escaped quotes in the TS test). Tests: bridgeLanes carries the u32 admission cases, sqlite sample fixture got 2 stud rows and `bridgeLanes` 4294967295.
- Mutations: 6 old kinds removed (dirs, enum, kind list, JSON registry, protocol tags, edit rules, testkit, fixtures, oracle manifest, shared taxonomy, test lists); stud kinds wired to `mut-stud-rows` (workspace member + dependency, script.ts, mutations/io Cargo.toml, `use` lines). Old group crate `📌️studs` keeps `change-beam-transverse-as`.
- Diff `beam-rows`, editor field-meta, catalogue panel (`stud_spacing_limits_m(beam, d)`), incremental units (beam rows read `studs`), kit dependencies (stud ids), DSL assets (3 rewritten by `en1994-studs/dsl_assets.py`), showcase generator, spec-trace C-15/C-16/C-20 (codegen verify `[]`).

## 3. Checks (per stud)
`checks/🔩️studs` (`part_studs`): station grouping (1 mm = studs per rib n_r), `P_Rd,i = min(P1,P2)*k_t/k_l` per stud (`sheeting_reduction` with its station's n_r), half-span capacity (stud on the middle belongs to both halves with half its resistance), gaps, cells, per-stud force.
- `en1994.6.6.3.1.prd` / `.etamin` / mrd use Σ P_Rd of the weaker half; remedies are `Remedy::insert` of the next stud into the widest gap of the weaker half.
- New `en1994.6.6.2.perstud.<beam>` (C-16): P_Ed,i/P_Rd,i with the uniform plastic flow of ductile studs (else elastic |V|) over the cell of each station; sampled along x; remedy diameter (<= 25 mm) or insert.
- `6.6.5.5.spacing` now the worst gap between neighbouring stations (5d, min(6 h_c, 800 mm)) with remedy on that stud's `xM`; `6.6.1.1.layout` = studs inside the span (warning + move remedy); detailing per stud (`stud_detailing_geometry`, `stud_spacing_rule` new in the catalogue next to `stud_detailing`); fatigue/end-plate/material rows use the studs of the beam. Unverified clause readings unchanged.
Findings: per-stud util is stricter than the global check when studs are bunched (beam-bending-fail prd 2.60, perstud 12.49).

## 4. Scene, grips, palette
- Every stud is a Fastener `<beam>:stud-<studId>` (cap 2000) bound to the per-stud check; lanes spread by `stud_ys`.
- Grips per stud: position (`studs[id].xM`), diameter (10..25 mm), height; the old beam-level stud grips (η, spacing, rows, count) and the η diagram marker binding are gone.
- Shear-flow diagram: elastic flow vs the resistance staircase of the cells (`v-rd-staircase`).
- Palette: `stud` tool; `insert-item`/palette resolve through `insert_item::resolve_edit` (duplicate -> widest gap, fresh id `<beam>-stud-<n>`); remove via `remove-item` (`remove-stud`). Cascade on beam removal: rows carry `beamId` (W3-EN1994c).

## 5. Tests
Rust: `part_studs` unit tests (stations, uniform forces sum, middle stud, next gap, gaps), compliance tests (worked example P_Rd 81.656 kN, degree of the weaker half, k_t per station, spacing gaps), remedy-law insert loop, id-path reorder, scene tests (one fastener per row, twins share a station, per-stud grips, staircase), editor placement test. Oracle (re-derived, numpy sampling Voronoi): `stud_self_check`, check-outcome and compliance oracles rewritten without the old aggregate adapter; all 70+ feature rows re-evaluated against it (0 diffs), new rows perstud/spacing/layout; catalogue rows e1-04-prd-two-per-rib (57 159.393 N) and e1-04-stud-gap (1.1764706). Fixtures by tools (convert + fit_pitch).

## 6. For the runner / next owner
- [open] `schema twins generate --under en1994` (W3-EN1994b queued it), then `schema twins check --strict`, `evolve.py contract en1994 --apply` (DDL lost a table: schemaBytes/tableWidths/valueBytes, SQLite test literals), `verify build-owners sync`, rebuild.
- Rustc risks to read first: `connection()` in compliance/beams (`Box<dyn Fn + '_>`, `max_by` tuples), `part_studs::cells/forces` closure patterns, handles `connector_handles`, `insert_item::placed_stud` (`FromValue` inference), `BeamFacts` struct literal order.
- Unverified: `n_f_req` is 0 without studs; non-ductile per-stud flow is the elastic |V| profile normalised per half; stud diameter grip range 10..25 mm is a UI bound (25 mm from 6.6.5.7), not a norm lower limit.
