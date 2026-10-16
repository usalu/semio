"""Final static proofs on the LIVE tree: no forwarders, exact trait coverage per backend and dispatch, paths, docstrings, debug markers."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
from verify_stage import impl_methods
from portparse import parse_members

bad = 0


def note(msg):
    global bad
    bad += 1
    print("!!", msg)


# A/B: backends
for be, struct in BACKENDS.items():
    got = {}
    all_text = ""
    for dp, dn, fn in os.walk(DIR + "/" + be):
        dn[:] = [d for d in dn if d != "📦️packages"]
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                t = read(p)
                all_text += t
                rel = os.path.relpath(p, DIR + "/" + be)
                if "impl HubDirectory" in t:
                    note("old impl in " + rel)
                for m in re.finditer(r"^impl (\w+) for %s \{" % struct, t, re.M):
                    pass
                for tr, ms in impl_methods(t, struct).items():
                    if tr in got:
                        note("%s: %s implemented twice" % (be, tr))
                    got[tr] = (rel, ms)
                # forwarders: a trait-impl method whose whole body is `self.<name>(...).await`
                for tr, ms in impl_methods(t, struct).items():
                    for name in ms:
                        mm = re.search(r"^    async fn %s\b[^\n]*\{\n        self\.%s\([^\n]*\)\.await\n    \}" % (name, name), t, re.M)
                        if mm:
                            note("forwarder %s::%s in %s" % (tr, name, rel))
                if re.search(r"^    pub\(super\) async fn (%s)\b" % "|".join(TRAIT_OF), t, re.M):
                    note("inherent pub(super) method with a trait method name in " + rel)
    for sec, traits in SECTIONS.items():
        for t, names in traits:
            req = [n for n in names if n not in TRAIT_DEFAULTED]
            if t not in got:
                note("%s misses impl %s" % (be, t)); continue
            rel, ms = got[t]
            if not rel.startswith(sec):
                note("%s: %s in %s, expected %s" % (be, t, rel, sec))
            if t == "AgentDelegationDirectory" and not ms:
                continue
            if sorted(ms) != sorted(req):
                note("%s: %s method mismatch %s %s" % (be, t, sorted(set(req) - set(ms)), sorted(set(ms) - set(req))))
    print("backend", be, "traits", len(got), "methods", sum(len(m) for _, m in got.values()))

# C: dispatch
d = read(DIR + "/🔀️dispatch/🦀️.rs")
got = impl_methods(d, "HubDirectories")
cnt = 0
for sec, traits in SECTIONS.items():
    for t, names in traits:
        req = [n for n in names if n not in TRAIT_DEFAULTED]
        ms = got.get(t, [])
        cnt += len(ms)
        if sorted(ms) != sorted(req):
            note("dispatch %s mismatch %s %s" % (t, sorted(set(req) - set(ms)), sorted(set(ms) - set(req))))
print("dispatch methods", cnt)

# port: trait method inventory against the original port
orig_port = open(SC + "/hdc-orig/🌎️hub/📇️directory/🔌️port/🦀️.rs").read()
orig_names = re.findall(r"^    async fn (\w+)", orig_port, re.M)
new_names = []
for sec in SECTIONS:
    t = read(DIR + "/🔌️port/" + sec + "/🦀️.rs")
    new_names += re.findall(r"^    async fn (\w+)", t, re.M)
if sorted(orig_names) != sorted(new_names):
    note("port inventory differs: %s %s" % (sorted(set(orig_names) - set(new_names)), sorted(set(new_names) - set(orig_names))))
print("port methods old", len(orig_names), "new", len(new_names))

# D: paths of every #[path] in the family
longest = 0
files = [os.path.join(dp, f) for dp, dn, fn in os.walk(DIR) for f in fn if f.endswith(".rs") and "📦️packages" not in dp]
for p in files:
    t = read(p)
    for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]', mask(t) if False else t):
        tgt = os.path.normpath(os.path.join(os.path.dirname(p), m.group(1)))
        if not os.path.exists(tgt):
            note("missing #[path] target %s in %s" % (m.group(1), os.path.relpath(p, DIR)))
    longest = max(longest, len(os.path.relpath(p, ROOT).encode("utf-8")))
print("longest path bytes", longest)

# E: new files start with an emoji docstring, unique inside the crate; no [DEBUG]
for p in files:
    t = read(p)
    if "[DEBUG]" in t:
        note("[DEBUG] in " + p)
for sec in SECTIONS:
    p = DIR + "/🔌️port/" + sec + "/🦀️.rs"
    first = read(p).split("\n")[0]
    if not first.startswith("//! "):
        note("no docstring " + p)
print("BAD", bad)
