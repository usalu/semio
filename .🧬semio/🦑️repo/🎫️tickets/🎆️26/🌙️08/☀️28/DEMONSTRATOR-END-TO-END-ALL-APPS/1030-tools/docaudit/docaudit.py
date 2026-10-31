#!/usr/bin/env python3
"""📜 Read-only docs audit over recently changed sources (docstrings, emoji lead, body comments, DEBUG markers)."""

import collections
import os
import re
import subprocess
import sys
import time

ROOT = "/Users/ueli/Documents/semio"
TICKET = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
OUT_MD = TICKET + "/1030-audit-docs.md"
OUT_TSV = TICKET + "/🗑️generated/1030/docaudit-findings.tsv"
DIRS = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"]
EXTS = (".rs", ".ts", ".tsx", ".py")
WINDOW_S = 72 * 3600
SKIP = ("node_modules/", "bazel-out/", "dist/", "🗑️generated/", "target/")
FE0F = 0xFE0F
ZWJ = 0x200D
EMOJI_SINGLE = {0x00A9, 0x00AE, 0x203C, 0x2049, 0x2122, 0x2139, 0x3030, 0x303D, 0x3297, 0x3299, 0x24C2}
PRIORITY = {"debug": 0, "non_emoji_doc": 1, "fn_body": 2, "impl_body": 3, "dup_emoji": 4, "other_block": 5}
RUST_CHAR = re.compile(r"'(?:\\.|[^'\\])'")


def norm(s):
    return s.replace("️", "")


def is_emoji(ch):
    c = ord(ch)
    return (
        0x1F000 <= c <= 0x1FAFF
        or 0x2600 <= c <= 0x27BF
        or 0x2300 <= c <= 0x23FF
        or 0x2B00 <= c <= 0x2BFF
        or 0x2194 <= c <= 0x21AA
        or 0x25AA <= c <= 0x25FE
        or 0x2934 <= c <= 0x2935
        or c in EMOJI_SINGLE
    )


def lead_emoji(text):
    s = text.lstrip()
    keycap = re.match(r"[#*0-9]\ufe0f?\u20e3", s)
    if keycap:
        return keycap.group(0)
    if not s or not is_emoji(s[0]):
        return None
    i = 1
    while i < len(s):
        c = ord(s[i])
        if c in (FE0F, 0xFE0E, 0x20E3) or 0x1F3FB <= c <= 0x1F3FF:
            i += 1
        elif c == ZWJ and i + 1 < len(s) and is_emoji(s[i + 1]):
            i += 2
        else:
            break
    return s[:i]


def changed_files():
    out = subprocess.run(
        ["git", "-c", "core.quotepath=off", "status", "--porcelain", "-z", "--untracked-files=all", "--"] + DIRS,
        cwd=ROOT, capture_output=True, check=True,
    ).stdout.decode("utf-8")
    toks = out.split("\0")
    paths = []
    i = 0
    while i < len(toks):
        t = toks[i]
        i += 1
        if len(t) < 4:
            continue
        xy, p = t[:2], t[3:]
        if "R" in xy or "C" in xy:
            i += 1
        paths.append(p)
    now = time.time()
    dirs = {norm(d) for d in DIRS}
    keep = []
    for p in paths:
        if norm(p.split("/", 1)[0]) not in dirs or not p.endswith(EXTS):
            continue
        if any(s in p for s in SKIP):
            continue
        full = os.path.join(ROOT, p)
        if not os.path.isfile(full) or now - os.path.getmtime(full) > WINDOW_S:
            continue
        keep.append(p)
    return sorted(set(keep))


def py_scan(lines):
    docs, comments = [], []
    stack = []
    prev = None
    i = 0
    n = len(lines)
    while i < n:
        raw = lines[i]
        s = raw.strip()
        if not s:
            i += 1
            continue
        ind = len(raw) - len(raw.lstrip())
        if s.startswith("#"):
            if stack and ind > stack[-1][0]:
                comments.append((i + 1, "fn_body" if stack[-1][1] == "def" else "impl_body", s))
            i += 1
            continue
        while stack and ind <= stack[-1][0]:
            stack.pop()
        opener = None
        for pre in ('r"""', "r'''", '"""', "'''"):
            if s.startswith(pre):
                opener = pre
                break
        if opener:
            q = opener[-3:]
            is_doc = prev is None or (prev.endswith(":") and re.match(r"(async\s+)?(def|class)\b", prev))
            body = s[len(opener):]
            if not body.strip() and i + 1 < n:
                j = i + 1
                while j < n and not lines[j].strip():
                    j += 1
                text = lines[j].strip() if j < n else ""
            else:
                text = body
            if is_doc:
                docs.append((i + 1, text))
            if q not in body:
                i += 1
                while i < n and q not in lines[i]:
                    i += 1
            prev = s
            i += 1
            continue
        m = re.match(r"(async\s+)?(def|class)\b", s)
        if m:
            stack.append((ind, m.group(2)))
        prev = s
        i += 1
    return docs, comments


def ts_header_kind(h, ext):
    if ext == ".rs":
        if re.search(r"\bfn\b", h):
            return "fn"
        if re.search(r"\b(impl|trait)\b", h):
            return "impl"
        return "other"
    if re.search(r"\b(class|interface|namespace|module)\b", h):
        return "impl"
    if re.search(r"\b(if|for|while|switch|catch)\b", h):
        return "other"
    if re.search(r"\bfunction\b|=>", h) or re.search(r"\)\s*(:[^{]*)?$", h):
        return "fn"
    return "other"


def skip_str(raw, p, q):
    p += 1
    while p < len(raw) and raw[p] != q:
        p += 2 if raw[p] == "\\" else 1
    return p + 1


def brace_scan(lines, ext):
    comments = []
    stack = []
    pending = ""
    in_block = False
    for idx, raw in enumerate(lines):
        lineno = idx + 1
        if in_block:
            end = raw.find("*/")
            if end < 0:
                continue
            raw = raw[end + 2:]
            in_block = False
        s = raw.strip()
        if s.startswith("//") and not s.startswith("///") and not s.startswith("//!"):
            if stack:
                kind = "fn_body" if "fn" in stack else ("impl_body" if stack[-1] == "impl" else "other_block")
                comments.append((lineno, kind, s))
            continue
        p = 0
        while p < len(raw):
            ch = raw[p]
            two = raw[p:p + 2]
            if two == "//":
                break
            if two == "/*":
                end = raw.find("*/", p + 2)
                if end < 0:
                    in_block = True
                    break
                p = end + 2
                continue
            if ch == "`" and ext != ".rs" or ch == '"':
                p = skip_str(raw, p, ch)
                continue
            if ch == "'":
                if ext == ".rs":
                    mm = RUST_CHAR.match(raw, p)
                    p = mm.end() if mm else p + 1
                else:
                    p = skip_str(raw, p, ch)
                continue
            if ch == "{":
                stack.append(ts_header_kind(pending, ext))
                pending = ""
            elif ch == "}":
                if stack:
                    stack.pop()
                pending = ""
            elif ch == ";":
                pending = ""
            else:
                pending += ch
            p += 1
        pending += " "
    return [], comments


def rust_or_ts_docs(lines, ext):
    docs = []
    i = 0
    n = len(lines)
    while i < n:
        s = lines[i].strip()
        if ext == ".rs" and (s.startswith("///") and not s.startswith("////") or s.startswith("//!")):
            start = i
            while i < n:
                t = lines[i].strip()
                if (t.startswith("///") and not t.startswith("////")) or t.startswith("//!"):
                    i += 1
                else:
                    break
            docs.append((start + 1, lines[start].strip()[3:]))
            continue
        if s.startswith("/**") and not s.startswith("/**/"):
            body = s[3:]
            content = body.strip().lstrip("*").strip()
            j = i
            if not content and "*/" not in body:
                while j + 1 < n:
                    j += 1
                    c = lines[j].strip()
                    if c.startswith("*/"):
                        break
                    content = c.lstrip("*").strip()
                    if content:
                        break
            docs.append((i + 1, content))
            if "*/" not in body:
                while i < n and "*/" not in lines[i]:
                    i += 1
            i += 1
            continue
        i += 1
    return docs


def analyse(rel):
    with open(os.path.join(ROOT, rel), encoding="utf-8", errors="replace") as fh:
        lines = fh.read().split("\n")
    ext = os.path.splitext(rel)[1]
    if ext == ".py":
        docs, comments = py_scan(lines)
    else:
        docs = rust_or_ts_docs(lines, ext)
        _, comments = brace_scan(lines, ext)
    findings = []
    leads = collections.Counter()
    lead_at = collections.defaultdict(list)
    non_emoji = 0
    for lineno, text in docs:
        lead = lead_emoji(text)
        if lead is None:
            non_emoji += 1
            snippet = text.strip() or "(empty)"
            findings.append((lineno, "non_emoji_doc", snippet))
        else:
            leads[lead] += 1
            lead_at[lead].append(lineno)
    dup = 0
    for lead, c in leads.items():
        if c > 1:
            dup += c - 1
            for ln in lead_at[lead][1:]:
                findings.append((ln, "dup_emoji", "duplicate leading emoji " + lead))
    counts = collections.Counter()
    safety = 0
    for lineno, kind, text in comments:
        counts[kind] += 1
        if text.startswith("// SAFETY:"):
            safety += 1
        findings.append((lineno, kind, text))
    debug = 0
    for idx, raw in enumerate(lines):
        if "[DEBUG]" in raw:
            debug += 1
            findings.append((idx + 1, "debug", raw.strip()))
    return {
        "file": rel,
        "docs": len(docs),
        "non_emoji": non_emoji,
        "dup": dup,
        "fn_body": counts["fn_body"],
        "impl_body": counts["impl_body"],
        "other_block": counts["other_block"],
        "safety": safety,
        "debug": debug,
        "findings": findings,
    }


def excerpt(text, width=110):
    t = " ".join(text.split()).replace("`", "'").replace("|", "\\|")
    return t if len(t) <= width else t[: width - 1] + "…"


def main():
    files = changed_files()
    results = [analyse(f) for f in files]
    rows = []
    for r in results:
        for lineno, cat, text in r["findings"]:
            rows.append((r["file"], lineno, cat, text.replace("\t", " ")))
    os.makedirs(os.path.dirname(OUT_TSV), exist_ok=True)
    with open(OUT_TSV, "w", encoding="utf-8") as fh:
        fh.write("file\tline\tcategory\ttext\n")
        for row in rows:
            fh.write("\t".join(str(x) for x in row) + "\n")

    totals = collections.Counter()
    for r in results:
        for k in ("docs", "non_emoji", "dup", "fn_body", "impl_body", "other_block", "safety", "debug"):
            totals[k] += r[k]

    file_total = {r["file"]: sum(r[k] for k in ("non_emoji", "dup", "fn_body", "impl_body", "other_block", "debug")) for r in results}
    prio = sorted(
        [row for row in rows if row[2] in PRIORITY],
        key=lambda row: (PRIORITY[row[2]], -file_total.get(row[0], 0), row[0], row[1]),
    )
    examples = []
    for cat in PRIORITY:
        per_file = collections.OrderedDict()
        for row in prio:
            if row[2] == cat:
                per_file.setdefault(row[0], []).append(row)
        while any(per_file.values()) and len(examples) < 50:
            for f in list(per_file):
                if per_file[f] and len(examples) < 50:
                    examples.append(per_file[f].pop(0))
        if len(examples) >= 50:
            break

    lines_md = []
    lines_md.append("# 📋 Docs Audit 1030 (docaudit lane)")
    lines_md.append("")
    lines_md.append("Read-only audit of `.rs`, `.ts`, `.tsx`, `.py` files under `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand` that are added or modified in the working tree (per `git status --porcelain`) with mtime within the last 72 h. Script: `1030-tools/docaudit/docaudit.py`. Raw findings: `🗑️generated/1030/docaudit-findings.tsv`.")
    lines_md.append("")
    lines_md.append(f"Files audited: **{len(results)}**. Docstrings found: **{totals['docs']}**. Non-emoji-first docstrings: **{totals['non_emoji']}**. Duplicate leading emoji (extra occurrences): **{totals['dup']}**. Comments in fn bodies: **{totals['fn_body']}**. Comments in impl/class bodies: **{totals['impl_body']}**. Comments in other blocks: **{totals['other_block']}**. Of all comments, `// SAFETY:` (not exempt): **{totals['safety']}**. `[DEBUG]` lines: **{totals['debug']}**.")
    lines_md.append("")
    lines_md.append("## Per-File Table")
    lines_md.append("")
    lines_md.append("| File | Docs | Non-emoji first | Dup emoji | Fn-body comments | Impl/class-body comments | Other-block comments | SAFETY (subset) | [DEBUG] |")
    lines_md.append("|---|---:|---:|---:|---:|---:|---:|---:|---:|")
    for r in results:
        lines_md.append(f"| `{r['file']}` | {r['docs']} | {r['non_emoji']} | {r['dup']} | {r['fn_body']} | {r['impl_body']} | {r['other_block']} | {r['safety']} | {r['debug']} |")
    lines_md.append("")
    lines_md.append("## Top 50 Examples (file:line)")
    lines_md.append("")
    lines_md.append("Ordered by category (DEBUG, non-emoji docstring, duplicate emoji, fn-body comment, impl/class-body comment, other-block comment), then round-robin across files so one file does not fill the table.")
    lines_md.append("")
    lines_md.append("| # | file:line | Category | Excerpt |")
    lines_md.append("|---:|---|---|---|")
    for i, (f, ln, cat, text) in enumerate(examples, 1):
        lines_md.append(f"| {i} | `{f}:{ln}` | {cat} | {excerpt(text)} |")
    lines_md.append("")
    lines_md.append("## Method And Limits")
    lines_md.append("")
    lines_md.append("- Emoji test: first non-space codepoint in the emoji blocks (U+1F000-1FAFF, U+2600-27BF, U+2300-23FF, U+2B00-2BFF, U+2194-21AA, U+25AA-25FE, plus common single codepoints). Keycap sequences (`#️⃣`, `1️⃣`) count as emoji starts.")
    lines_md.append("- Docstrings: Rust `///` and `//!` groups (first line of the group), TS/Rust `/** */` blocks (first content line), Python `\"\"\"`/`'''` strings that open right after a `def`/`class` header or as the first statement of a module.")
    lines_md.append("- Duplicate leading emoji: the same leading emoji sequence starting more than one docstring within one file; counts extra occurrences.")
    lines_md.append("- Body comments: `//` line comments only (not `///`, `//!`); Rust/TS brace tracking skips strings and char literals, classifies a block by its header (`fn`, `impl`/`trait`/`class`/`interface`, other). Python `#` comments indented deeper than the enclosing `def`/`class`. `/* */` block comments are not counted.")
    lines_md.append("- Heuristic brace parsing is approximate for TS template literals and regex literals; treat counts as an upper-bound audit list, not a parser-exact result.")
    sys.stdout.write("\n".join(lines_md) + "\n")
    with open(OUT_MD, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines_md) + "\n")


if __name__ == "__main__":
    main()
