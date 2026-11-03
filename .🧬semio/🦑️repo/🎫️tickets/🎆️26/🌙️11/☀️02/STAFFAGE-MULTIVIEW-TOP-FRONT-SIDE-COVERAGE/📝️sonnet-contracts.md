# 📝️ sonnet-contracts report

Owner: schema, contracts, depiction core, catalog validation. Sources: `📐️conventions.md` (derived conventions), `🔗️interfaces.md` (agreed interfaces), `📝️sonnet-contracts-backlog.md` (migration backlog per category), `🗑️generated/contracts/failures.json` (machine-readable).

## Changed files (all under L = `🎓️teaching/🏛️architecture/🧍️staffage`)

- Schema (JSON and TS twin in lockstep): `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`.
  - `SymbolRepresentation.geometry` requires elevation, front and plan.
  - `UnavailableRepresentation.cause`.
  - `Anchor.z`, `AppliedLoad` lateral fields, `distributed.wzNPerM`.
  - `Solid.revolved.axis`, with the D24 axis wording.
  - `Dimensions` envelope text.
  - Viewpoint, section and solid terminology.
- Contracts: `🔨️modules/🔗️contracts/🟦️.ts`.
  - `Vec3`, `BoneFrame`.
  - `EvaluatedBone.frame` and `tip`.
  - `WorldForce` 3D fields, `mzNm` rename and sign docstring.
  - `Fidelity`, `metadata.fidelity`, `metadata.omittedValues`.
  - `RepresentationResolution.unavailable.cause`.
- Depiction core:
  - `🎨️depiction/🔭️views/🟦️.ts` (new): cameras, `project`, `nearness`, `ViewMissingError`.
  - `🎨️depiction/🟦️.ts`: view-missing throw; the front dimension uses the front shapes; ground excluded from extents (`figureOf`); `patterns` defaults true; the omitted-values note out of the legend; captions per terminology.
  - `🎨️depiction/✂️section/🟦️.ts`: `VIEWPOINT_CAPTIONS` and `SECTION_CAPTIONS` as the single source.
  - `🎨️depiction/🧩️shapes/🟦️.ts`: unused `areaWeighted` removed.
  - `🎨️depiction/🖨️preview/🟦️.ts`: docstring wording.
- Resolver: `🔨️modules/🔭️multiview/🟦️.ts` (new); barrel `📦️packages/🟦️typescript/🟦️.ts`.
- Representation: `🖨️representation/🟦️.ts` (symbol suitability path, no `?? []` on views), `🪞️copies`, `🧭️pick` (`cause`).
- Evaluation: two small edits for D15 (`viewOf` throws `ViewMissingError`).
- Catalog validation: `🗂️catalog/🟦️.ts`, `🗂️catalog/🩺️check/🟦️.ts`.
  - `AssetContext.assets` is now a map from id to asset.
  - `Issue.data`.
  - `validateRanges` and `validateSymbols` exports.
- Fixtures: `🧫️fixtures/👤️sample-figure`, `🚶️reference-walker` and `🦶️sparse-smooth-walk` gained `cause` and a symbol `front` and `plan`.
- README conventions line and the solid line.

## Coverage completed

1. Conventions are written from the code, and the brief's handedness and side claims were corrected by D6 (code stays: left-handed, side elevation seen from the asset's right).
2. Schema gaps are closed. Validator rules in force are listed in `🔗️interfaces.md` sections 4 and 6; they cover per-view extents, symbol views, unavailable cause and drawability, asset alternatives, `valid-range-gap`, `solid-axis-not-ancestor`, `lateral-ambiguous` (D28), the manifest dimension rule, and the range and symbol print suitability.
3. Resolver: `resolveViews` and `resolveDrawing` project one evaluated state to the three viewpoints, or return a typed `DrawingUnavailable`. Substitution paths removed:
   - the `?? []` view tolerance in evaluation, `🖨️representation` and `🪞️copies`;
   - the front dimension that read the side elevation's extent;
   - ground-role shapes counted into extents;
   - a blank scene for an empty view, which is now `ViewMissingError`.
4. The interface document was written early, and the shared camera API is documented as stable.

## Verification evidence

- Typecheck: `bunx tsc --noEmit -p tsconfig.json` in `📦️packages/🟦️typescript`. My modules compile clean; the remaining errors were in other workers' in-flight files (sheet exports, a `d3-format` typing, tests of other workers).
- Backlog runs: `bun T/🛠️scripts/sonnet-contracts/failures.ts <out.json> [--ranges]` on the live catalog (149 assets, none crashed). The first run before the catalog migrations had 110 failing assets; the latest has 24 (see the backlog file for per-category rows).
- Probe `🛠️scripts/sonnet-contracts/extent-probe.ts`: tour-guide front width at 1:50 and 1:500 is exactly 0.500 m, so the extent measurement has no stroke bias (geometry only).
- Library tests: see the section below.

## Test state

Full library suite, `bunx vitest run -c 🧪️tests/🎚️config/🟦️.ts` in `📦️packages/🟦️typescript` (output `🗑️generated/contracts/vitest-full2.txt`): 15 of 76 files and 30 of 1718 tests failed (61 files and 1681 tests passed, 7 skipped). That run was taken while rigs, structural, exports and the category workers were still editing, so it is a snapshot, not a verdict.

Failures in my area, and what I did:
- `asset-validation` "accepts the sample figure", `smooth-continuity` sparse walk: the three fixture figures broke the new rules (`depthM` 0.46 against a drawn 0.54 to 0.56; fixed by setting `depthM` to 0.55 in the three fixtures). One finding remains: `unavailable-but-drawable` at 1:500 (the 1.75 m figure is 3.5 mm on paper, at least the D17 limit of 3 mm). The fixture needs a 1:500 band or a true-extent symbol; I did not author one. This keeps those two tests red until validation decides.
- `asset-validation` manifest case: the fixture expectation was renamed to `manifest-dimension-mismatch` (fixed).
- `print-suitability` "authors no front or plan geometry": measuring a representation without front or plan now throws by design (D15); the test now expects the throw (fixed).
- `section-views`: the caption expectations already follow the new vocabulary; the two remaining failures are catalog data (`three-seat-sofa` sections) and the cast manifest of the test (`viewpoint` required), not contracts.
- Other red files (`three-views`, `plan-projection`, `front-projection`, `continuous-rotation`, `pdf-reopen`, `zip-reopen`, `frame-timing`, `export-performance`, `cast-export`, `clip-sampling`, `multiview-coverage`) are catalog geometry, rig, export or other workers' areas; the first of them shows the migration still running.

Typecheck at the end: my modules compile; `🧮️evaluation` currently fails on the new `member` shape kind and revolved `axis` until rigs finishes `solid3`/`flattenGeometry`, and other files carry their workers' in-flight errors.

## Open issues

- The `--ranges` run is slow (about a minute per asset). Its result fills the last section of the backlog file.
- Symbol print minimum is implemented for `true-extent` symbols; paper-millimetre symbols are measured at their fixed paper size and are not validated against `minFeatureMm`.
- `lateral-ambiguous` uses θ_max over every pose, every clip and 9 clip samples, per D28; it does not sample root motion between the 9 samples.
- Consumers (viewer, exports, casts) must read `metadata.fidelity` and use `VIEWPOINT_CAPTIONS`; this is not enforced by the types.
- The validator keeps its own `?? []` in the checks that detect missing views, on purpose.
- Shared-file edits in `🧮️evaluation`, `🖨️representation` and the schema docs were done with other contributors editing the same files; every edit was re-read and applied by exact string.

## Triage of the full suite (`🗑️generated/contracts/vitest-full3.txt`)

Run on 2026-11-02: 17 of 77 files and 23 of 1716 tests failed (60 files passed, 1 test skipped). "Fixed" means fixed by contracts after that run; those tests were re-run (a partial re-run of seven files is the only evidence), not the whole suite.

| File | Test | Cause | Owner |
|---|---|---|---|
| `🧬️schema-conformance` | builds Shape from exactly the five geometries | `member` is the sixth Shape geometry | contracts: fixed (expects six) |
| `🧍️front-projection`, `🧍️plan-projection` | rest pose, stride, y extent (5 tests) | the tests patched a representation with only front or only plan; evaluation now throws `ViewMissingError` (D15) | contracts: fixed (the patch keeps the other views of the sample figure) |
| `🔄️continuous-rotation` | validates the spinning wheel, closing keys (3 tests) | `valid-range-gap` flagged ranges that tile exactly (74 then 75); a gap now means more than one denominator step | contracts: fixed. The earlier `valid-range-gap` backlog (32 issues, 11 assets) was partly this false positive; the workers adjusted ranges that did not need it. |
| `✅️asset-validation` | accepts the sample figure; manifest cases; the patched unavailable-decision cases (alternative-missing, alternative-unresolved, default-scale-unavailable, limiting-size, consistent manifest) | the sample-figure fixture is being rewritten (1:500 is now authored, so the cases that patch an unavailable 500 and the manifest row no longer match); `unavailable-but-drawable` at 1:500 for the 3.5 mm figure | humans (fixture owner), validation (cases) |
| `🌊️smooth-continuity` | sparse four-key walk | same fixture finding | humans |
| `🧍️front-projection` | refuses a representation with an emptied front or plan drawing | same `unavailable-but-drawable` finding on the sample figure | humans |
| `✂️section-views` | three-seat-sofa declares its sections | catalog data: the sofa declares no front section | furniture worker |
| `✂️section-views` | records the section of the viewpoint | cast manifest of the test has no `viewpoint` (required) | validation |
| `🧍️plan-projection` | sizes a plan cast to the plan | plan-commuter cast is 1.118 against a 0.15 limit | exports (cast exporter) / course |
| `🎭️cast-export` | draws the analytic views in the ink of the cast theme | stroke colour `#f7f3e3` still present in the analytic cast SVG | thermal / exports |
| `🎞️clip-sampling` | blends a transition and clamps outside its duration | `expected 5 to be 1` in the transition frame field | rigs (animation) |
| `🚀️export-performance` | equal the pinned digests of the reference export | `walker-line/0000.svg` changed: the title now reads "Dark line · Side elevation" (D40 captions and the fidelity and omitted-values metadata); the pinned digests need re-pinning | exports (pins), caused by contracts' caption change |
| `🧊️three-views` | seated-office-worker (`spread-elevation-x-plan-x`), skateboarder (`spread-elevation-y-front-y`) | catalog geometry of two assets: views disagree beyond tolerance | humans worker |
| `📑️multiview-sheet` | writes every viewpoint and the sheet to its own file name | timed out after 120 s | exports |
| `🎞️frame-timing`, `📐️paper-dimensions`, `📕️pdf-reopen`, `🗜️zip-reopen` | whole files | required tools missing in this environment: ffmpeg, rsvg-convert, qpdf (`repo tools setup all`) | environment |
| `🖊️dwg-reopen` | encodes through the wasm module | `staffage_dwg.wasm` not built | exports / build |
