# 📌 Source-text pins: for every TS/JSON file that names the store root, find the string literals it expects in Rust text and report those that existed in the pre-A2 root but are gone from the kernel root now, with the carved file that holds them. usage: pins.py
import os, re, subprocess, sys, json
OKD = os.path.dirname(os.path.abspath(__file__))
R = '/Users/ueli/Documents/semio'
S = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store'
pre = None
for line in open(OKD + '/pre-a2/INDEX.txt', encoding='utf8'):
    h, rel = line.rstrip('\n').split('\t')
    if rel.endswith('🏪️store/🦀️.rs'): pre = open(OKD + '/pre-a2/' + h, encoding='utf8').read()
cur = open(S + '/🦀️.rs', encoding='utf8').read()
# index of carved text
carved = {}
for dp, dn, fn in os.walk(S):
    if '/.' in dp: continue
    for f in fn:
        if f == '🦀️.rs' and ('📥️lifted' in dp or any(x in dp for x in ('⚙️engine', '🏗️initialization', '🧰️test-support', '🧩️composition', '🧾️document', '🎚️config', '📜️space-history/🏠️space', '📦️codec/🏭️factory'))):
            p = os.path.join(dp, f); carved[p] = open(p, encoding='utf8').read()
files = subprocess.run(['git', '-c', 'core.quotepath=false', 'grep', '-l', '-F', '🏪️store/🦀️.rs', '--', '🧰️framework', '✏️s'], cwd=R, capture_output=True, text=True).stdout.split('\n')
files = [f for f in files if f.endswith(('.ts', '.json')) and f]
LIT = re.compile(r'(?:toContain|includes|indexOf|toMatch|not\.toContain)\(\s*(["`])((?:\\.|(?!\1).){10,}?)\1')
for f in files:
    t = open(R + '/' + f, encoding='utf8').read()
    miss = []
    for m in LIT.finditer(t):
        needle = bytes(m.group(2), 'utf8').decode('unicode_escape', errors='ignore') if '\\' in m.group(2) else m.group(2)
        if needle in pre and needle not in cur:
            where = [os.path.relpath(p, S)[-50:] for p, x in carved.items() if needle in x][:2]
            miss.append((needle[:50], where))
    if miss: print(f, len(miss), miss[:3])
