# 📝️ sonnet-validation: coverage matrix, reference cases, proof sheets

Owner: sonnet-validation. L = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder. Written in session 2 from the code and the session-1 outputs (the session-1 report was never filed). Everything about numbers is re-measured in the `## Session 2` section below.

## 1. Where things are

| Piece | Path |
|---|---|
| Matrix engine (cells, checks, summary) | `L/🧪️tests/🔭️multiview-coverage/🧰️.ts` |
| Matrix generator (CLI, sharded) | `L/🧪️tests/🔭️multiview-coverage/📊️matrix.ts`: `bun 📊️matrix.ts --out <dir> [--jobs N] [--assets a,b]` writes `matrix.json`, `summary.json`, `shard-*.json` |
| Per-bone and anchor checks (D41) | `L/🧪️tests/🔭️multiview-coverage/⚓️landmarks.ts` |
| Derived-view detection (D34) | `L/🧪️tests/🔭️multiview-coverage/🪞️derived.ts` |
| Cases (vitest), scenarios, third-party oracle | `🟦️.ts`, `🥒️.feature`, `🐍️.py` (svgelements, scipy, shapely, numpy) in the same folder |
| Vectors | `L/🧫️fixtures/🔭️multiview-coverage/🔣️.json` |
| Proof sheets | `L/🧪️tests/🔭️multiview-coverage/🖨️proofs.ts` (contact and print sheets), ticket-local PDF and PNG builders in `T/🛠️scripts/validation/` |
| Summary and per-owner lists | `T/🛠️scripts/validation/summary.ts <matrixDir> <out.md>`, `owners.ts` |
| Runs | `T/🗑️generated/validation/<run>/` (`matrix.json`, `summary.json`, `summary.md`, `owners.md`) |

Run: `cd L/🧪️tests/🔭️multiview-coverage && bun 📊️matrix.ts --out T/🗑️generated/validation/<run> --jobs 12`, then `bun T/🛠️scripts/validation/summary.ts <run> <run>/summary.md`. A full run is about 30 minutes with 12 to 15 shards.

## 2. Cell definition

A cell is `asset × scale entry × viewpoint × mode`, and it carries the subjects checked inside it.

- **asset**: all 149 catalog assets, read without validating them (`loadCatalog`).
- **scale entry** (`scaleEntries`), enumerated from the declarations alone:
  - `preset`: 500, 200, 100, 50 (every asset must decide each);
  - `declared`: any further key in `representations`;
  - `band-min` and `band-max`: the two ends of every authored representation's `validRange` (D10), up to 1:1000, requested at that denominator;
  - `alternative`: one entry per alternative of an unavailable scale (`alt<i>:scale:<d>`, `alt<i>:symbol:<id>`, `alt<i>:asset:<id>`).
- **viewpoint**: `elevation` (side), `front`, `plan` (top).
- **mode**: `dark-line`, `dark-solid`, `light-line`, `light-solid`, `thermal`, `forces`, `moments`.
- **subjects inside a cell** (`subjects`): every pose; every clip at start, every keyframe, the middle of every key interval, the extremes of the rig bounds and view extents (48 probes), 8 uniform instants (16 for clips longer than 600 s) and the end; transitions of every pose to the rest pose plus three seeded random pairs (mulberry32 on the FNV-1a hash of the asset id) at fractions 0, 0.25, 0.5, 0.75, 1 of one second.
- **languages**: English everywhere; German additionally at poses and at start, key, extreme and end instants.
- The universe is `universe(assets)`, a list of keys. A key no shard produced becomes `cell-missing` (`assemble`). A cell where nothing was checked is `cell-unchecked`. Nothing is skipped silently.

One call to `resolveViews` per (subject, mode, language) evaluates once; the three viewpoint scenes of the cell group come from that one state (`state-identity` check), so asset, pose, scale, mode and time are shared by the three views.

## 3. Status of a cell

| Status | Meaning |
|---|---|
| `ok` | rendered and all checks pass |
| `unavailable` | the asset declares the scale unavailable with reason, cause, limiting feature and size, alternatives, and the resolver answers `scale-unavailable` in all three views |
| `range-end` (D30) | a band-min or band-max cell whose only finding is a print-profile violation (`unavailable-scale-unsuitable`); own column, not FAIL. At a nominal scale (preset or declared key) the same finding is FAIL |
| `group-drift` (D41) | only whole-group extent differences between views for bones with several shapes; own column, not FAIL |
| `fail` | any other named check |

## 4. Failure codes

Declarations: `representation-missing`, `unavailable-reason`, `unavailable-limiting-feature`, `unavailable-limiting-size`, `unavailable-alternatives`, `unavailable-cause`, `unavailable-resolver`, `unavailable-resolution`, `alternative-scale`, `alternative-symbol`, `alternative-asset`, `alternative-asset-scale`.

Resolver: `resolve-error` (query or depiction threw), `unavailable-<code>` where code is `scale-unavailable | scale-unsuitable | view-missing | symbol-unknown | symbol-scale` (a drawable cell answered unavailable), `state-identity`.

Scene: `scene-viewpoint`, `scene-mode`, `scene-time`, `scene-query`, `scene-scale` (silent substitution of view, mode, time, query or scale), `scene-size`, `scene-bounds`, `blank`, `blank-shapes` (the scene draws no path of the view's shapes; message gives role, pen and reason), `<view>-empty`, `<view>-bounds`.

Dimensions at the default pose (D18 tolerance max(2 %, 15 mm)): `dimension-elevation-width|height`, `dimension-front-width|height`, `dimension-plan-length|width`, and on paper `paper-width|height` (scene bounding box against `m * 1000 / denominator`).

Cross-view synchrony: whole-extent `sync-forward` (elevation x and plan x), `sync-height` (elevation y and front y), `sync-lateral` (front x and plan y); tier one (D41) `sync-bone-forward|height|lateral` for bones with a single shape per view and `anchor-landmark` (contact, attachment, emitter anchor inside the bone's drawn shapes); tier two reported as group-drift `group-drift-*`.

Representation hygiene: `derived-view-band-copy` (a view reused across two bands, D34, includes D33 `representation-copied`), `derived-view-in-band` (a view that is a copy, mirror, 90 degree rotation or scaled copy of another view in the same band, allowlisted only for genuinely symmetric bodies in the vectors), `derived-view` (detector crashed), `language-geometry` (German drawing differs in shape geometry from English).

Analysis: `analysis-cue` (forces and moments scene without hatch or dash), `analysis-cue-legend` (thermal legend without the 35 % unhatched statement).

Symbols: `symbol-<view>-missing`, `symbol-conventional-note`. Time: `loop-closure`.

Print profile: `unavailable-scale-unsuitable` (the resolver refuses a band as unsuitable for the print profile; `range-end` at band ends, FAIL at nominal scales and alternatives).

Engine: `cell-missing`, `cell-unchecked`.

## 5. Exhaustive versus sampled

Exhaustive: representation decisions of every preset and declared key; both validRange ends of every authored band; unavailable declarations including the resolver answer in the three views; geometry of every pose in the three viewpoints; cross-view synchrony of every evaluated instant; default-pose dimensions; loop closure; all seven modes of every pose at every scale entry.

Sampled: clip instants (rule above), transitions (pairs and fractions above), German text at selected instants only, band-end cells (poses and start and end instants of clips only), alternatives (scale alternatives reuse the target cells, symbols render every pose, asset alternatives only check that the target is authored at that scale).

## 6. Reference cases and where they live

`L/🧪️tests/🔭️multiview-coverage/🟦️.ts` (vitest) with scenarios in `🥒️.feature` and vectors in the fixture: known dimensions on paper at 1:50 and 1:200 in all views; real catalog dimensions on paper; asymmetric held objects by hand side; planted contacts and root motion; loop continuation with absolute time; moving thermal emitters; one magnitude per region across views; out-of-plane force and moment; minimum printable gap (shapely oracle); pens never below the profile; repeated view switching and arbitrary scrubbing; deterministic SVG, PDF and DXF bytes; CLI file naming; svgelements bounds; derived-view detection (scipy). Session 2 adds a fixture-driven regression test over the matrix (see Session 2) and a folder per reference case.

## Session 2

Appended below as the work proceeds.
