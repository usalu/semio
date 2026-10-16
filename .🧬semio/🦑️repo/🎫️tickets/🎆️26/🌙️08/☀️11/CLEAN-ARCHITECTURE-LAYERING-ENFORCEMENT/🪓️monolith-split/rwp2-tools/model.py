"""🧭️ WP2 crate model: file-to-crate assignment, rank order and the outline-based item scanner shared by the rwp2 tools."""
import os, re, sys
ROOT = '/Users/ueli/Documents/semio'
E = ROOT + '/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine'
EL = E + '/🧱️elements'
T = E + '/🎯️targets/🧊️wgpu'
EC = EL + '/⚙️EngineCanvas/🎯️targets/🧊️wgpu'
SC = EL + '/🎞️Scenes/🎯️targets/🧊️wgpu'
IC = EL + '/🖼️IconRenderHost/🎯️targets/🧊️wgpu'
CH = EL + '/📐️Canvas2dHost/🎯️targets/🧊️wgpu'
RANK = ['engine-registry', 'engine-text', 'engine-board', 'engine-graph', 'engine-attach', 'scene-state', 'canvas2d-host', 'scene-canvas2d', 'scene-ink', 'scene-feeds', 'scene-tables', 'scene-text-vfs', 'icon-host', 'scene-render']
FOLDER = {'engine-registry': '🗺️engine-registry', 'engine-text': '✍️engine-text', 'engine-board': '🎲️engine-board', 'engine-graph': '🕸️engine-graph', 'engine-attach': '🔗️engine-attach', 'scene-state': '🎭️scene-state', 'canvas2d-host': '🔄️canvas2d-host', 'scene-canvas2d': '🎨️scene-canvas2d', 'scene-ink': '✒️scene-ink', 'scene-feeds': '📰️scene-feeds', 'scene-tables': '🧮️scene-tables', 'scene-text-vfs': '📁️scene-text-vfs', 'icon-host': '💠️icon-host', 'scene-render': '🪞️scene-render'}
FILES = {
    'engine-registry': [EC + '/🦀️.rs', EC + '/🗂️engine-surface-registry/🦀️.rs', EC + '/📦️prepared-engine-canvas/🦀️.rs', EC + '/🎓️tutorial-surface-geometry/🦀️.rs'],
    'engine-text': [EC + '/✍️text-editor/🦀️.rs'],
    'engine-board': [EC + '/🎲️board2d/🦀️.rs', EC + '/🖌️paint2d/🦀️.rs', EC + '/🗺️tiled-map/🦀️.rs'],
    'engine-graph': [EC + '/🕸️node-graph/🦀️.rs'],
    'engine-attach': [EC + '/🧩️engine-surface-attach/🦀️.rs'],
    'scene-state': [SC + '/🦀️.rs', SC + '/🗃️scene-state/🦀️.rs', SC + '/🪣️pending-raster/🦀️.rs', SC + '/📷️scene-camera-dispatch/🦀️.rs'],
    'canvas2d-host': [CH + '/🦀️.rs'],
    'scene-canvas2d': [SC + '/🖌️canvas2d/🦀️.rs'],
    'scene-ink': [SC + '/✒️ink-canvas-model-state/🦀️.rs', SC + '/🖋️ink-canvas-render/🦀️.rs'],
    'scene-feeds': [SC + '/📰️event-feed-and-timeline/🦀️.rs', SC + '/🧭️node-graph-and-tiled-map/🦀️.rs'],
    'scene-tables': [SC + '/🧱️block-list-and-diff/🦀️.rs', SC + '/🧮️table-and-paint2d/🦀️.rs'],
    'scene-text-vfs': [SC + '/🧩️board2d-and-vfs/🦀️.rs', SC + '/✍️text-editor/🦀️.rs'],
    'icon-host': [IC + '/🦀️.rs', IC + '/📤️export/🦀️.rs', IC + '/📤️export/🎬️scene/🦀️.rs', IC + '/📤️export/📐️svg/🦀️.rs', IC + '/📤️export/🧊️gpu/🦀️.rs', IC + '/📤️export/📷️png/🦀️.rs', SC + '/🖼️icon-render/🦀️.rs'],
    'scene-render': [SC + '/⌨️scene-input/🦀️.rs', SC + '/🎨️scene-render-entry/🦀️.rs'],
}
ASSET_AUTHORITY_FILE = IC + '/📤️export/📥️asset/🦀️.rs'

def strip(t):
    o = []; i = 0; n = len(t)
    while i < n:
        c = t[i]
        if t.startswith('//', i):
            j = t.find('\n', i); i = n if j < 0 else j; continue
        if t.startswith('/*', i):
            d = 1; i += 2
            while i < n and d:
                if t.startswith('/*', i): d += 1; i += 2
                elif t.startswith('*/', i): d -= 1; i += 2
                else:
                    if t[i] == '\n': o.append('\n')
                    i += 1
            continue
        m = re.match(r'(?:br|rb|r|cr)(#*)"', t[i:i+12]) if c in 'rbc' and (i == 0 or not (t[i-1].isalnum() or t[i-1] == '_')) else None
        if m:
            e = '"' + m.group(1); j = t.find(e, i + m.end()); j = n if j < 0 else j + len(e)
            o.append('""' + '\n' * t[i:j].count('\n')); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            o.append('""' + '\n' * t[i:j].count('\n')); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", t[i:i+12])
            if m: o.append("''"); i += m.end(); continue
            m = re.match(r"'[A-Za-z_]\w*", t[i:])
            if m: o.append(' '); i += m.end(); continue
        o.append(c); i += 1
    return ''.join(o)

HEAD = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use)(?:\b|(?<=!))\s*([A-Za-z_<][\w:<>, ]*)?')

def items(raw):
    """Column-0 items of a Rust file: list of dicts (start, end, kind, name, text) with docs/attrs included."""
    L = strip(raw).split('\n'); R = raw.split('\n'); n = len(L)
    depth = 0; starts = []
    for l in L:
        starts.append(depth); depth += l.count('{') - l.count('}')
    out = []; ln = 0
    while ln < n:
        l = L[ln]
        m = HEAD.match(l) if starts[ln] == 0 and l and not l[0].isspace() and l[0] not in '}#' else None
        if m:
            d = 0; seen = False; j = ln
            while j < n:
                d += L[j].count('{') - L[j].count('}'); seen = seen or '{' in L[j]
                if (seen and d <= 0) or (not seen and L[j].rstrip().endswith(';')): break
                j += 1
            s = ln
            while s > 0 and R[s-1].lstrip().startswith(('///', '#[', '//!', '//#region', '//#endregion', '// #region', '// #endregion')): s -= 1
            kind = m.group(1); name = (m.group(2) or '').strip()
            if kind == 'impl': name = l.strip()
            out.append(dict(start=s, end=j, kind=kind, name=name, head=ln))
            ln = j + 1
        else: ln += 1
    return out

def item_name(it):
    if it['kind'] == 'impl': return None
    nm = re.match(r'[A-Za-z_]\w*', it['name'])
    return nm.group(0) if nm else None
