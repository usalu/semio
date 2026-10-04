# Complete Document Vector Producer

The full Draw end-user goal remains active. This stage extracts the existing authored-document preparation, encoded-image trace, and Boolean resolution into one vector producer shared by vector consumers and raster export. It does not activate the producer in the editor canvas.

## Implementation

- The existing preparation schema now defines a complete vector plan and its nested progress. Vector plans accept paths, images, groups, and semantic text, and refuse unresolved Boolean/trace records. The vector node schema reuses the preparation node contract.
- TypeScript `DocumentVectorJob` publishes a complete `DocumentScenePlan` after preparation → trace → Boolean resolution. `prepareDocumentVector` yields between grants, reports actual nested progress, and checks abort after observers, including completion.
- Rust `DocumentVectorJob<'a>` borrows the original snapshot, moves each complete intermediate plan, and moves the final plan into the consumer. No document clone or fake text geometry is introduced.
- Both `DocumentRasterJob` implementations now use that producer, removing their duplicated preparation/trace/Boolean orchestration. Raster still explicitly refuses semantic text because faithful font outlines are unfinished.
- Aggregate work admission includes all three vector stages. Candidates remain private, failures stay sticky, and cancellation prevents publication. This is work-granted computation; it does not yet provide admitted mounted-worker retirement or a scalable canvas resource carrier.
- Ten neutral full-document fixtures cover semantic text, shared image assets, ancestor transforms, hidden ancestors, gradients, Boolean differences, traced holes, and trace operands consumed by a Boolean. Native/TypeScript share exact fixture expectations; three algorithm fixtures also declare independent area/contour laws and authored SVG oracles.

## Executed Verification

1. Test-first TypeScript gate `83918` terminated with the expected missing `prepareDocumentVector` export. Log: `🗑️generated/document-vector-red.log`.
2. Initial implementation gate `41843` ran 239 scene tests; 238 passed and one asynchronous cancellation test exceeded its default timeout. That test now yields in grants of seven, retaining synchronous grant-one coverage.
3. Full TypeScript gate `60511`: 550 tests passed; strict checking, independent SVG/PDF checks, 69 Ajv field-patch cases, React Scheduler yielding, and publication authority checks completed. Log: `🗑️generated/document-vector-full-ts.log`.
4. Final schema and stage-failure TypeScript gate `91850`: **551 passed, zero failed, 1,717,396 assertions across 39 files**, plus the same strict/oracle/publication checks. Log: `🗑️generated/document-vector-full-ts-contract.log`.
5. Previous native gate `51592` terminated in shared ZIP/OPC compilation. Its actual source was subsequently corrected concurrently; no ZIP/OPC files were edited for this stage.
6. Native vector gate `99274` reached runtime: **514 tests executed; 505 passed and 9 failed**. Seven failures relate to the demo/text grammar and two to JSON integer versus float fixture comparisons. Log: `🗑️generated/document-vector-native.log`.
7. The actual native DSL reader and writer require record-valued map entries to have braces. The handcrafted demo image asset and its grammar now follow that syntax. This addresses the example load, DSL/pack round-trip, PDF example, and stroke validation failure chain; the repaired runtime gate confirms all of those tests now pass.
8. Native fixture comparisons now normalize only JSON number representations into binary64 while keeping every field and array entry. No geometry tolerance or ignored field is added.
9. Native repair gate **49514 terminated successfully: 514 tests passed, zero failed, zero skipped**. Native compilation completed and all Draw runtime tests ran. Log: `🗑️generated/document-vector-native-fixed.log`. The complete native gate took 7m 55s; assertions took 8.741s.
10. Final scoped whitespace validation passed on the preparation directory, demo asset, and its grammar. Concurrent changes in the original preparation fixture were preserved and are not attributed to this stage.

## Files Authored

- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🟦️.ts
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧬️schema/🔣️.json
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧫️fixtures/🎬️vector/🔣️.json
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🟦️.ts
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/📖️.grammar.semio

## Remaining End-User Integration

The editor canvas still invokes `flatten_drawing_document_with_transformation`, which performs synchronous ad hoc algorithm work. The new producer is available but not mounted there.

The existing host provides genuine snapshot-read ownership, mounted-job factory registration, generation/revision/instance context, maintenance/close hooks, cancellation effects, and immutable visual borrowing. FEM2D demonstrates those hooks; its domain types are not a Draw implementation.

Draw still needs a worker that owns a genuine immutable snapshot, safely drives its borrowed producer under host grants, refuses stale revisions, publishes a complete reusable vector view, and retires admitted owners under bounded close. Render, picking, framing, selection bounds and conversion commands must consume that same complete revision. Gesture previews must preserve authored source geometry and commit one logical undo step.

The current shared Canvas2d snapshot carrier has only four 4096-byte pages per snapshot. It cannot publish arbitrary encoded images or large scenes; scalable first-party resource admission is required. Semantic text can continue through the real canvas text renderer, while portable text raster/export needs actual fonts, shaping and glyph outlines.

Browser interaction, accessibility, localization, customization, multiuser reconciliation, import/export fidelity, image editing, and full end-user acceptance remain unfinished. The goal and ticket remain open; repo MCP lifecycle tools are unavailable in this session.

