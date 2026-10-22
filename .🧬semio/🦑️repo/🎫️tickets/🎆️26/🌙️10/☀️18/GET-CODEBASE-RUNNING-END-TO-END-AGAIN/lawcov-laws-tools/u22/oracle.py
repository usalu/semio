DERIVED = [("builder", "🏗builder", "construction:"), ("analyzer", "🧐analyzer", "analysis:"), ("composer", "🎹composer", "composition:")]


def row(i, kind):
    return [i, kind, "high", 0]


def artifacts(t):
    out = []
    for plugin, pd in sorted(t.children(PLUGINS).items()):
        if pd:
            for art, ad in sorted(t.children("%s/%s/🗿artifacts" % (PLUGINS, plugin)).items()):
                if ad:
                    out.append("%s/%s/🗿artifacts/%s" % (PLUGINS, plugin, art))
    return out


def migrated(t, art):
    return t.exists(art + "/🏅standards")


def builder(t):
    return [row("artifact-derived-builder-" + a, "stdio-artifacts/derived-facets") for a in artifacts(t) if not migrated(t, a) and t.exists(a + "/🏗builder")]


def has_impl(body, trait):
    s = re.sub(r"//[^\n]*", " ", re.sub(r"/\*.*?\*/", " ", body, flags=re.S))
    s = re.sub(r'"(?:[^"\\]|\\.)*"', '""', s)
    return re.search(r"\bimpl\s*(?:<[^>{]*>\s*)?%s\b\s*(?:<[^>{]*>\s*)?for\b" % trait, s, A) is not None


def decomposer(t):
    out, kind = [], "stdio-artifacts/decomposer"
    for a in artifacts(t):
        if migrated(t, a):
            continue
        facet = a + "/🪓decomposer"
        if not t.exists(facet):
            out.append(row("%s-missing-%s" % (kind, facet), kind))
            continue
        rs = facet + "/🦀.rs"
        if rs not in t.files:
            out.append(row("%s-rs-%s" % (kind, facet), kind))
        elif not has_impl(t.files[rs], "ArtifactDecomposer"):
            out.append(row("%s-trait-%s" % (kind, facet), kind))
        if facet + "/🟦.ts" not in t.files:
            out.append(row("%s-ts-%s" % (kind, facet), kind))
    return out


def derived(t, which):
    name, facet_dir, hook = DERIVED[which]
    out, checked = [], set()
    for d in dialects(t):
        for owner in (d["artRel"], d["standardRel"], d["subsetRel"]):
            if owner in checked:
                continue
            checked.add(owner)
            if t.exists(owner + "/" + facet_dir):
                out.append(row("artifact-derived-%s-explicit-%s" % (name, owner), "stdio-artifacts/derived-facets"))
        body = t.files.get(d["subsetRel"] + "/🧬schema/🦀.rs", "")
        if "derive_artifact_facets!" in body and hook in body:
            continue
        out.append(row("artifact-derived-%s-missing-%s" % (name, d["subsetRel"]), "stdio-artifacts/derived-facets"))
    return out


def run(c):
    t = Tree(expand(c))
    r, a = c["rule"], c.get("args", [])
    return jsort_rows({
        "policyArtifactIsMigrated": lambda: migrated(t, a[0]), "policyArtifactBuilderBreaches": lambda: builder(t), "policyArtifactDecomposerBreaches": lambda: decomposer(t),
        "policyArtifactBuilderMigratedBreaches": lambda: derived(t, 0), "policyArtifactAnalyzerBreaches": lambda: derived(t, 1), "policyArtifactComposerBreaches": lambda: derived(t, 2),
        "policyStdioCatalogBreaches": lambda: [row("stdio-catalog-owner-table-missing", "stdio-artifacts/catalog")],
    }[r]())


def jsort_rows(v):
    if isinstance(v, list):
        return sorted(v, key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))
    return v


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
