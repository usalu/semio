# 📚️ Generates the machine tables appended to PLAN-RENDERER-WGPU-CRATES.md: F (crate registry), G (hub units), H (cross-WP lift ledger).
import json, os, re, sys
import crates as C
PLAN = C.HERE + '/../PLAN-RENDERER-WGPU-CRATES.md'
import hubsplit as H
MODULE_CRATE = {
 'dock': 'dock', 'mesh_assets': 'host-base', 'agent_bridge': 'agent-elements', 'agent_presence': 'agent-elements', 'agent_approvals': 'agent-elements', 'agent_chat_panel': 'agent-elements', 'agent_bridge_door': 'agent-elements',
 'runtime_assets': 'host-base', 'space_administration': 'hub-workspace', 'hub_sign_in': 'hub-contracts', 'space_browser': 'hub-contracts', 'hub_connection': 'hub-workspace', 'canvas_presence': 'hub-workspace',
 'element_contracts': 'contracts', 'engine_canvas': 'engine-registry, engine-text, engine-board, engine-graph, engine-attach (facade module)', 'interpreter': 'ui-engine-focus, ui-engine-dump, ui-engine (facade module)',
 'media_slots': 'media-slots', 'program_bridge': 'program-bridge', 'plugin_catalog': 'host-base', 'scenes': 'scene-state, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, scene-render, icon-host (facade module)',
 'canvas2d_gumball': 'canvas2d-host (gumball types: scene-state)', 'canvas2d_paint': 'canvas2d-paint', 'native_temporal': 'native-platform', 'shell': 'shell-* (facade module until S0 and WP5 land)', 'icon_atlas': 'icon-host',
 'deadlines': 'host-plumbing', 'frame_latency': 'host-base', 'os_host': 'host-winit', 'native_caller': 'host-plumbing', 'render_snapshot': 'host-plumbing', 'browser_original_output': 'host-plumbing', 'runtime_mailbox_core': 'mailbox-core',
 'frame_job': 'frame-job', 'frame_authority': 'frame-job', 'surface_lane': 'host-plumbing', 'input_wire': 'host-plumbing', 'socket_door': 'doors', 'directory_door': 'doors', 'browser_worker': 'browser-worker', 'winit_app': 'host-winit',
 'native_accessibility': 'native-platform', 'parallel_runtime': 'kernel-protocol', 'asset_authority': 'asset-authority', 'glb_decode': 'asset-decode', 'asset_decode_jobs': 'asset-decode (renderer I/O slots: host-base)', 'scale_bench': 'kernel-runtime',
 'kernel_runtime': 'kernel-protocol, kernel-retained, kernel-pool, kernel-runtime (facade module)', 'frame_actions': 'frame-actions (RuntimeApply: runtime-mailbox)', 'presentation_raster': 'runtime-mailbox', 'mailbox_impl': 'runtime-mailbox',
 'frame_build': 'app-runtime, frame-transaction', 'frame_presentation': 'frame-presentation (AppFramePreparation: app-runtime)', 'app_runtime': 'app-runtime (boot_runtime: frame-transaction)', 'native_socket_probe': 'native-probe',
 'native_runtime_modules': 'program-bridge', 'fixture_original_turn': 'ui-engine-tests', 'async_boundary_tests': 'runtime-mailbox-tests', 'frame_action_ledger_tests': 'frame-actions', 'settle_pump_tests': 'shell-tests-chrome-6',
 'wheel_application_point_tests': 'frame-actions', 'boot_axis_parity_tests': 'host-base',
}
SINGLE = {'dock', 'mesh_assets', 'agent_bridge', 'agent_presence', 'agent_approvals', 'agent_chat_panel', 'agent_bridge_door', 'runtime_assets', 'space_administration', 'hub_sign_in', 'space_browser', 'hub_connection', 'canvas_presence',
          'element_contracts', 'media_slots', 'program_bridge', 'plugin_catalog', 'canvas2d_paint', 'native_temporal', 'icon_atlas', 'deadlines', 'frame_latency', 'os_host', 'native_caller', 'render_snapshot', 'browser_original_output',
          'runtime_mailbox_core', 'frame_job', 'frame_authority', 'surface_lane', 'input_wire', 'socket_door', 'directory_door', 'browser_worker', 'winit_app', 'native_accessibility', 'parallel_runtime', 'asset_authority', 'glb_decode',
          'scale_bench', 'presentation_raster', 'mailbox_impl', 'native_socket_probe', 'native_runtime_modules', 'canvas2d_gumball'}
def snake(key): return (C.PKG_PREFIX + key).replace('-', '_')
def appendix_f(crates):
    rows = ['| # | crate (`semio-framework-os-renderer-wgpu-…`) | folder under `T/` | WP | kind | cfg | rank | direct first-party dependencies (frozen; downward only) |', '|---:|---|---|---:|---|---|---:|---|']
    for i, k in enumerate(C.ALL_KEYS, 1):
        c = crates[k]
        cfg = {'both': 'native + wasm32', 'native': 'native only', 'wasm': 'wasm32-unknown-unknown only', 'all': 'every target'}[c['cfg']]
        rows.append(f"| {i} | `{k}` | `{c['folder']}` | {c['wp']} | {c['kind']} | {cfg} | {c['rank']} | {', '.join('`'+d+'`' for d in c['deps']) or '-'} |")
    return '\n'.join(rows)
def appendix_g():
    rows = ['| hub (WP) | module | cfg | target crate(s) | after a single-crate carve |', '|---|---|---|---|---|']
    for n, (folder, _, _) in H.HUBS.items():
        path = H.HUBDIR + '/' + folder + '/🦀️.rs'
        text = open(path, encoding='utf-8').read()
        items = re.findall(r'((?:^[ \t]*#\[[^\n]*\]\n)*)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', text, re.M)
        for attrs, name in items:
            cfg = ' '.join(a.strip() for a in attrs.split('\n') if a.strip().startswith('#[cfg'))
            cfg = cfg.replace('#[cfg(', '').replace(')]', '').strip() if cfg else '-'
            target = MODULE_CRATE.get(name, '?')
            if name in SINGLE:
                crate = target.split(' ')[0]
                alias = f"`{'#[cfg('+cfg+')] ' if cfg != '-' else ''}extern crate {snake(crate)} as {name};`"
            else: alias = 'keep the mount; the parent file in your tree becomes the facade'
            rows.append(f"| WP{n} | `{name}` | {'`'+cfg+'`' if cfg != '-' else '-'} | {target} | {alias} |")
    return '\n'.join(rows)
def appendix_h(crates):
    text = open(PLAN, encoding='utf-8').read()
    s52 = text[text.index('### 5.2 Item moves'):text.index('Additional moves implied by whole-file membership')]
    rows = ['| destination crate (WP) | from file | default crate of that file (WP) | lines | lift file to create (source WP) |', '|---|---|---|---:|---|']
    def wp(name):
        if name.startswith('shell-'): return 4
        return crates[name]['wp'] if name in crates else 0
    cross = 0
    for line in s52.split('\n'):
        cells = [c.strip() for c in line.strip().strip('|').split('|')]
        if len(cells) == 5 and cells[0].startswith('`') and cells[3].isdigit():
            dest, src, default, lines = cells[0].strip('`'), cells[1].strip('`'), cells[2].strip('`'), cells[3]
            default = default.replace('*', '*')
            wd, ws = wp(dest), wp(default.replace('shell-*', 'shell-state'))
            slug = re.sub(r'^[^\w]+', '', src).replace('/', '-')
            mark = f"`{C.folder(dest)}/📥️lifted/{src.lstrip()}/🦀️.rs` (WP{ws})" if wd != ws else 'same WP: move inside your tree'
            if wd != ws: cross += 1
            rows.append(f"| `{dest}` (WP{wd}) | `{src}` | `{default}` (WP{ws}) | {lines} | {mark} |")
    return '\n'.join(rows), cross
if __name__ == '__main__':
    crates = C.build()
    out = {'F': appendix_f(crates), 'G': appendix_g()}
    out['H'], cross = appendix_h(crates)
    print('F rows', out['F'].count('\n') - 1, 'G rows', out['G'].count('\n') - 1, 'H rows', out['H'].count('\n') - 1, 'cross-WP', cross)
    json.dump(out, open(C.GEN + '/rwp0-appendix.json', 'w'), ensure_ascii=False, indent=1)
    for k in 'GH': print(out[k][:1500]); print('...')
