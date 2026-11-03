# Wave 2 features agent v2: oracle gaps, Rust subject adapters, round-trip family, prose features

All Python sides dry-run with `/home/deck/semio/.venv/bin/python -I 🧰️scripts/wave2-features-dryrun.py <case>` (PYTHONPATH on `🔮️oracles/🏃️execution` for the mutation engine); nothing compiled, no cargo started. Unit tests of the step layer: 7 green. Parser check of all 67 non-mutation norm features: 0 errors, 804 scenarios (`🧰️scripts/wave2-features-v2-parse-all.py`).

## 1. Oracle gaps closed (independent computations)
- en1996 `🔮️oracles/⚖️compliance/🐍️.py`: M-06 (E = K_E f_k, E_lt = E/(1+phi_inf) with an independent upper-value creep table), alpha1 = m1/(w_Rd l^2) from the yield-line work balance, N_Rd = Phi_s t f_k/gamma_M for M-81. Dry-run 19/19.
- en1991 `🔮️oracles/⚖️compliance/🐍️.py`: E11.01 (annex-aware density table, layers, total), E11.02 (EN and DE partition bands), E11.09, E12.03 (Table E.1 log interpolation, delta_n = 1; active measures raise instead of guessing). Feature fix: E13.02 rows wrote `zone=2` (numeric) for a text input, now `zone:2`/`zone:3`. Dry-run 48/48.

## 2. Rust subject adapters (written, unbuilt)
- en1993 `🧮️clauses` (17 quantities E01-E13), `🔭️special-parts` (92 clause quantities + 6 wired-check utilisations on the special-parts showcase), `🌐️storey` (new, see 5): `🧪️tests/<case>/🦀️.rs` on the shared step layer, calls into `semio_s_artifact_norm_en1993_checks::clauses::*` and `check_full_steel_structure`.
- en1994 `🧮️catalogue-en1994` (28 published quantities) on `semio_s_artifact_norm_en1994_checks::catalogue::*`; `vpl` is the EN 1993 6.2.6(3) shear area formula inline (the catalogue ISection has no fillet radius); unverified beyond the E1-06 hand value.
- Manifests: `subjectHostPackages` added to en1993 (checks, model), en1994 (checks).
- Risk points to read first in a build log: glob import ambiguity in special-parts (same module set as the old unit test), `Load` closure lifetimes in `mcr_fe_lambda`, `TStubModes` field names.

## 3. EN 1992
The v2 owner moved the oracle to `🔮️oracles/⚖️compliance/🐍️.py` and wrote the adapters (`🧪️tests/📇️en1992-catalogue`); 20/20 rows pass the dry-run. My S-03, B-03, A-15 handlers (rect. block brentq, alpha_1 anchorage, h/b limit) survive but appear twice in the file (requests.md).

## 4. Step families bound to the ten artifacts
- `mutation-round-trip`: grammar changed to four steps (before-snapshot, mutation payload, apply-then-inverse, equality) so both fixtures are declared inputs. New case `🧪️tests/↩️round-trip-enXXXX-1` for all ten (change, insert, remove vector each; en1999 has no remove): Python = shared mutation vocabulary engine (`steps.vocabulary_round_trip(KINDS)`), Rust = production `apply_*`/`inverse_*`. Capability `enXXXX-1-round-trip` added to each `-python-independent` oracle; `subjectHostPackages` (mutations, io) added to all ten. Generator `🧰️scripts/wave2-features-v2-roundtrip.py`. Dry-run 29/29.
- Shared Rust layer now dispatches all nine families (`scene-law`, `mutation-round-trip`, `oracle-comparison` added; `Binding.{scene,round_trip,comparison}`), Python twin `oracle_comparison` replaces the previous placeholder.
- NOT bound, with reasons: `check-outcome` needs, per artifact, a document-driven independent evaluator that reports per check id; the existing oracles either take no document (en1993 `⚖️compliance`) or return per-wall maxima over load cases (en1996 `snapshot`), so the id mapping needs per-artifact design plus fixture values only a Rust run can confirm. `scene-law` needs a typed scene probe and blessed golden counts (scene crates never compiled; golden bless pending in requests.md). `oracle-comparison` fixtures (`🔮️oracle-cases/*.json`) carry `check`/`inputs`/`expected` with no kernel evaluator behind them (the Rust side cannot evaluate them; running the harness on both sides would be a self-comparison).

## 5. Prose features
- Discovery rule (platform): a feature is a case only when its directory's parent is `🧪️tests`; 40 of the prose files sit directly in `🧪️tests` and are therefore not discovered today.
- Converted: en1993 `🌐️storey` to `clause-vector` (7 quantities, oracle `🔮️oracles/🌐️storey`, Rust `clauses::global`), FE-only assertions kept in the description.
- 46 files (all had parse errors) now parse clean: 6 discovered cases (compliance check-stations, ndp-adoption, ndp-registry, incremental-evaluation, design-case, en1995 design-case) hold their scenarios as "Further specification assertions" (no adapters exist); the others got `@id-`, `@level-quick`, `@mode-conformance` per scenario and `id` cells per Examples row (en1998 scene wording `Then,` fixed). Scripts `🧰️scripts/wave2-features-v2-normalise-prose.py` (input), backups of the originals in `🗑️generated/features/backup-v2/` with `index.txt`.
- Honest status: these are not executable yet. Promotion needs a `🧪️tests/<case>/` directory, an oracle (or no-oracle decision with survey) in the norm root manifest and adapters; requests.md lists it for the shared agent.

## Open items
1. `check-outcome`, `scene-law`, `oracle-comparison` bindings (above).
2. en1993 `🕸️frame-eigen` case (appeared during this round) has no adapters.
3. The three old-protocol Python scripts in compliance case dirs (`check-stations`, `ndp-adoption`, `ndp-registry` `🐍️.py`) have no `adapter()`; they run at import and are called by Rust tests, so they must move to `🔮️oracles` before those cases get scenarios.
4. Unverified: Rust adapters, manifest capability `enXXXX-1-round-trip`, `subjectHostPackages` path strings, `vpl`/`util` mappings.
