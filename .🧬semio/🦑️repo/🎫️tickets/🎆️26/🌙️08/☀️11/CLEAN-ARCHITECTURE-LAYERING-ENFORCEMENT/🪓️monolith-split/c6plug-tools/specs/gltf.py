from specs.lib import *
GL = '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot'
CH = [
    ch('🧵', 'source-form-and-json', 'source_form_and_json', [(52, 131)],
       'Source form and JSON of the glTF snapshot: the wire dialect a snapshot was parsed from and the opaque JSON value with its value codecs.'),
    ch('🔢', 'attribute-map-and-defaults', 'attribute_map_and_defaults', [(164, 248)],
       'Attribute map and defaults of the glTF snapshot: the ordered accessor name codec and the spec-mandated scalar defaults with their omission predicates.'),
    ch('🌳', 'scene-graph', 'scene_graph', [(252, 434)],
       'Scene graph of the glTF snapshot: the asset, scenes, nodes, morph targets, mesh primitives and meshes.'),
    ch('🧮', 'accessors-and-buffers', 'accessors_and_buffers', [(438, 574)],
       'Accessors and buffers of the glTF snapshot: the sparse accessor parts, the accessors, the buffer views and the buffers.'),
    ch('🎨', 'materials-and-textures', 'materials_and_textures', [(578, 847)],
       'Materials and textures of the glTF snapshot: the texture infos, the PBR metallic roughness model, the alpha mode, the materials, textures, images and samplers.'),
    ch('🎬', 'skins-and-animations', 'skins_and_animations', [(851, 1002)],
       'Skins and animations of the glTF snapshot: the skins, the animation paths, channels, interpolation, samplers and animations.'),
    ch('📷', 'cameras', 'cameras', [(1006, 1149)],
       'Cameras of the glTF snapshot: the orthographic and perspective projections, the projection enum, the camera and their value codecs.'),
    ch('📄', 'document-and-decoded-input', 'document_and_decoded_input', [(1153, 1292)],
       'Document and decoded input of the glTF snapshot: the document root, the component and accessor types and the decoded accessor input.'),
]
SPEC = dict(orig='gltf.rs', file=GL + '/🦀️.rs', children=CH, external=mounted(GL + '/🦀️.rs', orig='gltf.rs'), nest=True, oracle_root=GL + '/📦️packages/🦀️rust/🦀️.rs', oracle_copy=[GL])
