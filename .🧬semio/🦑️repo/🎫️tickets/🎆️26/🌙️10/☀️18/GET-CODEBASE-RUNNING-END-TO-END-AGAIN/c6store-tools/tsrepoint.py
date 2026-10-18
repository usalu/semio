"""📝 C6STORE source-text reader re-pointer: on a given line of a TS law, the call that reads a split Rust file as text (`readFileSync(<path>, "utf8")`, `fs.readFileSync(...)`, `await Bun.file(<path>).text()`, `Bun.file(<path>).text()`) whose argument contains the needle becomes `readModuleTreeSource(<path>)` (the module tree is the text the unsplit file held); the import of the module-tree reader is added where the file lacks it.
usage as a library: repoint(file, [(line, needle, [occurrence])...], apply) -> list of messages."""
import os, re
R = "/Users/ueli/Documents/semio/"
IMPORT = 'import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";'
HEAD = re.compile(r"(?<![\w.])(?:(?:fs|node_fs|nodeFs)\.)?readFileSync\(|(?<![\w])(?:await\s+)?Bun\.file\(")

def balanced(text, open_idx):
    depth = 0
    i = open_idx
    q = None
    while i < len(text):
        c = text[i]
        if q:
            if c == "\\":
                i += 2
                continue
            if c == q:
                q = None
        else:
            if c in "\"'`":
                q = c
            elif c in "([{":
                depth += 1
            elif c in ")]}":
                depth -= 1
                if depth == 0:
                    return i
        i += 1
    raise ValueError("unbalanced")

def split_top(args):
    parts, depth, q, cur = [], 0, None, ""
    i = 0
    while i < len(args):
        c = args[i]
        if q:
            cur += c
            if c == "\\":
                cur += args[i + 1]
                i += 2
                continue
            if c == q:
                q = None
        else:
            if c in "\"'`":
                q = c
                cur += c
            elif c in "([{":
                depth += 1
                cur += c
            elif c in ")]}":
                depth -= 1
                cur += c
            elif c == "," and depth == 0:
                parts.append(cur)
                cur = ""
            else:
                cur += c
        i += 1
    parts.append(cur)
    return parts

def repoint_line(line, needle, occurrence=1, multi=False):
    out, pos, n, changed = "", 0, 0, 0
    for m in HEAD.finditer(line):
        start = m.start()
        if start < pos:
            continue
        open_idx = m.end() - 1
        close = balanced(line, open_idx)
        args = line[open_idx + 1:close]
        if needle not in args:
            continue
        n += 1
        if not multi and n != occurrence:
            continue
        head = m.group(0)
        if "Bun.file(" in head:
            rest = line[close + 1:]
            mt = re.match(r"\s*\.text\(\)", rest)
            if not mt:
                continue
            new = "readModuleTreeSource(" + args.strip() + ")"
            end = close + 1 + mt.end()
            prefix_len = start
            out += line[pos:prefix_len] + new
            pos = end
        else:
            parts = split_top(args)
            if len(parts) == 2 and re.fullmatch(r"\s*[\"'`]utf-?8[\"'`]\s*", parts[1]):
                new = "readModuleTreeSource(" + parts[0].strip() + ")"
            elif len(parts) == 1:
                continue
            else:
                continue
            out += line[pos:start] + new
            pos = close + 1
        changed += 1
    out += line[pos:]
    return out, changed

def ensure_import(text):
    if re.search(r"^import\b[^\n]*\breadModuleTreeSource\b", text, flags=re.M) or re.search(r"^\s*import\s*\{[^}]*\breadModuleTreeSource\b[^}]*\}\s*from", text, flags=re.M):
        return text, False
    lines = text.split("\n")
    idx = -1
    for i, l in enumerate(lines):
        if re.match(r"^import\b", l) and l.rstrip().endswith(";"):
            idx = i
    if idx < 0:
        lines.insert(0, IMPORT)
    else:
        lines.insert(idx + 1, IMPORT)
    return "\n".join(lines), True

def repoint(file, sites, apply=False):
    p = R + file
    text = open(p, encoding="utf-8").read()
    lines = text.split("\n")
    msgs = []
    for site in sites:
        ln, needle = site[0], site[1]
        occ = site[2] if len(site) > 2 else 1
        multi = occ == "all"
        new, ch = repoint_line(lines[ln - 1], needle, 1 if multi else occ, multi)
        if ch == 0:
            msgs.append(f"MISS {file.split('/')[-3]}/{file.split('/')[-2]}:{ln} {needle[-40:]}")
            continue
        lines[ln - 1] = new
        msgs.append(f"ok   {file.split('/')[-3]}/{file.split('/')[-2]}:{ln} x{ch} {needle[-40:]}")
    text = "\n".join(lines)
    text, added = ensure_import(text)
    if added:
        msgs.append("     + import readModuleTreeSource")
    if apply and not any(m.startswith("MISS") for m in msgs):
        open(p, "w", encoding="utf-8").write(text)
    return msgs
