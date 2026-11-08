import os, shutil
ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any'
SRC = f'{ART}/🪨️en1996/{S}'
DST = f'{ART}/📇️iso16757/{S}'
W = '✏️editor/🎭️modes/✏️edit/🪟️windows'
SWAPS = [('EN 1996', 'ISO 16757'), ('En1996', 'Iso16757'), ('en1996', 'iso16757'), ('EN1996', 'ISO16757'), ('"members"', '"catalogue"')]

def swap(t):
    for a, b in SWAPS:
        t = t.replace(a, b)
    return t

for old in ('📥️inputs', '📊️results'):
    shutil.rmtree(f'{DST}/{W}/{old}', ignore_errors=True)
for w in ('🧊️viewport', '📈️diagrams', '🧾️properties', '✅️checks', '🌲️outliner'):
    for root, _, files in os.walk(f'{SRC}/{W}/{w}'):
        for n in files:
            p = os.path.join(root, n)
            t = os.path.join(DST, os.path.relpath(p, SRC))
            os.makedirs(os.path.dirname(t), exist_ok=True)
            open(t, 'w', encoding='utf8').write(swap(open(p, encoding='utf8').read()))
for rel in ('✏️editor/🎭️modes/✏️edit/🦀️.rs', '✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs'):
    os.makedirs(os.path.dirname(f'{DST}/{rel}'), exist_ok=True)
    open(f'{DST}/{rel}', 'w', encoding='utf8').write(swap(open(f'{SRC}/{rel}', encoding='utf8').read()))
