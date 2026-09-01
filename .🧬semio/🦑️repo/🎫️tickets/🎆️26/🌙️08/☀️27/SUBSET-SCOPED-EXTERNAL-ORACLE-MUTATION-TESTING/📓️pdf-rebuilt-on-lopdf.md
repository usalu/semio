# 📄️ pdf — every subset rebuilt so the evidence is admissible

Companion to `📓️pdf-fixtures-are-not-admissible-evidence.md`, which established that pdf's fixtures
could not be fixed by registering a reader: their `after` bytes came from **our own** mutation engine.
This is the first subset actually rebuilt.

## What was wrong

`🏭️generator/🦀️engine/src/main.rs:36` imported `oracle_apply_mutation`, `oracle_inverse_spec`,
`oracle_round_trip` and `project_conformance` from `semio-s-plugin-stdio-test-oracle`. So
`mutated.pdf` was our code's output and `expected-projection.json` was our code's prediction of how it
reads. `lopdf` laid out the seed and serialised — a codec for our own answer.

## What replaced it

`🏭️generator/🦀️lopdf-engine`, a standalone `[workspace]` crate depending on **`lopdf` and nothing
else**, with three parts:

* `src/lib.rs` — `build_seed`, `arrange` (puts each kind's precondition in place), `apply` (performs
  the mutation through lopdf's own public COS API), `project` (reads the vt conformance axes back).
* `src/generate.rs` — writes `base.pdf` + `mutated.pdf` per kind.
* `src/reader.rs` — the judge: `project` / `compare`, emitting `probe-report/v2`.

The old `🦀️engine` was deleted rather than left beside it. Keeping it would have kept the disqualifying
import in the subset's generator tree, and the repository is greenfield — no compatibility layers.

**Observability is enforced at generation time.** `generate` projects both sides and refuses to write a
pair whose projection does not move. A no-op mutation therefore cannot be committed as a fixture that
would pass forever.

## Result: 16 of 18 kinds

| | |
|---|---|
| Witnessable, repointed onto `lopdf-pdf-1-7-vt-mutate-reader` | **16** |
| Recorded `-uncarried` | **2** |
| Gate directions correct | **32/32** — 16/16 `(base,base)` equal, 16/16 `(base,mutated)` unequal |
| Byte-reproducible | yes, aggregate sha256 unchanged across a full regeneration |

### The two uncarried kinds, and why

`insert-encryption-dictionary` and `remove-encryption-dictionary`. lopdf 0.44's writer takes its
encryption path whenever the trailer carries `/Encrypt`, and then requires the encryption state a
genuine decryption would have recorded. A synthetic encryption dictionary can therefore be neither
written nor read back — generation failed with `object ID 8 0 not found` on its own output.

This is a **writer-side** limit, the same category as `tiff::change-byte-order` and `gif@87a`. It was
found by the generator's own observability check refusing to proceed, not by inspection — which is the
argument for having that check.

## The gate moved with the repository

`fixtureWriterProvenanceBreaches` had been pinned at "expect 4 flagged pdf subsets". After the rebuild
it found 3 and the harness failed. That is the gate working: the expectation was updated to 3, with a
new `fixture-writer/vt-is-rebuilt` check asserting vt specifically no longer appears. The repository
was not trimmed to fit the expectation.

## Coverage

| | before | after |
|---|---|---|
| externalOracleCoverage | 455/658 (69.15%) | **471/658 (71.58%)** |
| oracleEvidenceCoverage | 375/658 (56.99%) | **379/658 (57.60%)** |
| Harness | 120/120 | **121/121** |

## Every pdf subset, rebuilt

The vt template was then applied to all nine remaining subsets. Each has its own
`🏭️generator/🦀️lopdf-engine` (standalone `[workspace]`, `lopdf` as its only dependency); every old
`🦀️engine` that imported our mutation engine was deleted rather than left beside it.

| Subset | Kinds | Reader | Uncarried | Notes |
|---|---|---|---|---|
| 1.7/vt | 18 | 16 | 2 | the template |
| 1.7/x | 14 | 12 | 2 | vocabulary already covered by vt's engine |
| 1.7/e | 12 | 10 | 2 | ditto |
| 1.7/a | 14 | 12 | 2 | added embedded files + `/AFRelationship` |
| 1.7/ua | 11 | 11 | 0 | **no generator and no fixtures before**; added the PDF/UA accessibility axes |
| 1.7/h | 10 | 10 | 0 | **no generator and no fixtures before**; added signature fields + `/Info` author |
| 1.7/base | 16 | 16 | 0 | generic COS layer: pages, objects, dict and trailer entries |
| 1.4/a | 2 | 2 | 0 | **no generator before** |
| 1.4/base | 5 | 5 | 0 | **no generator before** |
| 1.4/x | 2 | 2 | 0 | **no generator before** |
| **total** | **104** | **96** | **8** | |

`reader + uncarried == kinds` for every row: pdf is fully accounted for.

`1.7/base` keeps its pre-existing `report-strip.pdf` seed ASSET, which other subsets reuse — that
generator was always clean. Its single `📜️script.ts` now drives both engines (the asset engine and the
mutation engine), because CLAUDE.md permits exactly one script file per node.

### The only uncarried kinds: encryption, in the four subsets that declare it

`insert-encryption-dictionary` and `remove-encryption-dictionary`. lopdf 0.44's writer takes its
encryption path whenever the trailer carries `/Encrypt`, then requires the encryption state a genuine
decryption would have recorded — so a synthetic encryption dictionary can be neither written nor read
back. Generation failed with `object ID 8 0 not found` **on its own output**, which is how this was
found: the generator refuses to write a pair it cannot read back, rather than committing one.

A writer-side limit, the same category as `tiff::change-byte-order` and `gif@87a`.

## Coverage

| | turn start | after pdf |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **546/658 (82.98%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **471/658 (71.58%)** |
| Fixtures | 705 | **767** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

## What remains repo-wide

112 kinds. Of those, **33 are already recorded `-uncarried`** with source-verified reasons — obj 10
(tobj is a mesh-only reader, blind to document-only structure), pdf 8, jpg 6, gif@89a 5, png 3,
tiff 1. The genuinely open work is `fem2d`/`fem3d` (44, needing a third-party FEM library and having
no fixtures or runtime inventory at all), `gif@87a` (12, writer-blocked — see its own report),
`mathematical` (9), `semio` (5), `sequence` (4) and `draw` (3).
