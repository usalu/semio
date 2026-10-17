"""🔎 c4p20 (full non-stub list, no 400-row cap): compares two typeck-oracle runs of the same family (the `-new-<cfg>.txt` files of pos.py: non-stub rows of the current tree) as multisets keyed by error code, crate head and message with file path and line numbers stripped, so rows that moved with their code into a child file still match. usage: posdiff.py <pre-label> <post-label> [cfg...]"""
import collections, os, re, sys
G = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '🗑️generated')
pre, post = sys.argv[1], sys.argv[2]
cfgs = sys.argv[3:] or ['lib', 'test']
HDR = re.compile(r'^#\d+ (E\d+|\S+) (\S+?)/')


def load(label, cfg):
    p = os.path.join(G, 'c4p20-c4p20-%s-cur-%s-it1-nonstub.txt' % (label, cfg))
    keys = collections.Counter()
    cur = None
    for line in open(p, encoding='utf-8'):
        line = line.rstrip('\n')
        m = HDR.match(line)
        if m:
            cur = (m.group(1), m.group(2))
            continue
        if cur and line.startswith(('error', 'warning')):
            keys[cur + (re.sub(r'\s+', ' ', line),)] += 1
            cur = None
    return keys


rc = 0
for cfg in cfgs:
    a, b = load(pre, cfg), load(post, cfg)
    new = b - a
    gone = a - b
    print('cfg %s: pre rows %d post rows %d new %d gone %d' % (cfg, sum(a.values()), sum(b.values()), sum(new.values()), sum(gone.values())))
    for k, v in sorted(new.items()):
        print('  NEW  %dx %s' % (v, ' | '.join(k)[:260]))
        rc = 1
    for k, v in sorted(gone.items())[:20]:
        print('  gone %dx %s' % (v, ' | '.join(k)[:260]))
sys.exit(rc)
