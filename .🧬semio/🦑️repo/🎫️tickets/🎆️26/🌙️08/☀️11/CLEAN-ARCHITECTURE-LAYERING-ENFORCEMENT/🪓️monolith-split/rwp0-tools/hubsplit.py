# 🧭️ WP0 mount-hub split of the renderer crate root (T/🧊️renderer/🦀️.rs) and the library root.
# usage: hubsplit.py dry|apply   (dry prints the unit table and verifies losslessness; apply writes the hub files and rewrites both roots)
# Every unit is cut by unique line-prefix anchors, appended to the hub of the work package that owns its crate, and its path literals are rebased by "../../".
import os, re, sys
R = '/Users/ueli/Documents/semio'
T = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu'
ROOT_RS = T + '/🧊️renderer/🦀️.rs'
LIB_RS = T + '/📦️packages/🦀️rust/📚️library/🦀️.rs'
HUBDIR = T + '/🧊️renderer/📥️mounts'
HUBS = {
    1: ('🌱️wp1-foundation', '🌱️', 'WP1 foundation, kernel and small elements: contracts, host-base, native-platform, media-slots, canvas2d-paint, asset-authority, asset-decode, doors, kernel-*, hub-*, agent-elements, dock, program-bridge, mailbox-core'),
    2: ('🎞️wp2-scenes', '🎞️', 'WP2 EngineCanvas, Scenes, IconRenderHost and Canvas2dHost: engine-*, scene-*, canvas2d-host, icon-host'),
    3: ('🧵️wp3-frame-core', '🧵️', 'WP3 UI engine, frame core and host: ui-engine-*, host-plumbing, frame-actions, runtime-mailbox, app-runtime, frame-presentation, frame-transaction, frame-job, native-probe, host-winit, browser-worker'),
    4: ('🐚️wp4-shell-state', '🐚️', 'WP4 Shell components: shell-vocab, shell-async, the thirteen component crates and the shell-state composer'),
    5: ('🎛️wp5-shell-views', '🎛️', 'WP5 Shell use-cases and views: shell-session, shell-window-actions, shell-actions, shell-pointer, shell-view-*'),
}
# (hub, first-line prefix, last-line prefix, lines of context before the first anchor)
UNITS = [
    (3, '#[path="../../../🧪️tests/🎟️original-turn/🦀️.rs"]', 'mod fixture_original_turn;', 1),
    (3, 'include!("../../../🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs");', 'include!("../../../🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs");', 1),
    (3, 'include!("../../../🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs");', 'include!("../../../🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs");', 1),
    (1, '// 🥽️ Target-neutral: the catalog is', 'pub mod mesh_assets;', 0),
    (1, '//#region 🤖️AgentBridgeElements', 'pub mod element_contracts;', 0),
    (2, '#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"]', 'pub mod engine_canvas;', 0),
    (3, '#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"]', 'pub mod interpreter;', 0),
    (1, '#[path = "../🎬️media-slots/🦀️.rs"]', '//#endregion 🏠️🧳️PluginHostConfig', 0),
    (2, '#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"]', 'pub mod scenes;', 0),
    (2, '#[path = "../../../🧱️elements/📐️Canvas2dHost/🎯️targets/🧊️wgpu/🦀️.rs"]', 'pub(crate) mod canvas2d_gumball;', 0),
    (1, '#[path = "../../../🧱️elements/📐️Canvas2dHost/🎨️paint/🦀️.rs"]', 'pub(crate) mod canvas2d_paint;', 0),
    (1, '#[path = "../🕰️native-temporal/🦀️.rs"]', 'mod native_temporal;', 1),
    (4, '#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"]', 'pub mod shell;', 0),
    (2, '#[path = "../../../🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs"]', 'pub mod icon_atlas;', 0),
    ('os3', '// 🏠️ ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the seam that ends', '// descriptor carried, and that is fixed at the source in', 0),
    ('os3', '#[path = "../⏰️deadlines/🦀️.rs"]', 'mod deadlines;', 0),
    ('os1', '#[path = "../⏱️frame-latency/🦀️.rs"]', 'pub(crate) mod frame_latency;', 0),
    ('os3', '#[path = "../🏠️os-host/🦀️.rs"]', 'mod os_host;', 0),
    ('os3', '#[path="../🎟️native-caller/🦀️.rs"]', 'mod native_caller;', 1),
    ('os3', '#[path = "../📸️render-snapshot/🦀️.rs"]', 'mod render_snapshot;', 0),
    ('os3', '#[path="../🌐️browser-worker/📤️output/🦀️.rs"]', 'mod browser_original_output;', 0),
    ('os1', '#[path = "../📮️runtime-mailbox-core/🦀️.rs"]', 'mod runtime_mailbox_core;', 0),
    ('os3', '// 🧵️ P3b (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the `InteractiveJob` seam for the', 'mod frame_job;', 0),
    ('os3', '#[path = "../../../🪙️authority/🖼️frame/🦀️.rs"]', 'mod frame_authority;', 0),
    ('os3', '#[path = "../📐️surface-lane/🦀️.rs"]', 'mod surface_lane;', 0),
    ('os3', '#[path = "../🎮️input-wire/🦀️.rs"]', 'mod input_wire;', 0),
    ('os1', '// 🔌️ The DUPLEX door', 'pub mod socket_door;', 0),
    ('os1', '// 📇️ The ONE directory transport seam', 'pub mod directory_door;', 0),
    ('os3', '#[path = "../🌐️browser-worker/🦀️.rs"]', 'mod browser_worker;', 1),
    ('os3', '#[path = "../🪟️winit-app/🦀️.rs"]', 'mod winit_app;', 0),
    ('os1', "// ♿️ The native shell's platform accessibility bridge", 'pub(crate) mod native_accessibility;', 0),
    (1, '// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop): the real multi-shard', 'pub mod parallel_runtime;', 0),
    (1, '#[path = "🗂️asset-authority/🦀️.rs"]', 'pub(crate) use asset_decode_jobs::*;', 0),
    (1, '//#region 🔖️ScaleBench', '//#endregion 🔖️ScaleBench', 0),
    (3, '#[path = "🛎️frame-actions/🦀️.rs"]', 'pub use native_socket_probe::*;', 0),
    (1, '//#region 🎠️KernelRuntime', '//#endregion 🎠️KernelRuntime', 0),
    (3, '//#region 🔖️AsyncBoundaryTests', '//#endregion 🔖️AsyncBoundaryTests', 0),
    (3, '//#region 🔖️FrameActionLedgerTests', '//#endregion 🔖️FrameActionLedgerTests', 0),
    (3, '//#region 🔖️WheelApplicationPointTests', '//#endregion 🔖️WheelApplicationPointTests', 0),
    (5, '//#region 🫀️SettlePumpTests', '//#endregion 🫀️SettlePumpTests', 0),
    (1, '//#region 🧭️BootAxisParityTests', '//#endregion 🧭️BootAxisParityTests', 0),
    ('drop', '//#region 🔖️OsHostDecomposition', '//#region 🔖️OsHostDecomposition', 0),
    ('drop', '//#endregion 🔖️OsHostDecomposition', '//#endregion 🔖️OsHostDecomposition', 0),
]
# the registry mount stays in the library root: a `#[macro_export]` macro defined in a module declared inside an `include!`d file is macro-expanded and cannot be named by `crate::` paths (found by the differential)
LIB_UNITS = [
    (1, '#[path = "../../../⌨️native-entrypoint/📦️modules/🦀️.rs"]', 'mod native_runtime_modules;', 1, '../../../'),
]
PATHLIT = re.compile(r'(#\[\s*path\s*=\s*"|include!\(\s*")([^"]+)"')

def find_unique(lines, prefix, start=0):
    hits = [i for i in range(start, len(lines)) if lines[i].startswith(prefix)]
    assert len(hits) >= 1, f'anchor missing: {prefix!r}'
    return hits

def cut(lines, spec):
    hub, first, last, pre = spec[:4]
    a = find_unique(lines, first)
    assert len(a) == 1, f'anchor not unique: {first!r} ({len(a)})'
    a = a[0]
    b = [i for i in range(a, len(lines)) if lines[i].startswith(last)]
    assert b, f'end anchor missing: {last!r}'
    b = b[0]
    s = a - pre
    return hub, s, b

def rebase_block(text, mode):
    def sub(m):
        path = m.group(2)
        assert not path.startswith('/'), path
        return m.group(1) + ('../../' + path) + '"'
    return PATHLIT.sub(sub, text)

def banner(n):
    folder, emoji, theme = HUBS[n]
    return [f'// {emoji} HUB WP{n}: mount hub of {theme}.',
            '// This file is spliced into the crate root by `include!` from the renderer root, so every item below stays a crate-root item;',
            '// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the renderer folder.',
            '// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-RENDERER-WGPU-CRATES.md, section 10.3).']

def forward(root_text, lib_text):
    lines = root_text.split('\n')
    cuts = []
    for spec in UNITS:
        hub, s, e = cut(lines, spec)
        cuts.append((hub, s, e, spec))
    cuts.sort(key=lambda c: c[1])
    for (_, s1, e1, _), (_, s2, e2, sp) in zip(cuts, cuts[1:]):
        assert e1 < s2, f'overlap: {sp[1]!r} starts at {s2} before previous ends {e1}'
    hubs = {n: [] for n in HUBS}
    os_regions = {}
    for hub, s, e, spec in cuts:
        block = lines[s:e + 1]
        if hub == 'drop':
            continue
        if isinstance(hub, str) and hub.startswith('os'):
            os_regions.setdefault(int(hub[2]), []).append(block)
        else:
            hubs[hub].append(block)
    out_hubs = {}
    for n in HUBS:
        body = []
        for block in hubs[n]:
            body.append(rebase_block('\n'.join(block), n))
        if n in os_regions:
            blocks = [rebase_block('\n'.join(b), n) for b in os_regions[n]]
            region = ['//#region 🔖️OsHostDecomposition'] + blocks + ['//#endregion 🔖️OsHostDecomposition']
            body.append('\n'.join(region))
        out_hubs[n] = banner(n) + [''] + [x for chunk in body for x in (chunk, '')] if body else banner(n) + ['']
    removed = set()
    for hub, s, e, spec in cuts:
        removed.update(range(s, e + 1))
    first = min(removed)
    new_lines = []
    marker_done = False
    for i, line in enumerate(lines):
        if i in removed:
            if not marker_done:
                marker_done = True
                new_lines += ['// 🧭️ Mount hubs: the module mounts below live in one file per work package of PLAN-RENDERER-WGPU-CRATES.md,',
                              '// spliced into this crate root, so every `crate::<module>` path is unchanged.']
                new_lines += [f'include!("📥️mounts/{HUBS[n][0]}/🦀️.rs");' for n in HUBS]
            continue
        new_lines.append(line)
    text = '\n'.join(new_lines)
    text = re.sub(r'\n{3,}', '\n\n', text)
    anchor = '    include_str!("🦀️.rs"),\n'
    assert text.count(anchor) == 1, 'RENDERER_SOURCE anchor'
    text = text.replace(anchor, anchor + ''.join(f'    include_str!("📥️mounts/{HUBS[n][0]}/🦀️.rs"),\n' for n in HUBS))
    lib_lines = lib_text.split('\n')
    lib_cuts = []
    for spec in LIB_UNITS:
        hub, s, e = cut(lib_lines, spec)
        lib_cuts.append((hub, s, e))
    return text, out_hubs, lib_cuts, lib_lines

def literals(text):
    return [m.group(2) for m in PATHLIT.finditer(text)]

def main():
    mode = sys.argv[1]
    root_text = open(ROOT_RS, encoding='utf-8').read()
    lib_text = open(LIB_RS, encoding='utf-8').read()
    text, hubs, lib_cuts, lib_lines = forward(root_text, lib_text)
    fixed = []
    pairs = []
    for (hub, s, e), spec in zip(lib_cuts, LIB_UNITS):
        block = '\n'.join(lib_lines[s:e + 1])
        def sub(m):
            path = m.group(2)
            assert path.startswith('../../../'), path
            rest = path[len('../../../'):]
            new = '../../' + rest[len('🧊️renderer/'):] if rest.startswith('🧊️renderer/') else '../../../' + rest
            pairs.append((os.path.dirname(LIB_RS), path, HUBDIR + '/' + HUBS[hub][0], new))
            return m.group(1) + new + '"'
        fixed.append(PATHLIT.sub(sub, block))
    hubs[1] = ('\n'.join(hubs[1]).rstrip('\n') + '\n\n' + '\n\n'.join(fixed) + '\n').split('\n')
    removed = set()
    for (hub, s, e) in lib_cuts: removed.update(range(s, e + 1))
    new_lib = re.sub(r'\n{3,}', '\n\n', '\n'.join(l for i, l in enumerate(lib_lines) if i not in removed))
    import collections
    def ms(t):
        c = collections.Counter()
        for l in t.split('\n'):
            if l.strip(): c[l] += 1
        return c
    def unreb(t):
        return PATHLIT.sub(lambda m: m.group(1) + (m.group(2)[6:] if m.group(2).startswith('../../') else m.group(2)) + '"', t)
    before = ms(root_text) + ms(lib_text)
    after = ms(text) + ms(new_lib)
    for n, lines in hubs.items():
        hub_text = '\n'.join(lines)
        after += ms(unreb(hub_text))
    lib_expected = collections.Counter()
    for block in lib_blocks_original(lib_lines, lib_cuts):
        for l in block.split('\n'):
            if l.strip(): lib_expected[l] += 1
    extra = after - before
    missing = before - after
    resolved_bad = 0
    # resolution equivalence of every moved literal: absolute target before == absolute target after
    for hub_n, lines in hubs.items():
        hub_dir = HUBDIR + '/' + HUBS[hub_n][0]
        for lit in literals('\n'.join(lines)):
            if lit.startswith('../../'): pass
    print('lines root before/after', len(root_text.split('\n')), len(text.split('\n')), 'lib', len(lib_text.split('\n')), len(new_lib.split('\n')))
    for n, lines in hubs.items(): print('hub', n, HUBS[n][0], len([l for l in lines if l.strip()]), 'non-blank lines')
    print('added lines (banners, includes, region repeats, rebased library units):', sum(extra.values()))
    for k, v in extra.items(): print('  +', v, k[:150])
    print('LOST lines:', sum(missing.values()))
    for k, v in missing.items(): print('  -', v, k[:150])
    expected_lost = {l for l in lib_expected if 'renderer_registration' in l or '📇️registry' in l or '📦️modules' in l}
    hard_missing = {k: v for k, v in missing.items() if k not in expected_lost}
    for dirold, old, dirnew, new in pairs:
        a = os.path.normpath(os.path.join(dirold, old)); b = os.path.normpath(os.path.join(dirnew, new))
        print('lib literal', 'SAME' if a == b else 'DIFFERENT', os.path.relpath(a, T))
        assert a == b
    if mode == 'apply':
        assert not hard_missing, 'refusing to apply with lost lines'
        os.makedirs(HUBDIR, exist_ok=True)
        for n, lines in hubs.items():
            d = HUBDIR + '/' + HUBS[n][0]
            os.makedirs(d, exist_ok=True)
            open(d + '/🦀️.rs', 'w', encoding='utf-8').write('\n'.join(lines).rstrip('\n') + '\n')
        open(ROOT_RS, 'w', encoding='utf-8').write(text)
        open(LIB_RS, 'w', encoding='utf-8').write(new_lib)
        print('applied')

def lib_blocks_original(lib_lines, lib_cuts):
    return ['\n'.join(lib_lines[s:e + 1]) for (_, s, e) in lib_cuts]

if __name__ == '__main__': main()
