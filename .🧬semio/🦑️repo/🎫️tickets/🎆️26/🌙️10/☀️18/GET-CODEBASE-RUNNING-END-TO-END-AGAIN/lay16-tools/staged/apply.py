#!/usr/bin/env python3
"""Staged anchored edits for the files behind the HARD FREEZE (taxonomy, manifests). Usage: apply.py [--apply] [patch ...]  (default: dry run of every patch)."""
import json, os, sys


HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while ROOT != os.path.dirname(ROOT) and not (os.path.isfile(os.path.join(ROOT, "bunfig.toml")) and os.path.isfile(os.path.join(ROOT, "bun.lock"))):
    ROOT = os.path.dirname(ROOT)

FW = "\U0001F9F0️framework"
TAXONOMY = os.path.join("\U0001F9F0️framework", "\U0001F6CD️products", "\U0001F991️repo", "\U0001F528️modules", "\U0001F4DA️library", "\U0001F523️taxonomy.json")
SDK_PACKAGE_JSON = os.path.join("\U0001F9F0️framework", "\U0001F4E6️packages", "\U0001F7E6️typescript", "package.json")
MANIFEST_UI_AXES_GLUE = os.path.join("\U0001F9F0️framework", "\U0001F4E6️packages", "\U0001F7E6️typescript", "\U0001F6C2️manifest", "\U0001F39A️ui-axes")


def q(*parts):
    return "/".join(parts)


M = "\U0001F9F0️framework/\U0001F528️modules"
P = "\U0001F9F0️framework/\U0001F4E6️packages/\U0001F7E6️typescript"
HASH = "\U0001F50F️hash"
PROC = "\U0001F3C3️process"
PT = "\U0001F4E6️packages/\U0001F7E6️typescript"
EXEC = "\U0001F39B️execution"
CAP = "\U0001F3D7️build/\U0001F39F️capacity"
TESTING = "\U0001F9EA️testing"
BRAND = "\U0001F518️definitions-action-ids-and-shell-brand"
MANIFEST_DIR = "\U0001F6C2️manifest"
AXES_NEW = "./" + MANIFEST_DIR + "/\U0001F39A️ui-axes/\U0001F7E6️.ts"
AXES_OLD = "./" + MANIFEST_DIR + "/\U0001F916️generated/\U0001F39A️ui-axes/\U0001F7E6️.ts"

PATCHES = {
    "taxonomy-ui-roles": {
        "file": TAXONOMY,
        "edits": [
            (
                '          "' + M + "/\U0001F6C2️manifest/" + BRAND + '"\n        ],\n        "externalPackages": []\n      },\n      "ui-test-support": {',
                '          "' + M + "/\U0001F6C2️manifest/" + BRAND + '",\n          "' + M + "/" + HASH + '",\n          "' + P + "/" + HASH + '",\n          "' + P + "/" + MANIFEST_DIR + "/\U0001F39A\ufe0fui-axes" + '"\n        ],\n        "externalPackages": []\n      },\n      "ui-test-support": {',
            ),
            (
                '          "' + M + "/" + PROC + "/" + TESTING + '",\n',
                '          "' + M + "/" + PROC + "/" + TESTING + '",\n          "' + M + "/" + PROC + "/" + PT + "/" + EXEC + '",\n          "' + M + "/" + PROC + "/" + PT + "/" + CAP + '",\n          "' + P + "/" + PROC + "/" + EXEC + '",\n          "' + P + "/" + PROC + "/" + CAP + '",\n',
            ),
        ],
    },
}


def run(name, apply):
    spec = PATCHES[name]
    path = os.path.join(ROOT, spec["file"])
    text = open(path, encoding="utf-8").read()
    status = []
    for old, new in spec["edits"]:
        if new in text and old not in text:
            status.append("already applied")
        elif text.count(old) == 1:
            text = text.replace(old, new, 1)
            status.append("anchored")
        else:
            raise SystemExit(f"{name}: anchor found {text.count(old)} times, expected 1:\n{old[:200]}")
    if path.endswith(".json"):
        document = json.loads(text)
        roles = document["dependencyDirections"]["roles"]
        for role in ("ui-foundation", "ui-test-support"):
            for owner in roles[role]["ownerPaths"]:
                if not os.path.exists(os.path.join(ROOT, owner)):
                    raise SystemExit(f"{name}: owner path does not exist: {owner}")
        if set(roles["ui-foundation"]["ownerPaths"]) & set(roles["ui-test-support"]["ownerPaths"]):
            raise SystemExit(f"{name}: foundation and test-support owners overlap")
    if apply:
        open(path, "w", encoding="utf-8").write(text)
    print(f"{name}: {', '.join(status)} ({'written' if apply else 'dry run'})")


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    for n in (args or list(PATCHES)):
        run(n, "--apply" in sys.argv)
