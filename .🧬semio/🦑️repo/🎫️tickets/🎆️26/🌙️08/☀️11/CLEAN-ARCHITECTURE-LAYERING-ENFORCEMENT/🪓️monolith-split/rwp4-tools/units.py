# 🧱️ WP4 units: splits a Shell source file into header, units (top-level items and ShellState members) with trivia and region stacks; round-trips losslessly.
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

IMPL_STATE = re.compile(r'impl\s+ShellState\b')

def impl_type(head):
    h = head.split('{')[0].strip()
    h = re.sub(r'^impl\s*(<[^>]*(?:<[^>]*>[^>]*)*>)?\s*', '', h)
    if ' for ' in h: h = h.split(' for ', 1)[1]
    h = re.sub(r'\s+where\b.*$', '', h)
    m = re.match(r'([A-Za-z_]\w*)', h.strip())
    return m.group(1) if m else None

def trivia_start(src, first, floor):
    s = first
    while s > floor:
        t = src.lines[s-1]
        if t.strip() == '': break
        if REGION.match(t): break
        if t.lstrip().startswith('//!'): break
        if t.lstrip().startswith(('///', '#[', '//', '/*', '*', '*/')): s -= 1
        elif src.bl[s-1].strip() == '' : s -= 1
        else: break
    return s

def gap_extras(src, a, b):
    """non-structural content of the gap lines [a, b): comments and regions that open and close inside the gap, verbatim, in order"""
    out = []; i = a
    ends = {}
    stack = []
    for j in range(a, b):
        m = REGION.match(src.lines[j])
        if m and src.bl[j].strip() == '':
            if m.group(1) == 'region': stack.append(j)
            elif stack: ends[stack.pop()] = j
    inner_closed = set()
    for s, e in ends.items(): inner_closed.update(range(s, e + 1))
    while i < b:
        l = src.lines[i]
        m = REGION.match(l)
        if m and src.bl[i].strip() == '':
            if i in ends:
                out += src.lines[i:ends[i] + 1]; i = ends[i] + 1; continue
            i += 1; continue
        if l.strip() != '': out.append(l)
        i += 1
    return out

class Unit:
    def __init__(self, **kw): self.__dict__.update(kw)

def region_events(src, lo, hi):
    ev = {}
    for i in range(lo, hi):
        m = REGION.match(src.lines[i])
        if m and src.bl[i].strip() == '' : ev[i] = (m.group(1), src.lines[i])
    return ev

def split_file(path):
    text = open(path, encoding='utf-8').read()
    src = Src(text)
    n = len(src.lines)
    its = top_items(src)
    units = []
    prev_end = -1
    for it in its:
        floor = prev_end + 1
        s = trivia_start(src, it['first'], floor)
        k = it['kind']
        u = Unit(file=path, first=s, code=it['first'], last=it['last'], kind=k, head=it['head'], members=None, name=None)
        if k == 'impl':
            u.name = impl_type(it['head'])
            u.state_impl = bool(IMPL_STATE.match(it['head']))
            if u.state_impl:
                mem = []
                mprev = it['first']
                for m in top_items(src, it['first'] + 1, it['last'], 4):
                    ms = trivia_start(src, m['first'], mprev + 1)
                    lead = gap_extras(src, mprev + 1, ms)
                    mem.append(Unit(file=path, first=ms, code=m['first'], last=m['last'], kind=m['kind'], head=m['head'], name=item_name(m['head'], m['kind']), lead=lead, trail=[]))
                    mprev = m['last']
                if mem:
                    mem[-1].trail = gap_extras(src, mprev + 1, it['last'])
                u.members = mem
        else:
            u.name = item_name(it['head'], k)
            u.state_impl = False
        units.append(u)
        prev_end = it['last']
    return src, units

if __name__ == '__main__':
    from model import *
    bad = 0
    for d, p in shell_files():
        src, units = split_file(p)
        # check: lines covered exactly once by gap/unit partitions is trivially true; verify spans nested & ordered
        last = -1
        for u in units:
            assert u.first > last, (d, u.name, u.first, last)
            last = u.last
            if u.members:
                ml = u.code
                for m in u.members:
                    assert m.first > ml and m.last < u.last, (d, m.name)
                    ml = m.last
        print(d, len(units), sum(len(u.members or []) for u in units))
