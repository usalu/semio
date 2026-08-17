# Open-Closed Executable GLTF Leaves

## Result

The GLTF taxonomy now separates executable behavior, not only declarations.

## Inferences

- The shared measure module defines the generic GltfInferenceStage context/output contract.
- Every one of the 14 indicator files implements its own stage:
  size, area-volume, compactness, proportion, mass-distribution, curvature,
  thickness, concavity, clearance, adjacency, orientation, symmetry,
  roughness, and topology.
- Each stage owns both available computation and unavailable-result semantics.
- Pair and multi-part interpretations are owned by the relevant area-volume,
  clearance, adjacency, orientation, and symmetry leaves.
- The geometry module owns canonical decoding, topology, shared low-level
  geometry, context construction, and stage composition only.
- The geometry module contains no direct construction of any of the 14
  indicator records.

## Mutations

- The planning module defines the GltfSemanticMutation command interface and
  shared index/reference utilities.
- All 28 mutation payload files implement their own semantic validation and
  snapshot application.
- All command diff leaves plan directly through their payload implementation;
  set-snapshot retains its specialized direct snapshot diff.
- All inverse leaves retain their command-specific inverse implementation.
- The shared planner performs base validation, command dispatch, final
  validation, and structural diff creation only.
- The closed serialized mutation union remains the single exhaustive dispatch
  boundary required by the frozen schema; it contains no command semantics.

## Enforced Architecture

The ticket conformance audit now fails if:

- an inference group lacks its executable stage implementation;
- the geometry kernel directly constructs an indicator group;
- a mutation payload lacks its semantic command implementation;
- a diff leaf delegates through the closed command union;
- an inverse leaf lacks its implementation; or
- the shared mutation planner destructures command-specific payload fields.

## Verification

- Open-closed conformance audit: passed with 14 inference stages and 28 semantic commands.
- Rust formatting over all GLTF inference and mutation leaves: passed.
- Rust stdio compilation: passed.
- Focused TypeScript no-emit check: passed.
- Focused GLTF Nx suite: 91 tests run, 91 passed, 3367 skipped.
- Scoped git diff check: passed.
