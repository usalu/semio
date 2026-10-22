REP_TYPES = ["HalfedgeMesh", "BrepEngineHost", "DrawingStore", "DrawingEngine", "EngineCache"]
CACHE_DIRS = ["🧰framework/🔨modules/◻2d/🧮compute", "🧰framework/🛍products/💻os/🔨modules/🔌plugin"]
REP_ALLOW = {"✏s/🔌plugins/📐cad/🗿artifacts/📐cad/🏅standards/🔖1/🪆subsets/✳any/⚙engine/🦀.rs", "✏s/🔌plugins/🏭process/🗿artifacts/🧊process3d/🏅standards/🔖1/🪆subsets/✳any/⚙engine/🦀.rs", "🧰framework/🔨modules/◻2d/🗄store/🦀.rs", "🧰framework/🔨modules/🧊3d/📐brep/⚙engine/🖥host/🦀.rs"}
CANONICAL_TYPES = ["SemioAnimationSnapshot", "SemioAudioSnapshot", "SemioBrepSnapshot", "SemioCadSnapshot", "SemioDocumentSnapshot", "SemioDrawingSnapshot", "SemioFlowSnapshot", "SemioGraphSnapshot", "SemioImageSnapshot", "SemioKitSnapshot", "SemioMeshSnapshot", "SemioModelSnapshot", "SemioObjectSnapshot", "SemioPresentationSnapshot", "SemioTableSnapshot", "SemioTextSnapshot", "SemioValueSnapshot", "SemioVideoSnapshot"]
CHILD = {"ts": r"@child\s+kind\s*=\s*([^\s*/]+)", "json": r'"x-semio-child"\s*:\s*"([^"]*)"', "graphql": r'@child\s*\(\s*kind\s*:\s*"([^"]*)"\s*\)', "proto": r"@child\s+kind\s*=\s*([^\s*/]+)"}


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def rust(t):
    return t.walk([""], lambda p, n: n.endswith(".rs"))


def cache_owner(p):
    return any(p == d or p.startswith(d + "/") for d in CACHE_DIRS)


def rep_escape(t):
    out = []
    for p in rust(t):
        c = t.files[p]
        held = [r for r in REP_TYPES if re.search(r"^\s*(pub\s+)?[a-z_][a-z0-9_]*\s*:\s*[^,;]*\b%s\b" % r, c, re.M | A) or re.search(r"static\s+[A-Z0-9_]+\s*:\s*[^=;]*\b%s\b" % r, c, A)]
        allow = normalize_rel(p) in REP_ALLOW
        if held:
            if not allow:
                out.append(row("dissolved-rep-escape-" + p, "dissolved-kernels/rep-escape", "medium"))
        elif allow:
            out.append(row("dissolved-rep-escape-stale-" + p, "dissolved-kernels/rep-escape", "low"))
    return out


def cache_scope(t):
    return [row("dissolved-engine-cache-scope-" + p, "dissolved-kernels/engine-cache-scope", "medium") for p in rust(t) if not cache_owner(p) and re.search(r"\bEngineCache::new\b", t.files[p], A)]


def replace(t):
    seen = {}
    for p in rust(t):
        if "🧬mutations/" in p and "/🟤set-snapshot/" in p:
            d = p[: p.index("/🟤set-snapshot/") + len("/🟤set-snapshot")]
            seen["dissolved-whole-document-replace-" + d] = row("dissolved-whole-document-replace-" + d, "dissolved-kernels/whole-document-replace", "medium")
    return list(seen.values())


def canonical(kind):
    seg = kind.split(".")
    ok = lambda s: len(s) > 0 and not s.startswith("-") and not s.endswith("-") and "--" not in s and re.fullmatch(r"[a-z0-9-]+", s)
    return len(seg) == 3 and seg[0] == "s" and bool(ok(seg[1])) and bool(ok(seg[2]))


def commented(c, idx):
    start = c.rfind("\n", 0, idx) + 1
    return "//" in c[start:idx]


def artifact_kinds(t):
    out = []
    for p in rust(t):
        c = t.files[p]
        if "ArtifactKindSpec" not in c:
            continue
        for m in re.finditer(r'ArtifactKindSpec\s*\{\s*id:\s*"((?:[^"\\]|\\.)*)"', c):
            if commented(c, m.start()) or canonical(m.group(1)):
                continue
            n = c[:m.start()].count("\n") + 1
            out.append(row("canonical-artifact-kind-%s-%d" % (p, n), "composition/canonical-artifact-kind", "medium", n))
    return out


def child_kinds(t):
    out = []
    pred = lambda p, n: "/🧬schema/" in p and "/🧪tests/" not in p and "/🧫fixtures/" not in p and (n.endswith(".rs") or n.split(".")[-1] in CHILD)
    for p in t.walk(["✏s/🔌plugins"], pred):
        fmt = p.split(".")[-1]
        if fmt == "rs":
            continue
        c = t.files[p]
        for m in re.finditer(CHILD[fmt], c):
            start = c.rfind("\n", 0, m.start()) + 1
            if fmt == "graphql" and "#" in c[start:m.start()]:
                continue
            if not canonical(m.group(1)):
                n = c[:m.start()].count("\n") + 1
                out.append(row("canonical-child-kind-%s-%d" % (p, n), "composition/canonical-child-kind", "high", n))
    return out


def redefinition(t):
    out = []
    for p in rust(t):
        if "🔌plugins/" not in p or "🔌plugins/🗄stdio/" in p:
            continue
        c = t.files[p]
        for name in CANONICAL_TYPES:
            if name not in c:
                continue
            m = re.search(r"^[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum)\s+%s\b" % name, c, re.M | A)
            if not m or commented(c, m.start()):
                continue
            n = c[:m.start()].count("\n") + 1
            out.append(row("dissolved-kind-redefinition-%s-%s" % (p, name), "composition/dissolved-kind-redefinition", "high", n))
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    return {
        "policyDissolvedKernelsBreaches": lambda: rep_escape(t) + cache_scope(t) + replace(t), "policyCanonicalArtifactKindBreaches": lambda: artifact_kinds(t),
        "policyCanonicalChildKindBreaches": lambda: child_kinds(t), "policyDissolvedKindRedefinitionBreaches": lambda: redefinition(t),
        "policyCompositionBreaches": lambda: artifact_kinds(t) + child_kinds(t) + redefinition(t),
    }[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
