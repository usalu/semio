#!/usr/bin/env python3
"""Builds the staged i18n-core plan from a root: `staged/plan.json` and `staged/blobs/<sha256>` (never writes below the root).

usage: build.py [--step core|<name>] [--root ROOT] [--no-write]   ROOT defaults to the live checkout; use the tree the plan will be applied to; --no-write only checks that every anchor holds
Every edit is an anchored string edit asserted against ROOT (exactly the expected number of occurrences, else the build
refuses and names the anchor); new files come from staged/files. The language-law allowlist is cut from the tree as it
will look after the plan (the ratchet: its ceiling is never raised above an existing ceiling).
After a round that rewrote anchors, re-run this build against the new tree and apply again."""
import importlib.util, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from lib import *
import importlib, re

STEPS = {"core": ("staged", ["edits_resolvers", "edits_catalogs"])}
STEPS_FILE = os.path.join(HERE, "steps.json")
if os.path.exists(STEPS_FILE):
    for _name, _row in json.load(open(STEPS_FILE, encoding="utf8")).items():
        STEPS[_name] = ("staged-" + _name, _row)
STAGED = os.path.join(HERE, "staged")
LAW = L + "/⚖️laws/🗣️language-catalogs"
ALLOWLIST = LAW + "/🔒️allowlist/🔣️.json"


def load_checker(tree=None):
    """The checker as it will be after the plan: the tree's own copy when it has one, else the staged one."""
    rel = LAW + "/🧪️tests/🗣️corpus/🐍️checker.py"
    module = type(sys)("language_law_checker")
    source = tree.text(rel) if tree is not None and tree.exists(rel) else open(os.path.join(HERE, "staged", "files", rel), encoding="utf8").read()
    exec(compile(source, rel, "exec"), module.__dict__)
    return module


def virtual_sources(tree, checker):
    rows = {row["path"]: row["text"] for row in checker.read(tree.root)}
    for rel, text in tree.new.items():
        parts = rel.split("/")
        inside = any(rel.startswith(root + "/") for root in checker.ROOTS)
        if inside and not any(part in checker.SKIP for part in parts[:-1]) and (rel.endswith(".json") or checker.SOURCE.search(rel)):
            rows[rel] = text
    return [{"path": path, "text": rows[path]} for path in sorted(rows)]


def main(argv):
    global STAGED
    root = os.path.abspath(argv[argv.index("--root") + 1]) if "--root" in argv else LIVE
    step = argv[argv.index("--step") + 1] if "--step" in argv else "core"
    STAGED = os.path.join(HERE, STEPS[step][0])
    tree = Tree(root)
    files = os.path.join(STAGED, "files")
    for base, _dirs, names in os.walk(files):
        for name in sorted(names):
            full = os.path.join(base, name)
            rel = os.path.relpath(full, files)
            if rel == ALLOWLIST:
                continue
            with open(full, encoding="utf8") as handle:
                text = handle.read()
            text = re.sub(r'"@P/([^"]+)"', lambda m: '"' + rel_import(rel, P + "/" + m.group(1)) + '"', text)
            tree.put(rel, text)
    for module in STEPS[step][1]:
        importlib.import_module(module).run(tree)
    checker = load_checker(tree)
    cut = checker.cut(virtual_sources(tree, checker))
    if tree.exists(ALLOWLIST):
        previous = json.loads(tree.text(ALLOWLIST))
        for name in ("ceiling", "defaultsCeiling"):
            if name in previous and name in cut:
                cut[name] = min(previous[name], cut[name])
    tree.put(ALLOWLIST, json.dumps(cut, ensure_ascii=False, indent=1) + "\n")
    if "--no-write" in argv:
        print("checked against", root, "-", len(tree.changed()), "files would change (nothing written)")
        return
    shutil.rmtree(os.path.join(STAGED, "blobs"), ignore_errors=True)
    os.makedirs(os.path.join(STAGED, "blobs"))
    plan = {"version": 1, "built-for": root, "created": [], "edited": []}
    for rel in tree.changed():
        data = tree.new[rel].encode("utf8")
        digest = sha(data)
        with open(os.path.join(STAGED, "blobs", digest), "wb") as handle:
            handle.write(data)
        row = {"path": rel, "after": digest}
        if rel in tree.base:
            row["base"] = tree.base[rel]
            plan["edited"].append(row)
        else:
            plan["created"].append(row)
    longest = max(len(rel) for rel in tree.changed())
    json.dump(plan, open(os.path.join(STAGED, "plan.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("built for", root)
    print("created %d, edited %d, longest path %d chars, allowlist %d files / %d literals, %d assumed-language lines in %d files" % (len(plan["created"]), len(plan["edited"]), longest, len(cut["files"]), cut["ceiling"], cut.get("defaultsCeiling", 0), len(cut.get("defaults", {}))))


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except Refusal as error:
        print("REFUSED:", error)
        sys.exit(2)
