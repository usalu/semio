# DIN EN 16798 content tests, 2026-11-24

## Causes and fixes

1. and 2. `unoccupiedVentilation` read-only and without English label. The zone's optional evidence record had no setter at all. The zone diff patch could not express it, so no mutation kind existed and the editor rules did not cover its path. The field meta had no entry either.
   - Diff: `Din16798ZonePatch` is now written with a local `optional_record_patch!` macro that carries `unoccupiedVentilation` plus an explicit `clearUnoccupiedVentilation` flag. A nested `Option` cannot tell unchanged from cleared on the wire. The same pattern is used for en1992 tightness. All four schema facets (json, ts, proto, graphql) are updated, and the 20 zone-patch diff fixtures gain the two null keys.
   - New mutation kinds `change-zone-unoccupied-ventilation` (tag 50) and `clear-zone-unoccupied-ventilation` (tag 51). Wired through zones-more, the enum and KINDS, text and binary codecs, the protocol, the ts/json/graphql aggregates, the oracle catalogue, the fixtures and the unit tests. The edit rules point `zones[].unoccupiedVentilation` at them, so nested leaves are edited through the whole-record replacement.
   - Field meta: bilingual entries for the record and for operation, assessed area, zone volume, purge minutes, residential rate and outdoor air. New unit `L/(s·m²)` (`LitrePerSecondSquareMetre`), which the snapshot schema already declared.
3. `usageType` changed no check. In the DE annex the encoded tables for office and classroom are identical. The engine now also reports the usage category's typical occupancy: `din16798-1.occupancy.<zone>` is an Info row whose value is area divided by the typical m² per person (office 10, meeting and classroom 2, retail 7), or NotApplicable where the standard gives none. It is registered in spec-trace VE-02.
   - Independent check: `🔣️occupancy.json` is verified by a sqlite derivation in `🐍️.py --occupancy-fixture` and by a Rust test.
   - Caveat: the density values come from my recollection of the EN 16798-1 Annex B and CR 1752 typical occupancy. See follow-up 1: they are now unverified registry rows.
   - Test plumbing: the perturbation helper builds a record from the null leaf and has an alternative token for `operation`. Applicability gating for the purge-only and rate-only leaves follows the existing `clothingClo` precedent. No assertion was weakened.
4. Hygienic check on European dilution. The engine is consistent with the repo's other derivations: CO₂ generation of 20 l/h (800 ppm gives 25 m³/h in the reference tsv, the scene oracle and the cross-check oracle) and Table B.10 rates. The hygienic oracle and fixture were wrong. They used 18000 for dilution and an invented `9n + 1.44A` for method 3, so the 144 m³/h floor wrongly dominated.
   - Corrected: dilution is now 10 × 20000 / 1350 = 148.148. Method 3 for category IV is max(10 × 5.5, 10 × 0.55) × 3.6 = 198. The sqlite oracle now has the full category and annex tables.
   - A masked case then surfaced: "Residential scope stays separate" in the German annex. The engine deliberately reports `GermanResidentialBasis` and UnverifiedBasis there (NA.5.2, DIN 1946-6), so the 100.8 expectation contradicted the encoded norm. The case now uses the EN annex, category II and Table B.11: (10 × 2.5 + 10 × 0.15) × 3.6 = 95.4.
   - The other agent's `assert_serde_snapshot_law` / `validate_json_schema` call and its non-lowercased annex value are untouched.

## Results

- Root crate (`-p semio-s-artifact-norm-din16798`, 52 tests): `test result: ok. 52 passed; 0 failed`. Before the fix: 47 passed, 4 failed.
- Mutations crate with `--features test-support`: `test result: ok. 10 passed; 0 failed` and `committed_vectors_hold ... ok`.
  - Without the feature the crate's lib test does not compile (`ChangeAnnex: serde::Serialize`). That predates this work and is unrelated to it.
- Norm JS native suite (`run-norm-js-native.sh`): `12058 pass`, `0 fail`.
  - The coordinator's failing test counted two more din16798 bindings: my two edit rules, both with real payload schemas. They are correct, so `🧫️cases/🔣️.json` now says `"din16798": 50`. The fixture holds only family counts, so there is no per-binding list.
- Examples family run (`test-family-examples.sh`): `[DEBUG] din16798 exit 0: 3 passed, 0 failed` (script's own output line, not a debug log of mine).

## Changed files

All paths are relative to `✏️s/🔌️plugins/📕️norm/`.

- `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🦀️.rs`
- `🧪️tests/🧭️payload-bindings/🧫️cases/🔣️.json`
- Under `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/`:
  - `🧬️schema/🔺️diff/` (rs, json, ts, proto, graphql)
  - `🧬️schema/🧬️mutations/`: new `🌛️change-zone-unoccupied-ventilation/` and `🌜️clear-zone-unoccupied-ventilation/`; edited `🦀️.rs`, `🟦️.ts`, `🔣️.json`, `🔗️.graphql`, `🗺️zones/`, `🗺️zones-more/`, `🧭️edit-rules/`, `🧪️tests/🔬️unit/`
  - `🚪️io/📝️text/🧬️mutations/🦀️.rs`
  - `🚪️io/💾️binary/🧬️mutations/` (`🦀️.rs`, `📡️.protocol.semio`)
  - `✏️editor/🏷️field-meta/🦀️.rs`
  - `🧬️schema/💡️inferences/⚖️checks/` (`🏠️zone-evaluation/`, `📜️annex-parameters-and-parts/`)
  - `🧭️spec-trace/` (json, rs)
  - `🔮️oracles/🔣️.json`
  - `🧪️tests/🧫️fixtures/🧬️mutations/`: 20 edited `.diff.json`, plus new directories for the two kinds
  - `🧬️schema/🧪️tests/⚖️compliance/`: `🦀️.rs`, `🐍️.py`, `🔣️hygienic-airflow.json`, new `🔣️occupancy.json`


## Follow-ups (coordinator, same day)

1. Occupancy densities as unverified registry rows. NEEDS A HUMAN DOUBLE-KEY against EN 16798-1 Annex B.
   - Four rows `ndp.din16798-1.occupancy.{office,meeting,classroom,retail}` (10, 2, 2 and 7 m²/person) are in the NDP registry (`📜️annex-and-tables/🔖️ndp/🔣️.json`) with status `unverified`.
   - A new part crate `🗃️table/din16798` joins the table ahead of din4108, so the table stays id-sorted. The engine reads the density through `ndp_number_opt` and cites the id on the check.
   - The check stays `Info`. `UnverifiedBasis` would set `complies = false`, and an informative row must not block a document. It cites the unverified id, so `unverified_basis()` is true, and its explanation states "Unverified basis ... awaits a double-key" in English and German.
   - Test: `usage_category_sets_the_typical_occupancy_...` asserts the Info status, the cited unverified id and the explanation text. It passes in the root crate run: `test result: ok. 52 passed; 0 failed`.
2. Python mutation oracle and BDD feature table: `change-zone-id`, `change-vent-id` (also missing), `change-zone-unoccupied-ventilation` and `clear-zone-unoccupied-ventilation` are added to KINDS, VECTORS and both Examples tables (the text now says 52 kinds).
   - Two taxonomy rows for the new kinds are in `🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`.
   - Shared engine fix in `🔮️oracles/🏃️execution/🔧️verbs/🐍️.py`: the inverse of a rename of the addressed record's own id now addresses the new id. Without it the zone-id and vent-id inverses were refused.
   - Verification: I drove the adapter's handlers through the host's `Adapter`/`Context` classes with the committed fixtures. `semio_repo_test` does not exist outside the host, so I used a shim. Result for din16798: `113 handlers, 0 failures`. That covers mutate and inverse for all 52 kinds, the extra dupe/gone/clamp rows and identity-round-trip.
   - For the other families I compared failure counts with and without the engine change and they are identical, so there is no regression. The driver's own asset-path assumptions explain the non-zero counts in vdi3805, iso16757, en1993, en1994, en1996 and din18599. I could not run the repo's BDD runner or pytest (pytest is not installed).
3. Mutations crates now compile their lib tests bare. A self dev-dependency `<crate> = { path = ".", features = ["test-support"] }` is added to the `-mutations` crates of din18599, en1991, en1992, en1994, en1996, en1997, en1998, en1999, din4108, vdi3805, iso16757 and din16798. en1990 already had its own pattern. en1993 and en1995 are untouched.
   - Bare `-p semio-s-artifact-norm-din16798-mutations`: `test result: ok. 10 passed; 0 failed`, and `committed_vectors_hold ... ok`.
   - Bare `-p semio-s-artifact-norm-en1992-mutations`: the lib test now compiles bare and `test result: ok. 16 passed; 0 failed`. The integration test `committed_vectors_hold` fails: `committed vector file ...⭕️change-bar-layer-lap-share/⛔range/🔣️.before.json is readable: No such file or directory`. That fixture is missing from en1992 and has nothing to do with this work. I did not touch it.
   - The other families' bare builds are not run.

After these follow-ups: root crate `test result: ok. 52 passed; 0 failed`, Norm JS native `12058 pass`, `0 fail`.
