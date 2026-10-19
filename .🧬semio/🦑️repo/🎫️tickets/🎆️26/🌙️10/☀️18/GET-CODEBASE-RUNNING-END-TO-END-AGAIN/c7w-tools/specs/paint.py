from specs.lib import *
F = W + '/🖌️paint/🦀️.rs'
c = Cut(F, 'paint.rs')
c.child('🔤', 'retained-glyph-and-node-cursors', 'retained_glyph_and_node_cursors', [(33, 366)],
        'Retained glyph and node cursors: the tree and text limits, the glyph cursor and its weighted and flowed steps and the node paint cursor with its step.')
c.child('📝', 'retained-text-steps', 'retained_text_steps', [(368, 732)],
        'Retained text steps: the fixed output budgets, the select close route, the caret, presence and text node steps, the tree row lookups and the table header and cell steps.')
c.child('🌲', 'retained-tree-node-painting', 'retained_tree_node_painting', [(734, 1092)],
        'Retained tree node step: the tree node paint step and the `paint_node_step` entry point.')
c.child('🎨', 'node-painting-with-driver', 'node_painting_with_driver', [(1094, 1928)],
        'Paint node step with driver: the per-variant node painting of `paint_node_step_with_driver`.')
c.child('🔄', 'interactive-state-sync', 'interactive_state_sync', [(1947, 2462)],
        'Interactive state sync: the retained sync keys, records, frames, phases and cursor and the interactive state node step.')
c.child('📶', 'progress-celebrate-and-widget-painters', 'progress_celebrate_and_widget_painters', [(2466, 2711)],
        'Progress, celebrate and widget painters: the presence overlay, the progress bars, the celebrate strokes and the focus ring, loading border, stack frame, separator, checkbox and slider painters.')
c.child('🖼', 'ui-image-sources-skeleton-and-overlays', 'ui_image_sources_skeleton_and_overlays', [(2728, 3109)],
        'Ui image sources, skeleton and overlays: the image admission ledger and data url decoding, the skeleton blocks and the overlay backdrop, surface, chrome and tooltip painters.')
SPEC = c.spec()
