"""Third-party oracle for the sandbox root guard's directory sameness (python3 os.path / pathlib)."""
import json, os, shutil, sys, tempfile, unicodedata

VECTORS = sys.argv[1]
doc = json.load(open(VECTORS, encoding="utf8"))
base = os.path.realpath(tempfile.mkdtemp(prefix="guard-g-oracle-"))


def probe(kind):
    root = os.path.join(base, "probe-" + kind)
    os.makedirs(root)
    if kind == "case":
        name, other = "Probe", "pROBE"
    else:
        name, other = unicodedata.normalize("NFC", "é"), unicodedata.normalize("NFD", "é")
    os.mkdir(os.path.join(root, name))
    return os.path.exists(os.path.join(root, other))


folds = {"folds-case": probe("case"), "folds-normalization": probe("normalization")}
rows, bad = [], 0
for case in doc["cases"]:
    root = os.path.join(base, case["name"])
    os.makedirs(root)
    for op in case["ops"]:
        target = os.path.join(root, op[1])
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if op[0] == "mkdir":
            os.makedirs(target, exist_ok=True)
        elif op[0] == "file":
            open(target, "w").close()
        else:
            os.symlink(os.path.join(root, op[2]), target)
    a, b = os.path.join(root, case["a"]), os.path.join(root, case["b"])
    if os.path.exists(a) and os.path.exists(b):
        verdict = os.path.samefile(a, b)
        how = "samefile"
    else:
        verdict = os.path.realpath(a) == os.path.realpath(b)
        how = "realpath"
    expected = folds[case["same"]] if isinstance(case["same"], str) else case["same"]
    ok = verdict == expected
    bad += not ok
    rows.append(f"{case['name']:38} {how:9} oracle={str(verdict):5} expected={str(expected):5} {'ok' if ok else 'MISMATCH'}")
shutil.rmtree(base)
print("\n".join(rows))
print(f"folds-case={folds['folds-case']} folds-normalization={folds['folds-normalization']} mismatches={bad}")
sys.exit(1 if bad else 0)
