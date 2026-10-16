# 🗺️ WP4 assignment: ShellState members by Appendix B, WP5-bound free items by Appendix C with a downward pin fixpoint.
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import *
from units import *

RANK = ["shell-vocab","shell-async","shell-chrome-state","shell-surfaces","shell-plugins","shell-history","shell-prefs","shell-directory","shell-sync","shell-docs","shell-doc-open","shell-windows","shell-dock","shell-input","shell-commands","shell-state","shell-session","shell-window-actions","shell-actions","shell-pointer","shell-view-paint","shell-view-chrome","shell-view-overlays","shell-view-panels"]
WP5 = set(RANK[RANK.index('shell-session'):])
FREE_FN_HOMES = {}
GENERIC = {"label", "refusal", "tests", "new", "default", "from", "drop", "debug_log"}

def slug(d): return d
def short(d): return d.split('/')[0]

def load():
    ab = appendix_b()
    home = {}
    for c, ns in ab.items():
        for n in ns: home[n] = c
    files = []
    for d, p in shell_files():
        src, units = split_file(p)
        files.append(dict(slug=d, path=p, src=src, units=units))
    return home, files

def free_candidates(f, u):
    d = f['slug']
    if u.kind in ('use', 'mod') or u.state_impl: return None
    nm = u.name or ''
    src = f['src']
    if u.kind == 'fn' and nm in FREE_FN_HOMES and FREE_FN_HOMES[nm] in WP5: return FREE_FN_HOMES[nm]
    if d == '🧱️chrome-paint-and-navbar':
        if nm == 'ShellHubRemoteV1': return None
        if u.kind == 'mod': return None
        return 'shell-view-paint'
    if d == '🖼️chrome-frame-and-main-window': return 'shell-view-chrome'
    if d == '🎨️theme-file-import-and-accessibility':
        a, b = 1566, 2107
        ln = u.code + 1
        if a <= ln <= b and not (ln >= 1709 and ln <= 2067): return 'shell-view-paint'
    if d == '🗨️chrome-overlays-and-tour' and nm in ('chrome_tour_layout', 'chrome_tour_anchor_rect'): return 'shell-view-overlays'
    return None

def tokens(src, u):
    t = '\n'.join(src.bl[u.code:u.last + 1])
    return t

def analyse(rehomed=True):
    home, files = load()
    if rehomed:
        p = GEN + '/rwp4-homes.json'
        if os.path.exists(p):
            home = json.load(open(p))
            FREE_FN_HOMES.clear(); FREE_FN_HOMES.update(home)
    items = []
    for f in files:
        for u in f['units']:
            if u.state_impl:
                for m in u.members:
                    m.crate = home[m.name]; m.cand = None; m.f = f; m.is_member = True; m.text = tokens(f['src'], m)
                    items.append(m)
            else:
                u.cand = free_candidates(f, u)
                u.crate = u.cand
                u.f = f; u.is_member = False; u.text = tokens(f['src'], u)
                items.append(u)
    return home, files, items

if __name__ == '__main__':
    home, files, items = analyse()
    # defs: names of candidate free items (impl blocks bind to their type)
    cand = [u for u in items if not u.is_member and u.cand]
    bytype = collections.defaultdict(list)
    for u in cand:
        if u.kind == 'impl': continue
        bytype[u.name].append(u)
    for u in cand:
        if u.kind == 'impl' and u.name in bytype: pass
    names = {n for n in bytype if n and n not in GENERIC and len(n) > 3}
    rx = re.compile(r'(?<![A-Za-z0-9_])(' + '|'.join(sorted(map(re.escape, names), key=len, reverse=True)) + r')(?![A-Za-z0-9_])')
    # unit identity -> crate key; impl blocks inherit type unit
    def key(u): return id(u)
    typeunit = {}
    for n, us in bytype.items():
        for u in us: typeunit[(u.f['slug'], n)] = u
    users = collections.defaultdict(set)   # name -> set of user units
    for u in items:
        if not u.text: continue
        for m in set(rx.findall(u.text)):
            if u.kind == 'fn' or True:
                users[m].add(u)
    # pinned fixpoint
    pinned = set()
    changed = True
    cur = {id(u): u.cand for u in cand}
    def eff(u):
        if u.is_member: return u.crate
        return cur.get(id(u), None) if id(u) in cur else None
    # impl blocks of a candidate type are candidates only when their type is; bind
    for u in cand:
        if u.kind == 'impl' and u.name not in bytype: cur[id(u)] = None
    while changed:
        changed = False
        for n, us in bytype.items():
            group = us + [x for x in cand if x.kind == 'impl' and x.name == n]
            for d in us:
                if cur.get(id(d)) is None: continue
                for usr in users.get(n, ()):
                    if usr in group or usr is d: continue
                    uc = eff(usr)
                    if uc is None or (uc in RANK and RANK.index(uc) < RANK.index(cur[id(d)])):
                        # user outside WP5 or lower: pin (outside) or lower to user's crate
                        if uc is None:
                            for g in group: cur[id(g)] = None
                        else:
                            for g in group: cur[id(g)] = uc
                        changed = True
                        break
    stage = collections.defaultdict(list)
    for u in cand:
        if cur.get(id(u)): stage[cur[id(u)]].append(u)
    for c in RANK[RANK.index('shell-session'):]:
        us = stage.get(c, [])
        print(c, len(us), sum(u.last - u.first + 1 for u in us))
    pin = [u for u in cand if cur.get(id(u)) is None and u.cand]
    print('pinned (stay with tier 1):', len(pin), sum(u.last - u.first + 1 for u in pin))
    print(sorted({u.name for u in pin if u.name})[:200])
    cnt = collections.Counter()
    for u in items:
        if u.is_member: cnt[u.crate] += 1
    print(dict(cnt))
