#!/usr/bin/env python3
"""🔎️ External API surface of semio-framework-plugin-host: every path the repo's Rust code names below the crate root."""
import os, sys, re, subprocess, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod
from rsitems import lex

REPO = '/Users/ueli/Documents/semio'
HOSTDIR = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/'
CRATE = 'semio_framework_plugin_host'


def files():
    out = subprocess.run(['git', '-c', 'core.fsmonitor=false', '-c', 'core.quotepath=false', 'grep', '-l', '-F', CRATE, '--', ':(glob)**/*.rs', ':!.🧬semio'], cwd=REPO, capture_output=True, text=True).stdout.splitlines()
    return [f for f in out if not f.startswith(HOSTDIR)]


def paths_in(src):
    toks = [hcmod.Tok(t) for t in lex(src)]
    n = len(toks)
    found = []
    alias = {CRATE}
    i = 0
    # use statements
    while i < n:
        t = toks[i]
        if t.k == 'id' and t.x == 'use' and (i == 0 or toks[i - 1].x in (';', '}', '{', ']', ')') or True):
            try:
                leaves, j = hcmod.parse_use(toks, i, n)
            except Exception:
                i += 1
                continue
            for segs, binding, glob in leaves:
                if segs and segs[0] == CRATE:
                    if len(segs) == 1 and binding != CRATE:
                        alias.add(binding)
                    found.append(('use', segs[1:], glob))
            i = max(j, i + 1)
            continue
        i += 1
    # qualified paths
    i = 0
    while i < n:
        t = toks[i]
        if t.k == 'id' and t.x in alias and not (i > 0 and toks[i - 1].x == ':' and toks[i - 2].x == ':') and i + 2 < n and toks[i + 1].x == ':' and toks[i + 2].x == ':':
            segs = []
            j = i
            while j + 2 < n and toks[j].k == 'id':
                segs.append(toks[j].x)
                if toks[j + 1].x == ':' and toks[j + 2].x == ':':
                    j += 3
                else:
                    break
            found.append(('path', segs[1:], False))
            i = j
            continue
        i += 1
    return found


if __name__ == '__main__':
    heads = collections.Counter()
    full = collections.Counter()
    per = collections.defaultdict(set)
    for f in files():
        try:
            src = open(os.path.join(REPO, f), encoding='utf-8').read()
        except Exception:
            continue
        for kind, segs, glob in paths_in(src):
            if not segs:
                continue
            heads[segs[0]] += 1
            full['::'.join(segs) + ('::*' if glob else '')] += 1
            per[segs[0]].add(f)
    json.dump({'heads': heads, 'full': full, 'files': {k: sorted(v) for k, v in per.items()}}, open(sys.argv[1] if len(sys.argv) > 1 else '/tmp/phc-consumers.json', 'w'), ensure_ascii=False, indent=1)
    for k, v in sorted(heads.items(), key=lambda x: -x[1]):
        print(v, k, len(per[k]))
