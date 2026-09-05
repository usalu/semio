# Semio Artifact Emoji Repair

## Scope

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`, reviewed as 19 independent semantic subsets rather than one automatic rename batch.

## Baseline

- Whole-artifact audit: 5,591 files, 3,732 directories, 9,262 governed nodes.
- Findings: 1,309 missing identities, 2 generic identities, 38 presentation errors, 342 sibling duplicates, no multiple-emoji or reserved-name findings.
- The subset roots all currently use the repeated generic `✳️` identity.
- Current per-subset finding census identifies `mesh` as the largest bounded subtree with 491 findings, followed by `brep` with 393. Every remaining subset has fewer than 105 findings.

## Working order

1. Inspect and repair `mesh` case by case, including its subset root and exact consumers.
2. Re-audit and verify `mesh` independently before entering `brep`.
3. Continue remaining subsets in descending finding count while preserving their wire IDs and payload bytes.

## Mesh decisions

- Subset root: `✳️mesh` → `🔺️mesh`, identifying triangular surface meshes.
- Generator: `🏭️generator` → `🏗️generator`; its families become `🧮️booleans`, `⚠️degenerate`, `🔷️primitives`, `📏️scale`, and `🕸️topology`.
- The Cargo module `generate.rs` becomes `🏗️generate.rs` with an explicit Rust path mount. Conventional `src/lib.rs` remains literal under the central fixed-name contract.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`.
- IO carrier: both stale `🧊️gltf` directories become `🎬️gltf`, preserving `🧊️obj` for Wavefront geometry.
- Mutation fixture directories use the corresponding mutation identities; `set-primitive-material` uses `🧲️` so the canonical `🔗️.graphql` format leaf remains unique.
- Mutation JSON Schema sidecars become `🧬️.schema.json`.
- Corpus bundle leaves use role/format identities: `🅰️operand-a.stl`, `🅱️operand-b.stl`, `🔺️expected.stl`, `🧊️expected.obj`, `🧱️expected.ply`, `🎬️expected.gltf`, `📊️expected.metrics.json`; the mutation example uses `⬅️before.json` and `➡️after.json`.

Each of the 67 flat corpus fixture identities is selected individually and remains distinct among its siblings:

- Booleans: `🙈️boolean-difference-cube-blind-bore`, `🔩️boolean-difference-cube-countersink`, `🛤️boolean-difference-cube-groove`, `1️⃣boolean-difference-cube-single-bore`, `🪟️boolean-difference-cube-slot-through`, `3️⃣boolean-difference-cube-three-bores`, `✖️boolean-difference-cube-two-crossing-bores`, `📦️boolean-difference-nested-cavity-box`, `🫧️boolean-difference-nested-cavity-sphere`, `🌙️boolean-difference-sphere-crescent`, `➖️boolean-difference-torus-minus-cube`, `🤝️boolean-intersection-cube-sphere`, `🛢️boolean-intersection-cylinder-cylinder`, `🔍️boolean-intersection-sphere-sphere-lens`, `🎛️boolean-union-cube-cylinder-boss`, `👆️boolean-union-cube-sphere-tangent-contact`, `↔️boolean-union-disjoint-spheres`, `🫂️boolean-union-overlapping-spheres`, `🎱️boolean-union-tangent-spheres`, and `⛓️boolean-union-three-spheres-chain`.
- Mutation example: `🎨️create-material-applied`.
- Degenerate geometry: `🗺️degenerate-coplanar-faces-union`, `🧵️degenerate-hairline-groove`, `🕸️degenerate-high-tessellation-sphere`, `🔬️degenerate-microscopic-cube`, `🧲️degenerate-near-coincident-union`, `🪡️degenerate-needle-cone`, `🪶️degenerate-sliver-thin-slab`, `🦈️degenerate-thin-fin`, `🤏️degenerate-tiny-bore-below-tolerance`, and `🔘️degenerate-tiny-boss-on-large-plate`.
- Primitives: `📣️primitives-cone-16`, `🌋️primitives-cone-32`, `🚧️primitives-cone-8`, `🧱️primitives-cube-rectangular`, `🎲️primitives-cube-unit`, `🥫️primitives-cylinder-16`, `🧻️primitives-cylinder-32`, `🧯️primitives-cylinder-64`, `🥁️primitives-cylinder-8`, `🌐️primitives-sphere-16`, `🌍️primitives-sphere-32`, `🪐️primitives-sphere-64`, `⚽️primitives-sphere-8`, `💍️primitives-torus-16`, `🍩️primitives-torus-32`, and `🛟️primitives-torus-64`.
- Scale: `📏️scale-bore-boss-1`, `🦠️scale-bore-boss-1e-3`, `🏢️scale-bore-boss-1e3`, `🌌️scale-bore-boss-1e6`, `🛞️scale-torus-1`, `🦟️scale-torus-1e-3`, `🎡️scale-torus-1e3`, and `🛰️scale-torus-1e6`.
- Topology: `⚖️topology-disconnected-mixed-sizes`, `🍡️topology-disconnected-three-components`, `🕶️topology-disconnected-two-components`, `0️⃣topology-genus0-sphere`, `🥯️topology-genus1-single-bore`, `🥽️topology-genus2-two-bores`, `🥨️topology-genus3-three-bores`, `📋️topology-high-aspect-ratio-plate`, `🪄️topology-high-aspect-ratio-rod`, `🎁️topology-thin-shell-box`, `🥚️topology-thin-shell-sphere`, and `🚪️topology-thin-wall-partition`.

## Constraints

- No automatic emoji selection, generated rename plan, bulk path replacement, or compatibility aliases.
- Every move is selected from the node's actual role, checked against its physical siblings, and applied without overwriting.
- Conventional reserved names remain literal.
- Central taxonomy and cross-cutting policy changes are delegated to the root owner after physical paths are final.

## Verification

- The `mesh` cutover moved 67 corpus directories and 375 payload leaves through literal, non-overwriting moves. The semantic fixture IDs and all payload bytes remain unchanged.
- The production generator now contains the 67 reviewed directory identities as a literal authority map plus explicit operand and expected-format filenames. It rejects unregistered recipes, roles, and formats; it does not derive or select emojis.
- A ticket-local generation produced 373 payloads. All 373 compared byte-for-byte with the moved canonical payloads before the canonical generated manifests were refreshed.
- The known `degenerate-microscopic-cube` recipe still reports `Not manifold`; its four emitted carrier files remain preserved and it is not silently added to the 65-entry manifest. This is a pre-existing explicit generator refusal, not a naming failure.
- Both canonical manifest authorities contain 65 fixture records, every declared file path resolves, and `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.semio` reports 65 fixtures and zero file problems.
- The focused path-statute audit reports 741 files, 410 directories, 1,136 governed nodes, and zero missing, generic, presentation, spacing, duplicate, multiple, reserved-name, or oracle findings.
- Conventional Cargo `src`, `src/lib.rs`, and `src/main.rs` remain literal under the exact central Cargo source-directory and entrypoint contracts. The handpicked executable is mounted at `src/🏗️generate.rs`.
- The isolated production bridge resolved its renamed source paths but its first run failed with 745 Rust compile errors. The focused diagnostic reached the renamed mesh sources without any missing-file or unreadable-path error; its first failures are unresolved `crate::artifacts::semio::standards::v1::subsets::base` imports in the mesh snapshot/diff/mutation modules and unresolved `value_derive` macros in the unchanged base geometry module. No native success is claimed, and those broader Rust API/module failures were not bypassed or altered as part of naming repair.
- The final focused audit again reports 741 files, 410 directories, 1,136 governed nodes, and zero findings in every category. The canonical fixture verifier again reports 65 fixtures and zero file problems. The mesh oracle, canonical fixture index, base oracle, and glTF oracle JSON authorities all parse successfully.
- The 373-file ticket-local proof generation and both Cargo `target` directories created by the isolated generator/bridge qualification were removed after verification; no generated build cache remains inside the repaired mesh source tree.

## B-rep decisions

- Subset root: `✳️brep` → `🧊️brep`, identifying the solid boundary representation rather than repeating the generic subset marker.
- Generator: `🏭️generator` → `🏗️generator`; its families become `🧮️booleans`, `🔄️geometry-replace`, `📍️move-vertex`, `🧱️topology-build`, and `🧹️topology-remove`.
- The Cargo module `generate.rs` becomes `🏗️generate.rs` with an explicit Rust path mount. Conventional `src/lib.rs` remains literal.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`; `🏷classification` becomes `🏷️classification`.
- The B-rep `create-edge` mutation becomes `🖇️create-edge`, distinguishing topology linkage from its sibling GraphQL format leaf `🔗️.graphql`.
- All 13 mutation JSON Schema sidecars become `🧬️.schema.json`; the 13 descriptors and oracle mutation manifests retain their semantic IDs and point at those exact schema files.
- Corpus bundle leaves use role/format identities: `🅰️operand-a.step`, `🅱️operand-b.step`, `📐️expected.step`, `🔺️expected.mesh.json`, and `📊️expected.metrics.json`; the mutation example uses `⬅️before.json` and `➡️after.json`.

Each of the 72 generated corpus fixture identities is selected individually and remains distinct among its siblings:

- Booleans: `🏢️booleans-coincident-face-stack-fuse-large`, `🧱️booleans-coincident-face-stack-fuse-small`, `🫥️booleans-cut-fully-engulfed-empty`, `⛓️booleans-cut-fuse-chain-large`, `🔗️booleans-cut-fuse-chain-small`, `↔️booleans-cut-no-overlap-noop`, `⚖️booleans-disjoint-compound-mixed-scale`, `3️⃣booleans-fuse-cut-intersect-three-step`, `🕳️booleans-intersect-non-overlap-empty`, `🔭️booleans-multistep-complex-scaled-1e4`, `🪆️booleans-nested-void-double-large`, `🫧️booleans-nested-void-double-small`, `🌑️booleans-nested-void-single-large`, `⚫️booleans-nested-void-single-small`, `📐️booleans-non-manifold-corner-touch-boxes`, `🛢️booleans-tangent-cylinders-line-contact`, `🎱️booleans-tangent-spheres-point-contact`, and `🪐️booleans-tangent-spheres-point-contact-large`.
- Geometry replacement: `〰️geometry-replace-curve-arc-to-spline-square`, `🌈️geometry-replace-curve-line-to-arc-square-large`, `🪃️geometry-replace-curve-line-to-arc-square-small`, `➰️geometry-replace-curve-line-to-spline-rectangle`, `🏛️geometry-replace-curve-prism-line-to-arc-large`, `🧊️geometry-replace-curve-prism-line-to-arc-small`, `🪝️geometry-replace-curve-tangent-arc-fillet-like`, `🧻️geometry-replace-surface-cylindrical-strip-to-bspline-large`, `🎗️geometry-replace-surface-cylindrical-strip-to-bspline-small`, `💿️geometry-replace-surface-disk-plane-to-general`, `5️⃣geometry-replace-surface-plane-to-bspline-pentagon`, `🔳️geometry-replace-surface-plane-to-bspline-square-large`, `◼️geometry-replace-surface-plane-to-bspline-square-small`, and `🔺️geometry-replace-surface-plane-to-bspline-triangle`.
- Vertex movement: `🏗️move-vertex-inplane-shift-prism-solid-large`, `📦️move-vertex-inplane-shift-prism-solid-small`, `◻️move-vertex-inplane-shift-quad-large`, `▫️move-vertex-inplane-shift-quad-small`, `🛫️move-vertex-lifts-third-corner-off-base-plane`, `🚀️move-vertex-lifts-third-corner-off-base-plane-large`, `🛑️move-vertex-makes-pentagon-nonplanar-rejected`, `🚫️move-vertex-makes-quad-nonplanar-rejected`, `🎀️move-vertex-self-intersect-bowtie-rejected`, and `🦋️move-vertex-self-intersect-bowtie-rejected-large`.
- Topology build: `🎸️topology-build-create-edge-chord-across-disk`, `📏️topology-build-create-edge-diagonal-across-square-large`, `✏️topology-build-create-edge-diagonal-across-square-small`, `🧢️topology-build-create-face-caps-open-cylinder`, `🎁️topology-build-create-face-closes-open-box-large`, `🗃️topology-build-create-face-closes-open-box-small`, `🐚️topology-build-create-shell-from-cylinder-faces`, `🔄️topology-build-create-shell-second-shell-flipped-sense`, `🔃️topology-build-create-shell-second-shell-flipped-sense-large`, `🏘️topology-build-create-solid-second-disjoint-solid-large`, `🏠️topology-build-create-solid-second-disjoint-solid-small`, `⭕️topology-build-create-solid-void-boundary-as-solid`, `🏔️topology-build-create-vertex-apex-above-box-large`, `⛰️topology-build-create-vertex-apex-above-box-small`, and `🎳️topology-build-create-vertex-three-loose-points`.
- Topology remove: `🛡️topology-remove-delete-edge-boundary-edge-rejected`, `✂️topology-remove-delete-edge-loose-diagonal-large`, `🪓️topology-remove-delete-edge-loose-diagonal-small`, `🎭️topology-remove-delete-face-from-redundant-shell-large`, `🎟️topology-remove-delete-face-from-redundant-shell-small`, `🔒️topology-remove-delete-face-still-bounding-closed-shell-rejected`, `🥚️topology-remove-delete-shell-only-shell-of-solid-rejected`, `🦪️topology-remove-delete-shell-redundant-shell-large`, `🐌️topology-remove-delete-shell-redundant-shell-small`, `1️⃣topology-remove-delete-solid-one-of-three`, `2️⃣topology-remove-delete-solid-second-disjoint-solid-large`, `✌️topology-remove-delete-solid-second-disjoint-solid-small`, `💥️topology-remove-delete-vertex-corner-cascade-rejected`, `📍️topology-remove-delete-vertex-loose-apex-large`, and `📌️topology-remove-delete-vertex-loose-apex-small`.
- Separate mutation fixture: `➕️create-vertex-applied`.

## B-rep verification

- Baseline: 629 files, 349 directories, 964 governed nodes; 368 missing identities, 1 presentation error, and 22 sibling duplicates.
- The cutover moved 73 fixture directories and 294 payload leaves through literal, non-overwriting moves. The order-independent aggregate SHA-256 before and after those moves is `0b10202966472d3aa0131c00d52fb6eb233a38da008dc6d87bc65ba74184db3c`.
- The production generator contains the 72 reviewed fixture directory identities and five reviewed carrier filenames as literal authorities. It rejects unregistered recipe IDs and operand roles; it never derives or selects an emoji.
- A ticket-local proof generation produced 292 files and reproduced all 72 fixtures. Its aggregate payload SHA-256, `d52550c3112bfb0a465b5f31dea7f5629590ebc519444cbcb108f728b483190f`, exactly matched the canonical 292-file generated corpus before canonical regeneration.
- Canonical generation then completed 72/72 fixtures with 72/72 byte-identical second passes. It refreshed only the generated file coordinates and generator command inside the oracle manifests, preserving their independent tolerance profiles.
- The oracle and canonical fixture index each declare 72 fixtures and 292 files; every declared path resolves. All 13 mutation descriptor `payloadSchema` targets resolve, and all B-rep JSON authorities parse successfully.
- The canonical fixture verifier reports 137 Semio fixtures and zero file problems across the repaired mesh and B-rep subsets.
- The final B-rep statute audit reports 629 files, 349 directories, 964 governed nodes, and zero missing, generic, presentation, spacing, duplicate, multiple, reserved-name, or oracle findings.
- The isolated native bridge reached the renamed B-rep sources without a missing-file error, then failed with 698 pre-existing Rust API/module errors. The first failures are unresolved `subsets::base` and `schema::engine` imports plus unresolved `value_derive` derives in the unchanged base geometry module. No native success is claimed, and none of those broader API problems was bypassed.

## Document decisions

- Subset root: `✳️document` → `📑️document`, identifying a multi-part document and remaining distinct from every sibling subset.
- Generator: `🏭️generator` → `🏗️generator`; the reviewed families are `🧱️blocks`, `📜️document`, `🖋️runs`, and `🎨️styles`.
- Generator Rust leaves become `🏗️generate.rs` and `📖️reader.rs`; Cargo mounts both exact names while conventional `src/lib.rs` remains literal.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`; the example becomes `🗒️memo`.
- The `set-style-based-on` mutation becomes `🧬️set-style-based-on`, distinguishing style inheritance from the sibling GraphQL format leaf. `🏷set-style-name` receives the required presentation selector as `🏷️set-style-name`.
- Both mutation-test fixture collections and canonical corpus bundles reuse the already reviewed mutation meanings: `🧱️insert-block`, `📸️insert-image`, `🧶️insert-style`, `⏸️no-mutation`, `🪓️remove-block`, `🪦️remove-image`, `🪥️remove-style`, `📦️set-block-content`, `📐️set-heading-level`, `📷️set-image-block`, `🧮️set-image-bytes`, `🔢️set-list-ordered`, `🪶️set-paragraph-style`, `🎨️set-run-style`, `🧵️set-run-text`, `🟤️set-snapshot`, `🧬️set-style-based-on`, and `🏷️set-style-name`.
- Corpus leaves distinguish role and carrier without sibling collisions: DOCX uses `⬅️before.docx`, `➡️after.docx`, and `⚠️counterexample.docx`; Markdown uses `⏮️before.md`, `⏭️after.md`, and `🚫️counterexample.md`; JSON uses `⬅️before.json` and `➡️after.json`.

## Document verification

- Baseline: 272 files, 190 directories, 451 governed nodes; 76 missing identities, 1 generic identity, 1 presentation error, and 24 sibling duplicates.
- The cutover comprised 106 literal, non-overwriting moves: the subset and structural roots, 18 mutation-test fixture directories, 18 canonical fixture directories, and 56 canonical carrier files.
- The production generator contains the 18 reviewed directory identities and eight reviewed carrier filenames as literal authority. The Rust carrier generator has an exact three-kind directory match. Unknown recipes, variants, carriers, or JSON kinds are rejected; neither generator derives an emoji.
- A ticket-local proof regenerated 50 third-party carrier files. Every file matched the moved canonical payload byte-for-byte; both trees have the aggregate SHA-256 `25cc4f1674ee1c56ff9766d762eb9b4db9a3ff9bf7e43e528a9483c8ebe229b1`.
- The complete 56-file corpus, including the three JSON carrier pairs, retained its pre-move aggregate SHA-256 `c0cc345b2027a5f7dcc80586646f0feb9d1c1de555c077907fb83c087301a82a` after canonical regeneration.
- The canonical index and oracle each declare 27 fixtures and 56 files. Every declared file resolves, no path contains an old unprefixed fixture directory or carrier filename, and all JSON authorities parse.
- The canonical Semio fixture verifier reports 164 fixtures and zero file problems.
- The final document statute audit reports 272 files, 190 directories, 451 governed nodes, and zero findings in every category.

## CAD decisions

- Subset root: `✳️cad` → `📐️cad`, identifying drafting geometry.
- Generator: `🏭️generator` → `🏗️generator`; its reviewed families become `🧮️booleans`, `🖊️dxf-entities`, `⭕️step-line-circle`, `🕸️topology`, and `📏️scale`.
- The JSON generator entrypoint becomes `src/🏗️generate.rs` through an exact Cargo target. Conventional `src/lib.rs` remains literal.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`.
- Import/export carrier roots distinguish DXF transfer (`🔄️dxf`), DWG drafting (`🖊️dwg`), and STEP geometry (`📐️step`).
- Mutation directories receive the missing presentation selector without changing their semantic IDs: `✂️remove-block-entity`, `🎚️set-layer`, `🏌️set-block-entity-layer`, `🏳️set-entity-layer`, `🗂️add-layer`, and `🗑️remove-entity`.
- The 16 mutation-test directories use their operation meanings: `🧱️add-block`, `🧩️add-block-entity`, `🔷️add-entity`, `🗂️add-layer`, `⏸️no-mutation`, `🚫️remove-block`, `✂️remove-block-entity`, `🗑️remove-entity`, `🧹️remove-layer`, `📍️set-block-base-point`, `🔺️set-block-entity-geometry`, `🏌️set-block-entity-layer`, `📐️set-entity-geometry`, `🏳️set-entity-layer`, `🎚️set-layer`, and `🟤️set-snapshot`.
- The 22 canonical fixture directories are handpicked from each drawing change: `🚪️add-block-door`, `🌀️add-block-entity-door-swing`, `🌙️add-entity-arc-fillet`, `🗂️add-layer-applied`, `🫥️add-layer-hidden-services`, `⏸️no-mutation-identity`, `🪟️remove-block-entity-window-mullion`, `🏚️remove-block-window`, `〰️remove-entity-middle-polyline`, `🧹️remove-layer-scratch`, `📍️set-block-base-point-door`, `🔲️set-block-entity-geometry-window-pane`, `🍃️set-block-entity-layer-door-leaf`, `⭕️set-entity-geometry-circle-radius`, `🏷️set-entity-layer-text-to-annotations`, `🎨️set-layer-walls-color`, `🖼️set-snapshot-replaces-drawing`, `➕️step-add-entity-circle`, `🪞️step-no-mutation-identity`, `➖️step-remove-entity-line`, `📏️step-set-entity-geometry-circle-radius`, and `🔄️step-set-snapshot-replaces-entities`.
- Fixture roles are exact: `⬅️before.*`, `➡️after.*`, and `⚠️counterexample-after.*`.

## CAD verification

- The Rust probe and JSON generators contain explicit reviewed recipe-to-directory and kind-to-directory maps; neither derives or selects an emoji.
- Ticket-local generation and canonical regeneration produced the same 46 fixture files. The path-and-byte SHA list is `8c5de214f8d24cb2c3c1290c05b59cdf5a96d8f59ca82e049fd71de41206d2a5`.
- The canonical verifier reports 21 CAD fixtures and zero file problems.
- The final CAD statute audit reports 253 files, 182 directories, 422 governed nodes, and zero findings in every category.

## Base decisions

- Subset root: `✳️base` → `✉️base`, identifying the Semio envelope union rather than repeating a generic subset marker.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`; the document example becomes `🗒️memo`.
- Seven mutation directories receive their missing presentation selectors: `🎞️apply-animation`, `🏛️apply-model`, `📽️apply-presentation`, `🕸️apply-mesh`, `🖊️apply-drawing`, `🖼️apply-image`, and `🗂️apply-table`.
- The 18 envelope fixture directories preserve their semantic IDs while identifying the wrapped arm: `🎞️apply-animation-applied`, `🔊️apply-audio-applied`, `🧊️apply-brep-applied`, `📐️apply-cad-applied`, `📑️apply-document-applied`, `🖊️apply-drawing-applied`, `🌊️apply-flow-applied`, `🕸️apply-graph-applied`, `🖼️apply-image-applied`, `🧰️apply-kit-applied`, `🔺️apply-mesh-applied`, `🏛️apply-model-applied`, `📦️apply-object-applied`, `📽️apply-presentation-applied`, `🗂️apply-table-applied`, `🔤️apply-text-applied`, `🔢️apply-value-applied`, and `🎬️apply-video-applied`.
- Every two-file fixture uses `⬅️before.json` and `➡️after.json`. Manifest IDs and wire subset `base` remain unchanged.

## Base verification

- The physical cutover used literal, non-overwriting moves and exact manifest/Rust mount edits. All 26 Base mounts sampled from the Stdio Rust barrel resolve.
- The fixture content-hash multiset is unchanged before and after the 54 fixture moves: `74602eb516c7c288eb0981f87bfcf6804872757408fd500cde05b6db31877c3c`.
- The canonical verifier reports 18 Base fixtures and zero file problems.
- The final Base statute audit reports 274 files, 147 directories, 421 governed nodes, and zero findings in every category.

## Drawing decisions

- Subset root: `✳️drawing` → `🖊️drawing`, identifying vector drawing content and remaining unique among the Semio subset siblings.
- Generator: `🏭️generator` → `🏗️generator`; its two independent engines become `🔣️json-engine` and `🖼️svg-engine`. The JSON entrypoint becomes `src/🏗️generate.rs` through an exact Cargo target while conventional `src/lib.rs` remains literal.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`; the flattened-scene inference receives the missing selector as `🎛️flattened-scene`.
- The two DXF carrier roots become `🔄️dxf`, distinguishing interchange from the sibling DWG drawing carrier `🖊️dwg`.
- All 17 mutation JSON Schema sidecars become `🧬️.schema.json`; the descriptors and oracle mutation manifests retain their semantic IDs and point to those exact files.
- Mutation-test and canonical fixture directories use the reviewed operation meanings: `🖌️change-stroke-color`, `📐️change-stroke-width`, `🌱️create-layer`, `➕️create-node`, `🗑️delete-layer`, `➖️delete-node`, `🖐️drag-nodes`, `🫓️flatten-node`, `🧷️group-nodes`, `📍️move-node`, `🔀️reorder-nodes`, `🪣️replace-fill`, `🛤️replace-path`, `🔄️rotate-node`, `📏️scale-node`, `🎈️unflatten-node`, and `💫️ungroup-node`; the separate applied-layer JSON fixture is `🗂️create-layer-applied`.
- SVG and JSON fixture pairs use exact roles: `⬅️before.*` and `➡️after.*`. The generators contain those directory and carrier identities as explicit literal maps and reject unregistered fixture kinds; they do not derive or select emoji.

## Drawing verification

- Baseline: 391 files, 343 directories, 721 governed nodes; 55 missing identities, 1 presentation error, and 39 sibling duplicates.
- The SVG and JSON engines generated 36 files into a ticket-local proof tree. All 36 matched their moved canonical counterparts byte-for-byte, with aggregate SHA-256 `3e42d01ecfc66177dd63ba959f3ddc448c89c629c94f80644a9f22dc3795a25b` on both sides.
- The independent mutation-test fixture payload digest remains `0036a299937fc86183fa429a5885e549c9fc1971cdb37d73dd22cb1351fa9ddd`; the 17 schema sidecars retain aggregate digest `e5b9fecec9c2e8b5fd6fa85c44ea96183f76629ba9633eda8d1a374e619a32fe`.
- Canonical manifest generation registered all 17 explicit fixture directories. The Stdio Rust barrel has 80 exact Drawing mounts after the cutover, and every mount resolves.
- The canonical fixture verifier reports 17 Drawing fixtures and zero file problems.
- The final Drawing statute audit reports 391 files, 343 directories, 721 governed nodes, and zero findings in every category.

## Presentation decisions

- Subset root: `✳️presentation` → `📽️presentation`, identifying a slide deck and remaining unique among Semio subset siblings.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`.
- Two generic mutation identities become semantic: `🟤️set-snapshot` → `📸️set-snapshot` and `🧊set-text-box-blocks` → `✍️set-text-box-blocks`. Their descriptor owners, declared emoji, Rust mounts, and fixture comments use the same identities.
- The 13 canonical JSON fixtures reuse the matching operation identities: `🧩️insert-layout`, `🎓️insert-master`, `🔷️insert-shape`, `🎬️insert-slide`, `🪃️remove-layout`, `🪄️remove-master`, `🔶️remove-shape`, `🪒️remove-slide`, `🔧️set-layout-master`, `🪟️set-shape-frame`, `🧭️set-slide-layout`, `🧾️set-slide-notes`, and `📝️set-text-box-blocks`. Every pair is `⬅️before.json` and `➡️after.json`.
- The mutation test corpus contained two different physical fixture sets per semantic operation. The three-file specification vectors use the same 15 primary operation identities as the schema. The mutation-only real-deck vectors use distinct, handpicked sibling identities (`📐️`, `👑️`, `➕️`, `🆕️`, `🪞️`, `🗑️`, `👋️`, `➖️`, `⏏️`, `🔗️`, `🖼️`, `🧱️`, `🗒️`, `🔄️`, and `✍️`) rather than repeating a generic fixture emoji. The real talk source becomes `🎙️talk`.

## Presentation verification

- Baseline: 261 files, 211 directories, 472 governed nodes; 54 missing identities and 17 sibling duplicates.
- The cutover used 76 literal, non-overwriting moves: 26 canonical role leaves, 13 canonical fixture directories, 31 test-corpus directories, two option directories, the oracle, two semantic mutation directories, and the subset root.
- The 26 canonical JSON payloads retain the same order-independent aggregate SHA-256 before and after the moves: `5f2b0d82d4c0031a5a90349d273095e8ce6cd2e832df27bd627670d5a0e1e6f2`.
- The oracle declares 13 fixtures and all 26 declared file paths resolve. All 23 exact Presentation mounts in the Stdio Rust barrel resolve.
- The canonical fixture verifier reports 13 Presentation fixtures and zero file problems.
- The final Presentation statute audit reports 261 files, 211 directories, 472 governed nodes, and zero findings in every category. The global non-ticket stale scan is zero for the old subset, oracle, and corrected mutation identities.

## Flow decisions

- Subset root: `✳️flow` → `🌊️flow`, identifying node/edge flow topology and remaining unique among Semio subset siblings.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options` while their sibling configuration directories retain `🎚️config`.
- The production mutation owners are individually semantic and sibling-unique: `📸️set-snapshot`, `➕️insert-node`, `🗑️remove-node`, `🏷️set-node-kind`, `🔤️set-node-label`, `📍️set-node-position`, `🎛️set-node-param`, `🧹️remove-node-param`, `🌉️insert-edge`, `✂️remove-edge`, `🔌️set-edge-endpoints`, and `🎨️set-edge-kind`. In particular, `🌉️insert-edge` avoids colliding with the sibling GraphQL link identity, and `➕️`/`🗑️` replace generic colored node markers.
- The set-snapshot JSON Schema sidecar becomes `🧬️.schema.json`; its descriptor and mutation manifest reference that exact identity. The retained fixture scenario is exactly `🔤️relabels-and-repositions-the-transform-node`.
- The mutation-test corpus uses the corresponding operation identities plus `⏸️no-mutation`. Its two real Nakagin carriers distinguish the textual DSL as `📝️nakagin-capsule-tower.dsl.semio` and binary pack as `📦️nakagin-capsule-tower.pack.semio`.
- The 11 canonical fixture directories use the matching operation meanings, retain their semantic `*-applied` IDs, and distinguish roles as `⬅️before.json` and `➡️after.json`.

## Flow verification

- Baseline: 208 files, 146 directories, 354 governed nodes; 33 missing identities, 4 presentation-selector errors, and 18 sibling duplicates.
- The cutover used literal, non-overwriting moves for the subset root, 12 mutation owners, one schema sidecar, two option directories, the oracle, 15 mutation-test corpus siblings, 11 canonical fixture directories, and 22 canonical role leaves.
- The 22 canonical JSON payloads retain the same order-independent aggregate SHA-256 before and after the moves: `960cf33e6f21a289090e157e70baa5f31700eaa511e6bc43f7ab317a832f26ae`.
- The oracle JSON parses, all 22 declared canonical fixture paths resolve, every mutation descriptor owner resolves, all Flow paths in the shared Stdio Rust barrel resolve, and every exact production mutation mount resolves.
- The canonical fixture verifier reports 11 Flow fixtures and zero file problems.
- The final Flow statute audit reports 208 files, 146 directories, 354 governed nodes, and zero findings in every category.

## Animation decisions

- Subset root: `✳️animation` → `🎞️animation`, identifying a timeline/clip animation and remaining unique among Semio subset siblings.
- Oracle: `🧪️oracle` → `🔮️oracle`; both editor/viewer option directories become `☑️options`, while configuration directories retain their distinct `🎚️config` identity.
- Three production mutation owners were repaired individually: generic `🟤️set-snapshot` becomes `📸️set-snapshot`, and the missing presentation selectors on `set-timeline-name` and `remove-channel` become `🏷️set-timeline-name` and `🗑️remove-channel`. The duration inference receives its missing selector as `⏱️duration`.
- The retained set-snapshot scenario is `🌀️steps-the-spin-channel-and-appends-a-keyframe`, identifying the spin-channel change instead of retaining a generic colored-square marker.
- The mutation-test corpus uses the operation meanings `📻insert-channel`, `🔑insert-keyframe`, `🎬insert-timeline`, `⏸️no-mutation`, `🗑️remove-channel`, `🔓remove-keyframe`, `🧹remove-timeline`, `📈set-channel-interpolation`, `🎯set-channel-target`, `🕐set-keyframe-time`, `🔢set-keyframe-value`, `📸️set-snapshot`, and `🏷️set-timeline-name`.
- The 11 canonical applied-fixture directories use those same operation meanings and every pair distinguishes roles as `⬅️before.json` and `➡️after.json`.

## Animation verification

- Baseline: 208 files, 154 directories, 362 governed nodes; 33 missing identities, 3 presentation-selector errors, and 15 sibling duplicates.
- The cutover used 55 literal, non-overwriting moves: the subset root, three production mutations, one scenario, one inference, two option directories, the oracle, 13 mutation-test fixture directories, 11 canonical fixture directories, and 22 canonical role leaves.
- The 22 canonical JSON payloads retain the same order-independent aggregate SHA-256 before and after the moves: `aab5a27b77cb565146a0b1fe25dff58169f457732614fb50a3ef2075bfa67d2d`.
- The oracle JSON parses, all 22 declared canonical fixture paths resolve, all 25 exact Animation mounts in the shared Stdio Rust barrel resolve, and no stale Animation-owned coordinate remains.
- The canonical fixture verifier reports 11 Animation fixtures and zero file problems.
- The final Animation statute audit reports 208 files, 154 directories, 362 governed nodes, and zero findings in every category.
- Separate source-contract debt remains explicit: 12 mutation descriptors declare absent `🔣️.schema.json` payload schemas. The naming repair did not invent replacement schemas or weaken validation; this does not affect the successful canonical fixture-file verifier result above.
