import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"

def rel(frm_dir, to_abs):
    return os.path.relpath(to_abs, frm_dir)

def read(p):
    return open(p, encoding="utf-8").read()

def write(p, s):
    open(p, "w", encoding="utf-8").write(s)

def workspace_files_of(pkg_abs_dir):
    """workspace Cargo.toml files that list pkg dir as member or path dependency (by textual rel path)."""
    out = []
    for ws in ["Cargo.toml", "🧰️framework/Cargo.toml", "🎓️teaching/Cargo.toml", "✏️s/Cargo.toml", "🌎️hub/Cargo.toml"]:
        p = os.path.join(ROOT, ws)
        if os.path.exists(p):
            out.append(p)
    return out

def add_member(ws_file, after_dir_abs, new_dir_abs):
    base = os.path.dirname(ws_file)
    text = read(ws_file)
    a = rel(base, after_dir_abs); n = rel(base, new_dir_abs)
    if f'"{n}"' in text:
        return False
    line_re = re.compile(r'^(\s*)"' + re.escape(a) + r'",\s*$', re.M)
    m = line_re.search(text)
    if not m:
        return None
    ins = f'{m.group(1)}"{n}",\n'
    text = text[:m.end() + 1] + ins + text[m.end() + 1:] if text[m.end():m.end() + 1] == "\n" else text[:m.end()] + "\n" + ins + text[m.end():]
    write(ws_file, text)
    return True

def add_ws_dep(ws_file, after_name, new_name, new_dir_abs):
    base = os.path.dirname(ws_file)
    text = read(ws_file)
    n = rel(base, new_dir_abs)
    if re.search(r'^\s*(\[workspace\.dependencies\.' + re.escape(new_name) + r'\]|' + re.escape(new_name) + r'\s*=)', text, re.M):
        return False
    m = re.search(r'^(\s*)\[workspace\.dependencies\.' + re.escape(after_name) + r'\]\n(\s*)path\s*=\s*"[^"]*"\n', text, re.M)
    if m:
        block = f'\n{m.group(1)}[workspace.dependencies.{new_name}]\n{m.group(2)}path = "{n}"\n'
        text = text[:m.end()] + block + text[m.end():]
        write(ws_file, text)
        return True
    m = re.search(r'^' + re.escape(after_name) + r'\s*=\s*\{[^\n]*\}\n', text, re.M)
    if m:
        text = text[:m.end()] + f'{new_name} = {{ path = "{n}" }}\n' + text[m.end():]
        write(ws_file, text)
        return True
    return None

def add_owner(after_name, name, root_rel):
    p = os.path.join(ROOT, "📋️build-owners.json")
    text = read(p)
    if f'"name": "{name}"' in text:
        return False
    rows = list(re.finditer(r'    \{\n      "name": "([^"]+)",\n(?:      [^\n]*\n)*?    \},?\n', text))
    pos = None
    for m in rows:
        if m.group(1) > name:
            pos = m.start(); break
    row = f'    {{\n      "name": "{name}",\n      "root": "{root_rel}",\n      "manifest": "{root_rel}/📋️project.json",\n      "packages": []\n    }},\n'
    if pos is None:
        last = rows[-1]
        blk = last.group(0)
        text = text[:last.start()] + blk.rstrip("\n").rstrip(",") + ",\n" + row.rstrip(",\n") + "\n" + text[last.end():]
    else:
        text = text[:pos] + row + text[pos:]
    json.loads(text)
    write(p, text)
    return True

def project_json(name, scope="framework"):
    return json.dumps({"name": name, "tags": [f"scope:{scope}", "language:rust"]}, ensure_ascii=False, indent=2) + "\n"

import subprocess
def all_workspaces():
    out = subprocess.check_output(["git", "-C", ROOT, "ls-files", "-z", "--", "*Cargo.toml", ":!.🧬semio"]).decode().split("\0")
    res = []
    for f in out:
        if not f:
            continue
        p = os.path.join(ROOT, f)
        try:
            t = read(p)
        except OSError:
            continue
        if re.search(r'^\[workspace\]', t, re.M):
            res.append(p)
    return res

def register(old_dir, old_name, new_dir, new_name, old_owner, new_owner, new_root_rel, dep_ws=None, verbose=True):
    done = []
    members = []
    for ws in all_workspaces():
        base = os.path.dirname(ws)
        text = read(ws)
        a = rel(base, old_dir)
        if f'"{a}",' in text and re.search(r'^\s*"' + re.escape(a) + r'",\s*$', text, re.M):
            members.append(ws)
            done.append((ws, "member", add_member(ws, old_dir, new_dir)))
    for ws in (dep_ws if dep_ws is not None else members):
        done.append((ws, "dep", add_ws_dep(ws, old_name, new_name, new_dir)))
    done.append(("owners", "row", add_owner(old_owner, new_owner, new_root_rel)))
    if verbose:
        for d in done:
            print(os.path.relpath(d[0], ROOT) if d[0] != "owners" else d[0], d[1], d[2])
    return done
