# BSP Original-Plane Lineage Repair

The original-plane lineage correction is now accepted by ten independent kernel tests and the cached actual2,443-Face component gate. Native replay remains on hold for the separate forward-paint oracle/renderer acceptance; this kernel gate is no longer pending.

The original implementation recomputed a BSP splitter plane from an already clipped thin triangle. In the observed source 5860/polygon 0 descendant, that reconstructed normal drifted by about 4×10⁻⁹; two vertices then lay about 1.00018×10⁻¹² m from its own reconstructed plane, beyond the unchanged 64-ULP classification bound of 1.42109×10⁻¹⁴ m. The unchanged face was repeatedly queued on one side. The 30-second baseline reached 717,056 nodes after true splits stopped at 7,854. This is a functional nonprogress defect, not a timing limit.

The saved implementation keeps one plane per validated original input Face instance. All concave ears and later split descendants inherit that plane. Source/polygon IDs are preserved but do not identify a plane: canonical warped triangles and disconnected inputs may reuse those IDs. Each selected splitter must actually classify coplanar against its inherited plane; violation raises an explicit invariant error. Source validation, 64-ULP precision, material geometry and output normals are unchanged.

## Test-First Chronology

The auditor recovered four irreducible original inputs (4047, 5672, 5673 and the 5860 octagon). Public ordered_fragments runs normal preparation and reproduces the same sliver. Removing any remaining original input loses that exact witness. The permanent language-neutral lineage case contains these four inputs; it never feeds the sliver directly into preparation.

The lineage plane retains the exact original validation origin, including concave inputs whose first prepared ear starts at another vertex. A separate permanent regression protects that anchor.

The regression requires every node to consume material, so node count cannot exceed prepared convex input count plus actual splits. It also checks NumPy area conservation, 783 independent actual-camera original-versus-painted rays, unchanged output normals, and identical geometry when all four inputs deliberately reuse the same source/polygon IDs. Seven existing scenario groups remain in place.

AJV accepts the new contract and rejects seven malformed controls. An additional pure-stdlib Python 3.14.4 smoke completes the exact four original inputs as eight fragments/eight nodes in 0.0010561 s, with unchanged source/schema/case endpoints. This cross-interpreter termination smoke does not replace the NumPy/Pillow/full-component gate. Its retained record is stdlib-lineage-smoke.json (SHA-256 97afe6e4f25a69e9903c1bd9fd04e2401804eeeaee64c7ad1439fc1ce6155247). The first admitted teaching-runtime run (41373) completed nine tests with eight passing and one new actual-camera ray-oracle failure; exact endpoints matched. The inherited fixed 1e−10 boundary predicate falsely included a point 5.0315 cm from a 1.744 nm edge. Independent actual-segment geometry confirmed the cause. A new portable tiny-edge regression and a metric 64-ULP boundary correction strengthen the test oracle; the kernel implementation is unchanged. Retry35286 was then submitted with ten tests plus the cached2,443-input benchmark; its completed accepted result follows below. The original first-run records are preserved in lineage-kernel-first-run. A separate cp1252 console-output failure occurred after durable records were written; the ticket validator now emits UTF-8. The earlier queued baseline handle 51596 was cancelled before admission after the corrective save; it is not a test result. The independently recorded original red remains preserved.

## Exact Inputs and Current Source

- Kernel: 20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822
- Schema: 0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4
- Portable cases: d720c478b5f92ba84a361782e94db093c00780ce5ffcf5155e05a69a1249d6b9
- Python tests: a673a7c26d96c95ed8d2a3a6c014fbc4f3c5908fb0de2e275ecba8eb6eb0e518
- Original 2,443-Face component: 37600c55448e9582e9bd03534346dddcc6c32ab683c9e7b2ef2923ceb7cb953a
- Four-input witness: bc5ea1e97053cfeb357f0f1276c1f8fb64bb16635620b046246c7fdb5037c9d6

Inputs and audit scripts remain in this ticket. Generated red records are bsp-self-plane-witness.json, bsp-lineage-witness.json, bsp-minimal-public-result.json and largest-bsp-benchmark.json. The current benchmark consumes the cached component directly and does not rebuild the renderer or geometry.

## Current Accepted Kernel Gate

Retry35286 completed all ten tests in10.131s with exact unchanged endpoint maps. The actual component completed in2.703s:2,443 original inputs,2,460 prepared convex pieces,11,431 true splits,13,437 consuming partition nodes and13,891 positive output pieces. Every output has at least three vertices and strictly positive independent NumPy area. Maximum per-source boundary area error is1.7763568394002505e−15. The per-node consumption invariant holds. The aggregate process exited1 only because the later, separate paint probe had an incorrect model-schema dependency path before import/geometry; that is neither a kernel failure nor an observed paint RED. The path is corrected, and isolated paint baseline79208 is queued normally.

Kernel durable record (source endpoints identical):

```json
{
  "tests": 10,
  "success": true,
  "failures": 0,
  "errors": 0,
  "before": {
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🧪️cases\\🔣️.json": "d720c478b5f92ba84a361782e94db093c00780ce5ffcf5155e05a69a1249d6b9",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🧪️tests\\🐍️.py": "a673a7c26d96c95ed8d2a3a6c014fbc4f3c5908fb0de2e275ecba8eb6eb0e518",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🧪️tests\\🟨️.cjs": "8571a99dc4079f0c81db1664360e123834d5d49684e01fc1cf05d1cc643ff349",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-kernel.py": "4bcd0351156b370fc60f479a3225911eb1377c4d5388ff7106db1670a457448a"
  },
  "after": {
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🧪️cases\\🔣️.json": "d720c478b5f92ba84a361782e94db093c00780ce5ffcf5155e05a69a1249d6b9",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🧪️tests\\🐍️.py": "a673a7c26d96c95ed8d2a3a6c014fbc4f3c5908fb0de2e275ecba8eb6eb0e518",
    "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🧪️tests\\🟨️.cjs": "8571a99dc4079f0c81db1664360e123834d5d49684e01fc1cf05d1cc643ff349",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-kernel.py": "4bcd0351156b370fc60f479a3225911eb1377c4d5388ff7106db1670a457448a"
  },
  "unchanged": true
}
```

Actual-component durable record:

```json
{
  "outcome": "completed",
  "error": null,
  "elapsed": 2.702999999994063,
  "input_faces": 2443,
  "output_pieces": 13891,
  "counts": {
    "splits": 11431,
    "nodes": 13437,
    "classifications": 218601,
    "prepared": 2460
  },
  "maximum_area_error": 1.7763568394002505e-15,
  "before": {
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
    "C:\\git\\semio\\.🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\largest-component-faces.json": "37600c55448e9582e9bd03534346dddcc6c32ab683c9e7b2ef2923ceb7cb953a",
    "C:\\git\\semio\\.🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-lineage-bsp.py": "cacf15dd1d6cdec39f13635e1575b5322ee70d7b5e0b144e0cc49e117fff066e"
  },
  "after": {
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
    "C:\\git\\semio\\.🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\largest-component-faces.json": "37600c55448e9582e9bd03534346dddcc6c32ab683c9e7b2ef2923ceb7cb953a",
    "C:\\git\\semio\\.🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-lineage-bsp.py": "cacf15dd1d6cdec39f13635e1575b5322ee70d7b5e0b144e0cc49e117fff066e"
  },
  "unchanged": true
}
```

Record SHA-256 values: fragment-kernel/result.json=70fa3ed19d3ba6f0d06797a82ab2d536212e46222af99dc99017a9caf523e426; lineage-bsp/green.json=e2d084953434bd4abd15947ad6bdee103fc562fe8d60ad96e23ee914610a3d6d.
