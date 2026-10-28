"""Heuristic: a test file is hand-routed when an ancestor's script/project text names its directory relative to that ancestor (or the workspace-rooted path). usage: routed-by-script.py <list-file>"""
import os, sys, re
R = '/Users/ueli/Documents/semio/'
def texts(d):
    for n in ('📜️script.ts', '📋️project.json'):
        f = os.path.join(R, d, n)
        if os.path.isfile(f) and os.path.getsize(f) < 3_000_000:
            yield n, open(f, encoding='utf-8', errors='replace').read()
def routed(path):
    parts = path.split('/')
    dirs = parts[:-1]
    cur = list(dirs)
    while len(cur) > 2:
        cur.pop()
        a = '/'.join(cur)
        rel = dirs[len(cur):]
        for n, t in texts(a):
            full = '/'.join(rel)
            spaced = '", "'.join(rel)
            wr = '{workspaceRoot}/' + '/'.join(dirs)
            if full in t or spaced in t or '/'.join(dirs) in t:
                if rel == ['🧪️tests'] and not ('🧪️tests/🟦️.ts' in t or '"🧪️tests", "🟦️.ts"' in t or '🧪️tests/🪆' in t):
                    continue
                return a.split('/')[-1] + '/' + n
    return None
if __name__ == '__main__':
    for p in open(sys.argv[1], encoding='utf-8').read().split('\n'):
        if p:
            h = routed(p)
            print(('R ' if h else 'U ') + '/'.join(p.split('/')[-5:-1]) + ('  <- ' + h if h else ''))
