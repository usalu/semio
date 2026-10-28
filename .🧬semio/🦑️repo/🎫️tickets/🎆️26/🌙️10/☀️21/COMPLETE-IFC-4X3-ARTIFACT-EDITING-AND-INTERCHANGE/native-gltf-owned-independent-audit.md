# Independent Original-Owned Native Geometry Audit

## Scope and evidence boundary

Read-only source checkpoint on 2026-10-21. Read native-gltf-owned-plan.md and native-gltf-owned-implementation.md; inspected actual native-gltf-owned.rs, native-gltf-export.rs, native-gltf-import.rs, native-gltf-json.rs, native-gltf-graph.rs, geometry.rs, geometry-project.rs, normal native_exchange.rs registrations, and genuine-output oracle gates. No production changes, compiler bypass, lifecycle changes, or generated-output cleanup were performed. Direct rg/PowerShell read fallback follows the parent session's documented unavailable repo runner/MCP boundary.

This is a limited independent source audit, not full verification of every receiving/writer leaf or framework retirement implementation. Authentic ingress previously refused before rustc; no new execution was attempted. Authored tests, syntax parsing, and skipped third-party gates establish no native type-checking, route execution, receipt conservation, heap peak, drain completion, or native-output correctness.

## Actionable finding

### P2 — Reject one-sided Axis/RefDirection before evaluating selected IFC coordinates

Source: IFC4x3 any/io/native-gltf-export.rs, axis(), lines 124-128 at inspection. Axis and RefDirection are independently defaulted when Unset. IFC4X3_ADD2.exp lines 4177-4189 defines AxisAndRefDirProvision at line 4184: NOT ((EXISTS (Axis)) XOR (EXISTS (RefDirection))). Both must therefore be present or both absent.

For example, an otherwise supported triangle product with an IFCAXIS2PLACEMENT3D(origin, Axis=[0,1,0], RefDirection=$) passes the current axis branch, obtains X=[1,0,0], and may publish transformed glTF geometry. The reverse one-sided case also passes for nonparallel supplied X. This is selected placement admission, rather than unsupported peripheral semantic validation: invalid coordinate authority is actively interpreted and published. Add a paired-presence check before normalization, preserve default/default and valid nonparallel pair acceptance, and assert atomic refusal for each one-sided case in native owned-route tests and the neutral corpus.

The earlier proposed negative-X fallback defect is withdrawn. Axis=[-1,0,0] with omitted RefDirection violates the same paired-provision rule and is not a valid counterexample; normative IfcFirstProjAxis itself explicitly checks positive X. No fallback change is justified by that invalid case.

## Source observations without additional confirmed defects

- Actual normal routes call original_pack serializers/deserializers. The four geometry directions use source-written gltf_owned import/export, with explicit CARRIER_BINARY GlbIntoIfcNative; cold helpers remain separate. Six explicit rows plus the four domain rows give the documented ten routes.
- Geometry-only Pack APIs are named receive_geometry_pack / encode_geometry_pack. The receiver retains a schema-received private RecordValue source and selected target under original owners; the publisher borrows actual GltfSnapshot fields through OriginalPackWriting. Semantic omissions are bounded geometry-loss diagnostics; this does not advertise a general lossless foreign codec.
- Port.effect checks cancellation, admits item/copy/depth frontier, runs bounded work, records actual accepted progress, then advances native work. Vector/string reservations use original allocation frontiers. The record-before-post-step pattern is consistent with retaining accepted-prefix progress on cancellation. Same-recipient retirement and five-axis conservation still require genuine native execution and framework-level review.
- Natural framing checks GLB version/extent/aligned chunks, JSON first, BIN second, duplicate recognized chunks, and retains spans into the original source. JSON catalog is offset metadata, uses two bounded passes, preserves effective last-key lookup, tolerates leading BOM, and admits finite integer-valued Binary64 decimal/exponent values. GLB JSON trailing bytes are checked for literal spaces. Complete BIN/publisher correctness was not independently re-proven in this checkpoint.
- Export classifies products using static metadata::is_subtype(IFCPRODUCT), resolves positive IDs via a funded sorted index, requires a sole project and explicit SI metre authority, scales coordinates, evaluates placement/context chains, handles PnIndex, and derives FLOAT min/max from emitted quantized coordinates. Unsupported mapped/swept/BREP/geographic/void evaluation is explicitly refused and is documented scope, not a new defect.
- Import checks dense FLOAT VEC3 positions, aligned/interleaved buffers, unsigned triangle indices, bounded topology, finite affine authored transforms and exact orientation, with reflected winding correction and Y-up/Z-up conversion. Graph construction writes actual shared IfcSnapshot/IfcValue entities with SI metre project/context and tessellated proxies. These observations do not imply EXPRESS-wide conformance.
- Native oracle tests require SEMIO_IFC_NATIVE_ORACLE_OUTPUT and a genuine original-native-gltf-owned-tests producer marker. Their skipped state is honestly documented. Existing reference/TS success cannot satisfy this gate.

## Required verification before claiming completion

Execute authentic native compilation and tests after the repository ingress is available; confirm the paired-direction regression and existing neutral lexical/product/geometry cases; execute original recipients and five-axis refusal/cancellation/drain assertions; then emit genuine native outputs and run Khronos, three.js, and IfcOpenShell interfaces. Until then this report is a limited source checkpoint with one selected-placement admission finding, not a runtime approval.
