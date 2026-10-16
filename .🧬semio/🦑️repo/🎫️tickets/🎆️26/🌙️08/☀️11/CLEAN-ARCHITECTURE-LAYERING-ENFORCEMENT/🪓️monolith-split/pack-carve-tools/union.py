"""🔗 Union sandbox: simulates the carved crates as modules of the facade crate, enforcing crate-level privacy, then runs the full-resolution differential."""
import os, re, shutil, subprocess, sys, pickle, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import oracle as o
from crates import *
from rs import tokenize

PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')
INCLUDE_RX = re.compile(r'\binclude!\(\s*"([^"]*)"\s*\)')

def mounted_files(root, libroot_abs):
    """All .rs files reachable from a lib root through #[path] mounts (cfg-agnostic)."""
    seen = []; stack = [libroot_abs]
    while stack:
        f = stack.pop()
        if f in seen: continue
        seen.append(f)
        text = open(f, encoding="utf8").read()
        for m in list(PATH_RX.finditer(text)) + list(INCLUDE_RX.finditer(text)):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if p.endswith(".rs") and os.path.exists(p): stack.append(p)
    return seen

def build_sandbox(dest, only=None):
    shutil.rmtree(dest, ignore_errors=True)
    shutil.copytree(BASE, dest, symlinks=True)
    for dp, dn, fn in os.walk(STAGE):
        for f in fn:
            sp = os.path.join(dp, f); rel = os.path.relpath(sp, STAGE)
            if f.endswith(".deleted"):
                t = os.path.join(dest, rel[:-len(".deleted")])
                if os.path.exists(t): os.remove(t)
                continue
            if not rel.endswith(".rs"): continue
            tp = os.path.join(dest, rel)
            os.makedirs(os.path.dirname(tp), exist_ok=True)
            shutil.copy2(sp, tp)
    present = [k for k, *_ in CRATES if os.path.exists(os.path.join(STAGE, libroot(k))) and (only is None or k in only)]
    glue = os.path.join(dest, P, "📦️packages/🦀️rust/🦀️.rs")
    # collect file sets before rewriting
    sets = {k: mounted_files(dest, os.path.join(dest, libroot(k))) for k in present}
    owner_of = {}
    for k, fl in sets.items():
        for f in fl:
            if "⏱️trace" in f: continue
            assert f not in owner_of, ("file owned by two crates", f, owner_of[f], k)
            owner_of[f] = k
    # rewrite crate-owned files
    for k, fl in sets.items():
        mod = "__" + ident(k)
        for f in fl:
            if "⏱️trace" in f: continue
            t = open(f, encoding="utf8").read()
            t = re.sub(r"(?<![\w$])crate::(?!__)", "crate::" + mod + "::", t)
            t = re.sub(r"pub\(\s*crate\s*\)", "pub(in crate::" + mod + ")", t)
            t = re.sub(r"(#\[cfg\(test\)\]\s*)?#\[global_allocator\]\s*static [^;]*;", "", t)
            open(f, "w", encoding="utf8").write(t)
    # replace extern crate idents everywhere in the pack tree
    for dp, dn, fn in os.walk(os.path.join(dest, P)):
        for f in fn:
            if not f.endswith(".rs"): continue
            p = os.path.join(dp, f)
            t = open(p, encoding="utf8").read(); n = t
            for k in present:
                n = re.sub(r"(?<![\w])(?<!::)" + ident(k) + r"(?=\s*::)", "crate::__" + ident(k), n)
            if n != t: open(p, "w", encoding="utf8").write(n)
    # mount crates in the glue
    g = open(glue, encoding="utf8").read()
    gdir = os.path.dirname(glue)
    for k in present:
        rel = os.path.relpath(os.path.join(dest, libroot(k)), gdir)
        g += f'\n#[path = "{rel}"]\npub mod __{ident(k)};\n'
    for tag, name in (("🔬️record-laws", "__record_laws"), ("🧮️record-schema-laws", "__record_schema_laws")):
        wrapper = os.path.join(dest, P, "🧪️tests/" + tag + "/🦀️.rs")
        if os.path.exists(wrapper):
            w = open(wrapper, encoding="utf8").read()
            w = re.sub(r"(#\[cfg\(test\)\]\s*)?#\[global_allocator\]\s*static [^;]*;", "", w)
            open(wrapper, "w", encoding="utf8").write(w)
            g += '\n#[cfg(test)]\n#[path = "' + os.path.relpath(wrapper, gdir) + '"]\nmod ' + name + ';\n'
    open(glue, "w", encoding="utf8").write(g)
    o.patch_tree(os.path.join(dest, P))
    return present

def parse_sig(counter):
    """Splits an error counter into (external, internal) by whether the message only concerns an external crate."""
    ext = collections.Counter(); inner = collections.Counter()
    EXT = ("`protocol`", "`semio_framework_value`", "`semio_framework_dsl_record`", "`semio_framework_hash`", "`semio_framework_async`", "`semio_framework_diagnostic`", "`semio_framework_pack_error`", "`semio_framework_pack_source`", "`semio_framework_pack_json`", "`semio_framework_io_schema`", "`semio_framework_io_sqlite_snapshot`", "`semio_framework_trace`", "`semio_framework_job`", "serde_json", "`serde`", "blake3", "`semio_framework_dsl_record_derive`", "`semio_framework_dsl`", "`semio_framework_async_macros`", "CARGO_MANIFEST_DIR", "cannot find attribute")
    for (code, m, loc), n in counter.items():
        if code in ("E0433", "E0432", "") and any(x in m for x in EXT) and "crate::__" not in m and "__semio" not in m and not any(("`"+ident(k)+"`") in m for k, *_ in CRATES):
            ext[(code, re.sub(r"`[^`]*`", "`_`", m))] += n
        else:
            inner[(code, m)] += n
    return ext, inner

def run_all(dest, tags=None, extra_cfg=None):
    base = pickle.load(open(SP + "/base-pack.pkl", "rb"))
    lib = os.path.join(dest, P, "📦️packages/🦀️rust/🦀️.rs")
    report = {}
    for tag, extra in o.CFGS.items():
        if tags and tag not in tags: continue
        ex = list(extra)
        if tag != "native-nodeflate": ex += ["--cfg", 'feature="deflate"']
        c, err = o.run(dest, lib, "pack", ex)
        open(f"{SP}/err-S1-{tag}.txt", "w").write(err)
        b = base[tag]
        be, bi = parse_sig(b); ce, ci = parse_sig(c)
        only_split = ci - bi; only_base = bi - ci
        report[tag] = (sum(b.values()), sum(c.values()), only_split, only_base)
        print(f"== {tag}: baseline {sum(b.values())} (internal {sum(bi.values())}) split {sum(c.values())} (internal {sum(ci.values())}); only-split {sum(only_split.values())} only-baseline {sum(only_base.values())}")
        for (code, m), n in sorted(only_split.items())[:80]: print("  +split", n, code, m[:200])
        for (code, m), n in sorted(only_base.items())[:40]: print("  -split", n, code, m[:200])
    return report

if __name__ == "__main__":
    dest = SP + "/S1"
    present = build_sandbox(dest)
    print("crates:", present)
    run_all(dest, tags=sys.argv[1:] or None)
