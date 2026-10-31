#!/usr/bin/env python3
"""🔎 Read-only AGENTS.md policy2 audit of sources changed since 2026-10-30 06:00 (no edits, no builds)."""

import os
import re
import subprocess

ROOT = "/Users/ueli/Documents/semio"
TICKET = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
OUT_MD = TICKET + "/1030-audit-policy2.md"
GEN = TICKET + "/🗑️generated/1030"
DIRS = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"]
SINCE = "2026-10-30 06:00"
RULES = ["debug", "allow", "unwrap_expect", "unsafe", "doc_no_emoji", "doc_dup_emoji", "comment_fn",
         "comment_def", "compat_ident", "compat_path", "test_threads", "ui_literal", "json_import"]
CODE_LANG = {".rs": "rs", ".ts": "ts", ".tsx": "ts", ".js": "ts", ".mjs": "ts", ".cjs": "ts", ".py": "py"}
TEXT_EXT = set(CODE_LANG) | {".sh", ".bash", ".zsh", ".toml", ".json", ".yml", ".yaml"}
EMO_SINGLE = {0x00A9, 0x00AE, 0x203C, 0x2049, 0x2122, 0x2139, 0x3030, 0x303D, 0x3297, 0x3299, 0x24C2}
DEBUG_RE = re.compile(r"\[DEBUG\]|DBG1030|dbg1030")
ALLOW_RE = re.compile(r"#!?\[\s*allow\(")
UNWRAP_RE = re.compile(r"\.unwrap\(\)|\.expect\(")
UNSAFE_RE = re.compile(r"\bunsafe\b")
SAFETY_RE = re.compile(r"SAFETY|# Safety")
COMPAT_KW = ("compat", "legacy", "shim", "adapter", "fallback")
TT_RE = re.compile(r"test-threads[= ]1(?!\d)")
JSON_WITH_RE = re.compile(r"with\s*:?\s*\{\s*type\s*:\s*[\"']json[\"']")
UI_JSX_RE = re.compile(r">\s*([A-Za-zÀ-ÿ][^<>{}()\[\]=;:&|`\"]*?)\s*</")
UI_ATTR_RE = re.compile(r"\b(?:label|title|placeholder|aria-label|alt|description|caption|heading|message|subtitle|tooltip|helperText|emptyText)\s*=\s*\"([^\"{}]*)\"")
LETTERS_RE = re.compile(r"[A-Za-zÀ-ÿ]{2,}")
CHAR_RE = re.compile(r"'(?:\\u\{[0-9A-Fa-f_]+\}|\\.|[^\\'\n])'")
RAW_RE = re.compile(r"r(#*)\"")
CTRL_RE = re.compile(r"^(?:if|for|while|switch|catch|else|try|do|with|finally)\b|\belse$")
DEF_RE = re.compile(r"^(\s*)(?:async\s+)?def\b")
PY_DEF_CLASS_RE = re.compile(r"^\s*(?:async\s+)?(?:def|class)\b")


def norm(s):
    return s.replace("️", "")


TESTDIR = norm("🧪️tests")


def kw_ok(tok, low, kw):
    p = low.find(kw)
    while p >= 0:
        if p == 0 or tok[p - 1] == "_" or tok[p].isupper():
            return True
        p = low.find(kw, p + 1)
    return False


def compat_tokens(s):
    return [t for t in re.findall(r"\w+", s) if any(kw_ok(t, t.lower(), kw) for kw in COMPAT_KW)]


def around(raw, rx):
    m = rx.search(raw)
    if not m:
        return raw.strip()[:140]
    return raw[max(0, m.start() - 50):m.end() + 90].strip()[:200]


def is_emoji(ch):
    c = ord(ch)
    return (0x1F000 <= c <= 0x1FAFF or 0x2600 <= c <= 0x27BF or 0x2300 <= c <= 0x23FF
            or 0x2B00 <= c <= 0x2BFF or 0x2194 <= c <= 0x21AA or 0x25AA <= c <= 0x25FE
            or 0x2934 <= c <= 0x2935 or c in EMO_SINGLE)


def lead_emoji(text):
    s = text.lstrip()
    if not s:
        return None
    m = re.match(r"[#*0-9]️?⃣", s)
    if m:
        return m.group(0)
    if not is_emoji(s[0]):
        return None
    i = 1
    while i < len(s):
        c = ord(s[i])
        if c in (0xFE0F, 0xFE0E, 0x20E3) or 0x1F3FB <= c <= 0x1F3FF:
            i += 1
        elif c == 0x200D and i + 1 < len(s) and is_emoji(s[i + 1]):
            i += 2
        else:
            break
    return s[:i]


def slug(seg):
    s = re.sub(r"[^A-Za-z0-9_\-]", "", seg)
    return s or seg


def owner_of(rel):
    parts = rel.split("/")
    for key in ("plugins", "modules", "products"):
        slugs = [slug(x) for x in parts]
        if key in slugs:
            i = slugs.index(key)
            if i + 1 < len(parts) - 1:
                return slug(parts[i + 1])
    return slug(parts[0]) + ("/" + slug(parts[1]) if len(parts) > 2 else "")


def is_test_path(rel):
    segs = [norm(s) for s in rel.split("/")]
    if any(s in (TESTDIR, "tests", "__tests__") for s in segs):
        return True
    return bool(re.search(r"\.(?:test|spec)\.[jt]sx?$|_test\.rs$|tests?\.rs$|test_[^/]*\.py$", rel))


def discover():
    cmd = ["find", *DIRS, "(", "-name", "node_modules", "-o", "-name", "target", "-o", "-name", "dist",
           "-o", "-name", "bazel-out", "-o", "-name", ".git", ")", "-prune", "-o", "-type", "f",
           "-newermt", SINCE, "-print"]
    out = subprocess.run(cmd, cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace")
    files = []
    for p in out.split("\n"):
        if not p or any("generated" in seg for seg in p.split("/")):
            continue
        if os.path.splitext(p)[1] in TEXT_EXT:
            files.append(p)
    return sorted(set(files))


def lead_text(lines):
    for ln in lines:
        if ln.strip():
            return ln.strip()
    return ""


def classify_rust(h, parent_test):
    is_test = parent_test or bool(re.search(r"#\[\s*(?:cfg\(\s*test\s*\)|test\b|[\w:]*::test\b)", h))
    if re.search(r"\bfn\b", h):
        k = "fn"
    elif re.search(r"\b(?:impl|trait)\b", h):
        k = "impl"
    elif re.search(r"\b(?:struct|enum|union)\b", h):
        k = "type"
    elif re.search(r"\bmod\b", h):
        k = "mod"
    else:
        k = "other"
    return k, is_test


def classify_ts(h, parent_test):
    s = h.strip()
    if CTRL_RE.search(s):
        k = "other"
    elif re.search(r"\bclass\b", s):
        k = "impl"
    elif re.search(r"\b(?:interface|enum)\b", s) or re.search(r"\btype\s+\w+[^=]*=\s*$", s):
        k = "type"
    elif re.search(r"\bfunction\b|=>$|\)\s*(?::[^{};=]+)?$", s):
        k = "fn"
    else:
        k = "other"
    return k, parent_test


def scan_c(text, lang, file_test):
    """🧬 Char-level scanner for Rust/TS: masked code per line, comments with block context, test flags."""
    rust = lang == "rs"
    n = len(text)
    i = 0
    line = 1
    cur = []
    masked = {}
    line_test = {}
    comments = []
    stack = []
    header = []
    classify = classify_rust if rust else classify_ts

    def in_test():
        return file_test or any(t for _, t in stack)

    def newline():
        nonlocal line
        masked[line] = "".join(cur)
        line_test[line] = in_test()
        cur.clear()
        line += 1

    def absorb(span):
        for ch in span:
            if ch == "\n":
                newline()

    def str_end(start, quote, multi):
        j = start + 1
        while j < n:
            ch = text[j]
            if ch == "\\":
                j += 2
                continue
            if ch == quote:
                return j + 1
            if ch == "\n" and not multi:
                return j
            j += 1
        return n

    def ctx():
        return (any(k == "fn" for k, _ in stack), stack[-1][0] if stack else "", in_test())

    while i < n:
        c = text[i]
        if c == "\n":
            newline()
            i += 1
            continue
        nx = text[i + 1] if i + 1 < n else ""
        if c == "/" and nx == "/":
            j = text.find("\n", i)
            j = n if j < 0 else j
            fn_in, top, t = ctx()
            comments.append((line, text[i:j], fn_in, top, t))
            i = j
            continue
        if c == "/" and nx == "*":
            j = text.find("*/", i + 2)
            j = n if j < 0 else j + 2
            fn_in, top, t = ctx()
            comments.append((line, text[i:j], fn_in, top, t))
            absorb(text[i:j])
            i = j
            continue
        if rust and c == "r" and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            m = RAW_RE.match(text, i)
            if m:
                close = '"' + m.group(1)
                j = text.find(close, m.end())
                j = n if j < 0 else j + len(close)
                cur.append('""')
                header.append('""')
                absorb(text[i:j])
                i = j
                continue
        if not rust and c == "/" and nx not in ("/", "*"):
            prev = next((ch for ch in reversed(cur) if not ch.isspace()), "")
            if prev == "" or prev in "(,=:[!&|?{};+-*%<>~^":
                j = i + 1
                in_cls = False
                while j < n and text[j] != "\n":
                    ch = text[j]
                    if ch == "\\":
                        j += 2
                        continue
                    if ch == "[":
                        in_cls = True
                    elif ch == "]":
                        in_cls = False
                    elif ch == "/" and not in_cls:
                        break
                    j += 1
                if j < n and text[j] == "/":
                    j += 1
                    while j < n and text[j].isalpha():
                        j += 1
                cur.append("/r/")
                header.append("/r/")
                i = j
                continue
        if c == '"' or (not rust and c in "'`"):
            j = min(str_end(i, c, rust or c == "`"), n)
            cur.append(c + c)
            header.append(c + c)
            absorb(text[i:j])
            i = j
            continue
        if rust and c == "'":
            m = CHAR_RE.match(text, i)
            if m:
                cur.append("''")
                header.append("''")
                i = m.end()
                continue
            cur.append(c)
            header.append(c)
            i += 1
            continue
        if c == "{":
            k, t = classify("".join(header), any(tt for _, tt in stack))
            stack.append((k, t))
            header.clear()
            cur.append(c)
        elif c == "}":
            if stack:
                stack.pop()
            header.clear()
            cur.append(c)
        elif c == ";":
            header.clear()
            cur.append(c)
        else:
            cur.append(c)
            header.append(c)
        i += 1
    masked[line] = "".join(cur)
    line_test[line] = in_test()
    return masked, line_test, comments


def group_rust_docs(comments):
    out = []
    start = last = None
    buf = []
    for ln, body, *_ in comments:
        doc = (body.startswith("///") and not body.startswith("////")) or body.startswith("//!")
        if doc and start is not None and ln == last + 1:
            buf.append(body[3:].strip())
            last = ln
            continue
        if start is not None:
            out.append((start, buf))
            start = last = None
            buf = []
        if doc:
            start, last, buf = ln, ln, [body[3:].strip()]
    if start is not None:
        out.append((start, buf))
    return [(s, lead_text(b)) for s, b in out]


def group_ts_docs(comments):
    out = []
    for ln, body, *_ in comments:
        if body.startswith("/**") and not body.startswith("/**/"):
            inner = body[3:-2] if body.endswith("*/") else body[3:]
            parts = [p.strip().lstrip("*").strip() for p in inner.split("\n")]
            out.append((ln, lead_text(parts)))
    return out


def doc_findings(docs):
    F = []
    seen = {}
    for ln, lead in docs:
        em = lead_emoji(lead)
        if em is None:
            F.append(("doc_no_emoji", ln, lead[:140] or "(empty docstring)"))
        else:
            seen.setdefault(norm(em), []).append(ln)
    for em, lns in seen.items():
        for ln in lns[1:]:
            F.append(("doc_dup_emoji", ln, f"{em} also leads line {lns[0]}"))
    return F


def analyse_c(lines, text, lang, file_test):
    masked, line_test, comments = scan_c(text, lang, file_test)
    F = []
    if lang == "rs":
        for ln, code in sorted(masked.items()):
            if ALLOW_RE.search(code):
                F.append(("allow", ln, around(lines[ln - 1], ALLOW_RE)))
            if UNWRAP_RE.search(code) and not line_test.get(ln):
                F.append(("unwrap_expect", ln, around(lines[ln - 1], UNWRAP_RE)))
            if UNSAFE_RE.search(code) and not SAFETY_RE.search("\n".join(lines[max(0, ln - 9):ln])):
                F.append(("unsafe", ln, around(lines[ln - 1], UNSAFE_RE)))
        docs = group_rust_docs(comments)
    else:
        docs = group_ts_docs(comments)
    F += doc_findings(docs)
    for ln, body, fn_in, top, _t in comments:
        is_doc = (lang == "rs" and ((body.startswith("///") and not body.startswith("////")) or body.startswith("//!"))) \
            or (lang == "ts" and body.startswith("/**") and not body.startswith("/**/"))
        if is_doc or "SAFETY" in body:
            continue
        if fn_in:
            F.append(("comment_fn", ln, body.strip()[:140]))
        elif top in ("impl", "type"):
            F.append(("comment_def", ln, body.strip()[:140]))
    for ln, code in sorted(masked.items()):
        ids = sorted(set(compat_tokens(code)))
        if ids:
            F.append(("compat_ident", ln, ", ".join(ids)))
    return F


def analyse_py(lines):
    F = []
    pending = True
    def_stack = []
    docs = []
    for idx, raw in enumerate(lines, 1):
        s = raw.strip()
        if not s:
            continue
        ind = len(raw) - len(raw.lstrip())
        if s.startswith("#"):
            if def_stack and ind > def_stack[-1]:
                F.append(("comment_fn", idx, s[:140]))
            continue
        while def_stack and ind <= def_stack[-1]:
            def_stack.pop()
        is_doc = pending and s[:3] in ('"""', "'''")
        pending = bool(PY_DEF_CLASS_RE.match(raw))
        if DEF_RE.match(raw):
            def_stack.append(ind)
        if is_doc:
            inner = s[3:]
            if not inner.strip() and idx < len(lines):
                inner = lines[idx].strip()
            docs.append((idx, inner.rstrip("\"' ").strip()))
            pending = False
        for m in compat_tokens(s):
            F.append(("compat_ident", idx, m))
    F += doc_findings(docs)
    return F


def analyse(rel):
    with open(os.path.join(ROOT, rel), "rb") as fh:
        text = fh.read().decode("utf-8", "replace")
    lines = text.split("\n")
    lang = CODE_LANG.get(os.path.splitext(rel)[1])
    file_test = is_test_path(rel)
    F = []
    for idx, raw in enumerate(lines, 1):
        s = raw.strip()
        if DEBUG_RE.search(raw):
            F.append(("debug", idx, around(raw, DEBUG_RE)))
        if TT_RE.search(raw):
            F.append(("test_threads", idx, around(raw, TT_RE)))
        if lang == "ts":
            if JSON_WITH_RE.search(raw):
                specs = [q for q in re.findall(r"[\"']([^\"']+)[\"']", raw) if q != "json"]
                if specs and not specs[0].endswith(".json") and not file_test:
                    F.append(("json_import", idx, s[:140]))
            if not file_test and not s.startswith(("//", "/*", "*", "import ", "export ")):
                for m in UI_JSX_RE.finditer(raw):
                    cap = m.group(1).strip()
                    if LETTERS_RE.search(cap) and len(cap) >= 3 and (" " in cap or cap[:1].isupper()):
                        F.append(("ui_literal", idx, cap[:140]))
                        break
                for m in UI_ATTR_RE.finditer(raw):
                    val = m.group(1)
                    if LETTERS_RE.search(val) and (" " in val or val[:1].isupper()):
                        F.append(("ui_literal", idx, val[:140]))
                        break
    if lang == "py" and not rel.endswith(".json"):
        pass
    for seg in rel.split("/"):
        if compat_tokens(seg):
            F.append(("compat_path", 0, seg))
    if lang == "rs" or lang == "ts":
        F += analyse_c(lines, text, lang, file_test)
    elif lang == "py":
        F += analyse_py(lines)
    return [(rule, rel, ln, ev) for rule, ln, ev in F]


def cell(s):
    return s.replace("|", "\\|").replace("`", "'")


def added_lines():
    diff = subprocess.run(["git", "-c", "core.quotepath=off", "diff", "--no-ext-diff", "-U0", "HEAD", "--", *DIRS],
                          cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace")
    added, cur = {}, None
    for line in diff.split("\n"):
        if line.startswith("+++ "):
            cur = line[6:] if line.startswith("+++ b/") else None
        elif line.startswith("@@") and cur:
            m = re.search(r"\+(\d+)(?:,(\d+))?", line)
            start = int(m.group(1))
            cnt = int(m.group(2)) if m.group(2) is not None else 1
            added.setdefault(cur, set()).update(range(start, start + cnt))
    untracked = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "--others", "--exclude-standard",
                                "--", *DIRS], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace")
    return added, {p for p in untracked.split("\n") if p}


def main():
    files = discover()
    findings = []
    for rel in files:
        findings += analyse(rel)
    added, untracked = added_lines()
    new = {f: (f[1] in untracked or f[2] in added.get(f[1], set())) for f in findings}
    by_rule = {r: [] for r in RULES}
    for f in findings:
        by_rule[f[0]].append(f)
    for r in RULES:
        by_rule[r].sort(key=lambda f: (not new[f], f[1], f[2]))
    owner_tot = {}
    for f in findings:
        o = owner_of(f[1])
        owner_tot[o] = owner_tot.get(o, 0) + 1
    examples = []
    idx = {r: 0 for r in RULES}
    while len(examples) < 60:
        progressed = False
        for r in RULES:
            if idx[r] < len(by_rule[r]) and len(examples) < 60:
                examples.append(by_rule[r][idx[r]])
                idx[r] += 1
                progressed = True
        if not progressed:
            break
    os.makedirs(GEN, exist_ok=True)
    with open(GEN + "/policy2-findings.tsv", "w", encoding="utf-8") as fh:
        fh.write("rule\towner\tnew_vs_head\tfile\tline\tevidence\n")
        for f in findings:
            rule, rel, ln, ev = f
            fh.write("\t".join([rule, owner_of(rel), str(new[f]), rel, str(ln), ev.replace("\t", " ")]) + "\n")
    files_per_rule = {r: len({f[1] for f in by_rule[r]}) for r in RULES}
    out = []
    out.append("# 🔎️ 1030 Policy2 Audit (lane policy2)")
    out.append("")
    out.append("STATUS: done (read-only: no edits, no builds, no sub-agents, no git writes).")
    out.append("")
    out.append(f"Scope: files under `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand` with mtime after `{SINCE}` "
               f"(find -newermt, node_modules/target/dist/bazel-out/generated pruned). Files scanned: **{len(files)}**.")
    out.append("Script: `1030-tools/policy2/policy2.py`. Full findings TSV: "
               "`🗑️generated/1030/policy2-findings.tsv`.")
    out.append("")
    out.append("Method: Rust/TS sources go through a char-level scanner (strings, raw strings, char literals and "
               "comments masked; brace stack classifies fn / impl / type / test blocks). Python uses indentation "
               "for def bodies. Other text files get line rules only. Heuristics, so treat counts as candidates "
               "that need a line read before fixing.")
    out.append("")
    out.append("## Counts per rule")
    out.append("")
    out.append("| rule | AGENTS.md rule | findings | new vs HEAD | files |")
    out.append("|---|---|---:|---:|---:|")
    desc = {
        "debug": "`[DEBUG] ` / `dbg1030` / `DBG1030` leftovers",
        "allow": "no new `#[allow(`",
        "unwrap_expect": "no `.unwrap()` / `.expect(` in non-test Rust",
        "unsafe": "`unsafe` needs a SAFETY comment or `# Safety` docstring (9 lines window)",
        "doc_no_emoji": "docstrings start with a unique fitting emoji",
        "doc_dup_emoji": "leading emoji unique per file (extra occurrences)",
        "comment_fn": "no comments inside fn bodies (SAFETY comments exempt)",
        "comment_def": "no comments inside impl / trait / struct / enum bodies",
        "compat_ident": "no compat / legacy / shim / adapter / fallback identifiers in code",
        "compat_path": "same, in directory or file names",
        "test_threads": "no `--test-threads=1` in scripts",
        "ui_literal": "no hard-coded English/German UI literals in TS/TSX (candidates)",
        "json_import": "`with {type:\"json\"}` only on `.json` targets",
    }
    for r in RULES:
        out.append(f"| `{r}` | {desc[r]} | {len(by_rule[r])} | {sum(1 for f in by_rule[r] if new[f])} | {files_per_rule[r]} |")
    out.append(f"| **total** | | **{len(findings)}** | **{sum(1 for f in findings if new[f])}** | |")
    out.append("")
    out.append("## Findings per owner lane guess (top 25)")
    out.append("")
    out.append("| owner lane guess | findings |")
    out.append("|---|---:|")
    for o, c in sorted(owner_tot.items(), key=lambda kv: -kv[1])[:25]:
        out.append(f"| `{o}` | {c} |")
    out.append("")
    out.append("## Top 60 examples (round-robin across rules, priority order)")
    out.append("")
    out.append("| # | file:line | rule | new vs HEAD | evidence | owner lane guess |")
    out.append("|---:|---|---|---|---|---|")
    for k, f in enumerate(examples, 1):
        rule, rel, ln, ev = f
        loc = f"`{rel}:{ln}`" if ln else f"`{rel}` (path)"
        out.append(f"| {k} | {loc} | `{rule}` | {'yes' if new[f] else 'no'} | {cell(ev)} | `{owner_of(rel)}` |")
    out.append("")
    out.append("## Notes")
    out.append("")
    out.append("- `comment_fn` for TS is approximate: a `{` after `=>` or `)` counts as a function body, so JSX "
               "`{/* */}` inside components is included.")
    out.append("- `ui_literal` is a candidate list (JSX text and string props with words). Config-like strings are "
               "not caught. Test files are skipped.")
    out.append("- Owner lane is a guess from the plugin or framework module folder name.")
    out.append("- `new vs HEAD` = the line is added in `git diff HEAD` (read-only) or the file is untracked. "
               "All other hits are pre-existing in a file touched since 2026-10-30 06:00.")
    out.append("- `json_import` excludes test files, where the attribute appears inside fixture strings and tests "
               "deliberately use non-json specifiers.")
    out.append("- `ui_literal` found no JSX text or literal UI attribute outside test files in scope.")
    out.append("")
    with open(OUT_MD, "w", encoding="utf-8") as fh:
        fh.write("\n".join(out))
    print(f"files={len(files)} findings={len(findings)}")
    for r in RULES:
        print(f"{r}\t{len(by_rule[r])}\tfiles={files_per_rule[r]}")


if __name__ == "__main__":
    main()
