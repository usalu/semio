# 🧰️ WP4 T1 library: component tables, parameter naming, call-site and signature rewriting for the ShellState decomposition.
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *
from model import *

COMPS = json.load(open(GEN + '/rwp4-components.json'))
STRUCTS = [c['struct'] for c in COMPS['components']]
FIELD_OF = COMPS['names']                     # struct -> composer field name
S2CRATE = {c['struct']: c['crate'] for c in COMPS['components']}
F2S = {}
for c in COMPS['components']:
    for f in c['fields']: F2S[f] = c['struct']
SRANK = {s: i for i, s in enumerate(STRUCTS)}
PLAN1 = json.load(open(GEN + '/rwp4-t1-plan.json'))
TIER1 = {n for n, v in PLAN1.items() if v['tier1']}

def pname(struct, body_idents):
    base = FIELD_OF[struct]
    return base if base not in body_idents else base + '_state'

def body_idents_of(blank_text):
    """identifiers used as local names (not preceded by `.`), to avoid shadowing a parameter name"""
    ids = set()
    for m in re.finditer(r'(?<![.\w])([A-Za-z_]\w*)', blank_text): ids.add(m.group(1))
    return ids

def find_matching(b, i):
    d = 0
    while i < len(b):
        c = b[i]
        if c in '([{': d += 1
        elif c in ')]}':
            d -= 1
            if d == 0: return i
        i += 1
    return -1

SIG_FN = re.compile(r'\bfn\s+([A-Za-z_]\w*)\s*(<[^>]*(?:<[^>]*>[^>]*)*>)?\s*\(')

def signature(blank):
    """(fn name, index of '(' , index of matching ')', receiver text, receiver lifetime) for the first fn in blank text"""
    m = SIG_FN.search(blank)
    if not m: return None
    op = m.end() - 1
    cl = find_matching(blank, op)
    inner = blank[op + 1:cl]
    rm = re.match(r'\s*(&\s*(?:\'(\w+)\s+)?(?:mut\s+)?self|mut\s+self|self)\b', inner)
    recv = rm.group(1) if rm else None
    life = rm.group(2) if rm and rm.group(2) else None
    return dict(name=m.group(1), op=op, cl=cl, recv=recv, life=life, inner=inner)

def extra_params(info_params, life, names):
    out = []
    for s, mut in info_params:
        ref = '&' + (f"'{life} " if life else '') + ('mut ' if mut else '')
        out.append(f'{names[s]}: {ref}{s}')
    return out

def call_extras_tier1(info_params, names):
    return [names[s] for s, mut in info_params]

def call_extras_usecase(info_params):
    return [('&mut ' if mut else '&') + f'self.{FIELD_OF[s]}' for s, mut in info_params]

def insert_args(raw, b, lp, rp, extras):
    """text for a call whose '(' is at lp and ')' at rp: returns replacement for raw[lp+1:rp]"""
    args = raw[lp + 1:rp]
    if not extras: return args
    ex = ', '.join(extras)
    if args.strip() == '': return ex
    stripped = args.rstrip()
    if stripped.endswith(','):
        return stripped + ' ' + ex + args[len(stripped):]
    return stripped + ', ' + ex + args[len(stripped):]
