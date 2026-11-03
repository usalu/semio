import json, sys, collections
src = open("../catalogue-bridge-v5/geometry_gen.py").read()
exec(compile(src[:src.index("def main():")], "geometry_gen", "exec"))

GEOM_UNITS = {"Metre", "Millimetre", "Centimetre", "Degree", "Radian", "Minute", "Second", "Hour", "SquareMillimetre", "QuarticMillimetre", "SquareMetre", "PerSquareMetre", "Dimensionless"}

def vocab_spec(vocab, eid, key):
    found = [r for r in vocab if r["key"] == key and (not r["scope"] or eid in r["scope"])]
    scoped = [r for r in found if eid in r["scope"]]
    return scoped[0] if scoped else (found[0] if found else None)

if __name__ == "__main__":
    hist = collections.Counter()
    for code, (name, rel) in CASES.items():
        entries, vocab = extract_entries(code), extract_vocabulary(code)
        for eid, (title, inputs) in entries.items():
            for k in inputs:
                s = vocab_spec(vocab, eid, k)
                hist[(s or {}).get("unit", "?") + "/" + (s or {}).get("kind", "?")] += 1
    for k, v in hist.most_common(): print(v, k)
