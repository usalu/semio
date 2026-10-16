#!/usr/bin/env python3
"""🧩️ Dry run or apply of the law-group re-pointing for the scripts that name package semio-framework-plugin-host."""
import os, sys, difflib, hashlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, lawmap, tsfix_host, plan_host
from carve import REPO

SCRIPTS = [
    '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts',
    '🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts',
    '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts',
]


def main(apply=False):
    ws, model, roots = state.load()
    table = lawmap.build_map(model, state.base_ws(ws))
    for rel in SCRIPTS[1:]:
        p = os.path.join(REPO, rel)
        text = open(p, encoding='utf-8').read()
        new, skipped, unmapped = tsfix_host.rewrite_text(text, table)
        d = list(difflib.unified_diff(text.split('\n'), new.split('\n'), lineterm='', n=0))
        print('==', rel.split('/')[-4], len(d), 'diff lines; skipped', len(skipped), 'unmapped', unmapped[:5])
        if apply and new != text:
            if open(p, encoding='utf-8').read() != text:
                raise RuntimeError('concurrent edit ' + p)
            open(p, 'w', encoding='utf-8').write(new)


if __name__ == '__main__':
    main('--apply' in sys.argv)
