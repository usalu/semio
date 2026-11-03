import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(__file__))
import w3_ndp_values_scan as s
VALUE = re.compile(r'(?:\bndp_(?:number|number_opt|common|lookup|is_verified)|(?<![\w.])ndp|\bnormative_limit\w*|\bread|\bn|\bde)\(\s*(?:[\w:&.()]+,\s*)?$')
CASE = re.compile(r'\bc\.ndp\(\s*$')
def kind(before, code):
    if VALUE.search(before): return "value"
    if CASE.search(before): return "value" if code == "en1992" else "cite?"
    if re.search(r'\.ndps?\(\s*(?:&\[)?\s*(?:[\w:."]+,\s*)*$', before) or re.search(r'&\[\s*(?:[\w:."&]+,\s*)*$', before): return "cite"
    return "other"
def classify(code):
    out = collections.defaultdict(lambda: collections.defaultdict(list))
    for dp, dn, fn in os.walk(os.path.join(s.ART, s.CODES[code])):
        dn[:] = [d for d in dn if not d.startswith("🧪") and d not in ("tests", "target", "__pycache__", "🗑️generated")]
        for f in fn:
            if not f.endswith(".rs"): continue
            p = os.path.join(dp, f); rel = os.path.relpath(p, s.ROOT)
            for ln, line in enumerate(open(p, encoding="utf-8").read().split("\n"), 1):
                for m in s.LIT.finditer(line):
                    out[m.group(1)][kind(line[:m.start()], code)].append(f"{rel}:{ln}")
    return out
if __name__ == "__main__":
    code = sys.argv[1]
    k = classify(code)
    missing = {"en": [], "de": []}
    for i, kinds in sorted(k.items()):
        ids = s.resolve(i)
        if not ids: continue
        en = any(s.E[x]["recommended"] is not None for x in ids); de = any(s.E[x]["naDe"] is not None for x in ids)
        for kd in kinds:
            if kd in ("value", "other", "cite?"):
                if not en: missing["en"].append((i, kd, kinds[kd][0]))
                if not de: missing["de"].append((i, kd, kinds[kd][0]))
    tot = collections.Counter(kd for kinds in k.values() for kd in kinds)
    print(code, dict(tot))
    for a in ("en", "de"):
        for i, kd, w in sorted(set(missing[a])): print(" no", a, kd, i, w.split("/")[-2], w.split(":")[-1])
