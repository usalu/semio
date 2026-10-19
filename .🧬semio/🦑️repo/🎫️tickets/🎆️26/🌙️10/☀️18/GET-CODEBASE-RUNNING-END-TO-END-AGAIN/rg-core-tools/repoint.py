import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl, deps
R = rgl.ROOT

def split_use(text, old_lib, new_lib, moved):
    """rewrite `use old_lib::{...}` / `use old_lib::Name;` statements: moved names go to a new `use new_lib::{...}` statement; also qualified paths."""
    out = text
    pat = re.compile(r'^([ \t]*)((?:pub )?)use\s+' + re.escape(old_lib) + r'::(\{[^;]*?\}|[A-Za-z_][A-Za-z0-9_:]*)\s*;', re.M | re.S)
    def fix(m):
        indent, vis, body = m.group(1), m.group(2), m.group(3)
        if body.startswith("{"):
            items = [i.strip() for i in re.split(r',(?![^{]*\})', body[1:-1]) if i.strip()]
            keep, move = [], []
            for i in items:
                base = re.split(r'\s+as\s+', i)[0].strip()
                head = base.split("::")[0]
                (move if head in moved and "::" not in base or base in moved else keep).append(i)
            if not move:
                return m.group(0)
            res = []
            if keep:
                res.append(f"{indent}{vis}use {old_lib}::{{{', '.join(keep)}}};")
            res.append(f"{indent}{vis}use {new_lib}::{{{', '.join(move)}}};")
            return "\n".join(res)
        else:
            head = re.split(r'\s+as\s+', body)[0].strip().split("::")[0]
            if head in moved:
                return f"{indent}{vis}use {new_lib}::{body};"
            return m.group(0)
    out = pat.sub(fix, out)
    q = re.compile(re.escape(old_lib) + r'::(' + "|".join(map(re.escape, sorted(moved, key=len, reverse=True))) + r')\b')
    out = q.sub(lambda m: f"{new_lib}::{m.group(1)}", out)
    return out

def run(files, old_lib, new_lib, moved, anchor, new_name, new_dir, owner_override=None):
    for f in files:
        p = R + "/" + f
        t = rgl.read(p)
        t2 = split_use(t, old_lib, new_lib, set(moved))
        if t2 != t:
            rgl.write(p, t2)
        if new_lib in t2:
            m = (owner_override or {}).get(f) or deps.manifest_with_anchor(p, anchor)
            if m is None:
                print("NO-OWNER", f[-70:]); continue
            print(os.path.relpath(m, R)[-70:], deps.add_dep(m, anchor, new_name, new_dir))
        else:
            print("no-change", f[-60:])
