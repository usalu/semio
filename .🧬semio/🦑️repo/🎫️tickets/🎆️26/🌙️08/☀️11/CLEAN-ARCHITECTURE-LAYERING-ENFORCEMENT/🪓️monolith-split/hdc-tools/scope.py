"""Module-aware trait-in-scope analysis for the hub callers of the HubDirectory port.
For every hub Rust file: which capability traits do its method calls need, which does it import (own `use`) and inherit (through `use super::*`)."""
import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *

PATH_MOD = re.compile(r'((?:\s*#\[[^\]]*\]\s*)*)\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', re.M)
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
USE_GLOB_SUPER = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+super::\*\s*;', re.M)
USE_RX = re.compile(r'(?:^|[;{}\n])\s*(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', re.S)


def mounts(path, text):
    """[(child_abs_path)] of file-mounted modules in this file."""
    out = []
    m_text = mask(text)
    for m in PATH_MOD.finditer(m_text):
        attrs = text[m.start(1):m.end(1)] if m.group(1) else ""
        pa = PATH_ATTR.search(attrs)
        name = m.group(2)
        if pa:
            rel = pa.group(1)
            out.append(os.path.normpath(os.path.join(os.path.dirname(path), rel)))
        else:
            d = os.path.dirname(path)
            c1 = os.path.join(d, name + ".rs")
            c2 = os.path.join(d, name, "mod.rs")
            out.append(c1 if os.path.exists(c1) else c2)
    return out


def used_names(text):
    """Flat set of identifiers brought in by `use` items (last segments / alias targets) plus the raw use strings."""
    m_text = mask(text)
    uses = []
    for m in USE_RX.finditer(m_text):
        uses.append(" ".join(m.group(2).split()))
    return uses


def expand(tree, prefix=""):
    tree = tree.strip()
    parts, cur, depth = [], "", 0
    for ch in tree:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    parts.append(cur)
    out = []
    if len(parts) > 1:
        for p in parts:
            if p.strip():
                out += expand(p, prefix)
        return out
    t = parts[0].strip()
    m = re.match(r"^(.*?)::\s*\{(.*)\}$", t, re.S)
    if m:
        return expand(m.group(2), prefix + m.group(1).strip() + "::")
    if t.startswith("{") and t.endswith("}"):
        return expand(t[1:-1], prefix)
    return [prefix + t]


def build():
    files = [os.path.join(ROOT, f) for f in hub_files()]
    texts = {p: read(p) for p in files}
    parent = {}
    for p, t in texts.items():
        for c in mounts(p, t):
            if c in texts:
                parent.setdefault(c, []).append(p)
    return files, texts, parent


def imports_of(text):
    """set of (leaf, fullpath) imported by use statements; globs recorded as leaf '*'."""
    res = []
    for u in used_names(text):
        for path in expand(u):
            alias = None
            m = re.match(r"^(.*?)\s+as\s+(\w+)$", path.strip())
            if m and not path.strip().endswith("::*"):
                path, alias = m.group(1), m.group(2)
            path = re.sub(r"\s+", "", path)
            leaf = path.split("::")[-1]
            res.append((alias or leaf, path))
    return res


def trait_calls(text):
    t = mask(text)
    calls = collections.defaultdict(list)
    for name, tr in TRAIT_OF.items():
        for m in re.finditer(r"(?:\.|::)" + re.escape(name) + r"\s*(?:\(|::<)", t):
            line = t.count("\n", 0, m.start()) + 1
            calls[tr].append((name, line))
    return calls


if __name__ == "__main__":
    files, texts, parent = build()
    print(len(files), "files", len(parent), "mounted")
