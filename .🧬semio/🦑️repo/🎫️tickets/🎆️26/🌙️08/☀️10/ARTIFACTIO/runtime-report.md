# Artifact Runtime IO Extraction

Runtime extraction is implemented across plugin artifact roots, but native Rust compile closure is still running. This report deliberately does not claim the complete Rust graph compiles. Parent owns the ticket and final repository checks.

## Ownership

Semantic schema keeps decoded types, validation, mutation application, inversion and pure domain analysis. IO owns OpText/OpBinary, ArtifactDsl/ArtifactPack, explicit JSON readers/writers, codec mirrors and representation metadata. Artifact host modules own publication, replay, retained stores and worker admission. Codec trait ownership is coordinated by the parent in replication IO; this task reuses those traits without adding compatibility aliases.

The Rust lexical scanner masks comments and string literals before selecting top-level items. It extracts explicit codec trait implementations, COMPONENT_GRAMMAR/PROTOCOL constants, codec-shaped helper functions and functions whose bodies call ArtifactDsl/ArtifactPack or first-party JSON decode/encode. It uses artifact module mounts to resolve semantic ownership before rewriting callers; global name-only matching was rejected and corrected to stay inside the same artifact or matching Cargo crate. Diff helper extraction is owned by the separate Diff agent and excluded from consumer rewrites here.

## Completed Changes

- Initial runtime extraction covered 221 semantic roots and 548 codec seeds. Subsequent native/body passes covered 54 roots/156 seeds, 80 roots/217 seeds, 28 roots/50 seeds and a final 12 deep codec functions. These figures describe passes, not disjoint file counts.
- Native text and binary implementations now live under `io/{text,binary}/{snapshot,diff,mutations,inferences}`. Private codec record types and helper implementations moved with the representations; public semantic types remain schema. 86 incorrectly placed byte helpers were corrected from text to binary.
- Generation3d publication/replay/retained authority moved from its former mutation binary source into `🔨️modules/🏠️host/🦀️.rs`. The binary module owns operation bytes; text owns its operation mirror and conversion. Snapshot DSL examples moved to text snapshot IO; empty decoded snapshot construction remains semantic.
- Trinity Jack wire-runtime authority moved from schema into artifact host, with parse/print and bytes separated into representation owners. Drawing owned snapshot/store lifecycle moved from schema/owned into host/owned. Process3d mounted pack owner and its session constructor live in binary snapshot IO.
- Twelve mixed snapshot pack modules moved out of schema; ArtifactDsl implementations were split into text snapshot IO. Eleven SQLite/native snapshot control directories and five copy directories moved out of schema. A later native adapter pass moved 43 snapshot decoding/encoding directories into SQLite snapshot IO.
- GeoJSON semantic conformance now accepts a neutral `GeoJsonConformanceControl`; SQLite supplies its adapter from IO. RFC8259 number lexeme validation stays semantic. Fourteen language-agnostic number cases and serde_json comparison/cancellation tests were authored.
- Derived IO orchestration moved from 134 artifact schema roots into subset IO. Domain construction/analysis helpers were reviewed: pure WFC bitmap region edits and VDI3805 attribute/number inference helpers were restored semantic.
- Semantic reexport aliases were removed from text/binary codecs, callers redirected to semantic ownership, and 38 duplicate entity IO mounts removed. The final two Puzzle3d and PLY representation aliases were made private imports.
- Five WFC inference services now separate retained job/ToolJobFactory/InferredField lifecycle into artifact host, pure compile/entropy algorithms into schema, and request/result JSON plus incremental chunk encoding into text inference IO. Moved tests retain their original fixture anchors.
- Drawing patches now hold typed transform, fill, stroke, trace parameters and layer values. Fill/stroke replacement wrappers preserve untouched versus explicitly cleared values; nested Option alone was rejected because the existing value-codec test proves it collapses both states. Domain diff constructors moved back from text IO to semantic diff, and apply no longer parses JSON. Rust, TypeScript, GraphQL, Proto, JSON Schema, owned replay and 21 language-agnostic mutation diff fixtures were updated together. TypeScript numeric-carrier admission lives in text diff IO, before strict semantic Binary64 guards.
- Deep Drawing/Layout decode bridges, Energy JSON byte census and further mapped helpers were extracted. Energy census Inference/Default adapters moved with the representation computation, leaving the schema types independent of IO functions.

## Verification

| Check | Actual Result |
| --- | --- |
| `bun nx run @semio-tech/procedural-generation3d:test --excludeTaskDependencies --skip-nx-cache` | Passed; actual TypeScript build/test target completed. |
| Ticket Drawing runtime verifier | Passed typed clear/untouched law and decoded all 21 mutation diff fixture documents through text IO. |
| Drawing text IO unit tests, integrated in existing `@semio-tech/draw-js:test` | Focused IO tests passed. Broad suite completed 616 passing, one skipped, eight failing: six timeouts, paint oracle draft-2020 metadata unavailable, and typed diff schema references. The latter is being fixed with local definitions; broad suite is not passing. |
| `@semio-tech/draw-drawing-rs:test` filtered `typed_patch` | Running through native owner preparation; no completed compiler/test result yet. Serde_json equality and explicit-clear round-trip tests authored. |
| Generation3d Rust first check | Reached compiler; exposed invalid framework alias paths. Parent corrected them. |
| Generation3d Rust second check | Reached compiler; exposed 14 unqualified `io` paths in raw-binary IO declaration. Corrected to canonical artifact paths. |
| JSON Rust neutral number test | Earlier run failed before compilation at Cargo workspace preparation. No claim of passing Rust test. |
| Stdio assembly/WFC bitmap/Trinity Jack Rust broad checks | WFC bitmap reached compiler and failed with 30 errors. Other three projects remain running. Host trait visibility, dangling attribute and canonical IO-path errors were corrected; semantic base64 fields are being converted to intrinsic bytes by the TypeScript execution agent. |
| Rust syntax sweep | First pass found imports prepended before module inner docs in two consumer files; corrected. Second pass running. Syntax alone is not compile verification. |

Compiler/log output remains exclusively under the ticket `🗑️generated` directory while checks are running. Parent will remove generated output at ticket closure.

### Compiler Findings And Repairs

The WFC bitmap compiler reported an unattached `#[allow(clippy::too_many_arguments)]` left after moving `write_region`, 23 visibility qualifiers incorrectly applied to trait implementation methods, five `crate::io` paths without that root module, and two missing base64 functions in semantic snapshot methods. The dangling attribute was removed, trait methods have their required inherited visibility, and paths now resolve through the canonical standard/subset IO owner. The missing base64 calls exposed the underlying semantic bytes issue rather than merely a missing import; the sibling agent is replacing snapshot/mutation/diff/solve data with intrinsic bytes and text IO mirrors.

The expanded body scan reached every mounted non-test schema Rust source and moved another 26 roots/40 codec seeds into 18 representation destinations. CAD SQL row admission now belongs to SQLite snapshot IO while literal ordering and retained value collection remain semantic. Writer completion JSON, VDI native serializers, GIS descriptor JSON and other direct serializer bodies were included. glTF GraphQL/proto/protobuf adapters and retained serde oracle ownership, Semio inference byte-cache bridges, and Puzzle fill checkpoint bytes are subsequent targeted passes in progress.

## Files And Remaining Closure

The source file list and ownership records are retained in `runtime-files.md` and the ticket scripts. No modifying Git commands or worktrees were used.

Known remaining review areas are Lowpoly media conversion JSON adapters reported by the separate semantic agent, NoteBlock's encoded block patch field, and deeper inherent method bodies outside the specifically separated WFC services. Native compile results must resolve privacy, module mount and trait closure; parser success cannot stand in for those results. Other agents own framework artifact roots, shared scalar ownership, TypeScript extraction and semantic Diff/replay separation.

## Runtime Closure Update

The focused Drawing official Nx target completed successfully after fixing self-contained diff schema definitions and the nullable fill contract. The run included three IO tests (Ajv independent JSON contract and DataView numeric oracle), two selection/status tests, and the package check. The language-agnostic clear-style fixture remains distinct from an untouched style.

The Drawing native typed-patch target completed unsuccessfully before reaching Drawing: raw binary IO mutation framing retained two references to removed schema::mutations::COMPONENT_PROTOCOL_SEMIO. The physical metadata route repair updated 94 references to their representation owner. This is a repair, not a passing native rerun.

Further ownership changes completed: CAD SQLite frontier admission and keyed/dense adapters moved to IO while pure ordering/filtering remains semantic; glTF GraphQL/proto JSON decoding and protobuf varint decoding moved to text/binary mutations; glTF retained serde oracle implementations are test-only under text snapshot; Semio Drawing and Mesh cache serializer bridges moved to text inferences; Puzzle3d fill checkpoint fixed-width bytes moved to binary inferences. Bitmap host now consumes intrinsic bytes and uses the shared text binder for physical request/response conversion. The TypeScript agent owns Bitmap schema/mirrors and Forms response export.

Lowpoly's previously reported document JSON helpers are already replaced in the shared checkout by lowpoly_snapshot_from_mesh, mesh_document_value, and mesh_from_document_value. The inspected functions now expose typed snapshots/intrinsic values.

The latest broad Rust check target covers generation3d, DWG, TXT, and Bitmap. It began after the shared graph construction wait; no completion is claimed here. Prior compiler failures exposed DWG relocated SQLite imports and TXT registry/module collisions, repaired with exact canonical semantic/physical imports and registry dispatch paths. XML/SVG snapshot helpers are already physically extracted; their stale schema-qualified caller routes are being closed.
