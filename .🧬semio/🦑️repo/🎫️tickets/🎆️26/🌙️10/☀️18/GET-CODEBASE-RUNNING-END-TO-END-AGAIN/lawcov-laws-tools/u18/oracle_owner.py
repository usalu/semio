FALLBACK_FACETS = ["🧬schema", "⚙engine", "🚪io", "🪓decomposer"]
LEGACY = {"🗣dsl", "🔧op", "📡spr", "🔺diff", "📸snapshot"}
FALLBACK_SCHEMA = ["📸snapshot", "🔺diff", "🧬mutations"]


def kind_file(kind):
    k = TAXONOMY["fileKinds"][kind]
    return k["emoji"] + k["extensionChains"][0]


def strip_rust(body):
    body = re.sub(r"/\*.*?\*/", " ", body, flags=re.S)
    body = re.sub(r"//[^\n]*", " ", body)
    return re.sub(r'"(?:[^"\\]|\\.)*"', '""', body)


def real_trait_impl(body, trait):
    return re.search(r"\bimpl\s*(?:<[^>{]*>\s*)?%s\b\s*(?:<[^>{]*>\s*)?for\b" % re.escape(trait), strip_rust(body), A) is not None


def run(c):
    a, r = c.get("args", []), c["rule"]
    if r == "policyStdioArtifactFacets":
        dirs = a[0].get("artifactComponentDirs")
        if not dirs:
            return list(FALLBACK_FACETS)
        out = list(FALLBACK_FACETS)
        for d in dirs:
            if d not in LEGACY and d not in out:
                out.append(d)
        return out
    if r == "policyStdioSchemaChildDirs":
        return list(a[0].get("schemaChildDirs") or FALLBACK_SCHEMA)
    if r == "policyStdioRepresentationDirs":
        return list(a[0]["nativeRepresentationDirs"])
    if r == "policyStdioFormatDir":
        return (a[0].get(a[1]) or {}).get("dir")
    if r == "policyStdioArtifactsDirName":
        return TAXONOMY["artifactsDirName"]
    if r == "policyRustFileHasRealTraitImpl":
        return real_trait_impl(a[0], a[1])
    if r == "specLeaves":
        text = [kind_file(k) for k in ("grammar-semio", "ebnf", "antlr", "graphql", "json", "protobuf")] + [RS, TS]
        binary = [kind_file(k) for k in ("protocol-semio", "abnf", "kaitai", "spicy")] + [RS, TS]
        return {"text": text, "binary": binary}
    raise KeyError(r)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases if c.get("oracle") is not False})
