# 🧭️ WP0 mount-hub split of the plugin crate root (PLUG/🦀️.rs): the `mod` mounts move into seven hubs, one per work package.
# usage: hubsplit.py dry|apply     (dry prints the unit table and verifies losslessness; apply writes the hub files and rewrites the root)
# Every unit is cut by its exact line range (anchored by its first line), appended to the hub of the work package that owns its crate, and its path literals are rebased by "../../".
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rootunits as ru
import crates as C

HUBDIR = C.PLUG + '/' + C.MOUNTS_DIR
HUBS = {
    1: ('🌱️wp1-foundation', '🌱️', 'WP1 foundations and windows: abi, actor-host, identity, interaction, operation, window-config, window-transient, window-kits, window-kits-rich'),
    2: ('📜️wp2-declaration', '📜️', 'WP2 declaration contracts: decl-views, decl-emission, decl-app, decl-surface, decl-registry, app-builder, plugin-program'),
    3: ('👥️wp3-owners', '👥️', 'WP3 owners and extension: child, composition-emission, composition-publication, publication-mounted, document-media, job, command, peer, retained-command, extension'),
    4: ('⚛️wp4-reactor', '⚛️', 'WP4 reactor: reactor-app, reactor-tasks, reactor-lifecycle, runtime-instances, runtime-ops, runtime-exchange, reactor-patches, reactor-surfaces, reactor-core, reactor-jobs, reactor-turn, reactor-wit-bridge, reactor-context'),
    5: ('⏯️wp5-features', '⏯️', 'WP5 features: tool-run, time-travel, time-travel-panel (their root mounts live in the job facet module, so this hub is empty until a feature crate needs a root mount)'),
    6: ('🧱️wp6-instance-tier1', '🧱️', 'WP6 instance tier 1: instance-stores, instance-history, instance-children, instance-jobs, instance-docs, instance-extension'),
    7: ('🧵️wp7-instance-tier2', '🧵️', 'WP7 instance tier 2 and 3, builder, testing: instance-commands, instance-typed-ops, instance-render, instance-ladders, time-travel-turns, tool-run-turns, instance-app, builder, testing-laws, testing-acceptance and the root test fixtures'),
}
# units that stay in the root: macro-textual tests and the macro-exporting generated contracts; every unit whose file defines a `#[macro_export]` macro stays too (found by the differential: a macro-exporting module declared inside an `include!`d hub makes `crate::<macro>!` paths an error)
STAY = {'generated_test_contracts', 'derived_artifact_children_tests', 'subset_macro_tests'}
OVERRIDE = {
    'ui_action_laws': 7, 'declaration_fixture_mutations': 7, 'publication_fixture': 7, 'window_transient_owners': 7, 'test_app_mutation_fixture': 7, 'contributed_mutation_wire': 7, 'app_testing': 7, 'builder': 7, 'artifact_instance': 7,
    'cooperative_pump_tests': 4, 'plugin_runtime_runtime_laws': 4, 'fixture_projection_retirement_tests': 2, 'segmented_download_contract': 2, 'natural_file_lifecycle_tests': 2, 'bounded_initializer_laws': 1, 'action_ingress': 6,
}
WP = {}
def wp_table():
    crates, _ = C.build()
    return {k: c['wp'] for k, c in crates.items()}

def hub_of(unit, wps):
    import ownership as ow
    n = unit['name']
    if n in STAY: return 'stay'
    if '#[macro_export]' in open(C.PLUG + '/' + unit['path'], encoding='utf-8').read(): return 'stay'
    if n in OVERRIDE: return OVERRIDE[n]
    w = ow.owner_wp(unit['path'])
    if w is None: raise SystemExit('unassigned unit ' + n + ' -> ' + unit['path'])
    return w

PATHLIT = re.compile(r'(#\[\s*path\s*=\s*")([^"]+)"')
def rebase(text):
    def sub(m):
        assert not m.group(2).startswith('/'), m.group(2)
        return m.group(1) + '../../' + m.group(2) + '"'
    return PATHLIT.sub(sub, text)

def banner(n):
    folder, emoji, theme = HUBS[n]
    return [f'// {emoji} HUB WP{n}: mount hub of {theme}.',
            '// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;',
            f'// its `#[path]` literals resolve against this folder (`{C.MOUNTS_DIR}/<hub>/`), hence the `../../` prefix of the plugin folder.',
            '// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).']

def plan(text):
    wps = wp_table()
    lines = text.split('\n')
    units = ru.units(text)
    moved = []
    for u in units:
        h = hub_of(u, wps)
        if h != 'stay': moved.append((h, u))
    return lines, units, moved

def build(text):
    lines, units, moved = plan(text)
    removed = set()
    for h, u in moved: removed.update(range(u['start'], u['end'] + 1))
    hubs = {n: [] for n in HUBS}
    for h, u in sorted(moved, key=lambda x: x[1]['start']):
        hubs[h].append(u)
    out_hubs = {}
    for n in HUBS:
        body = []; cur = None
        for u in hubs[n]:
            if u['region'] != cur:
                if cur: body.append('//#endregion ' + cur)
                if u['region']: body.append('//#region ' + u['region'])
                cur = u['region']
            body.append(rebase(u['text']))
        if cur: body.append('//#endregion ' + cur)
        out_hubs[n] = banner(n) + ([''] + body if body else [])
    first = min(u['start'] for h, u in moved if u['region'] == '🏛️OwnershipFacet')
    new_lines = []; inserted = False
    for i, l in enumerate(lines):
        if i in removed:
            if i >= first and not inserted:
                inserted = True
                new_lines += ['// 🧭️ Mount hubs: the module mounts that used to follow live in one file per work package of PLAN-PLUGIN-CRATES.md,',
                              '// spliced into this crate root, so every `crate::<module>` path is unchanged.']
                new_lines += [f'include!("{C.MOUNTS_DIR}/{HUBS[n][0]}/🦀️.rs");' for n in HUBS]
            continue
        new_lines.append(l)
    # drop region marker pairs that enclose nothing any more
    out = []; i = 0
    while i < len(new_lines):
        l = new_lines[i]
        if l.strip().startswith('//#region'):
            j = i + 1
            while j < len(new_lines) and not new_lines[j].strip(): j += 1
            if j < len(new_lines) and new_lines[j].strip().startswith('//#endregion'):
                i = j + 1; continue
        out.append(l); i += 1
    t = '\n'.join(out)
    t = re.sub(r'\n{3,}', '\n\n', t)
    return t, out_hubs, moved, units

def main():
    mode = sys.argv[1]
    text = open(ru.ROOT_RS, encoding='utf-8').read()
    if '🪝️mounts' in text: raise SystemExit('root already hubbed')
    new_root, hubs, moved, units = build(text)
    def ms(t):
        c = collections.Counter()
        for l in t.split('\n'):
            if l.strip(): c[l] += 1
        return c
    def unreb(t): return PATHLIT.sub(lambda m: m.group(1) + (m.group(2)[6:] if m.group(2).startswith('../../') else m.group(2)) + '"', t)
    before = ms(text); after = ms(new_root)
    for n, lines in hubs.items(): after += ms(unreb('\n'.join(lines)))
    extra = after - before; missing = before - after
    print('units', len(units), 'moved', len(moved), 'stay', len(units) - len(moved))
    cnt = collections.Counter(h for h, u in moved)
    for n in HUBS: print('hub', n, HUBS[n][0], cnt[n], 'units')
    print('root lines', len(text.split('\n')), '->', len(new_root.split('\n')))
    print('added non-blank lines (banners, includes, region repeats):', sum(extra.values()))
    for k, v in extra.items(): print('  +', v, k[:140])
    print('LOST lines:', sum(missing.values()))
    for k, v in missing.items(): print('  -', v, k[:140])
    if mode == 'apply':
        assert not missing, 'refusing to apply with lost lines'
        cur = open(ru.ROOT_RS, encoding='utf-8').read()
        assert cur == text, 'root changed while planning; rerun'
        for n, lines in hubs.items():
            d = HUBDIR + '/' + HUBS[n][0]
            os.makedirs(d, exist_ok=True)
            open(d + '/🦀️.rs', 'w', encoding='utf-8').write('\n'.join(lines).rstrip('\n') + '\n')
        open(ru.ROOT_RS, 'w', encoding='utf-8').write(new_root)
        print('applied')

if __name__ == '__main__': main()
