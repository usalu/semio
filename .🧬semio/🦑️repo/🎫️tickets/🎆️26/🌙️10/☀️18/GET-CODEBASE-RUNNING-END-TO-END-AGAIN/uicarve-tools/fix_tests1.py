import os, re, sys, json
sys.path.insert(0, 'lib')
import carve, rs, crategen
from carve import R, UI, rd, wr
W = UI + '/🎯️targets/🧊️wgpu'
def sub(path, old, new, count=1):
    s = rd(path)
    if old not in s:
        print('SKIP (absent)', path[-60:], old[:50]); return
    wr(path, s.replace(old, new, count))
sub(UI + '/🧪️tests/🌳️tree-row-rects/🦀️.rs', 'semio_framework_ui_wgpu_widgets::TREE_ROW_HEIGHT', 'semio_framework_ui_theme::metrics::TREE_ROW_HEIGHT')
sub(UI + '/🧪️tests/🔬️targets-wgpu-widget-metrics/🦀️.rs', 'use super::{HitKind, HitTarget, InputState, WidgetInteractionMaps};', 'use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState};\nuse semio_framework_ui_wgpu_widget_kit::WidgetInteractionMaps;')
sub(UI + '/🧪️tests/🎯️retained-hit-targets/🦀️.rs', 'use super::{HitKind, HitTarget, InputState};', 'use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState};')
# paint reference names: retarget paint -> paint_reference
P = json.load(open('lists/wgpu-peels.json'))
names = sorted({n for k, n in P['paint_reference']['moved'] if n and k not in ('impl', 'comment')})
spec = carve.Spec('specs/wgpu.json')
mini = carve.Spec('specs/wgpu.json')
mini.rules = carve.Rules([{'from': n, 'crate': 'paintref', 'to': n} for n in names])
cnt = 0
files = set()
for k, c in spec.crates.items():
    if k == 'paintref': continue
    for f in crategen.crate_files(spec, c):
        if f.endswith('.rs') and os.path.exists(f): files.add((f, k))
for f, k in sorted(files):
    old = rd(f)
    if 'semio_framework_ui_wgpu_paint' not in old: continue
    new = carve.rewrite_text(old, mini, k, None, only_roots={'semio_framework_ui_wgpu_paint'})
    if new != old: wr(f, new); cnt += 1; print('paintref retarget', os.path.relpath(f, UI)[-70:])
# engine root: drop the cfg(test)-only imports that served the relocated tests
p = W + '/⚙️engine/🦀️.rs'
s = rd(p)
for pat in (r'(?m)^#\[cfg\(test\)\]\nuse semio_framework_ui_wgpu_paint[^\n]*paint_tree;\n', r'(?m)^#\[cfg\(test\)\]\nuse semio_framework_ui_wgpu_hosting::scene_slots::collect_scene_slots;\n', r'(?m)^#\[cfg\(test\)\]\nuse semio_framework_ui_wgpu_paint_reference::paint_tree;\n'):
    s = re.sub(pat, '', s)
wr(p, s)
print('done', cnt)
