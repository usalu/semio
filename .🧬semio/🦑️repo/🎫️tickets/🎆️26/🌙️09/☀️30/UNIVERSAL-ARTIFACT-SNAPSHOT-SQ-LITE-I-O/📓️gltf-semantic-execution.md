# GLTF Semantic Snapshot Execution

## Verified Ownership

The artifact definition, actual native document declaration and retained 149-dialect inventory agree on exactly `s.stdio.gltf@2.0/*`. Its snapshot root is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot`. Camera, mesh, material, skin and other physical folders hold mutation facets, not separately declared snapshot dialects; no inferred profiles will be registered.

Native GltfSnapshot owns schema, a full typed GltfDocument, independently retained resolved byte buffers and source form Json/Glb. The document owns asset, optional selected scene, scenes, nodes, meshes, accessors, buffer views, buffer declarations, materials, textures, images, samplers, skins, animations, cameras, ordered extension-used/required names, extensions and extras. It has explicit types for sparse indices/values/accessors, morph targets, primitive attributes, metallic/roughness and three texture-info models, animation sampler/channel/target, both camera projection variants and a local GltfJson value enum.

The local GltfJson owns ordered array values and ordered object member pairs, including duplicate keys. Its Number is f64. Every optional extras/extensions distinguishes absent from present Null. Its declared presence codec already documents this essential state. These are actual typed extension values; their own SQL value tree is appropriate only for these declared slots, never a fallback for GLTF entities.

Current ArtifactDsl and ArtifactPack invoke native JSON/GLB wire conversion, not owned RecordSpecs. They can lose schema, sourceForm, independent resolved buffers and IEEE/intermediate state. The root currently declares a protocol-digest hash and a bare codec, so full logical persistence, actual structural identity and required SQLite capability must be authored together and verified through real declaration-driven erased Binary/Text I/O.

## Authored Domain Plan

| Adjacent SQLite domain | Actual owned semantic tables |
| --- | --- |
| Document | Snapshot identity/source form/scene reference, asset, scenes, ordered scene-node membership, used/required extension names, resolved buffers and ordered intrinsic byte values |
| Node | Node identity/options/references, children, fixed matrix/translation/rotation/scale components and weights |
| Mesh | Meshes, primitives, ordered attributes, morph targets/attributes and weights |
| Buffer/accessor | Buffer declarations, buffer views, typed accessors, optional bound values, sparse accessor/indices/values |
| Material | Materials, metallic/roughness, base-color/emissive/metallic-roughness texture bindings, normal texture and occlusion texture |
| Texture | Texture, image and sampler declarations |
| Skin | Skins and ordered joints |
| Animation | Animations, samplers, channels and independently owned channel targets |
| Camera | Camera declarations and orthographic/perspective components |
| Extras/extensions | Explicit owned JSON value, array element and object member entities, with root ownership through actual extras/extension foreign keys |

All IEEE fields will have individually named query REAL, signed INTEGER exact binary64 word and numeric-class companions, using already verified FloatColumn/FloatRow helpers with literal field positions. All u64/usize source indices, lengths, counts, filter/wrap/mode/texture-coordinate values will use explicit high/low unsigned32 INTEGER words; occurrence ordinals use actual bounded entity order. Required or optional source references retain their numeric source index independently of whether a valid target exists, so typed intermediate native state is not silently coerced or rejected. Actual valid target links can be separately queryable FK columns without replacing original numeric identity.

The current TypeScript snapshot is a finite JSON wire twin rather than the entire owned native state: safe number indices, finite number f64 fields, object maps that lose duplicate member identity, and omissible defaults. The canonical snapshot facet will use bigint, Binary64 words and tagged ordered GltfJson entities with actual adjacent parser/consumer changes, without compatibility unions. Native file parsing remains a separate concern. Shared language-neutral fixtures will explicitly convert finite wire samples at the native/source test boundary and carry unsigned/IEEE word cases for complete domain fidelity.

## Required Verification

Tests first: independent Bun SQLite joins through actual mesh/accessor/material/node relations, edited projection values, complete typed reconstruction, all optional ownership and empty/present-empty distinctions, duplicate extras members, byte buffers differing from declared buffers, u64MAX metadata, all nine binary64 boundary words, malformed companion/ordinal/owner/schema cells, exact wildcard dialect and cancellation before large copying. Every expensive projection/reconstruction/encoding step must preflight checked resource arithmetic and observe bounded cancellation. Physical SQLite and providers remain runtime dependency free. Owner Nx native and public source package gates must compile and exercise actual providers, factories, schema hashes and authored demo assets before completion.

The initial owning registered native command `bun nx run @semio-tech/stdio-gltf-rs:test sqlite_snapshot_ --skip-nx-cache` reached actual compilation and failed with four missing owned SQLite provider methods in 14.7 seconds. The two native laws and shared full typed fixture were authored before provider implementation; one independently queries mesh/primitive/accessor/material joins and edits both a material and a resolved buffer byte. A handwritten 57-table DDL is now authored in ten adjacent domain SQL files. Provider implementation and complete verification are in progress; no passing GLTF runtime result is claimed yet.

All ten native projection/reconstruction domains are now authored and wired. The first provider compile exposed my incorrect shared FloatRow method name and missing SqliteSnapshotError-to-String mapping; those were corrected. The next registered gate reached the concurrently authored intrinsic Bytes prerequisite and stopped on shared DSL base64 module visibility and four shared store/directory exhaustive matches. These are owned by the I/O worker; no passing native provider result is inferred from the prerequisite failure. The GLTF own GltfJson FromValue now explicitly rejects Bytes, because intrinsic octets are not a permitted extras value. New native laws cover independent SQLite reserialization of all nine IEEE words, full unsigned source fields, duplicate ordered extras members, erased Binary/Text fidelity, malformed independent relationships/IEEE cells and bounded admission/cancellation.
