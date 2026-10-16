"""📝️ Rewrites the item list and counts in the docstring of every lift file the WP3 lifts filled, from the items the file really holds."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import cutplan as cp
from rsitems import split_items
P = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
FILES = ['📜️declaration/📨️emission/🛬️lifted/📦️preparation/🦀️.rs', '📜️declaration/🛬️lifted/💡️inference/🦀️.rs', '🏗️builder/🔌️program/🛬️lifted/💡️inference/🦀️.rs', '🏗️builder/🔌️program/🛬️lifted/🚪️gateway/🦀️.rs', '⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs', '📜️declaration/🧩️app/🛬️lifted/🧬️context/🦀️.rs', '📜️declaration/🧩️app/🛬️lifted/📸️fields/🦀️.rs', '📜️declaration/🧩️app/🛬️lifted/🎞️media-turn/🦀️.rs', '📜️declaration/🧩️app/🛬️lifted/🛫️intrinsic/🦀️.rs']
for rel in FILES:
    p = P + rel
    text = open(p, encoding='utf8').read()
    lines = text.split('\n')
    k = 0
    while k < len(lines) and lines[k].startswith('//!'): k += 1
    head, body = lines[:k], '\n'.join(lines[k:])
    items = [d for d in cp.itemize(body) if d['kind'] not in ('use', 'other')]
    names = []
    for d in items:
        n = d['defname'] or (d['kind'] == 'impl' and f"impl {d['trait']+' for ' if d['trait'] else ''}{d['target']}") or d['first'][:30]
        if n and n not in names: names.append(n)
    nlines = sum(d['it'].end - d['it'].start + 1 for d in items)
    ht = '\n'.join(head)
    ht = re.sub(r'cuts the items here \([^)]*\)', 'cuts the items here (' + ', '.join(names[:6]) + (', ...' if len(names) > 6 else '') + ')', ht, count=1)
    ht = re.sub(r'\d+ item\(s\), about \d+ lines', f'{len(items)} item(s), about {nlines} lines', ht, count=1)
    if ht != '\n'.join(head):
        open(p, 'w', encoding='utf8').write(ht + '\n' + body); print('head fixed', rel[-60:], len(items), nlines)
