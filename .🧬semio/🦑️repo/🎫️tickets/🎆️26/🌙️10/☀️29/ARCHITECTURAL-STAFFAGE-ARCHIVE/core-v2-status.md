# Core v2 status (core worker, session 2)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder. State: M1 engine landed; the check runs against the real thermal/transport modules (no `--no-physics` needed); official per-asset results in [check-v2-results.md](check-v2-results.md).

## Commands (run from S; bun only through `repo tools exec bun -- …`, a run may queue for minutes behind the shared capacity floor)

- Schema JSON regenerated from the TS twin: `repo tools exec bun -- T/🧰️scripts/core/generate-schema-v2.ts 🧬️schema/🔣️.json` (done 2026-10-29; includes `passengers`, `subject`, `hidden`).
- Check one asset: `repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts [--asset id] [--category id] [--file asset.json] [--samples 9] [--profile id] [--no-physics] [--quiet]`.
- Check every asset in one process (table): `repo tools exec bun -- T/🧰️scripts/core/check-all.ts --out T/check-v2-results.md --json T/🗑️generated/core/check-v2.json --samples 4 [--only a,b]`.
- Preview: `repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset id [--catalog root|--file asset.json] --scale 100 [--pose id|--clip id --time s|--symbol id] [--view v] [--sheet] [--print-size] [--profile id] --out dir`.
- Suitability probe (violations with positions): `T/🧰️scripts/core/suit-probe.ts --asset id | --file f --limit n`.
- Proof sheet (A3 landscape, SVG + PNG + PDF, true paper size): `T/🧰️scripts/core/proof-sheet.ts --assets a,b|all --out T/🗑️generated/core/proof [--name n] [--language en|de]`.
- Coverage matrix: `T/🧰️scripts/core/coverage-matrix.ts --md T/coverage-matrix.md --csv T/🗑️generated/core/coverage-matrix.csv [--only a,b] [--no-transport]`.
- Library tests: `cd S/📦️packages/🟦️typescript && repo tools exec bun -- x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts`.

## API

- `evaluate(asset, query, selector?)`: no selector ⇒ no geometry (`representation.kind === "none"`); physics identical for every selector (test `♾️physical-invariance`). `state.bounds` = extent of the drawn elevation geometry, `rigBounds(state)` the rig extent.
- `🖨️representation`: `resolveRepresentation`, `resolveSymbol`, `pickRepresentation`, `measureSuitability`, `compareRepresentations` (copy detection), `resolutionLabel/resolutionText` (bilingual UI labels: exact, derived naming source drawing + valid range, unavailable with supported scales, unsuitable, symbol), `RepresentationError`.
- `🖨️representation/🪞️copies`: `compareRepresentations(coarse, fine) → identical | subset | scaled | distinct` on flattened local geometry (0.1 mm); `validateAsset` raises `representation-copied` for any non-distinct pair.
- New validation codes: `representation-copied`, `valid-range-overlap` (strict overlap; a shared endpoint is allowed), `pen-units` (pens/hatch/penMm outside 0.03…2 mm, hatch spacing > 20 mm), `limiting-size`, `required-missing` (e.g. `thermal.passengers` absent; the validator no longer crashes on it).
- `depict(state, options, analysis)`: `Analysis.transport?: TransportField`; with `options.transport.length > 0` the transport overlay (transport worker's `transportView`) is drawn on any view, legends appended; `depictAll` solves transport once.
- `📐️units`: `viewportMm(widthMm, heightMm, zoom)` (zoom is a pure magnification; nothing in evaluation/representation/depiction takes a zoom).
- Barrel `📦️packages/🟦️typescript/🟦️.ts` additionally exports `compareRepresentations`, `resolutionLabel`, `resolutionText`, `viewportMm` (additive; coordinator please keep).
- `CheckReport` now also carries `issues` (structured) and `scales` (decision + suitability per preset).

## Test migration (v1 → v2)

Removed `🔭️lod`; `📏️print-size` lost its LOD/pen-table scenarios; `🖊️line-hierarchy` and `📐️paper-dimensions` run on `rolesAsset()` (representation pens); the v1 `🚶️reference-walker` is replaced by `👤️sample-figure` (v2; scales redrawn at 200/100/50 so they are not copies) in clip-sampling, foot-plant, evaluation-state, asset-validation (63 cases, v2 codes added), schema-conformance (v2 invalid cases), smooth-continuity (`🦶️sparse-smooth-walk` is now a v2 document), continuous-rotation (wheel is a v2 asset with four redrawn scales). New cases: `🧭️representation-resolution`, `🪞️representation-copied`, `♾️physical-invariance`, `🎨️seven-modes`.

## Requested contract changes / open issues

- `🏗️structure` `structuralLegend` lacks `Legend.range` (structure owner); structure harness lacks ThermalInputs `externalWorkW`/`breathing`.
- Website must use `evaluate(asset, query, options.representation)` and drop `lodFor`/`lineWeightMm`.
- The export CLI/tests (`🎞️frame-timing`, `🗜️zip-reopen`) draw `walking-commuter` at 1:100 under the default profile, which the measured suitability rejects (see check-v2-results.md): content or the export tests must choose a suitable scale.
- Thermal/transport tests with failures not mine: see core-v2-report.md.

## Progress log (18:05)

- Done: schema JSON regenerated; validation (copied scales, range overlap strict, pen units, limiting size, required-missing incl. `presentation.defaultScale`); gap measurement per coordinator ruling (visible outlines only, ink gap > 0 and < min gap; touching/overlapping ink = join; negative-space slivers via inked-silhouette raster); resolution/derived/unavailable/unsuitable + bilingual labels; transport hook in `depict`; preview `--transport`; tests migrated (library vitest: all core cases pass; remaining failures belong to thermal/transport/export owners: thermal-legend float equality, thermal-scenarios, thermal-transient, transport-field timeout, frame-timing + zip-reopen use `walking-commuter` at 1:100 which office-laser rejects).
- Running/pending (queued behind the capacity floor): final `check-all` refresh (`check-v2-results.md`, then `summarize-check.py` appends summary + sample slivers), proof sheets (`🗑️generated/core/proof/*.svg|png`, `proof-sheet.pdf`), coverage matrix (`coverage-matrix.md` already written once: 10 184 rows, 25 failing rows; rerun at the end).

## Gap measurement (coordinator ruling applied, 18:30)

A gap violation is only `0 < ink gap < min gap` between the VISIBLE outlines (hidden lines removed) of two different shapes (ink gap = centreline distance − half of each pen), or a negative-space sliver inside the inked silhouette (fills + pen-wide strokes) thinner than min gap with area > 3·w² (the V between legs may taper to a point, a channel may not). Touching, crossing and ink-overlapping outlines are joins. Measured over all named poses. The remaining small positive values (0.003…0.14 mm) are real near-misses; samples with paper-mm positions (origin = model origin, x right, y down) per asset are in the "Sample violations" section of `check-v2-results.md`, e.g. jogger 1:200 `torso+upper-arm-r 0.003 @0.1,-7.3`, commuter 1:100 `cuff-l+hand-l 0.001`, 1:50 `thumb-l+forearm-l 0.009 @3.7,-9.3`; `negative-space 0.025` means about one raster pixel wide (pitch 0.025 mm).

Newest core items: `ThermalInputs.passengers` (animation blend), `solveThermalAt` in depict/preview/check, `legendBlock` conditions/assumptions lines.
