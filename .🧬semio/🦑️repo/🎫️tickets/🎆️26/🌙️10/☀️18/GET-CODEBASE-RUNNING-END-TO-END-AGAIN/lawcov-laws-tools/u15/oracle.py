GENERIC_BRIDGES = {"Value", "SetDocumentMutation"}
PACK_ALLOW = set()
ENVELOPE_ALLOW = {"architect/spine", "os/dsl", "products/os", "lowpoly/spr", "os/workflow", "animate/presentation/spr", "gis/artifact", "mathematical/equation/artifact", "note/artifact", "raster/artifact", "reasoning/artifact", "space/artifact", "vcs/artifact"}
FACETS = {"⚙engine", "🎒pack", "📡spr", "🔧op", "🔺diff", "🗣dsl"}
NAMES = ["Mutation", "MutationDiff", "MutationMeta", "OpText", "Edit", "merge_concurrent_diffs", "ReconcileReport", "ReconcileSeverity", "HybridLogicalTimestamp", "MutationEnvelope", "MutationDag", "UndoPolicy", "MergeStrategyKind", "HubClientFrame", "HubServerFrame"]
QUALIFIED = re.compile(r"\bvcs::(%s)\b" % "|".join(NAMES), A)
USE_BLOCK = re.compile(r"use\s+(?:::)?vcs::\{([^}]*)\}", re.S | A)
DERIVE = re.compile(r"derive\([^)]*\bDsl(?:Document|Ops|Record|Enum|Scalar)\b[^)]*\)", A)


def row(i, kind, prio="high", line=0):
    return [i, kind, prio, line]


def usages(scope, content):
    out = []
    for m in re.finditer(r"impl\s+DocumentApp\s+for\s+(\w+)\s*\{", content, A):
        body = extract_body(content, m.start())
        s = re.search(r"type\s+Snapshot\s*=\s*([\w:<>]+)\s*;", body, A)
        mu = re.search(r"type\s+Mutation\s*=\s*([\w:<>]+)\s*;", body, A)
        if not s or not mu:
            continue
        out.append({"line": content[:m.start()].count("\n") + 1, "app": m.group(1), "snapshot": re.sub(r"<.*$", "", s.group(1).split("::")[-1]), "mutation": re.sub(r"<.*$", "", mu.group(1).split("::")[-1])})
    return out


def rust_files(t):
    return t.walk([""], lambda p, n: n.endswith(".rs"))


def dsl_completeness(t):
    files = [(p, t.files[p]) for p in rust_files(t)]
    doc, ops, alias = set(), set(), {}
    for _, c in files:
        lines = c.split("\n")
        for m in re.finditer(r"^\s*(?:pub\s+)?(?:struct|enum)\s+(\w+)\b", c, re.M | A):
            n = c[:m.start()].count("\n") + 1
            if DERIVE.search("\n".join(lines[max(0, n - 6): n - 1])):
                doc.add(m.group(1))
                ops.add(m.group(1))
        for m in re.finditer(r"impl(?:<[^>]*>)?\s+(?:vcs::|protocol::)?(DocumentDsl|OpText)\s+for\s+(?:vcs::|protocol::)?(\w+)", c, A):
            (doc if m.group(1) == "DocumentDsl" else ops).add(m.group(2))
        for m in re.finditer(r"\b(\w+)\s+as\s+(\w+)\b", c, A):
            alias[m.group(2)] = m.group(1)

    def resolve(name):
        for _ in range(5):
            nxt = alias.get(name)
            if not nxt or nxt == name:
                break
            name = nxt
        return name

    out = []
    for p, c in files:
        for u in usages(p, c):
            for label, name, names in (("Snapshot", u["snapshot"], doc), ("Mutation", u["mutation"], ops)):
                if name in GENERIC_BRIDGES or name in names or resolve(name) in names:
                    continue
                out.append(row("dsl-completeness-%s-%d-%s" % (p, u["line"], label), "dsl-migration/completeness", "high", u["line"]))
    return out


def pack(t):
    out = []
    for p in rust_files(t):
        if normalize_rel(p) in PACK_ALLOW:
            continue
        c = t.files[p]
        if not ("assert_dsl_round_trip(" in c or "assert_document_text_round_trip(" in c):
            continue
        if "assert_dsl_pack_equivalence(" in c or "assert_document_pack_round_trip(" in c:
            continue
        out.append(row("pack-completeness-" + p, "pack-migration/completeness"))
    return out


def envelope(t):
    out = []
    for p in rust_files(t):
        if normalize_rel(p) in ENVELOPE_ALLOW:
            continue
        c = t.files[p]
        if not ("assert_dsl_pack_equivalence(" in c or "assert_document_pack_round_trip(" in c) or "assert_command_envelope_round_trip" in c:
            continue
        out.append(row("command-envelope-completeness-" + p, "protocol-migration/command-envelope-completeness"))
    return out


def ts_facade(t):
    out = []
    for p in t.walk(["✏s"], lambda p, n: n == "🟦.ts"):
        if "/🗿artifacts/" not in p:
            continue
        c = t.files[p]
        stub = re.search(r"wire\s+.+\s+to plugin WASM", c, re.I) or re.search(r"""throw new Error\(\s*["'`][^"'`]*WASM[^"'`]*["'`]\s*\)""", c)
        parts = p.split("/")
        if not stub or ("🗿artifacts" in parts and parts[-2] in FACETS):
            continue
        out.append(row("ts-facade-misplaced-stub-" + p, "dsl-migration/ts-facade-completeness", "medium"))
    return out


def protocol_migration(t):
    out = []
    for p in rust_files(t):
        c = t.files[p]
        lines = re.split(r"\r?\n", c)
        seen = set()
        line_of = lambda i: len(re.split(r"\r?\n", c[:i]))
        for m in QUALIFIED.finditer(c):
            if c[max(0, m.start() - 7): m.start()] == "crate::":
                continue
            n = line_of(m.start())
            if re.match(r"^\s*(//|\*)", lines[n - 1] if n - 1 < len(lines) else "") or n in seen:
                continue
            seen.add(n)
            out.append(row("protocol-migration-%s-%d" % (p, n), "protocol-migration/vcs-shim-regression", "high", n))
        for m in USE_BLOCK.finditer(c):
            tokens = {x for x in re.split(r"[^\w]+", m.group(1), flags=A) if x}
            if not any(n in tokens for n in NAMES):
                continue
            n = line_of(m.start())
            if n in seen:
                continue
            seen.add(n)
            out.append(row("protocol-migration-use-%s-%d" % (p, n), "protocol-migration/vcs-shim-regression", "high", n))
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    if r == "policyDocumentAppUsages":
        return ["%s|%d|%s|%s|%s" % (c["scope"], u["line"], u["app"], u["snapshot"], u["mutation"]) for u in usages(c["scope"], "\n".join(c["source"]) + "\n")]
    if r in ("policyGrammarFileBreaches", "policyProtocolFileBreaches"):
        return []
    return {"policyDslCompletenessBreaches": dsl_completeness, "policyPackCompletenessBreaches": pack, "policyCommandEnvelopeCompletenessBreaches": envelope, "policyTsFacadeBreaches": ts_facade, "policyProtocolMigrationBreaches": protocol_migration}[r](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
