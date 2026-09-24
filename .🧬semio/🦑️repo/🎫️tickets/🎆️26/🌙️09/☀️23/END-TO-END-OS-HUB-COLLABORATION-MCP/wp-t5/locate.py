"""For each mutation manifest: the crate and Rust path of the production aggregate whose DESCRIPTORS the bridge reports."""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
rows = json.load(open(root + ".tmp-ticket/wp-t5/generated/manifests.json"))

def crate_of(path):
    d = os.path.dirname(path)
    while d.startswith(root):
        m = d + "/📦️packages/🦀️rust/Cargo.toml"
        if os.path.exists(m):
            t = open(m, encoding="utf-8").read()
            lib = re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', t)
            return d, re.search(r'(?m)^name\s*=\s*"([^"]+)"', t).group(1), os.path.normpath(os.path.join(d, "📦️packages/🦀️rust", lib.group(1) if lib else "src/lib.rs"))
        d = os.path.dirname(d)
    return None

def module_path(lib, target, seen=None):
    """Walks the crate's module tree (inline `mod x {` blocks and `#[path]` file mounts, recursively) to the module that includes `target`."""
    target = os.path.normpath(target)
    def walk(path, prefix, depth):
        if depth > 12 or not os.path.exists(path): return None
        base = os.path.dirname(path)
        stack = [(prefix, base)]
        pending = None
        for line in open(path, encoding="utf-8").read().splitlines():
            s = line.strip()
            if s.startswith("//"): continue
            p = re.match(r'#\[path\s*=\s*"([^"]+)"\]', s)
            if p: pending = p.group(1); continue
            m = re.match(r"(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(;|\{)", s)
            if m:
                cur_prefix, cur_base = stack[-1]
                name = m.group(1)
                if m.group(2) == "{":
                    nb = os.path.normpath(os.path.join(cur_base, pending)) if pending else os.path.join(cur_base, name)
                    stack.append((cur_prefix + ([] if name == "component" else [name]), nb))
                else:
                    child = os.path.normpath(os.path.join(cur_base, pending)) if pending else None
                    mp = cur_prefix + ([] if name == "component" else [name])
                    if child == target: return mp
                    if child and child.endswith(".rs") and child != path:
                        found = walk(child, mp, depth + 1)
                        if found is not None: return found
                pending = None
                continue
            if not s.startswith("#"): pending = None
            closes = s.count("}") - s.count("{")
            if closes > 0 and len(stack) > 1:
                del stack[max(1, len(stack) - closes):]
        return None
    return walk(lib, [], 0)

out, missing = [], []
for r in rows:
    if r["inventory"]: continue
    owner = root + r["owner"]
    cands = [owner + "/🧬️schema/🧬️mutations/🦀️.rs", owner + "/🚪️io/🧬️mutations/🦀️.rs"]
    src = next((c for c in cands if os.path.exists(c)), None)
    if not src: missing.append((r, "no vocabulary source")); continue
    text = open(src, encoding="utf-8").read()
    e = re.search(r"#\[derive\([^)]*Mutations[^)]*\)\][^\n]*\n(?:\s*#\[[^\n]*\n)*\s*pub enum (\w+)", text)
    reexport = re.search(r"(?m)^pub use (crate::[\w:]+)::\*;", text)
    c = crate_of(src)
    if not c: missing.append((r, "no crate")); continue
    if e:
        mp = module_path(c[2], src)
        if mp is None: missing.append((r, f"module path of {os.path.relpath(src, root)} not found in {os.path.relpath(c[2], root)}")); continue
        path = "::".join(mp + [e.group(1)])
    elif reexport:
        other = re.search(r"pub enum (\w+)", text)
        missing.append((r, f"re-export {reexport.group(1)}")); continue
    else:
        missing.append((r, "no Mutations enum")); continue
    out.append({**r, "crate": c[1], "crateDir": os.path.relpath(c[0], root), "rustPath": path})
json.dump(out, open(root + ".tmp-ticket/wp-t5/generated/bridge-targets.json", "w"), ensure_ascii=False, indent=1)
print(len(out), "located;", len(missing), "missing")
for r, why in missing: print("MISSING", r["artifact"], r["standard"], r["subset"], why[:160])
