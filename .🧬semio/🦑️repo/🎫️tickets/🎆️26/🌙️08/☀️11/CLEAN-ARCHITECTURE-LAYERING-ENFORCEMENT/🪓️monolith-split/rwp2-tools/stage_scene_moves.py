from model import *
import mvtool
DST = SC + '/🗃️scene-state/🦀️.rs'
G = [
 ('⌨️scene-input', ['MAP_MARQUEE_THRESHOLD_PX', 'canvas_active_utility', 'request_canvas_pointer_gesture_cancel_for_host', 'write_scene_action_batch', 'BLOCK_LIST_PALETTE_DRAG_MIME', 'SceneListTransferStart', 'SceneListTransferRelease', 'SceneModifiers', 'SceneListHit'], '⌨️SceneInputKit'),
 ('🎨️scene-render-entry', ['render_placeholder', 'VirtualFileSystemChromeLabels', 'BlockListChromeLabels', 'IconRenderChromeLabels', 'SceneChromeLabels', 'SceneEngineHosts'], '🎨️SceneRenderKit'),
 ('🧮️table-and-paint2d', ['TableStepperFocusTarget', 'TableEditableTextFocusTarget', 'TableStepperAccessibilityCell', 'TableEditableTextAccessibilityCell', 'TableButtonAccessibilityCell'], '🧮️TableAccessibilityKit'),
 ('🧭️node-graph-and-tiled-map', ['retire_tiled_map_scene_identity'], '🧭️TiledMapRetirement'),
 ('✍️text-editor', ['TextEditorUiState', 'TextEditorRenameDraft', 'text_editor_ui'], '✍️TextEditorUiKit'),
 ('✒️ink-canvas-model-state', ['InkBoundsF'], '✒️InkBounds'),
 ('🖋️ink-canvas-render', ['draw_ink_rect_outline'], '🖋️InkOutline'),
 ('🧩️board2d-and-vfs', ['VfsAccessibilityFocusTarget'], '🧩️VfsFocusKit'),
]
for f, names, region in G:
    text, out = mvtool.cut(SC + '/' + f + '/🦀️.rs', set(names), DST, region=region)
    print(f, len(text.split('\n')), 'lines')
