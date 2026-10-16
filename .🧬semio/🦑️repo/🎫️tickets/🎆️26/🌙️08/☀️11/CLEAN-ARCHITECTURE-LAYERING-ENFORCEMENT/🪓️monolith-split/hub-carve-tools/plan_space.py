#!/usr/bin/env python3
"""🪐️ Plan for `semio-hub-space`: foundation < operations < two command crates < app < laws, with the plugin registration left in the root crate."""
import os, sys, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ws as wsm, hcmod, carve, union
from carve import Edits, Carver, Model, REPO
from helpers import *

SP = P('🧩️compositions/🪐️space/')
ENG = SP + '⚙️engine/🪐️space/'
ROOT = SP + '🦀️.rs'
COMP = ENG + '🦀️.rs'
OPS = ENG + '⚙️engine/🦀️.rs'
CMD = ENG + '🎮️commands/'

FOUNDATION = ('semio_hub_space_foundation', ENG + '🧱️foundation/🦀️.rs')
OPERATIONS = ('semio_hub_space_operations', OPS)
CMD_AUTHORING = ('semio_hub_space_commands_authoring', CMD + '🛠️authoring/🦀️.rs')
CMD_MEDIA = ('semio_hub_space_commands_media', CMD + '🖼️media/🦀️.rs')
APP = ('semio_hub_space_app', COMP)
LAWS = ('semio_hub_space_laws', ENG + '🧪️tests/🏛️laws/🦀️.rs')
HUBROOT = ('semio_hub_space', ROOT)
ROOTS = collections.OrderedDict([FOUNDATION, OPERATIONS, CMD_AUTHORING, CMD_MEDIA, APP, LAWS, HUBROOT])
TEST_TRUE = set()

AUTHORING = ['add_parameter', 'bind_parameter_field', 'close_focused_instance', 'copy_app_instance', 'delete_selection', 'duplicate_app_instance', 'node_graph_edit', 'node_graph_viewport', 'move_media_node', 'open_instance', 'paste_app_instance', 'patch_app_instances', 'patch_parameter', 'remove_app_instance', 'remove_parameter', 'rename_app_instance', 'reorganize_workflow', 'spawn_app', 'unbind_parameter_field']

DOCS = {
    'semio_hub_space_foundation': 'Studio foundation: the persisted space configuration, presence, terminology and the identifiers every studio layer shares.',
    'semio_hub_space_operations': 'Studio operations: the headless compute over the kernel workflow snapshot that every command builds its mutations from.',
    'semio_hub_space_commands_authoring': 'Studio authoring commands: node, instance, parameter and selection edits over the workflow document.',
    'semio_hub_space_commands_media': 'Studio media and session commands: media graph edits, media and pack import and export, engagement and panel state.',
    'semio_hub_space_app': 'Studio app: the artifact app, its manifest and dispatch, the main window modes and the panels.',
    'semio_hub_space_laws': 'Studio laws: the per-command laws over the real studio app.',
}
HEADER = '''#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
use semio_s_space_core::*;
'''


def header_after_docs(src):
    lines = src.split('\n')
    n = 0
    while n < len(lines) and lines[n].startswith('//!'):
        n += 1
    return '\n'.join(lines[:n]) + '\n\n' + HEADER + ('\n' if n < len(lines) else '') + '\n'.join(lines[n:]).lstrip('\n')


def command_mounts(root_src):
    pairs = re.findall(r'#\[path = "⚙️engine/🪐️space/🎮️commands/([^/]+)/🦀️\.rs"\]\n\s*pub mod (\w+);', root_src)
    return pairs


def modes_panels_text(root_src):
    a = root_src.index('        #[path = "."]\n        pub mod modes {')
    b = root_src.index('    }\n}\n//#endregion ⚙️Engine')
    block = root_src[a:b].rstrip('\n')
    block = '\n'.join(l[8:] if l.startswith('        ') else l for l in block.split('\n'))
    return block.replace('"⚙️engine/🪐️space/', '"')


def replace_all_text(ws, file, old, new, skip_prefix=None):
    s = ws.read(file)
    out = re.sub(r'(?<![\w:])' + re.escape(old), new, s)
    assert out != s, old
    ws.write(file, out)


def space_structural(ws, E):
    root_src = ws.read(ROOT)
    pairs = command_mounts(root_src)
    names = {m: d for d, m in pairs}
    authoring = [m for m in AUTHORING]
    media = [m for d, m in pairs if m not in AUTHORING]
    assert len(pairs) == 40 and all(m in names for m in authoring), len(pairs)
    modes_panels = modes_panels_text(root_src)
    # foundation
    fcommon = lambda doc: f'//! 🧱️ {doc}\n\n{HEADER}\n'
    found_text = fcommon(DOCS['semio_hub_space_foundation']) + '''#[path = "."]
pub mod config {
    #[path = "../🎚️config/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}

#[path = "."]
pub mod presence {
    #[path = "../👥️presence/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}

#[path = "../🗣️terminology/🦀️.rs"]
pub mod terminology;
'''
    create(ws, FOUNDATION[1], found_text)
    E.move(COMP, ['S_PLAY_APP_ID', 'S_PLAY_CONTROLLER_ID', 'S_PLAY_CATALOGUE_TAB_ID', 'S_PLAY_PARAMETERS_TAB_ID', 'S_PLAY_INSPECTOR_TAB_ID', 'S_PLAY_CATALOGUE_BODY_KEY', 'S_PLAY_PARAMETERS_BODY_KEY', 'S_PLAY_INSPECTOR_BODY_KEY', 'S_PLAY_CATALOGUE_DRAG_MIME', 'S_STUDIO_EXAMPLES', 'S_PLAY_INTERACTION_DOMAIN'], FOUNDATION[1])
    wf = ENG + '🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs'
    E.move(wf, ['S_PLAY_WINDOW_WORKFLOW', 'S_PLAY_BODY_WORKFLOW', 'S_PLAY_SURFACE_WORKFLOW'], FOUNDATION[1])
    E.proxy[FOUNDATION[1]] = COMP
    base = ('engine', 'space')
    for old, (nf, new) in {
        ('config',): (FOUNDATION[1], ('config',)),
        ('presence',): (FOUNDATION[1], ('presence',)),
        ('modes',): (COMP, ('modes',)),
        ('modes', 'main'): (COMP, ('modes', 'main')),
        ('modes', 'main', 'windows'): (COMP, ('modes', 'main', 'windows')),
        ('modes', 'main', 'windows', 'workflow'): (COMP, ('modes', 'main', 'windows', 'workflow')),
        ('modes', 'main', 'windows', 'workflow', 'options'): (COMP, ('modes', 'main', 'windows', 'workflow', 'options')),
        ('modes', 'main', 'windows', 'media_vfs'): (COMP, ('modes', 'main', 'windows', 'media_vfs')),
        ('modes', 'main', 'windows', 'compiled_dag'): (COMP, ('modes', 'main', 'windows', 'compiled_dag')),
        ('panels',): (COMP, ('panels',)),
    }.items():
        E.inline_map[(ROOT, base + old)] = (nf, new)
    replace_all_text(ws, COMP, 'workflow::S_PLAY_WINDOW_WORKFLOW', 'semio_hub_space_foundation::S_PLAY_WINDOW_WORKFLOW', skip_prefix='windows::')
    # operations
    E.move(COMP, ['negotiate_connect_or_notify', 'connect_edge_operation', 'primary_selected_node_id', 'config_space_id'], OPS)
    ws.write(OPS, header_after_docs(ws.read(OPS)))
    # commands
    for ident, root, mods in ((CMD_AUTHORING[0], CMD_AUTHORING[1], authoring), (CMD_MEDIA[0], CMD_MEDIA[1], media)):
        text = fcommon(DOCS[ident])
        for m in sorted(mods):
            text += f'#[path = "../{names[m]}/🦀️.rs"]\npub mod {m};\n'
        create(ws, root, text)
    # command tests live with the studio app they drive
    law_text = f'//! 🏛️ {DOCS[LAWS[0]]}\n\n{HEADER}\n'
    for d, m in pairs:
        cf = CMD + d + '/🦀️.rs'
        if os.path.exists(CMD + d + '/🧪️tests/🔬️unit/🦀️.rs'):
            drop_mounts(ws, cf, ['tests'])
            law_text += f'#[cfg(test)]\n#[path = "../../🎮️commands/{d}/🧪️tests/🔬️unit/🦀️.rs"]\nmod {m}_tests;\n\n'
    create(ws, LAWS[1], law_text.rstrip('\n') + '\n')
    # app
    src = header_after_docs(ws.read(COMP))
    ws.write(COMP, src.rstrip('\n') + '\n\n//#region 🔖️Windows\n' + modes_panels + '\n//#endregion 🔖️Windows\n')
    # root
    remove_items(ws, ROOT, lambda it, toks: it.kind == 'mod' and it.name == 'engine')
    s = ws.read(ROOT)
    s = s.replace('//#region ⚙️Engine\n//#endregion ⚙️Engine\n\n', '')
    ws.write(ROOT, s)
    return pairs


ALLFEAT = ('feature="testkit"',)


def main():
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    base = Model(base_ws, collections.OrderedDict([('semio_hub_space', ROOT)]), allow_dup=True)
    ws = wsm.WS()
    E = Edits(ws)
    space_structural(ws, E)
    return ws, E, base, base_ws


def prepare():
    import scan
    ws, E, base, base_ws = main()
    roots = collections.OrderedDict(ROOTS)
    C = Carver(ws, E, base)
    C.test_true_crates = set(TEST_TRUE)
    C.build_new(roots)
    C.pass1()
    C.build_new(roots)
    C.pass2()
    C.build_new(roots)
    C.pass3(roots)
    C.build_new(roots)
    C.finalize_exports()
    C.build_new(roots)
    C.open_test_imports()
    C.build_new(roots)
    C.flip_not_test_partners()
    C.build_new(roots)
    found = scan.member_scan(C.new)
    C.widen_members(found)
    C.build_new(roots)
    C.member_findings = found
    return ws, C, roots, base_ws


def baseline_variant(label, under, base_ws):
    import pickle
    os.makedirs('/tmp/hubw/res', exist_ok=True)
    pk = f'/tmp/hubw/res/spbase-{label}.pkl'
    if os.path.exists(pk):
        return pickle.load(open(pk, 'rb'))
    hcmod.FS = base_ws
    u = union.Union(base_ws, collections.OrderedDict([('semio_hub_space', ROOT)]), tag='spbase-' + label)
    diags, err = u.run(under, ALLFEAT)
    c = union.multiset(diags)
    pickle.dump(c, open(pk, 'wb'))
    return c


def split_variant(label, ws, roots, under):
    import pickle
    u = union.Union(ws, roots, tag='spsplit-' + label)
    diags, err = u.run(under, ALLFEAT)
    c = union.multiset(diags)
    pickle.dump(c, open(f'/tmp/hubw/res/spsplit-{label}.pkl', 'wb'))
    return c, diags


def report(label, b, s, crates, limit=60):
    def clean(c):
        out = collections.Counter()
        for k, n in c.items():
            if union.is_extern_noise(k[0], k[1], crates):
                continue
            out[(k[0], re.sub(r'`[\w:]*::(\w+)`', r'`\1`', k[1]), k[2])] += n
        return out
    b, s = clean(b), clean(s)
    only_s, only_b = union.diff(s, b)
    print(f'[{label}] base {sum(b.values())} / split {sum(s.values())}; only-split {sum(only_s.values())} only-base {sum(only_b.values())}')
    for (code, msg, f), n in sorted(only_s.items(), key=lambda x: (x[0][2], x[0][1]))[:limit]:
        print(f'   SPLIT+ {n}x {code} {msg[:150]}   @ {f[-70:]}')
    return only_s, only_b


def structure(C):
    import plan_lib, scan
    names = set(ROOTS)
    import regs
    normal, testonly = {}, {}
    for ident in ROOTS:
        n, t = regs.scan_usage(C.new, ident, names)
        n.discard(ident)
        t.discard(ident)
        normal[ident] = n
        testonly[ident] = t - n
    print('CRATE DAG:')
    for f, ts in normal.items():
        print('   ', f, '->', ', '.join(sorted(ts)))
    print('CYCLES:', plan_lib.find_cycles(normal) or 'none')
    print('TEST-ONLY EDGES:')
    for f, ts in testonly.items():
        if ts:
            print('   ', f, '->', ', '.join(sorted(ts)))
    print('CYCLES INCLUDING TEST EDGES:', plan_lib.find_cycles({f: set(normal[f]) | set(testonly[f]) for f in normal}) or 'none')
    print('ORPHAN/INHERENT findings:')
    for r in scan.orphan_scan(C.new):
        print('   ', r)
    print('MEMBER widenings:', len(getattr(C, 'member_findings', [])))


def run_variant(ident):
    ws, C, roots, base_ws = prepare()
    crates = set(ROOTS)
    b = baseline_variant('test', 'semio_hub_space', base_ws)
    s, _ = split_variant('test-' + ident, ws, roots, ident)
    report('test:' + ident, b, s, crates, 80)


if __name__ == '__main__' and len(sys.argv) > 1:
    run_variant(sys.argv[1])
elif __name__ == '__main__':
    ws, C, roots, base_ws = prepare()
    structure(C)
    C.report.dump()
    crates = set(ROOTS)
    b = baseline_variant('native', None, base_ws) | baseline_variant('test', 'semio_hub_space', base_ws)
    s, _ = split_variant('native', ws, roots, None)
    report('native', b, s, crates, 150)
