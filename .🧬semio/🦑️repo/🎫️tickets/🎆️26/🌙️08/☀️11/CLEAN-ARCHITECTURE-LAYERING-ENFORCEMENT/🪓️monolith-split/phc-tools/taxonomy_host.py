#!/usr/bin/env python3
"""🏷️ Registers the folder names the plugin-host carve introduced in the taxonomy's `crate-module-children` row (byte-sorted, text surgery, hash-guarded)."""
import os, sys, re, json, hashlib
from carve import REPO

PATH = os.path.join(REPO, '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json')
NAMES = ['🏛️guest-laws', '🏛️laws', '🔀️conversion', '🔗️wit-bindings', '🔧️engine']


def main(apply=False):
    text = open(PATH, encoding='utf-8').read()
    t = json.loads(text)
    row = t['semanticDirectoryMemberKinds']['crate-module-children']
    existing = row['memberNames']
    anywhere = {n: k for k, v in t['semanticDirectoryMemberKinds'].items() for n in v.get('memberNames', [])}
    todo = [n for n in NAMES if n not in anywhere]
    print('already registered elsewhere:', {n: anywhere[n] for n in NAMES if n in anywhere}, 'to add:', todo)
    i = text.index('"crate-module-children"')
    j = text.index('"memberNames": [', i)
    k = text.index(']', j)
    block = text[j:k]
    lines = block.split('\n')
    head, items = lines[0], [l for l in lines[1:] if l.strip()]
    names = [l.strip().rstrip(',').strip('"') for l in items]
    indent = re.match(r'\s*', items[0]).group(0)
    for n in sorted(todo, key=lambda s: s.encode('utf-8')):
        pos = next((x for x, e in enumerate(names) if e.encode('utf-8') > n.encode('utf-8')), len(names))
        names.insert(pos, n)
    new_block = head + '\n' + ',\n'.join(f'{indent}"{n}"' for n in names) + '\n' + re.match(r'\s*', lines[-1]).group(0)
    new_text = text[:j] + new_block + text[k:]
    json.loads(new_text)
    print('row names', len(existing), '->', len(names))
    if apply and todo:
        if open(PATH, encoding='utf-8').read() != text:
            raise RuntimeError('concurrent edit of the taxonomy')
        open(PATH, 'w', encoding='utf-8').write(new_text)


if __name__ == '__main__':
    main('--apply' in sys.argv)
