# Semio Material Texture References — 2026-10-03

The existing owned `SemioMaterial` contract now carries optional texture IDs in all five glTF slots: `baseColorTexture`, `metallicRoughnessTexture`, `normalTexture`, `occlusionTexture`, and `emissiveTexture`. Rust owns `Option<String>` fields; sparse diffs own `Option<Option<String>>`, which distinguish unchanged, cleared, and replaced references. No new service, adapter, or runtime dependency was introduced.

## Authored paths

Paths below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`.

- `🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/{🦀️.rs,🟦️.ts,🔣️.json,🛰️.proto,🔗️.graphql}` owns the schema, structured DSL tuple, actual binary words and optional-string presence bytes.
- `🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🔺️diff/{🦀️.rs,🟦️.ts,🔣️.json,🛰️.proto,🔗️.graphql}` owns apply, between, inverse, absorb, and text/binary sparse codecs.
- The snapshot/diff/mutation `📝️text/📖️.grammar.semio` material productions describe the nine-field material tuple; sparse material diffs describe eight fields.
- Snapshot `🛬️native/🦀️.rs` and `🛫️native/🦀️.rs` read/write every optional ID through the existing controlled native codec. Admission includes the five string payloads via `native_fields`.
- Snapshot `🪶️sqlite/{🦀️.rs,🟦️.ts,🗄️.sql}` stores five actual texture foreign keys, projects textures before materials, resolves source IDs on reconstruction, and rejects missing sources. The composed base SQLite SQL mirrors the added columns.
- `🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/📬️preparation/🦀️.rs` copies each optional texture string with the existing retained text cursor and explicitly retires partial ownership.
- `🦀️.rs` adds all five owned optional strings to the existing material retirement declaration.
- Existing glTF import/export serializer paths under `🔺️mesh/🚪️io` map all five bindings through glTF textures to actual image sources. Import follows `texture.source`, including permuted texture/image indices; export checks each owned ID and emits the correct texture index. Missing, out-of-range or unavailable image sources fail explicitly.
- Existing `🎨create-material` refuses unknown texture sources; `🕳️delete-texture` refuses referenced sources. Texture deletion inverse now inserts the removed texture at its original position through the existing `SnapshotPatch`, without removing referenced siblings.
- Snapshot and glTF Rust tests, diff Rust tests, delete-texture mutation tests, the existing mesh mutation TypeScript mirror, and existing SQLite TypeScript tests cover the authored behavior.
- Handcrafted mesh mutation JSON snapshots, diffs and payloads, base apply-mesh assets, and the cube/mutation `.dsl.semio` and `.pack.semio` carrier assets carry the current contract. No runtime support for the prior tuple remains.
- Embedded `✉️base/🧬️schema/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json` material definition mirrors the new properties.

## Neutral fixtures and independent witnesses

`🔺️mesh/🧬️schema/📸️snapshot/🧫️fixtures/🎨️material-textures/🔣️.json` contains three valid one-pixel PNG payloads and deliberately permuted glTF texture/image indices. Its expected five source IDs are consumed by the native glTF/diff laws and independently resolved by glTF Transform's `NodeIO` reader. The existing SQLite neutral fixture declares all five IDs and an independently executed five-way material-to-texture SQL join.

The existing TypeScript mirror reads/writes the actual DSL/pack carrier layouts and validates both committed binary/text assets. Its inverse law restores an unused leading texture while preserving material bindings to sibling textures. Runtime receipts use `[DEBUG]` prefixes.

## Actual validation

- `bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-source --skip-nx-cache`: **85 passed, 0 failed**, including independent SQLite integrity/FK joins, missing-source refusal, explicit clearing, glTF Transform's five permuted source PNG byte checks, authored DSL/pack assets, and position-preserving undo. Final run duration 27.2 seconds.
- Initial source run before the additional carrier/undo law: 84 passed, 0 failed. An intermediate carrier run caught a material tuple count mistake in the TypeScript mirror; it was fixed and the final 85-test run passed.
- `bun nx run @semio-tech/stdio-semio:check --skip-nx-cache`: the initial run exposed a readonly corruption assignment in the new test; corrected. The retry proceeded past that change and failed in the concurrently edited drawing mutation schema at `🖊️drawing/🧬️schema/🧬️mutations/🟦️.ts:131`, `TS1109: Expression expected`. No package-wide TypeScript success is claimed yet.
- Native Semio glTF/diff/controlled codec laws are authored but **not run yet**. Parent owns shared closure recovery; its current os-kernel ValueError migration failures prevent the final native proof. Relevant native filters are `material_texture_refs`, `sqlite_snapshot_semio_mesh_`, and `texture_references_refuse_deletion_`.
- Global schema-catalog regeneration remains with the parent after concurrent schema edits settle.

## Contract limits

The new fields own texture IDs and bytes. Sampler settings, alternate UV sets, normal scale, occlusion strength, emissive factors and material extensions remain outside this material contract. External image URIs require supplied image bytes through a buffer view; unresolved URI payloads now produce an explicit import failure. The serializer's surrounding carrier pipeline remains owned by existing artifact routes.

## Fresh Package Check After Shared Syntax Repair

The parent corrected the unrelated drawing mutation union terminator. A fresh registered `bun nx run @semio-tech/stdio-semio:check --skip-nx-cache` then ran for 59.8 seconds and failed on exactly one remaining diagnostic: drawing snapshot SQLite test `🖊️drawing/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts(26,1644)`, TS2540 assigning a readonly sql property. No texture-field or owned mesh TypeScript diagnostics were emitted. The full package remains unverified until that separate test-source error is corrected and the check passes. Root was notified with the exact source and transcript. Native texture filters remain pending the parent's stable native gate.

## Full Package TypeScript Closure

The registered `bun nx run @semio-tech/stdio-semio:check --skip-nx-cache` finally passed on the current source. Root repaired unrelated drawing SQLite readonly test assumptions and prematurely terminated Drawing/Brep mutation unions; this report does not attribute those edits to the material lane. The complete package typecheck now verifies all five material texture reference fields and the authored owning codecs. The independent 85-case portable codec/mutation/oracle suite remains passed; native codecs are still pending shared dependency compilation.


## Native SQLite and Production Rust Closure

Mutation owner ran the current combined registered Semio native gate with filter `test(sqlite_snapshot_semio_base_controlled_refusals_preserve_owner_categories) | test(sqlite_snapshot_semio_mesh)`: **11/11 passed**, 2362 unrelated tests skipped, Nextest 0.319 seconds/Nx 4m16 including fresh compilation. This covers the controlled base category law and all existing native Mesh SQLite laws, including the five-reference material/texture join, independent Bun SQLite integrity/references and owned DSL/pack restoration. Existing glTF/diff/deletion laws whose names lie outside this filter are still pending and are not included in that receipt. The complete Semio Rust production `:check` also passed under the mutation owner's registered gate. Native test compilation required removing exact compiler-reached stale JSON aliases in base/value/image/audio tests and fixing exhaustive PatchSnapshot fixture rosters, preserving their independent serde assertions.


The focused native texture trio filter (`test(material_texture_refs) | test(texture_references_refuse_deletion)`) was attempted after capacity recovery. It stopped before tests at exactly six fresh IO metadata helper controlled-call errors (`attach_sqlite_snapshot_metadata`, source lines 2196/2198/2205/2208/2210/2214), all typed ValueError propagated through its old String signature. The earlier 84 Store/IO errors no longer appeared in this attempt. Parent owns the remaining helper repair. The texture trio remains unrun, while its SQLite reference laws already have the separate green11 receipt.

### 2026-10-03 Typed SQLite Owner Closure

The fresh registered Flow production gate reached 1,343 Semio errors after compiling the mesh kernel. This lane repaired only Base, Value, Image, Audio, Mesh, Graph, and Flow SQL owners, their Base native bounds, and the two compiler-reached terminal native checkpoints. The concrete projection bodies now live once in inherent `project_sqlite_database` methods returning first-party `ValueError`; the union calls those typed methods across all eighteen subsets. Existing public artifact trait methods convert once at their original String boundary. Reconstruction, native field bounds, numeric float projection traits, relational helpers, and shared value-tree helpers retain typed errors. Arithmetic width/count overflow is WorkLimit; malformed rows, variants, and references are InvalidValue. Existing semantic IO validation converts once at its existing IoError boundary.

The installed Rust Tree-sitter grammar parsed all ten changed production sources without errors. Root owns the single combined production check; this is source coherence, not a claimed compile or runtime pass. The existing neutral native-category law was extended to actual union projection cancellation/row admission, native Bound arithmetic overflow, and Mesh typed reconstruction cancellation, using the already owned language-neutral category cases.

The three focused material native laws retry ended before test execution with 1,322 Semio SQL errors on the earlier source snapshot. This receipt is recorded at `🗑️generated/mesh-output/material-texture-native-trio-retry.log`; no unchanged retry was launched.
