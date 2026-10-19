"""Import the carve.py helper namespace without running main. usage: from lib import *"""
import sys, os, re, json
RT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
_src = open(RT + "/rg-rest-tools/carve.py", encoding="utf8").read().replace("\nmain()\n", "\n")
_g = {"__name__": "carvelib", "__file__": RT + "/rg-rest-tools/carve.py"}
exec(compile(_src, "carve.py", "exec"), _g)
globals().update({k: v for k, v in _g.items() if not k.startswith("__")})

def strip_targets(mt):
    mt = re.sub(r'(?ms)^\[\[(?:test|bench|example|bin)\]\]\n.*?(?=^\[|\Z)', '', mt)
    mt = re.sub(r'(?ms)^\[features\]\n.*?(?=^\[|\Z)', '', mt)
    return mt.rstrip("\n") + "\n"

def register_crate(ws, old_name, old_pkg, new_name, new_dir, desc, lib_rel, script=None, scope=None, text_for_prune=None):
    """create <new_dir>/📦️packages/🦀️rust/Cargo.toml from old manifest, register in workspace, prune, owners sync."""
    npkg = new_dir + "/📦️packages/🦀️rust"
    old_manifest = rd(old_pkg + "/Cargo.toml")
    mt = rebase_manifest2(old_manifest, old_pkg, npkg)
    mt = manifest_for(mt, new_name, desc, lib_rel)
    mt = strip_targets(mt)
    wr(npkg + "/Cargo.toml", mt)
    wsdir = os.path.dirname(ws)
    rel = lambda p: p[len(wsdir) + 1:] if wsdir else p
    ws_add_member2(ws, rel(npkg))
    ws_add_dep(ws, new_name, rel(npkg), after=old_name)
    if script:
        script_add2(script, old_name, new_name)
    txt = closure_text(rgp.closure(npkg + "/Cargo.toml"))
    cur = rd(npkg + "/Cargo.toml")
    pr = prune_new(cur, txt)
    if pr != cur:
        wr(npkg + "/Cargo.toml", pr)
    return npkg

def add_dep_like(consumer_manifest, old_name, new_name, new_pkg):
    """add new dep to consumer manifest, same style as its dep on old_name."""
    mtx = rd(consumer_manifest)
    for sec in ("dependencies", "dev-dependencies"):
        b = sec_bounds(mtx, sec)
        if not b: continue
        body = mtx[b[0]:b[1]]
        mline = re.search(r'(?m)^' + re.escape(old_name) + r'\s*(?:=|\.workspace)[^\n]*', body)
        if not mline: continue
        line = mline.group(0)
        if "workspace" in line:
            nl = "%s = { workspace = true }" % new_name
        else:
            nl = '%s = { path = "%s" }' % (new_name, os.path.relpath(os.path.join(ROOT, new_pkg), os.path.join(ROOT, os.path.dirname(consumer_manifest))))
        add_dep_line(consumer_manifest, sec, new_name, nl)
        mtx = rd(consumer_manifest)


def ws_add_dep_inline(ws, name, path_rel, after):
    t = rd(ws)
    if re.search(r'(?m)^' + re.escape(name) + r'\s*=', t):
        return False
    m = re.search(r'(?m)^' + re.escape(after) + r'\s*=\s*\{[^\n]*\}\n', t)
    line = '%s = { path = "%s" }\n' % (name, path_rel)
    if m:
        wr(ws, t[:m.end()] + line + t[m.end():])
    else:
        raise SystemExit("no anchor dep line for " + after)
    return True
