"""🔗 Union sandbox: simulates the carved crates as modules of one crate (crate-level privacy kept), then runs the full-resolution differential."""
import os, re, shutil, sys, pickle, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C
import roracle as o

def mounted_files(libroot_abs):
    seen = []; stack = [libroot_abs]
    while stack:
        f = stack.pop()
        if f in seen: continue
        seen.append(f)
        text = open(f, encoding="utf8").read()
        for m in PATH_RX.finditer(text):
            raw = m.group(1) if m.group(1) is not None else m.group(2)
            if not raw: continue
            p = os.path.normpath(os.path.join(os.path.dirname(f), raw))
            if p.endswith(".rs") and os.path.exists(p): stack.append(p)
    return seen

def overlay(dest):
    shutil.rmtree(dest, ignore_errors=True)
    shutil.copytree(BASE, dest, symlinks=True)
    for dp, dn, fn in os.walk(STAGE):
        for f in fn:
            sp = os.path.join(dp, f); rel = os.path.relpath(sp, STAGE)
            if f.endswith(".deleted"):
                t = os.path.join(dest, rel[:-len(".deleted")])
                if os.path.exists(t): os.remove(t)
                continue
            tp = os.path.join(dest, rel)
            os.makedirs(os.path.dirname(tp), exist_ok=True)
            shutil.copy2(sp, tp)

ALLOC_RX = re.compile(r"(#\[cfg\(test\)\]\s*)?#\[global_allocator\]\s*static [^;]*;")

def build_sandbox(dest, facade=True, tests_wrappers=()):
    overlay(dest)
    glue = os.path.join(dest, FACADE_ROOT)
    gdir = os.path.dirname(glue)
    sets = {k: mounted_files(os.path.join(dest, C.libroot(k))) for k in C.KEYS}
    owner = {}
    for k, fl in sets.items():
        for f in fl:
            assert f not in owner, ("file owned by two crates", f, owner[f], k)
            owner[f] = k
    for k, fl in sets.items():
        mod = "__" + C.ident(k)
        for f in fl:
            t = open(f, encoding="utf8").read()
            t = re.sub(r"(?<![\w$])crate::(?!__)", "crate::" + mod + "::", t)
            t = re.sub(r"pub\(\s*crate\s*\)", "pub(in crate::" + mod + ")", t)
            t = ALLOC_RX.sub("", t)
            open(f, "w", encoding="utf8").write(t)
    for dp, dn, fn in os.walk(os.path.join(dest, R)):
        for f in fn:
            if not f.endswith(".rs"): continue
            p = os.path.join(dp, f)
            t = open(p, encoding="utf8").read(); n = t
            for k in C.KEYS:
                n = re.sub(r"(?<![\w])(?<!::)" + C.ident(k) + r"(?=\s*::)", "crate::__" + C.ident(k), n)
            if n != t: open(p, "w", encoding="utf8").write(n)
    if not facade:
        g = ""
    else:
        g = open(glue, encoding="utf8").read()
        for k in C.KEYS:
            g = re.sub(r"(?<![\w])(?<!::)" + C.ident(k) + r"(?=\s*::)", "crate::__" + C.ident(k), g)
        g = ALLOC_RX.sub("", g)
    for k in C.KEYS:
        rel = os.path.relpath(os.path.join(dest, C.libroot(k)), gdir)
        g += f'\n#[path = "{rel}"]\npub mod __{C.ident(k)};\n'
    extra = os.path.join(dest, R, "🔗️causal/🔀️transition/🧪️tests/🔬️supersede-fold/🦀️.rs")
    if os.path.exists(extra):
        g += '\n#[cfg(test)]\n#[path = "' + os.path.relpath(extra, gdir) + '"]\nmod __history_supersede_fold;\n'
    open(glue, "w", encoding="utf8").write(g)
    o.patch_tree(os.path.join(dest, R))
    return glue

EXT = ("`semio_framework_", "semio_framework_", "`serde", "`blake3`", "serde_json", "`serde_json`", "CARGO_MANIFEST_DIR", "cannot find attribute", "`dsl`", "`protocol`")

def split_sig(counter):
    ext = collections.Counter(); inner = collections.Counter()
    for (code, m, loc), n in counter.items():
        norm = re.sub(r"`[^`]*`", "`_`", m)
        if any(x in m for x in EXT) and not any(("crate::__" + C.ident(k)) in m for k in C.KEYS):
            ext[(code, norm)] += n
        else:
            inner[(code, m)] += n
    return ext, inner

def run_all(dest, tags=None, show=60):
    base = pickle.load(open(SP + "/base-rep.pkl", "rb"))
    lib = os.path.join(dest, FACADE_ROOT)
    for tag, extra in o.CFGS.items():
        if tags and tag not in tags: continue
        c, err = o.run(lib, "protocol", extra)
        open(SP + "/err-S1-" + tag + ".txt", "w").write(err)
        b = base[tag]
        be, bi = split_sig(b); ce, ci = split_sig(c)
        only_split = ci - bi; only_base = bi - ci
        print(f"== {tag}: baseline {sum(b.values())} (internal {sum(bi.values())}) split {sum(c.values())} (internal {sum(ci.values())}); only-split {sum(only_split.values())} only-baseline {sum(only_base.values())}")
        for (code, m), n in sorted(only_split.items())[:show]: print("  +split", n, code, m[:230])
    return

if __name__ == "__main__":
    dest = SP + "/S1"
    build_sandbox(dest, facade=("--facade" in sys.argv))
    run_all(dest, tags=[a for a in sys.argv[1:] if not a.startswith("--")] or None)
