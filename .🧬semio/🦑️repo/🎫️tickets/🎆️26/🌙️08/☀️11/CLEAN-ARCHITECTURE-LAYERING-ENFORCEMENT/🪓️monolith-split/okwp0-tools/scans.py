# 🔬 Scans the differential cannot do: (1) orphan/placement of impls in the carved crates, (2) non-public members of the carved crates named by consumers outside them.
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from okrs import items, tokenize
import okcrates as C, okreal
from wp1tree import tree
R = C.R; M = C.M + '/'
SCR = '/private/tmp/claude-501/-Users-ueli/Documents-semio'
ROOT_FILES = {'os-list-delta': ['🪡️list-delta/🦀️.rs'], 'os-semio': ['🧬️semio/🦀️.rs'], 'os-space-sqlite-cells': ['🪐️space/🪶️sqlite/🦀️.rs'], 'os-plugin-registry-schema': ['🔌️plugin/📇️registry/📦️deployment/🧬️schema/🦀️.rs'],
              'os-extension': ['🧩️extension/🦀️.rs'], 'os-pack-tools': ['🎒️pack/⌨️cli/🦀️.rs'], 'os-spr': ['📡️spr/🦀️.rs'], 'os-spr-cli': ['📡️spr/⌨️cli/🦀️.rs'], 'os-dsl': ['🗣️dsl/🦀️.rs']}
crate_files = {k: [f for f in tree([M + r for r in rs]) if f.startswith(M) and '/🧪️test' not in f and 'tests/' not in f] for k, rs in ROOT_FILES.items()}
all_crate_files = {f for fs in crate_files.values() for f in fs}
STD = {'Self', 'String', 'Vec', 'Option', 'Result', 'Box', 'Arc', 'Rc', 'str', 'bool', 'u8', 'u16', 'u32', 'u64', 'usize', 'i64', 'f64', 'HashMap', 'BTreeMap', 'Mutex', 'RwLock', 'Cursor', 'PhantomData'}
print('== orphan / placement of impls')
susp = 0
for k, files in crate_files.items():
    local = set()
    for f in files:
        for it in items(open(f, encoding='utf8').read()):
            if it['kind'] in ('struct', 'enum', 'trait', 'type', 'union') and it['name']: local.add(it['name'])
    for f in files:
        for it in items(open(f, encoding='utf8').read()):
            if it['kind'] != 'impl': continue
            head = it['name'] or ''
            m = re.match(r'(?:<[^>]*>\s*)?(.*?)\s+for\s+(.+)$', head)
            trait, ty = (m.group(1), m.group(2)) if m else (None, head)
            ids = lambda s: set(re.findall(r'[A-Z][A-Za-z0-9_]*', s))
            tl = ids(trait or '') & local; yl = ids(ty) & local
            if trait is None and not (ids(ty) & local):
                susp += 1; print('  INHERENT on foreign type?', k, os.path.relpath(f, M)[:50], head[:70])
            elif trait is not None and not tl and not yl:
                susp += 1; print('  TRAIT impl with no local trait/type', k, os.path.relpath(f, M)[:50], head[:80])
print('suspicious impls', susp)
print('== non-public members named by consumers')
# private fields/methods of carved crates
priv = collections.defaultdict(set)
for k, files in crate_files.items():
    for f in files:
        src = open(f, encoding='utf8').read()
        for it in items(src):
            if it['kind'] in ('struct',) and it['vis'] == 'pub':
                body = it['text']
                for m in re.finditer(r'\n\s+(?!pub\b)([a-z_][a-z0-9_]*)\s*:', body): priv[k].add(m.group(1))
            if it['kind'] == 'impl':
                for m in re.finditer(r'\n\s+(?:pub\(crate\)\s+|pub\(super\)\s+)?(?:async\s+)?fn\s+([a-z_][a-z0-9_]*)', it['text']):
                    line = it['text'][max(0, m.start()):m.end()]
                    if 'pub ' not in line.split('fn')[0]: priv[k].add(m.group(1))
orig = json.load(open(C.GEN + '/../🗑️generated/okwp0-orig-closure.json')) if False else None
cons = []
kroot = C.K + '/🦀️.rs'
live_files = set()
import subprocess
paths = json.load(open('/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/orig-paths.json'))
for rel in paths:
    p = R + '/' + rel
    if p.endswith('.rs') and p not in all_crate_files and '/🗣️dsl/' not in p and '/📡️spr/' not in p and os.path.exists(p): cons.append(p)
defined = set()
for p in cons:
    t = open(p, encoding='utf8').read()
    defined |= set(re.findall(r'\bfn\s+([a-z_][a-z0-9_]*)', t)) | set(re.findall(r'\n\s+(?:pub(?:\([a-z]+\))?\s+)?([a-z_][a-z0-9_]*)\s*:', t))
for k in priv: priv[k] = {n for n in priv[k] if n not in defined}
hits = collections.Counter()
common = {'new', 'len', 'get', 'next', 'default', 'from', 'into', 'fmt', 'eq', 'hash', 'clone', 'drop', 'is_empty', 'iter', 'push', 'insert', 'remove', 'run', 'step', 'open', 'read', 'write', 'main', 'parse', 'text', 'id', 'name', 'kind', 'error', 'value', 'ok', 'finish', 'close', 'build'}
for p in cons:
    toks = [t for t in tokenize(open(p, encoding='utf8').read()) if t[0] not in ('ws', 'comment')]
    ids = collections.Counter()
    for i, t in enumerate(toks):
        if t[0] == 'id' and i and toks[i - 1][1] == '.': ids[t[1]] += 1
    for k, names in priv.items():
        for n in names - common:
            if ids[n]: hits[(k, n, os.path.relpath(p, M)[:60])] += ids[n]
print('candidate member names (field/method access `.name` in consumers):', len(hits))
bykey = collections.defaultdict(list)
for (k, n, f), v in hits.items(): bykey[(k, n)].append(f)
for (k, n), fs in sorted(bykey.items())[:40]: print('  ', k, n, len(fs), fs[0])
