"""🕵️ Exhaustive private-method scan: private method names of dependency crate A that crate B (lib files only) calls and does not define itself."""
import re, collections
from lib import *
from crates import *
import privscan
DEPS = {"retained": ["format"], "catalog": ["format", "retained"], "rretained": ["catalog", "schema"], "encode": ["format", "schema"], "preflight": ["format", "schema"], "record": ["format", "encode", "schema"],
        "intrinsic": ["format", "retained", "catalog", "rretained", "record", "schema", "encode"], "io": ["format"], "http": ["async"]}
def files_of(key, lib_only=True):
    return [f for f in crawl(libroot(key)) if not ("🧪️tests" in f or "⏱️trace" in f)] if lib_only else crawl(libroot(key))
def own_fn_names(texts):
    out = set()
    for t in texts:
        out |= set(re.findall(r"\bfn\s+([a-z_][a-z_0-9]*)", t))
    return out
def private_methods(key):
    out = collections.defaultdict(set)
    for f in files_of(key, lib_only=False):
        t = read_stage_or_base(f)
        for ty, ms in privscan.impl_methods(t).items():
            for n, p in ms:
                if not p: out[n].add(ty)
        # trait impls: methods are public via the trait, skip
    return out
for b, deps in DEPS.items():
    btexts = [read_stage_or_base(f) for f in files_of(b)]
    calls = set()
    for t in btexts: calls |= set(re.findall(r"\.\s*([a-z_][a-z_0-9]*)\s*(?:::<[^>]*>)?\s*\(", t)) | set(re.findall(r"\b[A-Z][A-Za-z0-9_]*::([a-z_][a-z_0-9]*)\s*\(", t))
    own = own_fn_names(btexts)
    for a in deps:
        pm = private_methods(a)
        hit = {n: sorted(pm[n]) for n in pm if n in calls}
        if hit: print(b, "<-", a, hit)
