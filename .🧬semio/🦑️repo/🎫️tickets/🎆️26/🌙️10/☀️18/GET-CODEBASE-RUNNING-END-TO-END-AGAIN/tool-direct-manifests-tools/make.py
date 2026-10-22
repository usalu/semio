#!/usr/bin/env python3
"""🛠️ Builds the staged increment `staged/` of the direct-tool-declarations set (stdlib only, Python 3.9+, writes nothing outside --out).

The codemod fails closed:
  - only the roots listed in scope.json are read for rewriting and written; a root that does not exist or holds no file stops the build;
  - an empty file scope stops the build; a rule that matches nothing stops the build unless the rule says `optional`;
  - after every row is applied in memory the law runs over the whole repository (rewritten roots with their new text): any finding that no allow row covers stops the build.

usage: make.py [--root BASE] [--worktree] [--list FILE] [--tool-iface DIR] [--out DIR] [--scope FILE] [--literals FILE]
  --root BASE       the tree the rows are derived from (default: the repository this ticket lives in); the rows apply to any tree whose files still hold the anchored text
  --worktree        list tracked and untracked files (an overlay that is not committed)
  --list FILE       NUL-separated file list instead of asking git
  --scope FILE      the roots and rules (default: scope.json next to this script); --literals FILE the anchored edits (default: literals.json)
  --no-law          build no create rows (self-test on a scratch repository)
  --tool-iface DIR  staged set of the tool registry; rows that need its text (`requires`) are derived from its staged files when the base lacks it
Exit 0 = staged, 1 = refused (message names the reason)."""
import hashlib
import importlib.util
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
opt = lambda name, default=None: args[args.index(name) + 1] if name in args else default
ROOT = os.path.abspath(opt("--root", LIVE))
OUT = os.path.abspath(opt("--out", os.path.join(HERE, "staged")))
TOOL_IFACE = os.path.abspath(opt("--tool-iface", os.path.join(HERE, "..", "tool-iface-tools", "staged")))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
LAW_DIR = L + "/⚖️laws/🧰️direct-tool-declarations"
B = L + "/🏗️build"

spec = importlib.util.spec_from_file_location("checker", os.path.join(HERE, "law", "checker.py"))
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)
patterns = checker.load_patterns(os.path.join(HERE, "law", "patterns.json"))
law = checker.Law(patterns)
scope = json.load(open(opt("--scope", os.path.join(HERE, "scope.json")), encoding="utf-8"))
literals = json.load(open(opt("--literals", os.path.join(HERE, "literals.json")), encoding="utf-8"))["rows"]
sha = lambda data: hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()


def refuse(message):
    print("REFUSED: " + message, file=sys.stderr)
    sys.exit(1)


def listing():
    if opt("--list"):
        return [p for p in open(opt("--list"), "rb").read().decode("utf-8").split("\0") if p]
    command = ["git", "-C", ROOT, "ls-files", "-z"] + (["-co", "--exclude-standard"] if "--worktree" in args else [])
    return [p for p in subprocess.run(command, capture_output=True, check=True).stdout.decode("utf-8").split("\0") if p]


def read(path):
    full = os.path.join(ROOT, path)
    if not os.path.isfile(full) or os.path.islink(full):
        return None
    try:
        return open(full, encoding="utf-8", newline="").read()
    except (UnicodeDecodeError, OSError):
        return None


def staged_text(path):
    manifest = os.path.join(TOOL_IFACE, "manifest.json")
    if not os.path.exists(manifest):
        return None
    for row in json.load(open(manifest, encoding="utf-8"))["files"]:
        if row["path"] == path:
            return open(os.path.join(TOOL_IFACE, "after", row["after"]), encoding="utf-8", newline="").read()
    return None


files = listing()
if not files:
    refuse("the file list is empty")
roots = scope["roots"]
under = {root: [f for f in files if f == root or f.startswith(root + "/")] for root in roots}
for root, members in under.items():
    if not members:
        refuse("root %r holds no tracked file of %s" % (root, ROOT))
in_scope = sorted(set(f for members in under.values() for f in members))
if not in_scope:
    refuse("the file scope is empty")
in_scope_set = set(in_scope)
text = {}
rows = []
sweeps = []
problems = []
report = {"rules": {}, "literals": {}}


def current(path):
    if path not in text:
        text[path] = read(path)
    return text[path]


for row in literals:
    if row["group"] not in scope["groups"]:
        problems.append("literal %s names an unknown group %s" % (row["id"], row["group"]))
        continue
    path = row["path"]
    base = current(path)
    if "requires" in row and (base is None or row["requires"] not in base):
        alternative = staged_text(path)
        if alternative is None or row["requires"] not in alternative:
            problems.append("literal %s: %s lacks %r and the staged tool set does not provide it" % (row["id"], path, row["requires"]))
            continue
        base = alternative
        text[path] = base
    if base is None:
        problems.append("literal %s: %s does not exist" % (row["id"], path))
        continue
    found = base.count(row["old"])
    if found != row["count"]:
        problems.append("literal %s: anchor occurs %d times (expected %d) in %s" % (row["id"], found, row["count"], path))
        continue
    text[path] = base.replace(row["old"], row["new"])
    out = {"kind": "literal", "id": row["id"], "group": row["group"], "path": path, "old": row["old"], "new": row["new"], "count": row["count"]}
    if "requires" in row:
        out["requires"] = row["requires"]
    if "witness" in row:
        out["witness"] = row["witness"]
    rows.append(out)
    report["literals"][row["id"]] = 1
    in_scope_set.add(path)

for group in scope["groups"]:
    for rule in [r for r in scope["rules"] if r["group"] == group]:
        regexes = [checker.glob_regex(g) for g in rule["files"]]
        pattern = re.compile(rule["pattern"], re.S)
        total = 0
        touched = 0
        for path in in_scope:
            if not any(r.match(path) for r in regexes) or not law.rewritable(path):
                continue
            base = current(path)
            if base is None:
                continue
            after, count = pattern.subn(rule["repl"], base)
            if count == 0:
                continue
            first = pattern.search(base)
            expanded = first.expand(rule["repl"])
            line_end = after.find("\n", first.start() + len(expanded))
            tail = after[first.start() + len(expanded): line_end if line_end >= 0 else len(after)][:30]
            candidates = [expanded, expanded + tail, base[max(0, first.start() - 15):first.start()] + expanded]
            text[path] = after
            if not rule.get("optional"):
                rows.append({"kind": "regex", "id": rule["id"], "group": group, "path": path, "pattern": rule["pattern"], "repl": rule["repl"], "count": count, "proof": None, "_candidates": candidates, "_before": base})
            total += count
            touched += 1
        if rule.get("optional"):
            sweeps.append({"id": rule["id"], "group": group, "files": rule["files"], "pattern": rule["pattern"], "repl": rule["repl"]})
        report["rules"][rule["id"]] = {"group": group, "files": touched, "matches": total}
        if total == 0 and not rule.get("optional"):
            problems.append("rule %s matched nothing (fails closed; mark it optional only when zero is a valid state)" % rule["id"])

for row in rows:
    if row["kind"] != "regex":
        continue
    final = text[row["path"]]
    chosen = [c for c in row.pop("_candidates") if len(c) >= 8 and c in final and c not in row["_before"]]
    row.pop("_before")
    if not chosen:
        problems.append("rule %s: no snippet proves the rewrite in %s" % (row["id"], row["path"]))
        continue
    row["proof"] = chosen[0]

creates = []
law_files = {
    LAW_DIR + "/🟦️.ts": "law/ts/law.ts",
    LAW_DIR + "/🔣️.json": "law/patterns.json",
    LAW_DIR + "/🧬️schema/🔣️.json": "law/schema.json",
    LAW_DIR + "/⌨️command/🟦️.ts": "law/ts/command.ts",
    LAW_DIR + "/🧪️tests/🟦️.ts": "law/ts/tests.ts",
    LAW_DIR + "/🧪️tests/🧫️fixtures/🔣️.json": "law/fixtures.json",
    LAW_DIR + "/🧪️tests/🐍️oracle.py": "law/checker.py",
    B + "/🧰️tool-registry/🧪️tests/🔎️exec-command/🟦️.ts": "law/ts/exec-command-tests.ts",
}
for path, source in ([] if "--no-law" in args else sorted(law_files.items())):
    data = open(os.path.join(HERE, source), "rb").read()
    if len((os.path.join(LIVE, path)).encode("utf-16-le")) // 2 > 256:
        problems.append("path over 256 UTF-16 units: " + path)
    have = read(path)
    if have is not None and have != data.decode("utf-8"):
        problems.append("create %s: the file exists with other content" % path)
    creates.append({"path": path, "sha256": sha(data), "blob": sha(data)})

residue = []
uncovered = []
for path in files:
    if law.excluded(path) is not None or law.kind_of(path) is None:
        continue
    body = text.get(path) if path in text else read(path)
    if body is None:
        continue
    for finding in law.scan_text(path, body.replace("\r\n", "\n")):
        if law.verdict(finding):
            continue
        (residue if path in in_scope_set else uncovered).append(finding)
for finding in residue[:20]:
    problems.append("residue after the rows: %(path)s:%(line)d %(tool)s %(command)s" % finding)
for finding in uncovered[:20]:
    problems.append("outside every root: %(path)s:%(line)d %(tool)s %(command)s" % finding)
if len(residue) > 20 or len(uncovered) > 20:
    problems.append("(%d residue, %d outside the roots in all)" % (len(residue), len(uncovered)))
if problems:
    print("\n".join("PROBLEM: " + p for p in problems), file=sys.stderr)
    sys.exit(1)

os.makedirs(os.path.join(OUT, "blobs"), exist_ok=True)
for row in creates:
    source = [s for p, s in law_files.items() if p == row["path"]][0]
    data = open(os.path.join(HERE, source), "rb").read()
    open(os.path.join(OUT, "blobs", row["blob"]), "wb").write(data)
manifest = {
    "version": 1,
    "base": os.path.basename(ROOT) if ROOT != LIVE else "live",
    "roots": roots,
    "rows": rows,
    "sweeps": sweeps,
    "creates": creates,
    "frozen": [L + "/", "🧰️framework/🔨️modules/🏃️process/"],
}
json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
by_group = {}
by_tool = {}
for row in rows:
    by_group.setdefault(row["group"], {"files": set(), "rows": 0, "edits": 0})
    by_group[row["group"]]["files"].add(row["path"])
    by_group[row["group"]]["rows"] += 1
    by_group[row["group"]]["edits"] += row["count"]
summary = {"files": len(set(r["path"] for r in rows)), "creates": len(creates), "rows": len(rows), "sweeps": len(sweeps), "edits": sum(r["count"] for r in rows),
           "groups": {g: {"files": len(v["files"]), "rows": v["rows"], "edits": v["edits"]} for g, v in by_group.items()},
           "rules": report["rules"], "scannedRoots": {k: len(v) for k, v in under.items()}, "inScopeFiles": len(in_scope)}
json.dump(summary, open(os.path.join(OUT, "report.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({k: summary[k] for k in ("files", "creates", "rows", "sweeps", "edits", "groups")}, ensure_ascii=False))
