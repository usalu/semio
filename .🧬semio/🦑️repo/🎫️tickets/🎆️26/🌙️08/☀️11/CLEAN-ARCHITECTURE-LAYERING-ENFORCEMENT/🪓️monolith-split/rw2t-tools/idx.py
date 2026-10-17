"""🗂️ rw2t: first-party export index (crate, module path) per name, the alias table of the crate roots' own imports, and the law-file free-name analysis."""
import sys, os, re, json, collections
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rtb-tools")
from lib import *
import exports as EX
import modtree
from rs import blank, KEYWORDS, PRIMS, uses, item_names

CRATES = crate_table()
RANK = {k: v["rank"] for k, v in json.load(open(GEN + "/rwp0-crates.json")).items()}
DEPS = {}

def manifest_deps(suffix):
    mf = os.path.join(T, CRATES[suffix], "📦️packages/🦀️rust/Cargo.toml")
    t = read(mf)
    out = {"normal": set(), "dev": set()}
    sec = None
    for line in t.split("\n"):
        m = re.match(r"\[(.+)\]", line)
        if m:
            sec = m.group(1)
            continue
        dm = re.match(r'(semio-framework-os-renderer-wgpu-[a-z0-9-]+)\s*=', line)
        if dm and sec:
            nm = dm.group(1)[len(PKG_PREFIX):]
            out["dev" if "dev-dependencies" in sec else "normal"].add(nm)
    return out

def closure(suffix):
    seen = set()
    stack = [suffix]
    while stack:
        c = stack.pop()
        if c in seen or c not in CRATES:
            continue
        seen.add(c)
        if c not in DEPS:
            DEPS[c] = manifest_deps(c)
        stack.extend(DEPS[c]["normal"] | DEPS[c]["dev"])
    return seen

def build_index():
    idx = collections.defaultdict(list)
    def walk(suffix, file, mp):
        for name, is_pub, f, cfg in EX.child_mounts(file):
            if is_pub:
                exp = EX.module_exports(f, CRATES)
                for n, c in exp.items():
                    idx[n].append((suffix, mp + (name,), tuple(sorted(c))))
                walk(suffix, f, mp + (name,))
    for suffix, folder in CRATES.items():
        root = os.path.join(T, folder, "🦀️.rs")
        if not os.path.exists(root):
            continue
        exp = EX.module_exports(root, CRATES)
        for n, c in exp.items():
            idx[n].append((suffix, (), tuple(sorted(c))))
        walk(suffix, root, ())
    return idx

def alias_table(suffixes):
    tab = collections.defaultdict(set)
    for suffix in suffixes:
        root = os.path.join(T, CRATES[suffix], "🦀️.rs")
        for f in modtree.tree(root):
            t = read(f)
            for cfg, vis, p, local, glob in uses(t, top_only=True):
                if glob or vis.startswith("pub"):
                    continue
                if p and p[0] in ("crate", "super", "self"):
                    continue
                tab[local].add("::".join(p))
    return tab

def strip_uses(text):
    return re.sub(r"(?m)^[ \t]*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]*;", lambda m: " " * len(m.group(0)), text)

def free_tokens(text):
    clean = blank(strip_uses(text))
    out = collections.Counter()
    for m in re.finditer(r"(?<![A-Za-z0-9_$])([A-Za-z_][A-Za-z0-9_]*)", clean):
        s, e = m.start(), m.end()
        w = m.group(1)
        if w in KEYWORDS or w == "_":
            continue
        k = s - 1
        while k >= 0 and clean[k] in " \t\n":
            k -= 1
        if k >= 1 and clean[k - 1:k + 1] == "::":
            continue
        if k >= 0 and clean[k] == "." and not (k >= 1 and clean[k - 1] == "."):
            continue
        j = e
        while j < len(clean) and clean[j] in " \t":
            j += 1
        if j < len(clean) and clean[j] == ":" and clean[j:j + 2] != "::":
            continue
        out[w] += 1
    for s in re.finditer(r'"((?:[^"\\\n]|\\.)*)"', text):
        for f in re.finditer(r"\{([A-Za-z_][A-Za-z0-9_]*)(?::[^{}]*)?\}", s.group(1)):
            out[f.group(1)] += 1
    return out

def local_names(text):
    names = set(item_names(text))
    for m in re.finditer(r"\bmod\s+([A-Za-z_]\w*)", blank(text)):
        names.add(m.group(1))
    for cfg, vis, p, local, glob in uses(text):
        if local:
            names.add(local)
    return names
