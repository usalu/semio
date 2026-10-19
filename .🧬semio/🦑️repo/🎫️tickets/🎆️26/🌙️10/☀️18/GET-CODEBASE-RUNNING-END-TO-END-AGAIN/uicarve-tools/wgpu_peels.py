import json, os, re, sys
sys.path.insert(0, 'lib')
import carve, peel
from carve import R, UI, rd, wr
W = UI + '/🎯️targets/🧊️wgpu'
E = UI + '/🧱️elements'
dry = '--write' not in sys.argv
reports = {}
# P1 input -> hit (mounted-layout crate child)
P1 = {'RetainedSceneHit', 'RetainedHitRegistration', 'RetainedTreeRow', 'retained_tree_row', 'retained_tree_section_header_band', 'retained_scene_hit', 'retained_hit_registration', 'retained_tree_chevron_registration', 'retained_tree_drag_handle_registration', 'retained_hit_target_tests'}
reports['input_hit'] = peel.peel(W + '/📥️input/🦀️.rs', lambda it: it.name in P1 or (it.kind == 'impl' and it.target == 'RetainedHitRegistration'),
    W + '/📌️mounted_layout/🎯️hit/🦀️.rs', '🎯️ Retained hit registration: projects laid-out retained nodes onto the pointer registry at the band paint owns.',
    'crate::wgpu::input', 'crate::wgpu::hit', prefix_cut={'tests': '#[cfg(test)]'}, strip_cfgs=('#[cfg(feature = "wgpu-engine")]', '#[cfg(feature="wgpu-engine")]'), dry=dry)
# P2 widgets -> kit
KIT = {'InputMeta', 'SliderMeta', 'StepperMeta', 'RingMeta', 'SelectPopupWheelScope', 'WidgetInteractionMaps', 'WidgetContext', 'SelectItem', 'KeyValueEntry', 'TreeItemAction', 'TreeItem', 'TreeSection', 'ControlNode', 'WidgetNode',
       'register_input_meta', 'register_select_meta', 'register_toggle_meta', 'render_scroll_region', 'draw_icon', 'measure_text_width', 'draw_text_wrapped', 'wrap_text', 'draw_text_weighted', 'draw_text_on', 'draw_text_face_on',
       'draw_text_face_affine_on', 'draw_text_overlay_on', 'affine_glyph_tests', 'pen_glyph_run', 'draw_text', 'draw_text_face', 'draw_text_overlay', 'gizmo'}
KITIMPL = {'InputMeta', 'WidgetInteractionMaps', 'WidgetNode'}
reports['widgets_kit'] = peel.peel(W + '/🪀️widgets/🦀️.rs', lambda it: it.name in KIT or (it.kind == 'impl' and it.target in KITIMPL),
    W + '/🪀️widgets/🧰️kit/🦀️.rs', '🧰️ Widget kit: the immediate-mode context, interaction maps, widget node model and text drawing every control renders through.',
    'crate::wgpu::widgets', 'crate::wgpu::widgets_kit', prefix_cut={'metrics_tests': '#[cfg(test)]'}, strip_cfgs=('#[cfg(feature = "wgpu-engine")]',), dry=dry)
# P3 engine -> engine_surface
src = rd(W + '/⚙️engine/🦀️.rs')
its = peel.top_items(src)
ui_start = next(i.start for i in its if i.name == 'Ui' and i.kind == 'struct')
reports['engine_surface'] = peel.peel(W + '/⚙️engine/🦀️.rs', lambda it: it.start < ui_start and it.kind != 'use',
    W + '/⚙️engine/🪟️surfaces/🦀️.rs', '🪟️ Engine surfaces: windows, tooltips, retained paint walk, surface registry, document ingress and layout worker admission behind the `Ui` facade.',
    'crate::wgpu::engine', 'crate::wgpu::engine_surface', prefix_cut={'Ui': '//#region 🔖️Ui'}, dry=dry)
# P4 push_stepper_* paint -> stepper element
reports['stepper'] = peel.peel(W + '/🖌️paint/🦀️.rs', lambda it: it.name in ('push_stepper_chrome', 'push_stepper_icon'),
    E + '/🪜️Stepper/🎯️targets/🧊️wgpu/🦀️.rs', '', 'crate::wgpu::paint', 'crate::wgpu::stepper', append=True, dry=dry)
# P5 constrain_number_input events -> kit (after P2 the kit file exists; in dry mode it is only planned)
if not dry:
    reports['constrain'] = peel.peel(W + '/⚡️events/🦀️.rs', lambda it: it.name == 'constrain_number_input',
        W + '/🪀️widgets/🧰️kit/🦀️.rs', '', 'crate::wgpu::events', 'crate::wgpu::widgets_kit', append=True, dry=dry)
for k, v in reports.items():
    print('==', k, 'lines', v['lines'], 'moved', len(v['moved']))
    print('  used_by_rest', v['used_by_rest'][:60])
    print('  used_from_rest', v['used_from_rest'])
json.dump(reports, open('lists/wgpu-peels.json', 'w'), indent=1, ensure_ascii=False)
