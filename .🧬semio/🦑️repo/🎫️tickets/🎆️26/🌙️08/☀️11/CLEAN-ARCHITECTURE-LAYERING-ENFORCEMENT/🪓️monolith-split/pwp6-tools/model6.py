import sys; sys.path.insert(0, '.')
from lib6 import *
from methods6 import INSTANCE_FILES, FN

class Member:
    def __init__(self, **k): self.__dict__.update(k)
class Piece:
    def __init__(self, **k): self.__dict__.update(k)

def parse_file(rel, plugp=PLUGP, root=ROOT):
    path = root + plugp + rel
    raw = open(path, encoding='utf-8').read()
    R = raw.split('\n'); L = mask(raw).split('\n')
    its, _, _ = pt.outline(path)
    pieces = []
    prev_end = 0
    for it in its:
        s = it['s'] - 1; e = it['e']
        # extend upward over doc/attr/plain comment lines directly above (no blank between)
        while s > prev_end and R[s - 1].strip().startswith(('///', '#[', '//')) and not R[s - 1].strip().startswith('//!'): s -= 1
        # lines between previous end and s: gap (blank or stray comments)
        gap = R[prev_end:s]
        p = Piece(kind='item', s=s, e=e, itkind=it['kind'], head=it['head'], name=it['name'], gap=[g for g in gap], file=rel)
        if it['kind'] == 'impl':
            tgt, tr = pt.impl_target(it['head'])
            p.target, p.trait = tgt, tr
            if tgt == 'VcsArtifactApp':
                p.kind = 'vcsimpl'
                # head = from s .. line containing the opening brace of the impl
                k = it['s'] - 1
                while '{' not in L[k]: k += 1
                p.open_line = k
                p.head_lines = R[s:k + 1]
                # members: depth-1 fn items
                depth = 0; ms = []
                for j in range(s, e):
                    l = L[j]; before = depth
                    if j > k or (j == k and False):
                        pass
                    depth = before + l.count('{') - l.count('}')
                    if j > k and before == 1:
                        m = FN.match(l)
                        if m:
                            a = j
                            while a - 1 > k and R[a - 1].strip().startswith(('///', '#[', '//')): a -= 1
                            ms.append(Member(name=m.group(3), start=j, a=a, vis=(re.match(r'\s*(pub(?:\([^)]*\))?)\s', l).group(1) if re.match(r'\s*pub', l) else ''), sig=l.strip()))
                for x, y in zip(ms, ms[1:] + [None]):
                    x.e = (y.a if y else e - 1)
                p.members = ms
                p.close_idx = e - 1
                p.first_a = ms[0].a if ms else e - 1
        pieces.append(p)
        prev_end = e
    tail = R[prev_end:]
    first_start = pieces[0].s if pieces else len(R)
    header = R[:first_start]
    # a dangling outer doc comment at the end of the header belongs to the first item
    k = len(header)
    while k > 0 and (not header[k - 1].strip() or header[k - 1].lstrip().startswith('///')): k -= 1
    dangling = [l for l in header[k:] if l.strip()]
    header = header[:k + (len(header[k:]) - len(header[k:]))] if False else header
    if dangling:
        header = header[:k] + [l for l in header[k:] if not l.lstrip().startswith('///')]
    pieces[0].gap = dangling
    return dict(rel=rel, R=R, L=L, header=header, pieces=pieces, tail=tail, raw=raw)

def nonblank(lines): return [l for l in lines if l.strip()]

def lossless_self_check(f):
    """pieces + gaps + header + tail reproduce the file's non-blank lines"""
    out = list(f['header'])
    for p in f['pieces']:
        out += p.gap
        out += f['R'][p.s:p.e]
    out += f['tail']
    return nonblank(out) == nonblank(f['R'])

if __name__ == '__main__':
    tot = 0
    for rel in INSTANCE_FILES:
        f = parse_file(rel)
        ok = lossless_self_check(f)
        nm = sum(len(p.members) for p in f['pieces'] if p.kind == 'vcsimpl')
        tot += nm
        gaps = sum(len(nonblank(p.gap)) for p in f['pieces'])
        print(ok, rel.ljust(60)[:60], 'header', len(f['header']), 'pieces', len(f['pieces']), 'methods', nm, 'strayGapLines', gaps)
    print(tot)
