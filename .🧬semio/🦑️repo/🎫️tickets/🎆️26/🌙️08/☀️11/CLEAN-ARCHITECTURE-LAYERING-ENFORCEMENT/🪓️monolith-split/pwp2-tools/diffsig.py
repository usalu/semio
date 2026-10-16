#!/usr/bin/env python3
# 🔎️ Signal rows of a WP2 differential: error multiset of A minus B (and B minus A) without external-crate / extern-alias noise; usage: diffsig.py <tag> <config> [--noise]
import json, re, sys, collections, importlib.util
ARGS = list(sys.argv[1:])
from common import *
spec = importlib.util.spec_from_file_location('pwp0oracle', SPLIT + '/pwp0-tools/oracle.py')
O = importlib.util.module_from_spec(spec)
sys.path.insert(0, SPLIT + '/pwp0-tools')
spec.loader.exec_module(O)

NOISE = re.compile(r"in (?:module )?`(?:protocol|store|dsl|vcs|io)`|unresolved module or unlinked crate|cannot find (?:attribute|derive macro|macro)|unresolved imports? `(?:semio_framework|serde|store|protocol|dsl|vcs|pack|replication|wit_bindgen|tokio|base64)|could not find `[a-zA-Z_0-9]+` in `(?:store|protocol|dsl|vcs)`|no `[A-Za-z_0-9]+` in `[a-z:_0-9]*(?:store|protocol|dsl|vcs)`|^cannot find (?:type|trait|function|value|struct, variant or union type|function, tuple struct or tuple variant) `[A-Za-z_0-9]+` in (?:this scope|module `(?:store|protocol|dsl|vcs)`)$")
EXTERNAL = re.compile(r'`(?:semio_framework|semio_framework_[a-z_0-9]+|serde|serde_json|pack|replication|wit_bindgen)`')

def load(path, sb):
    c = collections.Counter()
    where = {}
    for line in open(path):
        line = line.strip()
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        if d.get('level') != 'error' or str(d.get('message', '')).startswith('aborting due to'):
            continue
        code = (d.get('code') or {}).get('code') or ''
        sp = next((s for s in d.get('spans', []) if s.get('is_primary')), None)
        label = text = file = ''
        line_no = 0
        if sp:
            label = sp.get('label') or ''
            file = sp['file_name'].split('/🔌️plugin/')[-1]
            line_no = sp.get('line_start')
            t = sp.get('text') or []
            if t:
                l = t[0]
                text = (l['text'][l['highlight_start'] - 1:l['highlight_end'] - 1] if len(t) == 1 else l['text'][l['highlight_start'] - 1:]).strip()[:80]
        msg = d['message']
        key = (code, msg, text, label)
        c[key] += 1
        where.setdefault(key, []).append((file, line_no))
    return c, where

def main():
    tag, cfg = ARGS[0], ARGS[1]
    a, wa = load(f'{WS}/pwp0-oracle-w2{tag}-A-{cfg}.jsonl', None)
    b, wb = load(f'{WS}/pwp0-oracle-w2{tag}-B-{cfg}.jsonl', None)
    new, old = a - b, b - a
    def sig(k):
        code, msg, text, label = k
        if '--noise' in ARGS:
            return True
        return not (NOISE.search(msg) or NOISE.search(label) or EXTERNAL.search(msg) and code in ('E0432', 'E0433'))
    ns = {k: v for k, v in new.items() if sig(k)}
    os_ = {k: v for k, v in old.items() if sig(k)}
    print(f'A {sum(a.values())} B {sum(b.values())}  only-A {sum(new.values())} ({len(new)} rows)  only-B {sum(old.values())} ({len(old)} rows)  signal only-A {sum(ns.values())} ({len(ns)} rows)  signal only-B {sum(os_.values())} ({len(os_)} rows)')
    for k, v in sorted(ns.items(), key=lambda kv: -kv[1])[:80]:
        print('  +A', v, k[0], k[1][:130], '|', k[2][:40], '|', [f'{f[-60:]}:{n}' for f, n in wa[k][:2]])
    for k, v in sorted(os_.items(), key=lambda kv: -kv[1])[:40]:
        print('  +B', v, k[0], k[1][:130], '|', k[2][:40], '|', [f'{f[-60:]}:{n}' for f, n in wb[k][:2]])

if __name__ == '__main__':
    main()
