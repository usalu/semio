# Lowpoly End-to-End Exploration Audit

Source exploration on 2026-11-04. Runtime and artifact executions are separate.

## Entry and Commands

React serve owner: 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts. CLI: repo <dev-package>/📜️script.ts serve lowpoly react prototype. Missing activation is prepared automatically. Dev package: 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript.

Serve: repo run @semio-tech/framework-os-dev:serve-hold -- --serve http://127.0.0.1:6070/ --variant lowpoly

Browser: repo run @semio-tech/framework-os-dev:program-matrix -- --serve http://127.0.0.1:6070/ --locale en --roles editor,viewer --only lowpoly --out <ticket-generated-dir>

Tools: repo run @semio-tech/framework-os-dev:tool-run-matrix -- --serve http://127.0.0.1:6070/ --locale en --only lowpoly --out <ticket-generated-dir>

Repeat matrix for de. General verify e2e reads S_STUDIO_URL or S_OS_PORT, default 6070.

## Ownership

🌎️hub/🧩️compositions/💠️lowpoly/🦀️.rs registers editor and read-only viewer. @semio-tech/lowpoly-plugin:test invokes semio-hub-lowpoly Rust surface laws. @semio-tech/lowpoly-js:test checks fixture example presence and interactive-job inventories against independent Ajv policy oracle. @semio-tech/lowpoly-lowpoly owns TypeScript artifact build/check/test. @semio-tech/lowpoly-lowpoly-rs owns Rust build/check/test/test-snapshot-sqlite.

Editor source owner: ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any. Mounted/unit tests: ✏️editor/🧪️tests. Manifest: ✏️editor/📋️manifest/🦀️.rs.

## User Flows

Manifest declares cube/plane/cylinder/cone/icosphere creation; object/vertex/edge/face selection; rectangle/lasso and additive/subtractive/invertive/range; transform; duplicate/delete; extrude/inset/bevel/loop-cut/subdivide/triangulate/mirror/decimate/flip/merge/dissolve/snap/smooth; UV seams/unwrap; brush/eraser/fill/eyedropper/layers; OBJ/PLY/STL export and mesh import; camera/sun; undo/redo. Panels: artifact/catalogue/inspection/layers. Windows: main/UV. English and German labels exist.

## Risks

Structural tests do not prove browser behavior. Demo-session TypeScript test only asserts asset length exceeds eight bytes. Browser matrix is first gate, then explicit creation/picking/transform/undo/paint/UV/import/export/reload persistence. Framework React renderer has world3d-interaction-check and world3d-modelling-check targets. Program/tool-run harnesses launch Chromium with --use-angle=metal: native Windows/Linux portability needs verification. Matrix needs local-only loaded plugins; joined-hub servers can expose only installable programs and select zero. No lowpoly-specific browser target found.

## Catalog Bootstrap Follow-up

Specific catalog publication wrapper: ✏️s/🧑‍💻dev/🧩️catalog/📣️publication/📜️script.ts generate, or registered @semio-tech/s-dev-composition:generate. Its sibling 🟦️.ts specificCatalogPublicationV1 already combines the S publication JSON with 🌎️hub/📇️catalog/🔣️.json, so the JSON alone is not the full contribution set. Output root: ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated. Pass absolute root through --catalog to dev script or SEMIO_PLUGIN_CATALOG. Neutral registry contributes no applications and absent env reads empty projection. Follow-up boot audit verified with actual path resolution that the four-parent production imports and five-parent test imports are correct; no import-path defect exists. The dynamic activation owner is @semio-tech/framework-os-dev:activate-lowpoly-react-prototype.
