"""🧭 C4P28 mount scan: every `#[path]` and `include!` target of the crate closure of each split root exists, each new file is reached exactly once, and no file is mounted twice (duplicate mounts: before versus after).
usage: mounts.py stage|live"""
import collections, glob, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
import re
from rslex import lex

REPO = "/Users/ueli/Documents/semio"


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
                out.append(("path", os.path.normpath(os.path.join(cur, m.group(1)))))
            continue
        if re.match(r"^(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{\s*$", st) and line == line.lstrip():
            stack.append(pending or cur)
            pending = None
            continue
        if line == "}" and stack:
            stack.pop()
            continue
        if st.startswith("mod ") or st.startswith("pub mod ") or st.startswith("pub(crate) mod "):
            pending = None
        for m2 in re.finditer(r'\binclude!\(\s*"([^"]+)"\s*\)', line):
            out.append(("include", os.path.normpath(os.path.join(cur, m2.group(1)))))
    return out


def closure(root_abs, reader):
    counts = collections.Counter()
    missing = []
    seen = set()
    order = []

    def visit(p):
        if p in seen:
            return
        seen.add(p)
        order.append(p)
        txt = reader(p)
        if txt is None:
            return
        for kind, target in path_mounts(txt, os.path.dirname(p)):
            counts[target] += 1
            if reader(target) is None:
                missing.append((os.path.relpath(p, REPO), os.path.relpath(target, REPO)))
            else:
                visit(target)

    visit(root_abs)
    return order, counts, missing


mode = sys.argv[1]
bad = 0
for path in sorted(glob.glob(HERE + "/specs/*.json")):
    n = int(os.path.basename(path).split(".")[0])
    spec = json.load(open(path, encoding="utf-8"))
    root_abs = os.path.join(REPO, spec["file"])
    base = spec["stage"] if mode == "stage" else REPO

    def now(p):
        rel = os.path.relpath(p, REPO)
        for cand in ([os.path.join(base, rel)] if mode == "stage" else []) + [p]:
            if os.path.isfile(cand):
                return open(cand, encoding="utf-8").read()
        return None

    def before(p):
        if p == root_abs:
            return open(spec["original"], encoding="utf-8").read()
        return open(p, encoding="utf-8").read() if os.path.isfile(p) else None

    o_b, c_b, m_b = closure(root_abs, before)
    o_a, c_a, m_a = closure(root_abs, now)
    dup_b = {k for k, v in c_b.items() if v > 1}
    dup_a = {k for k, v in c_a.items() if v > 1}
    new_files = [os.path.join(REPO, os.path.dirname(spec["file"]), c["folder"], "🦀️.rs") for c in spec["chunks"]]
    reach = {f: c_a.get(f, 0) for f in new_files}
    ok = not m_a and not (dup_a - dup_b) and all(v == 1 for v in reach.values())
    bad += 0 if ok else 1
    print(f"[{n}] closure files before={len(o_b)} after={len(o_a)} missing targets before={len(m_b)} after={len(m_a)} duplicate mounts before={len(dup_b)} after={len(dup_a)} new={len(dup_a - dup_b)} new files reached once each={all(v == 1 for v in reach.values())} {'OK' if ok else 'PROBLEM'}")
    for x in m_a[:5]:
        print("    missing", x)
print("MOUNTS", "OK (0 new duplicates, all targets exist)" if bad == 0 else f"{bad} PROBLEM")
sys.exit(1 if bad else 0)
