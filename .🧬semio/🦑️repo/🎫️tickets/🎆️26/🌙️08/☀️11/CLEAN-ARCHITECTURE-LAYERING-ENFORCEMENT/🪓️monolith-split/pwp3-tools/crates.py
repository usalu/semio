# 🧭️ The frozen crate table of the plugin partition (WP0): names, roots, work packages, ranks, first-party and external dependencies, features.
# Sources: PLAN-PLUGIN-CRATES.md 6.1 (parsed), 6.4, 7.5, the D1-D8 decisions of the coordinator, unitdeps.py (token evidence), the folder/emoji choices below.
import json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = HERE + '/../🗑️generated'
R = '/Users/ueli/Documents/semio'
PLUG_REL = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
PLUG = R + '/' + PLUG_REL
PKG_REL = PLUG_REL + '/📦️packages/🦀️rust'
PKG = R + '/' + PKG_REL
PLAN = HERE + '/../PLAN-PLUGIN-CRATES.md'
FE0F = '️'
FACADE = 'semio-framework-plugin'
LIFT = '🛬' + FE0F + 'lifted'
COMPANIONS_DIR = '🧷' + FE0F + 'companions'
MOUNTS_DIR = '🪝' + FE0F + 'mounts'

def e(s): return s.replace('️', '')

def parse_61():
    t = open(PLAN, encoding='utf-8').read()
    i = t.index('### 6.1 Catalogue'); j = t.index('### 6.2 Notes')
    rows = []
    for line in t[i:j].split('\n'):
        c = [x.strip() for x in line.strip().strip('|').split('|')]
        if len(c) == 8 and c[0].startswith('`'):
            name = c[0].strip('`')
            root = c[5].strip('`'); new = root.startswith('†'); root = root.lstrip('† ').strip().strip('`')
            deps = re.findall(r'`([^`]+)`', c[7]) if c[7] != '-' else []
            rows.append(dict(key=name, layer=c[1], plan_lines=int(c[2].replace(',', '')), plan_kb=int(c[3].replace(',', '')), wp=int(c[4]), root=root, plan_new=new, members=c[6], deps=deps))
    return rows

LOWER_ORDER = ['abi', 'actor-host', 'identity', 'interaction', 'operation', 'window-config', 'window-transient', 'window-kits', 'window-kits-rich', 'decl-views', 'decl-emission', 'decl-app', 'app-builder', 'decl-surface',
               'child', 'composition-emission', 'composition-publication', 'document-media', 'job', 'command', 'peer', 'publication-mounted', 'retained-command', 'reactor-app', 'reactor-tasks', 'decl-registry', 'plugin-program',
               'reactor-lifecycle', 'runtime-instances', 'runtime-ops', 'extension', 'reactor-patches', 'reactor-surfaces', 'reactor-core', 'reactor-jobs', 'runtime-exchange', 'reactor-turn', 'reactor-wit-bridge', 'reactor-context',
               'time-travel-panel', 'tool-run', 'time-travel']
TIER1 = ['instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-docs']
FAMILY_ORDER = TIER1 + ['instance-extension', 'instance-commands', 'instance-typed-ops', 'instance-render', 'instance-ladders', 'time-travel-turns', 'tool-run-turns', 'instance-app', 'builder', 'testing-laws', 'testing-acceptance']

# docstring emoji (unique per crate); folder emoji of new directories are chosen below
EMOJI = {
 'abi': '🌉', 'actor-host': '🎭', 'identity': '🪪', 'interaction': '🕹', 'operation': '🔄', 'window-config': '🎚', 'window-transient': '🫧', 'window-kits': '🧰', 'window-kits-rich': '🧱', 'decl-views': '👁', 'decl-emission': '📨',
 'decl-app': '📐', 'app-builder': '🏗', 'decl-surface': '🪟', 'child': '🪆', 'composition-emission': '🌱', 'composition-publication': '📬', 'publication-mounted': '🏇', 'document-media': '🎞', 'job': '👷', 'command': '🎮', 'peer': '👥',
 'retained-command': '🧵', 'reactor-app': '🔌', 'reactor-tasks': '⏲', 'decl-registry': '📇', 'plugin-program': '🧬', 'reactor-lifecycle': '♻', 'runtime-instances': '📦', 'runtime-ops': '⚙', 'extension': '🧩', 'reactor-patches': '🩹',
 'reactor-surfaces': '🖼', 'reactor-core': '⚛', 'reactor-jobs': '💼', 'runtime-exchange': '🔁', 'reactor-turn': '🔂', 'reactor-wit-bridge': '🪢', 'reactor-context': '🎟', 'time-travel-panel': '🕰', 'tool-run': '⏯', 'time-travel': '⏪',
 'instance-stores': '🏪', 'instance-history': '📜', 'instance-children': '👶', 'instance-jobs': '🛠', 'instance-docs': '📄', 'instance-extension': '🪝', 'instance-commands': '⌨', 'instance-typed-ops': '🧾', 'instance-render': '🖌',
 'instance-ladders': '🪜', 'time-travel-turns': '⏳', 'tool-run-turns': '⏩', 'instance-app': '🏛', 'builder': '🔨', 'testing-laws': '⚖', 'testing-acceptance': '✅', 'test-support': '🧯',
}
FRUIT = ['🍎', '🍐', '🍊', '🍋', '🍌', '🍉', '🍇', '🍓', '🫐', '🍈', '🍒', '🍑', '🥭', '🍍', '🥥', '🥝', '🍅', '🥑', '🍆', '🥔', '🥕', '🌽']

DESC = {
 'abi': 'Guest wire ABI of the plugin SDK: io wire, authority returns, snapshots, sqlite wire, wasip2 bindings, owned ABI and the persistent-local macro',
 'actor-host': 'Actor host API of the plugin SDK: host requests, body, backbone binding and the reactor executors',
 'identity': 'Artifact identity vocabulary of the plugin SDK',
 'interaction': 'Local interaction ownership, query, capture, live state, diff, authority and topology of the plugin SDK',
 'operation': 'Cancellation, operation progress, initialization, completion and cold-task vocabulary of the plugin SDK',
 'window-config': 'Window config owners, retained pack load, registry and preparation of the plugin SDK',
 'window-transient': 'Window transient owners, engagement, mutation and window surface of the plugin SDK',
 'window-kits': 'Window kit contract, form panels, label axes, the small kits and the context menu builder of the plugin SDK',
 'window-kits-rich': 'Tree, table, world3d and document window kits of the plugin SDK',
 'decl-views': 'Declaration view contracts of the plugin SDK: views, child, command, tool machine, tool run, media and the empty lane owners',
 'decl-emission': 'Declaration emission contracts of the plugin SDK: emit, child emit preparation, task sources',
 'decl-app': 'The ArtifactApp contract of the plugin SDK together with the tool job contract and completion retirement',
 'app-builder': 'Declarative app builder DSL of the plugin SDK: AppBuilder, App, example sources and gumball audience',
 'decl-surface': 'Surface declarations of the plugin SDK: window surface, natural file import and the editor and viewer surface traits',
 'child': 'Child content ownership, reads and pins of the plugin SDK',
 'composition-emission': 'Composition emission, metadata, root and genesis of the plugin SDK',
 'composition-publication': 'Store publication of composed children, group owners and pending publication of the plugin SDK',
 'publication-mounted': 'Mounted member groups, receipts and members of the plugin SDK publication',
 'document-media': 'Document ingress, store replacement, archive loads and media export of the plugin SDK',
 'job': 'Tool jobs, reserved jobs, outcomes and bytes of the plugin SDK',
 'command': 'Command log, history views, latest-wins and admission of the plugin SDK',
 'peer': 'Peer roster and presence of the plugin SDK',
 'retained-command': 'Retained typed commands and full operations of the plugin SDK',
 'reactor-app': 'The PluginApp trait, its dyn_enum closure prelude and the small turn helpers of the plugin SDK',
 'reactor-tasks': 'Task spawn, records, resumes and extension invocation queue of the plugin reactor',
 'decl-registry': 'Artifact declaration registry, registration and declaration tree of the plugin SDK',
 'plugin-program': 'Plugin program, app instances, surface contexts and the wire descriptor helpers of the plugin SDK',
 'reactor-lifecycle': 'Reactor lifecycle knot of the plugin SDK: close registry, native lifetime registry, instance close lease and the plugin runtime state',
 'runtime-instances': 'Runtime instance registry, descriptor, opening and cleanup pumps of the plugin runtime',
 'runtime-ops': 'Runtime operations of the plugin SDK: wire limits, codecs, document ops, media, surfaces, render, refresh, context menu, command ingress and typed operations',
 'extension': 'Extension bundle, inference gateway, describe and extension retirement of the plugin SDK',
 'reactor-patches': 'Pending patches and patch tracker of the plugin reactor',
 'reactor-surfaces': 'Cold pair, checkpoint and surface slots of the plugin reactor',
 'reactor-core': 'Reactor core registries, presence, instance metadata, package assets and armed timers of the plugin reactor',
 'reactor-jobs': 'Reactor jobs, kind registry, reserved and infer jobs of the plugin reactor',
 'runtime-exchange': 'The plugin exchange function of the plugin runtime',
 'reactor-turn': 'The reactor turn loop, receiving and completed inputs of the plugin reactor',
 'reactor-wit-bridge': 'The wasip2 WIT bridge of the plugin reactor',
 'reactor-context': 'Original actor context, call bridge and exports of the plugin reactor',
 'time-travel-panel': 'Time travel panel view, pointer and panel of the plugin SDK',
 'tool-run': 'Tool run ledger, driver and continuous controls of the plugin SDK',
 'time-travel': 'Time travel ledger, store state and supersessions of the plugin SDK',
 'instance-stores': 'Instance components of the plugin SDK: constructor state, stores, peer roster, action ingress and local interaction',
 'instance-history': 'Instance components of the plugin SDK: command history, host events and transactions',
 'instance-children': 'Instance component of the plugin SDK: child composition',
 'instance-jobs': 'Instance components of the plugin SDK: pending reserved work, tool operations and tool machines',
 'instance-docs': 'Instance components of the plugin SDK: media transfers and envelope jobs',
 'instance-extension': 'The InstanceExtension hook trait and its hook bundle types of the plugin SDK',
 'instance-commands': 'Instance use-cases of the plugin SDK: command routes, dispatch, groups and ingress over the command bundle',
 'instance-typed-ops': 'Instance use-cases of the plugin SDK: typed operations, typed commands and tool machine turns over the typed bundle',
 'instance-render': 'Instance use-cases of the plugin SDK: render, interaction state and media jobs over the render bundle',
 'instance-ladders': 'Instance use-cases of the plugin SDK: close, maintenance and envelope ladders over the ladder bundle',
 'time-travel-turns': 'Instance turns of the time travel feature and its InstanceExtension implementation',
 'tool-run-turns': 'Instance turns of the tool run feature and its InstanceExtension implementation',
 'instance-app': 'The VcsArtifactApp composer of the plugin SDK: component fields, constructors, public methods and the PluginApp impl',
 'builder': 'The PluginBuilder typestate of the plugin SDK',
 'testing-laws': 'Artifact app laws of the plugin SDK for plugin authors (feature artifact-app-testing)',
 'testing-acceptance': 'History edit acceptance, node drag history and ui action collection laws of the plugin SDK (feature artifact-app-testing)',
 'test-support': 'Source-text law support of the plugin SDK tests: reads a module tree at run time instead of embedding it',
}

# roots of the crates the plan table does not locate (7.5 and D8), and class-A / class-B decisions
FAMILY_ROOT = {
 'instance-stores': ('🧵️instance/🏪️stores', True, 'WP6'), 'instance-history': ('🧵️instance/📜️history', True, 'WP6'), 'instance-children': ('🧵️instance/🪆️children', True, 'WP6'),
 'instance-jobs': ('🧵️instance/👷️jobs', True, 'WP6'), 'instance-docs': ('🧵️instance/📄️docs', True, 'WP6'), 'instance-extension': ('🧵️instance/🧩️extensions', False, 'WP6'),
 'instance-commands': ('🧵️instance/🧷️command-routes', False, 'WP7'), 'instance-typed-ops': ('🧵️instance/🧾️typed-operations', False, 'WP7'), 'instance-render': ('🧵️instance/🖼️render', False, 'WP7'),
 'instance-ladders': ('🪜️maintenance-ladder', False, 'WP7'), 'time-travel-turns': ('⏪️time-travel/🧵️instance', False, 'WP7'), 'tool-run-turns': ('⏯️tool-run/🧵️instance', False, 'WP7'),
 'instance-app': ('🧵️instance', False, 'WP7'), 'builder': ('🏗️builder', False, 'WP7'), 'testing-laws': ('🧪️testing', False, 'WP7'), 'testing-acceptance': ('🧪️tests/🧪️history-edit-acceptance', False, 'WP7'),
}
FAMILY_MEMBERS = {
 'instance-stores': 'methods and fields of ConstructorState, InstanceStores, PeerRoster, ActionIngress, LocalInteraction cut from the instance files; `🎮️command/📥️ingress` (action_ingress); `🪆️child/👁️read/🧪️test-view`',
 'instance-history': 'CommandHistory, HostEvents, Transactions and the 875 free lines cut from the instance files',
 'instance-children': 'ChildComposition cut from the instance files',
 'instance-jobs': 'PendingReserved, ToolOperations, ToolMachineRuntime (`🧵️instance/🛠️tool-machine` residue), `🧵️instance/⏳️operation-progress`, `🪟️window/🫧️transient/🔁️document-replacement` (jobs part)',
 'instance-docs': 'MediaTransfers, EnvelopeJobs, MediaExportPreparation; `🎮️command/🎟️lease`',
 'instance-extension': 'the InstanceExtension trait file (`🧩️extensions`), the hook bundle types and `ExtensionCtx`',
 'instance-commands': 'CommandCtx: command routes, dispatch, group and ingress use-cases (`🧵️instance/📨️dispatch`, `🧵️instance/🤝️group`, `🧵️instance/🚪️ingress` residue)',
 'instance-typed-ops': 'TypedCtx: typed operations and tool machine turns',
 'instance-render': 'RenderCtx: render, interaction state, media jobs',
 'instance-ladders': 'LadderCtx, CloseCursors, MaintenanceCursors; `🪜️close-ladder`, `🪜️envelope-ladder`, `♻️lifecycle/🏪️stores`, `🧵️instance/🔄️full-operation`, `🪟️window/🫧️transient/🔁️document-replacement` (ladder part)',
 'time-travel-turns': 'TimeTravelCtx: the whole `🧵️instance` file of the time travel feature and `TimeTravelExtension`',
 'tool-run-turns': 'ToolRunCtx: the whole `🧵️instance` file of the tool run feature and `ToolRunExtension`',
 'instance-app': 'the composer: `🧵️instance/🦀️.rs` root, `🧵️instance/🔌️plugin-app-impl`, `editor_surface`/`viewer_surface` (I3), `InstanceExtensions` alias',
 'builder': '`PluginBuilder` typestate (`🏗️builder/🦀️.rs`); optional `canvas-runtime-assets`',
 'testing-laws': '`artifact_app_laws`, `🎟️caller`, `⚖️authoring-law` (`PLUG/⚖️authoring-law`)',
 'testing-acceptance': '`🧪️tests/🧪️history-edit-acceptance`, `🧪️tests/🧪️node-drag-history`, `🧪️tests/🧰️ui-action-collection`',
}
FAMILY_FAMDEPS = {
 'instance-stores': [], 'instance-history': ['instance-stores'], 'instance-children': ['instance-stores'], 'instance-jobs': ['instance-history', 'instance-children'], 'instance-docs': ['instance-jobs'],
 'instance-extension': TIER1, 'instance-commands': TIER1 + ['instance-extension'], 'instance-typed-ops': ['instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-extension'],
 'instance-render': ['instance-stores', 'instance-children', 'instance-jobs', 'instance-extension'], 'instance-ladders': TIER1 + ['instance-extension'],
 'time-travel-turns': TIER1 + ['instance-extension'], 'tool-run-turns': ['instance-stores', 'instance-children', 'instance-jobs', 'instance-extension'],
 'instance-app': TIER1 + ['instance-extension', 'instance-commands', 'instance-typed-ops', 'instance-render', 'instance-ladders', 'time-travel-turns', 'tool-run-turns'],
 'builder': ['instance-app'], 'testing-laws': ['builder', 'instance-app'], 'testing-acceptance': ['testing-laws'],
}
# the plan's table plus the D8 root for reactor-tasks
EXTRA_LOWER = {'reactor-tasks': dict(key='reactor-tasks', layer='L7', plan_lines=349, plan_kb=0, wp=4, root='⚛️reactor/⏲️tasks', plan_new=True, members='D8 Option B: spawn_task, queue_extension_invocation, TASK_RECORDS, TASK_RESUMES, ARMED_TIMERS and their records (definition scan by WP4 confirms)', deps=['actor-host', 'decl-emission'])}
LOWER_DEPS_ADD = {'reactor-core': ['reactor-tasks'], 'reactor-turn': ['reactor-tasks']}

FEATURES = {
 'artifact-app-testing': ['child', 'job', 'retained-command', 'decl-surface', 'instance-typed-ops', 'instance-commands', 'instance-app', 'app-builder', 'tool-run', 'window-kits', 'peer', 'document-media', 'reactor-core', 'time-travel-turns'],
 'media-turn-testing': ['decl-views', 'operation', 'document-media'],
 'reserved-producer-testing': ['decl-app'],
 'original-actor-context-testing': ['reactor-context', 'reactor-patches', 'reactor-lifecycle', 'runtime-ops', 'extension'],
}
# bins: (hosts, owner wp)
BINS = [(['none'], 1), (['window-config'], 1), (['decl-app', 'decl-views'], 2), (['decl-app', 'document-media'], 2), (['document-media'], 3), (['extension', 'reactor-turn'], 3), (['reactor-turn', 'time-travel'], 4),
        (['time-travel'], 5), (['time-travel'], 5)] + [(['TOP'], 7)] * 12

NATIVE_REL_DEP = {'reactor-wit-bridge': 'wasip2'}

def crate_names(key):
    return dict(name='semio-framework-plugin-' + key, id='plugin-' + key, project='@semio-tech/plugin-' + key + '-rs')

def folder_of(root):  # last segment of a root directory
    return root.split('/')[-1]

def monolith_root(path):
    if not os.path.isfile(path): return False
    head = open(path, encoding='utf-8').read(400)
    return 'skeleton of `' + FACADE + '`' not in head and 'Lift file' not in head

def build():
    plan = parse_61()
    crates = {}
    for r in plan + list(EXTRA_LOWER.values()):
        k = r['key']
        c = dict(key=k, kind='lower', wp=r['wp'], root=r['root'], plan_new=r['plan_new'], members=r['members'], deps=list(r['deps']) + [d for d in LOWER_DEPS_ADD.get(k, []) if d not in r['deps']], plan_lines=r['plan_lines'], plan_kb=r['plan_kb'], layer=r['layer'])
        crates[k] = c
    for k in FAMILY_ORDER:
        root, new, wp = FAMILY_ROOT[k]
        crates[k] = dict(key=k, kind='instance', wp=int(wp[2:]), root=root, plan_new=new, members=FAMILY_MEMBERS[k], deps=[], famdeps=FAMILY_FAMDEPS[k], layer='T', plan_lines=0, plan_kb=0)
    crates['test-support'] = dict(key='test-support', kind='support', wp=0, root=COMPANIONS_DIR + '/🧯️test-support', plan_new=True, members='', deps=[], layer='-', plan_lines=0, plan_kb=0)
    for i, (hosts, wp) in enumerate(BINS, 1):
        k = 'tests-%02d' % i
        crates[k] = dict(key=k, kind='companion', wp=wp, root=COMPANIONS_DIR + '/' + FRUIT[i - 1] + FE0F + k, plan_new=True, members='hosts: ' + ', '.join(hosts), hosts=hosts, deps=[], layer='-', plan_lines=0, plan_kb=0)
        EMOJI[k] = FRUIT[i - 1]
    order = ['test-support'] + LOWER_ORDER + FAMILY_ORDER + ['tests-%02d' % i for i in range(1, len(BINS) + 1)]
    unit = json.load(open(GEN + '/pwp0-unitdeps.json'))
    for k, c in crates.items():
        c.update(crate_names(k))
        c['rank'] = order.index(k)
        c['emoji'] = EMOJI[k] + FE0F
        c['desc'] = DESC.get(k) or ('Companion test crate %s of the plugin SDK partition (hosts: %s)' % (k, ', '.join(c.get('hosts', []))))
        c['cfg'] = 'wasip2' if k == 'reactor-wit-bridge' else 'both'
        c['features'] = [f for f, ks in FEATURES.items() if k in ks] + (['canvas-runtime-assets'] if k == 'builder' else [])
        c['class'] = 'B' if monolith_root(PLUG + '/' + c['root'] + '/🦀️.rs') else 'A'
        c['dir'] = PLUG_REL + '/' + c['root']
        c['pkg'] = c['dir'] + '/📦️packages/🦀️rust'
        c['tests_in_crate'] = c['kind'] == 'lower'
    # instance family lower deps from the token evidence; reactor-side edges are replaced by the I6 helper moves and the reactor-tasks crate
    HIGH = {'reactor-lifecycle', 'runtime-instances', 'runtime-ops', 'extension', 'reactor-patches', 'reactor-surfaces', 'reactor-core', 'reactor-jobs', 'runtime-exchange', 'reactor-turn', 'reactor-wit-bridge', 'reactor-context'}
    for k in FAMILY_ORDER:
        c = crates[k]
        ev = unit['instance'].get(k, {}).get('deps', [])
        low = [d for d in ev if d not in HIGH]
        if any(d in HIGH for d in ev): low.append('reactor-app')
        if 'reactor-core' in ev or k in ('instance-typed-ops', 'instance-commands'): low.append('reactor-tasks')
        if k == 'builder': low = ['app-builder', 'plugin-program', 'decl-registry', 'extension', 'identity', 'window-kits', 'decl-app', 'decl-surface', 'reactor-app', 'decl-views', 'decl-emission', 'job']
        if k == 'testing-laws': low = sorted(set(low + ['decl-registry', 'plugin-program', 'extension']), key=lambda d: crates[d]['rank'])
        if k in ('time-travel-turns',) and 'time-travel' not in low: low.append('time-travel')
        if k in ('tool-run-turns',) and 'tool-run' not in low: low.append('tool-run')
        if k == 'instance-extension': low = [d for d in ('time-travel', 'tool-run', 'decl-views', 'decl-app', 'child', 'interaction', 'decl-emission', 'job', 'command') if d in set(low) | {'time-travel', 'tool-run'}]
        c['deps'] = sorted(set(low), key=lambda d: crates[d]['rank']) + c['famdeps']
    # companions: hosts plus their transitive lower dependencies plus test-support
    def closure(ds):
        out = set(); st = list(ds)
        while st:
            d = st.pop()
            if d in out or d not in crates: continue
            out.add(d); st.extend(crates[d]['deps'])
        return out
    for k, c in crates.items():
        if c['kind'] != 'companion': continue
        hosts = [h for h in c['hosts'] if h in crates]
        if c['hosts'] == ['TOP']: hosts = ['builder', 'instance-app', 'testing-laws']
        if c['hosts'] == ['none']: hosts = []
        c['deps'] = sorted(closure(hosts), key=lambda d: crates[d]['rank']) + ['test-support']
    for k, c in crates.items():
        for d in c['deps']:
            assert d in crates, (k, d)
            assert crates[d]['rank'] < c['rank'], ('rank violation', k, d)
    return crates, order

if __name__ == '__main__':
    crates, order = build()
    print(len(crates), 'crates')
    for k in order:
        c = crates[k]; print(f"{c['rank']:3d} {k:24s} {c['kind']:9s} WP{c['wp']} {c['class']} {'NEW' if c['plan_new'] else '   '} deps={len(c['deps'])} {c['root'][:60]}")
    json.dump(crates, open(GEN + '/pwp0-crates.json', 'w'), ensure_ascii=False, indent=1)
    # uniqueness of docstring emoji
    import collections
    cnt = collections.Counter(c['emoji'] for c in crates.values()); print('dup emoji', [x for x, n in cnt.items() if n > 1])
    # path lengths
    longest = max((len((c['pkg'] + '/📋️project.json').encode()), k) for k, c in crates.items()); print('longest manifest path bytes', longest)
