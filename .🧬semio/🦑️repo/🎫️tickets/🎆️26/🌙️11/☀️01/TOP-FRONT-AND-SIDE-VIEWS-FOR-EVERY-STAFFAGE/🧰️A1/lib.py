import json, math, os, sys
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *['..']*8))
S = os.path.join(ROOT, '🎓️teaching', '🏛️architecture', '🧍️staffage')
def path(a): return os.path.join(S, '🗂️catalog', '🐕️animals', a, '🔣️.json')
def load(a):
    with open(path(a), encoding='utf-8') as f: return json.load(f)
def save(a, doc):
    with open(path(a), 'w', encoding='utf-8', newline='\n') as f:
        json.dump(doc, f, ensure_ascii=False, indent=2); f.write('\n')
def rest(asset):
    W = {}
    for b in asset['rig']['bones']:
        a = math.radians(b['angleDeg']); c, s = math.cos(a), math.sin(a)
        L = (c, s, -s, c, b['x'], b['y'])
        P = W[b['parent']] if b['parent'] else (1, 0, 0, 1, 0, 0)
        W[b['id']] = (P[0]*L[0]+P[2]*L[1], P[1]*L[0]+P[3]*L[1], P[0]*L[2]+P[2]*L[3], P[1]*L[2]+P[3]*L[3],
                      P[0]*L[4]+P[2]*L[5]+P[4], P[1]*L[4]+P[3]*L[5]+P[5])
    return W
def ap(M, x, y): return (M[0]*x+M[2]*y+M[4], M[1]*x+M[3]*y+M[5])
