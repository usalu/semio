from specs.lib import *
F = W + '/📌️mounted_layout/🦀️.rs'
H = 'impl MountedLayoutJob {'
c = Cut(F, 'mlay.rs').impl(H, [1038, 1326])
c.child('🌲', 'tree-metrics-and-window-measures', 'tree_metrics_and_window_measures', [(18, 437)],
        'Tree metrics and window measures: the layout credits, the fault and intrinsic size, the tree row metrics and heights, the document table helpers and the tree window measures.')
c.child('🔤', 'layout-input-and-text-measurement', 'layout_input_and_text_measurement', [(444, 735)],
        'Layout input and text measurement: the layout input nodes, the glyph ranges and previews, the text worker, the atlas candidate and the text and glyph measuring.')
c.child('🧱', 'job-state-and-admission', 'job_state_and_admission', [(737, 1037)],
        'Job state and admission: the layout result, the admission phase, the mounted layout job and identity and the job constructor with the node admission.')
c.child('📐', 'text-admission-and-measure-steps', 'text_admission_and_measure_steps', [(1038, 1325)],
        'Text admission and measure steps: the tree content, the text admission, the unwind and worker steps, the shaping, the child gathering, the measuring and the arrangement.')
c.child('🏁', 'collect-publish-and-close', 'collect_publish_and_close', [(1326, 1602)],
        'Collect, publish and close: the collection, the preview and rejection queries, the publish step, the close steps, the immediate layout, the fault display and the interactive job.')
SPEC = c.spec()
