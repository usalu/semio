# Controlled IFC Geometry Payload Plan and Public Scene Audit

Read-only audit of current common geometry TS/Rust serialization and native original-context adapters. No production edits/native execution.

## Proven Current Faults

Actual-source TypeScript execution: complete mesh with finite vertices[(1e308,0,0),(1e308,1,0),(1e308,0,1)], triangle(0,1,2) yields cameraTarget.x=Infinity and cameraPosition.x=Infinity. Existing midpoint adds bounds before halving; native uses same arithmetic. Opposite extreme signs also overflow high-low extent and camera offset. Safe midpoint lo/2+hi/2 fixes the first operation only; robust range/camera admission must handle overflowed extent and offset addition explicitly. Refuse unrepresentable camera/payload with finite diagnostic or normalize display coordinates under an explicit contract; never serialize invalid camera nulls as valid geometry.

Actual TS public GeometryScene with one vertex and triangle(0,1,2) throws TypeError from undefined point indexing. Native meshes_json indexes mesh.vertices[triangle index] directly and will panic for same malformed scene. GeometryScene/GeometryMesh fields are public, so projection-produced validity cannot be assumed by serializer. Validate finite exact coordinates, triangle indices/cardinality, nonzero suitable normal scale and finite final normals/camera before output. Native zero-length normals divide by zero then fall back silently; current JSON writer encodes nonfinite numbers as null, so explicit domain validation is essential.

Current normals/bounds/flattening/full JSON and status issue serialization consume no original caller control. render_context_with_selection and IFC2x3 render_in_context control only projection, then call render_scene outside callback; cancel/fuel during payload generation is unobserved.

## Existing First-Party JSON Infrastructure

Package semio-framework-pack-json, manifest framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml, lib ../../🦀️.rs. IFC4 owned model already depends on it and framework-value/native-owned. Public existing APIs in JSON/🎛️controlled-and-borrowed-bridge/🦀️.rs:

- JsonWriteSource::node_at_path(path)->JsonWriteNode and object_key_at_path(path,index)->borrowed str.
- JsonWriteNode::{Null,Bool,Number,String,NativeString,Array,Object}.
- JsonBorrowedWriteCursor::new,normal_step_demands(source),step(source,maximum_units,NativeEncodeControl,RetainedCloneGrant),normal_step_progress,progress and controlled close support.
- JsonWriteCursor<S> owns a source when needed; borrowed cursor can instead retain source through caller scene/frame.
- to_json_string_controlled(ToValue,NativeEncodeControl) is simpler but first builds controlled DslValue; it does not avoid payload-tree copying.

Use existing borrowed writer rather than inventing escaping, float formatting, byte counting or another JSON tree. It measures output first, charges exact allocation, prints escaped Unicode scalar transitions and exposes actual retained receipts. NativeEncodeControl byte maximum limits cumulative ownership; mesh/instance/status outputs need one shared overall cap plus per-output cap. Its progress observer must consult original cx cancellation/yield and consume original fuel; report cursor receipts through same cx. Do not create an always-accept or independent unlimited operation for serialization.

TypeScript package @semio-tech/framework-pack-json-ts currently exports members and testing fixtures only. Searched pack TS source: no production controlled writer twin found. Explicitly account for this missing common layer; do not claim native borrowed writer exists in TS. A TS generic ordinal JSON writer should live adjacent to common JSON encoding infrastructure with neutral escaping/numeric/control fixtures, then IFC payload module supplies domain adapter. Avoid an IFC-local second JSON implementation.

## Focused Shared Payload Module

Place IFC/📐️geometry/📦️payload (or equivalent domain leaf) beside graph/matrix/Boolean. Schema-first PayloadLimits{work,bytes,vertices,triangles}, controlled outcome/status and neutral vectors. Native geometry payload code belongs to owned model/shared source; render adapter alone depends on StepContext. Domain-neutral tick/progress callback passed from same original render context, and same original remaining budget flows projection→normals→bounds→writer rather than resetting per phase.

Prepare normals one triangle/vertex unit at a time with index validation; accumulate finite bounds and robust scene scale. Borrow original mesh positions and triangle indices via JsonWriteSource ordinal paths, exposing flattened scalar arrays without copying positions/indices into additional Vecs. Retain only normals and small IDs/metadata plus writer state. Expose meshJSON,instanceJSON,camera,status only together after all validation/bytecaps/final callback pass. On any invalid/limited/cancel return both mesh and instance arrays empty and finite neutral camera, preserving diagnostic status; never publish one successful payload while its pair failed. Account/close partial strings/normals under original ownership if truly resumable; one-shot bounded failure must not be mislabeled resume.

Neutral tests should cover empty/noncomplete scenes, exact u64 decimal IDs, Unicode/NUL/escape labels, invalid indices and NaN, camera extremes, normal cancellation per triangle, JSON cancel per scalar/character, cumulative bytecap boundary+1, one failed second output yielding neither output, and standard-independent identical payloads. Independent JSON.parse/serde_json and Three normal/bounds arithmetic can validate outputs without runtime external dependencies.

## Actual Adapter and Mount Inventory

Native IFC4 any/io/geometry render_scene owns current camera+mesh+instance+status output. IFC4 editor and viewer root render handlers call render_selected_in_context with original interaction; ordinary main windows call render. IFC4x3 any editor original render calls the IFC4 render_in_context adapter over its shared actual snapshot; noncontext path calls same render. IFC2x3 standard/io/geometry is separately source-mounted with owned base graph type and is used through main wrappers by base,CV2.0,SAV,COBie editors and viewers (eight original-context root calls). Fix both adapters or extract shared payload logic they both call; no cross-standard snapshot conversion.

TypeScript common geometryPayloads is reexported through standard geometry facades and consumed by eight IFC2x3 main-window twins plus IFC4 standard surfaces. Its public signature currently has no control or byte policy. Retain same source-owned payload type and supply optional explicit caller policy only if consistent with greenfield API design; no legacy compatibility wrapper is required.

Own generated transformed modules used for the two TS repros were removed. No native panic was executed; direct native indexing is source evidence, not a compiled runtime observation.

## Focused Validity Patch Recommendations

For TS public typed scene validate vertex tuple length exactly3 plus every finite scalar, triangle tuple exactly3 plus safe integer bounds and pairwise distinct indices before dereference. Native fixed arrays enforce arity but still require finite scalar/index/distinct checks. Reject nonfinite/zero computed face normals rather than assigning arbitrary positiveZ; likewise detect invalid accumulated normals before encoding. There is no justification for an arbitrary epsilon rejection threshold in serialization; exact nonzero and finite scaled arithmetic is the validity boundary. Per-mesh global normalization may underflow very small faces within a huge mesh; scale triangle edges locally or return an explicit numerical diagnostic instead of manufactured normals.

Empty vertex+triangle mesh is valid. Vertices without triangles or unused vertices require an explicit normal policy; silently replacing all zero normals with positiveZ is a guess. Full manifold validity is a separate Boolean/projection concern; payload validator need not duplicate CSG topology algorithms merely to avoid panic/nonfinite output.

Camera safe midpoint uses lo+(hi−lo)/2 when finite span exists, or halfends to avoid midpointoverflow. The final extent must be finite. Check scalar camera offset multiplication itself:1.5*extent can overflow before adding target, so trying the opposite sign is only sufficient when the offset is representable. Try preferred target+offset then target−offset with finite checks; if neither valid refuse atomically. Finite neutral camera is only an error-state output after both mesh+instance payloads are withheld, never evidence original malformed scene was rendered.

## Fresh Validity Patch Peer Review

Camera helpers now use finite span, stable midpoint, per-axis preferred offset then opposite-sign finite candidate and final finite tuple admission. Unrepresentable span/offset fails closed; direct native camera returns neutral finite camera, and both render adapters must mark limited/clear meshes before publishing. Native meshes_json and instances_json both consult payload_issue before indexing, preventing prior public invalid-index panic by source inspection. TS geometryPayloads computes one issue/status and clears both output arrays atomically. Zero face normal is diagnosed; unused/cancelled vertex normals now remain zero rather than arbitraryZ. Existing serialization original-context control gap is unchanged and is not claimed fixed. Native array map/from_fn test usages have inferable3element target through scene/camera signatures; no obvious new native type error found.

Concrete remaining TS defect executed against actual patched source: triangle[0,,2] is length3 but Array.some skips the hole, and pairwise equality does not reject a single undefined. It reaches normal projection and throws TypeError reading undefined[0], instead of limited+empty paired outputs. Validate all fixed tuple slots explicitly or require dense indices. Reported directly to main. Own transformed repro module cleaned.

## Sparse Tuple Resolution

Fresh source now requires Array.isArray plus exactlength3 and validates explicit [0,1,2] slot values for both vertex and triangle tuples. Sparse single-slot triangle and coordinate holes can no longer bypass iteration; prior reproduced defect is resolved by source inspection. Main reports actual red-before/green-after regression and58geometry groups.
