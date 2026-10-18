from specs.lib import *
F = U + '/🎬️scene/📏️world3d-modelling/🦀️.rs'
CH = [
    ch('🏷', 'text-tone-and-annotation-types', 'text_tone_and_annotation_types', [(134, 271)],
       'Text, tone and annotation types: the bilingual text, the theme tone, the marker shape, the dimension, angle, marker and leader annotations and the annotation enum.'),
    ch('📐', 'annotation-codecs', 'annotation_codecs', [(273, 484)],
       'Annotation codecs: the annotation constructors and validation, the annotation lane codec and the annotation layer with its lane codec.'),
    ch('🌡', 'scalar-field', 'scalar_field', [(488, 700)],
       'Scalar field: the scalar domain and colour ramp, the scalar range and legend, the legend tick and the scalar field with its retirement and lane codec.'),
    ch('🎯', 'interaction-options', 'interaction_options', [(704, 911)],
       'Interaction options: the pick granularity and targets, the section plane and cap, the sub-element style, the highlight and the modelling options with their lane codecs.'),
    ch('🧭', 'overlay-projection', 'overlay_projection', [(915, 1098)],
       'Overlay projection: the annotation pixel constants, the projected arrow, arc, marker and label records, the annotation projection and the `project_world3d_annotations` pass.'),
    ch('🔤', 'legend-strings-and-sub-element-paint', 'legend_strings_and_sub_element_paint', [(1100, 1272)],
       'Legend strings and sub-element paint: the modelling strings, the legend value formatting and lines, the sub-element defaults and paint and the modelling lane text.'),
]
EXPORT = [
    ('mod text_tone_and_annotation_types;', 'pub use text_tone_and_annotation_types::{World3dMarkerShape, World3dTone};'),
    ('mod scalar_field;', 'pub use scalar_field::{World3dColorRamp, World3dScalarDomain};'),
    ('mod interaction_options;', 'pub use interaction_options::World3dPickGranularity;'),
]
SPEC = dict(orig='w3dm.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='w3dm.rs'),
            patch_parent=[(a, a + '\n' + b) for a, b in EXPORT], expect_extra=[b for a, b in EXPORT])
