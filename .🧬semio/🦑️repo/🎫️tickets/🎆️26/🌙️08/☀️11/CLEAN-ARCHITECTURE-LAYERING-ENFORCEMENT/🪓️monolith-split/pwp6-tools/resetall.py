import sys; sys.path.insert(0, '.')
from common6 import *
import touched6
SNAP = SCR + '/pwp6-snap/' + PLUGP
def run():
    t = touched6.load()
    for rel in t['created']:
        p = PLUG + rel
        if os.path.exists(p): os.remove(p)
        d = os.path.dirname(p)
        while d.startswith(PLUG) and d != PLUG and os.path.isdir(d) and not os.listdir(d):
            os.rmdir(d); d = os.path.dirname(d)
    for rel in t['modified'] + t['deleted']:
        os.makedirs(os.path.dirname(PLUG + rel), exist_ok=True)
        shutil.copy2(SNAP + rel, PLUG + rel)
    json.dump(dict(created=[], modified=[], deleted=[]), open(GEN + '/pwp6-touched.json', 'w'))
    print('RESET ALL: removed', len(t['created']), 'restored', len(t['modified']) + len(t['deleted']))
import shutil
if __name__ == '__main__': run()
