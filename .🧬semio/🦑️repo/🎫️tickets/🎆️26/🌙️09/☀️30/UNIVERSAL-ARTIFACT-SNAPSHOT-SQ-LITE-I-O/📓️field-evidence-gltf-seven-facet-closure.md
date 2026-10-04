# GLTF Seven Remaining Facet Source Closure

Complete current projection/reconstruction bodies for node, mesh, material, texture/image/sampler, skin, animation and camera facets were read. Exact primary paths/SHA are retained in generated/field-evidence-gltf-seven-facet-closure.json. This closes the field-mapping residue from wave1, not a test/runtime GREEN.

Node:7–22 preserves name, mesh/camera/skin refs, optional16/3/4/3 matrix/translation/rotation/scale presence and exact components, every ordered child/weight, extensions/extras. Fixed dimensions are checked on reconstruction; float companions carry raw bits/class.

Mesh:6–20 preserves name/weights/extensions/extras, every primitive optional indices/material/mode, ordered attribute (name,accessor) pairs and complete ordered morph-target attribute pairs. Attributes deliberately remain occurrence vectors; no Map sort/dedupe occurs. Primitive extras/extensions each retain their own tree owner.

Material:8–24 preserves name/emissive-factor/alpha-mode/cutoff/double-sided, optional PBR base-color factors/metallic/roughness/texture slots, normal scale/occlusion strength/emissive texture. Texture-info index/texCoord and every independent extensions/extras field have relational scalar/tree refs. Optional rows distinguish absent blocks from default blocks.

Texture:3–12 preserves texture sampler/source/name, image URI/MIME/bufferView/name, sampler mag/min/filter and wrapS/T words, with all extension/extra fields. Skin:3–4 preserves inverse-bind/skeleton/name, ordered joints and extras/extensions. Animation:3–14 preserves each sampler input/output/interpolation, channel sampler/own metadata and target node/path/independent metadata. Camera:4–14 preserves camera tag/name/metadata and all perspective optional aspect/zfar/yfov/znear or orthographic xmag/ymag/zfar/znear and each projection's metadata.

All references use actual unsigned split-word/index storage rather than f64. All current float factors/components use FloatRow companions. Extra/extension recursive owner remains actual GltfJson (null/bool/f64/string/ordered array/object); it is not all9 DslValue and cannot be credited as unsigned/octet semantics it does not declare. The current source domains' document/buffer/accessor/extras branches were traced in wave1; together all actual persisted root and declared GLTF leaf fields have current projection/reconstruction mappings. No concrete omitted field or serialized whole GLTF payload found.

The ledger continues to require strict malformed/ownership/cycle/row shape neutral-case review and actual owning System allocation/cancellation/error/release proofs. Helpers' paid reserve calls are source evidence, not allocator equality. Actual independent Source leaf and its neutral/thirdparty fixtures remain linked in wave1; no new executable receipt claimed.
