"""🗺️ The WP3 crate map: which source file (module) belongs to which crate of the renderer partition, in the staging layout of the old monolith. Single source of truth for the census, the flip and the ownership gate."""
import json, os, sys
ROOT = os.environ['ROOT']; E = os.environ['E']; T = os.environ['T']; R = os.environ['R']; V = '️'
I = E + f'/🧱{V}elements/🗣{V}Interpreter/🎯{V}targets/🧊{V}wgpu'
def rs(base, *parts): return os.path.join(base, *[p.replace('@', V) for p in parts], f'🦀{V}.rs')
SUFFIX_FOLDER = {
 'ui-engine-focus': '👆@ui-engine-focus', 'ui-engine-dump': '🔎@ui-engine-dump', 'ui-engine': '🖱@ui-engine', 'host-plumbing': '🔧@host-plumbing',
 'frame-actions': '🛎@frame-actions', 'runtime-mailbox': '📨@runtime-mailbox', 'app-runtime': '🏃@app-runtime', 'frame-presentation': '📺@frame-presentation',
 'frame-transaction': '🧾@frame-transaction', 'frame-job': '👷@frame-job', 'native-probe': '🔬@native-probe', 'host-winit': '🎪@host-winit', 'browser-worker': '🌍@browser-worker',
}
MAP = {
 'ui-engine-focus': [rs(I, '✍@text-and-table-focus'), rs(I, '🖋@ink-and-vfs-focus'), rs(I, '🧩@retained-ui-engine', '🎬@scene-intents-and-clipboard')],
 'ui-engine-dump': [rs(I, '🔬@introspection'), rs(I, '🎯@chrome-ledger'), rs(I, '🧱@mesh-and-board2d-stats')],
 'ui-engine': [rs(I), rs(I, '🧩@retained-ui-engine'), rs(I, '🖱@scene-interaction'), rs(I, '📄@retained-document-consumer'), rs(I, '🧮@render-plan-validator')],
 'host-plumbing': [rs(T, '⏰@deadlines'), rs(T, '🎟@native-caller'), rs(T, '📸@render-snapshot'), rs(T, '📐@surface-lane'), rs(T, '🎮@input-wire'), rs(T, '🌐@browser-worker', '📤@output')],
 'frame-actions': [rs(R, '🛎@frame-actions'), rs(R, '🛎@frame-actions', '🎮@interaction-state')],
 'runtime-mailbox': [rs(R, '🎨@presentation-raster'), rs(R, '🎨@presentation-raster', '🌐@native-asset-stream'), rs(R, '📮@mailbox-impl'), rs(R, '🛎@frame-actions', '🔁@runtime-apply')],
 'app-runtime': [rs(R, '🧠@app-runtime'), rs(R, '🧾@frame-build'), rs(R, '🧾@frame-build', '📋@frame-preparation')],
 'frame-transaction': [rs(R, '🧾@frame-build', '🧮@frame-transaction')],
 'frame-presentation': [rs(R, '🖥@frame-presentation')],
 'frame-job': [rs(T, '🧵@frame-job'), rs(E, '🪙@authority', '🖼@frame')],
 'native-probe': [rs(R, '🔬@native-socket-probe'), rs(R, '🪶@sqlite')],
 'host-winit': [rs(T, '🏠@os-host'), rs(T, '🪟@winit-app')],
 'browser-worker': [rs(T, '🌐@browser-worker')],
}
LIFTS = {  # files lifted into other packages' crates; they are mounted by the source module until the destination wires them
 'contracts': [rs(T, '🤝@contracts', '📥@lifted', '🧩@retained-ui-engine'), rs(T, '🤝@contracts', '📥@lifted', '🖼@ui-image-loading')],
 'scene-state': [rs(T, '🎭@scene-state', '📥@lifted', '🎯@chrome-ledger')],
 'host-base': [rs(T, '🏛@host-base', '📥@lifted', '🌐@browser-worker'), rs(T, '🏛@host-base', '📥@lifted', '🎨@presentation-raster')],
 'scene-canvas2d': [rs(I, '🖼@ui-image-loading')],
}
def owners():
    o = {}
    for k, files in list(MAP.items()) + list(LIFTS.items()):
        for f in files:
            assert f not in o or o[f] == k, ('file owned twice', f)
            o[f] = k
    return o
if __name__ == '__main__':
    miss = [f for fs in list(MAP.values()) + list(LIFTS.values()) for f in fs if not os.path.exists(f)]
    print('missing', miss)
    out = {k: [os.path.relpath(f, ROOT) for f in v] for k, v in list(MAP.items()) + list(LIFTS.items())}
    json.dump(out, open(os.path.dirname(os.path.abspath(__file__)) + '/crate-map.json', 'w', encoding='utf8'), ensure_ascii=False, indent=1)
    print({k: len(v) for k, v in out.items()})
