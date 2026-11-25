"""🚶️ Walks the module tree of lib roots the way rustc does (inline blocks add directory components, `#[path]` files act as mod-rs, the OS resolves `..` through real directories) and reports every `mod x;` whose file does not exist.
usage: walk-mod-paths.py <root.rs>..."""
import os, re, sys
bad = 0
seen = set()
def walk(file, base):
    global bad
    if file in seen: return
    seen.add(file)
    t = open(file, encoding="utf8").read()
    t = re.sub(r"/\*.*?\*/", "", t, flags=re.S)
    t = re.sub(r"//[^\n]*", "", t)
    t = re.sub(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', lambda m: "@@PATH(" + m.group(1) + ")@@", t)
    t = re.sub(r'r#*"(?:.|\n)*?"#*', '""', t)
    t = re.sub(r'"(?:\\.|[^"\\])*"', '""', t)
    t = re.sub(r"'(?:\\.|[^'\\])'", "''", t)
    stack = []
    attr_path = None
    pos = 0
    tok = re.compile(r'@@PATH\(([^)]*)\)@@|\bmod\s+(\w+)\s*([;{])|([{}])')
    depth = []
    cfg_ok = True
    for m in tok.finditer(t):
        if m.group(1) is not None:
            attr_path = m.group(1); continue
        if m.group(2):
            name, kind = m.group(2), m.group(3)
            dirs = [d for d, _ in depth if d is not None]
            cur = os.path.join(base, *dirs)
            if kind == "{":
                comp = attr_path if attr_path is not None else name
                depth.append((comp, "mod"))
            else:
                rel = attr_path if attr_path is not None else None
                if rel is not None:
                    target = os.path.join(cur, rel)
                    ok = os.path.isfile(target)
                else:
                    c1, c2 = os.path.join(cur, name + ".rs"), os.path.join(cur, name, "mod.rs")
                    target = c1 if os.path.isfile(c1) else c2
                    ok = os.path.isfile(target)
                print(("ok      " if ok else "MISSING ") + os.path.relpath(file)[-60:] + " -> " + (rel or name)[:80])
                if not ok: bad += 1
                elif rel is not None: walk(target, os.path.dirname(target))
                else: walk(target, os.path.dirname(target))
            attr_path = None
        elif m.group(4) == "{":
            if not (depth and depth[-1][1] == "pending"): depth.append((None, "blk"))
        elif m.group(4) == "}":
            if depth: depth.pop()
    return
for root in sys.argv[1:]:
    walk(os.path.abspath(root), os.path.dirname(os.path.abspath(root)))
print("MISSING total", bad)
