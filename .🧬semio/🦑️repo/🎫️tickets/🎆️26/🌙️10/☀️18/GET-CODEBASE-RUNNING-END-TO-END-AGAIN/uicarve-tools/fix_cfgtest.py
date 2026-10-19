import os, re, sys
sys.path.insert(0, 'lib')
import carve
from carve import UI, rd, wr
W = UI + '/🎯️targets/🧊️wgpu'
TK = '#[cfg(any(test, feature = "testkit"))]'
def edit(path, pairs):
    s = rd(path); n = s
    for old, new, cnt in pairs:
        c = n.count(old)
        if c != cnt: print('COUNT MISMATCH', path[-50:], repr(old[:60]), c, 'expected', cnt)
        n = n.replace(old, new)
    if n != s: wr(path, n)
edit(W + '/🖌️paint/🦀️.rs', [('    #[cfg(test)]\n    pub(crate) fn terminal_is_empty(&self) -> bool {', '    ' + TK + '\n    pub fn terminal_is_empty(&self) -> bool {', 2)])
edit(W + '/📨️scene_slots/🦀️.rs', [('#[cfg(test)]\npub fn collect_scene_slots<', TK + '\npub fn collect_scene_slots<', 1), ('#[cfg(test)]\nfn collect_scene_slots_node<', TK + '\nfn collect_scene_slots_node<', 1)])
edit(W + '/⚙️engine/🦀️.rs', [
 ('    #[cfg(test)]\n    pub fn frame<H: SceneHost>', '    ' + TK + '\n    pub fn frame<H: SceneHost>', 1),
 ('    #[cfg(test)]\n    pub(crate) fn progressive_layout_preview', '    ' + TK + '\n    pub fn progressive_layout_preview', 1),
 ('    #[cfg(test)]\n    pub(crate) fn progressive_glyph_preview', '    ' + TK + '\n    pub fn progressive_glyph_preview', 1),
 ('#[cfg(test)] use semio_framework_ui_wgpu_paint_reference::paint_tree;', TK + ' use semio_framework_ui_wgpu_paint_reference::paint_tree;', 1),
 ('#[cfg(test)] use semio_framework_ui_wgpu_hosting::scene_slots::collect_scene_slots;', TK + ' use semio_framework_ui_wgpu_hosting::scene_slots::collect_scene_slots;', 1)])
edit(W + '/⚙️engine/🪟️surfaces/🦀️.rs', [('    #[cfg(test)]\n    pub fn len(&self) -> usize {', '    ' + TK + '\n    pub fn len(&self) -> usize {', 1)])
# paint: test-only imports that only the relocated reference pipeline used
p = W + '/🖌️paint/🦀️.rs'
s = rd(p)
for pat in (r'(?m)^#\[cfg\(test\)\] use semio_framework_ui_wgpu_layout::chrome::chrome_item_bg;\n', r'(?m)^#\[cfg\(test\)\] use semio_framework_ui_declaration::ui::\{[^\n]*\};\n', r'(?m)^#\[cfg\(test\)\] use semio_framework_ui_retained_tree::tree::EditState;\n', r'(?m)^#\[cfg\(test\)\] use semio_framework_ui_wgpu_widget_kit::wrap_text;\n', r'(?m)^#\[cfg\(test\)\] use semio_framework_ui_declaration::IconName;\n'):
    s = re.sub(pat, '', s)
wr(p, s)
print('ok')
