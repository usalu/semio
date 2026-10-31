import re, subprocess, sys, os, json, hashlib
from collections import Counter, defaultdict

ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS")
GEN = os.path.join(TICKET, "🗑️generated/1029")
LIST = os.path.join(GEN, "policy29-src.txt")
CHANGED = os.path.join(GEN, "policy29-changed.txt")
OUT = os.path.join(GEN, "policy29-findings.json")

def git(*args):
    return subprocess.run(["git", "-c", "core.quotePath=false", *args], cwd=ROOT,
                          capture_output=True, text=True, encoding="utf-8", errors="replace").stdout

def rel(p):
    return os.path.relpath(os.path.join(ROOT, p) if not p.startswith(ROOT) else p, ROOT)

tracked = set(git("ls-files").split("\n"))
diff_added = defaultdict(set)
cur = None
newno = 0
for line in git("diff", "-U0", "--no-color", "HEAD").split("\n"):
    if line.startswith("+++ "):
        cur = line[6:] if line.startswith("+++ b/") else None
    elif line.startswith("@@") and cur:
        m = re.search(r"\+(\d+)(?:,(\d+))?", line)
        newno = int(m.group(1))
    elif line.startswith("+") and cur and not line.startswith("+++"):
        diff_added[cur].add(newno)
        newno += 1
    elif line.startswith("-") and not line.startswith("---"):
        pass

files = [l.strip() for l in open(LIST, encoding="utf-8").read().split("\n") if l.strip()]
files = [rel(f) for f in files]

def test_lines(lines):
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
        if test_depth is not None and depth <= test_depth and (opens or closes):
            test_depth = None
    return marked

def is_test_path(p):
    parts = p.split("/")
    name = parts[-1]
    return any(re.search(r"test", x, re.I) for x in parts[:-1]) or name.endswith("_test.rs") or name == "tests.rs" or name.startswith("test_") or re.search(r"\.(test|spec)\.(ts|tsx|mts|js|mjs)$", name) is not None

EMOJI_MIN = 0x2190
def starts_with_emoji(text):
    t = text.strip()
    return bool(t) and ord(t[0]) >= EMOJI_MIN

findings = []
def add(cat, p, n, text, note):
    findings.append({"cat": cat, "file": p, "line": n, "new": n in diff_added.get(p, set()),
                     "text": text.strip()[:200], "note": note})

def scan_file(p):
    full = os.path.join(ROOT, p)
    if not os.path.isfile(full):
        return
    try:
        with open(full, encoding="utf-8", errors="replace") as fh:
            lines = fh.read().split("\n")
    except OSError:
        return
    ext = p.rsplit(".", 1)[-1] if "." in p.split("/")[-1] else ""
    name = p.split("/")[-1]
    is_rs = ext == "rs"
    is_ts = ext in ("ts", "tsx", "mts", "cts")
    is_js = ext in ("js", "jsx", "mjs", "cjs")
    is_go = ext == "go"
    tl = test_lines(lines) if is_rs else set()
    tp = is_test_path(p)

    if name == "Cargo.toml":
        for i, t in enumerate(lines, 1):
            if re.search(r"rust-source-boundary|max[-_]?lines|max[-_]?bytes|limit", t, re.I):
                add("1-boundary-limit", p, i, t, "source-boundary / limit key in Cargo.toml")
        return

    if is_rs:
        for i, t in enumerate(lines, 1):
            s = t.strip()
            if not s:
                continue
            if re.search(r"\bpub(\([^)]*\))?\s+use\b", s) and re.search(r"\bas\s+\w+", s):
                add("2-reexport-alias", p, i, t, "pub use ... as ... (possible compat alias)")
            elif re.search(r"\bpub(\([^)]*\))?\s+use\b", s):
                add("2-reexport", p, i, t, "pub use re-export (check if a moved item is re-exported from old location)")
            if re.search(r"\btype\s+(Original|Legacy|Old)\w*\s*=|\b(Original|Legacy)[A-Z]\w*", s):
                add("2-legacy-alias", p, i, t, "Original*/Legacy* name")
            if re.search(r"\b(legacy|compat|compatibility|shim)\b", s, re.I):
                add("2-compat-name", p, i, t, "legacy/compat/shim wording")
            if re.search(r"#!?\[allow\(", s):
                add("3-allow", p, i, t, "#[allow(...)] attribute")
            if re.search(r"#\[deprecated", s):
                add("2-deprecated", p, i, t, "#[deprecated] attribute")
            if not (tp or i in tl) and re.search(r"\.(unwrap|expect)\(|\bpanic!\(|\bunimplemented!\(|\btodo!\(", s):
                add("4-unwrap-expect-panic", p, i, t, "unwrap/expect/panic in non-test code")
            if "[DEBUG]" in s or re.search(r"\bdbg!\(", s):
                add("7-debug", p, i, t, "[DEBUG] or dbg! marker")
            if s.startswith("//") and not s.startswith("///") and not s.startswith("//!"):
                if t[:len(t) - len(t.lstrip())] != "":
                    add("5-comment-in-body", p, i, t, "indented // comment (likely inside a body)")
            elif re.search(r"[;{})\],]\s+//\s", t) and not re.search(r'"[^"]*//[^"]*"', t):
                add("5-comment-in-body", p, i, t, "trailing // comment after code")
            if s.startswith("///") or s.startswith("//!"):
                prev = lines[i - 2].strip() if i >= 2 else ""
                if not (prev.startswith("///") or prev.startswith("//!")):
                    if not starts_with_emoji(s[3:]):
                        add("6-docstring-no-emoji", p, i, t, "doc block start without leading emoji")

    if is_ts or is_js:
        for i, t in enumerate(lines, 1):
            s = t.strip()
            if not s:
                continue
            if re.search(r"\bexport\s+(\*|\{[^}]*\})\s+from\b", s) and re.search(r"\bas\b", s):
                add("2-reexport-alias", p, i, t, "re-export with alias (possible compat shim)")
            if re.search(r"\b(Original|Legacy|legacy|compat|shim|deprecated)\w*", s):
                add("2-compat-name", p, i, t, "legacy/compat/shim/deprecated wording")
            if "[DEBUG]" in s or re.search(r"console\.(log|debug|info)\(\s*[\"'`]\[DEBUG\]", s):
                add("7-debug", p, i, t, "[DEBUG] console output")
            elif re.search(r"console\.log\(", s) and not tp:
                add("7-console-log", p, i, t, "console.log in non-test code (check for temporary logs)")
            if is_ts:
                if re.search(r"@ts-(ignore|expect-error|nocheck)", s):
                    add("8-ts-ignore", p, i, t, "@ts-ignore / @ts-expect-error")
                if re.search(r"as\s+unknown\s+as\b", s):
                    add("8-as-unknown-as", p, i, t, "as unknown as cast")
                if re.search(r"(:\s*any\b|<any>|\bas\s+any\b|\bany\[\]|<any\[\]>|Array<any>|Record<[^>]*,\s*any>)", s):
                    add("8-any", p, i, t, "TS any type")
            if s.startswith("//") and t[:len(t) - len(t.lstrip())] != "" and not s.startswith("///"):
                add("5-comment-in-body", p, i, t, "indented // comment (check if inside a body)")
            elif re.search(r"[;{})\],]\s+//\s", t) and not re.search(r'["\'`][^"\'`]*//[^"\'`]*["\'`]', t) and not re.search(r"https?:", t):
                add("5-comment-in-body", p, i, t, "trailing // comment after code")
            if s.startswith("/**"):
                add("6-jsdoc-check", p, i, t, "JSDoc block: verify it starts with an emoji")

    if is_go:
        for i, t in enumerate(lines, 1):
            s = t.strip()
            if "[DEBUG]" in s or re.search(r"fmt\.Print\w*\(\s*\"\[DEBUG\]", s):
                add("7-debug", p, i, t, "[DEBUG] marker")
            if t[:len(t) - len(t.lstrip())] != "" and s.startswith("//"):
                add("5-comment-in-body", p, i, t, "indented // comment (check if inside a body)")

for p in files:
    scan_file(p)

# category 9: copies of tracked sources inside ticket folders changed since 01:55
tracked_blobs = {}
for line in git("ls-files", "-s").split("\n"):
    if not line:
        continue
    meta, path = line.split("\t", 1)
    tracked_blobs.setdefault(meta.split()[1], []).append(path)

tickets_root = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets")
find_out = subprocess.run(["find", tickets_root, "-newermt", "2026-10-29 01:55", "-type", "f",
                           "(", "-name", "*.rs", "-o", "-name", "*.ts", "-o", "-name", "*.tsx", "-o", "-name", "*.mts",
                           "-o", "-name", "*.mjs", "-o", "-name", "*.go", "-o", "-name", "*.py", "-o", "-name", "*.wit", ")"],
                          capture_output=True, text=True, encoding="utf-8", errors="replace").stdout
nine = []
for full in find_out.split("\n"):
    if not full:
        continue
    with open(full, "rb") as fh:
        data = fh.read()
    h = hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()
    if h in tracked_blobs:
        nine.append({"ticket_file": os.path.relpath(full, ROOT), "tracked_as": tracked_blobs[h]})
        findings.append({"cat": "9-ticket-copy-of-tracked", "file": os.path.relpath(full, ROOT), "line": 0,
                         "new": True, "text": "identical blob to " + ", ".join(tracked_blobs[h])[:200],
                         "note": "copy of tracked source in ticket folder"})
print("ticket source files since 01:55:", len([f for f in find_out.split("\n") if f]))

with open(OUT, "w", encoding="utf-8") as fh:
    json.dump({"findings": findings, "nine": nine}, fh, ensure_ascii=False, indent=1)
print(json.dumps(Counter(f["cat"] for f in findings), ensure_ascii=False, indent=1))
print("files scanned", len(files))
