# W3-SHOWCASE: showcase DSL assets for en1991, en1993, en1994, en1998, en1999

Nothing compiled or run on the Rust side (no cargo, no `repo run`). `freeze.txt` was absent throughout. The earlier Haiku DSL-assets agent left nothing for these five (no showcase asset existed).

## Assets (new)
Layout and naming as the existing four (`🖼️assets/<emoji><slug>/<emoji><slug>/🗣️.dsl.semio`, magic `semio norm.enXXXX.dsl v1`):
| Code | Asset | Size |
|---|---|---|
| en1991 | `🌉️bridge-showcase` | 5.7 KB |
| en1993 | `🏗️steel-showcase` | 20.4 KB |
| en1994 | `🌉️composite-showcase` | 42.5 KB |
| en1998 | `🏢️seismic-showcase` | 40.1 KB |
| en1999 | `🏠️aluminium-showcase` | 10.1 KB |

Content is the committed showcase fixture of each code (`🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, all collections populated, schema-validated by the earlier builders): members, connections/joints, loads and actions, cases, plus the data behind the diagrams. No invented fields: every column comes from the schema JSON, every unit suffix from the Rust `#[dsl(unit)]`.

Generator: `🧰️scripts/showcase/dsl_assets.py [--check] [enXXXX]` (python3 -I, idempotent, `--check` reports "unchanged" for all five). It builds a loose draft from the fixture and normalises it with the asset-laws `patch.py` against the Rust model (field order, header tags, unit suffixes, enum spelling, nested records); top-level record `site` is printed flattened as in the committed en1998 assets, `annex`/`reliability-class` lower-case. Packs are not hand-written.

## Palette / catalogue entries
Editor `✏️editor/🦀️.rs` of each code: new `<slug>_example()` (`ExampleSource::new("<slug>", EN, DE, include_str!(asset), "file")`) chained into `examples()` next to `full-showcase`. Titles: "Bridge loads showcase / Brückenlasten-Gesamtbeispiel", "Steel frame and joints showcase / Stahlrahmen- und Anschluss-Gesamtbeispiel", "Composite bridge showcase / Verbundbrücken-Gesamtbeispiel", "Seismic showcase / Erdbeben-Gesamtbeispiel", "Aluminium showcase / Aluminium-Gesamtbeispiel". This also satisfies the asset-law `every_asset_is_classified_and_in_the_palette_with_both_titles` (palette id = asset id, text byte-equal), which also runs the round trip, evaluate, projection-law, spec-trace and `regen_dsl_and_pack_assets_match_the_codecs_or_bless` tests over the new assets automatically.

## Tests
Editor unit tests `✏️editor/🧪️tests/🔬️unit/🦀️.rs` of each code gained `the_<slug>_asset_decodes_to_the_showcase_fixture_and_evaluates_with_scene_and_diagrams`: palette offers the id with EN and DE title, `decode_enXXXX_dsl` of the asset equals `full_showcase()`, `NormFamily::evaluate` is non-empty, `viewport::scene_of` has entities, `scene::diagrams` is non-empty.

## Existing four: palette gap fixed
en1990 `road-bridge-showcase`, en1992 `rc-building-showcase`, en1996 `masonry-showcase`, en1997 `ground-showcase` had an asset but no palette entry under their id (the palette only had `full-showcase`/`full-catalogue`), so the asset-law test would have flagged each as "orphaned". Added the same `<slug>_example()` entry to those four editors (no new test there). Rebuild line appended to their builds too.

## Risks for the first build (read from source, unverified)
- Schema JSON is ahead of the Rust snapshot twins in five places (other agents' pending regeneration): en1991 `fatigueModels`, en1994 `studs`, en1993 `joints.beamWebFy`, en1998 `assessments.intermediateSteelAreaM2`, en1999 `connections.loadEccentricity`. The assets already contain these columns/collections in schema position (the fixtures need them too, `full_showcase()` uses the Reject policy); the asset decodes only once the Rust fields land. Where Rust and schema disagree on a unit, Rust wins (en1993 `frameActions.fx/fz`, en1991 FatigueModel loads written without unit; en1998 buildings `plan-*-m` with `m`).
- Float cells carry fixture reprs such as `0.031000000000000003`, like the existing assets; the regen bless (`SEMIO_UPDATE_GOLDEN=1`) rewrites the DSL canonically and generates the packs. Both the DSL and the pack need to be listed in `bless-allow.txt` (new files, not "empty"; see `post-build-plan.md` G2/G3).
- `assert_eq!(document, full_showcase())` fails if a unit suffix changes a value (e.g. mm vs m); then fix the generator, not the test.

## Files
Ticket: `🧰️scripts/showcase/dsl_assets.py`, this report, `🗑️generated/build-en{1990,1991,1992,1993,1994,1996,1997,1998,1999}/latest.md` ("- ready for rebuild: showcase").
Artifacts: five new DSL assets; nine `✏️editor/🦀️.rs`; five `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
