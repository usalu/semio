import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../lib')
import carve, rs, reg, peel
from carve import R, UI, rd
U = '🧰️framework/🔨️modules/🖱️ui'
W = U + '/🎯️targets/🧊️wgpu'
E = U + '/🧱️elements'
def el(d, name): return {'name': name, 'file': '%s/%s/🎯️targets/🧊️wgpu/🦀️.rs' % (E, d)}
crates = {
 'light': {'pkg': 'semio-framework-ui-wgpu-light', 'id': 'ui-wgpu-light', 'dir': W + '/🪶️light', 'desc': 'semio wgpu light tier: minimap geometry, tree window request rule, presence bar and tabs builders',
   'mounts': [{'name': 'minimap', 'file': W + '/🗺️minimap/🦀️.rs'}, {'name': 'tree_window', 'file': E + '/🌳️Tree/🪟️window/🦀️.rs'}, el('👥️PresenceBar', 'presence_bar'), el('📑️Tabs', 'tabs')]},
 'input': {'pkg': 'semio-framework-ui-wgpu-input', 'id': 'ui-wgpu-input', 'dir': W + '/📥️input', 'root': W + '/📥️input/🦀️.rs', 'desc': 'semio wgpu input state: hit registry, pointer and keyboard actions, clipboard intents'},
 'layout': {'pkg': 'semio-framework-ui-wgpu-layout', 'id': 'ui-wgpu-layout', 'dir': W + '/🧱️layout-stack', 'desc': 'semio wgpu layout stack: chrome metrics, layout rules and the flex solver',
   'mounts': [{'name': 'chrome', 'file': W + '/🖥️chrome/🦀️.rs'}, {'name': 'layout', 'file': W + '/🧮️layout/🦀️.rs'}, {'name': 'flex', 'file': W + '/📐️flex/🦀️.rs'}]},
 'mounted': {'pkg': 'semio-framework-ui-wgpu-mounted-layout', 'id': 'ui-wgpu-mounted-layout', 'dir': W + '/📌️mounted_layout', 'root': W + '/📌️mounted_layout/🦀️.rs', 'desc': 'semio wgpu mounted layout: retained layout job, tree row metrics and retained hit registration'},
 'kit': {'pkg': 'semio-framework-ui-wgpu-widget-kit', 'id': 'ui-wgpu-widget-kit', 'dir': W + '/🪀️widgets/🧰️kit', 'root': W + '/🪀️widgets/🧰️kit/🦀️.rs', 'desc': 'semio wgpu widget kit: immediate-mode context, interaction maps, widget node model and text drawing'},
 'controls': {'pkg': 'semio-framework-ui-wgpu-controls', 'id': 'ui-wgpu-controls', 'dir': W + '/🔘️controls', 'desc': 'semio wgpu control renderers: select, slider, stepper, input, button, toggle, key value, ring and icon selector',
   'mounts': [el('🔽️Select', 'select'), el('🎚️Slider', 'slider'), el('🪜️Stepper', 'stepper'), el('✏️Input', 'input_element'), el('🔘️Button', 'button'), el('🔀️Toggle', 'toggle'), el('🔑️KeyValue', 'key_value'), el('⭕️Ring', 'ring'), el('🎴️IconSelector', 'icon_selector')]},
 'events': {'pkg': 'semio-framework-ui-wgpu-events', 'id': 'ui-wgpu-events', 'dir': W + '/⚡️events', 'root': W + '/⚡️events/🦀️.rs', 'desc': 'semio wgpu event router: hit test, capture, focus, overlays, drag and drop, tooltips and commands'},
 'paint': {'pkg': 'semio-framework-ui-wgpu-paint', 'id': 'ui-wgpu-paint', 'dir': W + '/🖌️paint', 'root': W + '/🖌️paint/🦀️.rs', 'desc': 'semio wgpu retained paint: node walk, overlays, select popups, images, skeletons and tooltips'},
 'widgets': {'pkg': 'semio-framework-ui-wgpu-widgets', 'id': 'ui-wgpu-widgets', 'dir': W + '/🪀️widgets', 'root': W + '/🪀️widgets/🦀️.rs', 'desc': 'semio wgpu immediate-mode widget measure and render dispatch with the tree element'},
 'hosting': {'pkg': 'semio-framework-ui-wgpu-hosting', 'id': 'ui-wgpu-hosting', 'dir': W + '/🧭️hosting', 'desc': 'semio wgpu hosting: accessibility projection, cursors, shell, window host and scene slots',
   'mounts': [{'name': 'accessibility', 'file': W + '/♿️accessibility/🦀️.rs'}, {'name': 'cursor', 'file': W + '/👆️cursor/🦀️.rs'}, {'name': 'shell', 'file': W + '/🐚️shell/🦀️.rs'},
              {'name': 'host', 'file': W + '/🏃️host/🦀️.rs', 'cfg': '#[cfg(not(target_os = "wasi"))]'}, {'name': 'scene_slots', 'file': W + '/📨️scene_slots/🦀️.rs'}]},
 'engine_surface': {'pkg': 'semio-framework-ui-wgpu-engine-surface', 'id': 'ui-wgpu-engine-surface', 'dir': W + '/⚙️engine/🪟️surfaces', 'root': W + '/⚙️engine/🪟️surfaces/🦀️.rs', 'desc': 'semio wgpu engine surfaces: windows, tooltips, retained paint walk, surface registry and document ingress'},
 'paintref': {'pkg': 'semio-framework-ui-wgpu-paint-reference', 'id': 'ui-wgpu-paint-reference', 'dir': W + '/🖌️paint/🔬️reference', 'root': W + '/🖌️paint/🔬️reference/🦀️.rs', 'desc': 'semio wgpu immediate paint reference: the golden oracle the retained paint laws compare against'},
 'events-laws': {'pkg': 'semio-framework-ui-wgpu-events-laws', 'id': 'ui-wgpu-events-laws', 'dir': W + '/⚡️events/🧪️tests/🥣️companion', 'root': W + '/⚡️events/🧪️tests/🥣️companion/🦀️.rs', 'desc': 'semio wgpu event router laws: hit test, capture, focus, overlays, drag and drop, keyboard and control commit', 'tests_only': True, 'owner': 'events'},
 'events-control-laws': {'pkg': 'semio-framework-ui-wgpu-events-control-laws', 'id': 'ui-wgpu-events-control-laws', 'dir': W + '/⚡️events/🧪️tests/🎛️companion-control', 'root': W + '/⚡️events/🧪️tests/🎛️companion-control/🦀️.rs', 'desc': 'semio wgpu event router control laws: retained control commit and the scrub press gesture', 'tests_only': True, 'owner': 'events'},
 'engine-scene-laws': {'pkg': 'semio-framework-ui-wgpu-engine-scene-laws', 'id': 'ui-wgpu-engine-scene-laws', 'dir': W + '/⚙️engine/🧪️tests/🎬️companion-scene', 'root': W + '/⚙️engine/🧪️tests/🎬️companion-scene/🦀️.rs', 'desc': 'semio wgpu engine scene laws: hostile retained documents, section collapse, select origin, presented source and physical job close', 'tests_only': True, 'owner': 'engine'},
 'paint-laws': {'pkg': 'semio-framework-ui-wgpu-paint-laws', 'id': 'ui-wgpu-paint-laws', 'dir': W + '/🖌️paint/🧪️tests/🥣️companion', 'root': W + '/🖌️paint/🧪️tests/🥣️companion/🦀️.rs', 'desc': 'semio wgpu retained paint laws: fixtures, steppers, sliders, select popups, carets, grants, progress and skeletons', 'tests_only': True, 'owner': 'paint'},
 'engine-laws': {'pkg': 'semio-framework-ui-wgpu-engine-laws', 'id': 'ui-wgpu-engine-laws', 'dir': W + '/⚙️engine/🧪️tests/🥣️companion', 'root': W + '/⚙️engine/🧪️tests/🥣️companion/🦀️.rs', 'desc': 'semio wgpu engine laws: scene lifetime, frame storm, overlays, goldens, document ingress and presented source', 'tests_only': True, 'owner': 'engine'},
 'input-laws': {'pkg': 'semio-framework-ui-wgpu-input-laws', 'id': 'ui-wgpu-input-laws', 'dir': W + '/📥️input/🧪️tests/🥣️companion', 'root': W + '/📥️input/🧪️tests/🥣️companion/🦀️.rs', 'desc': 'semio wgpu input laws: hit registry, pointer and keyboard actions against the widget kit', 'tests_only': True, 'owner': 'input'},
 'layout-laws': {'pkg': 'semio-framework-ui-wgpu-layout-laws', 'id': 'ui-wgpu-layout-laws', 'dir': W + '/🧱️layout-stack/🧪️tests/🥣️companion', 'root': W + '/🧱️layout-stack/🧪️tests/🥣️companion/🦀️.rs', 'desc': 'semio wgpu layout laws: tree row rectangles and the table row grid against the engine, events and paint', 'tests_only': True, 'owner': 'layout'},
 'mounted-laws': {'pkg': 'semio-framework-ui-wgpu-mounted-layout-laws', 'id': 'ui-wgpu-mounted-layout-laws', 'dir': W + '/📌️mounted_layout/🧪️tests/🥣️companion', 'root': W + '/📌️mounted_layout/🧪️tests/🥣️companion/🦀️.rs', 'desc': 'semio wgpu retained hit target laws against the engine', 'tests_only': True, 'owner': 'mounted'},
 'laws': {'pkg': 'semio-framework-ui-wgpu-laws', 'id': 'ui-wgpu-laws', 'dir': W + '/🧪️tests/🥣️companion', 'root': W + '/🧪️tests/🥣️companion/🦀️.rs', 'desc': 'semio wgpu cross-crate laws: document tree reconcile, conformance corpus, theme token parity, text renderer, prepared engine, prepared raster refusal and clipboard ingress', 'tests_only': True,
   'extra': [W + '/🎟️prepared/♻️refusal/🧪️tests/🦀️.rs', U + '/🧪️tests/📋️clipboard-ingress/🦀️.rs'],
   'targets': [('original_prepared_raster_refusal', W + '/🎟️prepared/♻️refusal/🧪️tests/🦀️.rs'), ('original_clipboard_ingress', U + '/🧪️tests/📋️clipboard-ingress/🦀️.rs')]},
 'engine': {'pkg': 'semio-framework-ui-wgpu-engine', 'id': 'ui-wgpu-engine', 'dir': W + '/⚙️engine', 'root': W + '/⚙️engine/🦀️.rs', 'desc': 'semio wgpu retained-mode engine: the `Ui` facade over surfaces, events, layout and paint'},
}
rules = []
def r(frm, crate, to): rules.append({'from': frm, 'crate': crate, 'to': to})
def rx(frm, ident, to): rules.append({'from': frm, 'ident': ident, 'to': to})
MODS = [('accessibility', 'hosting', 'accessibility'), ('chrome', 'layout', 'chrome'), ('cursor', 'hosting', 'cursor'), ('engine', 'engine', ''), ('events', 'events', ''), ('flex', 'layout', 'flex'), ('host', 'hosting', 'host'),
        ('input', 'input', ''), ('layout', 'layout', 'layout'), ('minimap', 'light', 'minimap'), ('mounted_layout', 'mounted', ''), ('hit', 'mounted', 'hit'), ('paint', 'paint', ''), ('scene_slots', 'hosting', 'scene_slots'),
        ('shell', 'hosting', 'shell'), ('widgets', 'widgets', ''), ('widgets_kit', 'kit', ''), ('engine_surface', 'engine_surface', ''), ('select', 'controls', 'select'), ('button', 'controls', 'button'), ('input_element', 'controls', 'input_element'),
        ('toggle', 'controls', 'toggle'), ('key_value', 'controls', 'key_value'), ('slider', 'controls', 'slider'), ('stepper', 'controls', 'stepper'), ('ring', 'controls', 'ring'), ('icon_selector', 'controls', 'icon_selector'),
        ('tree_element', 'widgets', 'tree_element'), ('tree_window', 'light', 'tree_window'), ('presence_bar', 'light', 'presence_bar'), ('tabs', 'light', 'tabs')]
for m, c, t in MODS: r('wgpu::' + m, c, t)
# item-level overrides from the peels
P = json.load(open(HERE + '/../lists/wgpu-peels.json', encoding='utf8'))
def moved(rep): return sorted({n for k, n in rep['moved'] if n and k != 'impl' and k != 'comment'})
for n in moved(P['input_hit']): r('wgpu::input::' + n, 'mounted', 'hit::' + n)
for n in moved(P['widgets_kit']): r('wgpu::widgets::' + n, 'kit', n)
r('wgpu::widgets::constrain_number_input', 'kit', 'constrain_number_input')
r('wgpu::events::constrain_number_input', 'kit', 'constrain_number_input')
for n in moved(P['engine_surface']): r('wgpu::engine::' + n, 'engine_surface', n)
for n in ('push_stepper_chrome', 'push_stepper_icon'): r('wgpu::paint::' + n, 'controls', 'stepper::' + n)
for n in moved(P.get('paint_reference', {'moved': []})): r('wgpu::paint::' + n, 'paintref', n)
# select behaviour glob re-export
def pub_names(root_file):
    s = rd(root_file); names = set()
    for it in peel.top_items(s):
        if it.kind != 'use' and it.vis.startswith('pub') and it.name and it.name != '_': names.add(it.name)
    for u in rs.find_uses(s):
        if u.vis.startswith('pub'):
            for p, a in rs.flatten_tree(u.tree):
                b = a or p.split('::')[-1]
                if b != '*': names.add(b)
    return names
beh = pub_names(UI + '/🧱️elements/🔽️Select/🧭️behavior/🦀️.rs')
for n in sorted(beh): rx('wgpu::select::' + n, 'semio_framework_ui_select_behavior', n)
# root namespace
root = open(HERE + '/../lists/wgpu-old-root.rs.txt', encoding='utf8').read()
alias = {}
leaves = []
for u in rs.find_uses(root):
    for p, a in rs.flatten_tree(u.tree): leaves.append((p, a, u.vis))
EXTKEY = {}   # canonical ident -> dependency ident as written in the ui manifest
for key in reg.EXT:
    EXTKEY[key.replace('-', '_')] = key.replace('-', '_')
def canon(ident):
    return ident
firsts = {p.split('::')[0] for p, a, v in leaves if '::' in p}
for p, a, v in leaves:
    segs = p.split('::')
    b = a or segs[-1]
    if segs[0] in EXTKEY and b in firsts:
        alias[b] = (segs[0], segs[1:])
mod_names = {m for m, c, t in MODS}
unresolved = []
glob_sources = {}
for p, a, v in leaves:
    segs = p.split('::'); b = a or segs[-1]
    if b in alias and (segs[0] in EXTKEY): 
        ident, sub = alias[b]
        rx('wgpu::' + b, ident, '::'.join(sub)); continue
    if segs[0] in alias:
        ident, sub = alias[segs[0]]
        rest = segs[1:]
        if b == '*': glob_sources[p] = (ident, sub + rest[:-1]); continue
        rx('wgpu::' + b, ident, '::'.join(sub + rest)); continue
    if segs[0] in mod_names:
        rest = segs[1:]
        mod = segs[0]
        rule = carve.Rules([x for x in rules]).lookup(['wgpu', mod] + rest)
        # compose: module rule + rest
        mrule = [x for x in rules if x['from'] == 'wgpu::' + mod][0]
        tgt = rule[1] if rule else mrule
        frm = tuple(tgt['from'].split('::'))
        consumed = len(frm) - 1
        to = [t for t in tgt['to'].split('::') if t] + (['wgpu'] + [mod] + rest)[len(frm):]
        r('wgpu::' + b, tgt['crate'] if 'crate' in tgt else None, '::'.join(to)) if 'crate' in tgt else rx('wgpu::' + b, tgt['ident'], '::'.join(to)); continue
    if segs[0] in EXTKEY and len(segs) >= 2:
        rx('wgpu::' + b, segs[0], '::'.join(segs[1:])); continue
    unresolved.append(p)
# glob: component::ui::*
for p, (ident, path) in glob_sources.items():
    if ident == 'semio_framework_ui_declaration' and path == ['ui']:
        names = pub_names(UI + '/📜️declaration/🧩️ui-graph/🦀️.rs')
        for n in sorted(names): rx('wgpu::' + n, ident, 'ui::' + n)
    else: unresolved.append('GLOB ' + p)
# de-dup preserving first (explicit leaves win over glob: lookup picks longest/first by sort stability)
seen = set(); out = []
for x in rules:
    k = (x['from'])
    if k in seen: continue
    seen.add(k); out.append(x)
spec = {'tree': 'wgpu', 'roots': ['crate'], 'crates': crates, 'rules': out, 'unresolved': unresolved, 'alias': {k: v for k, v in alias.items()}}
json.dump(spec, open(HERE + '/wgpu.json', 'w'), indent=1, ensure_ascii=False)
print('rules', len(out), 'unresolved', unresolved[:20], 'aliases', list(alias))
