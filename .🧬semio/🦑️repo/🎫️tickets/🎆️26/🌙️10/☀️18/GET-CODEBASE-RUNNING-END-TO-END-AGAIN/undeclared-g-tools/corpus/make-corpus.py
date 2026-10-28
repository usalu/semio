#!/usr/bin/env python3
"""undeclared-g corpus: collects the final oracle report of each target into corpus.json (the regression corpus). One row per target: id, command, declared key size, backend, whether the command passed under the oracle, completeness, the undeclared reads (the baseline the law compares against) grouped by class.
Law (replay.sh): re-running a target must name no undeclared read outside `expected`; after the gate-g batch lands the config and css rows shrink to the by-design/open classes and `expected` is regenerated (this file is the evidence of what the law saw before).
usage: python3 make-corpus.py <reports dir> <out json> tag=label ..."""
import json, os, sys, collections
reports, out = sys.argv[1], sys.argv[2]
def classify(p):
    b = os.path.basename(p)
    if b == "package.json" or b.startswith("tsconfig"): return "module-config"
    if b in (".gitignore", ".ignore"): return "ignore-file"
    if b in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "config.toml") and ("Cargo" in b or "toolchain" in b or p.startswith(".cargo")): return "cargo-workspace-manifest"
    if "🔨️modules/📚️library/" in p and "🧪️" not in p: return "build-library-runtime"
    if "/🖱️ui/" in p: return "tailwind-scan-region(ui)"
    if "/📚️course/" in p: return "tailwind-scan-region(course)"
    return "other"
rows = []
for spec in sys.argv[3:]:
    tag, _, note = spec.partition("=")
    path = os.path.join(reports, tag + ".json")
    if not os.path.exists(path): rows.append({"tag": tag, "note": note, "missing": True}); continue
    r = json.load(open(path, encoding="utf8"))
    plan = json.load(open(os.path.join(reports, tag + ".plan.json"), encoding="utf8")) if os.path.exists(os.path.join(reports, tag + ".plan.json")) else {}
    by = collections.defaultdict(list)
    for p in r["learned"]: by[classify(p)].append(p)
    last = r["rounds"][-1] if r["rounds"] else {}
    rows.append({"tag": tag, "id": plan.get("id"), "note": note, "command": plan.get("command"), "cwd": plan.get("cwd"), "declared": plan.get("declared"), "backend": r.get("backend"), "strength": r.get("strength"),
                 "passed": r.get("passed"), "complete": bool(r.get("passed")) and last.get("exit") == 0 and not last.get("denied"), "relaxed": last.get("relaxed", 0), "rounds": len(r["rounds"]),
                 "undeclared": sum(len(v) for v in by.values()), "classes": {k: len(v) for k, v in sorted(by.items())}, "expected": {k: sorted(v) for k, v in sorted(by.items())}})
json.dump({"version": 1, "generated": "undeclared-g", "targets": rows}, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
for row in rows: print(row.get("id") or row["tag"], "|", "passed" if row.get("passed") else "NOT-PASSED", "|", row.get("undeclared"), row.get("classes"))
