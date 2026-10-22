RULES = {
    "R8": "leaf-mutable-base", "R9": "leaf-applies-diff", "R10": "leaf-between", "R11": "diff-derived-inverse", "R12": "base-clone-diff", "R13": "whole-state-diff",
    "R14": "restore-inverse", "R15": "inverse-sum-law-untested", "R16": "outcome-apply-to", "R17": "host-apply", "R18": "row-emitter",
}
MUTATIONS = "🧬mutations"
INFRA = {"🚪io", "🎮prepare", "📦codec"}
MACHINERY = {"apply", "absorb", "retire_cold", "retire_projection", "is_empty"}
LANES = {"✏editor", "🌉apply", "📬preparation"}
EXCLUDED = {"🚪io", "🔮oracles", "🧪tests", "🧫fixtures", "🧬schema", "🧬mutations", "🧬mutation-support"}
FRAMEWORK = "🧰framework/🛍products/💻os/"
PLUGIN_ROOT = re.compile(r"^✏s/🔌plugins/[^/]+/🗿artifacts/[^/]+/🦀\.rs$")
IMPL_PROBE = re.compile(r"\bimpl\b[^{;]*\bMutation(?:Kind)?\s*<", A)
DIFF_IMPL_PROBE = re.compile(r"\bimpl\b[^{;]*\bMutationDiff\s*<", A)
DIFFISH_FN_PROBE = re.compile(r"\bfn\s+\w*(?:diff|inverse|negative|negation|replacing|state_after)\w*", A)
DIFFISH_NAME = re.compile(r"(?:^|_)(?:diff|inverse|negative|negation|replacing)(?:_|$)|state_after", A)
INVERSEISH = re.compile(r"inverse|negative|negation")
PAYLOAD_PARAM = re.compile(r"^(?:payload|mutation|op|edit|self)$")
APPLY = re.compile(r"(?:\.|::)apply\s*\(|\bapply_diff\s*\(|\bApplyCapability\b", A)
BETWEEN = re.compile(r"(?<!\w)(?:\w*between\w*|\w*_replacing|\w*_state_diff|state_after\w*|value_diff\w*)\s*(?:::\s*<[^>(]*>)?\s*\(", A)
GEOMETRY = re.compile(r"^(?:\w+_)?(?:angle|distance|gap)_between\b", A)
SIMULATOR = re.compile(r"(?<!\w)(?:negative|state_after\w*|inverse_of|value_diff\w*)\s*\(|\.negat(?:e|ion)\w*\s*\(", A)
DIFF_CALL = re.compile(r"(?<!\w)diff\s*\(", A)
INVERSE_HELPER = re.compile(r"(?<![\w.])(\w*_inverse|\w*inverse_\w*)\s*\(", A)
DIFF_ARGUMENT = re.compile(r"\b\w*[dD]iff\w*\b|\boutcome\b", A)
RESTORE = re.compile(r"\b(?:SetSnapshot|PatchSnapshot|ReplaceDocument|ReplaceSnapshot|ReplaceConfig|ReplaceState|SetDocument|Restore\w*)\b|\b(?:Self|[A-Z]\w*)::\w*Snapshot\b|(?<!\blet\s)(?<!\bmut\s)\b\w+_(?:state|content|snapshot|document)\s*:(?!:)", A)
EPHEMERAL_MUTATION = re.compile(r"^\w*(?:Transient|Presence)Mutation$", A)
EPHEMERAL_SETTER = re.compile(r"^Self::\w*Snapshot$", A)
APPLY_TO = re.compile(r"(?:\.|::)apply_to\s*\(", A)
TYPE_DIFF = re.compile(r"\btype\s+Diff\s*=\s*([^;]+);", A)
ROW_BANNED = re.compile(r"\b(?:commit_built|equals_base|sparse_rows)\b|::produced\s*\(|\.(?:extrude_faces|inset_faces|bevel|loop_cut|subdivide_faces|triangulate|mirror|decimate|flip_faces|merge_vertices|dissolve_edges|snap_vertices)_job(?:_owned)?\s*\(", A)
ROW_PROBE = re.compile(r"commit_built|equals_base|sparse_rows|::produced|_job(?:_owned)?\s*\(", A)
BEFORE_AFTER_CALL = re.compile(r"\b(\w*(?:diff|delta|patch)\w*)\s*\(\s*&\s*\w*(?:before|old|prev|previous|original)\w*\s*,\s*&\s*\w*(?:after|new|next|updated|edited)\w*\s*[,)]", A | re.I)
BEFORE_NAME = re.compile(r"^(?:\w*_)?(?:before|old|prev|previous|original)\w*$", A | re.I)
AFTER_NAME = re.compile(r"^(?:\w*_)?(?:after|new|next|updated|edited)\w*$", A | re.I)
TS_REF_ASSIGN = re.compile(r"\b(\w*(?:Catalog|Snapshot|Document|Archive)\w*Ref)\.current\s*=\s*([^;\n]+)", A)
TS_LOCAL_APPLY = re.compile(r"\bapply(?:Local|Optimistic)\w*\s*\(|\bapply\w*Mutation\w*Local\w*\s*\(", A)
TS_PROBE = re.compile(r"Ref\.current|localDocumentArchive|encodeDocumentArchiveBytes|restore\w*Archive", A)


def blank(text):
    return re.sub(r"[^\n]", " ", text)


def mask_rust(c):
    out, kept, i, n = [], 0, 0, len(c)

    def put(a, b):
        nonlocal kept, i
        out.append(c[kept:a])
        out.append(blank(c[a:b]))
        kept = i = b

    while i < n:
        ch = c[i]
        nxt = c[i + 1] if i + 1 < n else None
        if ch == "/" and nxt == "/":
            eol = c.find("\n", i)
            put(i, n if eol == -1 else eol)
        elif ch == "/" and nxt == "*":
            depth, cur = 1, i + 2
            while cur < n and depth > 0:
                pair = c[cur:cur + 2]
                if pair == "/*":
                    depth += 1
                    cur += 2
                elif pair == "*/":
                    depth -= 1
                    cur += 2
                else:
                    cur += 1
            put(i, cur)
        elif ch == '"':
            cur = i + 1
            while cur < n and c[cur] != '"':
                cur += 2 if c[cur] == "\\" else 1
            put(i, min(cur + 1, n))
        elif ch in "rb" and not re.match(r"\w", c[i - 1] if i > 0 else " ", A):
            raw = re.compile(r"b?r(#*)\"").match(c, i)
            if raw:
                term = '"' + raw.group(1)
                end = c.find(term, i + len(raw.group(0)))
                put(i, n if end == -1 else end + len(term))
            else:
                i += 1
        elif ch == "'":
            if nxt == "\\":
                end = c.find("'", i + 3)
                put(i, i + 1 if end == -1 else end + 1)
            elif i + 2 < n and c[i + 2] == "'":
                put(i, i + 3)
            else:
                i += 1
        else:
            i += 1
    return "".join(out) + c[kept:]


def brace_close(m):
    close, stack = {}, []
    for i, ch in enumerate(m):
        if ch == "{":
            stack.append(i)
        elif ch == "}" and stack:
            close[stack.pop()] = i
    return close


def blank_tests(m, close):
    out, kept = [], 0
    for t in re.finditer(r"#\[\s*(?:cfg\s*\(\s*test\s*\)|test)\s*\]", m):
        if t.start() < kept:
            continue
        cur = t.end()
        while cur < len(m) and m[cur] not in "{;":
            cur += 1
        end = close[cur] + 1 if cur < len(m) and m[cur] == "{" and cur in close else min(cur + 1, len(m))
        out.append(m[kept:t.start()])
        out.append(blank(m[t.start():end]))
        kept = end
    return "".join(out) + m[kept:]


def skip_angles(text, open_):
    depth = 0
    for i in range(open_, len(text)):
        ch = text[i]
        if ch == "<":
            depth += 1
        elif ch == ">" and text[i - 1] != "-":
            depth -= 1
            if depth == 0:
                return i + 1
    return len(text)


def paren_end(text, open_):
    depth = 0
    for i in range(open_, len(text)):
        if text[i] == "(":
            depth += 1
        elif text[i] == ")":
            depth -= 1
            if depth == 0:
                return i
    return len(text)


def split_top(text):
    parts, depth, start = [], 0, 0
    for i, ch in enumerate(text):
        if ch in "<([{":
            depth += 1
        elif ch in ")]}" or (ch == ">" and text[i - 1] != "-"):
            depth -= 1
        elif ch == "," and depth == 0:
            parts.append(text[start:i])
            start = i + 1
    parts.append(text[start:])
    return [p.strip() for p in parts if p.strip()]


def find_fns(code, close):
    fns = []
    for m in re.finditer(r"\bfn\s+([A-Za-z_]\w*)", code, A):
        cur = m.end()
        while cur < len(code) and code[cur].isspace():
            cur += 1
        if cur < len(code) and code[cur] == "<":
            cur = skip_angles(code, cur)
            while cur < len(code) and code[cur].isspace():
                cur += 1
        if cur >= len(code) or code[cur] != "(":
            continue
        pend = paren_end(code, cur)
        open_, depth = pend + 1, 0
        while open_ < len(code):
            ch = code[open_]
            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
            elif depth == 0 and ch in "{;":
                break
            open_ += 1
        if open_ >= len(code) or code[open_] != "{" or open_ not in close:
            continue
        params = split_top(code[cur + 1:pend])
        refs = []
        for p in params:
            named = re.match(r"^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*(&(?!\s*mut\b))?", p, A)
            if named and named.group(2) and not re.search(r"\w*(?:Diff|Delta|Patch|Mutation)\w*", p[p.index(":"):], A):
                refs.append(named.group(1))
        fns.append({"name": m.group(1), "start": m.start(), "body": open_, "end": close[open_], "refs": refs, "params": params})
    return fns


def find_impls(code, close):
    impls = []
    for m in re.finditer(r"\bimpl\b", code, A):
        before = m.start() - 1
        while before >= 0 and code[before].isspace():
            before -= 1
        if before >= 0 and code[before] not in ";}{]":
            continue
        open_ = m.start()
        while open_ < len(code) and code[open_] not in "{;":
            open_ += 1
        if open_ >= len(code) or code[open_] != "{" or open_ not in close:
            continue
        header = re.sub(r"\s+", " ", code[m.start() + 4:open_]).strip()
        cur = skip_angles(header, 0) if header[:1] == "<" else 0
        rest = header[cur:].lstrip()
        rest = rest[1:] if rest.startswith("!") else rest
        trait = re.match(r"^([\w:]+)\s*<", rest, A)
        kind, args, self_ = "other", [], ""
        if trait:
            name = trait.group(1).split("::")[-1]
            ao = len(trait.group(0)) - 1
            ae = skip_angles(rest, ao)
            args = split_top(rest[ao + 1:ae - 1])
            sm = re.match(r"^\s*for\s+(.+?)(?:\s+where\b.*)?$", rest[ae:], A)
            self_ = sm.group(1) if sm else ""
            kind = "mutation" if name in ("MutationKind", "Mutation") else "diff" if name == "MutationDiff" else "algebra" if name == "DiffAlgebra" else "outcome" if name == "MutationOutcome" and self_ == "" else "other"
        impls.append({"kind": kind, "args": args, "self": self_, "trait": re.search(r"(?:^|[\s>])for\s", rest) is not None, "start": m.start(), "body": open_, "end": close[open_]})
    return impls


def type_name(t):
    return re.sub(r"<.*$", "", t, flags=re.S).lstrip("& \t\n").strip().split("::")[-1].strip()


def preceded_by_fn(text, idx):
    return re.search(r"\bfn\s+\Z", text[:idx], A) is not None


def within(spans, idx):
    return any(a <= idx <= b for a, b in spans)


def breach(rule, path, line):
    return ["diff-only-%s-%s-%d" % (rule.lower(), path, line), "diff-only-mutation/" + RULES[rule], "high", line]


def leaf_file(path, content):
    if not path.endswith(".rs"):
        return []
    dirs = path.split("/")[:-1]
    under_mut, under_schema, under_support = MUTATIONS in dirs, "🧬schema" in dirs, "🧬mutation-support" in dirs
    test_path = any(d in ("🧪tests", "🧫fixtures") for d in dirs)
    infra = any(d in INFRA or d.endswith("-internals") for d in dirs)
    every = (not test_path) and (not infra) and (under_mut or under_support or (under_schema and any(d in ("🔺diff", "↩inverse") for d in dirs)))
    leaf = (not test_path) and (every or under_schema or IMPL_PROBE.search(content) or DIFF_IMPL_PROBE.search(content) or (path.startswith("✏s/") and "/🗿artifacts/" in path and DIFFISH_FN_PROBE.search(content)))
    apply_to = "apply_to" in content
    if not leaf and not apply_to:
        return []
    allm = mask_rust(content)
    close = brace_close(allm)
    seen, out = set(), []

    def add(rule, idx):
        line = content[:idx].count("\n") + 1
        if (rule, line) in seen:
            return
        seen.add((rule, line))
        out.append(breach(rule, path, line))

    if apply_to:
        for m in APPLY_TO.finditer(allm):
            add("R16", m.start())
        if "MutationOutcome" in allm:
            for imp in [i for i in find_impls(allm, close) if i["kind"] == "outcome"]:
                for m in re.finditer(r"\bfn\s+apply_to\b", allm[imp["body"]:imp["end"]], A):
                    add("R16", imp["body"] + m.start())
    if not leaf:
        return out
    code = blank_tests(allm, close)
    impls = find_impls(code, close)
    for imp in impls:
        if imp["kind"] == "diff" and len(imp["args"]) == 1 and type_name(imp["args"][0]) == type_name(imp["self"]):
            add("R13", imp["start"])
    mut_impls = [i for i in impls if i["kind"] == "mutation"]
    has_diff = any(i["kind"] == "diff" for i in impls)

    def innermost(idx):
        c = [i for i in impls if i["body"] <= idx <= i["end"]]
        return sorted(c, key=lambda i: -i["body"])[0] if c else None

    classified = []
    for fn in find_fns(code, close):
        impl = innermost(fn["start"])
        logic = impl is not None and impl["kind"] in ("mutation", "diff", "algebra")
        machinery = impl is not None and impl["kind"] in ("diff", "algebra") and fn["name"] in MACHINERY
        applier = (impl is not None and impl["kind"] == "diff" and fn["name"] == "apply") or (has_diff and not under_mut and "apply" in fn["name"]) or (not under_mut and "ApplyCapability" in code[fn["start"]:fn["body"]])
        exempt = machinery or applier or "between" in fn["name"] or (impl is not None and impl["kind"] == "other" and impl["trait"])
        classified.append((fn, applier, (not exempt) and (every or logic or (not infra and DIFFISH_NAME.search(fn["name"])))))
    scanned = [f for f, a, s in classified if s]
    appliers = [(f["start"], f["end"]) for f, a, s in classified if a]
    scanned_spans = [(f["start"], f["end"]) for f in scanned]

    def diff_side(fn):
        impl = innermost(fn["start"])
        return (impl is not None and impl["kind"] in ("diff", "algebra")) or (under_schema and not under_mut)

    for fn in scanned:
        builder = fn["name"] == "diff" or DIFFISH_NAME.search(fn["name"])
        rx = re.compile(r"&\s*mut\b") if builder else re.compile(r"&\s*mut\s+(?:\w+::)*\w*(?:Snapshot|Document|State)\b", A)
        for m in rx.finditer(code[fn["start"]:fn["end"] + 1]):
            add("R8", fn["start"] + m.start())
    everywhere = (not infra) and (under_mut or under_schema or under_support)
    no_use = re.sub(r"\buse\s[^;]*;", lambda m: blank(m.group(0)), code, flags=A)
    for m in APPLY.finditer(no_use):
        if m.group(0).startswith("apply_diff") and preceded_by_fn(no_use, m.start()):
            continue
        if not within(appliers, m.start()) and (everywhere or within(scanned_spans, m.start()) or within([(i["body"], i["end"]) for i in mut_impls], m.start())):
            add("R9", m.start())
    for m in BETWEEN.finditer(code):
        if preceded_by_fn(code, m.start()):
            continue
        if within(scanned_spans, m.start()) and not GEOMETRY.search(m.group(0)):
            add("R10", m.start())
    for fn in [f for f in scanned if INVERSEISH.search(f["name"])]:
        body = code[fn["body"]:fn["end"] + 1]
        for m in SIMULATOR.finditer(body):
            if not preceded_by_fn(body, m.start()):
                add("R11", fn["body"] + m.start())
        if not diff_side(fn):
            for m in DIFF_CALL.finditer(body):
                add("R11", fn["body"] + m.start())
            for m in INVERSE_HELPER.finditer(body):
                op = m.end() - 1
                args = body[op + 1:paren_end(body, op)]
                if re.search("diff", m.group(1), re.I) or DIFF_ARGUMENT.search(args):
                    add("R11", fn["body"] + m.start())
        owner = innermost(fn["start"])
        setter = owner is not None and owner["kind"] == "mutation" and EPHEMERAL_MUTATION.search(type_name(owner["self"]))
        for m in RESTORE.finditer(body):
            if not (setter and EPHEMERAL_SETTER.search(m.group(0))):
                add("R14", fn["body"] + m.start())
    for fn in scanned:
        roots = [r for r in fn["refs"] if not PAYLOAD_PARAM.search(r)]
        if not roots:
            continue
        body = code[fn["body"]:fn["end"] + 1]
        if "let mut" not in body:
            continue
        copy = re.compile(r"\blet\s+mut\s+\w+\s*(?::[^=;]*)?=\s*(?:\(\s*\*\s*)?(?:%s)\b[^;{}]*?(?:\.(?:clone|to_owned|to_vec)\s*\(\s*\)|\.iter\s*\(\s*\)\s*\.cloned\s*\(\s*\)\s*\.collect[^;]*)\s*\)?\s*;" % "|".join(roots), A)
        for m in copy.finditer(body):
            add("R12", fn["body"] + m.start())
    for imp in mut_impls:
        declared = TYPE_DIFF.search(code[imp["body"]:imp["end"]])
        if not declared:
            continue
        name = type_name(declared.group(1))
        if name == "Self" or name == type_name(imp["self"]) or (imp["args"] and name == type_name(imp["args"][0])):
            add("R13", imp["body"] + declared.start())
    return out


def row_emitter(path, content):
    if not path.endswith(".rs") or not ROW_PROBE.search(content):
        return []
    dirs = path.split("/")[:-1]
    if any(d in ("🧪tests", "🧫fixtures") for d in dirs):
        return []
    if not (MUTATIONS in dirs or ("🥽mesh" in dirs and "📝journal" in dirs)):
        return []
    masked = mask_rust(content)
    code = blank_tests(masked, brace_close(masked))
    seen, out = set(), []
    for m in ROW_BANNED.finditer(code):
        line = content[:m.start()].count("\n") + 1
        if line not in seen:
            seen.add(line)
            out.append(breach("R18", path, line))
    return out


def host_scope(path):
    dirs = path.split("/")[:-1]
    if any(d in EXCLUDED or d == "node_modules" for d in dirs):
        return None
    if path.endswith(".rs"):
        host = any(d == "🔨modules" and i + 1 < len(dirs) and dirs[i + 1] == "🏠host" for i, d in enumerate(dirs))
        return "rust" if host or any(d in LANES for d in dirs) or path.startswith(FRAMEWORK) or PLUGIN_ROOT.search(path) else None
    if re.search(r"\.tsx?$", path) and not re.search(r"\.(?:test|spec)\.tsx?$", path) and not path.endswith(".d.ts"):
        return "ts"
    return None


def host_apply(path, content):
    scope = host_scope(path)
    if scope is None:
        return []
    seen, out = set(), []

    def add(idx):
        line = content[:idx].count("\n") + 1
        if line not in seen:
            seen.add(line)
            out.append(breach("R17", path, line))

    if scope == "ts":
        if not TS_PROBE.search(content):
            return []
        code = re.sub(r"/\*[\s\S]*?\*/|//[^\n]*", lambda m: blank(m.group(0)), content)
        locals_ = set(m.group(1) for m in re.finditer(r"\b(?:const|let)\s+(\w+)\s*(?::[^=;\n]+)?=\s*(?:await\s+)?(?:\w+\.)*apply(?:Local|Optimistic)\w*\s*\(", code, A))
        for m in TS_REF_ASSIGN.finditer(code):
            rhs = m.group(2).strip()
            if TS_LOCAL_APPLY.search(rhs) or re.sub(r"\W.*$", "", rhs, flags=re.S) in locals_:
                add(m.start())
        return out
    allm = mask_rust(content)
    code = blank_tests(allm, brace_close(allm))
    framework = path.startswith(FRAMEWORK)
    for m in re.finditer(r"\bsnapshot_mut\b", code, A):
        add(m.start())
    if not framework:
        for m in BEFORE_AFTER_CALL.finditer(code):
            add(m.start())
    for fn in find_fns(code, brace_close(allm)):
        if re.search(r"diff|delta|patch", fn["name"]):
            refs = []
            for p in fn["params"]:
                if framework and not re.search(r"Snapshot|Document", p):
                    continue
                r = re.match(r"^(?:mut\s+)?(\w+)\s*:\s*&(?!\s*mut\b)", p, A)
                if r:
                    refs.append(r.group(1))
            if any(BEFORE_NAME.search(r) for r in refs) and any(AFTER_NAME.search(r) for r in refs):
                add(fn["start"])
    return out


def inverse_fn_spans(content):
    blanked = re.sub(r"//[^\n]*|/\*[\s\S]*?\*/|r#*\"[\s\S]*?\"#*|\"(?:\\[\s\S]|[^\"\\])*\"|'(?:\\.|[^'\\])'", lambda m: " " * len(m.group(0)), content)
    spans = []
    for head in re.finditer(r"\bfn\s+(\w+)\b[^{;]*\{", blanked, A):
        start = head.end() - 1
        depth, end = 0, start
        while end < len(blanked):
            if blanked[end] == "{":
                depth += 1
            elif blanked[end] == "}":
                depth -= 1
                if depth == 0:
                    break
            end += 1
        spans.append((head.group(1), start, end))
    return spans


CALL = "assert_mutation_inverse_sum_law"


def sum_reach(content):
    if CALL not in content:
        return ""
    spans = inverse_fn_spans(content)
    body = lambda s: content[s[1]:s[2] + 1]
    seeds = {s[0] for s in spans if CALL in body(s)}
    if not seeds:
        return ""
    reached = set(seeds)
    for _ in range(2):
        for s in spans:
            b = body(s)
            if s[0] in reached:
                for c in re.finditer(r"\b(\w+)\s*\(", b, A):
                    if any(o[0] == c.group(1) for o in spans):
                        reached.add(c.group(1))
            elif any(re.search(r"\b%s\s*\(" % n, b, A) for n in reached):
                reached.add(s[0])
    return "\n".join(body(s) for s in spans if s[0] in reached)


def family_scope(leaf):
    segs = leaf.split("/")
    facet = segs.index(MUTATIONS)
    sub = segs.index("🪆subsets") if "🪆subsets" in segs else -1
    if sub != -1 and sub + 1 < facet:
        return "/".join(segs[:sub + 2])
    marker = "/".join(segs[:facet + 1])
    at = marker.find("/🏅standards/")
    if at > 0:
        return marker[:at]
    return "/".join(marker.split("/")[:-1])


def sum_law_untested(tree):
    leaves, payloads, composites, tested, reach = {}, {}, set(), set(), {}
    for path in tree:
        segs = path.split("/")
        facet = segs.index(MUTATIONS) if MUTATIONS in segs else -1
        if "🧪tests" in segs and path.endswith(".rs"):
            r = sum_reach("\n".join(tree[path]))
            if r:
                reach[path] = r
        if facet == -1:
            continue
        plan = segs.index("🧩plan", facet + 1) if "🧩plan" in segs[facet + 1:] else -1
        if plan != -1:
            composites.add("/".join(segs[:plan]))
        tests = segs.index("🧪tests", facet + 1) if "🧪tests" in segs[facet + 1:] else -1
        if tests != -1:
            if path.endswith(".rs") and CALL in "\n".join(tree[path]):
                tested.add("/".join(segs[:tests]))
            continue
        diff = segs.index("🔺diff", facet + 1) if "🔺diff" in segs[facet + 1:] else -1
        if diff != -1 and path.endswith(".rs"):
            leaves.setdefault("/".join(segs[:diff]), path)
        elif len(segs) == facet + 3 and segs[facet + 2] == "🦀.rs":
            kinds = re.findall(r"\bMutationKind\s*<[^;{]*?>\s*for\s+(\w+)", "\n".join(tree[path]), A)
            if kinds:
                leaf = "/".join(segs[:facet + 2])
                leaves[leaf] = path
                payloads[leaf] = kinds
    out = []
    for leaf, anchor in leaves.items():
        if leaf in composites or leaf in tested:
            continue
        slug = re.sub(r"^[^a-z0-9]+", "", leaf.split("/")[-1])
        variant = "".join(p[:1].upper() + p[1:] for p in slug.split("-"))
        names = payloads.get(leaf, []) + [variant]
        scope = family_scope(leaf) + "/"
        texts = [t for p, t in reach.items() if p.startswith(scope)]
        if any(('"%s"' % slug) in t or any(re.search(r"\b%s\b" % n, t, A) for n in names) for t in texts):
            continue
        out.append(breach("R15", anchor, 1))
    return out


def run(case):
    rule = case["rule"]
    if rule == "policyInverseSumLawUntestedLeafBreaches":
        return sum_law_untested(case["tree"])
    text = "\n".join(case["source"]) + "\n"
    return {"policyDiffOnlyFileBreaches": lambda: leaf_file(case["path"], text) + host_apply(case["path"], text) + row_emitter(case["path"], text), "policyRowEmitterFileBreaches": lambda: row_emitter(case["path"], text), "policyHostApplyFileBreaches": lambda: host_apply(case["path"], text)}[rule]()
cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases if c.get("oracle") is not False})
