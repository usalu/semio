def kind_file(kind):
    k = TAXONOMY["fileKinds"][kind]
    return k["emoji"] + k["extensionChains"][0]


MARKER = kind_file(TAXONOMY["windowEmptyFacetFileKindId"])
COMPONENTS = {k: kind_file(v) for k, v in TAXONOMY["componentFileKinds"].items()}
COMPONENT_FILES = set(COMPONENTS.values())
SCHEMA_DIRS, REPS, IO_DIRS, IO_CHILD = TAXONOMY["schemaChildDirs"], TAXONOMY["representationDirs"], TAXONOMY["ioDirectionDirs"], TAXONOMY["ioDirectionChildDirs"]
ARTIFACT_CHILDREN = TAXONOMY["artifactChildDirs"] + TAXONOMY["newArtifactChildDirs"]


def dirs(t, d):
    return [n for n, is_dir in sorted(t.children(d).items()) if is_dir and n not in SKIP_NAMES]


def scope_id(crate):
    return crate["pluginId"] or strip_non_ascii(crate["ownerRel"].split("/")[-1])


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def surface_roots(t, owner):
    roots = []
    arts = owner + "/" + TAXONOMY["artifactsDirName"]
    for a in dirs(t, arts):
        std_root = "%s/%s/%s" % (arts, a, TAXONOMY["standardsDirName"])
        for s in dirs(t, std_root):
            sub_root = "%s/%s/%s" % (std_root, s, TAXONOMY["subsetsDirName"])
            for u in dirs(t, sub_root):
                for role in TAXONOMY["surfaceRoles"]:
                    surface = "%s/%s/%s" % (sub_root, u, TAXONOMY["surfaceDirNames"][role])
                    if t.exists(surface):
                        roots.append(surface)
    return roots


def taxonomy_dirs(t, crates):
    out = []
    for crate in crates:
        owner, prio = crate["ownerRel"], "medium" if crate["pluginId"] else "low"
        sid = scope_id(crate)
        add = lambda i: out.append(row("taxonomy-dirs-" + i, "taxonomy/dirs", prio))

        def schema_facet(nested_root):
            for nested in dirs(t, nested_root):
                if nested not in SCHEMA_DIRS:
                    add("artifact-%s-%s" % (nested_root, nested))

        def io_facet(nested_root):
            for nested in dirs(t, nested_root):
                if nested in REPS:
                    continue
                if nested not in IO_DIRS:
                    add("artifact-%s-%s" % (nested_root, nested))
                    continue
                expected = IO_CHILD.get(nested)
                direction = nested_root + "/" + nested
                for codec in dirs(t, direction):
                    if expected and codec == expected:
                        codec_root = direction + "/" + codec
                        for mid in dirs(t, codec_root):
                            if mid != TAXONOMY["artifactsDirName"]:
                                add("artifact-%s-%s" % (codec_root, mid))
                        continue
                    add("artifact-%s-%s" % (direction, codec))

        arts = owner + "/" + TAXONOMY["artifactsDirName"]
        for art in dirs(t, arts):
            art_dir = arts + "/" + art
            for child in dirs(t, art_dir):
                if child not in ARTIFACT_CHILDREN:
                    add("artifact-%s-%s" % (art_dir, child))
                    continue
                if child == "🧬schema":
                    schema_facet(art_dir + "/" + child)
                elif child == "🚪io":
                    io_facet(art_dir + "/" + child)
                elif child == TAXONOMY["standardsDirName"]:
                    std_root = art_dir + "/" + child
                    for std in dirs(t, std_root):
                        std_dir = std_root + "/" + std
                        for sc in dirs(t, std_dir):
                            if sc not in TAXONOMY["standardChildDirs"]:
                                add("standard-%s-%s" % (std_dir, sc))
                                continue
                            if sc != TAXONOMY["subsetsDirName"]:
                                continue
                            sub_root = std_dir + "/" + sc
                            for sub in dirs(t, sub_root):
                                sub_dir = sub_root + "/" + sub
                                for subc in dirs(t, sub_dir):
                                    if subc not in TAXONOMY["subsetChildDirs"]:
                                        add("subset-%s-%s" % (sub_dir, subc))
                                        continue
                                    if subc == "🧬schema":
                                        schema_facet(sub_dir + "/" + subc)
                                    elif subc == "🚪io":
                                        io_facet(sub_dir + "/" + subc)
                                    elif subc in TAXONOMY["surfaceDirNames"].values():
                                        surface = sub_dir + "/" + subc
                                        for sf in dirs(t, surface):
                                            if sf not in TAXONOMY["surfaceChildDirs"]:
                                                add("surface-%s-%s" % (surface, sf))

        def walk_windows(rel):
            for entry in dirs(t, rel):
                child = rel + "/" + entry
                if entry == TAXONOMY["windowsDirName"]:
                    for w in dirs(t, child):
                        wdir = child + "/" + w
                        for c in dirs(t, wdir):
                            if c in TAXONOMY["windowChildDirs"]:
                                continue
                            if TAXONOMY["windowsDirName"] in TAXONOMY["testOwnerKinds"] and c in (TAXONOMY["testsDirName"], TAXONOMY["testFixturesDirName"]):
                                continue
                            add("window-%s-%s" % (wdir, c))
                    continue
                walk_windows(child)

        for root in surface_roots(t, owner):
            walk_windows(root)
    return out


def window_completeness(t, crates):
    out = []
    for crate in crates:
        def walk(rel):
            for entry in dirs(t, rel):
                child = rel + "/" + entry
                if entry != TAXONOMY["windowsDirName"]:
                    walk(child)
                    continue
                for w in dirs(t, child):
                    wdir = child + "/" + w
                    actual = set(dirs(t, wdir))
                    for req in TAXONOMY["windowRequiredChildDirs"]:
                        cap = wdir + "/" + req
                        if req not in actual:
                            out.append(row("taxonomy-window-completeness-%s-%s" % (wdir, req), "taxonomy/window-completeness", "high"))
                            continue
                        members = dirs(t, cap)
                        marker = cap + "/" + MARKER
                        for f in sorted(COMPONENT_FILES, key=lambda s: s.encode("utf-16-be")):
                            if t.exists(cap + "/" + f):
                                out.append(row("taxonomy-window-facet-component-%s-%s" % (cap, f), "taxonomy/window-facet-component", "high"))
                        if not members:
                            if not t.exists(marker):
                                out.append(row("taxonomy-window-empty-facet-" + cap, "taxonomy/window-empty-facet", "high"))
                            continue
                        if t.exists(marker):
                            out.append(row("taxonomy-window-populated-facet-marker-" + cap, "taxonomy/window-empty-facet", "high"))
                        for m in members:
                            for lang in TAXONOMY["windowComponentLangs"]:
                                if not t.exists("%s/%s/%s" % (cap, m, COMPONENTS[lang])):
                                    out.append(row("taxonomy-window-component-%s/%s-%s" % (cap, m, lang), "taxonomy/window-component", "high"))
        for root in surface_roots(t, crate["ownerRel"]):
            walk(root)
    return out


def mode_completeness(t, crates):
    out, required = [], TAXONOMY["modeRequiredChildDirs"]
    for crate in crates:
        def walk(rel):
            for entry in dirs(t, rel):
                child = rel + "/" + entry
                if entry != TAXONOMY["modesDirName"]:
                    walk(child)
                    continue
                for mode in dirs(t, child):
                    mdir = child + "/" + mode
                    actual = set(dirs(t, mdir))
                    for c in required:
                        cdir, marker = mdir + "/" + c, mdir + "/" + c + "/" + MARKER
                        if c not in actual:
                            out.append(row("taxonomy-mode-completeness-%s-%s" % (mdir, c), "taxonomy/mode-completeness", "high"))
                            continue
                        members = dirs(t, cdir)
                        if not members:
                            if not t.exists(marker):
                                out.append(row("taxonomy-mode-empty-child-" + cdir, "taxonomy/mode-empty-child", "high"))
                            continue
                        if t.exists(marker):
                            out.append(row("taxonomy-mode-populated-child-marker-" + cdir, "taxonomy/mode-empty-child", "high"))
        for root in surface_roots(t, crate["ownerRel"]):
            walk(root)
    return out


def run(c):
    t = Tree(expand(c))
    return {"policyTaxonomyDirsBreaches": taxonomy_dirs, "policyWindowCompletenessBreaches": window_completeness, "policyModeCompletenessBreaches": mode_completeness}[c["rule"]](t, c["crates"])


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
