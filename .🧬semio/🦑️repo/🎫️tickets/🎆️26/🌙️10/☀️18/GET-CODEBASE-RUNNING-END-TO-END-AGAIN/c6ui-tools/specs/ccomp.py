from specs.lib import *
F = U + '/🧬️contract/🧩️component/🦀️.rs'
CH = [
    ch('🎛', 'container-roles-and-row-actions', 'container_roles_and_row_actions', [(22, 218)],
       'Container roles and row actions: the container role, input kind and row action placement enums, the drop overlay, select item and key value records, the `RowAction` and the `RowTarget` with their credited arguments.'),
    ch('🎨', 'basic-widget-props', 'basic_widget_props', [(222, 463), (928, 975)],
       'Basic widget props: the container, text, button, separator, input, select, toggle, key value list, ring, icon select and progress props with their appearances.'),
    ch('🔢', 'number-scale-and-slider-props', 'number_scale_and_slider_props', [(465, 669), (1336, 1358)],
       'Number scale and slider props: the number scale, slider appearance, number bound and limits, the slider and number stepper props and the range and snap validity checks.'),
    ch('🎚', 'slider-geometry-and-key-handling', 'slider_geometry_and_key_handling', [(671, 926)],
       'Slider geometry and key handling: the slider axis position and value, the dial, the pointer value and snap, the slider and number key values, the text input keys, the number display and the colour hex parsing.'),
    ch('🌳', 'tree-props', 'tree_props', [(977, 1166)],
       'Tree props: the tree props, presentation, window and row extent, the section props and the tree item props with their credited targets.'),
    ch('🗃', 'table-image-extension-and-component', 'table_image_extension_and_component', [(1168, 1325)],
       'Table, image, extension and component: the table and table row props, the image and extension props and the `Component` enum with its accessors.'),
]
SPEC = dict(orig='ccomp.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='ccomp.rs'))
