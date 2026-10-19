"""rg-plug helpers: top-level Rust item splitter, workspace + manifest registration."""
import json, os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"

def rd(p):
    return open(p if p.startswith("/") else ROOT + "/" + p, encoding="utf8").read()

import hashlib
BK = C + "/🗑️generated/rgplug-x/backup"

def backup(p):
    os.makedirs(BK, exist_ok=True)
    led = BK + "/ledger.json"
    L = json.load(open(led)) if os.path.exists(led) else {}
    rel = rel_to_root(p)
    if rel in L:
        return
    h = hashlib.sha1(rel.encode()).hexdigest()
    if os.path.exists(p):
        open(BK + "/" + h + ".orig", "wb").write(open(p, "rb").read())
        L[rel] = h
    else:
        L[rel] = None
    json.dump(L, open(led, "w"), ensure_ascii=False, indent=1)

def wr(p, s):
    p = p if p.startswith("/") else ROOT + "/" + p
    backup(p)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf8").write(s)

def top_items(src):
    """Split Rust source into top-level chunks: list of (kind, name, text). Each chunk = leading attrs/docs/blank + item up to `;` or matching `}` at depth 0."""
    out = []
    i = 0
    n = len(src)
    start = 0
    depth = 0
    line_start = 0
    j = 0
    chunk_start = 0
    in_str = False
    in_line = False
    in_block = 0
    raw_hashes = None
    item_started = False
    k = 0
    while k < n:
        c = src[k]
        nx = src[k + 1] if k + 1 < n else ""
        if in_line:
            if c == "\n":
                in_line = False
            k += 1
            continue
        if in_block:
            if c == "*" and nx == "/":
                in_block -= 1
                k += 2
                continue
            if c == "/" and nx == "*":
                in_block += 1
                k += 2
                continue
            k += 1
            continue
        if raw_hashes is not None:
            end = '"' + "#" * raw_hashes
            if src.startswith(end, k):
                k += len(end)
                raw_hashes = None
                continue
            k += 1
            continue
        if in_str:
            if c == "\\":
                k += 2
                continue
            if c == '"':
                in_str = False
            k += 1
            continue
        if c == "/" and nx == "/":
            in_line = True
            k += 2
            continue
        if c == "/" and nx == "*":
            in_block = 1
            k += 2
            continue
        m = re.match(r'r(#*)"', src[k:k + 12])
        if m and (k == 0 or not (src[k - 1].isalnum() or src[k - 1] == "_")):
            raw_hashes = len(m.group(1))
            k += len(m.group(0))
            continue
        if c == '"':
            in_str = True
            k += 1
            continue
        if c == "'":
            m2 = re.match(r"'(?:\\.[^']*|[^\\'])'", src[k:k + 12])
            if m2:
                k += len(m2.group(0))
                continue
            k += 1
            continue
        if c in "{([":
            depth += 1
        elif c in "})]":
            depth -= 1
            if depth == 0 and c == "}":
                if re.match(r"\s*;", src[k + 1:k + 40]) or re.match(r"\s*(as\s|,|where\b)", src[k + 1:k + 40]) and False:
                    k += 1
                    continue
                end = k + 1
                # swallow trailing newline
                if src[end:end + 1] == "\n":
                    end += 1
                out.append(src[chunk_start:end])
                chunk_start = end
        elif c == ";" and depth == 0:
            end = k + 1
            if src[end:end + 1] == "\n":
                end += 1
            out.append(src[chunk_start:end])
            chunk_start = end
        k += 1
    if src[chunk_start:].strip():
        out.append(src[chunk_start:])
    elif src[chunk_start:]:
        out.append(src[chunk_start:])
    return out

def item_name(chunk):
    body = re.sub(r'(?m)^\s*(//[^\n]*|#!?\[[^\n]*\])\n', '', chunk)
    m = re.search(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(mod|fn|struct|enum|trait|type|const|static|use|macro_rules!|impl|extern\s+crate)\s*([A-Za-z0-9_:<>{}, ]*)', body)
    return (m.group(1), m.group(2).strip()) if m else (None, None)

def cargo_ident(name):
    return name.replace("-", "_")

def use_root(chunk):
    m = re.match(r'\s*(?:#\[[^\n]*\]\s*)*pub\s+use\s+([A-Za-z0-9_]+)::', chunk)
    return m.group(1) if m else None

def use_symbols(chunk):
    m = re.search(r'pub\s+use\s+[A-Za-z0-9_]+::(?:\{([^}]*)\}|([A-Za-z0-9_]+))', chunk)
    if not m:
        return []
    body = m.group(1) if m.group(1) is not None else m.group(2)
    return [x.strip() for x in body.split(",") if x.strip()]

def mod_name(chunk):
    m = re.search(r'(?m)^\s*(?:pub\s+)?mod\s+([A-Za-z0-9_]+)\s*\{', chunk)
    return m.group(1) if m else None

def split_flat_root(src, move_mods):
    """Split a flat mutation-family root. Returns (old_text, new_body, moved_symbols, preamble)."""
    items = top_items(src)
    pre, old_body, new_body = [], [], []
    syms = []
    seen_mod = False
    for it in items:
        mn = mod_name(it)
        ur = use_root(it)
        if mn is not None and '#[path = "."]' in it:
            seen_mod = True
            if mn in move_mods:
                new_body.append(it)
                syms.append(mn)
            else:
                old_body.append(it)
        elif ur is not None and seen_mod:
            if ur in move_mods:
                new_body.append(it)
                syms.extend(use_symbols(it))
            else:
                old_body.append(it)
        elif not seen_mod:
            pre.append(it)
        else:
            old_body.append(it)
    return "".join(pre), "".join(old_body), "".join(new_body), syms

def prefix_paths(text, prefix):
    return re.sub(r'(#\[path = ")((?!\.")[^"]*)"', lambda m: m.group(1) + prefix + m.group(2) + '"', text)

# ---------- manifests and workspaces ----------
def rel_to_root(p):
    return p[len(ROOT) + 1:] if p.startswith(ROOT + "/") else p

def sec_bounds(text, header):
    m = re.search(r'(?m)^\[' + re.escape(header) + r'\]\s*$', text)
    if not m:
        return None
    n = re.search(r'(?m)^\[', text[m.end():])
    return (m.end(), m.end() + n.start() if n else len(text))

def dep_key_re(key):
    return re.compile(r'(?m)^' + re.escape(key) + r'\s*=')

def add_dep_line(manifest_path, section, key, line):
    """Insert `line` (full dependency line) into [section] in sorted position unless the key exists. Returns True if changed."""
    t = rd(manifest_path)
    b = sec_bounds(t, section)
    if b is None:
        return False
    body = t[b[0]:b[1]]
    if dep_key_re(key).search(body):
        return False
    lines = body.split("\n")
    idx = None
    last_key = None
    for i, l in enumerate(lines):
        m = re.match(r'([A-Za-z0-9_.\-"\']+)\s*=', l)
        if m:
            last_key = i
            if m.group(1) > key and idx is None:
                idx = i
    if idx is None:
        idx = (last_key + 1) if last_key is not None else 1
    lines.insert(idx, line)
    wr(manifest_path, t[:b[0]] + "\n".join(lines) + t[b[1]:])
    return True

def remove_dep_line(manifest_path, section, key):
    t = rd(manifest_path)
    b = sec_bounds(t, section)
    if b is None:
        return False
    body = t[b[0]:b[1]]
    new = re.sub(r'(?m)^' + re.escape(key) + r'\s*=[^\n]*\n', '', body)
    if new == body:
        return False
    wr(manifest_path, t[:b[0]] + new + t[b[1]:])
    return True

def ws_add_member(ws, member_rel):
    t = rd(ws)
    m = re.search(r'(?ms)^members\s*=\s*\[\n(.*?)\n\]', t)
    rows = m.group(1).split("\n")
    entry = '    "%s",' % member_rel
    if entry in rows:
        return False
    keys = [r.strip().strip(",").strip('"') for r in rows]
    pos = len(rows)
    for i, k in enumerate(keys):
        if k > member_rel:
            pos = i
            break
    rows.insert(pos, entry)
    wr(ws, t[:m.start(1)] + "\n".join(rows) + t[m.end(1):])
    return True

def ws_remove_member(ws, member_rel):
    t = rd(ws)
    entry = '    "%s",\n' % member_rel
    if entry not in t:
        return False
    wr(ws, t.replace(entry, "", 1))
    return True

def ws_add_dep(ws, name, path_rel, after=None):
    t = rd(ws)
    hdr = "[workspace.dependencies.%s]" % name
    if hdr in t:
        return False
    block = hdr + '\npath = "%s"\n\n' % path_rel
    if after:
        m = re.search(r'(?m)^\[workspace\.dependencies\.' + re.escape(after) + r'\]\n(?:[^\[\n][^\n]*\n)*\n?', t)
        if m:
            wr(ws, t[:m.end()] + block + t[m.end():])
            return True
    wr(ws, t.rstrip("\n") + "\n\n" + block.rstrip("\n") + "\n")
    return True

def ws_remove_dep(ws, name):
    t = rd(ws)
    m = re.search(r'(?m)^\[workspace\.dependencies\.' + re.escape(name) + r'\]\n(?:[^\[\n][^\n]*\n)*\n?', t)
    if not m:
        return False
    wr(ws, t[:m.start()] + t[m.end():])
    return True

def script_add(script, after, new):
    t = rd(script)
    old = '  "%s",\n' % after
    if old not in t or ('  "%s",\n' % new) in t:
        return False
    keys = re.findall(r'(?m)^  "(semio-[a-z0-9\-]+)",$', t)
    # insert in sorted position among package list
    names = sorted(set(keys + [new]))
    i = names.index(new)
    if i + 1 < len(names):
        nxt = names[i + 1]
        t = t.replace('  "%s",\n' % nxt, '  "%s",\n  "%s",\n' % (new, nxt), 1)
    else:
        prev = names[i - 1]
        t = t.replace('  "%s",\n' % prev, '  "%s",\n  "%s",\n' % (prev, new), 1)
    wr(script, t)
    return True

def manifest_for(old_manifest_text, new_name, description, lib_path):
    t = old_manifest_text
    t = re.sub(r'(?m)^name = "[^"]+"', 'name = "%s"' % new_name, t, count=1)
    t = re.sub(r'(?m)^description = "[^"]*"', 'description = "%s"' % description, t, count=1)
    t = re.sub(r'(?m)^(\[lib\]\s*\n(?:[^\[\n][^\n]*\n)*?)path = "[^"]*"', lambda m: m.group(1) + 'path = "%s"' % lib_path, t, count=1)
    return t

def prune_manifest(manifest_text, used_text, dev_text=None, keep=()):
    """Drop dependency lines whose ident is not mentioned in used_text (dev section: dev_text or used_text)."""
    out = []
    section = None
    for line in manifest_text.split("\n"):
        m = re.match(r'\[([^\]]+)\]\s*$', line)
        if m:
            section = m.group(1)
            out.append(line)
            continue
        if section in ("dependencies", "dev-dependencies", "build-dependencies"):
            km = re.match(r'([A-Za-z0-9_\-]+)\s*=', line)
            if km:
                key = km.group(1)
                ident = key.replace("-", "_")
                text = (dev_text if (section == "dev-dependencies" and dev_text is not None) else used_text)
                if key not in keep and not re.search(r'\b' + re.escape(ident) + r'\b', text):
                    continue
        out.append(line)
    return "\n".join(out)

def closure(manifest_rel):
    """Union test-cfg closure over all target roots of one manifest: list of {p,l,b}."""
    tmp = "/tmp/rgp-closure-%d.json" % os.getpid()
    r = subprocess.run(["bun", C + "/rgplug-tools/allclosure.ts", tmp, manifest_rel], cwd=ROOT, capture_output=True, text=True)
    d = json.load(open(tmp))
    os.remove(tmp)
    return d[manifest_rel]

def closure_text(files):
    s = []
    for f in files:
        if f["p"].endswith(".rs"):
            try:
                s.append(rd(f["p"]))
            except Exception:
                pass
    return "\n".join(s)

def run_owners(scope, write):
    r = subprocess.run(["bun", C + "/rgplug-tools/owners.ts", scope, "1" if write else "0"], cwd=ROOT, capture_output=True, text=True)
    return r.stdout + r.stderr

# ---------- repointing importers ----------
def repoint_text(text, old_alias, new_alias, moved, old_crate_ident, new_crate_ident, chain=""):
    """Re-point `old_alias::[chain::]SYM` paths for SYM in moved to new_alias. Returns (new_text, count, flags)."""
    moved = set(moved)
    count = 0
    flags = []
    ch = (re.escape(chain) + "::") if chain else ""
    chain_s = (chain + "::") if chain else ""
    def split_use(m):
        nonlocal count
        head, prefix, body, tail = m.group(1), m.group(2), m.group(3), m.group(4)
        if "{" in body:
            flags.append("nested-use-list")
            return m.group(0)
        entries = [e.strip() for e in body.replace("\n", " ").split(",") if e.strip()]
        keep, mv = [], []
        for e in entries:
            base = re.split(r"\s+as\s+|::", e)[0].strip()
            (mv if base in moved else keep).append(e)
        if not mv:
            return m.group(0)
        count += len(mv)
        out = []
        if keep:
            out.append("%suse %s%s::%s{%s}%s" % (head, prefix, old_alias, chain_s, ", ".join(keep), tail))
        out.append("%suse %s%s::%s{%s}%s" % (head, prefix, new_alias, chain_s, ", ".join(mv), tail))
        return "\n".join(out)
    pat = re.compile(r'((?:^|(?<=\n))[ \t]*(?:#\[[^\n]*\]\s*)?(?:pub(?:\([a-z]+\))?\s+)?)use\s+((?:crate::|super::|self::|::)?)' + re.escape(old_alias) + r'::' + ch + r'\{([^{}]*)\}(;)')
    text = pat.sub(split_use, text)
    def one(m):
        nonlocal count
        sym = m.group(2)
        if sym in moved:
            count += 1
            return m.group(1) + new_alias + "::" + (m.group(0)[len(m.group(1)) + len(old_alias) + 2:])
        return m.group(0)
    text = re.sub(r'((?<![A-Za-z0-9_]))' + re.escape(old_alias) + r'::' + ch + r'([A-Za-z0-9_]+)', one, text)
    new_extern = "extern crate %s as %s;" % (new_crate_ident, new_alias)
    old_extern_re = re.compile(r'(?m)^([ \t]*)extern crate ' + re.escape(old_crate_ident) + r' as ' + re.escape(old_alias) + r';\n')
    uses_new = re.search(r'(?<![A-Za-z0-9_])' + re.escape(new_alias) + r'::', text) is not None
    if uses_new and new_alias != new_crate_ident and new_extern not in text:
        mo = old_extern_re.search(text)
        if mo:
            text = text[:mo.end()] + mo.group(1) + new_extern + "\n" + text[mo.end():]
        else:
            flags.append("no-old-extern")
    return text, count, flags

def rebase_manifest(text, extra_up):
    """Prefix `extra_up` levels of ../ to relative path values (workspace=, path=) that start with '..'."""
    pre = "../" * extra_up
    def f(m):
        return m.group(1) + pre + m.group(2) + '"'
    text = re.sub(r'(?m)^(workspace = ")(\.\.[^"]*)"', f, text)
    text = re.sub(r'(path = ")(\.\.[^"]*)"', f, text)
    return text

def block_bounds(src, open_idx):
    """Given index of '{', return index just after the matching '}' (string/comment aware enough for our sources)."""
    depth = 0
    k = open_idx
    n = len(src)
    in_str = False
    while k < n:
        c = src[k]
        if in_str:
            if c == "\\":
                k += 2
                continue
            if c == '"':
                in_str = False
        else:
            if c == '"':
                in_str = True
            elif c == "/" and src[k + 1:k + 2] == "/":
                k = src.index("\n", k)
                continue
            elif c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    return k + 1
        k += 1
    raise ValueError("unbalanced")

def split_nested_root(src, chain_mod, move_mods):
    """Root with nested wrapper modules ending in `pub mod <chain_mod> { <mod chunks> }`.
    Returns (head, tail, old_chunks, moved_chunks_by_name) where head ends right after the `{` of chain_mod."""
    m = re.search(r'pub mod ' + re.escape(chain_mod) + r' \{\n', src)
    start = m.end()
    end = block_bounds(src, m.end() - 2) - 1
    inner = src[start:end]
    chunks = top_items(inner)
    keep, moved = [], {}
    for ch in chunks:
        mn = mod_name(ch)
        if mn in move_mods:
            moved[mn] = ch
        else:
            keep.append(ch)
    return src[:start], src[end:], "".join(keep), moved

# ---------- upward-reference analysis ----------
ITEM_DEF = re.compile(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+|async\s+|const\s+)*(fn|struct|enum|trait|type|const|static|mod|macro_rules!|union)\s+([A-Za-z_][A-Za-z0-9_]*)')

def defined_names(src):
    """Top-level item names defined in src (depth 0 only via top_items)."""
    out = {}
    for ch in top_items(src):
        body = re.sub(r'(?m)^\s*(//[^\n]*|#!?\[[^\n]*\])\n', '', ch)
        m = ITEM_DEF.match(body.lstrip("\n"))
        if m:
            out[m.group(2)] = m.group(1)
    return out

def idents(src):
    s = re.sub(r'//[^\n]*', '', src)
    s = re.sub(r'"(?:\\.|[^"\\])*"', '""', s)
    return set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', s))

def use_names(use_stmt):
    """Names introduced by a `use ...;` statement (leaf idents / aliases)."""
    t = use_stmt.strip().rstrip(";")
    t = re.sub(r'^(pub(\([^)]*\))?\s+)?use\s+', '', t)
    names = []
    def walk(s):
        s = s.strip()
        if "{" in s:
            i = s.index("{")
            inner = s[i + 1:s.rindex("}")]
            depth = 0
            part = ""
            for ch in inner:
                if ch == "{": depth += 1
                if ch == "}": depth -= 1
                if ch == "," and depth == 0:
                    walk(part); part = ""
                else:
                    part += ch
            if part.strip(): walk(part)
        else:
            m = re.search(r'\s+as\s+([A-Za-z0-9_]+)$', s)
            if m: names.append(m.group(1))
            else:
                leaf = s.split("::")[-1].strip()
                if leaf not in ("self", "*"): names.append(leaf)
                elif leaf == "self": names.append(s.split("::")[-2])
    walk(t)
    return names
