CRDT = re.compile(r"\b(merge_strategy|MergeStrategyKind|merge_concurrent_diffs|ConflictRule|ResolutionPlan|assert_crdt_)", A)
VALIDATE = re.compile(r"\bfn\s+validate\s*\(\s*&self\b", A)
KIND_IMPL = re.compile(r"\bimpl\b[^\n{]*\b(?:CompositeMutationKind|MutationKind)\s*<", A)
HINT = re.compile(r"\bSeverity(?:::|\.)Hint\b", A)
SURFACES = ["🧰framework/🔨modules/📡replication/🧾wire/🦀.rs", "🧰framework/🔨modules/🎠kernel/🔄merge-outcome/🟦.ts", "🧰framework/🔨modules/🖱ui/🧱elements/📚I18n/🟦.tsx", "🧰framework/🔨modules/🖱ui/🎯targets/⚛react/🌐i18n/🤝collaboration/🟦.ts"]
SPELL = {"LaissezFaire": ["LaissezFaire", "laissezFaire"], "Normal": ["Normal", "normal"], "Vigilant": ["Vigilant", "vigilant"]}
GLUE = "🧰framework/🛍products/💻os/🔨modules/🗣dsl/✨derive/📦packages/🦀rust/🦀.rs"
OWNER = "🧰framework/🛍products/💻os/🔨modules/🗣dsl/✨derive/🦀.rs"


def row(i, kind, line=0):
    return [i, "mutation-migration/" + kind, "high", line]


def inventory(t):
    out = []
    for p in sorted(t.files):
        if not (p.endswith((".rs", ".ts", ".tsx", ".py", ".feature")) or p.endswith("🎯outcome/🔣.json")):
            continue
        if any(s in SKIP_NAMES for s in p.split("/")) or p == "📜script.ts":
            continue
        out.append(p)
    return jssort(out)


def line_of(c, i):
    return c[:i].count("\n") + 1


def crdt(t):
    out = []
    for p in [p for p in inventory(t) if p.endswith(".rs")]:
        c, seen = t.files[p], set()
        for m in CRDT.finditer(c):
            n = line_of(c, m.start())
            if n not in seen:
                seen.add(n)
                out.append(row("no-crdt-vocabulary-%s-%d" % (p, n), "no-crdt-vocabulary", n))
    return out


def validate(t):
    out = []
    for p in [p for p in inventory(t) if p.endswith(".rs")]:
        c, reported = t.files[p], set()

        def push(n):
            if n not in reported:
                reported.add(n)
                out.append(row("no-validate-override-%s-%d" % (p, n), "no-validate-override", n))

        if "/🧬mutations/" in p:
            for m in VALIDATE.finditer(c):
                push(line_of(c, m.start()))
        for im in KIND_IMPL.finditer(c):
            body = extract_body(c, im.start())
            vm = VALIDATE.search(body)
            if vm:
                push(line_of(c, c.find("{", im.start()) + vm.start()))
    return out


def severity(t):
    out = []
    for p in [p for p in inventory(t) if p.endswith((".rs", ".ts", ".tsx"))]:
        c, reported = t.files[p], set()
        for m in HINT.finditer(c):
            n = line_of(c, m.start())
            if n not in reported:
                reported.add(n)
                out.append(row("severity-hint-%s-%d" % (p, n), "no-severity-hint", n))
        for i, l in enumerate(split_lines(c)):
            if re.search(r"""["']hint["']""", l, re.I) and re.search(r"severity|level", l, re.I) and (i + 1) not in reported:
                reported.add(i + 1)
                out.append(row("severity-hint-literal-%s-%d" % (p, i + 1), "no-severity-hint", i + 1))
    return out


def parity(t):
    out = []
    for s in SURFACES:
        c = t.files.get(s, "")
        missing = [v for v in SPELL if not any(x in c for x in SPELL[v])] if c else list(SPELL)
        if missing:
            out.append(row("merge-policy-parity-" + s, "merge-policy-parity"))
    return out


def derive(t):
    glue = t.files.get(GLUE, "")
    if t.files.get(OWNER) and re.search(r'#\[path\s*=\s*"\.\./\.\./🦀\.rs"\]\s*mod\s+component\s*;', glue) and not re.search(r"\bfn\s+expand_", glue, A):
        return []
    return [row("derive-glue-mount-" + GLUE, "derive-glue-mount")]


def run(c):
    t = Tree(expand(c))
    return {"policyNoCrdtVocabularyBreaches": crdt, "policyNoValidateOverrideBreaches": validate, "policySeverityInfoBreaches": severity, "policyMergePolicyParityBreaches": parity, "policyDeriveGlueMountBreaches": derive}[c["rule"]](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
