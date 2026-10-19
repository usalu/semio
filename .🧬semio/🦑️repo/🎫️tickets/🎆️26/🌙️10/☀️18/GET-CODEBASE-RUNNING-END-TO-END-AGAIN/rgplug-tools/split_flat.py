"""Split a flat mutation-family crate into two crates (new one mounts leaf dirs of the old domain via ../<old-domain>/)."""
import sys, json, re, subprocess, os
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *

def kebab(s): return s.replace("_", "-")

def run(spec, apply=True):
    ws = spec["ws"]; wsdir = os.path.dirname(ws)
    old_dir, new_dir = spec["old_dir"], spec["new_dir"]
    old_pkg, new_pkg = old_dir + "/📦️packages/🦀️rust", new_dir + "/📦️packages/🦀️rust"
    old_base = os.path.basename(old_dir)
    src = rd(old_dir + "/🦀️.rs")
    move = set(spec["move"])
    pre, old_body, new_body, syms = split_flat_root(src, move)
    assert new_body, "nothing moved"
    allmods = [mod_name(i) for i in top_items(src) if mod_name(i) and '#[path = "."]' in i]
    missing = move - set(allmods)
    assert not missing, missing
    first_doc = pre.split("\n")[0]
    m = re.match(r'//! (\S+) (.*?`[^`]+`): (.*)\.$', first_doc)
    emoji_old = first_doc.split(" ")[1]
    listed = re.findall(r'`([a-z0-9\-]+)`', first_doc.split(":", 1)[1])
    moved_kebab = {kebab(x) for x in move}
    old_listed = [x for x in listed if x.replace("-", "_") not in move and not any(x.replace("-", "_") == mm for mm in move)]
    def doc_for(name_slug, emoji, names):
        return "//! %s Energy model mutation domain `%s`: %s.\n" % (emoji, name_slug, ", ".join("`%s`" % n for n in names))
    # mutation names of modules (module snake -> kebab)
    new_names = [kebab(x) for x in allmods if x in move]
    old_names = [kebab(x) for x in allmods if x not in move]
    old_doc = doc_for(old_base[len(emoji_old) + 1:] if False else spec["old_slug"], emoji_old, old_names)
    new_doc = doc_for(spec["new_slug"], spec["new_emoji"], new_names)
    pre_rest = pre.split("\n", 1)[1]
    old_root = old_doc + pre_rest + old_body
    new_root = new_doc + pre_rest + prefix_paths(new_body, "../" + old_base + "/")
    old_manifest_text = rd(old_pkg + "/Cargo.toml")
    if not apply:
        print(new_root[:1500]); print(old_root[:800]); return
    wr(new_dir + "/🦀️.rs", new_root)
    wr(old_dir + "/🦀️.rs", old_root)
    desc_new = spec["new_desc"]
    wr(new_pkg + "/Cargo.toml", manifest_for(old_manifest_text, spec["new_name"], desc_new, "../../🦀️.rs"))
    # workspace
    rel_member = lambda p: p[len(wsdir) + 1:]
    ws_add_member(ws, rel_member(new_pkg))
    ws_add_dep(ws, spec["new_name"], rel_member(new_pkg), after=spec["old_name"])
    # prune manifests by closure usage
    for pkg, name in ((new_pkg, spec["new_name"]), (old_pkg, spec["old_name"])):
        files = closure(pkg + "/Cargo.toml")
        txt = closure_text(files)
        mt = rd(pkg + "/Cargo.toml")
        pr = prune_manifest(mt, txt)
        if pr != mt:
            wr(pkg + "/Cargo.toml", pr)
    # scripts
    if spec.get("script"):
        script_add(spec["script"], spec["old_name"], spec["new_name"])
    # importers
    old_ident, new_ident = cargo_ident(spec["old_name"]), cargo_ident(spec["new_name"])
    subtree = spec["subtree"]
    out = subprocess.run(["git", "grep", "-l", "-e", old_ident, "-e", spec["old_alias"], "--", subtree], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    changed = []
    for f in [x for x in out if x.endswith(".rs")]:
        t = rd(f)
        aliases = set(re.findall(r'extern crate ' + old_ident + r' as (\w+);', t))
        aliases.add(spec["old_alias"]) if re.search(r'\b' + spec["old_alias"] + r'::', t) else None
        if re.search(r'(?<![A-Za-z0-9_])' + old_ident + r'::', t):
            aliases.add(old_ident)
        nt = t
        for al in aliases:
            nt, cnt, flags = repoint_text(nt, al, spec["new_alias"], syms, old_ident, new_ident)
            if flags: print("FLAG", f[-60:], al, flags)
        if nt != t:
            wr(f, nt); changed.append(f)
    print("repointed files:", len(changed))
    for f in changed: print("  ", f[-90:])
    fix_importers(spec, old_ident, new_ident, ws, old_pkg, new_pkg)
    print(run_owners(spec["subtree"], True))
    return syms

def fix_importers(spec, old_ident, new_ident, ws, old_pkg, new_pkg):
    subtree = spec["subtree"]
    mfs = subprocess.run(["git", "grep", "-l", "-e", spec["old_name"], "--", subtree], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    for mf in [x for x in mfs if x.endswith("Cargo.toml") and x != ws and x != old_pkg + "/Cargo.toml" and x != new_pkg + "/Cargo.toml"]:
        files = closure(mf)
        uses_new = False
        for f in files:
            if not f["p"].endswith(".rs"):
                continue
            t = rd(f["p"])
            body = re.sub(r'(?m)^\s*extern crate [^\n]*\n', '', t)
            if re.search(r'(?<![A-Za-z0-9_])' + spec["new_alias"] + r'::', body):
                uses_new = True
        if uses_new:
            have = any(("extern crate " + new_ident + " as ") in rd(f["p"]) for f in files if f["p"].endswith(".rs"))
            if not have:
                for f in files:
                    if not f["p"].endswith(".rs"):
                        continue
                    t = rd(f["p"])
                    mo = re.search(r'(?m)^([ \t]*)extern crate ' + old_ident + r' as (\w+);\n', t)
                    if mo:
                        wr(f["p"], t[:mo.end()] + "%sextern crate %s as %s;\n" % (mo.group(1), new_ident, spec["new_alias"]) + t[mo.end():])
                        break
                else:
                    print("NO EXTERN SITE for", mf)
        files = closure(mf)
        txt = closure_text(files)
        body = re.sub(r'(?m)^\s*extern crate [^\n]*\n', '', txt)
        uses_new = re.search(r'(?<![A-Za-z0-9_])' + spec["new_alias"] + r'::', body) is not None
        uses_old = re.search(r'(?<![A-Za-z0-9_])(' + old_ident + r'|' + spec["old_alias"] + r')::', body) is not None
        mt = rd(mf)
        for sec in ("dependencies", "dev-dependencies"):
            b = sec_bounds(mt, sec)
            if b and dep_key_re(spec["old_name"]).search(mt[b[0]:b[1]]) and uses_new:
                add_dep_line(mf, sec, spec["new_name"], "%s = { workspace = true }" % spec["new_name"])
        print("manifest", mf[-70:], "new:", uses_new, "old:", uses_old)
        if not uses_old:
            for sec in ("dependencies", "dev-dependencies"):
                remove_dep_line(mf, sec, spec["old_name"])
            for f in files:
                if f["p"].endswith(".rs"):
                    t = rd(f["p"])
                    nt = re.sub(r'(?m)^[ \t]*extern crate ' + old_ident + r' as \w+;\n', '', t)
                    if nt != t:
                        wr(f["p"], nt)

if __name__ == "__main__":
    spec = json.load(open(sys.argv[1]))
    if len(sys.argv) > 2 and sys.argv[2] == "fix":
        wsd = spec["ws"]; fix_importers(spec, cargo_ident(spec["old_name"]), cargo_ident(spec["new_name"]), wsd, spec["old_dir"] + "/📦️packages/🦀️rust", spec["new_dir"] + "/📦️packages/🦀️rust"); sys.exit(0)
    run(spec, apply=(len(sys.argv) < 3 or sys.argv[2] != "dry"))
