"""🏷️ Derives the platform verdicts of the three suite route tables from the live tree and writes the staged copies (tables and the project targets of the suites that skip a host). Re-run it (`python3 -I tag.py`) when `apply.py check` reports DRIFT on a table or project file: it re-reads the live files and the live test files (so suites added by other batches get their verdict too), rewrites the staged copies and records the live digests as their bases.
Usage: python3 -I tag.py [--root DIR] [--dry]"""
import importlib.util, json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
DRY = "--dry" in sys.argv
F = "🧰️framework"
LIB = F + "/🛍️products/🦑️repo/🔨️modules/📚️library"
SUITES = F + "/🔨️modules/🏃️process/🧪️testing/🗂️suites"
MODULE = SUITES + "/🖥️platforms"
TABLES = {
    "process": (SUITES + "/🔣️.json", F + "/🔨️modules/🏃️process/📋️project.json", F + "/🔨️modules/🏃️process"),
    "caching": (LIB + "/⚡️caching/🗂️suites/🔣️.json", LIB + "/⚡️caching/📋️project.json", LIB + "/⚡️caching"),
    "build": (LIB + "/🏗️build/🗂️suites/🔣️.json", LIB + "/⚡️caching/📋️project.json", LIB + "/🏗️build"),
}
CI_POLICY = LIB + "/🏗️build/🧾️ci-report/🗂️suites/🔣️.json"
LIVE_CACHE = {"version": 1, "class": "live", "reason": "exercises-host-resources"}
spec = importlib.util.spec_from_file_location("oracle", os.path.join(HERE, "files", MODULE, "🧪️tests/🐍️oracle/🐍️.py"))
oracle = importlib.util.module_from_spec(spec); spec.loader.exec_module(oracle)
rules = oracle.compile_policy(json.load(open(os.path.join(HERE, "files", MODULE, "🔣️.json"), encoding="utf-8")))
enumerated = json.loads(subprocess.run(["bun", "run", os.path.join(HERE, "..", "..", "enumerate.ts"), ROOT], check=True, capture_output=True, text=True, cwd=ROOT).stdout)
policy = json.load(open(os.path.join(ROOT, CI_POLICY), encoding="utf-8"))
required = {(row["table"], row["suite"]) for row in policy["mustRun"]["all"] + policy["mustRun"]["win32"]}
verdicts = {}
for row in enumerated:
    files = [os.path.join(row["root"], f) for f in row["files"]] + ([os.path.join(row["root"], row["function"]["module"])] if row["function"] else [])
    causes = oracle.suite_causes(files, rules)
    keep = (row["table"], row["id"]) in required
    verdicts[(row["table"], row["id"])] = (causes, keep)
def put(rel, text):
    path = os.path.join(HERE, "files", rel)
    if not DRY:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)
subset = {"process": [], "caching": []}
summary = {"all": 0, "posix": 0, "required-with-causes": []}
for name, (table_rel, project_rel, _) in TABLES.items():
    raw = open(os.path.join(ROOT, table_rel), encoding="utf-8").read()
    table = json.loads(raw)
    assert json.dumps(table, indent=2, ensure_ascii=False) + "\n" == raw or json.dumps(table, indent=2, ensure_ascii=False) == raw, table_rel
    for suite in table["suites"]:
        causes, keep = verdicts[(name, suite["id"])]
        suite.pop("platforms", None); suite.pop("platformCauses", None)
        if not causes or keep:
            suite["platforms"] = ["darwin", "linux", "win32"]
            summary["all"] += 1
            if causes: summary["required-with-causes"].append("%s/%s %s" % (name, suite["id"], ",".join(causes)))
        else:
            suite["platforms"] = ["darwin", "linux"]
            suite["platformCauses"] = causes
            summary["posix"] += 1
            subset["caching" if name in ("caching", "build") else "process"].append(suite["id"])
    put(table_rel, json.dumps(table, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else ""))
flips = {}
for key, project_rel in (("process", TABLES["process"][1]), ("caching", TABLES["caching"][1])):
    raw = open(os.path.join(ROOT, project_rel), encoding="utf-8").read()
    lines = raw.split("\n")
    changed = []
    for index, line in enumerate(lines):
        stripped = line.strip().rstrip(",")
        if not stripped.startswith('"test-'): continue
        name = stripped.split('"')[1]
        if name[5:] not in subset[key]: continue
        compact = stripped.count("\n") == 0 and stripped.endswith("}") and stripped.split(": ", 1)[1].startswith("{") and "\n" not in stripped
        if key == "process":
            continue
        target = json.loads(stripped.split(":", 1)[1].strip())
        if target.get("cache") is False and target.get("metadata", {}).get("semio", {}).get("cache", {}).get("class") == "live": continue
        target["cache"] = False
        target.setdefault("metadata", {}).setdefault("semio", {})["cache"] = dict(LIVE_CACHE)
        text = json.dumps(target, ensure_ascii=False, separators=(",", ":"))
        indent = line[:len(line) - len(line.lstrip())]
        lines[index] = indent + '"' + name + '": ' + text + ("," if line.rstrip().endswith(",") else "")
        changed.append(name)
    if key == "process":
        project = json.loads(raw)
        assert json.dumps(project, indent=2, ensure_ascii=False) in (raw, raw.rstrip("\n"))
        for name, target in project["targets"].items():
            if name.startswith("test-") and name[5:] in subset["process"] and not (target.get("cache") is False and target.get("metadata", {}).get("semio", {}).get("cache", {}).get("class") == "live"):
                target["cache"] = False
                target.setdefault("metadata", {}).setdefault("semio", {})["cache"] = dict(LIVE_CACHE)
                changed.append(name)
        put(project_rel, json.dumps(project, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else ""))
    else:
        put(project_rel, "\n".join(lines))
    flips[key] = changed
import hashlib
if not DRY:
    regenerated = {table_rel for table_rel, _, _ in TABLES.values()} | {project_rel for _, project_rel, _ in TABLES.values()}
    rows = [line.rstrip("\n").split("\t", 1) for line in open(os.path.join(HERE, "bases.tsv"), encoding="utf-8") if line.strip()]
    with open(os.path.join(HERE, "bases.tsv"), "w", encoding="utf-8") as handle:
        for base, rel in rows:
            if rel in regenerated:
                base = hashlib.sha256(open(os.path.join(ROOT, rel), "rb").read()).hexdigest()
            handle.write(base + "\t" + rel + "\n")
print(json.dumps({"summary": summary, "flipped-targets": {k: len(v) for k, v in flips.items()}}, ensure_ascii=False, indent=1))
