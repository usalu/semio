# 🧭️ The frozen crate table of the wgpu renderer partition (WP0): names, folders, work packages, cfg, first-party and external dependencies.
# Sources: PLAN-RENDERER-WGPU-CRATES.md sections 6.1, 6.2, 7.3, 7.7 and Appendix D (parsed by planmodel.py), shelledges.py (field-touch evidence), the folder emoji choices below.
import json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = HERE + '/../🗑️generated'
R = '/Users/ueli/Documents/semio'
T_REL = '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu'
T = R + '/' + T_REL
PKG_PREFIX = 'semio-framework-os-renderer-wgpu-'
ID_PREFIX = 'os-renderer-wgpu-'
TOP = 'semio-framework-os-renderer-wgpu'
FE0F = '️'

EMOJI = {
    'contracts': '🤝', 'host-base': '🏛', 'native-platform': '🖥', 'media-slots': '🎥', 'canvas2d-paint': '🖌', 'asset-authority': '🗂', 'asset-decode': '🦴', 'doors': '🛂',
    'kernel-protocol': '📡', 'kernel-retained': '🗃', 'kernel-pool': '🏊', 'kernel-runtime': '🧠', 'engine-registry': '🗺', 'engine-text': '✍', 'engine-board': '🎲', 'engine-graph': '🕸',
    'engine-attach': '🔗', 'scene-state': '🎭', 'canvas2d-host': '🔄', 'scene-canvas2d': '🎨', 'scene-ink': '✒', 'scene-feeds': '📰', 'scene-tables': '🧮', 'scene-text-vfs': '📁',
    'icon-host': '💠', 'scene-render': '🪞', 'ui-engine-focus': '👆', 'ui-engine-dump': '🔎', 'ui-engine': '🖱', 'hub-contracts': '🔐', 'hub-workspace': '🏘', 'agent-elements': '🙋',
    'dock': '🛰', 'program-bridge': '🪢', 'mailbox-core': '📬', 'host-plumbing': '🔧', 'frame-actions': '🛎', 'runtime-mailbox': '📨', 'app-runtime': '🏃', 'frame-presentation': '📺',
    'frame-transaction': '🧾', 'frame-job': '👷', 'native-probe': '🔬', 'host-winit': '🎪', 'browser-worker': '🌍',
    'shell-vocab': '📖', 'shell-async': '⏳', 'shell-chrome-state': '🎚', 'shell-surfaces': '🫧', 'shell-plugins': '🧲', 'shell-history': '⏪', 'shell-prefs': '🎛', 'shell-directory': '📒',
    'shell-sync': '🔃', 'shell-docs': '📄', 'shell-doc-open': '📂', 'shell-windows': '🔲', 'shell-dock': '⚓', 'shell-input': '🕹', 'shell-commands': '🎹', 'shell-state': '🐙',
    'shell-session': '🚶', 'shell-window-actions': '🪝', 'shell-actions': '🔀', 'shell-pointer': '🖐', 'shell-view-paint': '🖍', 'shell-view-chrome': '🍱', 'shell-view-overlays': '🗨', 'shell-view-panels': '🪀',
    'engine-attach-tests': '🧷', 'scene-ink-tests': '🧶', 'ui-engine-tests': '🧿', 'runtime-mailbox-tests': '🪤', 'test-support': '🧯',
    'shell-tests-windows-1': '🏙', 'shell-tests-windows-2': '🌆', 'shell-tests-chrome-1': '🌈', 'shell-tests-chrome-2': '🌌', 'shell-tests-chrome-3': '🌅', 'shell-tests-chrome-4': '🌴',
    'shell-tests-chrome-5': '🌲', 'shell-tests-chrome-6': '🍃', 'shell-tests-input-1': '🍎', 'shell-tests-input-2': '🍯', 'shell-tests-input-3': '🍰',
    'shell-tests-docs-1': '🥧', 'shell-tests-docs-2': '🥪', 'shell-tests-docs-3': '🥣',
}
DESC = {
    'contracts': 'Cycle-breaking contracts shared by the renderer elements: admitted-surface map, retained-UI cell, element contracts, find-item vocabulary',
    'host-base': 'Host statics, boot descriptor, logging, plugin catalog, runtime assets, mesh assets, frame latency and renderer I/O of the wgpu renderer',
    'native-platform': 'Native accessibility bridge and temporal adapters of the wgpu renderer',
    'media-slots': 'Bounded media slot descriptors of the wgpu renderer',
    'canvas2d-paint': 'Path geometry and paint of the Canvas2d host',
    'asset-authority': 'Renderer asset I/O authority and exact-request asset delivery',
    'asset-decode': 'GLB decoder and renderer asset decode jobs',
    'doors': 'Duplex socket door and directory door transports',
    'kernel-protocol': 'Kernel request, replay and job-progress protocol with the parallel runtime of the native kernel',
    'kernel-retained': 'Retained document exchange of the native kernel pool',
    'kernel-pool': 'Kernel pool state, client and loop of the native renderer',
    'kernel-runtime': 'Kernel runtime facade and the scale bench of the native renderer',
    'engine-registry': 'Engine surface registry, prepared engine canvas and tutorial surface geometry',
    'engine-text': 'Text editor engine surface',
    'engine-board': 'Board2d, paint2d and tiled map engine surfaces',
    'engine-graph': 'Node graph engine surface',
    'engine-attach': 'Engine surface attach registry and per-kind scene sync',
    'scene-state': 'Scene state, pending raster, camera dispatch and the shared scene kit',
    'canvas2d-host': 'Canvas2d gumball host dispatch',
    'scene-canvas2d': 'Canvas2d scene and the retained UI image loader',
    'scene-ink': 'Ink canvas scene model, state and render',
    'scene-feeds': 'Event feed, timeline, node graph and tiled map scenes',
    'scene-tables': 'Table, paint2d, block list and diff scenes',
    'scene-text-vfs': 'Board2d, virtual file system and text editor scenes',
    'icon-host': 'Icon atlas, icon export tree and icon render scene',
    'scene-render': 'Scene input wire and component scene render entry',
    'ui-engine-focus': 'Retained UI focus statics, key handlers, scene intents and clipboard IO',
    'ui-engine-dump': 'Retained UI introspection, chrome ledger and mesh and board statistics',
    'ui-engine': 'Retained UI engine dispatch, document consumer, scene interaction and render plan validator',
    'hub-contracts': 'Hub sign-in and space browser contract twins',
    'hub-workspace': 'Hub connection, space administration and canvas presence',
    'agent-elements': 'Agent bridge, presence, approvals, chat panel and the bridge door',
    'dock': 'Dock state, layout, stack chrome and split resize',
    'program-bridge': 'Program bridge entry, plugin filtering and loading, native runtime modules',
    'mailbox-core': 'Target-neutral bounded completion queue and interaction checkout ledger',
    'host-plumbing': 'Surface lane, input wire, render snapshot, deadlines, native caller and browser output adapters',
    'frame-actions': 'Frame action envelopes, receipts, owners and deferred cursors',
    'runtime-mailbox': 'Generic runtime mailbox, presentation and raster authorities, native asset transport and runtime apply',
    'app-runtime': 'App runtime, interaction state, frame engine packets and build cursors',
    'frame-presentation': 'App presenter, present stall watch and surface context',
    'frame-transaction': 'Frame transaction and boot runtime',
    'frame-job': 'Frame build job, handle and completion wake',
    'native-probe': 'Native socket probe, smoke runner and sqlite probe',
    'host-winit': 'OS host and winit application handler',
    'browser-worker': 'Browser renderer worker and bootstrap',
    'shell-vocab': 'Shell chrome strings, language axes, labels and refusal vocabulary',
    'shell-async': 'Shell detached tasks, pool futures, runners and import and export transfers',
    'shell-chrome-state': 'ChromeState component of the shell: notices, dialogs, tour, tooltips and agent approvals',
    'shell-surfaces': 'SurfaceState, TutorialState and SettleState components of the shell',
    'shell-plugins': 'PluginState component of the shell with panel records and projections',
    'shell-history': 'HistoryState component of the shell: time travel and local folders',
    'shell-prefs': 'PrefsState component of the shell: preferences, themes, drivers and storage door',
    'shell-directory': 'DirectoryState component of the shell: identity, directory, hub workspace and presence',
    'shell-sync': 'SyncState component of the shell: sync backbone, reseed and hub check-in',
    'shell-docs': 'DocumentState component of the shell: sessions, window documents and retirement',
    'shell-doc-open': 'OpeningState component of the shell: document opening, import transfers and icon export',
    'shell-windows': 'WindowState component of the shell: active window, utilities, topology and measures',
    'shell-dock': 'ShellDockState component of the shell: dock model, tabs, panel anchors and layouts',
    'shell-input': 'ShellInputState component of the shell: pointer, selection, context menu and retained hits',
    'shell-commands': 'CommandState component of the shell: search, find, staged commands and keybindings',
    'shell-state': 'The ShellState composer of the fifteen shell components',
    'shell-session': 'Shell use-cases: session, topology, persistence, time travel and tutorial runtime',
    'shell-window-actions': 'Shell use-cases: window actions, command bridge, refresh and boot',
    'shell-actions': 'Shell use-cases: action dispatch, document open and sync and directory pumps',
    'shell-pointer': 'Shell use-cases: pointer, keyboard, accessibility events and the settle driver',
    'shell-view-paint': 'Shell view paint helpers: chrome control presentation, accessibility roles and chrome groups',
    'shell-view-chrome': 'Shell chrome views: navbar, footer, panels, tooltips and tutorial bar',
    'shell-view-overlays': 'Shell overlay views: dialogs, context menus, tour, bands and hub workspace',
    'shell-view-panels': 'Shell panel views: main window, footer, accessibility nodes and icon export',
    'engine-attach-tests': 'Companion tests of engine-attach: engine surfaces, paint2d, board2d, node graph and gestures',
    'scene-ink-tests': 'Companion tests of scene-ink: ink canvas, raster frame cost and the standalone slice',
    'ui-engine-tests': 'Companion tests of ui-engine: engine surfaces, render plan validator, app catalogue and worker cell',
    'runtime-mailbox-tests': 'Companion tests of runtime-mailbox: async boundary and enqueue',
    'test-support': 'Source-text law support of the renderer tests: reads a module tree at run time instead of embedding it',
}
BIN_THEMES = {'windows': 'windows and panels', 'chrome': 'chrome and views', 'input': 'input and pointer', 'docs': 'documents, sync and directory'}

def folder(key): return EMOJI[key] + FE0F + key
def pkg_name(key): return PKG_PREFIX + key
def crate_id(key): return ID_PREFIX + key
def project_name(key): return '@semio-tech/' + ID_PREFIX + key + '-rs'
def crate_dir(key): return T + '/' + folder(key)
def pkg_dir(key): return crate_dir(key) + '/📦️packages/🦀️rust'
def rel_root(path): return os.path.relpath(path, R)

# rank order (PLAN section 4.3 with the shell crates in the order of 7.3 and 7.7)
SHELL_T1 = ['shell-chrome-state', 'shell-surfaces', 'shell-plugins', 'shell-history', 'shell-prefs', 'shell-directory', 'shell-sync', 'shell-docs', 'shell-doc-open', 'shell-windows', 'shell-dock', 'shell-input', 'shell-commands']
SHELL_T23 = ['shell-session', 'shell-window-actions', 'shell-actions', 'shell-pointer', 'shell-view-paint', 'shell-view-chrome', 'shell-view-overlays', 'shell-view-panels']
SHELL_ORDER = ['shell-vocab', 'shell-async'] + SHELL_T1 + ['shell-state'] + SHELL_T23
RANK_ORDER = ['contracts', 'host-base', 'native-platform', 'media-slots', 'canvas2d-paint', 'asset-authority', 'asset-decode', 'doors', 'kernel-protocol', 'kernel-retained', 'kernel-pool', 'kernel-runtime',
    'engine-registry', 'engine-text', 'engine-board', 'engine-graph', 'engine-attach', 'scene-state', 'canvas2d-host', 'scene-canvas2d', 'scene-ink', 'scene-feeds', 'scene-tables', 'scene-text-vfs', 'icon-host',
    'scene-render', 'ui-engine-focus', 'ui-engine-dump', 'ui-engine', 'hub-contracts', 'hub-workspace', 'agent-elements', 'dock', 'program-bridge'] + SHELL_ORDER + ['mailbox-core', 'host-plumbing', 'frame-actions',
    'runtime-mailbox', 'app-runtime', 'frame-presentation', 'frame-transaction', 'frame-job', 'native-probe', 'host-winit', 'browser-worker', 'entrypoint']
COMPANIONS = ['engine-attach-tests', 'scene-ink-tests', 'ui-engine-tests', 'runtime-mailbox-tests']
BINS = ['shell-tests-windows-1', 'shell-tests-windows-2', 'shell-tests-chrome-1', 'shell-tests-chrome-2', 'shell-tests-chrome-3', 'shell-tests-chrome-4', 'shell-tests-chrome-5', 'shell-tests-chrome-6',
        'shell-tests-input-1', 'shell-tests-input-2', 'shell-tests-input-3', 'shell-tests-docs-1', 'shell-tests-docs-2', 'shell-tests-docs-3']
ALL_KEYS = [k for k in RANK_ORDER if k != 'entrypoint'] + ['test-support'] + COMPANIONS + BINS
NATIVE_ONLY = {'native-platform', 'kernel-protocol', 'kernel-retained', 'kernel-pool', 'kernel-runtime'}
WASM_ONLY = {'browser-worker'}
EVERYWHERE = {'mailbox-core'}

ALIAS = {'infinite_world': 'infinite_canvas', 'dsl': 'semio_framework_os_kernel', 'dsl_core': 'semio_framework_os_kernel', 'protocol': 'semio_framework_os_kernel', 'store': 'semio_framework_os_kernel',
         'store_sync': 'semio_framework_os_kernel', 'camera_storage': 'semio_framework_os_renderer_camera'}
SPECIAL_KEYS = {'ui_wgpu': 'ui_wgpu', 'ui_render': 'ui_render', 'ui_contract': 'ui_contract', 'ui_host': 'ui_host', 'ui_styling': 'ui_styling', 'infinite_canvas': 'infinite_canvas', 'flow': 'flow',
                'framework_editor': 'framework_editor', 'replication': 'replication', 'serde': 'serde', 'serde_json': 'serde_json', 'image': 'image', 'usvg': 'usvg', 'tiny_skia': 'tiny-skia', 'resvg': 'resvg',
                'base64': 'base64', 'winit': 'winit', 'accesskit': 'accesskit', 'accesskit_winit': 'accesskit_winit', 'tokio': 'tokio', 'wasm_bindgen': 'wasm-bindgen', 'web_sys': 'web-sys', 'js_sys': 'js-sys',
                'getrandom': 'getrandom', 'vello': 'vello'}
def dep_key(ident):
    ident = ALIAS.get(ident, ident)
    return SPECIAL_KEYS.get(ident, ident.replace('_', '-'))

def load():
    model = json.load(open(GEN + '/rwp0-planmodel.json'))
    edges = json.load(open(GEN + '/rwp0-shelledges.json'))
    extscan = json.load(open(GEN + '/rwp0-extscan.json'))
    return model, edges, extscan

# curated additions to the plan's 6.2 external lists where the live tree drifted (canvas split, runtime asset port, camera storage)
EXT_ADD = {
    'host-base': ['semio_framework_canvas'],
    'engine-registry': ['semio_framework_canvas'], 'engine-board': ['semio_framework_canvas'], 'engine-graph': ['semio_framework_canvas'], 'engine-attach': ['semio_framework_canvas', 'framework_editor', 'semio_framework_os_node_graph', 'semio_framework_surface'],
    'scene-state': ['semio_framework_os_renderer_camera', 'semio_framework_canvas'],
    'host-winit': ['semio_framework_canvas'],
}
# the shell crates share the union of the identifiers the Shell tree names; each shell executor prunes to what its crate names
SHELL_EXT = ['base64', 'infinite_world', 'js_sys', 'protocol', 'semio_framework', 'semio_framework_actor', 'semio_framework_artifact_reference', 'semio_framework_async', 'semio_framework_io_base64', 'semio_framework_job',
             'semio_framework_os_config', 'semio_framework_os_kernel', 'semio_framework_os_services', 'semio_framework_pack_json', 'semio_framework_plugin_host', 'semio_framework_time_travel', 'semio_framework_tool_run',
             'semio_framework_trace', 'semio_framework_ui_locale', 'semio_framework_ui_viewport', 'semio_framework_value', 'semio_framework_value_derive', 'serde', 'serde_json', 'store', 'store_sync', 'tokio', 'ui_contract',
             'ui_host', 'ui_render', 'ui_styling', 'ui_wgpu', 'wasm_bindgen', 'web_sys']

def build():
    model, shell_edges, extscan = load()
    lib = model['lib']
    crates = {}
    for key in RANK_ORDER:
        if key == 'entrypoint': continue
        if key in lib:
            c = lib[key]
            cfg = 'native' if key in NATIVE_ONLY else 'wasm' if key in WASM_ONLY else 'all' if key in EVERYWHERE else 'both'
            deps = list(c['deps'])
            if c['shell_dep']:
                deps.append('shell-state')
                if key in ('app-runtime', 'frame-transaction', 'browser-worker'): deps.append('shell-input')
            ext = sorted(set(dep_key(i) for i in c.get('external_idents', []) + EXT_ADD.get(key, [])))
            crates[key] = dict(key=key, kind='lib', wp=c['wp'], cfg=cfg, deps=deps, ext=ext, tests=c['tests'] != '0 / 0 / 0', plan_lines=c['lib_lines'])
        else:
            shell = model['shell'][key]
            wp = shell['wp']
            comps = [k for k in SHELL_T1]
            if key in ('shell-vocab',): deps = []
            elif key == 'shell-async': deps = ['host-base']
            elif key in SHELL_T1:
                lower = shell_edges['edges'].get(key, [])
                deps = ['contracts', 'host-base', 'shell-vocab', 'shell-async'] + sorted(lower, key=SHELL_ORDER.index)
            elif key == 'shell-state':
                deps = ['contracts', 'host-base', 'shell-vocab', 'shell-async'] + comps
            else:
                i = SHELL_T23.index(key)
                deps = ['contracts', 'host-base', 'shell-vocab', 'shell-async', 'shell-state'] + comps + ([SHELL_T23[i - 1]] if i else [])
            ext = sorted(set(dep_key(i) for i in SHELL_EXT)) if key not in ('shell-vocab',) else sorted(set(dep_key(i) for i in ['semio_framework_ui_locale', 'serde_json']))
            crates[key] = dict(key=key, kind='shell', wp=wp, cfg='both', deps=deps, ext=ext, tests=True, plan_lines=shell['est_lines'])
    support = dict(key='test-support', kind='support', wp=0, cfg='both', deps=[], ext=[], tests=False)
    crates['test-support'] = support
    def comp(key, wp, deps, ext_from):
        e = set()
        for k in ext_from: e |= set(crates[k]['ext'])
        crates[key] = dict(key=key, kind='companion', wp=wp, cfg='both', deps=deps, ext=sorted(e), tests=False)
    comp('engine-attach-tests', 2, ['engine-attach', 'engine-registry', 'engine-text', 'engine-board', 'engine-graph', 'contracts', 'media-slots', 'canvas2d-paint', 'host-base'], ['engine-attach'])
    comp('scene-ink-tests', 2, ['scene-ink', 'scene-state', 'scene-canvas2d', 'contracts', 'engine-registry', 'engine-board'], ['scene-ink', 'scene-state'])
    comp('ui-engine-tests', 3, ['ui-engine', 'ui-engine-focus', 'ui-engine-dump', 'scene-render', 'scene-state', 'engine-text', 'engine-board', 'contracts', 'host-base'], ['ui-engine', 'ui-engine-focus', 'ui-engine-dump'])
    comp('runtime-mailbox-tests', 3, ['runtime-mailbox', 'mailbox-core', 'frame-actions', 'host-plumbing', 'host-base', 'contracts', 'asset-authority', 'asset-decode', 'engine-registry', 'engine-text', 'engine-board', 'scene-canvas2d', 'ui-engine-focus', 'program-bridge', 'test-support'], ['runtime-mailbox'])
    for key in BINS:
        wp = model['bins'][key]['wp']
        deps = ['contracts', 'host-base', 'shell-vocab', 'shell-async', 'shell-state', 'test-support'] + SHELL_T1 + (SHELL_T23 if wp == 5 else [])
        crates[key] = dict(key=key, kind='shell-tests', wp=wp, cfg='both', deps=deps, ext=sorted(set(dep_key(i) for i in SHELL_EXT)), tests=False)
    for key, c in crates.items():
        c['dir'] = rel_root(crate_dir(key)); c['pkg'] = rel_root(pkg_dir(key)); c['folder'] = folder(key)
        c['name'] = pkg_name(key); c['id'] = crate_id(key); c['project'] = project_name(key)
        c['rank'] = RANK_ORDER.index(key) if key in RANK_ORDER else len(RANK_ORDER) + 1 + ALL_KEYS.index(key)
    return crates

if __name__ == '__main__':
    crates = build()
    print(len(crates), 'crates')
    json.dump(crates, open(GEN + '/rwp0-crates.json', 'w'), ensure_ascii=False, indent=1)
    bad = []
    for k, c in crates.items():
        for d in c['deps']:
            if d not in crates: bad.append((k, d, 'unknown'))
            elif crates[d]['rank'] >= c['rank']: bad.append((k, d, 'rank'))
    print('rank violations / unknown deps:', bad)
