# ✏️ Asserting text edits for the OS kernel carve: every replacement states its expected count; every touched repo file is logged.
import os, re
R = '/Users/ueli/Documents/semio'
LOG = os.path.dirname(os.path.abspath(__file__)) + '/touched.txt'

def touch(path):
    rel = os.path.relpath(path, R)
    seen = set(open(LOG, encoding='utf-8').read().split('\n')) if os.path.exists(LOG) else set()
    if rel not in seen:
        with open(LOG, 'a', encoding='utf-8') as f: f.write(rel + '\n')

def read(path): return open(path, encoding='utf-8').read()

def write(path, text):
    touch(path)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, 'w', encoding='utf-8').write(text)

def edit(path, pairs, regex=False):
    t = read(path)
    for old, new, count in pairs:
        if regex:
            t, n = re.subn(old, new, t)
            assert n == count, (path, old, n, count)
        else:
            assert t.count(old) == count, (path, old[:80], t.count(old), count)
            t = t.replace(old, new)
    write(path, t)
