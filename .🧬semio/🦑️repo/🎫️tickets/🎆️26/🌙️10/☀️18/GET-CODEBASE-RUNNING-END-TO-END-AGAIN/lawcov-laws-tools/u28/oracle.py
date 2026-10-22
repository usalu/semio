BUILDER = re.compile(r'\b(?:MutationOutcome::(error|fatal)|MutationMessage::(info|warning|error|fatal))\s*\(\s*"([^"]*)"', A)
CHAIN = re.compile(r'\.(info|warning)\s*\(\s*"([^"]*)"', A)
RETIRED = re.compile(r"(?:\bMutationMessage::|\.)warn\s*\(", A)
APPLY = re.compile(r'\bMutationApplyError(?:::new\s*\(\s*|\s*\{\s*code:\s*)"([^"]*)"', A)
TS_REFUSE = re.compile(r'\brefuse\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([^"]*)"', A)
TS_OBJECT = re.compile(r'\blevel:\s*"(info|warn|warning|error|fatal)"\s*,\s*code:\s*"(mutation\.[^"]*)"|\bcode:\s*"(mutation\.[^"]*)"\s*,\s*level:\s*"(info|warn|warning|error|fatal)"', A)
PY_TUPLE = re.compile(r'\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([a-z0-9]+(?:[.-][a-z0-9]+)+)"\s*\)', A)
FEATURE_MESSAGE = re.compile(r"\blevel:\s*([a-z]+)\s*,\s*code:\s*(mutation\.[a-z0-9][a-z0-9.-]*[a-z0-9])(?![\w.<*-])", A)
LEVEL_ALIAS = re.compile(r'(?:"[a-z]+"\s*\|\s*)*"warn"\s*(?:\|\s*"[a-z]+"\s*)*=>\s*(?:Ok\(|Some\()?(?:[A-Za-z_]\w*::)*Severity::Warning\b', A)
LITERAL = re.compile(r"""["'`](mutation\.[a-z0-9][a-z0-9.-]*)["'`]""", A)
FEATURE_TOKEN = re.compile(r"(?<![\w.-])(mutation\.[a-z0-9][a-z0-9-]*(?:\.[a-z0-9][a-z0-9-]*)*)(?![\w-]|\.[<*])", A)
K = "mutation-migration/message-code"


def line_of(c, i):
    return c[:i].count("\n") + 1


def body_mask(content):
    masked = "\n".join(mask(l) for l in content.split("\n"))
    close, stack = {}, []
    for i, ch in enumerate(masked):
        if ch == "{":
            stack.append(i)
        elif ch == "}" and stack:
            close[stack.pop()] = i
    inside = [False] * len(content)
    for m in re.finditer(r"\bfn\s+[A-Za-z_]\w*", masked, A):
        op = masked.find("{", m.start())
        end = close.get(op, -1) if op != -1 else -1
        if end != -1 and "MutationOutcome" in content[op:end + 1]:
            for i in range(op, end + 1):
                inside[i] = True
    return inside


def outcome_code_file(vocab, path, content):
    out = []
    if not content or (path.endswith(".py") and "🧪tests/" not in path):
        return out
    codes = {c["code"]: c["level"] for c in vocab["codes"]}
    apply_re = re.compile(vocab["applyPattern"])
    reported = set()

    def check(code, level, index, apply=False):
        expected = codes.get(code) or (vocab["applyLevel"] if apply_re.search(code) else None)
        if apply:
            fault = None if apply_re.search(code) else "x"
        elif expected is None:
            fault = "x"
        else:
            fault = "x" if level is not None and level != expected else None
        if fault is None:
            return
        n = line_of(content, index)
        if (n, code) in reported:
            return
        reported.add((n, code))
        out.append(["mutation-message-code-%s-%d-%s" % (path, n, code), K, "high", n])

    leaf = any(s in "/" + path for s in ("/🧬mutations/", "/🔺diff/"))
    if path.endswith(".rs"):
        for m in BUILDER.finditer(content):
            check(m.group(3), m.group(1) or m.group(2), m.start())
        for m in APPLY.finditer(content):
            check(m.group(1), None, m.start(), True)
        if leaf:
            for m in LITERAL.finditer(content):
                check(m.group(1), None, m.start())
        bodies = body_mask(content) if "MutationOutcome" in content else [False] * len(content)
        for m in CHAIN.finditer(content):
            if bodies[m.start()]:
                check(m.group(2), m.group(1), m.start())
        for m in RETIRED.finditer(content):
            if m.group(0).startswith(".") and not bodies[m.start()]:
                continue
            n = line_of(content, m.start())
            out.append(["mutation-message-retired-builder-%s-%d-%d" % (path, n, m.start()), K, "high", n])
        for m in LEVEL_ALIAS.finditer(content):
            n = line_of(content, m.start())
            out.append(["mutation-message-level-alias-%s-%d" % (path, n), K, "high", n])
    elif path.endswith(".ts") or path.endswith(".tsx"):
        for m in TS_REFUSE.finditer(content):
            check(m.group(2), m.group(1), m.start())
        for m in TS_OBJECT.finditer(content):
            check(m.group(2) or m.group(3), m.group(1) or m.group(4), m.start())
        if leaf:
            for m in LITERAL.finditer(content):
                check(m.group(1), None, m.start())
    elif path.endswith(".py"):
        for m in PY_TUPLE.finditer(content):
            check(m.group(2), m.group(1), m.start())
        for m in LITERAL.finditer(content):
            check(m.group(1), None, m.start())
    elif path.endswith(".feature"):
        for m in FEATURE_TOKEN.finditer(content):
            if not re.search(r"\.(json|rs|ts|tsx|py|md|txt)$", m.group(1)):
                check(m.group(1), None, m.start())
        for m in FEATURE_MESSAGE.finditer(content):
            check(m.group(2), m.group(1), m.start())
    else:
        try:
            doc = json.loads(content)
        except ValueError:
            return out
        if not isinstance(doc, dict):
            return out
        cursor = 0
        messages = doc.get("messages") if isinstance(doc.get("messages"), list) else []
        for msg in [doc] + messages:
            if isinstance(msg, dict) and isinstance(msg.get("code"), str):
                at = content.find(json.dumps(msg["code"], ensure_ascii=False), cursor)
                if at != -1:
                    cursor = at + 1
                level = msg.get("level") if isinstance(msg.get("level"), str) else None
                check(msg["code"], level, max(at, 0))
    return out


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: outcome_code_file(c["vocabulary"], c["path"], "\n".join(c["source"]) + "\n") for c in cases})
