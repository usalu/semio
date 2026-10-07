# Original G3 glTF Dependency Readback

Original six-law76062 stopped before assertions on PNG22 plus glTF38. UI accepted PNG ownership; this lane accepted readback of the original glTF IO/compiler boundary. Fresh source already contains coherent external repairs for every captured glTF reference, so this lane authored no glTF change and preserved the concurrent migration owner.

- Original text diff now has exactly one diff_codec module at9 with its existing body and public reexport1306; binary diff has its own single module7/reexport1281.
- Original camera serde implementations have moved out of schema snapshot into text snapshot semantic_cache_codec154 under cfg(test), with explicit serde Deserialize/Deserializer/Serialize/Serializer160. Schema snapshot preserves domain ToValue/FromValue implementations and test-only serde imports29; no production serde trait dependency was added.
- Original schema mutation aggregate imports existing canonical io::text::mutations::gltf_row_mutation4, matching both original row helper callers.
- Original binary node-name protobuf module imports existing canonical io::text::mutations::GltfChangeNodeNameFacadeError54 alongside existing FacadeResult/facade_error, matching all five captured error constructors.
- Original binary inference imports existing canonical text inference module as text5, preserving all original framing/checksum/header/payload code.
- Original binary snapshot owned_pack::record88 is now pub(crate), so original text snapshot pack_codec imports the same physical schema owner17 and calls the same helper22; no alternate schema or re-encoder was introduced.
- Original SQLite snapshot decode58 and encode59 now refer directly to canonical io::binary::snapshot::owned_pack controlled_spec_producer/reconstruct_record_controlled/encode_native, preserving physical projection and reconstruction.

This readback is a coherent source release, not a compiler pass. Schemas, protocols, fixtures, assertions and codec bodies were not changed by this lane. A changed-source original metadata/G3 gate is justified after assigned PNG release; no duplicate or unchanged restart is required. The generic quoted scanner baseline73277 remains this lane's sole heavy runtime process before its next native G3 replay.

## Current glTF55 borrowed-shape prerequisite

Original metadata14 terminal59377/Nx13m3s advanced beyond prior PNG22/glTF38. Structured original compiler receipt exact-cargo-laws-jmJCg6/00/build.stdout contains55 glTF BorrowedDslField diagnostics: GltfJson50, and GltfPrimitive/GltfCameraProjection/GltfTexture/GltfImage/GltfMorphTarget one each. Original ordinary DslField implementations remain in binary snapshot owned_pack22–79 and explicitly use six existing physical helper types, not generic dynamic Value: GltfJson→json::Json flat typed nodes; MorphTarget→Target attribute records; Primitive→Primitive; CameraProjection→Projection tagged record; Image→Image kind+fields; Texture→Texture kind+fields.

Six finite BorrowedDslField declarations were added beside original json owner18, each forwarding the existing helper type's derived static SHAPE. Those helpers already derive original DslRecord/BorrowedDslRecord/field authority. No new schema/factory/owned metadata/source copy, codec body, ordinary shape, typed conversion, fixture or assertion was authored or altered. In particular GltfJson preserves its original flat typed Json nodes, not a dynamic Value or recursive re-lowering substitute.

Exact saved current-before hash645bb64711a645c083c09e38d15accbb9ea79e2fb3441348c3f27955bf799358; after688b0caf45b42cd736a5b020d282e83120660868d7a4be69811e2c5d89992a20. Removing exactly the six added one-line BorrowedDslField impls reproduces every before byte. Generated before capture document-continuation-2026-10-06/gltf-pack-before-borrowed-shape.rs. Fresh readback confirms mappings and original ordinary helpers; actual compiler/native gate remains pending. UI retains SVG38 closure and can run source-current original metadata15 now; this lane preserves its sole scanner86367 process and follows sequential G3 original seven-law gate after generic regression.
