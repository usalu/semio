FLAGSHIP = [STDIO + "/" + d for d in ("🎨svg", "🧊gltf", "🌳pdf", "🎞gif")]
K = "artifact-io/"


def row(i, kind, prio="high"):
    return [i, kind, prio, 0]


def strip_rust(body):
    body = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), body, flags=re.S)
    body = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), body)
    return re.sub(r'"(?:[^"\\]|\\.)*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', body)


def codec_ids(t):
    if not t.exists("✏s/🔌plugins/🗄stdio"):
        return []
    files = [p for p in t.walk([""], lambda p, n: n.endswith(".rs")) if p.startswith(STDIO + "/")]
    consts = {}
    for p in files:
        for m in re.finditer(r'(?:pub\s+)?const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]*)"\s*;', t.files[p]):
            consts[m.group(1)] = m.group(2)
    claims = {}
    for p in files:
        c = t.files[p]
        for m in re.finditer(r'register_document_codec\(\s*store::artifact_codec::<[\s\S]*?>\(\s*([\w:]+|"[^"]*")\s*\)\s*\)', c, A):
            raw = m.group(1)
            value = raw[1:-1] if raw.startswith('"') else consts.get(raw.split("::")[-1])
            if not value:
                continue
            claims.setdefault(value, []).append((p, c[:m.start()].count("\n") + 1))
    out = []
    for ident, sites in claims.items():
        if len(sites) > 1:
            out += [row("stdio-codec-id-duplicate-%s-%s-%d" % (ident, p, n), "stdio-artifacts/codec-id-uniqueness") for p, n in sites]
    return out


def round_trip(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".rs")):
        if not p.startswith("✏s/🔌plugins/") or not p.endswith("⚙engine/🦀.rs") or "🏅standards/🔖" not in p:
            continue
        c = t.files[p]
        signal = "#[cfg(test)]" in c and re.search(r"round_trip|roundtrip|decode_encode|encode_decode|lossless", c, re.I)
        if not signal:
            out.append(row("round-trip-test-" + p, K + "round-trip-test"))
    return out


def enum_variants(content):
    s = strip_rust(content)
    h = re.search(r"pub\s+enum\s+\w*Mutation\w*\s*\{", s, A)
    if not h:
        return None
    i = h.end()
    depth, seg, out = 1, i, []
    def push(raw):
        m = re.search(r"(?:#\[[^\]]*\]\s*)*(\w+)", raw, A)
        if m:
            out.append(m.group(1))
    while i < len(s) and depth > 0:
        ch = s[i]
        if ch in "{(":
            depth += 1
        elif ch in "})":
            depth -= 1
            if depth == 0:
                push(s[seg:i])
                break
        elif ch == "," and depth == 1:
            push(s[seg:i])
            seg = i + 1
        i += 1
    return out


def vocabulary(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".rs")):
        if not p.endswith("🧬mutations/🦀.rs") or not any(p.startswith(d + "/") for d in FLAGSHIP):
            continue
        v = enum_variants(t.files[p])
        real = len([x for x in (v or []) if x not in ("NoMutation", "SetSnapshot")])
        if real == 0:
            out.append(row("mutation-vocabulary-" + p, K + "mutation-vocabulary"))
    return out


def run(c):
    t = Tree(expand(c))
    return {"policyStdioCodecIdUniquenessBreaches": codec_ids, "policyRoundTripTestBreaches": round_trip, "policyMutationVocabularyBreaches": vocabulary}[c["rule"]](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
