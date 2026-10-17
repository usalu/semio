"""🧭 c4p25 structural checks. usage: c4pmore.py mounts|api|fmt stage|live
mounts: every `#[path]` / `include!` target of the lib-root closure exists, each new file is reached exactly once, no new duplicate mounts (nested `#[path = "."]` inline modules are followed).
api: the exact-`pub` and `pub(crate)` top-level item names of the pre-edit root versus root plus children, and that every child defining a `pub` item is re-exported with `pub use <mod>::*;`.
fmt: `rustfmt --check` hunk count (pinned nightly, repo rustfmt.toml, skip_children) of the pre-edit blobs versus the root plus children."""
import collections, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER
from rslex import outline

cmd, mode = sys.argv[1], sys.argv[2]
BASE = ROOT if mode == "live" else STAGE


def mask_keep_strings(text):
    t = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), text)
    return re.sub(r"/\*.*?\*/", lambda m: re.sub(r"[^\n]", " ", m.group(0)), t, flags=re.S)


def path_mounts(txt, base):
    out, stack, pending = [], [], None
    for line in mask_keep_strings(txt).split("\n"):
        st = line.strip()
        m = re.match(r'^#\[\s*path\s*=\s*"([^"]+)"\s*\]$', st)
        cur = stack[-1] if stack else base
        if m:
            if m.group(1) in (".", ".."):
                pending = os.path.normpath(os.path.join(cur, m.group(1)))
            else:
                out.append(os.path.normpath(os.path.join(cur, m.group(1))))
            continue
        if re.match(r"^(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{\s*$", st) and line == line.lstrip():
            stack.append(pending or cur)
            pending = None
            continue
        if line == "}" and stack:
            stack.pop()
            continue
        if st.startswith(("mod ", "pub mod ", "pub(crate) mod ")):
            pending = None
        for m2 in re.finditer(r'\binclude!\(\s*"([^"]+)"\s*\)', line):
            out.append(os.path.normpath(os.path.join(cur, m2.group(1))))
    return out


def closure(root_abs, reader):
    counts, missing, seen, order = collections.Counter(), [], set(), []

    def visit(p):
        if p in seen:
            return
        seen.add(p)
        order.append(p)
        txt = reader(p)
        if txt is None:
            return
        for target in path_mounts(txt, os.path.dirname(p)):
            counts[target] += 1
            if reader(target) is None and not os.path.isdir(target):
                missing.append((os.path.relpath(p, ROOT)[-70:], os.path.relpath(target, ROOT)[-70:]))
            elif reader(target) is not None:
                visit(target)
    visit(root_abs)
    return order, counts, missing


def pubs(text):
    out = set()
    for it in outline(text):
        if it["vis"] in ("pub", "pub(crate)") and it["kind"] in ("fn", "struct", "enum", "trait", "const", "static", "type", "union", "mod", "macro_rules!") and it["name"]:
            out.add((it["vis"], it["kind"], it["name"]))
    return out


bad = 0
tb = ta = 0
for key in ORDER:
    spec = PLANS[key]
    root_abs = ROOT + "/" + spec["rel"]
    base_dir = os.path.dirname(spec["rel"])
    files = [spec["rel"]] + [f"{base_dir}/{c['folder']}/🦀️.rs" for c in spec["children"]]
    if cmd == "mounts":
        def now(p):
            rel = os.path.relpath(p, ROOT)
            for cand in ([os.path.join(BASE, rel)] if mode == "stage" else []) + [p]:
                if os.path.isfile(cand):
                    return open(cand, encoding="utf-8").read()
            return None

        def before(p):
            if p == root_abs:
                return base_text(spec["rel"])
            return open(p, encoding="utf-8").read() if os.path.isfile(p) and os.path.relpath(p, ROOT) not in files else None
        lib = ROOT + "/" + spec["lib"]
        o_b, c_b, m_b = closure(lib, before)
        o_a, c_a, m_a = closure(lib, now)
        dup_b = {k for k, v in c_b.items() if v > 1}
        dup_a = {k for k, v in c_a.items() if v > 1}
        reach = {f: c_a.get(os.path.normpath(ROOT + "/" + f), 0) for f in files[1:]}
        ok = not m_a and not (dup_a - dup_b) and all(v == 1 for v in reach.values())
        bad += 0 if ok else 1
        print(f"[{key}] closure files before={len(o_b)} after={len(o_a)} missing targets before={len(m_b)} after={len(m_a)} duplicate mounts before={len(dup_b)} after={len(dup_a)} new={len(dup_a - dup_b)} new files reached once each={all(v == 1 for v in reach.values())} {'OK' if ok else 'PROBLEM'}")
        for x in m_a[:5]:
            print("    missing", x)
        for f, v in reach.items():
            if v != 1:
                print("    reached", v, f[-60:])
    elif cmd == "api":
        before = pubs(base_text(spec["rel"]))
        root_text = read(f"{BASE}/{spec['rel']}")
        after = pubs(root_text)
        problems = []
        for c in spec["children"]:
            cp = pubs(read(f"{BASE}/{base_dir}/{c['folder']}/🦀️.rs"))
            after |= cp
            has_pub = any(v == "pub" for v, k, n in cp)
            if spec["mode"] == "mod" and has_pub and not re.search(rf"pub use {c['mod']}::\*;", root_text):
                problems.append(f"{c['mod']}: defines pub items but is not re-exported with pub use")
        ob, oa = before - after, after - before
        ok = not ob and not oa and not problems
        bad += 0 if ok else 1
        print(f"[{key}] pub / pub(crate) top-level items before={len(before)} after={len(after)} lost={len(ob)} gained={len(oa)} {'OK' if ok else 'PROBLEM'}")
        for x in sorted(ob):
            print("    lost", x)
        for x in sorted(oa):
            print("    gained", x)
        for p in problems:
            print("    PROBLEM", p)
    elif cmd == "fmt":
        def hunks(path):
            r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config-path", ROOT + "/rustfmt.toml", "--config", "skip_children=true", path], capture_output=True, text=True, cwd=ROOT)
            return sum(1 for l in (r.stdout + r.stderr).split("\n") if l.startswith("Diff in"))
        b = hunks(f"{ORIG}/{key}.rs")
        a = sum(hunks(f"{BASE}/{f}") for f in files)
        tb += b
        ta += a
        print(f"[{key}] rustfmt --check hunks: original {b} -> root plus children {a}")
if cmd == "fmt":
    print(f"FMT TOTAL original {tb} -> split {ta}")
else:
    print(cmd.upper(), "OK" if bad == 0 else f"{bad} PROBLEM")
sys.exit(1 if bad else 0)
