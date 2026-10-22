SLUG = re.compile(TAXONOMY["exampleSlugPattern"].replace("\\uFE0F", chr(0xFE0F)))
FORBIDDEN_SLUGS = TAXONOMY["forbiddenExampleSlugs"]
PLURALS = TAXONOMY["forbiddenExamplePluralDirs"]
RS_EXT, TS_EXT = TAXONOMY["fileKinds"]["rust-source"]["extensionChains"], TAXONOMY["fileKinds"]["typescript-source"]["extensionChains"]
LEAF_PARENTS = set(TAXONOMY["taxonomyLeafParentDirs"])
FACET_DIRS = set(TAXONOMY["artifactComponentDirs"])
K = "taxonomy/semio-examples"


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def dirs(t, d):
    return [n for n, is_dir in sorted(t.children(d).items()) if is_dir and n not in SKIP_NAMES]


def scope_id(c):
    return c["pluginId"] or strip_non_ascii(c["ownerRel"].split("/")[-1])


def new_surface(c, p):
    return p if c["pluginId"] else "low"


def surface_roots(t, owner):
    roots, arts = [], owner + "/🗿artifacts"
    for a in dirs(t, arts):
        for s in dirs(t, "%s/%s/🏅standards" % (arts, a)):
            sub_root = "%s/%s/🏅standards/%s/🪆subsets" % (arts, a, s)
            for u in dirs(t, sub_root):
                for role in TAXONOMY["surfaceRoles"]:
                    surface = "%s/%s/%s" % (sub_root, u, TAXONOMY["surfaceDirNames"][role])
                    if t.exists(surface):
                        roots.append(surface)
    return roots


def validate_unit(t, rel, prio):
    out, slug = [], rel.split("/")[-1]
    if slug in FORBIDDEN_SLUGS or not SLUG.search(slug):
        out.append(row("semio-examples-slug-" + rel, K, prio))
    for plural in PLURALS:
        if t.exists(rel + "/" + plural):
            out.append(row("semio-examples-plural-%s-%s" % (rel, plural), K, prio))
    if not t.exists(rel + "/🦀.rs"):
        out.append(row("semio-examples-leaf-rs-" + rel, K, prio))
    if not t.exists(rel + "/🟦.ts"):
        out.append(row("semio-examples-leaf-ts-" + rel, K, prio))
    assets = rel + "/🖼assets"
    if not t.exists(assets):
        out.append(row("semio-examples-assets-" + rel, K, prio))
    else:
        for name, is_dir in sorted(t.children(assets).items()):
            if not is_dir and name not in SKIP_NAMES and len(t.files[assets + "/" + name].encode("utf-8")) == 0:
                out.append(row("semio-examples-zero-byte-asset-%s-%s" % (rel, name), K, prio))
    tests = rel + "/🧪tests"
    if not t.exists(tests):
        out.append(row("semio-examples-tests-" + rel, K, prio))
    else:
        if not t.exists(tests + "/🦀.rs"):
            out.append(row("semio-examples-test-rs-" + rel, K, prio))
        if not t.exists(tests + "/🟦.ts"):
            out.append(row("semio-examples-test-ts-" + rel, K, prio))
    return out


def examples(t, crates):
    out = []
    for c in crates:
        owner, prio = c["ownerRel"], new_surface(c, "medium")
        if t.exists(owner + "/📚examples"):
            out.append(row("semio-examples-plugin-root-" + owner, K, new_surface(c, "high")))
        for art in dirs(t, owner + "/🗿artifacts"):
            ex = "%s/🗿artifacts/%s/📚examples" % (owner, art)
            if not t.exists(ex):
                out.append(row("semio-examples-missing-" + ex, K, prio))
                continue
            sets = dirs(t, ex)
            if not sets:
                out.append(row("semio-examples-empty-" + ex, K, prio))
            for s in sets:
                out += validate_unit(t, ex + "/" + s, prio)
        for root in surface_roots(t, owner):
            ex = root + "/📚examples"
            if not t.exists(ex):
                continue
            sets = dirs(t, ex)
            if not sets:
                out.append(row("semio-examples-surface-empty-" + ex, K, prio))
            for s in sets:
                out += validate_unit(t, ex + "/" + s, prio)
    return out


def component_files(t, crates):
    out = []
    for c in crates:
        def walk(rel):
            files = [n for n, d in t.children(rel).items() if not d and n not in SKIP_NAMES]
            parent = rel.split("/")[-1]
            has_rs = any(any(f.endswith(e) for e in RS_EXT) for f in files)
            has_ts = any(any(f.endswith(e) for e in TS_EXT) for f in files)
            parts = rel.split("/")
            prio = new_surface(c, "medium")
            if len(parts) >= 2 and parts[-2] == "📚examples":
                if has_rs and "🦀.rs" not in files:
                    out.append(row("component-file-example-" + rel, "taxonomy/component-file", prio))
                if has_ts and "🟦.ts" not in files:
                    out.append(row("component-file-example-ts-" + rel, "taxonomy/component-file", prio))
            elif parent in LEAF_PARENTS:
                if has_rs and "🦀.rs" not in files:
                    out.append(row("component-file-" + rel, "taxonomy/component-file", prio))
                if parent in FACET_DIRS and has_ts and "🟦.ts" not in files:
                    out.append(row("component-file-ts-" + rel, "taxonomy/component-file", prio))
            for d in dirs(t, rel):
                walk(rel + "/" + d)

        walk(c["ownerRel"] + "/🗿artifacts")
    return out


def run(c):
    t = Tree(expand(c))
    return {"policySemioArtifactExamplesBreaches": examples, "policyComponentFileBreaches": component_files}[c["rule"]](t, c["crates"])


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
