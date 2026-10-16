import sys, os, re, json, shutil, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cut import *
KEPT_CLUSTER = {'interaction_gesture': 'interaction', 'interaction_schema': 'interaction', 'k_clipboard': 'kernel', 'k_content': 'kernel', 'k_message': 'kernel', 'k_cold': 'kernel', 'm_utilinit': 'manifest-actions', 'platform': 'action-bus'}
EXTRA_ITEMS = {
 'manifest-app-surface': '#[cfg(any(test, feature = "testkit"))]\n#[path = "../🧪️tests/🧰️testkit/🦀️.rs"]\npub mod testkit;',
 'interaction': '#[path = "🧬️schema/🦀️.rs"]\npub mod schema;',
 'action-bus': '#[path = "../🖥️platform/🦀️.rs"]\npub mod platform;',
}
OBSOLETE = [SRC['manifest'], SRC['m_canonical'], SRC['m_space']]
TYPEGEN_TEST = M + "🧬️schema/📽️projection/🧪️tests/🦀️.rs"
APP_LABEL = M + "🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs"
TYPEGEN_RE = re.compile(r'\n#\[cfg\(test\)\]\n#\[test\]\nfn exports_typescript_bindings\(\) \{.*?\n\}\n', re.S)
TESTKIT_TEST = M + "🛂️manifest/🧪️tests/🧰️testkit/🦀️.rs"
FRAMEWORK_RESERVED_TEST = M + "🛂️manifest/🧪️tests/🧩️framework-reserved/🦀️.rs"
APPWITH_RE = re.compile(r'\npub\(crate\) async fn app_with\(.*?\n\}\n', re.S)
def prepare(cut):
    cut.test_override = {}
    p = os.path.normpath(os.path.join(cut.R, APP_LABEL))
    t = open(p, encoding='utf-8').read()
    m = TYPEGEN_RE.search(t)
    if not m: raise SystemExit("typegen test block not found")
    cut.typegen_block = m.group(0)
    t = t[:m.start()] + t[m.end():]
    m = APPWITH_RE.search(t)
    if not m: raise SystemExit("app_with helper not found")
    cut.appwith_block = m.group(0)
    t = t[:m.start()] + '\n' + t[m.end():]
    t = 'use crate::testkit::app_with;\n' + t
    cut.test_override[p] = t
    for rel in (M + "⏪️time-travel/🕹️actions/🧪️tests/🦀️.rs", M + "⏯️tool-run/🕹️actions/🧪️tests/🦀️.rs"):
        q = os.path.normpath(os.path.join(cut.R, rel))
        tt = open(q, encoding='utf-8').read()
        lines = tt.split('\n')
        kept = [l for l in lines if 'crate::FRAMEWORK_RESERVED_CONTRIBUTIONS' not in l]
        if len(kept) == len(lines): raise SystemExit("reserved assert not found in " + rel)
        cut.test_override[q] = '\n'.join(kept)
    for rel in (M + "⏯️tool-run/🕹️actions/🧪️tests/🦀️.rs", M + "🛂️manifest/🧪️tests/🧩️reserved-contributions/🦀️.rs"):
        q = os.path.normpath(os.path.join(cut.R, rel))
        tt = cut.test_override.get(q) or open(q, encoding='utf-8').read()
        if 'crate::manifest::app_label_tests::app_with' not in tt: raise SystemExit('app_with use not found in ' + rel)
        cut.test_override[q] = tt.replace('crate::manifest::app_label_tests::app_with', 'TESTKIT_APP_WITH')
class Applied: pass
def apply(cut, dry=False):
    prepare(cut)
    rw = TestRewriter(cut)
    orig_rewrite = rw.rewrite
    out = {}
    apath = os.path.normpath(os.path.join(cut.R, APP_LABEL))
    old_open = open
    builds = {}
    for c in CLUSTERS:
        cut.kept_names[c] = set()
    for rnd in range(2):
        builds = {}
        for c in CLUSTERS:
            b = build_cluster_with(cut, c, rw, apath)
            builds[c] = b
            for o, names in list(b['cuse'].items()) + list(b['t_cuse'].items()):
                for n in names:
                    for key in cut.name2ents.get(n, []):
                        e = cut.ents[key]
                        if e['cluster'] == o and e['vis'] != 'pub' and e['kind'] != 'mod' and not n.startswith('<'):
                            cut.widen.add((e['file'], n))
        if rnd == 0: rw.log.clear()
    return builds, rw
def build_cluster_with(cut, c, rw, apath):
    info = CLUSTERS[c]
    units = cut.cluster_units[c]
    body, units = build_body(cut, c, units)
    if c in EXTRA_ITEMS: body = body.rstrip('\n') + '\n\n' + EXTRA_ITEMS[c] + '\n'
    cdir = os.path.join(cut.R, os.path.dirname(info['root']))
    tokens = cut.tokens_of(units) | cut.kept_tokens[c]
    imports, cuse = header_imports(cut, c, units, tokens)
    tests = test_entries(cut, c)
    ttext = {}; tokens_t = set()
    kept_tests = []
    for k, f in cut.files.items():
        if k in KEPT and KEPT_CLUSTER.get(k) == c:
            for u in f.units:
                if u.test and u.kind == 'mod':
                    m = ATTR_PATH_RE.search(u.attrs)
                    if m:
                        ab = os.path.normpath(os.path.join(os.path.dirname(os.path.join(cut.R, SRC[k])), m.group(1)))
                        ttext[ab] = rw.rewrite(open(ab, encoding='utf-8').read(), c)
    for name, ab, u in tests:
        raw = cut.test_override.get(os.path.normpath(ab)) or open(ab, encoding='utf-8').read()
        raw = raw.replace('TESTKIT_APP_WITH', 'crate::testkit::app_with' if c == 'manifest-app-surface' else 'semio_framework_manifest_app_surface::testkit::app_with')
        new = rw.rewrite(raw, c)
        ttext[ab] = new
        tokens_t |= set(TOK.findall(mask(new)))
    provided = set(cut.cluster_names[c]) | set(imports.values()) | {n for v in cuse.values() for n in v}
    t_imports, t_cuse = header_imports(cut, c, units, tokens_t - provided, extra_skip=provided)
    return dict(c=c, body=body, units=units, imports=imports, cuse=cuse, tests=tests, ttext=ttext, t_imports=t_imports, t_cuse=t_cuse, dir=cdir)
if __name__ == '__main__':
    cut = Cut(sys.argv[1] if len(sys.argv) > 1 else REPO)
    builds, rw = apply(cut)
    for c, b in builds.items():
        print(c, 'deps', sorted(b['cuse']), 'dev+', sorted(set(b['t_cuse']) - set(b['cuse'])))
    print(rw.log)

def write_text(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, 'w', encoding='utf-8').write(text)
def write_out(cut, builds, rw):
    R = cut.R
    roots = {}
    for c, b in builds.items():
        roots[c] = os.path.join(R, CLUSTERS[c]['root'])
    texts = {c: render_root(cut, b) for c, b in builds.items()}
    for c, p in roots.items(): write_text(p, texts[c])
    for c, b in builds.items():
        for ab, t in b['ttext'].items(): write_text(ab, t)
    for rel in OBSOLETE:
        p = os.path.join(R, rel)
        if os.path.exists(p) and all(os.path.normpath(p) != os.path.normpath(r) for r in roots.values()): os.remove(p)
    kit = rw.rewrite('use super::*;\n' + cut.appwith_block.replace('pub(crate) async fn app_with', 'pub async fn app_with'), 'manifest-app-surface')
    write_text(os.path.join(R, TESTKIT_TEST), '//! 🧰️ Shared test kit of the app surface: the minimal editor app the surface and the reserved features test against.\n' + kit)
    write_text(os.path.join(R, FRAMEWORK_RESERVED_TEST), '//! 🧩️ The framework hosts every reserved feature contribution.\nuse super::*;\n\n#[test]\nfn the_framework_hosts_the_history_edit_and_tool_run_contributions() {\n    assert!(FRAMEWORK_RESERVED_CONTRIBUTIONS.iter().any(|contribution| contribution.id == history_edit::RESERVED.id), "the framework hosts the history-edit contribution");\n    assert!(FRAMEWORK_RESERVED_CONTRIBUTIONS.iter().any(|contribution| contribution.id == tool_run::RESERVED.id), "the framework hosts the tool run contribution");\n}\n')
    tp = os.path.join(R, TYPEGEN_TEST)
    write_text(tp, '//! 🧬️ The TypeScript bindings rendered from the owned schema metadata equal the checked-in generated file.\nuse super::*;\n' + cut.typegen_block.replace('\n#[cfg(test)]\n#[test]', '\n#[test]', 1).replace('crate::schema_metadata', 'crate::schema_metadata').replace('../../🤖️generated/🪪️manifest/🟦️.ts', '../../../🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts'))
    pp = os.path.join(R, SRC['platform'])
    t = open(pp, encoding='utf-8').read()
    t = t.replace('use crate::action_bus::ActionBus;', 'use crate::ActionBus;').replace('use crate::ui::AppDefinition;', 'use semio_framework_manifest_app_surface::AppDefinition;')
    write_text(pp, t)
    return texts

def audit(cut, builds):
    emitted = set()
    for c, b in builds.items():
        for u in b['units']: emitted.add((u.file, u.start))
        for name, ab, u in b['tests']: emitted.add((u.file, u.start))
    problems = []
    for k, f in cut.files.items():
        for u in f.units:
            if (u.file, u.start) in emitted: continue
            if u.kind in ('use',): 
                if u.file in KEPT: continue
                body = re.sub(r'//[^\n]*', '', u.text)
                if re.search(r'(^|\n)\s*pub(\([^)]*\))?\s+use\b', body) and not u.test:
                    if k != 'root' and cut.assign_use(k, u, body) is None: problems.append(('pub use dropped', k, u.start, u.text[:80]))
                continue
            if u.kind == 'comment': continue
            if k in KEPT: continue
            if k == 'root': continue
            if u.test and u.kind == 'mod':
                key = (k, u.name)
                if key not in TESTS: problems.append(('test mount unassigned', k, u.start, u.name))
                continue
            problems.append(('unemitted', k, u.start, u.kind, u.name, u.text[:60].replace('\n', ' ')))
    return problems

NS_OF_FILE = lambda k: 'kernel' if (k == 'kernel' or k.startswith('k_')) else ('manifest' if (k == 'manifest' or k.startswith('m_')) else ('interaction' if k.startswith('interaction') else k))
def api_names(cut):
    """api[ns][cluster] = set(names) of the public surface the old module exposed"""
    api = collections.defaultdict(lambda: collections.defaultdict(set))
    for key, e in cut.ents.items():
        c = e['cluster']
        if c in (None, 'UNASSIGNED') or e['name'].startswith('<'): continue
        u0 = e['units'][0]
        if any(u.test for u in e['units'][:1]): continue
        pub = e['vis'] == 'pub' or (e['kind'] == 'macro_rules!' and 'macro_export' in u0.attrs)
        if e['file'] in KEPT:
            if e['file'] == 'platform': 
                continue
            continue
        if not pub: continue
        api[NS_OF_FILE(e['file'])][c].add(e['name'])
    for k, f in cut.files.items():
        if k == 'root': continue
        for u in f.units:
            if u.kind != 'use' or u.test: continue
            body = re.sub(r'//[^\n]*', '', u.text)
            if not re.search(r'(^|\n)\s*pub(\([^)]*\))?\s+use\b', body): continue
            c = cut.assign_use(k, u, body)
            if not c: continue
            for pub, items in file_imports(u.text):
                if not pub: continue
                for path, alias in items:
                    if alias == '*': continue
                    api[NS_OF_FILE(k)][c].add(alias)
    # kept children: kernel return_content / return_message modules, gesture mod, synthetic schema/platform
    api['interaction']['interaction'] |= {'schema'}
    return api


KEEP_ROOT_PREFIX = ('ui_wgpu', 'semio_framework_io_schema', 'semio_framework_io_sqlite_snapshot', 'semio_framework_value', 'semio_framework_diagnostic', 'semio_framework_mesh_engine', 'semio_framework_artifact_reference', 'action_bus', 'platform')
def render_facade(cut, api):
    root = cut.files['root']
    taken = set()
    keep = []
    kernel_pub = None
    for u in root.units:
        if u.kind != 'use' or u.test: continue
        body = re.sub(r'//[^\n]*', '', u.text)
        if not re.search(r'(^|\n)\s*pub\s+use\b', body): continue
        items = [it for pub, its in file_imports(u.text) if pub for it in its]
        if not items: continue
        first = items[0][0]
        head = first[0] if first[0] else first[1]
        if re.search(r'pub\s+use\s+manifest::kernel::', body):
            kernel_pub = u
            for p, a in items: taken.add(a)
            continue
        if head in KEEP_ROOT_PREFIX:
            keep.append(u)
            for p, a in items: taken.add(a)
    names = {ns: {c: sorted(v) for c, v in m.items()} for ns, m in api.items()}
    out = ["//! 🥅️ Render-independent framework facade: explicit re-exports of the carved producer crates under the module paths the framework always exposed (`kernel`, `manifest`, `ui`, `abi`, `action_bus`, `platform`, `interaction`, `source_projection`).", ""]
    def group(ns, indent='    ', skip=(), pub='pub use'):
        lines = []
        for c in sorted(names.get(ns, {})):
            ns_names = [n for n in names[ns][c] if n not in skip]
            if not ns_names: continue
            lines.append(f"{indent}{pub} {pkg_ident(c)}::{{{', '.join(ns_names)}}};")
        return lines
    out.append("pub mod kernel {")
    out += group('kernel')
    out.append("}")
    out.append("")
    out.append("pub mod manifest {")
    out.append("    pub use crate::kernel;")
    out += group('manifest')
    out.append("}")
    out.append("")
    out += ["pub use manifest as ui;", "pub use semio_framework_abi as abi;", "pub use semio_framework_action_bus as action_bus;", "pub use semio_framework_action_bus::platform;", "pub use semio_framework_interaction as interaction;", "pub use semio_framework_source_projection as source_projection;", ""]
    const_unit = [u for u in root.units if u.kind == 'const' and u.name == 'FRAMEWORK_RESERVED_CONTRIBUTIONS']
    if const_unit:
        out += ["//#region 🧩️ReservedFeatures", "pub use semio_framework_time_travel_actions as history_edit;", "pub use semio_framework_tool_run_actions as tool_run;", "pub use semio_framework_onboarding as onboarding;", "", re.sub(r'(?m)^\s*//#(end)?region.*$\n?', '', const_unit[0].text).rstrip('\n'), "//#endregion 🧩️ReservedFeatures", ""]
    for u in keep: out.append(re.sub(r'(?m)^\s*//#(end)?region.*$\n?', '', u.text))
    if kernel_pub is not None:
        t = re.sub(r'(?m)^\s*//#(end)?region.*$\n?', '', kernel_pub.text).replace('pub use manifest::kernel::', 'pub use kernel::')
        out.append(t)
    skip = set(taken)
    out.append("")
    for ns in ('abi', 'interaction'):
        lines = group(ns, indent='', skip=skip)
        out += lines
        for c, v in names.get(ns, {}).items(): skip |= set(v)
    mlines = group('manifest', indent='', skip=skip)
    out += mlines
    out.append("")
    out.append('//#region 🧬️SchemaMetadata\n#[cfg(test)]\n#[path = "../../🔨️modules/🧬️schema/📽️projection/🦀️.rs"]\npub mod schema_metadata;\n//#endregion 🧬️SchemaMetadata')
    out.append("")
    out.append('#[cfg(test)]\n#[path = "../../🔨️modules/🧬️schema/📽️projection/🧪️tests/🦀️.rs"]\nmod typegen_tests;')
    out.append("")
    out.append('#[cfg(test)]\n#[path = "../../🔨️modules/🛂️manifest/🧪️tests/🧩️framework-reserved/🦀️.rs"]\nmod framework_reserved_tests;')
    return '\n'.join(out) + '\n'

from cargo import *
def extern_sets(cut, b, facade_deps):
    idents = {k[1] for k in facade_deps}
    c = b['c']
    lib_tokens = cut.tokens_of(b['units']) | cut.kept_tokens[c]
    lib_ext = {p.split('::')[0] for p in b['imports']} | (lib_tokens & idents)
    lib_ext = {e for e in lib_ext if e in idents}
    t_tokens = set()
    for ab, t in b['ttext'].items(): t_tokens |= set(TOK.findall(mask(t)))
    dev_ext = {p.split('::')[0] for p in b['t_imports']} | (t_tokens & idents)
    dev_ext = {e for e in dev_ext if e in idents}
    return lib_ext, dev_ext
def crate_deps(b):
    lib = set(b['cuse'])
    dev = set(b['t_cuse'])
    # tests reference crates of the cluster's own ttext qualified paths
    return lib, dev
def qualified_cluster_refs(b, texts):
    refs = set()
    for t in texts:
        for m in re.finditer(r'\bsemio_framework_([a-z_]+?)::', t):
            ident = 'semio_framework_' + m.group(1)
            for c in CLUSTERS:
                if CLUSTERS[c]['pkg'].replace('-', '_') == ident: refs.add(c)
    return refs

FACADE_LIB_EXT = ['ui_wgpu', 'semio_framework_diagnostic', 'semio_framework_value', 'semio_framework_mesh_engine', 'semio_framework_artifact_reference', 'semio_framework_io_schema', 'semio_framework_io_sqlite_snapshot', 'semio_framework_actor', 'semio_framework_job', 'semio_framework_trace']
FACADE_DEV_EXT = ['semio_framework_async']
def render_facade_cargo(R_live, facade_deps, new_clusters):
    text = open(os.path.join(R_live, FACADE_PKG, 'Cargo.toml'), encoding='utf-8').read()
    head = text.split('[dependencies]')[0]
    tail = text[text.index('[[test]]'):]
    pd = os.path.join(R_live, FACADE_PKG)
    lines = ['[dependencies]']
    for e in FACADE_LIB_EXT:
        name, s = facade_deps[('dependencies', e)]
        lines.append(f'{name} = {s}')
    for c in sorted(new_clusters):
        rel = os.path.relpath(os.path.join(R_live, pkg_dir(c)), pd)
        lines.append(f'{CLUSTERS[c]["pkg"]} = {{ path = "{rel}" }}')
    lines += ['', '[dev-dependencies]']
    lines.append('serde_json = { workspace = true }')
    for e in FACADE_DEV_EXT:
        name, s = facade_deps[('dev-dependencies', e)]
        lines.append(f'{name} = {s}')
    return head + '\n'.join(lines) + '\n\n' + tail

from support import *
SUPPORT_FILES = ['🧰️framework/Cargo.toml', '📋️build-owners.json', '.vscode/launch.json', '.vscode/🧩️launch.seed.jsonc', FACADE_PKG + '/Cargo.toml']
def write_support(cut, builds, api, R, texts):
    facade_deps = parse_facade_deps(R)
    rows = []; members = []
    infos = {}
    qrefs_lib = {}; qrefs_dev = {}
    for c, b in builds.items():
        lib_texts = [mask(b['body'])]
        if c == 'action-bus': lib_texts.append(mask(open(os.path.join(R, SRC['platform']), encoding='utf-8').read()))
        lib_refs = qualified_cluster_refs(b, lib_texts) | set(b['cuse'])
        dev_refs = qualified_cluster_refs(b, [mask(t) for t in b['ttext'].values()]) | set(b['t_cuse'])
        lib_ext, dev_ext = extern_sets(cut, b, facade_deps)
        if c == 'action-bus':
            lib_ext |= {e for e in ('semio_framework_value',) if False}
        lib_refs.discard(c); dev_refs.discard(c)
        feats = {'testkit': []} if c == 'manifest-app-surface' else None
        devf = {'manifest-app-surface': ['testkit']} if any('semio_framework_manifest_app_surface::testkit' in t for t in b['ttext'].values()) and c != 'manifest-app-surface' else None
        if devf: dev_refs = dev_refs | {'manifest-app-surface'}
        write_text(os.path.join(R, pkg_dir(c), 'Cargo.toml'), render_cargo(R, c, lib_refs, lib_ext, dev_refs, dev_ext, facade_deps, feats, devf))
        write_text(os.path.join(R, pkg_dir(c), '📋️project.json'), render_project(c))
        rows.append(dict(name=f'@semio-tech/{c}-rs', root=pkg_dir(c), manifest=pkg_dir(c) + '/📋️project.json'))
        members.append(pkg_dir(c).split('/', 1)[1])
        infos[c] = dict(lib_refs=sorted(lib_refs), dev_refs=sorted(dev_refs), lib_ext=sorted(lib_ext), dev_ext=sorted(dev_ext))
    write_text(os.path.join(R, FACADE_PKG, 'Cargo.toml'), render_facade_cargo(R, facade_deps, list(CLUSTERS)))
    write_text(os.path.join(R, FACADE_PKG, '🦀️.rs'), render_facade(cut, api))
    add_workspace_members(os.path.join(R, '🧰️framework/Cargo.toml'), members)
    add_owner_rows(os.path.join(R, '📋️build-owners.json'), rows)
    for f in ('.vscode/launch.json', '.vscode/🧩️launch.seed.jsonc'):
        add_launch_routes(os.path.join(R, f), list(CLUSTERS))
    return infos
