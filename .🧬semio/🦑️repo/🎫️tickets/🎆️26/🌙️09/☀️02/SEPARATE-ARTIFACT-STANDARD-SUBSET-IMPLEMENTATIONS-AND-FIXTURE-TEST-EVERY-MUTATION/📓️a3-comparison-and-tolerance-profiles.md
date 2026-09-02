# 🅰️3️⃣ Comparison and tolerance profiles — fem/2d, fem/3d, json, sequence, pdf

Shard A3 territory: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d`, `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d`,
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json`, `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence`,
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf`.

## Before / after (measured against `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`, scoped to the five paths above)

| breach id | before | after |
| --- | --- | --- |
| `fixture-comparison-profile-unknown` | 45 (fem/3d 15, fem/2d 12, pdf 9, json 5, sequence 4) | **0** |
| `fixture-tolerance-profile-unknown` | 36 (fem/3d 15, fem/2d 12, json 5, sequence 4) | **0** |
| `missing-fixture` | 3 (1 each: fem/2d, fem/3d, sequence) | **0** |
| **total** | **84** | **0** |

Regression check (same scope): `tolerance-record-invalid` and `pipeline-record-invalid` are **0** both
before and after. Every OTHER breach id still present under these five paths after my edits
(`unsplit-artifact-subset` 58, `runtime-inventory-missing` 14, `oracle-capability-mismatch` 9,
`binary-protocol-drift` 5, `missing-external-oracle` 4, `stub-serializer` 4,
`unknown-mutation-catalog` 2, `oracle-in-production` 2, `capability-without-manifest` 1) matches the
baseline dumps in `🗑️generated/breach-<id>.json` exactly — none of them belong to this shard and none
of them moved.

The gate (`bun ./📜️script.ts test contract`) was run in the foreground before starting (baseline dumps
already captured in `🗑️generated/`) and again after all edits; the second run wrote a fresh
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` (1865 breaches repo-wide, most of them other shards'
open work) which is what the table above is measured against.

## Decisions, one per unknown name

For every unknown profile I first checked whether an existing profile (CORE or an already-registered
subset profile) already carried the same meaning, per the brief's instruction to repoint rather than
duplicate. Three of five surfaces resolved that way; two (the fem mesh geometry) were genuinely
missing and got a real, numerically-justified contribution.

### fem/◻2d and fem/🧊️3d — contributed (genuinely missing)

`semantic-fem-mesh-manifold-v1` (comparison) and `fem-polygon-exact` (tolerance) were referenced by 27
`replace-region-geometry`/`replace-solid-geometry` mesh fixtures (12 in ◻2d, 15 in 🧊️3d) and did not
exist anywhere. Added identically-reasoned entries to **both** subsets' own `🧪️oracle/🔣️.json` (each
subset owns its own registry — law 1), because the two corpora share the same construction (a
straight-edged polygon linearly extruded, exported through `three`/`manifold-3d`, welded back in) and
the same measured noise floor.

**`semantic-fem-mesh-manifold-v1`** — a structural comparison of the `expected-obj` +
`expected-stl` + `expected-measurements` bundle, restricted to the fields that are genuinely
TRIANGULATION-INVARIANT: `empty`, `solids` (connected-component count), `genus` /
`componentGenus` / `expectedGenus`, and `crossFormatTriangleCountsAgree`. Everything dimensional —
`vertexCount`/`triangleCount` (this subset's own `🔬️probes/📜️script.ts` `compareMeshes` gate
documents in its own comment that "volume and area are triangulation-invariant... unlike a curved
BRep solid", i.e. raw counts are producer freedom, not equality), `volume`, `surfaceArea`,
`boundingBox`/`boundingBoxDiagonal`, the `weld` block, `crossFormatTriangleCounts`, the `analytic*`
self-checks and `expectedBaseZ` — is stripped via `ignoreKeys` and left to the paired tolerance
profile instead, because no single flat number can honestly serve a comparison profile applied to
fixtures spanning a `boundingBoxDiagonal` of 4.5 mm to 4.5 Mm (nine orders of magnitude) in this
subset's own corpus.

**`fem-polygon-exact`** — every number was pulled from the fixture corpus itself, not guessed:

- Scanned all 27 `expected.metrics.json` files (12 in ◻2d + 15 in 🧊️3d). Worst observed float32
  STL/OBJ round-trip noise anywhere in the corpus: `analyticVolumeRelativeError` 2.036e-6 and
  `minZRelativeError` 2.12e-8, both at the smallest-scale fixture (`scale-one-hole-1e-3`,
  `boundingBoxDiagonal` ≈ 4.48e-3). The weld grid is `diagonal × 1e-7` at literally every scale, by
  construction (it is the welding tolerance the generator itself uses).
- `relativeLength: 1e-6`, `relativeArea: 1e-5`, `relativeVolume: 1e-5` — ≥10× margin over that
  measured noise floor.
- `absoluteLength/Area/Volume: 1e-9` — a floor for near-zero references, matching the smallest
  fixture's own weld grid (1e-9 at the 1e-3 scale).
- `normalizedHausdorffMax` / `normalizedCentroidDistanceMax: 1e-4` — derived from this same
  subset's own carrier-oracle rationale text (already committed in the `three-fem2d-mesh-reader` /
  `three-fem3d-mesh-reader` oracle entries in the same file), which records MEASURED accept/reject
  numbers: a genuinely different region/solid separates at `relativeVolumeError` up to 7.083e-1 and
  `normalizedSymmetricHausdorff` up to 1.242; the SUBTLEST real defect on record (same shape, wrong
  elevation — `solid-one-hole` vs `solid-one-hole-elevated-low`, where the elevation-invariant
  volume/area metrics read ≈0) still shows `normalizedSymmetricHausdorff` 4.465e-1. `1e-4` sits
  roughly 4×10³ below that smallest real signal and roughly 100× above the representation-noise
  floor.
- `maxOverrideFactor: 5` — matches the CORE `mechanical-standard`/`large-coordinate` profiles' own
  headroom; reserved for a future fixture with a documented, capped exception rather than a silently
  widened gate.

Both subsets' `🏭️generator/📜️script.ts` already wrote fixtures naming exactly these two ids (this was
already the intended design — the registry entries were simply never contributed), so no generator
edits were needed there.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json` — repointed

5 fixtures (`json-base-insert-array-element`, `-remove-array-element`, `-remove-member`,
`-set-member`, `-set-scalar`) named `comparisonProfile: "json-value-tree"` and
`toleranceProfile: "exact"`. Both were unregistered.

- `"json-value-tree"` → **CORE `ordered-json-v1`** ("Structural identity with array order
  significant; key order never is"). This subset's own oracle (`json-rust-rfc8259-mutate`) already
  declares `comparisonProfiles: ["ordered-json-v1"]` — the fixture manifest was simply naming a
  never-defined synonym instead of the profile the oracle itself already uses. `insert-array-element`
  / `remove-array-element` mutate array POSITION, so order-significant comparison is the correct
  (not merely convenient) semantics here.
- `"exact"` (tolerance) → **removed**, not replaced. `toleranceProfile` is optional on
  `FixtureManifest`, and a `ToleranceProfile` is a scale-relative GEOMETRIC policy
  (`absoluteLength`/`relativeVolume`/`normalizedHausdorffMax`/…) — nothing in a plain JSON edit is a
  continuous dimensional quantity, so inventing a profile named `"exact"` with every geometric field
  zeroed would be exactly the "stub that gates nothing" the brief warned against. `ordered-json-v1`'s
  own `tolerance: 0` default (implicit — the field is simply absent) already enforces byte-exact
  numeric equality; a tolerance profile adds nothing true to say.

Fixed in `🧪️oracle/🔣️.json`, the generator's hardcoded `comparisonProfile`/`toleranceProfile` literals
in `🏭️generator/📜️script.ts`, and the generator's companion listing `🧫️fixtures/🔣️.json` (not read by
the test harness — the harness reads `🧪️oracle/🔣️.json` — but left in sync so it doesn't read as a
second, stale source of truth).

### `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence` — repointed

4 fixtures (`sequence-csv-create-step`, `-delete-step`, `-duplicate-step`, `-edit-step-params`) named
`comparisonProfile: "csv-row-set"` and `toleranceProfile: "exact"`.

- `"csv-row-set"` → **CORE `unordered-json-v1`** ("arrays compared as multisets"). The owning
  oracle's own rationale text (`csv-rfc4180-reader` entry, same file) states explicitly that the
  reader "re-derives the ROW SET from bytes" — i.e. row order was deliberately declared
  insignificant (the CSV carrier has no edge/position concept — `move-step`/`connect-steps` write
  fields this carrier cannot see at all, per that same rationale — so a step's declaration order in
  the file is not meant to be gated here). That is exactly CORE `unordered-json-v1`'s semantics,
  under the assumption — which I verified is how the fixtures are shaped, one flat `{id, kind,
  params}` record per row, no nested arrays — that rows are objects, not tuples (a tuple would make
  `arrays: "set"`'s recursive sort scramble the row's own field order; an object's own key order is
  already insignificant, only the top-level list of row-objects gets set-compared).
- `"exact"` (tolerance) → **removed**, same reasoning as JSON: no continuous dimensional quantity
  in a CSV row edit, so no tolerance profile applies.

Fixed in `🧪️oracle/🔣️.json`, the generator's hardcoded literals in `🏭️generator/📜️script.ts`, and the
companion `🧫️fixtures/🔣️.json` listing.

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf` — repointed, all 9, no tolerance profile needed

9 fixtures across three subsets named `comparisonProfile: "semantic-pdf-conformance-v1"`, which was
never defined anywhere. In every case the fixture ALSO already names a real, already-wired
`comparisonPipeline` (`pdf-1-4-base-lopdf-compare-v1` / `-a-lopdf-compare-v1` / `-x-lopdf-compare-v1`
— each with registered, qualified probes and a gating `equal: true` stage), and each subset already
owns a correctly-scoped comparison profile matching that pipeline's actual axis:

| subset | fixtures | wrong name | repointed to |
| --- | --- | --- | --- |
| `✳️base` | insert-page, remove-page, move-page, resize-page, replace-page-text | `semantic-pdf-conformance-v1` | `semantic-pdf-structural-base-14-v1` (already defined: page list, MediaBox/CropBox/Rotate, content bytes, /Info, object inventory) |
| `✳️a` | set-page-text, clear-page-text | `semantic-pdf-conformance-v1` | `semantic-pdf-1-4-conformance-a-v1` (already defined, `tolerance: 0`) |
| `✳️x` | set-page-size, collapse-page-size | `semantic-pdf-conformance-v1` | `semantic-pdf-1-4-conformance-x-v1` (already defined, `tolerance: 0.001`) |

No tolerance profile was referenced by any PDF fixture (confirmed against the baseline
`breach-fixture-tolerance-profile-unknown.json` dump — PDF does not appear in it), so none was added.
Also fixed the same stale literal (`COMPARISON_PROFILE = "semantic-pdf-conformance-v1"`) in all three
subsets' `🏭️generator/📜️script.ts`, so a future regen writes the correct name instead of reintroducing
the breach. PDF has no companion `🧫️fixtures/🔣️.json` listing file, so nothing else to sync.

## The three `missing-fixture` cases

All three (`fem/◻2d/🧪️tests/mutate-fem2d-1`, `fem/🧊️3d/🧪️tests/mutate-fem3d-1`,
`sequence/🎬️sequence/🧪️tests/mutate-sequence-1`) had the identical shape: the `.feature` file's
`asset://…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` step referenced the OLD, pre-kind-only-
migration filename. The file on disk is `🗣️.dsl.semio` (kind-only, already migrated — same pattern
wave 0 fixed for 3676 other references, these three were simply missed). Repointed all four
occurrences of `🗣️example.dsl.semio` → `🗣️.dsl.semio` inside each of the three `.feature` files
(fem2d: 2 occurrences, fem3d: 2, sequence: 4) — no new fixture asset needed, the referenced document
already exists.

## Files touched

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — added `semantic-fem-mesh-manifold-v1` comparison profile + `fem-polygon-exact` tolerance profile
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — same, own copy
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🧪️tests/mutate-fem2d-1/🥒️.feature` — `🗣️example.dsl.semio` → `🗣️.dsl.semio` (×2)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/mutate-fem3d-1/🥒️.feature` — same (×2)
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🧪️tests/mutate-sequence-1/🥒️.feature` — same (×4)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧪️oracle/🔣️.json` — repointed 5 fixtures to `ordered-json-v1`, dropped `toleranceProfile: "exact"`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🏭️generator/📜️script.ts` — same literal fix
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧫️fixtures/🔣️.json` — synced (generator's companion listing, not read by the harness)
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — repointed 4 fixtures to `unordered-json-v1`, dropped `toleranceProfile: "exact"`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` — same literal fix
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🔣️.json` — synced
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧪️oracle/🔣️.json` — repointed 5 fixtures to `semantic-pdf-structural-base-14-v1`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🏭️generator/📜️script.ts` — same literal fix
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🔣️.json` — repointed 2 fixtures to `semantic-pdf-1-4-conformance-a-v1`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🏭️generator/📜️script.ts` — same literal fix
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🔣️.json` — repointed 2 fixtures to `semantic-pdf-1-4-conformance-x-v1`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🏭️generator/📜️script.ts` — same literal fix

## Left open (out of shard scope, noted for whoever picks it up)

- The `fem-mesh-manifold-v1` comparison profile does not (yet) wire an actual runtime comparator —
  the `expected-measurements` role file is written by both subsets' generators but nothing currently
  reads it back at test-execution time to produce an `actual-measurements` counterpart; the fixture
  reference now RESOLVES honestly (real semantics, real numbers) but the pipeline that would apply
  them end-to-end is future work, same as the rest of this ticket's scaffolding-in-progress mesh
  oracles.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🔬️probes/📜️script.ts` (and the 🧊️3d twin) has a doc-comment
  reference to `📓️fem-mesh-oracle-report.md`, which does not exist anywhere in the repo — a stale
  comment, not a fixture reference the judge checks, so it's not one of my three breach ids, but worth
  a follow-up.
- All other breach ids still present under my five paths (`unsplit-artifact-subset`,
  `runtime-inventory-missing`, `oracle-capability-mismatch`, `binary-protocol-drift`,
  `missing-external-oracle`, `stub-serializer`, `unknown-mutation-catalog`, `oracle-in-production`,
  `capability-without-manifest`) are pre-existing and out of this shard's assigned classes — counts
  confirmed unchanged against baseline (see table above).
