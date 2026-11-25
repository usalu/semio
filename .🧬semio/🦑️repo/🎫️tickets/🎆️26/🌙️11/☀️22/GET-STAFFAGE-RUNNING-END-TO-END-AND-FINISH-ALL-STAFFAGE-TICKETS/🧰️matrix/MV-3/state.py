"""📟️ The state table of batch MV-3 for the report: `python3 state.py <GEN> <S>` prints one markdown row per plant from what is on disk: whether the catalog file differs from the backup of the start (`$GEN/backup/<id>.json`), its size, and the proof lines of `$GEN/proof/<id>.txt` (gate, scan, silhouette, validate, sweep, check, matrix) reduced to their verdicts. The tests of a group are read from `$GEN/proof/group-*.txt`."""
import glob, os, re, sys

gen, staffage = sys.argv[1], sys.argv[2]
ids = [line.split("\t")[0] for line in open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "plants.tsv")) if line.strip()]
groups = {}
for path in sorted(glob.glob(os.path.join(gen, "proof", "group-*.txt"))):
    lines = open(path).read().splitlines()
    for a in lines[0].split(" for ")[-1].split():
        groups[a] = (os.path.basename(path)[6:-4], lines[1:])
print("| Asset | Catalog file | Gate (all subjects) | Scan | Silhouette, validate, sweep (now) | Check CLI | Tests (group) | Matrix |")
print("| --- | --- | --- | --- | --- | --- | --- | --- |")
for a in ids:
    file = glob.glob(os.path.join(staffage, "🗂️catalog", "*", a, "🔣️.json"))[0]
    backup = os.path.join(gen, "backup", a + ".json")
    same = os.path.exists(backup) and open(backup, "rb").read() == open(file, "rb").read()
    proof = os.path.join(gen, "proof", a + ".txt")
    lines = open(proof).read().splitlines() if os.path.exists(proof) else []
    def last(prefix):
        found = [line for line in lines if line.startswith(prefix)]
        return found[-1] if found else None
    def short(line, pattern, none="not run"):
        if line is None:
            return none
        m = re.search(pattern, line)
        return m.group(0) if m else line[:60]
    gate = short(last("gate exit"), r"hard findings: \d+")
    scan = short(last("scan exit"), r"\d+ clean, \d+ with a hard finding, \d+ hard findings")
    sil = [line for line in lines if " silhouette: " in line]
    val = [line for line in lines if " validate: " in line]
    swp = [line for line in lines if " sweep: " in line]
    trio = "not run" if not sil else "; ".join([sil[-1].split(" silhouette: ")[1][:20], "validate " + re.sub(r" issues.*", "", val[-1].split(" validate: ")[1]) + (" (before " + re.sub(r" issues.*", "", val[0].split(" validate: ")[1]) + ")" if len(val) > 1 else ""), "sweep " + re.sub(r" findings.*", "", swp[-1].split(" sweep: ")[1])])
    check = short(last("check:"), r"(✓|✗)[^;]*")
    tests = "not run"
    if a in groups:
        tag, rows = groups[a]
        bad = [row.split(" exit")[0] for row in rows if " exit " in row and (" exit 0 " not in row or "failed" in row)]
        tests = f"{tag}: " + ("6 files green" if not bad and sum(1 for row in rows if " exit 0 " in row) == 6 else f"NOT GREEN {bad} ({sum(1 for row in rows if ' exit ' in row)} of 6 run)")
    matrix = short(last("matrix exit"), r"status \{[^}]*\}")
    print(f"| `{a}` | {'as found' if same else 'written'}, {os.path.getsize(file)} bytes | {gate} | {scan} | {trio} | {check} | {tests} | {matrix} |")
