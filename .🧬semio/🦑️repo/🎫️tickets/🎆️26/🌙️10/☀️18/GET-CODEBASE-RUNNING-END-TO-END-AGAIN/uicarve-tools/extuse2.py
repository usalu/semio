import sys, os, collections, re
sys.path.insert(0, 'lib')
import carve, rs, crategen
from carve import R, UI
W = UI + '/🎯️targets/🧊️wgpu'
units = {}
for u, d in {'accessibility': '♿️accessibility', 'chrome': '🖥️chrome', 'cursor': '👆️cursor', 'engine': '⚙️engine', 'events': '⚡️events', 'flex': '📐️flex', 'host': '🏃️host', 'input': '📥️input', 'layout': '🧮️layout', 'minimap': '🗺️minimap', 'mounted_layout': '📌️mounted_layout', 'paint': '🖌️paint', 'scene_slots': '📨️scene_slots', 'shell': '🐚️shell', 'widgets': '🪀️widgets'}.items():
    units[u] = W + '/' + d + '/🦀️.rs'
E = UI + '/🧱️elements/'
for u, d in {'select': '🔽️Select/🎯️targets/🧊️wgpu', 'button': '🔘️Button/🎯️targets/🧊️wgpu', 'input_element': '✏️Input/🎯️targets/🧊️wgpu', 'toggle': '🔀️Toggle/🎯️targets/🧊️wgpu', 'key_value': '🔑️KeyValue/🎯️targets/🧊️wgpu', 'slider': '🎚️Slider/🎯️targets/🧊️wgpu', 'stepper': '🪜️Stepper/🎯️targets/🧊️wgpu', 'ring': '⭕️Ring/🎯️targets/🧊️wgpu', 'icon_selector': '🎴️IconSelector/🎯️targets/🧊️wgpu', 'tree_element': '🌳️Tree/🎯️targets/🧊️wgpu', 'presence_bar': '👥️PresenceBar/🎯️targets/🧊️wgpu', 'tabs': '📑️Tabs/🎯️targets/🧊️wgpu', 'tree_window': '🌳️Tree/🪟️window'}.items():
    units[u] = E + d + '/🦀️.rs'
std = {'std', 'core', 'alloc', 'crate', 'self', 'super', 'Self'}
for u, f in units.items():
    ids = collections.Counter()
    for ff in carve.reachable(f):
        if not ff.endswith('.rs') or '/🧪️tests/' in ff: continue
        s = carve.rd(ff); m = rs.mask(s)
        for x in re.finditer(r'(?<![\w:.])([a-z_][a-z0-9_]*)::', m): ids[x.group(1)] += 1
    ext = {k: v for k, v in ids.items() if k not in std and (k.startswith(('semio', 'ui_')) or k in ('wgpu', 'winit', 'taffy', 'bytemuck', 'arboard', 'js_sys', 'web_sys', 'wasm_bindgen', 'serde', 'serde_json', 'parley', 'swash', 'tiny_skia', 'unicode_width'))}
    print(u, dict(ext))
