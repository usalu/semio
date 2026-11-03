"""🔎 Non-exhaustive `match` over artifact enums (E0004): arms name `Enum::Variant` of one enum, no catch-all, some variants missing (guarded arms do not count)."""
import re, sys, os, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = os.environ.get("ADIR") or (ART + "/" + CODE[code])
structs, enums, files = index_types(adir)
cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "norm-enums.pkl")
shared = pickle.load(open(cache, "rb"))
allenums = {k: v for k, v in shared.items()}
for k, defs in enums.items():
    allenums[k] = sorted({x for _, v in defs for x in v})
amb = {k for k, defs in enums.items() if len({tuple(sorted(v)) for _, v in defs}) > 1}
IMPL = re.compile(r'\bimpl\b(?:\s*<[^{;]*?>)?\s*(?:[\w:<>, ]+?\s+for\s+)?([A-Z]\w*)(?:\s*<[^{;]*?>)?\s*(?:where[^{;]*)?\{')
MATCH = re.compile(r'\bmatch\b')
for f, src in files.items():
    impls = []
    for im in IMPL.finditer(src):
        ie = match_close(src, im.end() - 1)
        if ie > 0: impls.append((im.end(), ie, im.group(1)))
    for m in MATCH.finditer(src):
        j = m.end()
        depth = 0; k = j
        while k < len(src):
            ch = src[k]
            if ch in "([": depth += 1
            elif ch in ")]": depth -= 1
            elif ch == "{" and depth == 0: break
            elif ch == ";" and depth == 0: k = -1; break
            k += 1
        if k < 0 or k >= len(src): continue
        e = match_close(src, k)
        if e < 0: continue
        body = src[k + 1:e]
        arms = []
        d = 0; start = 0; i = 0
        # split into arms: pattern => expr, where expr ends at top-level comma or after a block
        while i < len(body):
            am = re.compile(r'=>').search(body, i)
            if not am: break
            pat = body[start:am.start()]
            # find the end of the arm expression
            p = am.end()
            while p < len(body) and body[p].isspace(): p += 1
            if p < len(body) and body[p] == "{":
                q = match_close(body, p)
                nxt = q + 1
                while nxt < len(body) and body[nxt].isspace(): nxt += 1
                if nxt < len(body) and body[nxt] == ",": nxt += 1
            else:
                q = p; dd = 0
                while q < len(body):
                    if body[q] in "([{": dd += 1
                    elif body[q] in ")]}": dd -= 1
                    elif body[q] == "," and dd == 0: break
                    q += 1
                nxt = q + 1
            arms.append(pat.strip())
            start = nxt; i = nxt
        if len(arms) < 2: continue
        cover = {}; catch = False
        for pat in arms:
            guard = re.search(r'\bif\b', pat)
            ptxt = pat if not guard else pat[:guard.start()]
            if re.fullmatch(r'_|[a-z_]\w*|ref\s+\w+|mut\s+\w+', ptxt.strip()) and not guard: catch = True
            if guard: continue
            for alt in split_top(ptxt, "|", angles=False):
                am2 = re.match(r'\s*(?:\w+::)*([A-Z]\w*)::([A-Z]\w*)', alt)
                if am2:
                    en, var = am2.groups()
                    if en == "Self":
                        hit = [n for a, b, n in impls if a <= m.start() <= b]
                        en = hit[-1] if hit else en
                    cover.setdefault(en, set()).add(var)
        if catch: continue
        for en, vs in cover.items():
            if en not in allenums or en in amb or en in ("Option", "Result", "Some", "Ok", "Err"): continue
            allv = set(allenums[en])
            if not vs <= allv: continue
            if vs != allv:
                missing = sorted(allv - vs)
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: match on {en} lacks {missing}")
