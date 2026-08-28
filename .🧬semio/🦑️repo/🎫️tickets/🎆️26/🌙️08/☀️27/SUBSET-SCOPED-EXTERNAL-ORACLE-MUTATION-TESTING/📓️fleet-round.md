# 📓️ What the fleet round produced, and what it corrected

Six subsets registered in one round, every one of them correcting something a previous pass had
asserted. The corrections are the point of recording this: each would have produced a green test that
proved nothing.

## Landed

| subset | mutations | witnessed | fixtures | notes |
| --- | ---: | ---: | ---: | --- |
| `semio@v1/document` | 18 | 15 | 24 | 3 uncarried — no export path emits the image store |
| `semio@v1/cad` | 16 | 16 | 21 | 2 carry an extra uncarried requirement for Ellipse/Dimension |
| `semio@v1/drawing` | 17 | 17 | 0 | fixtures deliberately withheld, see below |
| `bcf@2.1` | 14 | 12 | 15 | jszip implicit-folder-timestamp bug found and fixed |
| `gif@89a` · `las@1.0` · `pdf@1.7` | 52 | — | 3 | pdf subset scoping still being corrected |

## Six corrections, each caught by reading or measuring rather than trusting

* **`cad` → STEP via brepjs was my suggestion, and it is wrong.** Measured with the already-qualified
  cc6 probe so the result is attributable: a committed `brep` fixture gives `imported: 1`, a cad-shaped
  STEP gives `imported: 0`. OCCT transfers through product/shape-representation structure; our cad
  export emits bare primitives.
* **`ruststep` cannot WRITE.** `Record`, `DataSection` and `Exchange` do not implement `Display`. So cad's
  STEP fixtures are honestly classed `handcrafted`, not `third-party-generated`.
* **The DXF oracle reaches 7 of 9 shapes, not 9.** ELLIPSE, LWPOLYLINE and DIMENSION are R13+ and this
  dialect is r12. An earlier pass had read "our exporter maps all nine" as "the oracle can witness all
  nine" — those are different claims.
* **`dwg` is not a real writer**, contrary to the brief I gave. `serialize` is one line returning
  `Err(PackError::Schema("…unsupported…"))`.
* **docx FLATTENS `List`, so `set-list-ordered` has no docx encoding at all** — the prior research
  understated the losses as "only PageBreak and inline colour/font/link dropped".
* **DXF walks through a `Group` without applying its matrix**, so `rotate`, `scale`, `group`, `ungroup`,
  `flatten` and `unflatten` produce byte-identical DXF before and after. Six kinds a prior pass had
  listed as dxf-witnessable are not.

## Two structural fixes to the platform itself

* **The ownership key omitted the subset** (`📦️index.ts`). `cad::set-snapshot` collided with
  `document::set-snapshot`, `brep::move-vertex` with `mesh::move-vertex`, and each was reported as
  duplicate ownership. That is artifact-level reasoning inside the one platform whose purpose is
  subset-level scoping. Duplicates 3 → 0.
* **`oracleEvidenceCoverage`**, release-gated: a mutation counts only with a qualifying oracle AND at
  least one fixture targeting its subset. Registration alone had been reading as coverage.

## Why `drawing` has zero fixtures on purpose

A `third-party-generated` fixture's `after` state is the expected RESULT of a mutation. No third-party
library computes that for this subset, and writing it by hand is exactly what
`reimplementation-registered-as-third-party` now blocks. The agent left the gap open rather than filling
it with our own answer — and left the runtime-inventory breach standing rather than hand-writing a
bridge, which would have made the equality gate compare the manifest with itself.

That is the right instinct, and it is also why `oracleEvidenceCoverage` (45.7%) sits below
`externalOracleCoverage` (48.1%): the difference is registrations that cannot yet be exercised.

## Standing

| | |
| --- | --- |
| Manifests | 20 owners, 403 mutations |
| Qualifying oracle registered | 194 (48.1%) |
| **Oracle AND evidence** | **184 (45.7%)** |
| Subset ownership | 382/403 (94.8%) — 21 wildcard, pdf and jpg |
| Fixtures | 348, 100% provenance, 100% reproducibility |
| Harness | 115/115 |

Open findings, all real: `stub-serializer` 163, `reimplementation-registered-as-third-party` 38,
`wildcard-subset-owner` 21. Runtime inventory remains 0/20 — the oracle side of every subset above is
exercisable today, the SUBJECT side is not, because `semio-s-plugin-stdio` still does not compile.
