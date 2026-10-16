# 🕳️ Lost-item audit of the sync tree: top-level items of the pristine sync files against every Rust file under the live sync tree (all three crates).
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools')
import importlib.util
spec = importlib.util.spec_from_file_location('lost2', HERE + '/../oksh-tools/lost2.py')
src = open(HERE + '/../oksh-tools/lost2.py', encoding='utf8').read().split('files = json.load')[0]
ns = {}
exec(compile(src, 'lost2-head', 'exec'), ns)
keys = ns['keys']
R = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
SYNC = '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync'
before = collections.Counter(); where = collections.defaultdict(list)
for dp, dn, fn in os.walk(os.path.join(SCR, 'orig', SYNC)):
    for f in fn:
        if f.endswith('.rs'):
            for k in keys(open(os.path.join(dp, f), encoding='utf8').read()): before[k] += 1; where[k].append(f)
after = collections.Counter()
for dp, dn, fn in os.walk(os.path.join(R, SYNC)):
    for f in fn:
        if f.endswith('.rs'):
            for k in keys(open(os.path.join(dp, f), encoding='utf8').read()): after[k] += 1
lost = before - after; gained = after - before
print('orig items', sum(before.values()), 'live items', sum(after.values()), 'lost', sum(lost.values()), 'gained', sum(gained.values()))
for k, n in sorted(lost.items(), key=lambda kv: str(kv[0])): print('  LOST', n, k[0], k[1][:120])
for k, n in sorted(gained.items(), key=lambda kv: str(kv[0])): print('  new ', n, k[0], k[1][:120])
