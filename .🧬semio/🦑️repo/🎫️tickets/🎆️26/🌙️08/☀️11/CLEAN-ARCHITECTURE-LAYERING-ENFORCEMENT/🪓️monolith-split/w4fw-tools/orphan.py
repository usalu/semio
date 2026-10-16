import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import SourceFile
from cfg import *
def scan(R, census):
    res = json.load(open(census))
    names = {}
    own = {}
    for r in res:
        if 'list' in r and r['cfg'] == 'lib':
            for p, k, b, l in r['list']:
                if p.endswith('.rs'): own.setdefault(p, r['id'])
    files = {p: SourceFile(p, os.path.join(R, p)) for p in own}
    defined = collections.defaultdict(set)
    for p, f in files.items():
        for u in f.units:
            if u.kind in ('struct', 'enum', 'type', 'trait', 'union') and u.name: defined[own[p]].add(u.name)
            if u.kind == 'macro_rules!': pass
    out = []
    for p, f in files.items():
        c = own[p]
        for u in f.units:
            if u.kind != 'impl' or (u.test and False): continue
            head = re.sub(r'\s+', ' ', u.text.split('{')[0])
            m = re.search(r'impl(?:<.*?>)?\s+(.*?)\s+for\s+(.*)$', head)
            if not m: continue
            trait = re.match(r'[A-Za-z_:]*?([A-Za-z_]\w*)', m.group(1).strip().lstrip('&'))
            ty = m.group(2).strip()
            tyname = re.match(r'[&\s]*(?:mut\s+)?(?:dyn\s+)?([A-Za-z_][\w:]*)', ty)
            tn = tyname.group(1).split('::')[-1] if tyname else ty
            tr = trait.group(1) if trait else ''
            # local if the trait is local or the type (or any generic arg) is local
            toks = set(re.findall(r'[A-Za-z_]\w*', m.group(1) + ' ' + ty))
            if not (toks & defined[c]):
                out.append((c, p.split('/')[-2], u.start, head[:110]))
    return out
if __name__ == '__main__':
    for x in scan(sys.argv[1], sys.argv[2]): print(x)
