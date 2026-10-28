"""[gram3-g] compares two ROWS_DUMP directories owner by owner, line by line (streaming): the observation rows (kind, path, fact) must be identical and in the same order; the revision of analysis rows is normalised (it hashes the source of the Rust tokenizer, which the batch edits). usage: cmp-dump.py <dirA> <dirB>"""
import json, os, re, sys
a, b = sys.argv[1:3]
names = sorted(set(os.listdir(a)) | set(os.listdir(b)))
bad = 0
def norm(line):
    r = json.loads(line)
    if r[0] == "analysis":
        f = dict(r[2]); f["provider"] = re.sub(r":[0-9a-f]{64}$", ":rev", str(f.get("provider"))); f["revision"] = "rev"; r = [r[0], r[1], f]
    return json.dumps(r, sort_keys=False)
for name in names:
    pa, pb = os.path.join(a, name), os.path.join(b, name)
    if not (os.path.exists(pa) and os.path.exists(pb)):
        print("missing", name); bad += 1; continue
    n = 0; diff = None
    with open(pa, encoding="utf8") as fa, open(pb, encoding="utf8") as fb:
        for la, lb in zip(fa, fb):
            n += 1
            if la.strip() and norm(la) != norm(lb):
                diff = n; break
        else:
            if fa.readline() or fb.readline(): diff = n + 1
    print(f"{name}: {n} rows", "IDENTICAL" if diff is None else f"DIFFERS at row {diff}")
    bad += diff is not None
sys.exit(1 if bad else 0)
