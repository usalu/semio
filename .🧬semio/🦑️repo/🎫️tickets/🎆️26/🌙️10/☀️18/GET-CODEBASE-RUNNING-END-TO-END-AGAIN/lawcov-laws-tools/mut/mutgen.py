import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
sys.path.insert(0, HERE)
from lib import *
from template import MUT_TS, SCHEMA

K = json.load(open("/private/tmp/lawcov-laws/consts.json", encoding="utf8"))
DQ = re.compile(r'"((?:[^"\\\n]|\\.)*)"')
BT = re.compile(r'`([^`$\\\n]*)`')


def fam(name):
    return {"reader": "rust-family", "paths": list(K[name])}


def rs(path):
    return {"reader": "rust-source", "paths": [fix(path)]}


def mt(path):
    return {"reader": "module-tree", "paths": [fix(path)]}


def sf(path, tests=True):
    return {"reader": "source-family", "paths": [fix(path)], "tests": tests}


def wit(path):
    return {"reader": "wit-package", "paths": [fix(path)]}


def fs(path):
    return {"reader": "file", "paths": [fix(path)]}


def src(*parts, join="\n"):
    return {"parts": list(parts), **({"join": join} if join != "\n" else {})}


def sl(part, a, b):
    return {**part, "slice": [fix(a), fix(b)]}


def names(part, prefix, items):
    return {**part, "names": [fix(i) for i in items], "prefix": fix(prefix)}


def literals(path):
    text = open(path, encoding="utf8").read()
    out = []
    for m in DQ.finditer(text):
        try:
            out.append(json.loads('"%s"' % m.group(1)))
        except ValueError:
            pass
    out += [m.group(1) for m in BT.finditer(text)]
    seen, result = set(), []
    for s in out:
        if 4 <= len(s) <= 400 and s not in seen and "\n" not in s.strip("\n")[:0]:
            seen.add(s)
            result.append(s)
    return result


def pick(found, k):
    chosen, covered, signatures = [], set(), set()
    for item in found:
        new = set(item["added"]) - covered
        if new and len(chosen) < k:
            chosen.append(item)
            covered |= set(item["added"])
            signatures.add(tuple(item["added"]))
    for item in found:
        if len(chosen) >= k:
            break
        if item in chosen:
            continue
        if tuple(item["added"]) not in signatures:
            chosen.append(item)
            signatures.add(tuple(item["added"]))
    if len(chosen) < k:
        rest = [item for item in found if item not in chosen]
        step = max(1, len(rest) // max(1, k - len(chosen)))
        chosen += rest[::step][: k - len(chosen)]
    order = {id(item): i for i, item in enumerate(found)}
    return sorted(chosen, key=lambda item: order[id(item)])


def generate(unit_norm, test_name, doc, sources, calls, k=24, extra_literals=(), law_files=None, law_unit=None):
    unit = real_unit(unit_norm)
    law_dir = real_unit(law_unit) if law_unit else unit
    module = os.path.join(law_dir, fix("🟦.ts"))
    used = {a for c in calls for a in c["args"] if isinstance(a, str)}
    sources = {name: defn for name, defn in sources.items() if name in used}
    candidates = literals(os.path.join(unit, fix("🟦.ts")))
    for extra in law_files or []:
        candidates += [s for s in literals(extra) if s not in candidates]
    candidates += [s for s in extra_literals if s not in candidates]
    spec_path = "/private/tmp/lawcov-laws/probe-%s.json" % re.sub(r"\W", "_", N(unit_norm))
    json.dump({"module": module, "sources": sources, "calls": calls, "candidates": candidates}, open(spec_path, "w", encoding="utf8"), ensure_ascii=False)
    run = subprocess.run(["nice", "-n", "10", "bun", os.path.join(HERE, "probe.ts"), spec_path], capture_output=True, text=True, timeout=1800)
    if run.returncode != 0:
        raise SystemExit("probe failed: " + run.stderr[-2000:])
    probes = json.loads(run.stdout)
    cases, report = [], []
    for call, probe in zip(calls, probes):
        if "error" in probe:
            report.append((call["fn"], "ERROR " + probe["error"][:200]))
            continue
        base_ok = probe["baseAccepted"]
        row = {"id": "%s-live" % call["fn"], "fn": call["fn"], "args": call["args"], "mutation": {"op": "none"}, "expect": {"present": True, "baseAccepted": True, "mutatedAccepted": True, "added": []}}
        if not base_ok:
            row["defect"] = "the law rejects the live production source: " + "; ".join(probe["baseMessages"])[:600]
        cases.append(row)
        chosen = pick(probe["found"], call.get("k", k))
        for i, item in enumerate(chosen):
            cases.append({"id": "%s-%s-%s-%03d" % (call["fn"], item["op"], item["target"], i), "fn": call["fn"], "args": call["args"], "mutation": {"op": item["op"], "target": item["target"], "text": item["text"]},
                          "expect": {"present": item["present"], "baseAccepted": base_ok, "mutatedAccepted": item["mutatedAccepted"], "added": item["added"]}})
        report.append((call["fn"], "base=%s found=%d chosen=%d msgs=%d sizes=%s" % (base_ok, len(probe["found"]), len(chosen), len(set(m for c in chosen for m in c["added"])), probe["sizes"])))
    corpus = {"version": 1, "sources": sources, "cases": cases}
    oracle = open(os.path.join(HERE, "..", "pylib.py"), encoding="utf8").read() + "\n" + open(os.path.join(HERE, "oracle_tpl.py"), encoding="utf8").read().replace("@@SPLIT@@", repr(json.dumps(K["POLICY_SPLIT_SOURCE_ROOTS"])))
    rel = os.path.relpath(law_dir, os.path.join(unit, fix("🧪tests"), fix(test_name))).replace(os.sep, "/") + "/" + fix("🟦.ts")
    ts = MUT_TS.replace("@@DOC@@", doc).replace('"../../🟦.ts"', json.dumps(rel, ensure_ascii=False))
    folder, plen = write_test(unit_norm, test_name, ts, corpus, SCHEMA, oracle, fixdeep=False)
    for line in report:
        print("  ", *line)
    print(folder, plen, len(cases))
    return folder
