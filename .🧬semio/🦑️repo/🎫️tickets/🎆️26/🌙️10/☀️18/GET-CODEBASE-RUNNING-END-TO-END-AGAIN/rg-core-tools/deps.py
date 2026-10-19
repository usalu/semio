import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl
R = rgl.ROOT
_cache = {}

import json, subprocess
_closures = {}
def candidates(file_abs):
    d = os.path.dirname(file_abs)
    res = []
    while d.startswith(R) and d != R:
        for m in (os.path.join(d, "Cargo.toml"), os.path.join(d, "📦️packages", "🦀️rust", "Cargo.toml")):
            if os.path.exists(m) and re.search(r'^\[package\]', rgl.read(m), re.M):
                res.append(m)
        d = os.path.dirname(d)
    return res

def closures(manifests):
    need = [os.path.relpath(os.path.dirname(m), R) for m in manifests if m not in _closures]
    if need:
        out = subprocess.check_output(["bun", os.path.join(os.path.dirname(__file__), "owners.ts")], input=json.dumps(need).encode(), cwd=R)
        data = json.loads(out)
        for k, v in data.items():
            _closures[os.path.join(R, k, "Cargo.toml")] = v
    return {m: _closures[m] for m in manifests}

def manifest_of(file_abs):
    c = candidates(file_abs)
    if not c:
        return None
    rel = os.path.relpath(file_abs, R)
    cl = closures(c)
    for m in c:
        if rel in cl[m]["prod"] or rel in cl[m]["test"]:
            return m
    return None

def workspace_of(manifest):
    t = rgl.read(manifest)
    m = re.search(r'^\[package\][^\[]*?^workspace\s*=\s*"([^"]+)"', t, re.M | re.S)
    base = os.path.dirname(manifest)
    if m:
        return os.path.normpath(os.path.join(base, m.group(1), "Cargo.toml"))
    d = os.path.dirname(base)
    while d.startswith(R):
        p = os.path.join(d, "Cargo.toml")
        if os.path.exists(p) and re.search(r'^\[workspace\]', rgl.read(p), re.M):
            return p
        d = os.path.dirname(d)
    return None

SECTION = re.compile(r'^\[([^\]]+)\]\s*$', re.M)

def sections(text):
    marks = [(m.start(), m.end(), m.group(1)) for m in SECTION.finditer(text)]
    out = []
    for i, (s, e, name) in enumerate(marks):
        end = marks[i + 1][0] if i + 1 < len(marks) else len(text)
        out.append((name, s, e, end))
    return out

def has_dep(text, name):
    return re.search(r'^' + re.escape(name) + r'\s*(=|\.)', text, re.M) is not None or re.search(r'^\[(?:[^\]]*\.)?dependencies\.' + re.escape(name) + r'\]', text, re.M) is not None

def add_dep(manifest, anchor, new_name, new_dir_abs, want_section=None):
    """insert new_name next to anchor's line in the same section; return status"""
    text = rgl.read(manifest)
    if has_dep(text, new_name):
        return "present"
    base = os.path.dirname(manifest)
    secs = sections(text)
    cands = []
    for name, s, e, end in secs:
        if not re.search(r'(^|\.)(dev-|build-)?dependencies$', name) or name.startswith("workspace"):
            continue
        body = text[e:end]
        m = re.search(r'^' + re.escape(anchor) + r'\s*=\s*(.*)$', body, re.M)
        if m:
            cands.append((name, s, e, end, m))
    if not cands:
        return "no-anchor"
    pick = None
    if want_section:
        pick = next((c for c in cands if c[0] == want_section), None)
    if pick is None:
        pick = next((c for c in cands if c[0] == "dependencies"), cands[0])
    name, s, e, end, m = pick
    val = m.group(1)
    if "workspace = true" in val or "workspace=true" in val:
        line = f"{new_name} = {{ workspace = true }}"
        ws = workspace_of(manifest)
        ensure_ws_dep(ws, anchor, new_name, new_dir_abs)
    else:
        rel = os.path.relpath(new_dir_abs, base)
        pm = re.search(r'path\s*=\s*"([^"]+)"', val)
        if not pm:
            return "anchor-not-path"
        line = f'{new_name} = {{ path = "{rel}" }}'
    pos = e + m.end()
    text = text[:pos] + "\n" + line + text[pos:]
    rgl.write(manifest, text)
    return "added:" + name

def ensure_ws_dep(ws, anchor, new_name, new_dir_abs):
    t = rgl.read(ws)
    if re.search(r'^\s*(\[workspace\.dependencies\.' + re.escape(new_name) + r'\]|' + re.escape(new_name) + r'\s*=)', t, re.M):
        return False
    return rgl.add_ws_dep(ws, anchor, new_name, new_dir_abs)

def repoint(file_abs, replacements, anchor, deps):
    """replacements: list of (old, new); deps: list of (new_crate_name, new_dir_abs, lib_marker) added when marker occurs in the file"""
    t = rgl.read(file_abs)
    t2 = t
    for old, new in replacements:
        t2 = t2.replace(old, new)
    if t2 != t:
        rgl.write(file_abs, t2)
    m = manifest_of(file_abs)
    if m is None:
        return ("NO-OWNER", os.path.relpath(file_abs, R)[-60:])
    res = []
    for nm, d, marker in deps:
        if marker in t2:
            res.append(add_dep(m, anchor, nm, d))
    return (os.path.relpath(m, R)[-70:], res)

def manifest_with_anchor(file_abs, anchor):
    for m in candidates(file_abs):
        t = rgl.read(m)
        if has_dep(t, anchor):
            return m
    return None
