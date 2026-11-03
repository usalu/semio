# Wave 2 features agent v4: scan of every norm feature, rail/override case, member-modes rows

Nothing compiled, no cargo started. Python sides dry-run with `.venv/bin/python -I 🧰️scripts/wave2-features-dryrun.py <case>` (round-trip cases need `PYTHONPATH=…/🔮️oracles/🏃️execution` and no `-I`): every family case passes (catalogue 58/126/53/22/26/47/23, check-outcome 7/11/4/10/13/32/7/9/10/4, round-trip 3 each (en1999 2), frame-eigen 8/8, new rail case 15/15).

## Scan result (94 `🥒️.feature`)
- Only ONE feature had scenarios without any adapter: `🚄️rail-and-overrides-en1990-1` (also 11 parse errors: Examples without `id` cell, no `@id-<family>` tag). Rewritten and bound, see below.
- Every other feature with scenarios already has `🐍️.py` + `🦀️.rs`; the later EN 1990 v4 / EN 1991 v3-v4 / EN 1993 VIDS / EN 1994 E1-18 / EN 1995-1999 rows added to existing catalogue, check-outcome and round-trip cases dispatch by family and are covered by the shared layer (catalogue-fixture-case rows are fixture-driven; dry-run green).
- 35 platform features (compliance, structural-scene, app-surface, viewport, mechanics) and 11 artifact scene/design-case/schema features contain prose only (zero scenarios), see Blockers.
- Manifest: case registration is by convention (case dir with feature + adapters); only `subjectHostPackages` and capability/oracle tags are manifest data.

## Done
1. New case `en1990/…/🧪️tests/🚄️rail-and-overrides-en1990-1` (feature, `🐍️.py`, `🦀️.rs`): one `check-outcome` outline (15 rows: 4 factor-override verdicts `en1990.override.O1`, 2 E90.10, 2 E90.11, 3 E90.12 with `info` status) and one `clause-vector` outline (Fig. 6.10 lower/upper limit at 10 m and 40 m).
   - 11 hand-made documents `🧪️tests/🧫️fixtures/🚄️rail-and-overrides/<id>/🔣️.json` from the committed insert-factor-override before-snapshot; generator `🧰️scripts/wave2-features-v4-rail-fixtures.py`.
   - Python = independent restatement (annex override policy EN all but k_fi / DE psi only, X_d = eta X_k / gamma_m, a_d = a_nom +- delta a, alpha_cr >= 10, Fig. 6.10 power laws). Rust = io decode + production `evaluate` + `pick_outcome`; vectors from `semio_s_artifact_norm_en1990_checks::rail_dynamics::frequency_limits`.
   - Manifest: capability `en1990-1-evaluate` added to oracle `en1990-1-scipy`, feature tagged `@oracle-en1990-1-scipy` (the old tag `en1990-1-rail-overrides-independent` was unregistered). subjectHostPackages already hold checks, io, inferences, compliance.
   - The `deviates a term` assertion and the HSLM-A1 peak acceleration (6 percent Newmark vs scipy DOP853 spread, wider than every comparison profile, because the platform compares subject and oracle values by the profile) stay as prose in the feature description.
2. `en1993/…/🕸️frame-eigen`: 3 new `clause-vector` rows `member-pinned|cantilever|fixed` (mechanics `member_modes::buckling` Euler load vs pi^2 E I/(K L)^2, E I 16.8e6, L 5 m, tolerance 5e-3). Manifest of en1993 gained `semio-s-plugin-norm-mechanics` (host package). Dry-run 8/8.
3. Rebuild lines appended to `🗑️generated/build-en1990/latest.md`, `build-en1993/latest.md`, `integrate/latest.md`.

## Runnable vs total scenarios per artifact (platform adapters present; mutate counts are the mutation-engine feature, run by its existing adapter and not dry-run here)
| artifact | scenarios | runnable | of which mutate | prose-only features |
|---|---|---|---|---|
| en1990 | 357 | 357 | 274 | 1 (scene) |
| en1991 | 455 | 455 | 395 | 1 |
| en1992 | 342 | 342 | 296 | 1 |
| en1993 | 1259 | 1259 | 1093 | 1 |
| en1994 | 318 | 318 | ~236 | 1 |
| en1995 | 315 | 315 | 277 | 2 (scene, design-case) |
| en1996 | 290 | 290 | 257 | 1 |
| en1997 | 487 | 487 | 347 | 1 |
| en1998 | 514 | 514 | 507 | 2 (scene, schema) |
| en1999 | 423 | 423 | 411 | 1 |
| din18599 / din16798 / din4108 / vdi3805 / iso16757 | 43 / 91 / 109 / 41 / 67 | all | all | 0 |
| platform (compliance, scene, app-surface, viewport, mechanics) | 0 | 0 | | 35 |
Counts of en1990 and en1993 include the new 15 and 3 rows.

## Blockers (not done)
- `scene-law`: golden `🎬️scene/🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` of all 10 artifacts is still 0 bytes and no `scene_law_rows` exists; the 10 scene features stay prose.
- `oracle-comparison`: the registries are public (`oracle_wiring::registry()`), but `oracle_driver` (a test-dir file of compliance) has no `evaluate_case_file`; the subject cannot evaluate a `🔮️oracle-cases` file. Needs a pub fn over `load_cases` + `Registry` before an `oracle-comparison` case per artifact can exist.
- Catalogue coverage: EN 1991 registry 98 entries, only 38 have rows; EN 1996 40 entries, 16 have rows. A generator `🧰️scripts/wave2-features-v4-catalogue-rows.py` (registry input keys + catalogue-bridge tsv examples + independent oracle `ev`) yields only 1 row for EN 1991 because the tsv has no example values for 41 entries; not written. Owners need a per-entry example input table.
- Prose without a family: design-case (EN 1995, needs a document fixture with `cases[]`), NDP adoption/registry/check-stations (source-text laws), mechanics section/kernel (the Rust unit tests parse that feature file, so it must stay prose), app-surface/viewport/scene-kit (UI and runtime laws). No step family fits; a `registry-law` family would have to be added to `🧬️schema/🔣️.json` and both step layers.

## Unverified until a build
Rust adapters of the two touched cases (`Props`/`Ends`/`buckling` paths, `frequency_limits(AnnexChoice::En, …)`, override-row utilisation 0.0 from `informative`, E90.11/12 utilisation = limit/value), id equality of `en1990.e10/e11/e12.*` and `en1990.override.O1`, Cargo.lock edge frame-eigen host to mechanics, Examples `id` uniqueness across the merged check-outcome outline.
