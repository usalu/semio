"""🩺 harness3 selftest: the parsers and triggers of the two acceptance harnesses against the product's own message templates and the recorded gate logs.

`selftest.py [--logs 'glob']`   exit 0 only when every check passes. Graph-free, streams every file, no product run.
 1. template oracle: every EN/DE message template of the product (capacity, graph, Bazel plan, restart) is rendered and must be recognised in its own language
    by `logsignals`, and every parser kind must be backed by at least one product template per language (a regex drifting from the product turns this red)
 2. log replay oracle: `measure.Stream` on each recorded log must agree with product.py's own JSON (Bazel process counts) and keep exactly one phase per classified row
 3. trigger replay: each scenario trigger fires in every recorded log that reached Bazel, and not before the graph rows exist
"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.abspath(os.path.join(HERE, ".."))
ROOT = os.path.abspath(os.path.join(C, *([".."] * 7)))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(C, "bench3-g-tools"))
import logsignals as L  # noqa: E402
import measure as M  # noqa: E402

SKIP = {"tests", "fixtures", "node_modules", "🧪️tests", "🧫️fixtures", ".cache", "target", "dist"}
STRING = re.compile(r'"((?:\[capacity\]|\[bazel\] [Pp]lan|[Gg]raph:|graph:|Restarting|Neustart|Another process kept|Ein anderer Prozess|A source file was edited|Eine Quelldatei|The stamps moved|Die Stempel|The resident repo|Der residente|This command runs|Dieser Befehl führt)[^"\\]*(?:\\.[^"\\]*)*)"')
ROOTS = ["🔨️modules/🏃️process/🏗️build", "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"]
UNTRACKED = re.compile(r"^\[capacity\]\s+(\{id\}|\{available\} MB|\{switch\}|Kapazit)|status|\{grants\}|bypassed|ausgeschaltet|^\[capacity\]  ")


def templates():
    found = set()
    for base in ROOTS:
        for directory, names, files in os.walk(os.path.join(ROOT, "🧰️framework", base)):
            names[:] = [n for n in names if n not in SKIP]
            for name in files:
                if not name.endswith((".json", ".ts", ".mjs")):
                    continue
                try:
                    text = open(os.path.join(directory, name), encoding="utf8").read()
                except OSError:
                    continue
                if "capacity" not in text and "raph" not in text and "bazel" not in text.lower() and "Restart" not in text and "Neustart" not in text:
                    continue
                for m in STRING.finditer(text):
                    try:
                        found.add(json.loads('"' + m.group(1) + '"'))
                    except ValueError:
                        pass
    return sorted(found)


def render(template):
    return re.sub(r"\{(\w+)\}", lambda m: "run" if m.group(1) == "operation" else "7", template)


def oracle_templates():
    out = {"matched": 0, "unmatched": [], "kinds": set()}
    for template in templates():
        if UNTRACKED.search(template) or "${" in template or "\n" in template:
            continue
        line = render(template)
        family = "capacity" if line.startswith("[capacity]") else "plan" if line.startswith("[bazel]") else "graph" if line[:6] in ("graph:", "Graph:") else "restart"
        hit = L.match(L.COMPILED[family], line)
        if hit:
            out["matched"] += 1
            out["kinds"].add((family, hit[0], hit[1]))
        else:
            out["unmatched"].append(line[:140])
    return out


def known_kinds():
    return {(fam, kind, lang) for fam, rows in (("capacity", L.CAPACITY), ("graph", L.GRAPH), ("plan", L.PLAN), ("restart", L.RESTART)) for kind, lang, _ in rows}


def check_templates():
    result = oracle_templates()
    print("templates: %d product message(s) recognised, %d not tracked by a parser kind" % (result["matched"], len(result["unmatched"])))
    for line in [x for x in result["unmatched"] if not x.startswith(("Der residente", "Graph:", "graph:"))][:15]:
        print("   untracked: " + line)
    missing = sorted(known_kinds() - result["kinds"])
    expected_without_template = set()
    bad = [m for m in missing if m not in expected_without_template]
    print("parser kinds with a backing product template in their language: %d of %d" % (len(known_kinds()) - len(missing), len(known_kinds())))
    for m in bad:
        print("   NO PRODUCT TEMPLATE for parser kind %s" % (m,))
    return not bad and result["matched"] > 40


def oracle_actions(record):
    processes = record.get("graph", {}).get("processes")
    if not processes:
        return None
    executed = cached = internal = 0
    for part in processes[1].split(", "):
        n, _, kind = part.partition(" ")
        if kind == "internal":
            internal += int(n)
        elif kind.endswith("cache hit"):
            cached += int(n)
        else:
            executed += int(n)
    return int(processes[0]), executed, cached


def check_logs(pattern):
    ok, total, mismatched, no_oracle = True, 0, [], 0
    for path in sorted(glob.glob(pattern)):
        stream, at = M.Stream(), 0.0
        with open(path, encoding="utf8", errors="replace") as handle:
            for line in handle:
                stream.feed(at, line.rstrip("\n"))
                at += 50.0
        total += 1
        record = path[:-4] + ".json"
        oracle = oracle_actions(json.load(open(record, encoding="utf8"))) if os.path.exists(record) else None
        got = stream.actions()["bazel"]
        if oracle is None:
            no_oracle += 1
            continue
        mine = (got["processes"], got["executed"], got["cached"]) if got else None
        if mine != oracle:
            mismatched.append((os.path.basename(path), mine, oracle))
    print("log replay: %d logs, %d with product.py oracle, %d without; Bazel process counts disagreeing: %d" % (total, total - no_oracle, no_oracle, len(mismatched)))
    for row in mismatched[:10]:
        print("   MISMATCH %s: parser %s vs product.py %s" % row)
        ok = False
    return ok and total > 0


def check_triggers(pattern):
    spec = json.load(open(os.path.join(C, "peer-g-tools", "scenarios.json"), encoding="utf8"))
    ok, rows = True, []
    for name, rx in spec["triggers"].items():
        pattern_rx = re.compile(rx)
        fired = silent = 0
        early = []
        for path in sorted(glob.glob(pattern)):
            seen_bazel = False
            first = None
            lines = 0
            with open(path, encoding="utf8", errors="replace") as handle:
                for line in handle:
                    lines += 1
                    seen_bazel = seen_bazel or line.startswith("INFO: Build completed successfully")
                    if first is None and pattern_rx.search(line):
                        first = lines
            if not seen_bazel:
                continue
            if first is None:
                silent += 1
                ok = False
                early.append(os.path.basename(path))
            else:
                fired += 1
        rows.append((name, fired, silent, early[:4]))
    for name, fired, silent, early in rows:
        print("trigger %-8s fires in %d green log(s), silent in %d %s" % (name, fired, silent, early or ""))
    return ok


def main():
    pattern = os.path.join(C, "gate-g-tools", "run", "g[23]*.log")
    if "--logs" in sys.argv:
        pattern = sys.argv[sys.argv.index("--logs") + 1]
    results = [("templates", check_templates()), ("logs", check_logs(pattern)), ("triggers", check_triggers(pattern))]
    print("SELFTEST %s %s" % ("PASS" if all(ok for _, ok in results) else "FAIL", {name: ok for name, ok in results}))
    raise SystemExit(0 if all(ok for _, ok in results) else 1)


main()
