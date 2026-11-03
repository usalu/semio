# EN 1998-2 bridges v2 (wave 2)

Nothing compiled or run (no cargo/rustc/`repo run`). Gates: spec-trace `--write` then verify (en1998 68 ok), `schema twins generate` then `check --strict` repo-wide (identical=7568, drift 0, style 0), schema-sync (mismatches=0), fixture checker `kind-convention-v4/verify_fixtures.py en1998` (307 scenarios, 299 inverted, 0 failures; the 6 `change-bridge-*` failures are fixed), handle-paths-check (0 uncovered), `verify build-owners sync` via heavy.sh (passed, 0 changes). The repo has no "Schema evolution tool" section in the brief; no schema fields were added in v2 (only label texts and fixture values changed).

## 1. Uplift check removed
The repo spec (`📐️specs/en1997-en1998-target.md` 3.2) has no hold-down or uplift rule for bridges (BR-02 only gives `avg/ag = 0.7`, BR-06 the `E_z` combination). The invented `F_v <= G` check was removed: `check_action` now emits only `action.importance`; test lines for `.action.vertical` replaced by an assertion that no such check exists; BR-02 trace is now `Data(part_2::bridge_vertical_spectrum)` and says the spec names no uplift rule. The vertical spectrum remains as a diagram and 3D indicator.

## 2. Cracked stiffness (BR-05)
The repo spec only has the garbled line `I_eff = 0.08 … I_g`; it does not give the M_y/φ_y (Annex C) method, and the pier row has no reinforcement data for the shared `section::fibre` kernel (`FibreSection::characteristics` would need As, f_c, f_y, N). So the effective stiffness is a required designer input without any default or sentinel: `bridge::cracked_ratio` is `clamp(0, 1)` (zero now leaves the piers without stiffness instead of silently using the gross section), new check `en1998.2.<b>.action.stiffness` (EN 1998-2 2.3.6.1) fails for a share outside (0, 1] with a remedy, and its explanation warns when the gross section (1.0) is declared. Labels (EN + DE) no longer say "1 = gross section". Trace BR-05 is now a check row. A computed basis is the open follow-up once pier reinforcement fields exist.

## 3. Fixtures
The three `change-bridge-{importance-class,vertical-period-s,pier-cracked-inertia-ratio}` mutation fixtures were byte copies of `pier-shear-rd-n` (wrong payload keys, stale after/diff). Regenerated from the correct `deck-ei-nm2` fixture with real values (3.0, 0.25 s, 0.5); demo-case mutations set to the same values (the cracked demo value 1.5 would now fail the check). Note: `🧰️scripts/en1998-part2-kinds.py` fixture clone branch matches wrong file names and only runs when the kind directory is missing; do not rerun it for fixtures.

## 4. 3D and grips (scene crate `🎬️scene`)
- Importance class badge above the deck start (`<b>:importance`, toned by `action.importance`) with grip `<b>:importance` on `importanceClass` (1..3, step 1).
- Vertical spectrum indication (`<b>:vertical`, T_v and S_ve) with grip `<b>:vertical-period` on `verticalPeriodS`.
- Per-pier q label `<p>:q` (q <= q_max with alpha_s, or "squat"), toned by `pier.<p>.behaviour`.
- Pier cracked-stiffness colouring: `ColourChannel::Fields` with station field `pier-cracked-share`, viridis legend 0..1, detail text with the share, and grip `<p>:cracked` on the bridge `pierCrackedInertiaRatio`.
- New `ground_and_bridges::deck_origin`; the simple (row-less) bridge now also carries its handles.
Unverified by compiling: `Legend`/`ScalarField`/`StationValue` come from the same `scene::*` prelude as `buildings`; `norm::ndp_common` is imported as `super::norm::ndp_common`.

## 5. Remaining items (all trace rows with `not-modelled` reasons quoting the spec line)
BR-10 (isolation: needs W_j per isolator, no hysteresis model, criteria marked `?`), BR-11 (empty row), BR-12 (`d_ground = ...` placeholder), ST-03 (formula `a_g / V_s` is not a strain, no limit, no pipeline entity), TW-01 (ranges without selection rule), TW-05 (optional, no cable law). Spec ids added to `specIds`; codegen regenerated the `.rs`.

## Unverified
Registry rows `ndp.en1998.en2.*` from v1 remain from the repo spec, not the printed standard. Compile errors are possible in the new scene code and the stiffness check (`Remedy::at_most` signature taken from the compliance crate).

- ready for rebuild: en1998 part 2 v2
