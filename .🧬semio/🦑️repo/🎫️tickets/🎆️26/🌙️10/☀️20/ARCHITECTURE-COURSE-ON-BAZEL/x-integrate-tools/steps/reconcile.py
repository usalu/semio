"""Reconciliation of the staged sets where they disagree (see result-x-integrate.md, section 3).

R1  x-inputs + x-proctor: the course `check` keys on the proctor receipt (x-proctor) plus the course closure. The Rust sources and Cargo manifests x-inputs added to `check`
    are redundant behind the receipt and would invalidate `check` on edits that leave the executable unchanged, so `check` loses `proctorCargoManifests`/`proctorRustSources`,
    the named input `proctorRustSources` goes (only `test` reads proctor Cargo manifests) and the course-inputs law drops its `rust` read of `check`.
R2  x-site + x-inputs: once `build` is a cached receipt target its key must cover everything the Vite bundle reads. After x-site the router closure no longer reaches the browser bundle
    and x-inputs removes `ui/**` from `default`, so `build` declares `metadata.semio.closure.typescript` over a site-only tsconfig (entry + Vite configuration, same `paths` mapping),
    and the law gets a `build` row.
"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from proctor_step import STAGED
PROCTOR_APPLY = os.path.join(STAGED, "apply.py")


def drop_named_input(text, name):
    helpers = load_script(PROCTOR_APPLY)
    named = helpers["find_member"](text, "namedInputs", 2)
    found = helpers["find_member"](text, name, 4, named, "[", "]")
    if found is None:
        return text
    line = text.rfind("\n", 0, found[0])
    if text[found[1]] == ",":
        return text[:line] + text[found[1] + 1:]
    return text[:line].rstrip().removesuffix(",") + text[found[1]:]


def drop_check_inputs(text, names):
    helpers = load_script(PROCTOR_APPLY)
    check = helpers["find_member"](text, "check", 4, helpers["find_member"](text, "targets", 2))
    found = helpers["find_member"](text, "inputs", 6, check, "[", "]")
    items = json.loads(text[found[0]:found[1]])
    kept = [item for item in items if not (isinstance(item, str) and item in names)]
    if kept == items:
        return text
    rendered = "[\n" + ",\n".join("        " + helpers["dump"](item) for item in kept) + "\n      ]"
    return text[:found[0]] + rendered + text[found[1]:]


def build_closure(text):
    helpers = load_script(PROCTOR_APPLY)
    build = helpers["find_member"](text, "build", 4, helpers["find_member"](text, "targets", 2))
    block = text[build[0]:build[1]]
    if '"closure"' in block:
        return text
    if '"metadata"' in block:
        refuse("the course build target already has metadata; add metadata.semio.closure.typescript by hand")
    declaration = '      "metadata": { "semio": { "closure": { "typescript": ["{projectRoot}/tsconfig.site.json"] } } },\n'
    anchor = "\n      \"cache\": true,\n"
    if block.count(anchor) != 1:
        refuse("the course build target has no unique cache anchor; x-site must be applied first")
    out = block.replace(anchor, "\n" + declaration + "      \"cache\": true,\n", 1)
    return text[:build[0]] + out + text[build[1]:]


def fixture(text):
    document = json.loads(text)
    targets = document["targets"]
    reads = targets["check"]["reads"]
    targets["check"]["reads"] = [row for row in reads if row.get("kind") != "rust"]
    targets.setdefault("build", {"reads": [{"kind": "tsconfig", "path": "{project}/tsconfig.site.json"}]})
    return json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")


def run(root):
    changed = []
    course = read(root, COURSE)
    updated = drop_check_inputs(course, {"proctorCargoManifests", "proctorRustSources"})
    updated = drop_named_input(updated, "proctorRustSources")
    updated = build_closure(updated)
    json.loads(updated)
    if updated != course:
        write(root, COURSE, updated)
        changed.append(COURSE)
    if not os.path.exists(os.path.join(root, SITE_TSCONFIG)) or read(root, SITE_TSCONFIG) != SITE_TSCONFIG_TEXT:
        write(root, SITE_TSCONFIG, SITE_TSCONFIG_TEXT)
        changed.append(SITE_TSCONFIG)
    before = read(root, COURSE_INPUTS_FIXTURE)
    after = fixture(before)
    if before != after:
        write(root, COURSE_INPUTS_FIXTURE, after)
        changed.append(COURSE_INPUTS_FIXTURE)
    return changed
