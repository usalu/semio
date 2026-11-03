"""Appends showcase pass/fail rows (from Python oracle JSON in 🗑️generated) to an artifact's check-outcome feature; idempotent (python3 -I oracle_feature_rows.py <folder> <feature-dir> <fixture-stem> pass.json fail.json [skip-substrings...])."""
import json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact

folder, fdir, stem, good_path, bad_path, *skip = sys.argv[1:]
feature = artifact(folder) / "🧪️tests" / fdir / "🥒️.feature"
checks = lambda p: {c["id"]: c["utilization"] for c in json.load(open(p, encoding="utf-8"))["checks"]}
good, bad = checks(good_path), checks(bad_path)
keep = lambda k, v: v < 1000 and not any(s in k for s in skip)
rows = [f"      | pass-sc-{i + 1:02d} | shared://🚦️check-outcome/{stem}-pass/🔣️.json | {k} | pass | {v:.9g} | 1e-2 |" for i, (k, v) in enumerate((k, v) for k, v in good.items() if v <= 0.95 and keep(k, v))]
n = len(rows)
rows += [f"      | fail-sc-{n + i + 1:02d} | shared://🚦️check-outcome/{stem}-fail/🔣️.json | {k} | fail | {v:.9g} | 1e-2 |" for i, (k, v) in enumerate((k, v) for k, v in bad.items() if v >= 1.05 and keep(k, v))]
text = feature.read_text(encoding="utf-8")
if f"{stem}-pass/" in text:
    print("rows already present")
    sys.exit(0)
feature.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf-8")
print(len(rows), "rows appended")
