#!/usr/bin/env python3
"""Small anchored merge fixups between round-2 sets (integrate2). usage: apply.py --op <id> [--root DIR] [--write]; dry run by default; idempotent; all-or-nothing (one file per op, temp file + rename).
Ops:
  capacity-split-cli, capacity-split-birth  mem-gov split `runBuildCliV1` and `runOwnedBuildInvocationV1` into a capacity door and `runAdmittedBuildCliV1` / `runAdmittedBuildInvocationV1`; the suites that extract the function text by name (build CLI birth, root entry native recovery) follow the function that now holds the body.
  rbe-probes    cache decision (deterministic / pure-function-of-inputs) for the five hermetic remote-execution probe targets, which bzl-rbe authored as cache:true without a decision (law: unclassified-target).
  precision-storybook  input precision rule `workspace-storybook` also names `build-storybook` (bzl-nocache made it a cached target that declares the whole repository; input-precision only knew `test-storybook`). NOTE: gen-rules.py of input-precision regenerates rules.json and must carry the same name.
  policy-driver remove the exact `driver-owned` class row of the caching policy (its four targets are classified per manifest by play-bzl and demo-bzl).
  rbe-default   bzl-default's storage fixture lists the commands that skip the JavaScript install (`installSkipped`); bzl-rbe adds `remote-execution` to that list in the Bazel entry script, so the fixture row must carry it too."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
root = args[args.index("--root") + 1] if "--root" in args else None
if root is None:
    root = HERE
    while root != os.path.dirname(root) and not (os.path.isfile(os.path.join(root, "bunfig.toml")) and os.path.isfile(os.path.join(root, "bun.lock"))):
        root = os.path.dirname(root)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
OPS = {
    "policy-driver": [(L + "/⚡️caching/🔣️policy.json", """    {
      "decision": { "class": "driver-owned", "reason": "freshness-owned-by-driver" },
      "exact": ["activate-lanes-prototype", "activate-prototype", "prepare-prototype", "prepare-release"]
    },
""", "")],
    "rbe-default": [(B + "/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🧫️fixtures/🔣️.json", '"installSkipped": ["tools", "reset", "cache"]', '"installSkipped": ["tools", "reset", "cache", "remote-execution"]')],
}
PROBES = ["chain", "compose", "descriptor", "fanout", "platform"]
OPS["rbe-probes"] = [(L + "/⚡️caching/📋️project.json", '    "remote-execution-probe-%s": {' % name, None) for name in PROBES]
PRECISION = B + "/📥️inputs/🎯️precision/🔣️.json"
op = args[args.index("--op") + 1]
plan = []


def precision_storybook(text):
    document = json.loads(text)
    rule = next(row for row in document["rules"] if row["id"] == "workspace-storybook")
    if "build-storybook" not in rule["select"]["targets"]:
        rule["select"]["targets"].insert(0, "build-storybook")
    return json.dumps(document, indent=1, ensure_ascii=False) + "\n"


def rbe_probes(text):
    anchor = '      "metadata": {\n        "semio": {\n          "hermetic": {'
    insert = '      "metadata": {\n        "semio": {\n          "cache": {\n            "version": 1,\n            "class": "deterministic",\n            "reason": "pure-function-of-inputs"\n          },\n          "hermetic": {'
    for name in PROBES:
        start = text.index('    "remote-execution-probe-%s": {\n' % name)
        end = text.index("\n    },\n", start)
        block = text[start:end]
        if '"class": "deterministic"' in block:
            continue
        if block.count(anchor) != 1:
            sys.exit("REFUSED: anchor count %d in probe %s" % (block.count(anchor), name))
        text = text[:start] + block.replace(anchor, insert) + text[end:]
    json.loads(text)
    return text


def replace_all(count, old, new):
    def function(text):
        if old not in text:
            return text
        if text.count(old) != count:
            sys.exit("REFUSED: %r occurs %d times, expected %d" % (old, text.count(old), count))
        return text.replace(old, new)
    return function


FUNCTIONS = {
    "capacity-split-cli": (B + "/🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts", lambda text: replace_all(1, ';return runOwnedBuildInvocationV1', ';return runAdmittedBuildInvocationV1')(replace_all(1, 'node.name?.text==="runOwnedBuildInvocationV1"', 'node.name?.text==="runAdmittedBuildInvocationV1"')(text))),
    "capacity-split-birth": (B + "/📥️caller/🧪️tests/📥️birth/🟦️.ts", lambda text: replace_all(1, ';return runBuildCliV1;', ';return runAdmittedBuildCliV1;')(replace_all(2, 'text==="runBuildCliV1"', 'text==="runAdmittedBuildCliV1"')(text))),"rbe-probes": (L + "/⚡️caching/📋️project.json", rbe_probes), "precision-storybook": (PRECISION, precision_storybook)}
if op in FUNCTIONS:
    rel, function = FUNCTIONS[op]
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    out = function(text)
    print("pending: 1 file(s)" if out != text else "applied")
    if out != text and "--write" in args:
        open(path + ".fixup.tmp", "w", encoding="utf8", newline="").write(out)
        os.replace(path + ".fixup.tmp", path)
        print("written")
    sys.exit(0)
for rel, old, new in OPS[op]:
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    if (new and new in text) or (not new and old not in text):
        continue
    if text.count(old) != 1:
        sys.exit("REFUSED: anchor count %d in %s" % (text.count(old), rel))
    out = text.replace(old, new)
    if rel.endswith(".json"):
        json.loads(out)
    plan.append((path, out))
print("pending: %d file(s)" % len(plan) if plan else "applied")
if plan and "--write" in args:
    for path, out in plan:
        open(path + ".fixup.tmp", "w", encoding="utf8", newline="").write(out)
        os.replace(path + ".fixup.tmp", path)
    print("written")
