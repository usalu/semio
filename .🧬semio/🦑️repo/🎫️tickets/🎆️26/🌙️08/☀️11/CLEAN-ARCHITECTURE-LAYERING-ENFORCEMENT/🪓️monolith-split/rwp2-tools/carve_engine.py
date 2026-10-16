"""🔩️ EngineCanvas family carve: lib root, cross-crate `super::` rewrites and facade imports for one engine crate."""
import re, sys
from model import *
import imports, oracle2_closure as oc
MODULE_CRATE = {'engine_surface_registry': 'engine-registry', 'prepared_engine_canvas': 'engine-registry', 'tutorial_surface_geometry': 'engine-registry', 'text_editor': 'engine-text', 'board2d': 'engine-board', 'paint2d': 'engine-board', 'tiled_map': 'engine-board', 'node_graph': 'engine-graph', 'engine_surface_attach': 'engine-attach'}
MODULE_DIR = {'engine_surface_registry': '🗂️engine-surface-registry', 'prepared_engine_canvas': '📦️prepared-engine-canvas', 'tutorial_surface_geometry': '🎓️tutorial-surface-geometry', 'text_editor': '✍️text-editor', 'board2d': '🎲️board2d', 'paint2d': '🖌️paint2d', 'tiled_map': '🗺️tiled-map', 'node_graph': '🕸️node-graph', 'engine_surface_attach': '🧩️engine-surface-attach'}
PFX = 'semio_framework_os_renderer_wgpu_'
def rd(p): return open(p, encoding='utf8').read()
def wr(p, t): open(p, 'w', encoding='utf8').write(t)
def crate_ident(c): return PFX + c.replace('-', '_')

def rewrite_siblings(crate):
    """`super::<module>` of a module that now lives in another crate becomes that crate's path."""
    for mod, c in MODULE_CRATE.items():
        if c == crate: continue
        pass
    for mod in [m for m, c in MODULE_CRATE.items() if c == crate]:
        p = EC + '/' + MODULE_DIR[mod] + '/🦀️.rs'; t = rd(p); orig = t
        for other, oc_ in MODULE_CRATE.items():
            if oc_ == crate: continue
            t = re.sub(r'(?<![\w:])super::' + other + r'\b', crate_ident(oc_) + '::' + other, t)
        if t != orig: wr(p, t)

def facade_use(crate):
    p = EC + '/🦀️.rs'; t = rd(p)
    for mod in [m for m, c in MODULE_CRATE.items() if c == crate]:
        old = '#[path = "%s/🦀️.rs"]\nmod %s;\n' % (MODULE_DIR[mod], mod)
        assert t.count(old) == 1, old
        t = t.replace(old, 'use %s::%s;\n' % (crate_ident(crate), mod))
    wr(p, t)

def lib_root(crate, doc, aliases):
    mods = [m for m, c in MODULE_CRATE.items() if c == crate]
    rel = '../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/'
    out = doc.rstrip('\n') + '\n\n'
    out += ''.join(a + '\n' for a in aliases) + ('\n' if aliases else '')
    for m in mods: out += '#[path = "%s%s/🦀️.rs"]\npub mod %s;\n\n' % (rel, MODULE_DIR[m], m)
    out += '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n'
    wr(T + '/' + FOLDER[crate] + '/🦀️.rs', out)
