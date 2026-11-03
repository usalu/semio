# Showcase refresh (wave 2)

No cargo/builds. Python only (niced). The `repo` MCP server did not connect, so no `ticket_*` call.

## Showcase builders (`🧰️scripts/showcase/*.py`)
- `common.py`: new `conform()` maps strings to the schema enum spelling (case/kebab-insensitive) and runs on both the new and the existing fixture before the additive merge, so a stale committed value no longer wins; `finish` drops `reliabilityClass` when the schema has none (en1998 schema was regenerated at 04:24 without it).
- `en1993.py`, `en1999.py`, `en1998.py`, `en1994.py`: explicit `annex` and `reliabilityClass = "RC2"` (en1993/1994/1999; en1998 when the schema has it); en1999 `category: self` became `selfWeight`; en1994 adds `steel.rM`, `xM` on every beam action, `xM`/`yM` on slab actions (new required fields). All four collections stay populated, `--check` reports "unchanged" for all four.
- Fixtures validated with jsonschema 4.26 (venv python) against each schema JSON: en1993, en1994, en1998, en1999 showcase all OK. en1998 fixture regenerated (stale `B/F/E` category and `II` spellings replaced by the lower-case typed enum spelling now in the schema). en1993 fixture was only reformatted by the builder (semantically identical).
- en1994: freeze was still active at the start (03:24 entry); `freeze.txt` was gone when I came back, so en1994 builder and DSL fixes were done after it cleared (builder fixture needed no change; the three DSL assets did).

## Other example documents and DSL assets
- JSON assets: en1993 `heb240-compliant` and `high-strength-connection` `snapshot.json` had `annex` "de"/"en", now "De"/"En". en1999 JSON twins OK.
- 1043 en1993 JSON fixtures (512 mutation before + 512 after, 14 fixtures, 5 oracle files) had lower-case annex; fixed by token replace to `En`/`De`.
- DSL assets (33 of the ten codes) re-run with `🛠️asset-laws/patch.py` (Rust model order): en1997 x3 gained `reliability-class=rc2` (new field, trailing scalar line); en1990 x8 `traffic-groups`; en1991 x2 new scalars and collections (`bridge-curve-radius`, `derailment-cases`, ...); en1995 x4 `diaphragms trusses bracings decks`; en1999 x2 `transverse-stiffeners`; en1992, en1993, en1996, en1998, en1994: `reliability-class=rc2` moved from the header line to the position the struct order gives (after the last table, or after `fatigue-detail` in en1994); en1993 header cells refreshed. Dry run now: 0 errors. Remaining dry-run notes: nested block forms left as is (en1996 walls load-cases, en1998 storeys), en1999 `sections[1].kind: tube` is not a `SectionKind` variant (ticket for the owner, left unchanged).
- Packs (`🎒️.pack.semio`) are binary and stale for every asset touched; regenerate through `encode_en19xx_pack(decode_en19xx_dsl(dsl))` after the rebuild.

## Still stale, not touched (owners)
- en1998 mutation and check-outcome fixtures (about 595 files, errors: category `B/F/E`, `silos.contentCategory`, `site.enGroundType`/`enSpectrumType` missing in 8, `importanceClass II`) against the schema as of 04:24; the en1998 typed-enum spelling is in flux, so the generator should run once the schema settles. en1992 `change-bar-layer-lap-share` after.json has `lapShare` 2.0 over the maximum 1.
- The oracle JSONs and diff/mutation/outcome JSONs of en1990/en1993 are not snapshot-shaped (validator false positives).
- The en1993 editor unit test sets `annex` to `"de"` (lower case) via `SetField`, check against the typed enum after the rebuild.

## Rebuild
"- ready for rebuild: showcase refresh" appended to `build-en1990..en1999/latest.md`.

## Leftovers pass (SHOWCASE-LEFTOVERS)
- `common.py`: `conform()` removed, the `reliabilityClass` filter removed (en1998 builder no longer sets it; its schema has no such property), and the merged document (committed file plus builder rows) is now validated too, so a stale committed value fails instead of silently winning. No adapter remains.
- `en1998.py`: site literals `enGroundType "c"`, `importanceClass "iI"` and the storey, bridge, silo, tank and tower categories now use the schema spellings explicitly (`b`, `f`, `e`); annex stays `de` because the en1998 schema types annex as a free string.
- `en1993.py`, `en1994.py`, `en1999.py`: already emit `En`/`De` and `reliabilityClass` where the schema has it; unchanged.
- en1999 `sections[1].kind`: the showcase already carried `chs`; the source was the DSL asset `🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio` (`sec-chs168 tube` → `chs`, the schema's circular hollow variant `SectionKind::Chs`).
- en1993 editor unit test (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `render_right_after_an_edit_shows_updated_checks_without_evaluate`): `SetField annex "de"` → `"De"`.
- `--check` for en1993, en1994, en1998, en1999: all "unchanged", exit 0. jsonschema (venv, Draft 2020-12) of the four showcase fixtures against each schema: no errors.
- Not touched: en1998 mutation/check-outcome fixtures (requests.md line added: `[open] en1998: regenerate ~595 stale ...`).
- No cargo/rustc run, no git write. `freeze.txt` absent at start and during the pass; `heavy.sh` absent.
