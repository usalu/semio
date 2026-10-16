"""🔗️ Module closure of a Rust lib root: every file reached through `#[path] mod` and `include!`."""
import os, re
def children(path, text):
    base = os.path.dirname(path); out = []
    for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;', text): out.append(os.path.normpath(os.path.join(base, m.group(1))))
    for m in re.finditer(r'include!\(\s*"([^"]+)"\s*\)', text): out.append(os.path.normpath(os.path.join(base, m.group(1))))
    return out
def closure(root):
    seen = []; stack = [root]
    while stack:
        p = stack.pop()
        if p in seen or not os.path.isfile(p): continue
        seen.append(p); stack += children(p, open(p, encoding='utf-8').read())
    return seen
