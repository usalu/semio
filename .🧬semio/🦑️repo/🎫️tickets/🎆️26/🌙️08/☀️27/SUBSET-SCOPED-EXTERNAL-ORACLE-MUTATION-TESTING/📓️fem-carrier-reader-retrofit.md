# 🏗️ fem2d / fem3d — 44 kinds closed by changing the CARRIER, not the reader

Both fem subsets already carried two qualifying `third-party-library` oracles —
`three-*-mesh-reader` and `manifold-*-mesh-measure` — and both covered **three** kinds each. The other
**22 each** were recorded `-uncarried`.

That label was correct about the mesh and wrong as a general claim, which is the same mistake the gltf
retrofit turned up one layer down.

## Why the mesh oracles cover so little

They read the STL/OBJ export. A material's Young's modulus, a section's second moment of area, a
support's restrained DOFs, a load case's self-weight flag and the analysis settings **do not move a
single triangle**. No mesh reader, however good, can witness them. Hence 22 uncarried each.

## The carrier that does witness them

These subsets export to six formats. Five are stubs in the sense this ticket's `stub-serializer` gate
describes — csv, md and txt each wrap the DSL text in a **single blob**:

```rust
// csv leaf: one record, one field, the whole DSL as a quoted string
records: vec![CsvRecord { fields: vec![CsvField { value: "payload".into(), .. }] }, ...]
```

The **json** leaf is not:

```rust
// 🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json
let raw = serde_json::to_value(snapshot)?;   // the real structured tree, every field
Ok(JsonSnapshot::from_value(raw))
```

Its own docstring says so explicitly: *"the real structured JSON tree (every `Fem2dSnapshot` field, not
a single blob like the csv/md leaves)"*. So all nine collections — nodes, elements, regions/solids,
materials, sections, supports, loadCases, combinations, analysis — are **carrier-level facts**, and a
third-party JSON implementation witnesses every one of the 22.

This is the same shape as two readers already accepted in this repository: `quick-xml` judging svg, and
`burntsushi-csv` judging mathematical. The judge is a third-party implementation of the **carrier**,
and nothing here predicts the answer it is judging.

## What was built

`🏭️generator/🦀️json-engine` per subset — standalone `[workspace]`, **`serde_json` as its only
dependency**. It builds a deterministic seed carrying at least two of every collection (because
`delete-*` and `replace-*` are only observable when the collection does not empty to nothing), applies
each mutation as an edit to the JSON carrier, and reads the nine collections back through `serde_json`.

Field spelling follows the snapshot's own serde contract, read from the Rust source rather than
guessed: `#[serde(rename_all = "camelCase")]` on every record, `#[serde(tag = "kind")]` on the
`FemElement`/`FemLoad` enums, and `FemDof` **unrenamed** — so `"Tx"`, not `"tx"`.

fem3d differs where the domain differs: nodes carry `z`, materials carry a shear modulus `g`, sections
carry `iz` and `j`, elements are `Bar`/`Frame` (with a roll angle), the meshed body collection is
`solids` rather than `regions`, and loads name `wz`/`solidId`.

Each subset's existing `📜️script.ts` gained a `carrier` / `carrier-manifests` mode rather than a second
script file, per CLAUDE.md's one-script rule. The mesh recipes are untouched — this engine does not
replace them, it answers the question they structurally cannot.

## Evidence

* **88/88 directions correct** — 22/22 `(before, before)` equal and 22/22 `(before, after)` unequal, in
  each subset.
* Observability is enforced at **generation** time: the engine refuses to write a pair whose carrier
  projection does not move, so a no-op cannot be committed as a fixture that would pass forever.
* Comparison canonicalises object keys but preserves **array order**, so a reordering is a difference
  rather than a tie. Numbers are compared as `serde_json` parsed them — no tolerance, because one here
  would silently accept a changed stiffness.
* Both corpora regenerate byte-identically.

## Result

| | before | after |
|---|---|---|
| externalOracleCoverage | 546/658 (82.98%) | **590/658 (89.67%)** |
| oracleEvidenceCoverage | 471/658 (71.58%) | **515/658 (78.27%)** |
| Fixtures | 767 | **811** (100% provenance, 100% reproducible) |

## The same question, asked of the rest

`mathematical`, `sequence` and `draw` all have direct `serde_json::to_value` exports too, so the same
pattern should reach their 17 remaining kinds. One caveat found while checking: **`sequence`'s snapshot
is `{schema, content}` where `content` is a CHILD REFERENCE** to a separate `s.stdio.semio.flow`
artifact — its steps and edges are not in its own carrier at all, so its four kinds need the composed
child's carrier, not this pattern applied naively. `draw`'s `DrawLayerNode` is a seven-variant enum over
nested bodies, so its three kinds need a correspondingly careful seed. Neither was attempted here
rather than attempted badly.
