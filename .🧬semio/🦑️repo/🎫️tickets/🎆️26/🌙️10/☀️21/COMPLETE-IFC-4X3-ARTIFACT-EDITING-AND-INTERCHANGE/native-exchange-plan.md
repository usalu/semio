# Native IFC4x3 Exchange Plan

## Exact Scope and Files

Implement the first complete native host geometry interchange route: IFC4X3_ADD2 native snapshot pack ↔ glTF2.0 native snapshot pack, with existing native glTF JSON/GLB codec entry points. Use source-owned IFC/GltfSnapshot types and the shared schema geometry projector, never IFC4's strict STEP ingress. Typed callback control and IoRunControl wrappers must retain cancellation and publication atomicity. Fidelity is explicitly lossy geometry projection; unsupported representations and semantic losses are diagnostics.

Production files owned by this worker:

- New `IFC4x3 any/🚪️io/native_exchange.rs`: native conversions, composer entries, controlled IO adapters.
- New `IFC4x3 any/🚪️io/native_gltf.rs`: graph/mesh and glTF artifact projection, scoped unit/transform admission.
- New `IFC4x3 any/🚪️io/🧪️tests/native_exchange.rs` and `native-exchange.feature`: tests first, official ADD2 triangulated and Khronos Box inputs.
- Minimal module/dependency additions to `IFC4x3 any/📦️packages/🦀️rust/{🦀️.rs,Cargo.toml}`. Preserve concurrent changes, especially geometry dependencies.
- Minimal IFC artifact root `🦀️.rs` composer registration additions coordinated with the coordinator. No root TypeScript façade, editor or MVD changes.

The existing `🚪️io/step.rs` and its entries remain owned by ifc_schema_core. New entries register separately and must use exact source dialects. The glTF dependency is the repository-native `semio-s-artifact-stdio-gltf-model`, whose native pack, JSON/GLB codecs and accessor reader already exist.

## Admission and Fidelity

IFC export validates ADD2 through the dedicated native source/graph helper, calls shared `schema::geometry::project_controlled`, refuses cancelled/limited scenes and empty supported geometry, and preserves projection issues as diagnostics. Vertices from the shared projector are in source units; resolve exactly one project length unit and convert to SI metres, then rotate Z-up to glTF Y-up. Missing/ambiguous units, unsupported unit identities and unsupported georeferencing are refused. Exported GLTF uses finite float32 positions, indexed triangles, product identity extras and explicit losses for all authored/analytical semantics omitted.

glTF import uses the real typed snapshot/GLB reader. Admit static dense float32 triangle positions, integral indices, supported affine hierarchy transforms and explicit metre convention. Refuse required extensions, skins, animations, morphs and unsupported accessors/topology. Emit actual IFC project, metre units, Cartesian point lists, triangulated face sets, shape representations and proxy products. Preserve reflected winding and report missing authored BIM/analysis semantics.

Existing composer APIs have no IoRunControl argument. Pure composer routes use bounded callback-controlled helpers; controlled IO mechanism adapters expose original caller control. This API limitation must remain explicit, not hidden behind a claim of fully owned host cancellation.

## Evidence

Neutral scenarios require route discovery and native artifact-pack composition in both directions, GLB geometry bounds/triangle counts, schema-separated IFC4 refusal, cancellation/limits, explicit unsupported geometry and source immutability. Native Rust compile is currently blocked by existing shared syntax/static-evaluation problems; attempt focused tests, record actual diagnostics and use syntax validation plus existing independent TypeScript/IfcOpenShell/three geometry oracles without claiming Rust execution passed. Expand BIM/FEM native routes only after this route is concrete and reviewable.
