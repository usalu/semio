import sys
sys.path.insert(0,'/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/okwp23-tools')
# 🕳️ Lost-item audit: multiset of top-level items (impl headers with module qualifiers stripped, fns, structs, enums, traits, types, consts, statics, macro_rules, macro-call items) of the pristine store closure against the live store tree. An item that exists before and not after was cut and never re-placed.
# usage: lostitems.py [all]
import os, re, sys, json, collections
HERE = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/okwp23-tools'
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE)
import unionlib as U
from okrs import items, tokenize
R = U.R; SCR = U.SCR
STORE = '🧰️framework/🛍️products/💻️os/🔨️modules'
def norm(s): return re.sub(r'\s+', ' ', re.sub(r'(?:\b\w+::)+', '', s)).strip()
def keys(text, depth=0):
    out = []
    for it in items(text):
        k = it['kind']; t = it['text']
        code = re.sub(r'(?m)^\s*//.*$', '', t)
        if k == 'impl':
            hd = code[:code.index('{')] if '{' in code else code
            hd = re.sub(r'(?m)^\s*#\[.*$', '', hd)
            out.append(('impl', norm(hd)))
        elif k in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'union') and it['name']:
            out.append((k, it['name']))
        elif k == 'mod' and it['name'] and '{' in code.split('mod ' + it['name'], 1)[-1][:40]:
            body = code[code.index('{', code.index('mod ' + it['name'])) + 1: code.rindex('}')]
            out += keys(body, depth + 1)
        elif k is None:
            m = re.search(r'macro_rules!\s*(\w+)', code)
            if m: out.append(('macro', m.group(1)))
            else:
                m = re.match(r'\s*(?:#\[[^\]]*\]\s*)*([A-Za-z_][\w:]*)!\s*[\(\{\[]', code)
                if m: out.append(('call', m.group(1) + ':' + norm(re.sub(r'\s+', '', code))[:60]))
    return out
files = json.load(open(SCR + '/orig-paths.json'))
before = collections.Counter(); where = collections.defaultdict(list)
for f in files:
    if not (f.startswith(STORE + '/🏪️store') or f.startswith(STORE + '/🚪️io')) or not f.endswith('.rs'): continue
    op = os.path.join(SCR, 'orig', f)
    if not os.path.exists(op): continue
    for key in keys(open(op, encoding='utf8').read()):
        before[key] += 1; where[key].append(f)
after = collections.Counter()
for sub in ('🏪️store', '🚪️io'):
  for dp, dn, fn in os.walk(os.path.join(R, STORE, sub)):
    for f in fn:
        if f.endswith('.rs'):
            for key in keys(open(os.path.join(dp, f), encoding='utf8').read()): after[key] += 1
lost = before - after
print('orig items', sum(before.values()), 'live items', sum(after.values()), 'lost', sum(lost.values()))
for key, n in sorted(lost.items(), key=lambda kv: str(kv[0])):
    print(' ', n, key[0], key[1][:110], '|', where[key][0][-50:])
