# Generation3d IO Authority Boundary Review

Read-only source review on 2026-09-30. This review changes only this report. The concrete app native run stopped after 121 passing laws at a document-to-mesh path whose fresh evaluator supplies no retained geometry. Runtime implementation and reruns belong to the implementation owner; this review claims no new IO runtime success.

## Observed Boundary

The generation3d IO root is `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`. At lines 90–91, `mesh_bridge::preview_semio_mesh` invokes `crate::editor::generation3d::export_mesh_from_document`. This dependency sends an artifact format conversion upward into its editor component. The non-assembly branch reports that the evaluator feature is unavailable, so even its feature-independent format behavior depends on app assembly.

The editor root at the sibling `✏️editor/🦀️.rs` defines `export_mesh_from_document` at line 3237. It creates a default editor configuration and invokes `schema::with_host(... host.evaluate())`, then calls `preview_payload_from_eval` without a retained `FlowEvalSession`. `generation3d_mesh_from_document` at line 3246 decodes a graph snapshot, calls that helper, and retires the snapshot. The fresh host has no caller-supplied geometry port; the session-free preview cannot read the caller's already-owned mesh packs. This is consistent with the reported actual native empty-mesh failure.

The existing `export_mesh_from_session` at editor line 3232 reads `session.eval_json()` and passes that same session into the preview pipeline. It performs no second evaluation. Product export commands already obtain their owning resolved session and use this retained preview. Moving evaluation into the IO module would duplicate ownership and invert the same boundary again.

## Existing Neutral Contracts

The framework SDK `ArtifactSerializer` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:939` declares `From`, `Into`, their dialects, and `serialize(&From)`. It has no geometry evaluator or renderer context. Its input type can already express a resolved artifact; widening the universal serializer API is unnecessary for this repair.

The generic codec `DecodeContext` and `EncodeContext` in `🧰️framework/🔨️modules/🚪️io/🦀️.rs:379` and `:439` carry a policy, a bounded codec budget, and an optional host-owned `ResourceResolver`. Their source/sink views enforce resource limits and cancellation. That resolver is for payload resources, not procedural evaluation or geometry ownership; it should not conceal a geometry engine.

The generation3d IO root already exposes a pure `MeshData` → `SemioMeshSnapshot` conversion at line 61. It validates positions and indices, and preserves a mesh with no face connectivity as `Points`. Each of the six geometry leaves—STL, OBJ, PLY, glTF, LAS, and DWG—already has `serialize_mesh` and `serialize_mesh_bytes` accepting the first-party `SemioMeshSnapshot`. These delegate to the existing stdio-owned codecs. The snapshot-only `serialize`/`serialize_bytes` wrappers add the broken editor evaluation dependency. Text and structural JSON are document-only conversions and need no geometry authority.

## Recommended Contract

Require an explicit, borrowed resolved `SemioMeshSnapshot` at generation3d geometry export entry points. A whole-document entry may accept the document together with `Option<&SemioMeshSnapshot>` because text is graph-only; absent geometry for a geometry format must produce a named typed error. Remove the document-only geometry fallback and `preview_semio_mesh`'s editor dependency. Preserve the format roster, filename/MIME lookup, base64 policy, download envelope, imports, and six existing stdio codec delegations.

The caller owns evaluation and selection of the retained mesh. Concrete composition binds the supplied producer/session geometry ports, completes the real evaluation, and reads the retained packed mesh. IO consumes that materialized neutral input; it owns no geometry engine, editor configuration, native global fallback, or second session. A document-to-mesh convenience API that evaluates implicitly must be replaced with an explicitly supplied session/capability or moved to the concrete composition owner. A pure mesh/document conversion remains appropriate at the artifact boundary.

The existing registry also needs to state this input honestly. Its six geometry `ComposerEntry` rows at IO lines 621–626 currently declare only `GENERATION3D_DIALECT`, then rebuild a graph snapshot and invoke the snapshot-only leaf. If these geometry registry routes remain, declare and consume the resolved first-party mesh dialect as an additional source. The text route may keep the graph-only source. `ErasedComposeSource` already supports multiple typed artifact sources; no new global callback or generic evaluator context is needed. Any temporary rebuilt graph snapshot must retain its existing explicit retirement discipline, including failures.

## Semantic Proof To Preserve

The affected native composition laws must evaluate the actual procedural graph with the supplied geometry owner and retained `FlowEvalSession`, then invoke the changed generation3d IO entry with that resolved result. They must continue checking the same format bytes, decoding, triangle/connectivity expectations, imported graph semantics, and round-trip behavior. Do not substitute a unit-cube fixture into an authored graph law, skip an unavailable action, call a different codec helper to evade the changed entry, normalize equality, or weaken geometry assertions.

The existing neutral codec fixture tests remain useful for grammar behavior, but they cannot establish graph evaluation and ownership. The concrete app group must rerun after implementation, followed by the retained group and full source/355-law aggregate as coordinated by the parent. Captured packed previews may remain usable under their existing dirty-preview policy; this review proposes no new snapshot-digest policy or additional evaluation.

## Exact Review Output

The only new file owned by this read-only task is this report:

`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔍️2026-09-30-generation3d-io-authority-boundary-review.md`
