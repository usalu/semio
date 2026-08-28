# 📓️ Where this stands, and what the next session should pick up

## Verified now

| | |
| --- | --- |
| Mutation manifests | 13 owners, **286** mutations |
| Externally oracled | **238/286 (83.2%)** |
| Fixtures | **285**, 100% provenance, 100% reproducibility |
| Dependency isolation | 100% (164/164) |
| Harness | **104/104** |

Fully covered subsets: `semio@v1/mesh` 17/17, `semio@v1/brep` 13/13, plus `gltf` 120, `note` 33,
`png` 15, `jpg` 10, `tiff` 6, `bmp` 5, `pdf` 5, `step@ap214/cc6`.
Partially and honestly covered: `sequence` 4/8, `fem2d` 3/25, `fem3d` 3/25, `note` 16/33.

## The number that matters, and why it keeps falling

`stubSerializerBreaches` shipped finding 80 broken exporters and now finds **163**. Every increase came
from using the gate for something, never from reviewing it:

| shape | count | found by |
| --- | ---: | --- |
| DSL text under a standard extension | 97 | the original audit |
| pack-envelope transmute | 33 | reading `shooting → png` |
| serde coercion into an empty document | 19 | `architect/program` refusing to register |
| never reads its input | 14 | consolidating two shapes a worklist named |

Plus one false-POSITIVE class: `#[cfg(test)]` round-trip proofs call the functions the detector watches
for, so two REAL carriers read as stubs. A gate must be wrong in neither direction.

Reachability was restated three times: **664 → 300 → 290**, then reconciled against an independent
per-subset worklist to **357 reachable / 1616 blocked / 174 already manifested, out of 2147 mutation
kinds**. The final restatement was not another gate correction but a denominator correction — the
earlier sweeps counted mutation-leaf DIRECTORIES, while the manifest generator and coverage matrix both
read `mutationCatalogs[].kinds`. Counting what the tools actually consume put the two independent
measurements within seven kinds of each other. All of it is recorded in `📓️reachability.md` rather than
quietly replaced.

## Two self-inflicted bugs worth remembering

* **`manifest --write` replaced manifests wholesale.** It derives structure from leaf descriptors and
  knows nothing about scope, so re-running it flattened `sequence`'s hand-scoped 4-carried/4-uncarried
  split into eight undifferentiated mutations — turning an honest partial into a claim of blanket
  coverage. It now MERGES: derived fields refresh, refined `oracleRequirements`/`invariants`/`carriers`
  win. Pinned by `manifest/refined-scope-survives-regeneration`.
* **A peer tool rewrote 9 oracle files** from a template with no `mutationManifests` key, dropping 169
  mutations from the registry. Regenerating restored them. If it recurs, that template needs the field.

## Next, in order

1. **357 mutations behind a real carrier** are registrable with no new exporter. `📓️worklist.md` ranks
   them; ~23 subsets already have an oracle chosen and need only fixtures, probes and a manifest.
2. **~249 mutations across 15 composite owners** need ONE mechanism, not fifteen exporters: a
   child-resolving export context, so a composite delegates to its child's serializer. Its children are
   `semio.mesh` and `semio.brep` — already oracled. See `📓️composite-delegation.md`.
3. **Runtime inventory is still 0, but the blocker is nearly gone.** `semio-s-plugin-stdio` went from
   4620 compile errors to **10**, and the remainder moved to `semio-framework-plugin`: a `use super::{
   ArtifactEditor, ArtifactViewer, ViewerApp}` that no longer resolves and a `declarations` module at a
   different nesting depth, in a 27k-line shared file. Those names all still exist — the signature of
   the consolidation tooling reordering the file mid-refactor. Peer work; poll it, do not chase it.
4. **Withdraw export dialects that cannot be given a meaning** rather than implementing them. An STL of
   a generation graph is undefined, not unimplemented.
