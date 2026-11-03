# EN1990-SHOWCASE (wave 2)

No cargo/rustc/`repo run`, no git write. freeze.txt absent at every write. `- ready for rebuild: en1990 showcase` appended to `🗑️generated/build-en1990/latest.md`.

## Builder
`🧰️scripts/showcase/en1990.py` (`--write` default, `--check`): explicit `annex = "De"`, validated by `common.finish` against the snapshot schema (new `schema_rel` parameter of `finish`, the en1990 root schema only `$ref`s the snapshot enums) and by jsonschema Draft7 against `🧬️schema/📸️snapshot/🔣️.json`. `--check`: all four outputs unchanged, exit 0. `cases` and `factorOverrides` stay empty on purpose (nested `cases` DSL form has no verified example; an override would be an invented agreement).

## Document `road-bridge-showcase` (two-span continuous road bridge, 2 x 30 m)
- Permanents per span: `G-deck-s1/s2` (gSup), `G-surf-s1/s2` (gInf). Influence on a span: own action 1, adjacent span -1/3 (two-span elastic line at x = 0.375 L: loaded M = 0.0703 wL^2, other span loaded -0.0234 wL^2), so each span shows sup on its own and inf on the relieving adjacent action.
- Variables (psi/gamma from the registry via the annex, none typed in): `Q-ts` road_traffic, `Q-udl` road_udl, `Q-ped` road_pedestrian (group gr1a), `Q-sa` road_single_axle (gr1b), `Q-brake` road_horizontal (gr2), `Q-wind` wind_bridge, `Q-temp` temperature_bridge. `trafficGroups` rows TG-ts/udl/ped (GR1A), TG-sa (GR1B), TG-brake (GR2).
- Also: A-impact, E-1 (class III), bridgeSls sls-s1/s2 (deflection 0.03 vs L/800, frequency 4.2 Hz vs 3.0 min on the member), fatigue FS-1..4 (category 71 MPa, two bins per span), tes-1/tes-2 (strength normal, bearing stiffness lognormal), rel-1 FORM, placements abutment-A, pier-P, abutment-B (x = 0, 30, 60 m).
- Documented inputs (not normative): all gk/qk magnitudes, resistances, L/800, L/500 and L/300 ratios, 3 Hz, sample statistics, fatigue bins, delta-sigma_C 71 MPa (from the existing fatigue fixtures), beta 4.7.

## Outputs
- `⚖️en1990/…/🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, `🚦️check-outcome/road-bridge-showcase-pass` (same document) and `road-bridge-showcase-fail` (TG-sa moved to GR1A: group clash 1.8; resistances halved; deflection 0.072 vs 0.06; fatigue cycles 6e6; required strength 40; beta 3.0).
- DSL asset `🖼️assets/🏢️road-bridge-showcase/🏢️road-bridge-showcase/🗣️.dsl.semio` emitted by the builder with the collection headers of the (patched) `road-bridge-compliant` asset; kinds spelled `gsup/ginf`, groups `gr1a/gr1b/gr2`, importance `iii`.
- `🛠️asset-laws/patch.py --apply ⚖️en1990` run: the other 8 en1990 assets gained the typed `ENUM` headers (reliability-analyses, test-series, kind) from the current Rust model. Note: patch.py turns `gr1a` into `gr1-a` for the `LoadGroup` enum (explicit `#[dsl(key = "gr1a")]`); the builder spelling is the correct one, do not re-run patch.py over the showcase asset.
- Feature rows (`🧪️tests/🚦️check-outcome-en1990-1/🥒️.feature`, 44 rows `pass-rb-*`/`fail-rb-*`, tolerance 1e-3): pass on the pass document (STR/GEO 6.10, set C, 6.11b/6.12b, SLS 6.14b to 6.16b, EQU, A1.4 deflection/vibration, fatigue and damage, E10, Annex B/C/D, A2 deflection, `a2.traffic-groups` and `a2.traffic-group.TG-ts/udl/ped/sa/brake` = 1) and fail on the fail document (traffic-groups 1.8, str/geo 2.134, deflection 1.6/1.2, fatigue 5.91/1.82, annex-d 1.375, beta 1.208). Values come from the Python oracle `🔮️oracles/🚦️check-outcome/🐍️.py`; rows with utilisation within 0.97 to 1.03 (exact-limit design working life, material) were left out. The generator is `🧰️scripts/showcase/en1990_feature_rows.py` (idempotent).

## Gates
Builder `--check` unchanged; jsonschema (snapshot) ok for showcase, pass, fail; oracle runs on both fixtures. Feature dry run and fixture checker: see result lines below.

## Open
- Examples registry (`📚️examples/🦀️.rs` plus `🏢️road-bridge-showcase/🦀️.rs` and example test) not added: Rust, needs the next build (request for the en1990 owner, next pass).
- The Rust `evaluate` must produce the same ids for the new rows (existing traffic-group ids already exist in older rows; the other ids are the standard families); first build decides.
- Scene golden for a document with permanents per span (gamma-bands) is a new view, to be blessed.
