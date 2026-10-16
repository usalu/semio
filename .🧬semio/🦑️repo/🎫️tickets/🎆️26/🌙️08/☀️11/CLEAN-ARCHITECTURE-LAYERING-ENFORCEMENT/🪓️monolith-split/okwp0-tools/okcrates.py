# 🧭️ The frozen crate table of the OS kernel partition (OS-KERNEL-WP0): names, nodes, folders, work packages, cfg, first-party and external dependencies.
# Sources: PLAN-OS-KERNEL-CRATES.md 6.1, 6.2, 6.3, Appendix D (parsed by planmodel.py into okwp0-planmodel.json), the kernel manifest (okwp0-kernel-cargo.json), the decisions D1..D6.
import json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = HERE + '/../🗑️generated'
R = '/Users/ueli/Documents/semio'
M_REL = '🧰️framework/🛍️products/💻️os/🔨️modules'
K_REL = '🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
M = R + '/' + M_REL
K = R + '/' + K_REL
FE0F = '️'
PREFIX = 'semio-framework-'
TOP = 'semio-framework-os-kernel'

NOTP2 = 'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'
NATIVE = 'cfg(not(target_arch = "wasm32"))'
BROWSER = 'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'

NATIVE_ONLY = {'os-pack-tools', 'os-spr-cli', 'os-store-sync-native'}
NOT_P2 = {'os-extension', 'os-store-sync'}
WASM_ONLY = {'os-store-sync-wasm'}

RANK = ['os-list-delta', 'os-semio', 'os-space-sqlite-cells', 'os-plugin-registry-schema', 'os-extension', 'os-pack-tools', 'os-spr', 'os-dsl', 'os-store-space-schema', 'os-store-child', 'os-store-ledger',
        'os-store-retirement', 'os-store-replay', 'os-store-read', 'os-store-backbone', 'os-store-canonical-json', 'os-store-one-item-publication', 'os-store-operation-wire', 'os-store-space-history', 'os-store-text-io',
        'os-store-snapshot-clone', 'os-store-one-item-batch', 'os-store-owned-schema-fields', 'os-store-owned-schema-decode', 'os-store-member-open', 'os-store-engine-owners', 'os-store-codec-registry',
        'os-io', 'os-store-initialization-runtime', 'os-store-durable-group-wire', 'os-store-engine-core', 'os-store-engine-live', 'os-store-engine-stage', 'os-store-engine',
        'os-store-initialization-replay', 'os-store-codec-factory', 'os-store-space-member', 'os-store-document', 'os-store-member-open-store', 'os-store-config', 'os-store-durable-group',
        'os-store-composition', 'os-store-space', 'os-store-test-support', 'os-store-sync', 'os-store-sync-native', 'os-store-sync-wasm', 'os-spr-cli']
COMPANIONS = ['os-store-testing-fixtures', 'os-store-tests-01', 'os-store-tests-02', 'os-store-tests-03', 'os-store-tests-04', 'os-store-tests-05', 'os-store-tests-06', 'os-store-tests-07', 'os-store-tests-08',
              'os-store-tests-09', 'os-store-tests-10', 'os-store-sync-tests-1', 'os-store-sync-tests-2', 'os-dsl-tests', 'os-dsl-module-tests', 'os-dsl-fixture-sweep-tests', 'os-spr-laws-tests', 'os-store-member-open-tests',
              'os-store-durable-group-tests']
ALL_KEYS = RANK + COMPANIONS

# nodes the plan names that collide with an existing sibling emoji or an existing test unit file are replaced here (decision recorded in RESULT-OS-KERNEL-WP0.md section 4)
NODE_OVERRIDE = {
    'os-store-test-support': 'M/🏪️store/🧰️test-support',
    'os-store-tests-01': 'M/🏪️store/🧪️tests/🏙️bin-01', 'os-store-tests-02': 'M/🏪️store/🧪️tests/🌆️bin-02', 'os-store-tests-03': 'M/🏪️store/🧪️tests/🌈️bin-03',
    'os-store-tests-04': 'M/🏪️store/🧪️tests/🌌️bin-04', 'os-store-tests-05': 'M/🏪️store/🧪️tests/🌅️bin-05', 'os-store-tests-06': 'M/🏪️store/🧪️tests/🌴️bin-06',
    'os-store-tests-07': 'M/🏪️store/🧪️tests/🌲️bin-07', 'os-store-tests-08': 'M/🏪️store/🧪️tests/🍃️bin-08', 'os-store-tests-09': 'M/🏪️store/🧪️tests/🍎️bin-09',
    'os-store-tests-10': 'M/🏪️store/🧪️tests/🍯️bin-10',
    'os-store-sync-tests-1': 'M/🏪️store/🔄️sync/🧪️tests/🍰️bin-1', 'os-store-sync-tests-2': 'M/🏪️store/🔄️sync/🧪️tests/🥧️bin-2',
    'os-dsl-tests': 'M/🗣️dsl/🧪️tests/🥪️companion', 'os-dsl-module-tests': 'M/🗣️dsl/🧪️tests/🌴️module-companion',
    'os-spr-laws-tests': 'M/📡️spr/🧪️tests/🥣️laws-companion',
    'os-store-member-open-tests': 'M/🏪️store/🧩️composition/🚪️open/🧪️tests/🍰️companion',
    'os-store-durable-group-tests': 'M/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🥧️companion',
}

DESC = {
    'os-list-delta': 'Positional list deltas of the OS kernel: removed, inserted, moved and patched rows with base and after coordinates',
    'os-semio': 'The .semio container of the OS kernel: envelope wrap and unwrap, text preamble split',
    'os-space-sqlite-cells': 'Bounded ordinal and literal cell builders of the space SQLite snapshot',
    'os-plugin-registry-schema': 'Plugin registry deployment schema and the installed native catalog of the OS kernel',
    'os-store-space-schema': 'Space history document, snapshot entities and space mutation schemas of the Store',
    'os-extension': 'The .sxt extension package format: pack, unpack and verify (not for wasm32-wasip2 guests)',
    'os-pack-tools': 'The pack command line: inspect, verify, hash, to-dsl, from-dsl, diff, and the command-file control',
    'os-spr': 'The spr component of the OS kernel: range extraction, slice verification, content frontier, file lifecycle and materialization',
    'os-dsl': 'The OS product grammars: dsl component, family kits, grammar, language server and the schema registry',
    'os-store-child': 'Artifact children, child references and the retained clone of a composed child',
    'os-store-ledger': 'The Store edit-message ledger, cursor revision accumulator, history commit reservation and edit retirement',
    'os-store-retirement': 'Whole physical retirement of empty resident Store metadata backings',
    'os-store-replay': 'Effective forwards, edit replay with the prefix snapshot ring, pair merges and replay budget',
    'os-store-read': 'Opaque Store snapshot reads, owner catalogs and per-owner retirement factories',
    'os-store-backbone': 'Hot-swappable Store backbones: memory, channel, port, registry and their retirement',
    'os-store-canonical-json': 'Store-owned canonical JSON traversal, value admission, borrowed typed nodes and the bounded reader',
    'os-store-one-item-publication': 'The retained one-item publication protocol: live authority, prepared owner, seal, sealer and operation-wire vocabulary',
    'os-store-operation-wire': 'Prestaged operation bytes from borrowed typed sources and the paged operation output',
    'os-store-text-io': 'The Store serialization seam and the document text format: parse, print, operation identity',
    'os-store-snapshot-clone': 'Bounded retained-clone publication preparation: handoff, custody, law, metadata and config apply',
    'os-store-one-item-batch': 'Batch stage and publication, member one-item publication, owned batch emission and paged one-item preparation',
    'os-store-owned-schema-fields': 'Envelope field decoders: field catalog, SPR edit and array authorities, history entry decoders',
    'os-store-owned-schema-decode': 'The paged, credit-bounded envelope decode authority, completed records and fresh VCS authority',
    'os-store-member-open': 'The member-open input pipeline: request, admission, receiving, genesis framing, dictionary and history verification',
    'os-store-engine-owners': 'DocumentStoreOwners, the member store producer and the store-close port',
    'os-store-codec-registry': 'All Store codec and IO registries, dialect migrations, native snapshot capability and the assembly registry plan',
    'os-io': 'The OS artifact IO mechanism: typed registry, routing, IO fallback and wire framing',
    'os-store-space-history': 'SQLite snapshot IO of space history: native, preflight, admission, validation and receiving',
    'os-store-initialization-runtime': 'The Store initialization runtime: digest, edit index, owner catalog and clock helpers',
    'os-store-durable-group-wire': 'Durable owned-group wire vocabulary: schema constants, anchors, decisions and outcomes',
    'os-store-engine-core': 'ArtifactStore components for log, owners, wire and cursor, and the Store mint witness',
    'os-store-engine-live': 'ArtifactStore live and replay components and the StoreParts bundle',
    'os-store-engine-stage': 'Durable-group staging functions over the StoreParts bundle',
    'os-store-engine': 'The ArtifactStore composer: use-case methods, delegates and the store-close host implementation',
    'os-store-initialization-replay': 'Store initialization replay: the ArtifactStoreReplayInitializer state machine',
    'os-store-codec-factory': 'The artifact codec factory and codec reduction helpers of the Store',
    'os-store-space-member': 'SpaceMember, MemberFactory, member directory, link resolver and the unsupported open stubs',
    'os-store-document': 'Document hydration, history and the mounted pack of the Store',
    'os-store-member-open-store': 'The InitialMemberStoreOpen operation and the retained member store open',
    'os-store-config': 'The config store: config envelope, hydration and apply',
    'os-store-durable-group': 'Durable owned-group journal records and the three-store commit and recovery machinery',
    'os-store-composition': 'Store composition coordinator, closure, graph and group publication',
    'os-store-space': 'SpaceHost, member store creation and opening, and the SQLite snapshot registration glue',
    'os-store-test-support': 'Test support of the Store: demo owners, preparation doubles and scratch directories',
    'os-store-sync': 'The Store sync session, ArtifactHost and the shared socket and link vocabulary',
    'os-store-sync-native': 'The native Store sync actor and the folder event log storage',
    'os-store-sync-wasm': 'The browser wasm32 Store sync actor',
    'os-spr-cli': 'The spr command line: inspect, verify, hash, log, compile, decompile, diff, compact, repair, upgrade',
    'os-store-testing-fixtures': 'Companion test fixtures of the Store: demo, lossy, severity, timestamped and validated mutation fixtures',
    'os-dsl-tests': 'Companion tests of os-dsl: record laws, number and refusal laws and the protocol record through the kernel facade',
    'os-dsl-module-tests': 'Companion tests of os-dsl: the unit tests of the family kits, grammar, registry and receiving modules',
    'os-dsl-fixture-sweep-tests': 'Companion tests of os-dsl: the DSL fixture sweep',
    'os-spr-laws-tests': 'Companion tests of os-spr: protocol laws and mutation laws',
    'os-store-member-open-tests': 'Companion tests of os-store-member-open: factory, dictionary, open and history',
    'os-store-durable-group-tests': 'Companion tests of os-store-durable-group: journal and recovery laws',
}
for i in range(1, 11): DESC['os-store-tests-%02d' % i] = 'Companion test bin %02d of the OS kernel Store: the root-mounted store unit tests packed by subject' % i
for i in (1, 2): DESC['os-store-sync-tests-%d' % i] = 'Companion test bin %d of the Store sync: actor, backbone parity and link laws' % i

def crate_name(key): return PREFIX + key
def crate_id(key): return key
def project_name(key): return '@semio-tech/' + key + '-rs'

def node_path(key, model):
    n = NODE_OVERRIDE.get(key) or model['nodes'][key]['node']
    assert n.startswith('M/'), (key, n)
    return M_REL + '/' + n[2:]
def node_abs(key, model): return R + '/' + node_path(key, model)
def pkg_rel(key, model): return node_path(key, model) + '/📦️packages/🦀️rust'
def pkg_abs(key, model): return R + '/' + pkg_rel(key, model)
def ups(key, model): return len(pkg_rel(key, model).split('/'))

# extern ident -> (manifest key, lib ident)
IDENT_KEY = {'protocol': 'semio-framework-replication', 'pack': 'semio-framework-pack', 'vcs': 'semio-framework-os-vcs', 'web_sys': 'web-sys', 'js_sys': 'js-sys', 'wasm_bindgen': 'wasm-bindgen',
             'tokio_tungstenite': 'tokio-tungstenite', 'serde': 'serde', 'serde_json': 'serde_json', 'tokio': 'tokio', 'futures': 'futures', 'ureq': 'ureq', 'dsl_derive': 'dsl_derive', 'blake3': 'blake3'}
def ident_key(ident):
    if ident in IDENT_KEY: return IDENT_KEY[ident]
    return ident.replace('_', '-')

def old_entries(cargo):
    table = {}
    def add(key, kind, spec): table.setdefault(key, []).append((kind, spec))
    for k, v in cargo['dependencies'].items(): add(k, 'plain', v)
    for tk, tv in cargo['target'].items():
        kind = {NOTP2: 'notp2', NATIVE: 'native', BROWSER: 'browser'}[tk]
        for k, v in tv.get('dependencies', {}).items(): add(k, kind, v)
    for k, v in cargo['dev-dependencies'].items(): add(k, 'dev', v)
    return table

def load():
    model = json.load(open(GEN + '/okwp0-planmodel.json'))
    cargo = json.load(open(GEN + '/okwp0-kernel-cargo.json'))
    return model, cargo

# corrections of the plan's 6.2 external lists found by reading the sources (WP1 crates are scanned exactly, see RESULT section 5)
EXT_ADD = {
    'os-list-delta': ['semio_framework_value', 'protocol'],
}
COMPANION_DEPS = {
    'os-store-testing-fixtures': ([], ['semio_framework_value', 'protocol', 'semio_framework_dsl_record']),
    'os-dsl-module-tests': (['os-dsl', 'os-pack-tools'], ['semio_framework_value', 'semio_framework_dsl_record', 'protocol']),
    'os-dsl-tests': (['os-dsl', 'os-pack-tools'], ['semio_framework_value', 'semio_framework_dsl_record', 'protocol']),
    'os-dsl-fixture-sweep-tests': (['os-dsl', 'os-pack-tools'], ['semio_framework_value', 'semio_framework_dsl_record', 'protocol']),
    'os-spr-laws-tests': (['os-spr', 'os-pack-tools'], ['semio_framework_value', 'protocol']),
    'os-store-member-open-tests': (['os-store-member-open', 'os-spr'], ['semio_framework_value', 'protocol']),
    'os-store-durable-group-tests': (['os-store-durable-group', 'os-store-engine', 'os-store-testing-fixtures'], ['semio_framework_value', 'protocol']),
    'os-store-sync-tests-1': (['os-store-sync', 'os-store-sync-native', 'os-store-test-support', 'os-store-testing-fixtures'], ['semio_framework_value', 'protocol']),
    'os-store-sync-tests-2': (['os-store-sync', 'os-store-sync-native', 'os-store-test-support', 'os-store-testing-fixtures'], ['semio_framework_value', 'protocol']),
}
for i in range(1, 11): COMPANION_DEPS['os-store-tests-%02d' % i] = (['os-store-space', 'os-store-test-support', 'os-store-testing-fixtures'], ['semio_framework_value', 'protocol'])

def build():
    model, cargo = load()
    crates = {}
    for key in RANK:
        c = model['lib'][key]
        cfg = 'native' if key in NATIVE_ONLY else 'wasm' if key in WASM_ONLY else 'notp2' if key in NOT_P2 else 'all'
        ids = list(c.get('external_idents', [])) + EXT_ADD.get(key, [])
        ext = sorted(set(ident_key(i) for i in ids))
        crates[key] = dict(key=key, kind='lib', wp=c['wp'], cfg=cfg, deps=[d for d in c['deps'] if d in RANK], ext=ext, plan_cfg=c['cfg'])
    for key in COMPANIONS:
        deps, ids = COMPANION_DEPS[key]
        crates[key] = dict(key=key, kind='companion', wp=8, cfg='all', deps=deps, ext=sorted(set(ident_key(i) for i in ids)), plan_cfg='dev only')
    try:
        import okreal
        for key, spec in okreal.REAL.items():
            c = crates[key]
            c['deps'] = spec.get('deps', c['deps'])
            c['ext'] = sorted(set(ident_key(i) if '_' in i and not i.startswith('semio') else i for i in spec.get('ext', [])))
            c['dev'] = sorted(set(spec.get('dev', [])))
            c['features'] = spec.get('features', [])
            c['dep_features'] = spec.get('dep_features', {})
            c['real'] = True
    except ImportError:
        pass
    for key, c in crates.items():
        c['name'] = crate_name(key); c['id'] = crate_id(key); c['project'] = project_name(key)
        c['node'] = node_path(key, model); c['pkg'] = pkg_rel(key, model)
    return crates, model, cargo
