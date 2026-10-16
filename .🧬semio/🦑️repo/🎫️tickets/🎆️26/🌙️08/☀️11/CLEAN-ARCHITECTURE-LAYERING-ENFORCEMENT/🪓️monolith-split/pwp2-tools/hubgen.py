#!/usr/bin/env python3
# 🪝️ Generates the new hub 2 (`📜️wp2-declaration`) and the two root edits from the snapshot: every unit of a carved WP2 module becomes an extern crate, a re-export or a thin module.
import re, os, sys, collections
from common import *
import model as M
import engine as E

HUB_REL = '🪝️mounts/📜️wp2-declaration/🦀️.rs'
# old module name -> (crate, is_root_of_crate); crate-root modules become `extern crate` (or a thin module when items left); children are re-exported from their crate
def unit_plan():
    plan = {}
    for key in M.WP2:
        root_mod = next(n for n, f in M.MOD_FILE.items() if f == M.ROOT[key] and M.MOD_CRATE.get(n) == key)
        plan[root_mod] = (key, None)
        for mod in M.CHILDREN[key]:
            plan[mod] = (key, mod)
    return plan

PRIVATE_IN_SOURCE = {'OriginalInferenceSessionOwner', 'OriginalInferenceSession', 'original_inference_metadata'}

def outs_of(mod):
    """items that left module `mod` for another crate: [(crate, module-in-crate, [names])]"""
    f = M.MOD_FILE[mod]
    out = collections.defaultdict(list)
    for c in M.CUTS:
        if c['file'] == f:
            for n in c['names']:
                if n in PRIVATE_IN_SOURCE:
                    continue
                h = E.HOMES.home(mod, n, None)
                out[(c['to'], h[1] if h else None)].append(n)
    return [(k[0], k[1], v) for k, v in out.items()]

def unit_text(mod, plan):
    key, child = plan[mod]
    lib = M.libname(key)
    outs = outs_of(mod)
    root = child is None
    has_re = any(h['module'] == mod and h.get('reexport') for h in __import__('json').load(open(WS + '/hosts.json')))
    if not outs and not has_re:
        if root:
            return f'pub(crate) extern crate {lib} as {mod};'
        return f'pub(crate) use {lib}::{child};'
    lines = [f'pub(crate) mod {mod} {{']
    lines.append(f'    pub use {lib}::' + ('*' if root else f'{child}::*') + ';')
    for dst, dmod, names in outs:
        path = M.libname(dst) + ('::' + dmod if dmod else '')
        vis = 'pub'
        lines.append(f'    {vis} use {path}::' + (names[0] if len(names) == 1 else '{' + ', '.join(names) + '}') + ';')
    if mod == 'app_contract':
        dm = M.libname('document-media')
        lines.append(f'    pub use {dm}::copy_media_text_parts;')
        lines.append('    #[cfg(test)]')
        lines.append(f'    pub(crate) use {dm}::{{receive_media_artifact_fault, receive_media_output}};')
    import json
    for h in json.load(open(WS + '/hosts.json')):
        if h['module'] == mod and h.get('reexport'):
            lines.append('    #[cfg(' + h['cfg'] + ')]')
            lines.append('    pub(crate) use super::' + h['host'] + '::{' + ', '.join(h['reexport']) + '};')
    lines.append('}')
    return '\n'.join(lines)

def build_hub():
    src = E.src_text(HUB_REL)
    plan = unit_plan()
    out = []
    lines = src.split('\n')
    i = 0
    while i < len(lines):
        l = lines[i]
        if l.startswith('#[path = ') and i + 1 < len(lines):
            m = re.match(r'(pub(?:\([^)]*\))?\s+)?mod\s+(\w+);', lines[i + 1])
            if m and m.group(2) in plan and not lines[i - 1].startswith('#[cfg'):
                mod = m.group(2)
                if mod == 'declarations':
                    out.append(declarations_text())
                else:
                    out.append(unit_text(mod, plan))
                i += 2
                continue
        out.append(l)
        i += 1
    return '\n'.join(out).rstrip('\n') + '\n' + hosts_text() + '\n'

def hosts_text():
    import json
    hosts = json.load(open(WS + '/hosts.json'))
    out = ['//#region 🧪️WP2TestHosts']
    for h in hosts:
        out.append('#[cfg(' + h['cfg'] + ')]')
        out.append('#[path = "' + os.path.relpath(h['rel'], os.path.dirname(HUB_REL)) + '"]')
        out.append('mod ' + h['host'] + ';')
    out.append('//#endregion 🧪️WP2TestHosts')
    return '\n'.join(out)

def declarations_text():
    lib = M.libname('decl-registry')
    return '\n'.join([
        'pub mod declarations {',
        f'    pub use {lib}::declarations::*;',
        f'    pub(crate) use {M.libname("extension")}::{{commit_artifact_declarations, preflight_artifact_declarations}};',
        f'    pub use {M.libname("instance-app")}::{{editor_surface, viewer_surface, EditorSurfaceApp, ViewerSurfaceApp}};',
        ] + [l for h in __import__('json').load(open(WS + '/hosts.json')) if h['module'] == 'declarations' and h.get('reexport') for l in ('    #[cfg(' + h['cfg'] + ')]', '    pub(crate) use super::' + h['host'] + '::{' + ', '.join(h['reexport']) + '};')] + ['}'])

def root_edit():
    src = E.src_text('🦀️.rs')
    a = '#[path = "📜️declaration/📨️emission/🦀️.rs"]\npub(crate) mod app_emission;\n#[path = "📜️declaration/👷️job/🦀️.rs"]\npub(crate) mod tool_contract;\n'
    assert src.count(a) == 1
    return src.replace(a, '')

def extra_units():
    """the two root-resident units, now thin modules in the hub"""
    plan = unit_plan()
    return '\n'.join(unit_text(m, plan) for m in ('app_emission', 'tool_contract'))

if __name__ == '__main__':
    print(build_hub())
