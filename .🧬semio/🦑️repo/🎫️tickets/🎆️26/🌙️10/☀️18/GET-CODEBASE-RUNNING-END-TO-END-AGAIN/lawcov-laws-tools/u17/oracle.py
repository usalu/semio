FACETS = ["📸snapshot", "🔺diff", "🧬mutations"]
FIXTURE_ALLOW = {"stdio/avi", "stdio/epw", "stdio/html", "stdio/mp3", "stdio/mp4", "stdio/schema", "stdio/semio", "stdio/tsv", "stdio/wav"}
LANG_ALLOW = {"stdio/avi/standards#1.0", "stdio/epw/standards#energyplus", "stdio/html/standards#5", "stdio/mp3/standards#mpeg1-layer3", "stdio/mp4/standards#isobmff", "stdio/semio/standards#v1", "stdio/tsv/standards#iana", "stdio/wav/standards#riff-pcm"}
MIN_CALLS = 5


def row(i, kind, prio, line=0):
    return [i, "stdio-artifacts/" + kind, prio, line]


def looks_real(content, kind):
    lines = re.split(r"\r?\n", content)
    code_lines = [l for l in lines if not l.strip().startswith("#")]
    header = "dialect " + kind
    code = "\n".join(code_lines)
    abnf = any(l.strip().startswith(";") for l in code_lines) or re.search(r"%x[0-9A-Fa-f]", code) or "*hexdig" in code or "*OCTET" in code
    return any(l.strip() == header for l in lines) and any(re.match(r"^%s\s+\S+" % kind, l.strip()) for l in lines) and any(re.match(r"^start\s+\S+", l.strip()) for l in lines) and not abnf


def parseability(t, representation, leaf, kind, label):
    out = []
    for e in stdio_schema_owning(t):
        for facet in FACETS:
            rel = "%s/🧬schema/%s/%s/%s" % (e["subsetRel"], facet, representation, leaf)
            if rel not in t.files:
                continue
            if not looks_real(t.files[rel], kind):
                out.append(row("%s-parseability-%s" % (label, rel), "%s-parseability" % label, "medium"))
    return out


def fixture_honesty(t):
    out = []
    for art, is_dir in sorted(t.children(STDIO).items()):
        if not is_dir or art in SKIP_NAMES:
            continue
        art_rel = STDIO + "/" + art
        ident = strip_non_ascii(art)
        dsl = art_rel + "/📚examples/🎬demo/🖼assets/🗣.dsl.semio"
        dsl_ok = dsl in t.files and re.split(r"\r?\n", t.files[dsl])[0].strip().startswith("semio stdio." + ident)
        pack_ok = t.exists(art_rel + "/📚examples/🎬demo/🖼assets/🎒.pack.semio")
        allowed = ("stdio/" + ident) in FIXTURE_ALLOW
        honest = dsl_ok and pack_ok
        if not honest and not allowed:
            out.append(row("fixture-honesty-" + art_rel, "fixture-honesty", "medium"))
        elif honest and allowed:
            out.append(row("fixture-honesty-stale-" + art_rel, "fixture-honesty", "low"))
    return out


def language_registration(t):
    out, seen = [], set()
    for e in stdio_schema_owning(t):
        key = "stdio/%s/standards#%s" % (e["artifactId"], e["standardSlug"])
        if key in seen:
            continue
        seen.add(key)
        engine = e["standardRel"] + "/⚙engine/🦀.rs"
        count = len(re.findall(r"register_language", t.files[engine])) if engine in t.files else 0
        allowed = key in LANG_ALLOW
        if count < MIN_CALLS and not allowed:
            out.append(row("language-registration-" + key, "language-registration", "medium"))
        elif count >= MIN_CALLS and allowed:
            out.append(row("language-registration-stale-" + key, "language-registration", "low"))
    return out


def json_transfer(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".rs")):
        if not p.startswith(STDIO + "/"):
            continue
        c = t.files[p]
        trait = False
        for m in re.finditer(r"impl(?:<[^>]*>)?\s+(?:[\w:]+::)?(ArtifactPack|OpBinary|DiffCodec)[^{]*for", c, A):
            body = extract_body(c, m.start())
            if "serde_json::to_vec(" in body or "serde_json::from_slice(" in body:
                trait = True
                break
        io = "🚪io" in p and any(s in c for s in ("serde_json::to_vec(", "serde_json::from_slice(", "serde_json::to_string(", "serde_json::from_str("))
        if trait or io:
            out.append(row("stdio-json-transfer-ban-" + p, "json-transfer-ban", "medium"))
    return out


def run(t):
    return (parseability(t, "📝text", "📖.grammar.semio", "grammar", "grammar") + parseability(t, "💾binary", "📡.protocol.semio", "protocol", "protocol") + fixture_honesty(t) + language_registration(t) + json_transfer(t))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: [r for r in run(Tree(expand(c))) if r[1].startswith(c.get("kindFilter", ""))] for c in cases if c.get("oracle") is not False})
