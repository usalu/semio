#!/usr/bin/env python3
"""🚚️ Moves named top-level items between two modules of `semio-framework-plugin` and rewrites every path to them.

The items travel with their docs, attributes and every `impl` whose self type is moved; a private item becomes
`pub(crate)`. Afterwards every crate path that named a moved item through its old module (`crate::…`,
`super::…`, `crate::component::…`, any ancestor that re-exported it) names the new module instead: single leaves are
rewritten in place, a leaf inside a grouped `use` is cut out of its group and re-imported on its own line beside it.
The destination receives the source module's imports its new items read, plus imports of the source's own items it
names; the source receives imports of the moved items it still names. Unused leftovers are pruned by
`🔁️fix-loop.py --prune`. It compiles nothing.

Usage: move.py <from file> <to file> <name>[,<name>…|,!<macro called at top level>] [--doc "<//! line>"] [--dry]   (paths relative to the plugin crate)
"""
import importlib.util, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("facet_imports", os.path.join(HERE, "🧭️facet-imports.py"))
fi = importlib.util.module_from_spec(spec); spec.loader.exec_module(fi)
ri = fi.ri
PLUGIN = fi.PLUGIN
SKIP = ("🏇️mounted-owner/", "🖨️describe/", "📇️registry/", "🏗️build/", "🖥️host/", "🌐️browser-bundle/")
IDENT = re.compile(r"\b[A-Za-z_][A-Za-z0-9_]*\b")


def crate_files():
    out = []
    for d, _, fs in os.walk(PLUGIN):
        rel = os.path.relpath(d, PLUGIN)
        if any((rel + "/").startswith(s) for s in SKIP) or "/target" in d: continue
        out += [os.path.join(d, f) for f in fs if f.endswith(".rs")]
    return out


def module_paths():
    fi.walk(f"{PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{PLUGIN}/📦️packages/🦀️rust")
    paths = {}
    for m in fi.modules:
        if m.rng[0] == 0 and m.file not in paths:
            segs = m.path.split("::")
            paths[m.file] = "::".join(s for s in segs[1:] if s != "component")
    return paths


def parse_use(body):
    """🌳️ Leaves of one use tree as (path segments, alias, start, end) with offsets into `body`."""
    leaves = []

    def walk(prefix, a, b):
        depth, start, i = 0, a, a
        parts = []
        while i <= b:
            ch = body[i] if i < b else ","
            if ch == "{": depth += 1
            elif ch == "}": depth -= 1
            elif ch == "," and depth == 0:
                parts.append((start, i)); start = i + 1
            i += 1
        for s, e in parts:
            txt = body[s:e]
            if not txt.strip(): continue
            ls = s + len(txt) - len(txt.lstrip()); le = e - (len(txt) - len(txt.rstrip()))
            t = body[ls:le]
            if "{" in t:
                ob = ls + t.index("{"); cb = le - 1
                pre = [x for x in re.sub(r"\s+", "", body[ls:ob]).split("::") if x]
                walk(prefix + pre, ob + 1, cb)
            else:
                m = re.match(r"(.*?)\s+as\s+(\w+)$", t, re.S)
                path, alias = (m.group(1), m.group(2)) if m else (t, None)
                leaves.append((prefix + [x for x in re.sub(r"\s+", "", path).split("::") if x], alias, ls, le))
    walk([], 0, len(body))
    return leaves


USE_RE = re.compile(r"(?m)^([ \t]*)((?:#\[[^\]]*\][ \t]*)*)((?:pub(?:\([^)]*\))?[ \t]+)?)use[ \t]+")


def statements(src, cl):
    for m in USE_RE.finditer(cl):
        end = cl.index(";", m.end())
        depth = 0
        for k in range(m.end(), len(cl)):
            if cl[k] == "{": depth += 1
            elif cl[k] == "}": depth -= 1
            elif cl[k] == ";" and depth == 0: end = k; break
        yield m, m.end(), end


def cut_leaf(body, ls, le):
    j = le
    while j < len(body) and body[j] in " \t\r\n": j += 1
    if j < len(body) and body[j] == ",":
        return body[:ls] + body[j + 1:].lstrip(" ")
    i = ls
    while i > 0 and body[i - 1] in " \t\r\n": i -= 1
    if i > 0 and body[i - 1] == ",": return body[:i - 1] + body[le:]
    return body[:ls] + body[le:]


def tidy(body):
    for _ in range(4):
        body = re.sub(r"(?:,\s*)?\b\w+(?:::\w+)*::\{\s*\}", "", body)
        body = re.sub(r"\{\s*,\s*", "{", body)
        body = re.sub(r",\s*\}", "}", body)
        body = re.sub(r"(\w+(?:::\w+)*)::\{\s*(\w+)\s*\}", r"\1::\2", body)
    return body.strip()


def rewrite_file(f, old_prefixes, names, dst_path, dry, is_dst=False):
    src = open(f, encoding="utf-8").read()
    cl = ri.clean(src)
    edits = []
    for m, a, e in statements(src, cl):
        body = src[a:e]
        moved = []
        for segs, alias, ls, le in parse_use(body):
            if not segs or segs[-1] not in names: continue
            root = segs[0].lstrip("$")
            if root not in ("crate", "super", "self"): continue
            mid = [s for s in segs[1:-1] if s != "component"]
            if "::".join(mid) not in old_prefixes: continue
            moved.append((segs[-1], alias, ls, le))
        if not moved: continue
        whole = len(parse_use(body)) == len(moved)
        nb = body
        for nm, alias, ls, le in sorted(moved, key=lambda x: -x[2]): nb = cut_leaf(nb, ls, le)
        nb = tidy(nb)
        head = m.group(1) + m.group(2) + m.group(3) + "use "
        new = "" if is_dst and not m.group(3).strip() else "\n".join(f"{head}crate::{dst_path}::{nm}{' as ' + alias if alias else ''};" for nm, alias, _, _ in moved)
        if whole or not nb or nb in ("{}", ""):
            edits.append((m.start(), e + 1, new))
        else:
            edits.append((m.start(), e + 1, src[m.start():a] + nb + ";" + ("\n" + new if new else "")))
    out = src
    for s, e, t in sorted(edits, key=lambda x: -x[0]): out = out[:s] + t + out[e:]
    cl2 = ri.clean(out)
    pat = re.compile(r"\$?\b(?:crate|super)((?:::component)?(?:::\w+)*)::(" + "|".join(map(re.escape, names)) + r")\b")
    reps = []
    for mm in pat.finditer(cl2):
        mid = [s for s in mm.group(1).split("::") if s and s != "component"]
        if "::".join(mid) in old_prefixes:
            reps.append((mm.start(), mm.end(), ("$crate" if out[mm.start()] == "$" else "crate") + f"::{dst_path}::{mm.group(2)}"))
    for s, e, t in sorted(reps, key=lambda x: -x[0]): out = out[:s] + t + out[e:]
    if out != src:
        if dry: print(f"   rewrite {os.path.relpath(f, PLUGIN)}: {len(edits)} use, {len(reps)} path")
        else: open(f, "w", encoding="utf-8").write(out)
        return 1
    return 0


def bound_names(text):
    """🏷️ Names a module text defines at top level or binds through its own `use` items."""
    cl = ri.clean(text)
    its, _ = ri.items(text, cl, 0, len(cl))
    out = {it["name"] for it in its if it["name"] and it["kind"] not in ("use", "impl", "macro_call")}
    for m, a, e in statements(text, cl):
        for segs, alias, _, _ in parse_use(text[a:e]):
            if segs and segs[-1] not in ("*", "self"): out.add(alias or segs[-1])
            if segs and segs[-1] == "self" and len(segs) > 1: out.add(alias or segs[-2])
    return out


def header_insert(text, lines):
    lines = [l for l in lines if l not in text.split("\n")]
    if not lines: return text
    t = text.split("\n")
    i = 0
    while i < len(t) and (t[i].startswith("//!") or t[i].startswith("#![")): i += 1
    return "\n".join(t[:i] + lines + t[i:])


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    dry = "--dry" in sys.argv
    doc = sys.argv[sys.argv.index("--doc") + 1] if "--doc" in sys.argv else None
    if doc in args: args.remove(doc)
    src_f, dst_f, names = os.path.join(PLUGIN, args[0]), os.path.join(PLUGIN, args[1]), args[2].split(",")
    if not os.path.exists(dst_f):
        if not doc: sys.exit(f"[move] {args[1]} does not exist; pass --doc to create it")
        if not dry:
            os.makedirs(os.path.dirname(dst_f), exist_ok=True)
            open(dst_f, "w", encoding="utf-8").write(doc + "\n")
    paths = module_paths()
    src_path = paths.get(src_f)
    dst_path = paths.get(dst_f) or (sys.argv[sys.argv.index("--as") + 1] if "--as" in sys.argv else None)
    if not src_path or not dst_path: sys.exit(f"[move] unmounted module: {src_path} {dst_path}")
    raw = open(src_f, encoding="utf-8").read(); cl = ri.clean(raw)
    its, _ = ri.items(raw, cl, 0, len(cl))
    macros = {n[1:] for n in names if n.startswith("!")}
    names = [n for n in names if not n.startswith("!")]
    want = set(names)
    cut = [it for it in its if (it["name"] in want and it["kind"] not in ("use", "macro_call", "impl")) or (it["kind"] == "impl" and it.get("self") in want) or (it["kind"] == "macro_call" and it["name"] in macros)]
    missing = (want | macros) - {it["name"] for it in cut if it["kind"] != "impl"}
    if missing: sys.exit(f"[move] not found in {args[0]}: {sorted(missing)}")
    moved_text = "\n".join(cl[it["head"]:it["end"]] for it in cut)
    blocks, rest = [], raw
    for it in sorted(cut, key=lambda x: -x["start"]):
        blocks.append(rest[it["start"]:it["end"]].strip("\n"))
        rest = rest[:it["start"]] + rest[it["end"]:]
    blocks.reverse()
    blocks = [re.sub(r"^((?:\s*(?:///|//|#\[).*\n)*)(\s*)(?=(?:pub\(super\)\s+)?(?:const\s+(?!fn)|async\s+|unsafe\s+)?(?:struct|enum|fn|const|static|type|trait|union)\b)(pub\(super\)\s+)?",
                     lambda m: m.group(1) + m.group(2) + "pub(crate) ", b, count=1) if not re.match(r"^(?:\s*(?:///|//|#\[).*\n)*\s*pub\b(?!\(super\))", b) and not b.lstrip().startswith("impl") and not re.match(r"^(?:\s*(?:///|//|#\[).*\n)*\s*impl\b", b) else b for b in blocks]
    src_defs = {it["name"] for it in its if it["name"] and it["kind"] not in ("use", "impl", "macro_call") and it["name"] not in want}
    used_in_moved = set(IDENT.findall(moved_text))
    rest_cl = ri.clean(rest)
    src_imports, dst_imports = [], []
    for m, a, e in statements(raw, cl):
        if m.start() >= len(raw): continue
        if any(it["head"] <= m.start() < it["end"] for it in cut): continue
        stmt = raw[m.start():e + 1]
        if re.search(r"\bpub\b", m.group(3)): continue
        for segs, alias, ls, le in parse_use(raw[a:e]):
            bound = alias or segs[-1]
            if bound in used_in_moved and bound not in ("self", "*"):
                path = "::".join(segs)
                if segs[0] == "super" and len(src_path.split("::")) == 1: path = "crate::" + "::".join(segs[1:])
                if segs[0] == "self": path = f"crate::{src_path}::" + "::".join(segs[1:])
                dst_imports.append(f"{m.group(2).strip() + ' ' if m.group(2).strip() else ''}use {path}{' as ' + alias if alias else ''};")
    for d in sorted(src_defs & used_in_moved):
        dst_imports.append(f"use crate::{src_path}::{d};")
    used_in_rest = set(IDENT.findall(rest_cl))
    for n in names:
        if n in used_in_rest: src_imports.append(f"use crate::{dst_path}::{n};")
    dst_raw = open(dst_f, encoding="utf-8").read() if os.path.exists(dst_f) else ""
    taken = bound_names(dst_raw) | set(names)
    dst_imports = [l for l in dict.fromkeys(dst_imports) if l.rstrip(";").split("::")[-1].split(" as ")[-1] not in taken and not l.split("use ", 1)[1].startswith(f"crate::{dst_path}::")]
    src_taken = bound_names(rest)
    src_imports = [l for l in src_imports if l.rstrip(";").split("::")[-1] not in src_taken]
    print(f"[move] {len(cut)} items {src_path} -> {dst_path}; +{len(dst_imports)} imports in destination, +{len(src_imports)} in source")
    if dry:
        for b in blocks: print("   item:", b.split("\n")[-1][:100] if b.split("\n") else b)
    else:
        open(src_f, "w", encoding="utf-8").write(header_insert(re.sub(r"\n{3,}", "\n\n", rest), src_imports))
        d = open(dst_f, encoding="utf-8").read().rstrip("\n")
        d = header_insert(d + "\n\n" + "\n\n".join(blocks) + "\n", dst_imports)
        open(dst_f, "w", encoding="utf-8").write(d)
    prefixes = {src_path}
    segs = src_path.split("::")
    for k in range(1, len(segs)): prefixes.add("::".join(segs[:k]))
    n = sum(rewrite_file(f, prefixes, set(names), dst_path, dry, f == dst_f) for f in crate_files() if f not in (src_f,))
    print(f"[move] rewrote {n} files")


if __name__ == "__main__":
    main()
