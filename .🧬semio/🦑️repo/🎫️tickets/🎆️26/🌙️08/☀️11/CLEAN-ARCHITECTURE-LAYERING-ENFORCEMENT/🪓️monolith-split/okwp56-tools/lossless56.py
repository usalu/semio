# 🕳️ Lost-item audit of the carve: the multiset of top-level items (impl headers with module qualifiers stripped, fns, structs, enums, traits, types, consts, statics, macros) of the pre-carve kernel world against the post-carve union of the kernel world and every crate of the batch. usage: lossless56.py <dump.json>
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
from okrs import items
import okcrates as C, ns
R = C.R
def norm(s): return re.sub(r'\s+', ' ', re.sub(r'(?:\b\w+::)+', '', s)).strip()
KIND = ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'union', 'macro_rules', 'impl')
def keys(text, depth=0):
    out = []
    for it in items(text):
        k = it['kind']
        if k in KIND and it['name'] is not None:
            nm = norm(it['name'])
            if k == 'impl': nm = nm.split(' where ')[0]
            out.append((k, nm))
        elif k == 'mod' and it['text'].count('{') and depth < 3:
            body = it['text'][it['text'].index('{') + 1: it['text'].rindex('}')]
            out += keys(body, depth + 1)
    return out
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')
def mounted(libroot):
    seen = []; stack = [libroot]
    while stack:
        f = stack.pop()
        if f in seen or not os.path.exists(f): continue
        seen.append(f)
        t = open(f, encoding='utf8').read()
        for m in PATH.finditer(t):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if p.endswith('.rs'): stack.append(p)
    return seen
def main():
    d = json.load(open(sys.argv[1]))
    pre = collections.Counter()
    for f, t in d['base_text'].items():
        for k in keys(t): pre[k] += 1
    model = C.build()[1]
    files = set(ns.world().files)
    post = collections.Counter()
    seen = set()
    for f in ns.world().files:
        for k in keys(open(f, encoding='utf8').read()): post[k] += 1
        seen.add(f)
    crate_files = set()
    for k in d['batch']:
        for f in mounted(C.node_abs(k, model) + '/🦀️.rs'):
            if f in seen or f in crate_files: continue
            crate_files.add(f)
            for key in keys(open(f, encoding='utf8').read()): post[key] += 1
    lost = pre - post; extra = post - pre
    print('pre items', sum(pre.values()), 'post items', sum(post.values()), '| lost', sum(lost.values()), '| extra', sum(extra.values()))
    for k, v in sorted(lost.items())[:70]: print('  LOST', v, k)
    for k, v in sorted(extra.items())[:70]: print('  EXTRA', v, k)
main()
