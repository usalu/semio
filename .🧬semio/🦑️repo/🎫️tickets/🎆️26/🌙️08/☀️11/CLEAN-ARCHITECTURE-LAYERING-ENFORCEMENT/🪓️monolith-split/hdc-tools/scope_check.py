"""Post-edit proof: every directory trait call of every hub file has its capability trait in scope, and every imported capability trait is used (own calls or glob descendants)."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
from scope import *
import callers_plan

files, texts, parent = build()
rel = lambda p: os.path.relpath(p, ROOT)
DIRCRATE = ("🌎️hub/📇️directory/🦀️.rs", "🌎️hub/📇️directory/🔤️wire/🦀️.rs", "🌎️hub/📇️directory/🧪️tests/🔮️backend-corpus/🦀️.rs", "🌎️hub/📇️directory/🔌️port/")
SKIP = callers_plan.SKIP
globsuper = {p: bool(USE_GLOB_SUPER.search(mask(texts[p]))) for p in files}
children = collections.defaultdict(list)
for c, ps in parent.items():
    for p in ps:
        if globsuper.get(c):
            children[p].append(c)


def imported(p):
    return {leaf for leaf, path in imports_of(texts[p]) if leaf in ALL_TRAITS and "semio_hub_directory" in path.split("::")[0:1] + [path]}


def scope(p, seen=()):
    s = set(imported(p))
    if globsuper[p] and p in parent:
        for q in parent[p]:
            if q not in seen:
                s |= scope(q, seen + (p,))
    return s


def descendants_need(p, seen=()):
    s = set(callers_plan.need_of(texts[p]).keys())
    for c in children.get(p, []):
        if c not in seen:
            s |= descendants_need(c, seen + (p,))
    return s


bad = 0
unused = 0
checked = 0
for p in files:
    r = rel(p)
    if callers_plan.skipped(r) or any(r.startswith(d) for d in DIRCRATE):
        continue
    need = set(callers_plan.need_of(texts[p]).keys())
    if need:
        checked += 1
        miss = need - scope(p)
        if miss:
            bad += 1
            print("MISSING", r, sorted(miss))
    imp = imported(p)
    if imp:
        extra = imp - descendants_need(p)
        if extra:
            unused += 1
            print("UNUSED ", r, sorted(extra))
    if re.search(r"\bHubDirectory\b(?!ies)", mask(texts[p])):
        print("OLDNAME", r)
        bad += 1
print("files with calls", checked, "missing", bad, "unused-import files", unused)
