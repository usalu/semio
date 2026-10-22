STEMS = {s.lower() for s in TAXONOMY["bannedNameStems"]}
SUBSET_EXEMPT = {s.lower() for s in TAXONOMY["bannedNameStemExemptions"]["subsetDirectories"]}
VENDORED = set(TAXONOMY["bannedNameStemExemptions"]["vendoredFiles"])
ROOTS = ["✏s", "🧰framework", "🌎hub", "♻mit-bestand"]
SDK = "🧰framework/🛍products/💻os/🔨modules/🔌plugin/"


def row(i, kind, prio="high", line=0):
    return [i, kind, prio, line]


def banned_stems(t):
    out = []

    def walk(rel):
        subset_owner = rel.split("/")[-1] == TAXONOMY["subsetsDirName"]
        for name, is_dir in sorted(t.children(rel).items()):
            if name in SKIP_NAMES or name in ("node_modules", "target", "dist", "pkg"):
                continue
            child = rel + "/" + name
            stem = strip_non_ascii(re.sub(r"\.(rs|ts|tsx|js|mjs|cjs|go|py|cs)$", "", name)).lower()
            if stem in STEMS and not (subset_owner and is_dir and stem in SUBSET_EXEMPT) and child not in VENDORED:
                out.append(row("banned-name-stem-" + child, "taxonomy/banned-name-stem"))
            if is_dir:
                walk(child)

    for root in ROOTS:
        walk(root)
    return out


def plugin_root_shape(t):
    out = []
    for name, is_dir in sorted(t.children("✏s/🔌plugins").items()):
        if not is_dir or name in SKIP_NAMES:
            continue
        owner = "✏s/🔌plugins/" + name
        if t.exists(owner + "/🔌plugin"):
            out.append(row("plugin-root-nested-" + owner, "taxonomy/plugin-root-shape"))
        if not t.exists(owner + "/🦀.rs"):
            out.append(row("plugin-root-leaf-" + owner, "taxonomy/plugin-root-shape"))
        for child in TAXONOMY["pluginRequiredChildDirs"]:
            if not t.exists("%s/%s/🦀.rs" % (owner, child)):
                out.append(row("plugin-root-child-%s/%s" % (owner, child), "taxonomy/plugin-root-shape"))
    return out


def plugin_builder(t, crates):
    out = []
    for c in crates:
        if c["role"] not in ("plugin", "extension") or c["libRelPath"] not in t.files or c["libRelPath"].startswith(SDK):
            continue
        content = t.files[c["libRelPath"]]
        if "semio_plugin!" in content:
            out.append(row("plugin-builder-macro-" + c["libRelPath"], "taxonomy/plugin-builder"))
        if "PluginBundle::new" in content or "PluginBundle {" in content:
            out.append(row("plugin-builder-bundle-" + c["libRelPath"], "taxonomy/plugin-builder"))
    return out


def spr_naming(t, crates):
    out = []
    for c in crates:
        if c["shape"] != "taxonomy" or c["role"] != "plugin":
            continue

        def walk(rel):
            for name, is_dir in sorted(t.children(rel).items()):
                if name in SKIP_NAMES:
                    continue
                child = rel + "/" + name
                if re.match(r"^protocol$", strip_non_ascii(re.sub(r"\.rs$", "", name)), re.I):
                    out.append(row("spr-naming-path-" + child, "taxonomy/spr-naming", "medium"))
                if is_dir:
                    walk(child)

        walk(c["ownerRel"])
        if c["libRelPath"] not in t.files:
            continue
        content = t.files[c["libRelPath"]]
        for m in re.finditer(r"\b(?:mod|struct|enum|type)\s+(\w*[Pp]rotocol\w*)\b", content, A):
            out.append(row("spr-naming-ident-%s-%d" % (c["libRelPath"], utf16_index(content, m.start())), "taxonomy/spr-naming", "medium", content[:m.start()].count("\n") + 1))
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    return {"policyBannedNameStemBreaches": lambda: banned_stems(t), "policyPluginRootShapeBreaches": lambda: plugin_root_shape(t), "policyPluginBuilderBreaches": lambda: plugin_builder(t, c.get("crates", [])), "policySprNamingBreaches": lambda: spr_naming(t, c.get("crates", []))}[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases if c.get("oracle") is not False})
