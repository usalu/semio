"""🕸️ Follows `#[path]` / `mod` / `include!` edges from one crate root and returns every source file it compiles."""
import os, re
from use_rewrite import _mask

MOD_RX = re.compile(r"(?:#\[path\s*=\s*\"([^\"]+)\"\]\s*)?(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s*)?\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])")


def walk(root_file):
    """🚶️ Returns the ordered set of files the crate rooted at `root_file` compiles (modules and include! targets)."""
    seen = []
    stack = [(os.path.abspath(root_file), os.path.dirname(os.path.abspath(root_file)))]
    while stack:
        path, base = stack.pop()
        if path in seen or not os.path.isfile(path):
            continue
        seen.append(path)
        src = open(path).read()
        masked = _mask(src)
        _scan(src, masked, 0, len(src), base, path, stack)
        for m in re.finditer(r"include!\(\s*\"([^\"]+)\"\s*\)", src):
            if masked[m.start()] == "i":
                stack.append((os.path.normpath(os.path.join(os.path.dirname(path), m.group(1))), None))
    return seen


def _scan(src, masked, lo, hi, base, path, stack):
    i = lo
    while i < hi:
        m = MOD_RX.search(masked, i, hi)
        if not m:
            break
        attr_m = re.search(r"#\[path\s*=\s*\"([^\"]+)\"\]", src[m.start():m.end()])
        attr = re.sub(r"\\u\{([0-9a-fA-F]+)\}", lambda u: chr(int(u.group(1), 16)), attr_m.group(1)) if attr_m else None
        name = m.group(2)
        if m.group(3) == ";":
            if base is not None:
                target = os.path.normpath(os.path.join(base, attr)) if attr else os.path.join(base, name + ".rs")
                stack.append((target, os.path.dirname(target)))
            i = m.end()
        else:
            depth = 0
            j = m.end() - 1
            while j < hi:
                if masked[j] == "{":
                    depth += 1
                elif masked[j] == "}":
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            inner_base = None if base is None else (os.path.normpath(os.path.join(base, attr)) if attr else os.path.join(base, name))
            _scan(src, masked, m.end(), j, inner_base, path, stack)
            i = j + 1
