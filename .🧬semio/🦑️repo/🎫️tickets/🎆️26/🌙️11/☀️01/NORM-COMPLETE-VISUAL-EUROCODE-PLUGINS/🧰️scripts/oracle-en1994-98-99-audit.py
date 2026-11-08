# 🔎 Re-runs the fleet-1108 norm audit into a private output and lists the oracle classification of the en1994/en1998/en1999 rows without a third-party oracle
import importlib.util, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("fleet_audit", os.path.join(HERE, "fleet-1108-audit-norm.py"))
audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(audit)
audit.OUT = os.path.join(os.path.dirname(HERE), "🗑️generated", "oracle-en1994-98-99", "audit.json")
audit.main()
report = json.load(open(audit.OUT, encoding="utf-8"))
for key in ("en1994", "en1998", "en1999"):
    for row in report[key]["rows"]:
        if row["kind"] != "check":
            continue
        deps = row["oracle_deps"][0] if row["oracle_deps"] else "no hit"
        if not deps.startswith("third-party"):
            print(key, row["id"], deps, [o[0][-60:] for o in row["oracle"]])
