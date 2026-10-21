"""Shared helpers of the round-6 merged-copy makers: copy a staged tool tree, then patch the copy with exact-once replacements (any miss is a hard error: the copies are re-derived from the originals at every start)."""
import os, shutil

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SKIP = shutil.ignore_patterns("__pycache__", "backup", "backup-*", "backups", "originals", "node_modules", "🗑️generated")


def fresh_copy(source, name, beside=False):
    """beside=True puts the copy next to the originals in the ticket folder (tools that find the repository root from their own depth)."""
    target = os.path.join(C, name) if beside else os.path.join(HERE, name)
    shutil.rmtree(target, ignore_errors=True)
    shutil.copytree(source, target, ignore=SKIP)
    return target


def patch(path, old, new, count=1):
    text = open(path, encoding="utf8").read()
    n = text.count(old)
    if n != count:
        raise SystemExit("maker: %s: anchor found %d times (wanted %d): %r" % (os.path.relpath(path, C), n, count, old[:100]))
    open(path, "w", encoding="utf8").write(text.replace(old, new))


def drop_lines(path, needles):
    """Removes every top-level tuple row (single physical line) that contains one of the needles; returns how many."""
    lines = open(path, encoding="utf8").read().split("\n")
    kept, dropped = [], 0
    for line in lines:
        if any(x in line for x in needles):
            dropped += 1
        else:
            kept.append(line)
    open(path, "w", encoding="utf8").write("\n".join(kept))
    return dropped


def drop_rows(path, group, targets):
    """Removes the (possibly multi-line) tuple rows of an ops list whose first element is the constant `group` and whose second element's source equals one of `targets` (or starts with one when it ends with *)."""
    import ast
    text = open(path, encoding="utf8").read()
    tree = ast.parse(text)
    spans = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Tuple) and len(node.elts) >= 3 and isinstance(node.elts[0], ast.Constant) and node.elts[0].value == group:
            src = ast.get_source_segment(text, node.elts[1])
            if any(src == t or (t.endswith("*") and src.startswith(t[:-1])) for t in targets):
                spans.append((node.lineno, node.end_lineno))
    lines = text.split("\n")
    for a, b in sorted(spans, reverse=True):
        del lines[a - 1:b]
    open(path, "w", encoding="utf8").write("\n".join(lines))
    return len(spans)


def patch_ops_json(path, edits_key, file_suffix, old_anchor_prefix, fn):
    """JSON op lists {file, anchor, replacement}: applies fn(anchor, replacement) -> (anchor, replacement) to the one op whose file ends with file_suffix and whose anchor starts with old_anchor_prefix."""
    import json
    spec = json.load(open(path, encoding="utf8"))
    hits = [op for op in spec[edits_key] if op["file"].endswith(file_suffix) and op["anchor"].startswith(old_anchor_prefix)]
    if len(hits) != 1:
        raise SystemExit("maker: %s: %d ops match %r" % (os.path.relpath(path, C), len(hits), old_anchor_prefix[:60]))
    hits[0]["anchor"], hits[0]["replacement"] = fn(hits[0]["anchor"], hits[0]["replacement"])
    json.dump(spec, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
