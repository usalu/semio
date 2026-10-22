def run(c):
    if c["rule"] == "policyStdioStandardKey":
        return "stdio/%s/standards#%s" % (c["args"][0], c["args"][1])
    t = Tree(expand(c))
    out = []
    for e in stdio_schema_owning(t):
        name = e["subsetRel"].split("/")[-1]
        out.append({"artRel": e["artRel"], "artifactId": e["artifactId"], "standardSlug": e["standardSlug"], "subsetRel": e["subsetRel"], "subsetId": subset_id(name) or name})
    return sorted(out, key=lambda e: e["subsetRel"].encode("utf-16-be"))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
