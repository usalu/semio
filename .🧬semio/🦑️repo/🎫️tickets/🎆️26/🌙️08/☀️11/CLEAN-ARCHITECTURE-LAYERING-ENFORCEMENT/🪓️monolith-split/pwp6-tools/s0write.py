import sys; sys.path.insert(0, '.')
import s0final as F
from s0final import *
import hashlib, shutil

HUB = '🪝️mounts/🧱️wp6-instance-tier1/🦀️.rs'
SNAP = SCR + '/pwp6-snap/' + PLUGP

def snap_text(rel):
    return open(SNAP + rel, encoding='utf-8').read()

def apply(real=False):
    B = F.build_all(); out = F.assemble(B)
    targets = B['targets']; P = B['P']
    sources = list(P['model'].keys())
    # 0. the pre-state must be what the analysis read, and equal to the snapshot (no peer edit)
    for rel in sources:
        cur = open(PLUG + rel, encoding='utf-8').read()
        snap = snap_text(rel)
        if cur != snap: raise SystemExit('PEER EDIT detected in ' + rel)
    hub_cur = open(PLUG + HUB, encoding='utf-8').read(); assert hub_cur == snap_text(HUB), 'hub changed'
    created, modified, deleted = [], [], []
    plan = []
    for path, tx in out.items():
        exists = os.path.exists(PLUG + path)
        if exists and path not in sources: raise SystemExit('refusing to overwrite a foreign file ' + path)
        plan.append((path, tx, exists))
    keep = {path for path, _, _ in plan}
    for rel in sources:
        if rel not in keep: deleted.append(rel)
    # hub
    ai_t = [t for t in targets.values() if t.file in S.MODULE_ROOTED][0]
    old = '#[path="../../🎮️command/📥️ingress/🦀️.rs"]'
    assert hub_cur.count(old) == 1
    new_hub = hub_cur.replace(old, '#[path="../../' + ai_t.path + '"]')
    if not real:
        print('DRY:', len(plan), 'writes,', len(deleted), 'deletions, hub ->', ai_t.path)
        return B, out, deleted, new_hub
    for path, tx, exists in plan:
        os.makedirs(os.path.dirname(PLUG + path), exist_ok=True)
        # re-read right before the edit
        if exists:
            cur = open(PLUG + path, encoding='utf-8').read()
            if cur != snap_text(path): raise SystemExit('PEER EDIT detected in ' + path)
        open(PLUG + path, 'w', encoding='utf-8').write(tx)
        (modified if exists else created).append(path)
    for rel in deleted:
        cur = open(PLUG + rel, encoding='utf-8').read()
        if cur != snap_text(rel): raise SystemExit('PEER EDIT detected in ' + rel)
        os.remove(PLUG + rel)
    open(PLUG + HUB, 'w', encoding='utf-8').write(new_hub); modified.append(HUB)
    json.dump(dict(created=created, modified=modified, deleted=deleted), open(GEN + '/pwp6-s0-touched.json', 'w'), ensure_ascii=False, indent=1)
    import touched6; touched6.merge(created, modified, deleted)
    print('WRITTEN: created', len(created), 'modified', len(modified), 'deleted', len(deleted))
    return B, out, deleted, new_hub


def reset():
    t = json.load(open(GEN + '/pwp6-s0-touched.json', encoding='utf-8'))
    for rel in t['created']:
        if os.path.exists(PLUG + rel): os.remove(PLUG + rel)
        d = os.path.dirname(PLUG + rel)
        while d.startswith(PLUG) and d != PLUG and os.path.isdir(d) and not os.listdir(d):
            os.rmdir(d); d = os.path.dirname(d)
    for rel in t['modified'] + t['deleted']:
        os.makedirs(os.path.dirname(PLUG + rel), exist_ok=True)
        shutil.copy2(SNAP + rel, PLUG + rel)
    cur = json.load(open(GEN + '/pwp6-touched.json', encoding='utf-8'))
    for k in ('created', 'modified', 'deleted'): cur[k] = sorted(set(cur[k]) - set(t[k]))
    json.dump(cur, open(GEN + '/pwp6-touched.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('RESET')

if __name__ == '__main__':
    if '--reset' in sys.argv: reset()
    else: apply(real='--real' in sys.argv)
