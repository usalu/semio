ENGINE = "⚙engine"
SURFACE_ROLES = TAXONOMY["surfaceRoles"]
SURFACE_DIRS = TAXONOMY["surfaceDirNames"]
LEAVES = {"🦀rust": RS, "🟦typescript": TS}
OS_CONFIG = "🧰framework/🛍products/💻os/🎚config"


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def facet_totality(t, subs):
    return [row("subset-facet-missing-%s/%s" % (s, c), "subset-conformance/facet-totality", "medium") for s in subs for c in TAXONOMY["subsetChildDirs"] if not t.exists(s + "/" + c)]


def artifact_dirs(t):
    out = []
    for plugin, pd in t.children(PLUGINS).items():
        if pd:
            for art, ad in t.children("%s/%s/🗿artifacts" % (PLUGINS, plugin)).items():
                if ad:
                    out.append("%s/%s/🗿artifacts/%s" % (PLUGINS, plugin, art))
    return out


def engine_forbidden(t):
    out = []
    for art in artifact_dirs(t):
        owners = [art]
        standards = art + "/🏅standards"
        for std, sd in t.children(standards).items():
            if not sd:
                continue
            owners.append(standards + "/" + std)
            for sub, ud in t.children("%s/%s/🪆subsets" % (standards, std)).items():
                if ud:
                    owners.append("%s/%s/🪆subsets/%s" % (standards, std, sub))
        out += [row("artifact-engine-forbidden-" + o, "subset-conformance/engine-forbidden", "low") for o in owners if t.exists(o + "/" + ENGINE)]
    return out


def examples_not_at_artifact(t):
    out = []
    for art in artifact_dirs(t):
        if t.exists(art + "/📚examples"):
            out.append(row("artifact-examples-remaining-%s/📚examples" % art, "subset-conformance/example-placement", "medium"))
        for std, sd in t.children(art + "/🏅standards").items():
            if sd and t.exists("%s/🏅standards/%s/📚examples" % (art, std)):
                out.append(row("standard-examples-remaining-%s/🏅standards/%s/📚examples" % (art, std), "subset-conformance/example-placement", "medium"))
    return out


def phantom(t):
    out = []

    def walk(d):
        for name, is_dir in t.children(d).items():
            if not is_dir:
                continue
            child = d + "/" + name
            if "标准" in name:
                out.append(row("phantom-standards-" + child, "subset-conformance/phantom-standards", "medium"))
            walk(child)

    walk(PLUGINS)
    return out


def ts_parity(t, subs):
    out = []
    for s in subs:
        p = s + "/🧬schema/" + TS
        if p in t.files:
            n = len(split_lines(t.files[p]))
            if 0 < n <= 7:
                out.append(row("subset-ts-meta-stub-" + s, "subset-conformance/ts-parity", "medium"))
    return out


def glue_shim(t):
    out = []
    for plugin, pd in t.children(PLUGINS).items():
        glue = "%s/%s/📦packages/🦀rust/🦀.rs" % (PLUGINS, plugin)
        if glue in t.files and re.search(r"Shims:\s*keep pre-migration|pre-migration module paths", t.files[glue], re.I):
            out.append(row("glue-shim-" + glue, "subset-conformance/no-glue-shim-blocks", "medium"))
    return out


def surface_completeness(t, subs):
    out = []
    for s in subs:
        if not t.exists(s + "/🧬schema"):
            continue
        for role in SURFACE_ROLES:
            surface = "%s/%s" % (s, SURFACE_DIRS[role])
            if not t.exists(surface):
                out.append(row("subset-surface-missing-" + surface, "taxonomy/surface-completeness", "medium"))
                continue
            modes = surface + "/" + TAXONOMY["modesDirName"]
            complete = False
            for mode, md in t.children(modes).items():
                if not md:
                    continue
                windows = "%s/%s/%s" % (modes, mode, TAXONOMY["windowsDirName"])
                for w, wd in t.children(windows).items():
                    if wd and all((windows + "/" + w + "/" + LEAVES[lang]) in t.files for lang in TAXONOMY["windowLeafLangs"]):
                        complete = True
            if not complete:
                out.append(row("subset-surface-window-incomplete-" + surface, "taxonomy/surface-completeness", "medium"))
            if any("SCAFFOLD" in t.files[p] for p in t.walk([surface], lambda p, n: n in (RS, TS))):
                out.append(row("subset-surface-scaffold-residue-" + surface, "taxonomy/surface-scaffold-residue", "medium"))
    return out


def viewer_purity(t, subs):
    out = []
    pats = [(r"\.mutation\(", ".mutation("), (r"Emit::mutations", "Emit::mutations"), (r"artifact_mutations", "artifact_mutations"), (r"::editor::", "::editor::")]
    for s in subs:
        viewer = "%s/%s" % (s, TAXONOMY["viewerDirName"])
        if not t.exists(viewer):
            continue
        for p in t.walk([viewer], lambda p, n: True):
            for rx, label in pats:
                if re.search(rx, t.files[p]):
                    out.append(row("viewer-purity-%s-%s" % (p, label), "taxonomy/viewer-purity", "high"))
    return out


def own_components(t, owner):
    return t.walk([owner], lambda p, n: n == RS, skip=lambda rel, name: name == "🧩extensions")


def contributed_target(t, subs):
    out, owners = [], {}
    root_of = lambda s: "/".join(s.split("/")[:3])
    for s in subs:
        if t.exists(s + "/🧬schema"):
            owners.setdefault(s[len(root_of(s)) + 1:], root_of(s))
    for s in subs:
        if t.exists(s + "/🧬schema"):
            continue
        plugin = root_of(s)
        owner = owners.get(s[len(plugin) + 1:])
        for role in SURFACE_ROLES:
            surface = "%s/%s" % (s, SURFACE_DIRS[role])
            if not t.exists(surface):
                continue
            if not owner:
                out.append(row("contributed-surface-unknown-owner-" + surface, "plugin-dependency/contributed-surface-target", "medium"))
                continue
            if owner == plugin:
                continue
            owner_id = strip_non_ascii(owner.split("/")[-1])
            if any(re.search(r'\.depends_on\s*\(\s*"%s"' % re.escape(owner_id), t.files[p]) for p in own_components(t, plugin)):
                continue
            out.append(row("contributed-surface-target-undeclared-" + surface, "plugin-dependency/contributed-surface-target", "high"))
    return out


def os_config(t):
    if not t.exists(OS_CONFIG + "/🧬schema"):
        return [row("os-config-shape-missing-schema", "taxonomy/os-config-shape", "high")]
    return None


def run(c):
    t = Tree(expand(c))
    subs = top_level_subsets(t)
    r = c["rule"]
    if r == "policyListTopLevelSubsetDirs":
        return subs
    if r == "policySubsetConformanceBreaches":
        return facet_totality(t, subs) + engine_forbidden(t) + examples_not_at_artifact(t) + phantom(t) + ts_parity(t, subs) + glue_shim(t)
    return {
        "policySubsetFacetTotalityBreaches": lambda: facet_totality(t, subs), "policyArtifactEngineFacetForbiddenBreaches": lambda: engine_forbidden(t),
        "policyExampleNotAtArtifactLevelBreaches": lambda: examples_not_at_artifact(t), "policyPhantomStandardsBreaches": lambda: phantom(t),
        "policySubsetTsParityBreaches": lambda: ts_parity(t, subs), "policyNoGlueShimBlocksBreaches": lambda: glue_shim(t),
        "policySubsetSurfaceCompletenessBreaches": lambda: surface_completeness(t, subs), "policyViewerPurityBreaches": lambda: viewer_purity(t, subs),
        "policyContributedSurfaceTargetBreaches": lambda: contributed_target(t, subs), "policyOsConfigShapeBreaches": lambda: os_config(t),
    }[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
