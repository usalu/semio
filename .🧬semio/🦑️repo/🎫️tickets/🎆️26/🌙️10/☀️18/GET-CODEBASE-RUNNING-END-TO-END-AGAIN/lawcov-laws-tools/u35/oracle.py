UI_ROOTS = ["🧰framework/🔨modules/🖱ui/", "🧰framework/🛍products/💻os/🔨modules/📺renderer/", "🧰framework/🛍products/💻os/🖥host/", "✏s/🔌plugins/"]
BRIDGE = ["🧰framework/🛍products/💻os/🔨modules/🔌plugin/🖥host/🧵shard/🧵executor/🦀.rs"]
SANCTIONED = ["🧰framework/🔨modules/⏳async/"]
ROOTS = ["✏s", "🧰framework", "🌎hub"]
UI_PATTERNS = [("blocking-bridge", r"\bblock_on\s*\("), ("blocking-bridge", r"\brun_blocking\s*\("), ("sync-fs", r"\bstd::fs::"), ("sync-net", r"\bstd::net::"), ("sync-net", r"\breqwest::blocking\b"), ("sync-clipboard", r"\barboard::"), ("sync-process", r"\bstd::process::Command\b"), ("sync-db", r"\brusqlite::"), ("sync-db", r"\bsqlx::")]
RUNTIME_PATTERNS = [("thread-pool", r"\b(?:std::)?thread::spawn\s*\("), ("thread-pool", r"\b(?:std::)?thread::Builder\b"), ("thread-pool", r"\b(?:std::)?thread::scope\s*\("), ("thread-pool", r"\bWorkerPool::new\s*\("), ("thread-pool", r"\brayon::"), ("thread-pool", r"\btokio::runtime::Builder\b")]


def is_runtime_source(p):
    return any(p == r or p.startswith(r + "/") for r in ROOTS) and "/🧫fixtures/" not in p and "/🧪tests/" not in p and "/tests/" not in p and "/benches/" not in p and "/examples/" not in p and not p.endswith("/build.rs")


def mask_comments(raw, depth):
    m = mask(raw)
    code, i = "", 0
    while i < len(m):
        pair = m[i:i + 2]
        if depth > 0:
            if pair == "/*":
                depth += 1
                i += 2
            elif pair == "*/":
                depth -= 1
                i += 2
            else:
                i += 1
            continue
        if pair == "//":
            break
        if pair == "/*":
            depth = 1
            i += 2
            continue
        code += m[i]
        i += 1
    return code, depth


def same_line_kind(item):
    depth = 0
    for ch in item:
        if ch in "([":
            depth += 1
        elif ch in ")]":
            if depth == 0:
                return "member"
            depth -= 1
        elif depth == 0 and ch == ",":
            return "member"
        elif depth == 0 and ch in "{;":
            return "item"
    return "item"


def cfg_test_item_spans(lines):
    spans, stack, pending, depth = [], [], None, 0
    for i, raw in enumerate(lines):
        co = code_only(raw)
        attr = re.search(r"#\[cfg\((?![^\]]*\bnot\s*\()[^\]]*\btest\b[^\]]*\)\]", co, A)
        if attr:
            pending = i + 1
        same = co[attr.end():] if pending == i + 1 and attr else ""
        if same.strip() and not re.match(r"^\s*#\[", same) and same_line_kind(same) == "member":
            pending = None
        item = "" if pending is None else (co if i + 1 > pending else same)
        if pending is not None and item.strip() and not re.match(r"^\s*#\[", item):
            if item.count("{") > 0:
                stack.append((pending, depth))
                pending = None
            elif re.search(r";\s*$", item):
                spans.append((pending, i + 1))
                pending = None
            elif re.search(r",\s*$", item):
                pending = None
        depth += co.count("{") - co.count("}")
        while stack and depth <= stack[-1][1]:
            start, _ = stack.pop()
            spans.append((start, i + 1))
    return spans


def scan_file(t, path, patterns):
    content = t.files[path]
    if not content:
        return []
    lines = re.split(r"\r?\n", content)
    spans = spans_of_mods(lines) + cfg_test_item_spans(lines)
    depth, out = 0, []
    for i, raw in enumerate(lines):
        code, depth = mask_comments(raw, depth)
        if in_spans(spans, i + 1):
            continue
        for cat, rx in patterns:
            if re.search(rx, code, A):
                out.append([cat, path, i + 1, raw.strip()[:200]])
    return out


def scan(t):
    out = []
    for p in t.walk(ROOTS, lambda p, n: n.endswith(".rs")):
        if not is_runtime_source(p):
            continue
        if any(p.startswith(r) for r in UI_ROOTS) or p in BRIDGE:
            out += scan_file(t, p, UI_PATTERNS)
        if not any(p.startswith(r) for r in SANCTIONED):
            out += scan_file(t, p, RUNTIME_PATTERNS)
    return sorted(out, key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


def run(c):
    if c["rule"] == "interactivityIsRuntimeSource":
        return is_runtime_source(c["args"][0])
    return scan(Tree(expand(c)))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
