"""Split a nested-wrapper mutation crate (standards::v1::subsets::any::schema::mutations::<leaf mods>) into several new crates; the old crate keeps the remainder."""
import sys, json, re, subprocess, os
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *

CHAIN = "standards::v1::subsets::any::schema::mutations"

def run(spec):
    ws = spec["ws"]; wsdir = os.path.dirname(ws)
    old_pkg = spec["old_pkg"]
    old_root = spec["old_root"]
    src = rd(old_root)
    head, tail, keep, moved = split_nested_root(src, "mutations", set(sum([n["move"] for n in spec["new"]], [])))
    all_moved = set(sum([n["move"] for n in spec["new"]], []))
    assert set(moved) == all_moved, (all_moved - set(moved))
    old_manifest = rd(old_pkg + "/Cargo.toml")
    first = head.split("\n", 1)
    for n in spec["new"]:
        ndir = n["dir"]
        npkg = ndir + "/📦️packages/🦀️rust"
        body = "".join(moved[m] for m in n["move"])
        body = body.replace('"../../', '"../')
        ch = re.sub(r'^//! [^\n]*\n', "//! %s %s\n" % (n["emoji"], n["doc"]), head, count=1)
        wr(ndir + "/🦀️.rs", ch + body + tail)
        mt = rebase_manifest(old_manifest, 1)
        mt = manifest_for(mt, n["name"], n["desc"], "../../🦀️.rs")
        wr(npkg + "/Cargo.toml", mt)
        rel = lambda p: p[len(wsdir) + 1:]
        ws_add_member(ws, rel(npkg))
        ws_add_dep(ws, n["name"], rel(npkg), after=spec["old_name"])
        if spec.get("script"):
            script_add(spec["script"], spec["old_name"], n["name"])
    new_head = re.sub(r'^//! [^\n]*\n', "//! %s %s\n" % (spec["old_emoji"], spec["old_doc"]), head, count=1)
    wr(old_root, new_head + keep + tail)
    od = spec["old_desc"]
    wr(old_pkg + "/Cargo.toml", manifest_for(old_manifest, spec["old_name"], od, rd(old_pkg + "/Cargo.toml") and re.search(r'(?m)^\[lib\]\s*\npath = "([^"]*)"', old_manifest).group(1)))
    for pkg in [old_pkg] + [n["dir"] + "/📦️packages/🦀️rust" for n in spec["new"]]:
        files = closure(pkg + "/Cargo.toml")
        txt = closure_text(files)
        mt = rd(pkg + "/Cargo.toml")
        pr = prune_manifest(mt, txt)
        if pr != mt:
            wr(pkg + "/Cargo.toml", pr)
    old_ident = cargo_ident(spec["old_name"])
    subtree = spec["subtree"]
    out = subprocess.run(["git", "grep", "-l", "-e", old_ident, "--", subtree], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    changed = []
    for f in [x for x in out if x.endswith(".rs")]:
        t = rd(f)
        nt = t
        for n in spec["new"]:
            nt, cnt, flags = repoint_text(nt, old_ident, cargo_ident(n["name"]), n["move"], old_ident, cargo_ident(n["name"]), CHAIN)
            if flags: print("FLAG", f[-60:], flags)
        if nt != t:
            wr(f, nt); changed.append(f)
    print("repointed files:", len(changed))
    fix_manifests(spec, old_ident)
    print(run_owners(subtree, True))

def fix_manifests(spec, old_ident):
    ws = spec["ws"]
    subtree = spec["subtree"]
    mfs = subprocess.run(["git", "grep", "-l", "-e", spec["old_name"], "--", subtree], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    skip = {ws, spec["old_pkg"] + "/Cargo.toml"} | {n["dir"] + "/📦️packages/🦀️rust/Cargo.toml" for n in spec["new"]}
    for mf in [x for x in mfs if x.endswith("Cargo.toml") and x not in skip]:
        files = closure(mf)
        txt = closure_text(files)
        mt = rd(mf)
        for n in spec["new"]:
            ni = cargo_ident(n["name"])
            if re.search(r'\b' + ni + r'\b', txt):
                for sec in ("dependencies", "dev-dependencies"):
                    b = sec_bounds(mt, sec)
                    if b and dep_key_re(spec["old_name"]).search(mt[b[0]:b[1]]):
                        add_dep_line(mf, sec, n["name"], "%s = { workspace = true }" % n["name"])
                        mt = rd(mf)
        uses_old = re.search(r'\b' + old_ident + r'\b', txt) is not None
        print("manifest", mf[-70:], "old used:", uses_old)
        if not uses_old:
            for sec in ("dependencies", "dev-dependencies"):
                remove_dep_line(mf, sec, spec["old_name"])

if __name__ == "__main__":
    spec = json.load(open(sys.argv[1]))
    if len(sys.argv) > 2 and sys.argv[2] == "fix":
        fix_manifests(spec, cargo_ident(spec["old_name"])); sys.exit(0)
    run(spec)
