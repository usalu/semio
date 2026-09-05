# GLTF Inference Naming Review

The non-topology inference leaves were inspected against their declared measurement responsibilities and sibling source files. Their stable inference IDs, cache keys, algorithms and payloads are unchanged. Existing meaningful names are retained. Topology belongs to the parallel GLTF owner.

The compactness group becomes `🗜️compactness`, rather than an arbitrary white-circle identity. Exact handpicked leaf decisions:

## ⚖️mass-distribution

- `🐧️inertia-tensor` → `🧮️inertia-tensor`
- `🟥️principal-frame` → `🖼️principal-frame`
- `🍐️moments-of-inertia` → `🌀️moments-of-inertia`
- `🐞️centroid` → `🎯️centroid`

## 📏️proportion

- `🧪️aspect-ratios` → `🖼️aspect-ratios`
- `🌾️slenderness` → `📏️slenderness`
- `🔮️flatness` → `📃️flatness`
- `🟫️elongation` → `↔️elongation`

## 🪞️symmetry

- `🌻️rotational-symmetry-score` → `🔄️rotational-symmetry-score`
- `🐯️rotational-symmetries` → `🔁️rotational-symmetries`
- `🌴️reflection-symmetry-score` → `⚖️reflection-symmetry-score`
- `🐞️reflection-symmetries` → `🪞️reflection-symmetries`
- `🍊️modularity-ratio` → `🧩️modularity-ratio`
- `🐸️repetition-ratio` → `🔂️repetition-ratio`

## 🌀️curvature

- `🐝️mean-curvature` → `🌀️mean-curvature`
- `🐨️sharp-feature-proportion` → `🗡️sharp-feature-proportion`
- `🟥️curvature-histogram` → `📊️curvature-histogram`
- `🟪️gaussian-curvature` → `🧮️gaussian-curvature`

## ↔️clearance

- `🦁️clearance-distribution` → `📊️clearance-distribution`
- `🟡️interference-volume` → `🚧️interference-volume`
- `🌾️minimum-distance-to-neighbors` → `📏️minimum-distance-to-neighbors`
- `🌹️overlap-volume` → `🫂️overlap-volume`

## 🤝️adjacency

- `🌳️contact-graph-degree` → `🌐️contact-graph-degree`

## 🕳️concavity

- `🪻️reentrant-volume` → `📦️reentrant-volume`
- `🚪️concavity-index` → `🔢️concavity-index`
- `🟨️convex-hull-gap` → `📏️convex-hull-gap`
- `⚪️reentrant-area` → `🪣️reentrant-area`

## 🧱️area-volume

- `🐨️total-area` → `🧮️total-area`
- `🍐️exposed-area` → `☀️exposed-area`
- `⚪️void-volume` → `🕳️void-volume`
- `🖱️surface-area` → `🧥️surface-area`
- `🦅️contact-area` → `🤝️contact-area`
- `🟫️volume` → `📦️volume`
- `🐯️enclosed-volume` → `📥️enclosed-volume`

## 📦️size

- `🌳️axis-aligned-bounds` → `↔️axis-aligned-bounds`
- `🌹️oriented-bounds` → `🧭️oriented-bounds`
- `🐯️characteristic-length` → `🔗️characteristic-length`
- `🖨️footprint-area` → `🦶️footprint-area`

## ⚪️compactness

- `🟤️hull-fill-ratio` → `🫙️hull-fill-ratio`
- `🖱️surface-to-volume-ratio` → `➗️surface-to-volume-ratio`
- `⚓️compactness` → `🗜️compactness`
- `🚪️compactness-index` → `🔢️compactness-index`
- `🪁️sphericity` → `🌐️sphericity`

## 🌊️roughness

- `🖱️surface-waviness` → `🌊️surface-waviness`
- `🟫️deviation-from-ideal` → `🎯️deviation-from-ideal`
- `🦊️irregularity` → `🪨️irregularity`
- `🍎️deviation-from-smoothed-geometry` → `🧽️deviation-from-smoothed-geometry`
- `🎨️normal-variation` → `🧭️normal-variation`

## ↕️thickness

- `⚪️minimum-thickness` → `📉️minimum-thickness`
- `🟤️thickness-variability` → `↔️thickness-variability`
- `🦋️mean-thickness` → `⚖️mean-thickness`
- `🛰️thickness-distribution` → `📊️thickness-distribution`

## 🧭️orientation

- `🌻️orientation-consistency` → `🧲️orientation-consistency`
- `🚪️main-axis-direction` → `➡️main-axis-direction`

All 54 reviewed leaf directories and the compactness group have been moved. Exact TypeScript/Rust imports and collection directory entries use the final names; central inference membership names the compactness group. All 280 files inside moved leaves are byte-identical to their pre-move hashes. All 413 literal Rust include/path dependencies and 67 declared collection-member directories resolve. The scoped audit contains 393 files, 105 directories, 498 governed entries and zero findings in every category. The parallel GLTF owner separately verified 120 fixture carriers and its scenario/catalog repairs; no full native codec pass is implied by these path/data checks.
