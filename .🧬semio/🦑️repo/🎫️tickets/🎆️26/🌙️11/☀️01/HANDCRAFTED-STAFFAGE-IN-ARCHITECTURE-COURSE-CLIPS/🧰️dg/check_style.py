import ast
import io
import re
import subprocess
import sys
import tokenize
import unicodedata
from pathlib import Path

REPO = Path("C:/git/semio")
T = REPO / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
LIST = T / "r2-changed-files.md"
OUT = T / "🗑️generated/dg/findings.txt"


def listed_files():
    section = None
    paths = []
    for line in LIST.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            section = line
            continue
        m = re.match(r"- `(.+)`$", line.strip())
        if not m or section is None:
            continue
        if section.startswith("## Deleted") or section.startswith("## Uncertain"):
            continue
        p = m.group(1)
        if p.endswith((".py", ".ts", ".tsx")):
            paths.append((section.split()[1], p))
    return paths


def added_lines(rel, created):
    full = REPO / rel
    if not full.exists():
        return None
    if created:
        n = len(full.read_text(encoding="utf-8").splitlines())
        return set(range(1, n + 2))
    out = subprocess.run(
        ["git", "-c", "core.quotepath=false", "diff", "HEAD", "-U0", "--", rel],
        cwd=REPO, capture_output=True, text=True, encoding="utf-8",
    ).stdout
    lines = set()
    for m in re.finditer(r"^@@ -\S+ \+(\d+)(?:,(\d+))? @@", out, re.M):
        start = int(m.group(1))
        count = int(m.group(2)) if m.group(2) is not None else 1
        lines.update(range(start, start + count))
    return lines


def is_emoji_start(text):
    s = text.lstrip()
    if not s:
        return False
    c = s[0]
    o = ord(c)
    if len(s) > 1 and s[1] == "️":
        return True
    return o >= 0x1F000 or 0x2190 <= o <= 0x2BFF or o in (0xA9, 0xAE, 0x3030, 0x303D, 0x3297, 0x3299)


def py_check(rel, added):
    src = (REPO / rel).read_text(encoding="utf-8")
    out = []
    tree = ast.parse(src)
    body_ranges = []
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            body_ranges.append((node.lineno, node.end_lineno, node.name))
        if isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)):
            if node.body and isinstance(node.body[0], ast.Expr) and isinstance(getattr(node.body[0], "value", None), ast.Constant) and isinstance(node.body[0].value.value, str):
                d = node.body[0]
                name = getattr(node, "name", "<module>")
                if not is_emoji_start(d.value.value):
                    out.append(("c", d.lineno, d.end_lineno, f"docstring of {name} does not start with emoji: {d.value.value[:40]!r}"))
    stmt_ranges = [(n.lineno, n.end_lineno) for n in tree.body]
    for tok in tokenize.generate_tokens(io.StringIO(src).readline):
        if tok.type != tokenize.COMMENT:
            continue
        ln = tok.start[0]
        text = tok.string
        if ln == 1 and text.startswith("#!"):
            continue
        if re.search(r"type:\s*ignore|noqa|pyright:|pylint:|# type:", text):
            continue
        inside_def = [n for s, e, n in body_ranges if s <= ln <= e]
        if inside_def:
            out.append(("a", ln, ln, f"comment inside {inside_def[-1]}: {text[:80]}"))
        elif any(s <= ln <= e for s, e in stmt_ranges):
            out.append(("a?", ln, ln, f"comment inside module-level statement: {text[:80]}"))
    dbg = []
    for i, line in enumerate(src.splitlines(), 1):
        if "[DEBUG]" in line:
            dbg.append(("b", i, i, f"[DEBUG] marker: {line.strip()[:80]}"))
        if re.match(r"\s*print\(", line):
            dbg.append(("b?", i, i, f"print call: {line.strip()[:80]}"))
    out.extend(dbg)
    return [o for o in out if any(added is None or x in added for x in range(o[1], o[2] + 1))]


def regex_allowed(src, i):
    k = i - 1
    while k >= 0 and src[k] in " \t":
        k -= 1
    if k < 0:
        return True
    if src[k] in "(,=:[!&|?{};+-*%<>~^\n":
        return True
    m = re.search(r"(return|typeof|case|in|of)$", src[max(0, k - 8):k + 1])
    return m is not None


def ts_scan(src):
    comments = []
    stack = []
    n = len(src)
    line = 1
    i = 0

    def template(i, line):
        while i < n:
            ch = src[i]
            if ch == "\\":
                i += 2
                continue
            if ch == "`":
                return i + 1, line, False
            if src.startswith("${", i):
                return i + 2, line, True
            if ch == "\n":
                line += 1
            i += 1
        return i, line, False

    while i < n:
        c = src[i]
        if c == "\n":
            line += 1
            i += 1
        elif src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            comments.append(("line", line, len(stack), src[i:j]))
            i = j
        elif src.startswith("/*", i):
            j = src.find("*/", i + 2)
            j = n if j < 0 else j + 2
            text = src[i:j]
            comments.append(("block", line, len(stack), text))
            line += text.count("\n")
            i = j
        elif c == "/" and regex_allowed(src, i):
            j = i + 1
            in_class = False
            while j < n and src[j] != "\n":
                if src[j] == "\\":
                    j += 2
                    continue
                if src[j] == "[":
                    in_class = True
                elif src[j] == "]":
                    in_class = False
                elif src[j] == "/" and not in_class:
                    break
                j += 1
            i = j + 1
        elif c in "'\"":
            j = i + 1
            while j < n and src[j] != c and src[j] != "\n":
                j += 2 if src[j] == "\\" else 1
            i = j + 1
        elif c == "`":
            i, line, opened = template(i + 1, line)
            if opened:
                stack.append("${")
        elif c in "{([":
            stack.append(c)
            i += 1
        elif c in "})]":
            i += 1
            if c == "}" and stack and stack[-1] == "${":
                stack.pop()
                i, line, opened = template(i, line)
                if opened:
                    stack.append("${")
            elif stack:
                stack.pop()
        else:
            i += 1
    return comments


def ts_check(rel, added):
    src = (REPO / rel).read_text(encoding="utf-8")
    out = []
    for kind, ln, depth, text in ts_scan(src):
        if kind == "block" and text.startswith("/**"):
            body = text[3:].rstrip("*/").strip()
            if depth == 0 and not is_emoji_start(body):
                out.append(("c", ln, ln + text.count("\n"), f"JSDoc does not start with emoji: {body[:40]!r}"))
            elif depth > 0:
                out.append(("a", ln, ln + text.count("\n"), f"JSDoc inside body depth {depth}: {text[:60]!r}"))
            continue
        if depth > 0:
            out.append(("a", ln, ln, f"comment at depth {depth}: {text[:80]}"))
    for i, line in enumerate(src.splitlines(), 1):
        if "[DEBUG]" in line:
            out.append(("b", i, i, f"[DEBUG] marker: {line.strip()[:80]}"))
        if re.search(r"console\.(log|debug|info|warn)\(", line):
            out.append(("b?", i, i, f"console call: {line.strip()[:80]}"))
    return [o for o in out if any(added is None or x in added for x in range(o[1], o[2] + 1))]


def main():
    results = []
    files = listed_files()
    for status, rel in files:
        created = status == "Created"
        added = added_lines(rel, created)
        if added is not None and not added:
            continue
        try:
            if rel.endswith(".py"):
                found = py_check(rel, added)
            else:
                found = ts_check(rel, added)
        except SyntaxError as e:
            found = [("ERR", 0, 0, f"parse error {e}")]
        for kind, a, b, msg in found:
            results.append(f"{kind}\t{rel}:{a}\t{msg}")
    results.append(f"# files checked: {len(files)}")
    OUT.write_text("\n".join(results) + "\n", encoding="utf-8")
    print(f"checked {len(files)} files, {len(results) - 1} findings -> {OUT}")


if __name__ == "__main__":
    main()
