# Conservative Coverage Witness

## Current single-cover extension

A second sufficient proof now removes a target only when its complete padded Item bounding rectangle lies inside ONE eligible actual convex opaque cover. The collector performs the existing strict actual 3D-plane separation first. The helper uses the same normalized expanded half-planes (+1e-10). It does not infer hidden material from union-covered probes, colour, role, size or a budget. If no single cover proves containment, the preceding early-retain witness and unchanged exact union subtraction remain authoritative.

GH authored seven portable cases and observed the missing-helper/clipping-spy baseline red before this implementation. The cases protect reversed winding, a triangle whose bounding box contains the target but whose actual material does not, disconnected target footprints, a concave cover, and both sides of the exact expansion boundary. NumPy and Pillow independently verify the sufficient proof; integration checks opaque/transparent/behind/coplanar controls and skips clipping only for the proven cover.

Final durable replay: **7 tests pass in 0.118 s with identical exact source endpoints**. AJV accepts both seven-case sections (14 total cases) and rejects seven malformed contracts. Actual selected-ID/profile validation is GH-owned and pending for this extension; the earlier necessary-witness-only 256/512 comparisons were exact unchanged and do not substitute for that new measurement.

```json
{
  "tests": 7,
  "success": true,
  "failures": 0,
  "errors": 0,
  "before": {
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "eb45da343c036f330122907821bb18fb0c0fcfedccd99e4a9db9ddb5a42c2e06",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\👁️occlusion\\🐍️.py": "0f9c3090cb72e1913eabba8eec408652f8dba0d64e0f992d9865eacafb1e9113",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.json": "0a9a43f6e29f3e58f20ecd38134b5cb6d2f446e33eb93092cd447fabeeef10a8",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.schema.json": "ef8f1e4c2c86a6c2751fb4a2bd1c0e30962b31912e9b7423130098d946a5b714",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-coverage-witness.py": "40fdac3cffce32279ad1a00f2292b5b9c3910d2e5c47fd1bcf41abf83ff9bf38"
  },
  "after": {
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "eb45da343c036f330122907821bb18fb0c0fcfedccd99e4a9db9ddb5a42c2e06",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\👁️occlusion\\🐍️.py": "0f9c3090cb72e1913eabba8eec408652f8dba0d64e0f992d9865eacafb1e9113",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.json": "0a9a43f6e29f3e58f20ecd38134b5cb6d2f446e33eb93092cd447fabeeef10a8",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.schema.json": "ef8f1e4c2c86a6c2751fb4a2bd1c0e30962b31912e9b7423130098d946a5b714",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-coverage-witness.py": "40fdac3cffce32279ad1a00f2292b5b9c3910d2e5c47fd1bcf41abf83ff9bf38"
  },
  "unchanged": true
}
```

## Prior necessary-witness proof chronology

The seven portable occlusion cases and schema precede implementation. The optimization may only retain a face when a footprint vertex or its centre is outside every eligible convex cover, using the same normalized half-plane expansion of 1e-10 as exact subtraction. Sample coverage never proves hidden material: off-centre holes and disconnected unions still run the existing exact subtraction.

Tests compare an independently constructed NumPy outward-normal oracle, Pillow union masks and exact baseline clipping, and spy on subtraction calls. Physical source, poses and dimensions remain frozen.

## Current result

Five targeted permanent Python tests pass in 0.133 s with identical exact source-byte endpoint maps. The strict AJV schema accepts all seven portable cases and rejects four malformed contracts. Independent NumPy outward half-planes use normalized distances and the same 1e-10 expansion; Pillow union masks establish the hole/coverage outcomes. A clipping spy proves exposed corner/centre cases skip subtraction while off-centre holes, disconnected samples and joined full coverage still execute it. Exact selected-item identity is compared against the original subtraction path with the witness forced to fall through. The existing Cairo pixel comparison and unknown-opacity protection also pass.

## Red chronology

Before implementation the two new tests produced two real clipping-spy failures (unnecessary subtraction for exposed corners/centre), together with eleven missing-helper errors. After the minimal helper/branch save, the initial four targeted tests passed in 0.143 s; the durable five-test replay above supersedes that initial run.

## Scope and remaining evidence

The helper is a necessary-condition rejection only: any outside probe retains the physical face. No covered probe can remove a face; exact convex-union subtraction remains authoritative. No colour, role, size or time-budget filter was added. Physical model code, dimensions, poses and material geometry were untouched. GH owns the next justified actual-capsule retained-ID/raster/performance comparison; that evidence is pending and is not implied by these bounded synthetic proofs.

## Exact proof endpoints

```json
{
  "tests": 5,
  "success": true,
  "failures": 0,
  "errors": 0,
  "before": {
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "a3bb71536248ebc642a3052d024fb925193191a43031c887fe40bf6752398735",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\👁️occlusion\\🐍️.py": "197d30a5df2321ad90ffea60ffe283d6cb8d33e03cd27899b376c88bcd157a33",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.json": "9e5ea6ce972438479039add7895a7ef6267e4e47e50d149b58cda7a04abb35ee",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.schema.json": "2ab80cce8e198859ef0cf4db65193fe31edf8cb8be887dc1e360ce1e15694544",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-coverage-witness.py": "b3011ec134803a2bc344c129a0a56b2d1eb7ec243751a13b32ac2b9906f1f5df"
  },
  "after": {
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "a3bb71536248ebc642a3052d024fb925193191a43031c887fe40bf6752398735",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\👁️occlusion\\🐍️.py": "197d30a5df2321ad90ffea60ffe283d6cb8d33e03cd27899b376c88bcd157a33",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.json": "9e5ea6ce972438479039add7895a7ef6267e4e47e50d149b58cda7a04abb35ee",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\👁️occlusion\\🔣️.schema.json": "2ab80cce8e198859ef0cf4db65193fe31edf8cb8be887dc1e360ce1e15694544",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-coverage-witness.py": "b3011ec134803a2bc344c129a0a56b2d1eb7ec243751a13b32ac2b9906f1f5df"
  },
  "unchanged": true
}
```
