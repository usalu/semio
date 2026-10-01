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

## Fresh Public Verification And Ordered Extras

The uncached registered `bun nx run @semio-tech/stdio-gltf:test --skip-nx-cache` fourth run completed successfully in 48.2 seconds. It performed package/public/browser type checks and build, followed by 486 canonical finite-wire corpus laws with 10,702 assertions, one independent 57-table schema law with six assertions, and three semantic provider laws with 38 assertions (490 laws / 10,746 assertions total). This precedes the strengthened word tests below.

The subsequent direct owned Bun suite passed four laws / 44 assertions in 18.74 seconds. The word oracle now assigns every owned binary64 family: all node transforms and weights, mesh weights, accessor minima/maxima, PBR factors, emissive/cutoff, normal/occlusion scalar, both camera variants, and ordered duplicate extras. Node mesh, declared buffer size and sampler filter independently retain unsigned 64-bit maximum. Each case is exported, independently deserialized and serialized with Bun SQLite, reconstructed and reprojected for exact relational equality. Independently edited query REAL values, non-contiguous attribute ordinals and unowned relationships reject.

Inspection of the shared dynamic-value text printer showed that its Object arm sorts member names. A GLTF extras `Shape::Value` would therefore lose the model's explicit ordered object-member domain during erased Text lowering. The mixed-key oracle now includes `z first`, `a second` and duplicate names. GLTF's own logical pack uses explicit typed JSON-kind/primitive/array/member records, with textual member-name fields and ordered member occurrence records; extras remain a legitimate owned domain and no entire document is serialized as a dynamic JSON carrier. This change has not yet been admitted by the registered native compilation; no native pass is claimed.

The second logical native gate failed before compilation because a preparation subprocess's 30-second timeout included FIFO queue waiting. The third logical native gate passed that queue but failed before compilation on stale native receipt authority `stdio.native.bcf.v1`, propagating through the canonical stdio composition owner preparation. Both are prerequisite failures, not GLTF test outcomes. The parent owns queue admission and receipt prerequisites; further immediate retries are held until readiness.

## Final Source Gate For Current Implementation

The progress regression was run red: the dialect guard resolved despite its callback aborting the supplied signal. Adding the shared initial `projectSnapshot` checkpoint corrected this concrete admission seam. The strengthened direct owned suite then passed five laws / 51 assertions in 5.58 seconds. It covers aggregate byte refusal during projection/reconstruction and cancellation during both 600-entity walks at the 256-entity checkpoint.

A fresh uncached registered public gate, `bun nx run @semio-tech/stdio-gltf:test --skip-nx-cache`, passed the current source in one minute one second: 486 canonical corpus laws / 10,702 assertions, one independent DDL law / six assertions, five semantic provider laws / 51 assertions. Total: **492 laws / 10,759 assertions**. The actual public runner confirmed nine runtime exports and three owned suites. This run includes the complete IEEE-field oracle, differently named ordered extras, full unsigned metadata, independent edits/refusals and the repaired guard checkpoint.

## Owned Route And Bounded Extras Wire Shape

GLTF now owns explicit combined/native/source SQLite Nx routes in its Rust package project and existing `📜️script.ts`. Three ordered `launch.json` entries expose these routes, with standards taxonomy, SQLite physical engine and shared artifact runner as cache inputs. The registered source-only route passed seven laws / 63 assertions in 21.7 seconds (independent schema one / six, semantic provider six / 57). The added neutral minimal fixture demonstrates all 57 tables present even when domain collections are empty, and retains `None` versus `Some(empty)` accessor extrema, `Some(Null)` extras and an independently present empty resolved buffer.

Recursive logical extras records would consume shared DSL parser depth for each own JSON nesting level. GLTF's logical extras are therefore represented by explicit flat typed nodes: kind plus mutually exclusive bool/float/text values, ordered child indices and ordered named member occurrences. Projection uses a borrowed queue; reconstruction consumes forward child identities once in reverse order and refuses duplicate ownership, cycles, unrelated variant fields and unowned nodes. No full snapshot is stored in these JSON-domain records. Borrowed document/primitive/camera record lowering avoids recursively cloning extras before encoding. A separate actual-factory erased Binary/Text law retains 128 nested arrays around an exact NaN word without increasing logical wire depth. Native admission of these newly authored laws is pending.

## Language-Neutral Deep Extras Oracle

The shared exact-word fixture now declares `jsonDepth: 128`. Both native erased and source semantic tests consume this input. The direct source case passed one law / three assertions in 4.10 seconds: independent recursive SQLite reports precisely 128 array edges; the leaf query reports SQL NULL, class `nan` and exact word `7ff8000000000042`; independent SQLite serialization reconstructs an identical semantic projection. This supplements the earlier registered public gate and source-only seven-law route; a fresh final public gate is required after these last test additions.

The fourth logical native attempt failed during Nx graph construction on the removed dev composition-laws Cargo manifest. After canonical Cargo membership changed, the fifth native attempt successfully reached its owning package runner. No new native pass is claimed until its actual results arrive.


Fresh full source and native gates exposed concurrent mutation taxonomy renames, rather than SQLite behavior: source aggregate had stale relative imports and native root failed on required-extension old path before tests. Exact static references hand-aligned with existing renamed physical modules, including owned oracle/schema/test path references. No retired directory recreated. Latest registered source and native retries pending.


Aligned-taxonomy registered public gate GREEN: 486 corpus laws / 10702 assertions, schema oracle 1 / 6, semantic provider 7 / 60; total 494 laws / 10768 assertions, 9 exports, 3 suites, 86 seconds. This is current after minimal/deep JSON fixtures and all audited scalar fields. Native retry remains queued/preparing and is not counted as passing.
