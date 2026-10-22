MARKERS = re.compile(r"\b(CollectionDiff|CollectionMutation|Patchable|Identified)\b", A)


def row(i, kind, prio):
    return [i, "stdio-artifacts/" + kind, prio, 0]


def has_impl(body, trait):
    s = strip_rust_text(body)
    return re.search(r"\bimpl\s*(?:<[^>{]*>\s*)?%s\b\s*(?:<[^>{]*>\s*)?for\b" % trait, s, A) is not None


def machinery(t):
    return [row("stdio-vcs-machinery-ban-" + p, "vcs-machinery-ban", "high") for p in t.walk([""], lambda p, n: n.endswith(".rs")) if p.startswith(STDIO + "/") and MARKERS.search(strip_rust_text(t.files[p]))]


def diff_algebra(t):
    out = []
    for e in stdio_schema_owning(t):
        rs = e["subsetRel"] + "/🧬schema/🔺diff/🦀.rs"
        if rs in t.files and not has_impl(t.files[rs], "DiffAlgebra"):
            out.append(row("diff-algebra-missing-" + rs, "diff-algebra", "medium"))
    return out


def sniff(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".rs")):
        c = t.files[p]
        if "fn sniff(" in c and re.search(r"fn\s+sniff\(\s*_[A-Za-z0-9_]*\s*:", c, A):
            out.append(["sniff-reality-" + p, "artifact-io/sniff-reality", "high", 0])
    return out


def run(c):
    t = Tree(expand(c))
    r = {"policyStdioVcsMachineryBanBreaches": machinery, "policyDiffAlgebraBreaches": diff_algebra, "policySniffRealityBreaches": sniff, "policySchemaOverhaulS2Breaches": lambda t: machinery(t) + diff_algebra(t)}[c["rule"]](t)
    return [x for x in r if x[1].startswith(c.get("kindFilter", ""))]


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
