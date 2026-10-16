#!/usr/bin/env python3
"""✏️ Safe edit layer of PLUGIN-INTEGRATION-1: every write, delete and move goes through here so the pre-edit state of each path is kept once (`🗑️generated/pi1-orig/<repo-relative path>`), the touched set is recorded (`pi1-tools/touched.json`), and writes happen only inside the repository's plugin tree, framework manifests or the ticket folder."""
import json, os, shutil
from lib import *

ORIG = GEN + "/pi1-orig"
TOUCHED = TOOLS + "/touched.json"
ALLOWED = [PLUG + "/", REPO + "/🧰️framework/", REPO + "/.vscode/", REPO + "/Cargo.toml", REPO + "/📋️build-owners.json", SPLIT + "/"]


def _state():
    return json.load(open(TOUCHED)) if os.path.exists(TOUCHED) else {"existed": {}, "created": []}


def _save(st):
    write(TOUCHED, json.dumps(st, ensure_ascii=False, indent=1, sort_keys=True))


def _check(path):
    real = os.path.realpath(path)
    ok = any(real.startswith(os.path.realpath(a.rstrip("/")) + ("/" if a.endswith("/") else "")) or real == os.path.realpath(a) for a in ALLOWED)
    assert ok, ("write outside the allowed trees", path)


def _remember(path):
    st = _state()
    key = os.path.relpath(path, REPO)
    if key in st["existed"] or key in st["created"]:
        return
    if os.path.exists(path):
        dst = ORIG + "/" + key
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(path, dst)
        st["existed"][key] = True
    else:
        st["created"].append(key)
    _save(st)


def put(path, text):
    _check(path)
    _remember(path)
    write(path, text)


def delete(path):
    _check(path)
    _remember(path)
    if os.path.exists(path):
        os.remove(path)


def edit(path, old, new, count=1):
    text = read(path)
    assert text.count(old) == count, (rel(path), old[:80], text.count(old))
    put(path, text.replace(old, new))


def restore_all():
    st = _state()
    for key in st["existed"]:
        shutil.copy2(ORIG + "/" + key, REPO + "/" + key)
    for key in st["created"]:
        p = REPO + "/" + key
        if os.path.exists(p):
            os.remove(p)
