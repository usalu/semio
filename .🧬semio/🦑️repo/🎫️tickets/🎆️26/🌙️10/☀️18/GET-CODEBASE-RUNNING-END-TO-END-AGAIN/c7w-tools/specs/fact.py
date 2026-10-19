from specs.lib import *
F = W + '/🖍️draw/🔧️pipeline/🏭️factory/🦀️.rs'
c = Cut(F, 'fact.rs')
c.child('🎯', 'gpu-targets-and-pipeline-resources', 'gpu_targets_and_pipeline_resources', [(17, 449)],
        'Gpu targets and pipeline resources: the scene color, composite and shadow targets, the world globals ring, the vertex records and the `PipelineResources` state with the stencil states.')
c.child('🏗', 'pipeline-resources-construction', 'pipeline_resources_construction', [(450, 1390)],
        'Pipeline resources construction: the single `PipelineResources::new` that builds every pipeline, atlas, target and bind group.')
SPEC = c.spec()
