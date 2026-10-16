"""🏗️ WP8 generator: builds every new/changed file in memory (OUT) and the delete list (DEL); `apply` writes them through the snapshotting writer."""
import sys, os, re, json, collections, copy
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C, wp8lib as L, pathrw, vis, themes, genf, genbins, buildf as B, packing as P, part, prelude as PR, units as U, mounts
from common import FE, RSF, FIX_CRATE
OUT = collections.OrderedDict(); DEL = []; NOTES = []
def put(path, text):
    assert path not in OUT, path
    OUT[path] = text
def load():
    plan = C.load_plan()
    src, its, names = part.load()
    labels = {int(k): v for k, v in plan['labels'].items()}
    F = set(plan['F'])
    return plan, src, its, names, labels, F
def unit_level_uses(its, mode):
    """The `use` items of the unit file (except `use super::*`), rewritten for mode 'f' or 'bin'."""
    out = []
    for r in its:
        if r['kind'] != 'use': continue
        t = r['text'].strip()
        if re.match(r'^(?:#\[[^\]]*\]\s*)*use\s+super::\*\s*;', t): continue
        if 'fixture_mutations' in t: continue
        rw = pathrw.Rewriter(mode, depth=1)
        t2 = rw.rewrite(t)
        t2 = re.sub(r'^pub\(super\)\s+', '', t2)
        out.append(t2.strip().replace('; use ', ';\nuse '))
    return out
FIXB_CRATE = 'semio_framework_os_store_testing_doubles'
def fix_uses(mode):
    pre = 'crate::' if mode == 'a' else FIX_CRATE + '::'
    return [
        'use ' + pre + 'demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};',
        'use ' + pre + 'lossy::{LossyMutation, SetN as LossySetN};',
        'use ' + pre + 'severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};',
        'use ' + pre + 'timestamped::{SetN as TimestampedSetN, TimestampedMutation};',
        'use ' + pre + 'validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};',
    ]
def join_items(its, rows, mode, depth=1, extra_rw=None):
    parts = []; unresolved = []
    prev = None
    for r in rows:
        rw = pathrw.Rewriter(mode, depth=depth)
        t = rw.rewrite(r['text'])
        unresolved += [(r['name'], u) for u in rw.unresolved]
        if mode in ('a', 'b'):
            inherent = r['kind'] == 'impl' and not r.get('trait')
            if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'type', 'union') or inherent:
                t = vis.widen(t, r['kind'], inherent)
        if extra_rw: t = extra_rw(r, t)
        sep = '\n' if prev is not None and r['idx'] == prev + 1 else '\n\n'
        parts.append((sep if parts else '') + t.rstrip() + '\n')
        prev = r['idx']
    return ''.join(parts), unresolved

FBASE_ABS = B.FBASE
BNODE_ABS = FBASE_ABS + '/🎭' + FE + 'doubles'
BPKG_ABS = BNODE_ABS + '/📦' + FE + 'packages/🦀' + FE + 'rust'
APKG_ABS = FBASE_ABS + '/📦' + FE + 'packages/🦀' + FE + 'rust'
THEME_EMOJI = {'snapshot': '🧮', 'retirement': '♻', 'store': '🗄', 'preparation': '🏭', 'operations': '🧵', 'callers': '🧾', 'envelopes': '📨'}
def theme_dir(key):
    folder = [t[1] for t in themes.THEMES if t[0] == key][0]
    return (FBASE_ABS if themes.THEME_CRATE[key] == 'A' else BNODE_ABS) + '/' + folder
def set_owners(its, F):
    pathrw.OWNER.clear()
    for r in its:
        if r['idx'] in F:
            th = themes.theme_of(r)
            cr = themes.THEME_CRATE[th]
            nm = r['name']
            if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'type', 'macro_rules') and nm: pathrw.OWNER[nm] = cr
    for m in ('demo', 'lossy', 'severity', 'timestamped', 'validated', 'prelude'): pathrw.OWNER[m] = 'A'
    pathrw.OWNER['store'] = 'B'
def gen_f(its, labels, F):
    set_owners(its, F)
    by_theme = collections.OrderedDict((k[0], []) for k in themes.THEMES)
    for r in its:
        if r['idx'] in F: by_theme[themes.theme_of(r)].append(r)
    problems = []
    exports = collections.defaultdict(list)
    modtexts = {}
    uses_by_mode = {m: unit_level_uses(its, m) for m in ('a', 'b')}
    for key, folder, mod, doc in themes.THEMES:
        cr = themes.THEME_CRATE[key]; mode = 'a' if cr == 'A' else 'b'
        rows = by_theme[key]
        body, un = join_items(its, rows, mode)
        mod_path = theme_dir(key) + '/' + RSF
        body = C.rebase_includes(body, C.UNIT_ABS, mod_path, L.R + '/' + L.K_REL, APKG_ABS if cr == 'A' else BPKG_ABS)
        problems += un
        for r in rows:
            if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'type') and r['name'] and not (key == 'store' and r['name'] == 'ArtifactStore'):
                exports[key].append(r['name'])
        head = '//! ' + THEME_EMOJI[key] + FE + ' ' + doc + '\n//!\n//! 🧭' + FE + ' Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.\n\n'
        if cr == 'A': head += 'use crate::prelude::*;\nuse crate::*;\n'
        else: head += 'use ' + FIX_CRATE + '::prelude::*;\nuse ' + FIX_CRATE + '::*;\nuse crate::*;\n'
        head += '\n'.join(fix_uses(mode)) + '\n'
        head += '\n'.join(uses_by_mode[mode]) + '\n'
        if cr == 'B' and key != 'store': head += 'use crate::store::ArtifactStore;\n'
        if key == 'retirement' and EXTRACTED.get('close'):
            rw = pathrw.Rewriter(mode, depth=1)
            ct = vis.widen(rw.rewrite(EXTRACTED['close']), 'fn')
            body = body.rstrip('\n') + '\n\n' + ct.strip() + '\n'
            exports[key].append('close')
        if key == 'snapshot' and RETAINED_DSL_IMPL:
            body = body.rstrip('\n') + '\n\n' + RETAINED_DSL_IMPL['text'].strip() + '\n'
        modtexts[key] = C.repair_regions(head + '\n' + body)
    return by_theme, modtexts, exports, problems

BIN_DOC = {
    'tests-': ('Companion test bin %s of the OS kernel Store: the root-mounted store unit tests packed by subject.', {'tests-01': '🏙', 'tests-02': '🌆', 'tests-03': '🌈', 'tests-04': '🌌', 'tests-05': '🌅', 'tests-06': '🌴', 'tests-07': '🌲', 'tests-08': '🍃', 'tests-09': '🍎', 'tests-10': '🍯'}),
}
BIN_EMOJI = {'tests-01': '🏙', 'tests-02': '🌆', 'tests-03': '🌈', 'tests-04': '🌌', 'tests-05': '🌅', 'tests-06': '🌴', 'tests-07': '🌲', 'tests-08': '🍃', 'tests-09': '🍎', 'tests-10': '🍯', 'sync-1': '🍰', 'sync-2': '🥧', 'member-open': '🍰', 'durable': '🥧'}
ALLOWS = '#![allow(unused_imports, dead_code, ambiguous_glob_reexports, async_fn_in_trait, clippy::all)]\n'
ALLOC = '#[global_allocator]\nstatic TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;\n'
OWNER_RX = re.compile(r'crate::os_store::\$owner\b')
RETAINED_DSL_IMPL = {}
EXTRACTED = {}
CLOSE_RX = re.compile(r'super::(?:super::)*raw_replay_retirement_tests::close\b')
def file_rewrite(f, depth, new_manifest, old_manifest):
    t = L.read(L.R + '/' + f)
    if f.endswith('🔁️replay/🛑️cancel/🧪️tests/' + RSF):
        from okrs import items as _items
        for it in _items(t):
            if it['kind'] == 'fn' and it['name'] == 'close':
                EXTRACTED['close'] = it['text']
                t = t[:it['start']] + t[it['end']:]
                break
        else: raise SystemExit('raw replay close() not found')
        t = 'use ' + FIXB_CRATE + '::close;\n' + t
    t = CLOSE_RX.sub(FIXB_CRATE + '::close', t)
    if f.endswith('snapshot-capability/🧪️tests/' + RSF):
        from okrs import items as _items
        for it in _items(t):
            if it['kind'] == 'impl' and it['text'].lstrip().startswith('impl ArtifactDsl for RetainedTextSnapshot'):
                RETAINED_DSL_IMPL['text'] = it['text']
                t = t[:it['start']] + t[it['end']:]
                break
        else: raise SystemExit('RetainedTextSnapshot ArtifactDsl impl not found')
    t = OWNER_RX.sub('semio_framework_os_io::control::$owner', t)
    rw = pathrw.Rewriter('bin', depth=depth)
    t2 = rw.rewrite(t)
    t2 = re.sub(r'use semio_framework_os_kernel as store;\n?', '', t2)
    t2 = C.rebase_includes(t2, L.R + '/' + f, L.R + '/' + f, old_manifest, new_manifest) if 'CARGO_MANIFEST_DIR' in t2 else t2
    return t2, rw.unresolved
SOURCE_LAW_FILES = ['⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs', '⚙️engine/📥️lifted/🏪️store/🦀️.rs', '🎮️one-item/📬️publication/🦀️.rs']
def law_patch(r, t):
    if r['name'] != 'one_item_publication_source_denies_generic_or_unbounded_shortcuts': return t
    old = 'let source = concat!(include_str!("../../🦀️.rs"), include_str!("../../🎮️one-item/📬️publication/🦀️.rs"));'
    assert t.count(old) == 1, 'source law line'
    files = ', '.join('"@@SRC@@' + f + '"' for f in SOURCE_LAW_FILES)
    new = 'let source = [' + files + '].iter().map(|file| std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file)).unwrap_or_else(|error| panic!("read {file}: {error}"))).collect::<String>();'
    return t.replace(old, new)
def chunk_text(its, labels, label, mode_depth=1):
    rows = [r for r in its if labels.get(r['idx']) == label and r['kind'] not in ('use', 'mod')]
    rows = [r for r in rows if not (r['kind'] is None and r['text'].lstrip().startswith('include!'))]
    return rows
def gen_bin(bin_key, plan, its, labels, unit_roots, F, ctx):
    node = C.node_abs(bin_key); pkg = C.pkg_abs(bin_key)
    b = plan['bins'][bin_key]
    mods = []   # (modname, abs path)
    problems = []
    rewritten = {}
    for uname in b['units']:
        if uname in ('catalog_include',): continue
        if uname == 'fixture_descriptors':
            for fam in FAMILIES:
                p = L.R + '/' + L.S_REL + '/🧪' + FE + 'tests/🧪' + FE + 'fixture-' + fam + '/' + RSF
                mods.append(('fixture_' + fam + '_tests', p))
            continue
        root, depth = unit_roots[uname]
        for f, dp in genbins.tree_with_depth(root, depth).items():
            t2, un = file_rewrite(f, dp, pkg, L.R + '/' + L.K_REL)
            problems += [(f, u) for u in un]
            rewritten[L.R + '/' + f] = t2
        mods.append((uname, L.R + '/' + root))
    chunk = None
    if b.get('chunk') is not None or bin_key == 'member-open':
        label = b['chunk'] if b.get('chunk') is not None else 'M'
        rows = chunk_text(its, labels, label)
        body, un = join_items(its, rows, 'bin', depth=1, extra_rw=law_patch)
        problems += [(bin_key + ':chunk', u) for u in un]
        body = re.sub(r'"@@SRC@@([^"]+)"', lambda m: '"' + os.path.relpath(C.S_ABS + '/' + m.group(1), pkg) + '"', body)
        chunk_path = node + '/🔬' + FE + 'unit/' + RSF
        head = '//! ' + '🔬' + FE + ' Unit tests of the Store that exercise the demo document through the public API of the Store crates.\n//!\n//! 🧭' + FE + ' Moved from the former store unit test module (chunk %s of the cut by subject).\n\nuse super::*;\n\n' % (label if label == 'M' else label + 1)
        text = head + body.rstrip('\n') + '\n'
        text = rebase_chunk_includes(text, chunk_path, pkg)
        if bin_key == 'member-open':
            cat_rel = L.S_REL + '/🧩' + FE + 'composition/🚪' + FE + 'open/🏭' + FE + 'operation/📦' + FE + 'catalog/' + RSF
            ct, cun = file_rewrite(cat_rel, 1, pkg, L.R + '/' + L.K_REL)
            rewritten[L.R + '/' + cat_rel] = ct
            problems += [(cat_rel, u) for u in cun]
            cat = L.R + '/' + L.S_REL + '/🧩' + FE + 'composition/🚪' + FE + 'open/🏭' + FE + 'operation/📦' + FE + 'catalog/' + RSF
            text = text.rstrip('\n') + '\n\ninclude!("' + os.path.relpath(cat, os.path.dirname(chunk_path)) + '");\n'
        hd, bd = text.split('use super::*;\n\n', 1)
        rewritten[chunk_path] = hd + 'use super::*;\n\n' + C.repair_regions(bd)
        chunk = chunk_path
        mods.insert(0, ('store_unit_tests', chunk_path))
    return mods, rewritten, problems, chunk
FAMILIES = ['demo', 'lossy', 'severity', 'timestamped', 'validated']
def rebase_chunk_includes(text, chunk_path, pkg):
    text = C.rebase_includes(text, UNIT_ABS_PATH, chunk_path, L.R + '/' + L.K_REL, pkg)
    return text
UNIT_ABS_PATH = C.UNIT_ABS
def root_text(bin_key, mods, uses_bin, plan):
    node = C.node_abs(bin_key)
    emoji = BIN_EMOJI[bin_key]
    crate = C.BINS[bin_key][0]
    desc = ROOT_DESC.get(bin_key, 'the root-mounted store unit tests packed by subject')
    head = '//! ' + emoji + FE + ' Companion test crate `' + crate + '`: ' + desc + '.\n//!\n//! 🧭' + FE + ' The tests use the public API of the Store crates only; the shared demo document, the mutation fixtures and the store prelude come from the fixtures crate.\n'
    head += '#![cfg(test)]\n' + ALLOWS + '\n' + ALLOC + '\n'
    head += 'use ' + FIX_CRATE + '::prelude::*;\nuse ' + FIX_CRATE + '::*;\nuse ' + FIXB_CRATE + '::*;\nuse ' + FIXB_CRATE + '::store::ArtifactStore;\n'
    head += '\n'.join(fix_uses('bin')) + '\n' + '\n'.join(uses_bin) + '\n\n'
    body = ''
    for name, p in mods:
        body += '#[path = "' + os.path.relpath(p, node) + '"]\nmod ' + name + ';\n'
    return head + body
ROOT_DESC = {
    'sync-1': 'the Store sync unit tests (actor, link, persistence and backbone parity laws)',
    'sync-2': 'the replay, supersession and backbone laws of the Store',
    'member-open': 'the member factory, dictionary, open and history tests of the Store plus the retained member publication laws',
    'durable': 'the durable owned-group journal, recovery and tool transaction laws of the Store',
}

RSF = RSF
def gen_fixture_descriptors():
    out = {}
    base = L.R + '/' + L.S_REL + '/🧪' + FE + 'tests'
    fams = collections.OrderedDict((f, []) for f in FAMILIES)
    for d in sorted(os.listdir(base)):
        m = re.match(r'^🧪' + FE + r'fixture-(demo|lossy|severity|timestamped|validated)-(.+)$', d)
        if not m: continue
        fam, leaf = m.group(1), m.group(2)
        old = L.read(base + '/' + d + '/' + RSF)
        mm = re.search(r'assert_fixture_descriptor::<(\w+)>\(include_str!\("([^"]+)"\)\)', old)
        fn = re.search(r'fn (\w+)\(', old).group(1)
        fams[fam].append((leaf, mm.group(1), mm.group(2), fn, base + '/' + d + '/' + RSF))
    for fam, rows in fams.items():
        text = '//! 🧪' + FE + ' Descriptor laws of the ' + fam + ' mutation fixtures: every leaf descriptor equals its committed JSON descriptor.\n//!\n//! 🧭' + FE + ' Consolidated from one test file per leaf; each leaf keeps its own module so the test names are unchanged.\n\n'
        for leaf, ty, inc, fn, oldp in rows:
            mod = leaf.replace('-', '_')
            text += 'mod ' + mod + ' {\n    use ' + FIX_CRATE + '::{assert_fixture_descriptor, ' + fam + '::' + ty + '};\n\n    #[test]\n    fn ' + fn + '() {\n        assert_fixture_descriptor::<' + ty + '>(include_str!("' + inc + '"));\n    }\n}\n\n'
        out[base + '/🧪' + FE + 'fixture-' + fam + '/' + RSF] = text.rstrip('\n') + '\n'
        for leaf, ty, inc, fn, oldp in rows: DEL.append(oldp)
    return out

def gen_f_root(modtexts, exports):
    head = '//! 🧪' + FE + ' Companion fixtures of the Store: the demo snapshot, its mutation families and the Store test prelude.\n//!\n//! 🧭' + FE + ' This crate sits below the kernel facade: the kernel and the companion test crates depend on it, it depends only on the Store crates. The test doubles that need the Store engine live in `semio-framework-os-store-testing-doubles`.\n'
    head += ALLOWS + '\n'
    for fam, emoji in (('demo', '🧮'), ('lossy', '🪤'), ('severity', '🚦'), ('timestamped', '⏱'), ('validated', '🛂')):
        head += '#[path = "🧬' + FE + 'mutations/' + emoji + FE + fam + '/🧬' + FE + 'mutations/' + RSF + '"]\npub mod ' + fam + ';\n'
    head += '\n#[path = "🧭' + FE + 'prelude/' + RSF + '"]\npub mod prelude;\n\n'
    for key, folder, mod, doc in themes.THEMES:
        if themes.THEME_CRATE[key] != 'A': continue
        head += '#[path = "' + folder + '/' + RSF + '"]\nmod ' + mod + ';\npub use ' + mod + '::*;\n'
    return head
def gen_b_root(exports):
    head = '//! 🎭' + FE + ' Test doubles of the Store that need the Store engine: the test store wrapper, the retirement and preparation doubles, the original-caller doubles and the envelope builders.\n//!\n//! 🧭' + FE + ' This crate sits above the fixtures crate and below the companion test crates; it depends only on the Store crates and the fixtures crate.\n'
    head += ALLOWS + '\n'
    for key, folder, mod, doc in themes.THEMES:
        if themes.THEME_CRATE[key] != 'B': continue
        vis_ = 'pub ' if key == 'store' else ''
        head += '#[path = "' + folder + '/' + RSF + '"]\n' + vis_ + 'mod ' + mod + ';\n'
        if key == 'store': head += 'pub use store::{' + ', '.join(sorted(exports[key])) + '};\n'
        else: head += 'pub use ' + mod + '::*;\n'
    return head
def gen_all():
    plan, src, its, names, labels, F = load()
    set_owners(its, F)
    roots = genbins.unit_roots()
    uses_bin = unit_level_uses(its, 'bin')
    bins = collections.OrderedDict()
    consumer = []
    allproblems = []
    for bk in C.BINS:
        mods, rew, pr, chunk = gen_bin(bk, plan, its, labels, roots, F, {})
        bins[bk] = (mods, rew)
        allproblems += pr
        consumer += list(rew.values())
    by, modtexts, exports, problems = gen_f(its, labels, F)
    allproblems += problems
    desc = gen_fixture_descriptors()
    # mutation component edits
    mut = collections.OrderedDict()
    for f in genf.mutation_files():
        t, un = genf.edit_mutation_file(f)
        mut[f] = t; allproblems += [(f, u) for u in un]
    consumer += list(modtexts.values()) + list(mut.values())
    return dict(plan=plan, its=its, labels=labels, F=F, modtexts=modtexts, exports=exports, bins=bins, desc=desc, mut=mut, consumer=consumer, uses_bin=uses_bin, problems=allproblems)

import genmanifest as GM
def tokens_of_all(texts):
    toks = set()
    for t in texts:
        toks |= {x[1] for x in tokenize(t) if x[0] == 'id'}
    return toks
from okrs import tokenize
def gen_prelude(D):
    toks = tokens_of_all(D['consumer'])
    lines, hubs, missing, skipped = PR.build(toks)
    grouped = PR.group_lines(lines)
    body = '\n'.join(grouped) + '\n\n' + '\n\n'.join(hubs) + '\n'
    head = '//! 🧭' + FE + ' The store test prelude: the names the former kernel store root offered to its test modules, re-exported from the crates that own them.\n//!\n//! 🧪' + FE + ' Test code imports this module with a glob; traits are included anonymously so their methods stay in scope.\n\n'
    return head + body, missing, skipped
def build_all():
    D = gen_all()
    prelude_text, missing, skipped = gen_prelude(D)
    D['prelude'] = prelude_text; D['prelude_missing'] = missing; D['prelude_skipped'] = skipped
    return D
if __name__ == '__main__':
    D = build_all()
    print('prelude', len(D['prelude'].split('\n')), len(D['prelude'].encode()), 'missing', len(D['prelude_missing']), D['prelude_skipped'])
    for m in D['prelude_missing']: print('  MISSING', m)

F_EXTRA = ['semio-framework-replication', 'semio-framework-dsl-record', 'semio-framework-value-derive', 'semio-framework-pack-json', 'semio-framework-diagnostic', 'semio-framework-os-dsl', 'semio-framework-ui-locale',
           'semio-framework-schema-state', 'semio-framework-os-command', 'semio-framework-os-store-one-item', 'semio-framework-dsl-record-derive', 'semio-framework-trace-memory-testing', 'serde', 'serde_json']
B_EXTRA = ['semio-framework-os-store-testing-fixtures', 'semio-framework-replication', 'semio-framework-value']
BIN_EXTRA = ['semio-framework-os-store-testing-fixtures', 'semio-framework-os-store-testing-doubles', 'semio-framework-trace-memory-testing', 'semio-framework-replication', 'semio-framework-value']
SYNC_NATIVE = {'semio-framework-os-store-sync-native', 'tokio-tungstenite', 'ureq', 'futures'}
SYNC_NOTP2 = {'semio-framework-os-store-sync'}
def b_header(a_header):
    h = a_header
    h = h.replace('name = "semio-framework-os-store-testing-fixtures"', 'name = "semio-framework-os-store-testing-doubles"')
    h = h.replace('id = "os-store-testing-fixtures"', 'id = "os-store-testing-doubles"')
    h = re.sub(r'description = "[^"]*"', 'description = "Test doubles of the Store that need the Store engine: the test store wrapper, retirement and preparation doubles, original-caller doubles and envelope builders"', h, 1)
    h = h.replace('workspace = "../../../../../../../.."', 'workspace = "../../../../../../../../.."')
    return h
def assemble(D):
    out = collections.OrderedDict(); dele = list(DEL)
    out[FBASE_ABS + '/' + RSF] = gen_f_root(D['modtexts'], D['exports'])
    out[FBASE_ABS + '/🧭' + FE + 'prelude/' + RSF] = D['prelude']
    out[BNODE_ABS + '/' + RSF] = gen_b_root(D['exports'])
    for key, folder, mod, doc in themes.THEMES:
        out[theme_dir(key) + '/' + RSF] = D['modtexts'][key]
    for f, t in D['mut'].items(): out[f] = t
    atexts = [t for k, t in out.items() if k.startswith(FBASE_ABS) and not k.startswith(BNODE_ABS)]
    btexts = [t for k, t in out.items() if k.startswith(BNODE_ABS)]
    man, fnames = GM.build(APKG_ABS + '/Cargo.toml', APKG_ABS, atexts, extra_names=F_EXTRA)
    man = man.replace('[lib]\npath = "' + RSF + '"', '[lib]\npath = "../../' + RSF + '"')
    man = re.sub(r'description = "[^"]*"', 'description = "Companion test fixtures of the Store: the demo snapshot, its mutation families and the Store test prelude"', man, 1)
    out[APKG_ABS + '/Cargo.toml'] = man
    dele.append(APKG_ABS + '/' + RSF)
    a_header = GM.scan_manifest_header(APKG_ABS + '/Cargo.toml')
    a_header = a_header.replace('[lib]\npath = "' + RSF + '"', '[lib]\npath = "../../' + RSF + '"')
    bman, bnames = GM.build(None, BPKG_ABS, btexts, extra_names=B_EXTRA, header=b_header(a_header))
    out[BPKG_ABS + '/Cargo.toml'] = bman
    out[BPKG_ABS + '/📋' + FE + 'project.json'] = L.read(APKG_ABS + '/📋' + FE + 'project.json').replace('os-store-testing-fixtures-rs', 'os-store-testing-doubles-rs')
    dele.append(UNIT_ABS_PATH)
    for bk, (mods, rew) in D['bins'].items():
        node = C.node_abs(bk); pkg = C.pkg_abs(bk)
        rtext = root_text(bk, mods, D['uses_bin'], D['plan'])
        out[node + '/' + RSF] = rtext
        for p, t in rew.items():
            if p in out: raise SystemExit('duplicate output ' + p)
            if os.path.exists(p) and L.read(p) == t: continue
            out[p] = t
        texts = [rtext] + list(rew.values())
        if bk == 'tests-05': texts += list(D['desc'].values())
        sync = bk.startswith('sync')
        extra_f = {'tokio': ['rt', 'time']} if sync else {}
        man, names = GM.build(pkg + '/Cargo.toml', pkg, texts, extra_names=BIN_EXTRA + (['tokio'] if sync else []), native_only=SYNC_NATIVE if sync else (), notp2_only=SYNC_NOTP2 if sync else (), extra_features=extra_f)
        out[pkg + '/Cargo.toml'] = man
    for p, t in D['desc'].items(): out[p] = t
    return out, dele
F_EXTRA_NAMES = None
MOVED_MODS = None
def moved_mod_names(roots):
    names = {n for n, (root, depth) in roots.items() if depth == 1} | {'tests', 'fixture_mutations'}
    names -= {'retained_codec_catalog', 'include'}
    return names
def kernel_edits(D):
    out = {}
    sroot = S_ABS_ROOT
    t = L.read(sroot)
    roots = genbins.unit_roots()
    names = moved_mod_names(roots)
    removed = []
    for nm in sorted(names):
        rx = re.compile(r'(?:#\[cfg\(test\)\]\n)?(?:#\[path\s*=\s*"[^"]*"\]\n)?mod ' + re.escape(nm) + r';\n')
        found = rx.findall(t)
        if len(found) != 1: raise SystemExit('S-root mount of %s found %d times' % (nm, len(found)))
        t = rx.sub('', t, count=1); removed.append(nm)
    cm = '// 🧪️ Test modules of the carved crates that need test doubles the kernel root still owns; they moved here from their host modules (PLAN-OS-KERNEL-CRATES.md, 9.2) until the companion test crates land.\n'
    t = t.replace(cm, '')
    hp = '#[cfg(test)]\npub(crate) use semio_framework_os_store_ledger::hot_path_census;\n'
    assert t.count(hp) == 1
    t = t.replace(hp, '')
    t = re.sub(r'\n{4,}', '\n\n\n', t)
    out[sroot] = t
    # retained codec catalog test
    ct = L.R + '/' + L.S_REL + '/📦' + FE + 'codec/🎮' + FE + 'operation/🧪' + FE + 'tests/' + RSF
    c = L.read(ct)
    old = 'use super::super::{tests::DemoSnapshot, fixture_mutations::demo::DemoMutation};'
    assert c.count(old) == 1
    c = c.replace(old, 'use ' + FIX_CRATE + '::{DemoSnapshot, demo::DemoMutation};')
    out[ct] = c
    # kernel manifest
    km = L.R + '/' + L.K_REL + '/Cargo.toml'
    m = L.read(km)
    prune = ['semio-framework-os-store-ledger', 'semio-framework-os-store-replay', 'semio-framework-job-worker', 'semio-framework-os-store-cursor', 'semio-framework-job', 'semio-framework-schema', 'blake3', 'semio-framework-pack', 'semio-framework-os-store-sync', 'semio-framework-os-store-sync-native', 'tokio-tungstenite']
    lines = m.split('\n'); res = []; section = None
    for ln in lines:
        if ln.startswith('['): section = ln.strip()
        if section in ('[dev-dependencies]', "[target.'cfg(not(target_arch = \"wasm32\"))'.dev-dependencies]"):
            mm = re.match(r'^([A-Za-z0-9_-]+)\s*=', ln)
            if mm and mm.group(1) in prune: continue
        res.append(ln)
    m2 = '\n'.join(res)
    m2 = m2.replace('[target.\'cfg(not(target_arch = "wasm32"))\'.dev-dependencies]\n\n', '')
    m2 = m2.replace('[dev-dependencies]\n', '[dev-dependencies]\nsemio-framework-os-store-testing-fixtures = { path = "../../🔨️modules/🏪️store/🧪️testing/📦️packages/🦀️rust" }\n', 1)
    out[km] = m2
    return out, removed
S_ABS_ROOT = L.R + '/' + L.S_REL + '/' + RSF

import pickle, widen as WD, audit as AU
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
import okcrates as OKC
RANK_NAMES = set('semio-framework-' + k for k in OKC.RANK)
def test_texts(out):
    return {p: t for p, t in out.items() if p.endswith('.rs') and ('/🧪' + FE + 'tests/' in p or '/🧪' + FE + 'testing/' in p or '/companion/' in p or 'bin-' in p)}
def widen_plan(out):
    import defscan as DS
    fields, methods = pickle.load(open(L.GEN + '/okwp8-defindex.pkl', 'rb'))
    texts = test_texts(out)
    localtypes = set()
    for t in texts.values(): localtypes |= AU.type_defs(t)
    alltok = set(); dots = collections.Counter(); assoc = collections.Counter(); lits = collections.Counter()
    for t in texts.values():
        a, b, c = DS.usage(t); dots.update(a); assoc.update(b); lits.update(c)
        alltok |= {x[1] for x in tokenize(t) if x[0] == 'id'}
    want = set()   # (file, owner, member, kind)
    for name, cnt in dots.items():
        for d in fields.get(name, []):
            if d[2] != 'pub' and d[0] in RANK_NAMES and d[1] in alltok and d[1] not in localtypes: want.add((d[3], d[1], name, 'field'))
        for d in methods.get(name, []):
            if d[2] != 'pub' and d[0] in RANK_NAMES and d[1] in alltok and d[1] not in localtypes: want.add((d[3], d[1], name, 'method'))
    for (ty, nm), cnt in assoc.items():
        for d in methods.get(nm, []):
            if d[1] == ty and d[2] != 'pub' and d[0] in RANK_NAMES and ty not in localtypes: want.add((d[3], ty, nm, 'method'))
    for (ty, fl), cnt in lits.items():
        for d in fields.get(fl, []):
            if d[1] == ty and d[2] != 'pub' and d[0] in RANK_NAMES and ty not in localtypes: want.add((d[3], ty, fl, 'field'))
    return sorted(want)
def widen_apply(want):
    byfile = collections.defaultdict(list)
    for f, ow, mem, kind in want: byfile[f].append((ow, mem, kind))
    out = {}; report = []
    for f, rows in byfile.items():
        text = L.read(f); edits = []
        for ow, mem, kind in rows:
            e = WD.widen_field(text, ow, mem) if kind == 'field' else WD.widen_method(text, ow, mem)
            if e is None: continue
            if not e: report.append(('NOEDIT', L.rel(f)[-60:], ow, mem, kind)); continue
            edits += e
        if edits:
            # dedupe overlapping identical edits
            seen = set(); ded = []
            for e in edits:
                if e[:2] in seen: continue
                seen.add(e[:2]); ded.append(e)
            out[f] = WD.apply_edits(text, ded)
    return out, report
