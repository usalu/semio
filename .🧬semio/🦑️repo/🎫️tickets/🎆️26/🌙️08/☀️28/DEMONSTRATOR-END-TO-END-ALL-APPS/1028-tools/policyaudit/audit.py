import re, subprocess, sys, os, json

ROOT = "/Users/ueli/Documents/semio"
OUT = sys.argv[1]

def git(*args):
    return subprocess.run(["git", "-c", "core.quotePath=false", *args], cwd=ROOT,
                          capture_output=True, text=True, encoding="utf-8", errors="replace").stdout

def added_lines():
    """Yield (path, lineno, text) for every added line in tracked .rs diffs plus all lines of untracked .rs files."""
    diff = git("diff", "-U0", "--no-color", "--", "*.rs")
    path = None
    newno = 0
    for line in diff.split("\n"):
        if line.startswith("+++ "):
            path = line[6:] if line.startswith("+++ b/") else None
        elif line.startswith("@@"):
            m = re.search(r"\+(\d+)(?:,(\d+))?", line)
            newno = int(m.group(1))
        elif line.startswith("+") and path:
            yield path, newno, line[1:]
            newno += 1
    for p in git("ls-files", "-o", "--exclude-standard", "--", "*.rs").split("\n"):
        if not p:
            continue
        full = os.path.join(ROOT, p)
        try:
            with open(full, encoding="utf-8", errors="replace") as fh:
                for i, t in enumerate(fh, 1):
                    yield p, i, t.rstrip("\n")
        except OSError:
            pass

file_cache = {}
def file_lines(p):
    if p not in file_cache:
        with open(os.path.join(ROOT, p), encoding="utf-8", errors="replace") as fh:
            file_cache[p] = fh.read().split("\n")
    return file_cache[p]

def test_lines(p):
    """Set of 1-based line numbers inside #[cfg(test)] blocks."""
    lines = file_lines(p)
    marked = set()
    depth = 0
    pending = False
    test_depth = None
    for i, t in enumerate(lines, 1):
        s = t.strip()
        if s.startswith("#[cfg(test)]"):
            pending = True
        if test_depth is not None:
            marked.add(i)
        opens = t.count("{")
        closes = t.count("}")
        if pending and opens and test_depth is None:
            test_depth = depth
            marked.add(i)
            pending = False
        depth += opens - closes
        if test_depth is not None and depth <= test_depth and i > 0 and (opens or closes):
            if depth <= test_depth:
                test_depth = None
    return marked

def is_test_path(p):
    parts = p.split("/")
    name = parts[-1]
    return any("test" in x.lower() for x in parts[:-1]) or name.endswith("_test.rs") or name == "tests.rs" or name.startswith("test_")

EMOJI_MIN = 0x2190
def starts_with_emoji(text):
    t = text.strip()
    return bool(t) and ord(t[0]) >= EMOJI_MIN

findings = []
def add(cat, p, n, text, note):
    findings.append({"cat": cat, "file": p, "line": n, "text": text.strip()[:220], "note": note})

tcache = {}
for p, n, t in added_lines():
    s = t.strip()
    if not s:
        continue
    if re.search(r"\bpub\s+use\b", s):
        add("2-reexport", p, n, t, "pub use (check whether a moved item is re-exported from old location)")
    if re.search(r"#\[deprecated|#!?\[allow\(|\bdeprecated\b", s) and "#[deprecated" in s or "#[allow(" in s or "#![allow(" in s:
        cat = "3-allow" if ("allow(" in s) else "2-deprecated"
        add(cat, p, n, t, "allow attribute added" if cat == "3-allow" else "deprecated attribute")
    if re.search(r"\b(legacy|compat|compatibility|shim)\b", s, re.I):
        add("2-compat-name", p, n, t, "legacy/compat/shim wording in added code")
    if "[DEBUG]" in s or re.search(r"\bdbg!\(", s) or re.search(r"println!\(\s*\"\s*DEBUG", s):
        add("7-debug", p, n, t, "debug print or [DEBUG] marker")
    if re.search(r"\.(unwrap|expect)\(", s):
        if p not in tcache:
            tcache[p] = test_lines(p) if os.path.exists(os.path.join(ROOT, p)) else set()
        if not (is_test_path(p) or n in tcache[p]):
            add("4-unwrap-expect", p, n, t, "unwrap/expect in non-test code")
    if s.startswith("//") and not s.startswith("///") and not s.startswith("//!"):
        indented = t[:len(t) - len(t.lstrip())] != ""
        if indented:
            add("5-comment-in-body", p, n, t, "indented // comment (likely inside a body)")
    elif re.search(r"[;{})\],]\s+//\s", t) and not re.search(r'"[^"]*//[^"]*"', t):
        add("5-comment-in-body", p, n, t, "trailing // comment after code")
    if s.startswith("///") or s.startswith("//!"):
        text = s[3:]
        lines = file_lines(p) if os.path.exists(os.path.join(ROOT, p)) else []
        prev = lines[n - 2].strip() if n >= 2 and n - 2 < len(lines) else ""
        if not (prev.startswith("///") or prev.startswith("//!")):
            if not starts_with_emoji(text):
                add("6-docstring-no-emoji", p, n, t, "docstring block start without leading emoji")

with open(OUT, "w", encoding="utf-8") as fh:
    json.dump(findings, fh, ensure_ascii=False, indent=1)
from collections import Counter
print(Counter(f["cat"] for f in findings))
