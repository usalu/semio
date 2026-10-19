from specs.lib import *
F = W + '/📐️flex/🦀️.rs'
c = Cut(F, 'flex.rs')
c.child('🎚', 'layout-node-kind-geometry-and-job-stages', 'layout_node_kind_geometry_and_job_stages', [(28, 166), (1040, 1062)],
        'Layout node kind, geometry and job stages: the layout node kind catalogue, the flex rect and measure constraint and the layout job stage and step.')
c.child('🌊', 'flow-style', 'flow_style', [(170, 438)],
        'Flow style: the dimension, the flow kind and style, the flow from the authored spec, the tree content flow and `flow_for`.')
c.child('🎨', 'style-mapping-and-chrome-metrics', 'style_mapping_and_chrome_metrics', [(442, 587)],
        'Style mapping and chrome metrics: the taffy style mapping, the section and field chrome metrics and the grow children rule.')
c.child('🧮', 'flex-tree', 'flex_tree', [(591, 1034)],
        'Flex tree: the scratch leaf, the `FlexTree` with its measure and arrange steps, the flow style distribution and the scratch solver.')
SPEC = c.spec()
