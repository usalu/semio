#!/usr/bin/env python3
"""🔄️ Pulls manual on-disk edits made after the flush back into the persisted carve workspace so the differential and static checks see the tree as it is."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state


def main(apply=False):
    ws, model, roots = state.load()
    diff = []
    for p in sorted(ws.changed()):
        if p in ws.deleted:
            continue
        if not os.path.exists(p):
            diff.append((p, 'missing-on-disk'))
            continue
        disk = open(p, encoding='utf-8').read()
        if disk != ws.cur.get(p):
            diff.append((p, 'differs'))
            if apply:
                ws.cur[p] = disk
    for p, why in diff:
        print(why, p)
    print('files', len(ws.changed()), 'differing', len(diff))
    if apply and diff:
        state.save(ws)


if __name__ == '__main__':
    main('--apply' in sys.argv)
