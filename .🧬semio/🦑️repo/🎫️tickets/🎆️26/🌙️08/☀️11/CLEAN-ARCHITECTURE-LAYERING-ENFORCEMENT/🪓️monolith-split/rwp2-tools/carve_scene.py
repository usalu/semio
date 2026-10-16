"""🎞️ Scenes family carve: lib root with the flat namespace the `use super::*` children expect (external leaves, upstream names, glob-mounted children), path rewrites to the final crate homes."""
import re, os, sys, collections
from model import *
import imports, facade_names as fn, oracle2_closure as oc, widen as wd
PFXI = 'semio_framework_os_renderer_wgpu_'
def rd(p): return open(p, encoding='utf8').read()
def wr(p, t): open(p, 'w', encoding='utf8').write(t)
def ident(c): return PFXI + c.replace('-', '_')
REL = '../../../🧱️elements/'
ROOT_SC = SC + '/🦀️.rs'

# (module name, folder under SC) of the children each Scenes crate owns
CHILDREN = {
    'scene-canvas2d': [('canvas2d', '🖌️canvas2d')],
    'scene-ink': [('ink_canvas_model_state', '✒️ink-canvas-model-state'), ('ink_canvas_render', '🖋️ink-canvas-render')],
    'scene-feeds': [('event_feed_and_timeline', '📰️event-feed-and-timeline'), ('node_graph_and_tiled_map', '🧭️node-graph-and-tiled-map')],
    'scene-tables': [('table_and_paint2d', '🧮️table-and-paint2d'), ('block_list_and_diff', '🧱️block-list-and-diff')],
    'scene-text-vfs': [('board2d_and_vfs', '🧩️board2d-and-vfs'), ('text_editor', '✍️text-editor')],
    'scene-render': [('scene_input', '⌨️scene-input'), ('scene_render_entry', '🎨️scene-render-entry')],
}
UPSTREAM = {
    'scene-canvas2d': ['scene-state', 'canvas2d-host'],
    'scene-ink': ['scene-state', 'scene-canvas2d'],
    'scene-feeds': ['scene-state', 'scene-canvas2d', 'engine-board'],
    'scene-tables': ['scene-state', 'scene-feeds'],
    'scene-text-vfs': ['scene-state', 'scene-canvas2d', 'scene-feeds', 'engine-text', 'engine-board'],
    'scene-render': ['scene-state', 'canvas2d-host', 'scene-canvas2d', 'scene-ink', 'scene-feeds', 'scene-tables', 'scene-text-vfs', 'engine-registry', 'engine-board', 'engine-graph', 'engine-attach'],
}
ALIASES = {
    'element_contracts': 'contracts', 'asset_authority': 'asset_authority', 'host_base': 'host_base', 'canvas2d_paint': 'canvas2d_paint', 'mesh_assets': 'host_base', 'media_slots': 'media_slots', 'native_platform': 'native_platform',
}
def engine_name_map():
    out = {}
    for c, mods in [('engine-registry', ['engine_surface_registry', 'prepared_engine_canvas', 'tutorial_surface_geometry']), ('engine-text', ['text_editor']), ('engine-board', ['board2d', 'paint2d', 'tiled_map']), ('engine-graph', ['node_graph']), ('engine-attach', ['engine_surface_attach'])]:
        for f in oc.closure(T + '/' + FOLDER[c] + '/🦀️.rs'):
            if '🧪️tests' in f: continue
            raw = rd(f)
            mod = [m for m in mods if re.search(r'/%s/' % re.escape({'engine_surface_registry': '🗂️engine-surface-registry', 'prepared_engine_canvas': '📦️prepared-engine-canvas', 'tutorial_surface_geometry': '🎓️tutorial-surface-geometry', 'text_editor': '✍️text-editor', 'board2d': '🎲️board2d', 'paint2d': '🖌️paint2d', 'tiled_map': '🗺️tiled-map', 'node_graph': '🕸️node-graph', 'engine_surface_attach': '🧩️engine-surface-attach'}[m]), f)]
            if not mod: continue
            for it in items(raw):
                nm = item_name(it)
                if nm and nm != '_' and it['kind'] not in ('use', 'impl', 'mod'): out[nm] = (c, mod[0])
    return out
def rewrite_text(t, crate, names_by_engine):
    t = t.replace('crate::interpreter::ui_document_close_pending_for', 'crate::element_contracts::ui_document_close_pending_for')
    t = t.replace('crate::interpreter::framework_widget_context', 'crate::element_contracts::framework_widget_context')
    t = t.replace('crate::interpreter::note_world3d_live_camera', ident('scene-state') + '::note_world3d_live_camera')
    t = t.replace('crate::interpreter::current_ui_image_key', ident('scene-canvas2d') + '::current_ui_image_key')
    for n in ['log_debug_diagnostic', 'app_now_ms', 'spawn_app_task']:
        t = re.sub(r'(?<![\w:])crate::%s\b' % n, 'crate::host_base::%s' % n, t)
    t = t.replace('crate::shell::host_io_call', 'crate::host_base::host_io_call')
    t = t.replace('crate::renderer_asset_io', 'crate::asset_authority::renderer_asset_io').replace('crate::reserve_renderer_asset_request', 'crate::asset_authority::reserve_renderer_asset_request')
    t = re.sub(r'crate::canvas2d_gumball::', ident('canvas2d-host') + '::', t)
    for n in ['canvas_active_utility', 'scene_host_retiring', 'write_scene_action_batch']:
        t = t.replace('crate::scenes::' + n, ident('scene-state') + '::' + n)
    def eng(m):
        n = m.group(1)
        if n in names_by_engine:
            c, mod = names_by_engine[n]; return '%s::%s::%s' % (ident(c), mod, n)
        return m.group(0)
    t = re.sub(r'(?<![\w:])engine_canvas::([A-Za-z_]\w*)', eng, t)
    return t
def used_aliases(texts):
    al = set()
    for tx in texts:
        for m in re.finditer(r'crate::(element_contracts|asset_authority|host_base|canvas2d_paint|mesh_assets|media_slots|native_platform)\b', tx): al.add(m.group(1))
    return al
def carve(crate, doc):
    kids = CHILDREN[crate]
    names_by_engine = engine_name_map()
    files = [SC + '/' + f + '/🦀️.rs' for _, f in kids]
    texts = []
    for f in files:
        t = rewrite_text(rd(f), crate, names_by_engine); wr(f, t); texts.append(t)
    root_old = rd(ROOT_SC)
    alltext = '\n'.join(texts)
    need = [u for u in imports.uses(root_old) if u['alias'] not in ('_', '*', 'engine_canvas') and re.search(r'(?<![\w.])' + re.escape(u['alias']) + r'\b', strip(alltext))]
    lines = ''.join(imports.render(u) for u in need).replace('use crate::shell::', 'use crate::element_contracts::')
    ext = ''
    al = sorted(used_aliases(texts))
    for a in al: ext += 'extern crate %s%s as %s;\n' % (PFXI, ALIASES[a], a)
    if 'action_args_json' in alltext: ext += 'pub(crate) use %shost_base::action_args_json;\n' % PFXI
    up = ''
    toks = set(re.findall(r'[A-Za-z_]\w*', strip(alltext)))
    own = set()
    for f in files:
        for it in items(rd(f)):
            nm = item_name(it)
            if nm: own.add(nm)
            if it['kind'] == 'thread_local!': own |= set(re.findall(r'static\s+(\w+)\s*:', rd(f).split('\n')[it['start']] + ''))
    seen = {n: 'own' for n in own}
    for u in UPSTREAM[crate]:
        ex = fn.exported(u)
        groups = collections.defaultdict(list)
        for n, cfgs in ex.items():
            if n in toks and n not in seen:
                for c in cfgs: groups[c].append(n)
                seen[n] = u
        for c, ns in sorted(groups.items()):
            up += ''.join(a + '\n' for a in c) + 'use %s::{%s};\n' % (ident(u), ', '.join(sorted(ns)))
    mods = ''
    for m, f in kids:
        mods += '#[path = "%s🎞️Scenes/🎯️targets/🧊️wgpu/%s/🦀️.rs"]\nmod %s;\npub use %s::*;\n\n' % (REL, f, m, m)
    lib = doc.rstrip('\n') + '\n\n' + ext + lines + up + '\n' + mods + '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n'
    wr(T + '/' + FOLDER[crate] + '/🦀️.rs', lib)
    # facade: remove the children mounts
    t = rd(ROOT_SC)
    for m, f in kids:
        for vis in ('pub(crate) use', 'pub use'):
            old = '#[path = "%s/🦀️.rs"]\nmod %s;\n%s %s::*;\n\n' % (f, m, vis, m)
            if t.count(old) == 1: t = t.replace(old, ''); break
        else: raise AssertionError((crate, m))
    wr(ROOT_SC, t)
    return al, [u['alias'] for u in need]
