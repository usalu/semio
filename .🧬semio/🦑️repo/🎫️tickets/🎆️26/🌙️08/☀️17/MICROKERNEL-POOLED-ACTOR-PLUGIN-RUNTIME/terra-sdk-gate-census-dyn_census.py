#!/usr/bin/env python3
"""🔬 Repo-wide first-party dyn census, comment-stripped, path-explicit (R10-safe: no shell globbing over emoji paths)."""
import os
import re
import sys
import json

ROOT = "/Users/ueli/Documents/semio"
AREAS = ["🧰️framework", "✏️s", "🌎️hub"]
EXCLUDE_DIRS = {".🧬semio", "target", "node_modules", ".git"}

# also exclude any dir named exactly "🎯️target" or starting with "🎯️target" (ticket-local target dirs)
def is_excluded_dir(name):
    if name in EXCLUDE_DIRS:
        return True
    if name.startswith("🎯️target"):
        return True
    return False

def strip_comments_and_strings(src):
    """Remove // line comments, /* */ block comments, and string/char literal contents,
    to avoid false matches inside strings/comments. Keeps line structure (newlines preserved)."""
    out = []
    i = 0
    n = len(src)
    in_line_comment = False
    in_block_comment = False
    in_string = False
    in_raw_string = False
    raw_hashes = 0
    in_char = False
    while i < n:
        c = src[i]
        if in_line_comment:
            if c == '\n':
                in_line_comment = False
                out.append(c)
            else:
                out.append(' ')
            i += 1
            continue
        if in_block_comment:
            if c == '*' and i+1 < n and src[i+1] == '/':
                in_block_comment = False
                out.append('  ')
                i += 2
                continue
            out.append(' ' if c != '\n' else '\n')
            i += 1
            continue
        if in_string:
            if c == '\\' and i+1 < n:
                # 🩹️ preserve a real newline consumed as the second half of an escape pair
                # (backslash-newline line continuation inside a string literal) — replacing
                # it with two spaces silently ate one line per continuation and desynced
                # every subsequent line number for the rest of the file.
                out.append(' ' + (' ' if src[i+1] != '\n' else '\n'))
                i += 2
                continue
            if c == '"':
                in_string = False
                out.append(' ')
                i += 1
                continue
            out.append(' ' if c != '\n' else '\n')
            i += 1
            continue
        if in_raw_string:
            if c == '"':
                # check trailing hashes match raw_hashes
                j = i+1
                h = 0
                while j < n and src[j] == '#':
                    h += 1
                    j += 1
                if h >= raw_hashes:
                    in_raw_string = False
                    out.append(' ' * (1 + h))
                    i = j
                    continue
            out.append(' ' if c != '\n' else '\n')
            i += 1
            continue
        # not in any special state
        # 🔒️ Single quotes are deliberately NEVER treated as entering a char-literal state:
        # a Rust char literal's payload is always 1 escape-sequence long and can never spell
        # `dyn`/`trait`, so passing `'` through unchanged (covering both char literals and
        # lifetimes like `'static`) is safe for this census and avoids a lifetime/char
        # ambiguity that previously corrupted downstream parser state on `&'static` fields.
        if c == '/' and i+1 < n and src[i+1] == '/':
            in_line_comment = True
            out.append('  ')
            i += 2
            continue
        if c == '/' and i+1 < n and src[i+1] == '*':
            in_block_comment = True
            out.append('  ')
            i += 2
            continue
        if c == '"':
            in_string = True
            out.append(' ')
            i += 1
            continue
        out.append(c)
        i += 1
    return ''.join(out)

def handle_raw_strings_pre(src):
    """Pre-mask raw strings r"...", r#"..."#, r##"..."## etc, replacing content with spaces, before generic strip."""
    # 🐛️ Fixed: without the `(?<![A-Za-z0-9_])` lookbehind this matched the tail of any English
    # word ending in "r" immediately before a doc-comment quote (e.g. `` `"timer"` ``), which
    # misdetected a raw-string open and swallowed everything up to the next `"` as fake raw-string
    # content — silently corrupting downstream trait/dyn detection for the rest of the block.
    pattern = re.compile(r'(?<![A-Za-z0-9_])r(#*)"')
    out = []
    i = 0
    n = len(src)
    while i < n:
        m = pattern.match(src, i)
        if m:
            hashes = m.group(1)
            start = m.end()
            closer = '"' + hashes
            end = src.find(closer, start)
            if end == -1:
                out.append(src[i:])
                break
            content = src[i:end + len(closer)]
            out.append(''.join(' ' if ch != '\n' else '\n' for ch in content))
            i = end + len(closer)
            continue
        out.append(src[i])
        i += 1
    return ''.join(out)

TRAIT_DECL_RE = re.compile(r'\btrait\s+([A-Za-z_][A-Za-z0-9_]*)')
ASYNC_FN_RE = re.compile(r'\basync\s+fn\s+')
PLAIN_FN_RE = re.compile(r'(?<!async\s)\bfn\s+')
TAG_RE = re.compile(r'🚫️async:\s*(E\d)')

def area_of(path):
    for a in AREAS:
        marker = f"/{a}/"
        if marker in path or path.endswith(f"/{a}"):
            return a
    return None

def walk_rust_files():
    files = []
    for area in AREAS:
        area_path = os.path.join(ROOT, area)
        if not os.path.isdir(area_path):
            continue
        for dirpath, dirnames, filenames in os.walk(area_path):
            dirnames[:] = [d for d in dirnames if not is_excluded_dir(d)]
            for fn in filenames:
                if fn.endswith(".rs"):
                    files.append((area, os.path.join(dirpath, fn)))
    return files

def main():
    files = walk_rust_files()
    sys.stderr.write(f"Found {len(files)} .rs files under {AREAS}\n")

    trait_names = set()
    trait_decl_locations = {}
    file_stripped = {}  # path -> stripped src (cache for reuse in dyn + fn passes)
    file_raw = {}

    for area, path in files:
        try:
            with open(path, "r", encoding="utf-8", errors="replace") as f:
                src = f.read()
        except Exception as e:
            sys.stderr.write(f"ERROR reading {path}: {e}\n")
            continue
        file_raw[path] = src
        pre = handle_raw_strings_pre(src)
        stripped = strip_comments_and_strings(pre)
        file_stripped[path] = stripped
        for m in TRAIT_DECL_RE.finditer(stripped):
            name = m.group(1)
            trait_names.add(name)
            trait_decl_locations.setdefault(name, []).append(path)

    sys.stderr.write(f"Found {len(trait_names)} distinct first-party trait names declared.\n")

    STD_LANG = {"Future", "Fn", "FnMut", "FnOnce", "Any", "Error"}
    # ensure std/lang names aren't counted as first-party even if a first-party trait shares name (unlikely) - exclude explicitly
    trait_names_for_dyn = trait_names - STD_LANG

    dyn_re_template = r'\bdyn\s+({names})\b'
    # build one big regex alternation is expensive for huge sets; instead do generic dyn scan then classify
    GENERIC_DYN_RE = re.compile(r'\bdyn\s+([A-Za-z_][A-Za-z0-9_:<>\'\., ]*?)\b')
    # Simpler: find `dyn IDENT` occurrences (first identifier after dyn, handling leading :: or module paths minimally)
    DYN_IDENT_RE = re.compile(r'\bdyn\s+(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Za-z_][A-Za-z0-9_]*)')

    dyn_counts_by_trait = {}
    dyn_counts_by_area_trait = {}
    std_lang_counts = {name: 0 for name in STD_LANG}
    std_lang_by_area = {}
    unknown_dyn_names = {}
    total_dyn_matches = 0

    dyn_sites = []  # (area, path, line_no, name)

    for area, path in files:
        stripped = file_stripped.get(path)
        if stripped is None:
            continue
        for lineno, line in enumerate(stripped.splitlines(), start=1):
            if 'dyn ' not in line and 'dyn\t' not in line:
                continue
            for m in DYN_IDENT_RE.finditer(line):
                name = m.group(1)
                total_dyn_matches += 1
                if name in STD_LANG:
                    std_lang_counts[name] += 1
                    std_lang_by_area.setdefault(area, {}).setdefault(name, 0)
                    std_lang_by_area[area][name] += 1
                elif name in trait_names_for_dyn:
                    dyn_counts_by_trait[name] = dyn_counts_by_trait.get(name, 0) + 1
                    dyn_counts_by_area_trait.setdefault(area, {}).setdefault(name, 0)
                    dyn_counts_by_area_trait[area][name] += 1
                    dyn_sites.append((area, path, lineno, name))
                else:
                    unknown_dyn_names[name] = unknown_dyn_names.get(name, 0) + 1

    first_party_dyn_total = sum(dyn_counts_by_trait.values())
    std_lang_total = sum(std_lang_counts.values())

    # async-literal census
    async_counts_by_area = {a: 0 for a in AREAS}
    plain_fn_counts_by_area = {a: 0 for a in AREAS}
    tag_counts_by_class = {}
    tag_total = 0

    for area, path in files:
        stripped = file_stripped.get(path)
        raw = file_raw.get(path)
        if stripped is None:
            continue
        async_n = len(ASYNC_FN_RE.findall(stripped))
        # plain fn: total 'fn ' minus async fn (since our regex above uses negative lookbehind imperfectly for multi-space)
        all_fn = len(re.findall(r'\bfn\s+', stripped))
        plain_n = all_fn - async_n
        async_counts_by_area[area] += async_n
        plain_fn_counts_by_area[area] += plain_n
        for m in TAG_RE.finditer(raw):
            cls = m.group(1)
            tag_counts_by_class[cls] = tag_counts_by_class.get(cls, 0) + 1
            tag_total += 1

    total_async = sum(async_counts_by_area.values())
    total_plain = sum(plain_fn_counts_by_area.values())
    ratio = total_async / (total_async + total_plain) * 100 if (total_async+total_plain) else 0

    result = {
        "files_scanned": len(files),
        "trait_names_declared": len(trait_names),
        "first_party_dyn_total": first_party_dyn_total,
        "first_party_dyn_by_area": {a: sum(v.values()) for a, v in dyn_counts_by_area_trait.items()},
        "first_party_dyn_by_trait": dict(sorted(dyn_counts_by_trait.items(), key=lambda kv: -kv[1])),
        "first_party_dyn_by_area_trait": dyn_counts_by_area_trait,
        "std_lang_dyn_total": std_lang_total,
        "std_lang_dyn_counts": std_lang_counts,
        "std_lang_dyn_by_area": std_lang_by_area,
        "unknown_dyn_names_sample": dict(sorted(unknown_dyn_names.items(), key=lambda kv: -kv[1])[:40]),
        "async_fn_by_area": async_counts_by_area,
        "plain_fn_by_area": plain_fn_counts_by_area,
        "total_async_fn": total_async,
        "total_plain_fn": total_plain,
        "async_ratio_pct": round(ratio, 2),
        "tag_counts_by_class": tag_counts_by_class,
        "tag_total": tag_total,
    }
    with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/dyn_census_result.json", "w", encoding="utf-8") as f:
        json.dump(result, f, ensure_ascii=False, indent=2)

    # also dump dyn_sites for later spot check use
    with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/dyn_sites.json", "w", encoding="utf-8") as f:
        json.dump(dyn_sites, f, ensure_ascii=False, indent=2)

    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == "__main__":
    main()
