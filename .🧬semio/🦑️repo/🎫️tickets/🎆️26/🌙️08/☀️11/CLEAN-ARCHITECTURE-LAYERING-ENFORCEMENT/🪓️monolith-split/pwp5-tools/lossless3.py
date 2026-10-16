"""♻️ Losslessness of the WP3 lifts: the multiset of non-blank lines of every Rust file under the WP3 directories and the lift destination directories, snapshot (before the first edit) against now. Prints what was lost and what is new. usage: python3 lossless3.py [-v]"""
import collections, os, sys
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
SNAP = SCR + '/pwp5-snap'
P = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
DIRS = ['🪆️child', '🧩️composition', '🗃️document', '🎞️media', '👷️job', '🎮️command', '👥️peer', '🧵️retained-command', '🧩️extension', '💡️inference', '🛂️describe', '📜️declaration/📨️emission/🛬️lifted', '🏗️builder/🔌️program/🛬️lifted', '⏪️time-travel/🪟️panel/🛬️lifted', '📜️declaration/🛬️lifted', '📜️declaration/🧩️app/🛬️lifted', '📜️declaration/🪆️child/🛬️lifted']
def lines(base):
    c = collections.Counter(); n = 0
    for d in DIRS:
        root = os.path.join(base, d)
        if not os.path.isdir(root): continue
        for dp, dn, fn in os.walk(root):
            if '📦️packages' in dp: continue
            for f in fn:
                if f.endswith('.rs'):
                    n += 1
                    for l in open(os.path.join(dp, f), encoding='utf8').read().split('\n'):
                        if l.strip(): c[l.rstrip()] += 1
    return c, n
B, nb = lines(SNAP); A, na = lines(P)
lost = B - A; new = A - B
print('files before', nb, 'after', na, '| non-blank lines before', sum(B.values()), 'after', sum(A.values()), '| lost', sum(lost.values()), 'new', sum(new.values()))
if '-v' in sys.argv:
    print('--- lost'); [print(v, k[:230]) for k, v in sorted(lost.items(), key=lambda x: -x[1])[:80]]
    print('--- new'); [print(v, k[:230]) for k, v in sorted(new.items(), key=lambda x: -x[1])[:80]]
