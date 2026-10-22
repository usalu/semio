FACETS = ["📸snapshot", "🔺diff", "🧬mutations"]
SIBLINGS = ["🟦.ts", "🔗.graphql", "🔣.json", "🛰.proto"]
KEYWORDS = {"self", "where", "if", "else", "match", "for", "while", "let", "fn", "return", "in", "as", "dyn", "mut", "ref", "impl", "type"}
TS_KEYWORDS = set("interface type export import from extends implements class function const let var namespace module declare readonly public private protected static abstract new this super typeof keyof case default switch try catch finally throw async await yield get set constructor void never unknown any undefined null true false satisfies asserts is infer of in as if else for while return".split())


def camel(name):
    parts = name.split("_")
    return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])


def names(items):
    seen, out = set(), []
    for n in items:
        if n not in seen:
            seen.add(n)
            out.append(n)
    return [camel(n) for n in out if camel(n)]


def rust_fields(content):
    s = strip_rust_text(content)
    got = [m.group(1) for m in re.finditer(r"(?:^|[\s{,(])(?:pub\s+)?([a-z][a-z0-9_]*)\s*:\s*[A-Za-z_&\[<('\"]", s, re.M | A) if m.group(1) not in KEYWORDS and not m.group(1).startswith("r#")]
    return names(got)


def rust_tags(content):
    return list(dict.fromkeys(m.group(1) for m in re.finditer(r'#\[serde\([^\]]*\btag\s*=\s*"([a-zA-Z_][a-zA-Z0-9_]*)"', content)))


def rust_variants(content):
    s = strip_rust_text(content)
    return list(dict.fromkeys(m.group(1)[:1].lower() + m.group(1)[1:] for m in re.finditer(r"^\s*([A-Z][A-Za-z0-9]*)\s*[{(]", s, re.M | A)))


def ts_fields(content):
    s = strip_ts_text(content)
    return names([m.group(1) for m in re.finditer(r"(?:^|[\s{;(])([A-Za-z_][A-Za-z0-9_]*)\??\s*:\s*[^;\n=]+[;\n]", s, re.M | A) if m.group(1) not in TS_KEYWORDS])


def graphql_fields(content):
    s = re.sub(r"\([^)]*\)", "", re.sub(r"#.*$", "", content, flags=re.M))
    return names([m.group(1) for m in re.finditer(r"^\s*([a-z][A-Za-z0-9_]*)\s*:", s, re.M | A)])


def json_fields(content):
    found = []

    def visit(node):
        if isinstance(node, list):
            for i in node:
                visit(i)
        elif isinstance(node, dict):
            if isinstance(node.get("properties"), dict):
                found.extend(node["properties"].keys())
            for v in node.values():
                visit(v)

    visit(json.loads(content))
    return names(found)


def proto_fields(content):
    body = "\n".join(l for l in re.split(r"\r?\n", content) if not re.match(r"^\s*(enum|oneof|message|package|syntax|import)\b", l))
    return names([m.group(1) for m in re.finditer(r"^\s*(?:optional\s+|repeated\s+)?(?:map\s*<[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;", body, re.M | A)])


EXTRACT = {"🟦.ts": ts_fields, "🔗.graphql": graphql_fields, "🔣.json": json_fields, "🛰.proto": proto_fields}


def drift(t):
    out = []
    for e in stdio_schema_owning(t):
        for facet in FACETS:
            rs = "%s/🧬schema/%s/🦀.rs" % (e["subsetRel"], facet)
            if rs not in t.files:
                continue
            content = t.files[rs]
            fields = rust_fields(content)
            compare = fields + rust_tags(content) + rust_variants(content)
            notes = []
            for sib in SIBLINGS:
                path = "%s/🧬schema/%s/%s" % (e["subsetRel"], facet, sib)
                if path not in t.files:
                    notes.append(sib + ":MISSING_FILE")
                    continue
                sc = t.files[path]
                missing = [f for f in fields if f not in sc]
                if missing:
                    notes.append("%s:%d" % (sib, len(missing)))
                try:
                    sf = EXTRACT[sib](sc)
                except ValueError:
                    notes.append(sib + ":PARSE_ERROR")
                    continue
                extra = [f for f in sf if f != "schema" and f not in compare and not any(len(cf) >= 4 and cf.lower() in f.lower() for cf in compare)]
                if extra:
                    notes.append("%s:extra:%d" % (sib, len(extra)))
            if notes:
                out.append(["facet-mirror-drift-" + rs, "stdio-artifacts/facet-mirror-drift", "low", 0])
    return out


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: drift(Tree(expand(c))) for c in cases})
