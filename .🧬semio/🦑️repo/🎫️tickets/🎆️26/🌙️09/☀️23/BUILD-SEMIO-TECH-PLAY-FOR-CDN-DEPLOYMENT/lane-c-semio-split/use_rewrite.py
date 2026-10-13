"""🌲️ Rust path rewriting: linear qualified paths and nested `use` trees, flattened, mapped and re-grouped.

`mapper(segments) -> segments | None` receives a full path as a list (e.g. ["crate", "standards", "v1", ...]) and returns the
rewritten list, or None to keep it. `use` statements whose flattened paths change are re-emitted as ONE statement (attributes
before the statement keep applying to all of it)."""
import re

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


def _mask(src):
    """🙈️ Replaces string, char and comment contents with spaces (same length) so scans skip them."""
    out = list(src)
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                out[k] = " "
            i = j
        elif src.startswith("/*", i):
            j = src.find("*/", i + 2)
            j = n if j < 0 else j + 2
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
        elif c == "r" and re.match(r'r#*"', src[i:i + 10]) and (i == 0 or not re.match(r"[A-Za-z0-9_]", src[i - 1])):
            m = re.match(r'r(#*)"', src[i:])
            hashes = m.group(1)
            end = src.find('"' + hashes, i + len(m.group(0)))
            end = n if end < 0 else end + 1 + len(hashes)
            for k in range(i + len(m.group(0)), max(i + len(m.group(0)), end - 1 - len(hashes))):
                if out[k] != "\n":
                    out[k] = " "
            i = end
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            for k in range(i + 1, min(j, n)):
                if out[k] != "\n":
                    out[k] = " "
            i = j + 1
        elif c == "'" and re.match(r"'(\\.|[^\\'])'", src[i:i + 4]):
            m = re.match(r"'(\\.[^']*|[^\\'])'", src[i:i + 12])
            if m:
                for k in range(i + 1, i + len(m.group(0)) - 1):
                    out[k] = " "
                i += len(m.group(0))
            else:
                i += 1
        else:
            i += 1
    return "".join(out)


def _parse_tree(text, pos=0):
    """🌳️ Parses a use tree starting at `pos`; returns (items, pos) with items = [(segments, leaf)] where leaf is a name, `name as alias`, `*` or `self`."""
    items = []
    prefix = []

    def skip(p):
        while p < len(text) and text[p] in " \t\r\n":
            p += 1
        return p

    p = skip(pos)
    if text.startswith("::", p):
        prefix.append("")
        p = skip(p + 2)
    while True:
        p = skip(p)
        if text.startswith("{", p):
            p += 1
            while True:
                p = skip(p)
                if text.startswith("}", p):
                    p += 1
                    break
                sub, p = _parse_tree(text, p)
                items.extend((prefix + s, leaf) for s, leaf in sub)
                p = skip(p)
                if text.startswith(",", p):
                    p += 1
            return items, p
        if text.startswith("*", p):
            items.append((prefix, "*"))
            return items, p + 1
        m = re.compile(IDENT).match(text, p)
        if not m:
            raise ValueError("bad use tree at %r" % text[p:p + 40])
        name = m.group(0)
        p = skip(m.end())
        if text.startswith("::", p):
            prefix.append(name)
            p += 2
            continue
        am = re.compile(r"as\s+(" + IDENT + ")").match(text, p)
        if am:
            items.append((prefix, name + " as " + am.group(1)))
            return items, am.end()
        items.append((prefix, name))
        return items, p


def _emit(items):
    """🧾️ Re-emits flattened items as one use tree, grouping by prefix."""
    groups = {}
    order = []
    for segs, leaf in items:
        key = tuple(segs)
        if key not in groups:
            groups[key] = []
            order.append(key)
        if leaf not in groups[key]:
            groups[key].append(leaf)
    parts = []
    for key in order:
        leaves = groups[key]
        pre = "::".join(key)
        if len(leaves) == 1 and leaves[0] != "self":
            parts.append((pre + "::" if pre else "") + leaves[0])
        else:
            parts.append((pre + "::" if pre else "") + "{" + ", ".join(leaves) + "}")
    return parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"


USE_RX = re.compile(r"(?m)(^|[;{}]\s*|\]\s*)((?:pub(?:\s*\([^)]*\))?\s+)?use\s+)")


def rewrite_uses(src, mapper):
    """🔁️ Rewrites every `use` statement whose flattened paths the mapper changes; returns (src, count)."""
    masked = _mask(src)
    out = []
    last = 0
    count = 0
    for m in USE_RX.finditer(masked):
        start = m.start(2)
        if start < last:
            continue
        body_start = m.end(2)
        end = masked.find(";", body_start)
        if end < 0:
            continue
        body = src[body_start:end]
        try:
            items, p = _parse_tree(body, 0)
        except ValueError:
            continue
        if body[p:].strip():
            continue
        new_items = []
        changed = False
        for segs, leaf in items:
            full = segs + ([] if leaf in ("*", "self") else [leaf.split(" as ")[0]])
            mapped = mapper(full)
            if mapped is not None and mapped != full:
                changed = True
                if leaf in ("*", "self"):
                    new_items.append((mapped, leaf))
                else:
                    alias = leaf.split(" as ")
                    new_leaf = mapped[-1] + (" as " + alias[1] if len(alias) > 1 else "")
                    new_items.append((mapped[:-1], new_leaf))
            else:
                new_items.append((segs, leaf))
        if not changed:
            continue
        out.append(src[last:body_start])
        out.append(_emit(new_items))
        last = end
        count += 1
    out.append(src[last:])
    return "".join(out), count


def rewrite_linear(src, prefix_map):
    """➡️ Replaces linear qualified path prefixes (outside strings/comments): prefix_map = [(regex, replacement)]."""
    masked = _mask(src)
    count = 0
    for rx, repl in prefix_map:
        pieces = []
        last = 0
        for m in re.finditer(rx, masked):
            pieces.append(src[last:m.start()])
            pieces.append(m.expand(repl))
            last = m.end()
            count += 1
        if pieces:
            pieces.append(src[last:])
            src = "".join(pieces)
            masked = _mask(src)
    return src, count
