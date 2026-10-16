import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
from scope import *

files, texts, parent = build()
rel = lambda p: os.path.relpath(p, ROOT)
SKIP = ("/📇️directory/🪶️sqlite/", "/📇️directory/🐘️postgres/", "/📇️directory/🌐️neo4j/", "/📇️directory/🔌️port/", "/📇️directory/🔀️dispatch/")


def has_old(text):
    return any(leaf == "HubDirectory" for leaf, _ in imports_of(text))


def scope_has(p, seen=()):
    t = texts[p]
    if has_old(t):
        return True
    if USE_GLOB_SUPER.search(mask(t)) and p in parent:
        return any(scope_has(q, seen + (p,)) for q in parent[p] if q not in seen)
    return False


rows = []
for p in files:
    r = rel(p)
    if any(s in r for s in SKIP):
        continue
    t = texts[p]
    calls = trait_calls(t)
    own = has_old(t)
    if not calls and not own:
        continue
    rows.append((r, own, scope_has(p), {k: sorted({n for n, _ in v}) for k, v in calls.items()}))
for r, own, sc, calls in rows:
    print(("OWN " if own else "    ") + ("scope " if sc else "NOSCOPE ") + r)
    for k, v in sorted(calls.items()):
        print("      ", k, v)
