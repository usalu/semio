KIND = "taxonomy/owner-shape/"
SCAN_ROOTS = ["✏s/🔌plugins", "✏s/🔨modules", "🧰framework"]
IO_PATTERNS = [(r"\bparse_dsl\s*\(", "parse_dsl("), (r"\bprint_dsl\s*\(", "print_dsl("), (r"\bencode_pack\s*\(", "encode_pack("), (r"\bdecode_pack\s*\(", "decode_pack("), (r"\bArtifactDsl::", "ArtifactDsl::"), (r"\bArtifactPack::", "ArtifactPack::"), (r"include_bytes!", "include_bytes!"), (r"\bstd::fs::", "std::fs::"), (r"\bsemio_s_plugin_\w+::[\w:]*\bio\b", "semio_s_plugin_<other>::…::io")]


def row(i, sub, line=0):
    return [i, KIND + sub, "medium", line]


def module_slug(name):
    s = strip_non_ascii(name).replace("-", "_")
    return "_" + s if re.match(r"^[0-9]", s) else s


def standard_slug(name):
    s = re.sub(r"[.\-]", "_", strip_non_ascii(name))
    if re.match(r"^v[0-9]", s) or re.match(r"^v_", s):
        return s
    return "v" + s if re.match(r"^[0-9]", s) else "v_" + s


def split_subset(sub):
    p = sub.split("/")
    return {"sub": sub, "std": "/".join(p[:7]), "art": "/".join(p[:5]), "plugin": "/".join(p[:3]), "subSlug": module_slug(p[8]), "stdSlug": standard_slug(p[6]), "artSlug": module_slug(p[4])}


def mount_targets(content):
    return re.findall(r'#\[path\s*=\s*"([^"]+)"\]', content)


def existing_children(t, owner, allowed):
    return [n for n, d in t.children(owner).items() if d and n in allowed]


def owner_mounts(t, splits):
    out = []

    def check(rel, allowed):
        root = rel + "/" + RS
        if root not in t.files:
            out.append(row("owner-mounts-children-missing-root-" + rel, "owner-mounts-children"))
            return
        if existing_children(t, rel, allowed) and not mount_targets(t.files[root]):
            out.append(row("owner-mounts-children-not-self-mounting-" + rel, "owner-mounts-children"))

    for rel in sorted({s["art"] for s in splits}):
        check(rel, TAXONOMY["artifactChildDirs"])
    for rel in sorted({s["std"] for s in splits}):
        check(rel, TAXONOMY["standardChildDirs"])
    for s in splits:
        check(s["sub"], TAXONOMY["subsetChildDirs"])
    for plugin in sorted({s["plugin"] for s in splits}):
        glue = plugin + "/📦packages/🦀rust/🦀.rs"
        if glue not in t.files:
            continue
        content = t.files[glue]
        deep = ["🗿artifacts/", "🏅standards/", "🪆subsets/", "🧬schema/", "🚪io/", "👁viewer/", "✏editor/"]
        out_of_scope = []
        for tg in mount_targets(content):
            if "/🔨modules/" in tg or "/🎮commands/" in tg:
                continue
            if tg.endswith("/" + RS) and re.search(r"/🗿artifacts/[^/]+$", tg[:-len(RS) - 1]):
                continue
            if not any(d in tg for d in deep) and tg.endswith(RS):
                continue
            out_of_scope.append(tg)
        if out_of_scope:
            out.append(row("owner-mounts-children-glue-scope-" + plugin, "owner-mounts-children"))
        if re.search(r"pub use\s+[^;]*::(?:subsets|standards)::[^;]*;", content):
            out.append(row("owner-mounts-children-glue-shim-" + plugin, "owner-mounts-children"))
    return out


def isolation(t, splits):
    out = []
    for s in splits:
        files = t.walk([s["sub"]], lambda p, n: n.endswith(".rs") or n.endswith(".ts"))
        cross_std = re.compile(r"artifacts::%s::standards::(?!%s\b)(\w+)" % (re.escape(s["artSlug"]), re.escape(s["stdSlug"])), A)
        cross_sub = re.compile(r"artifacts::%s::standards::%s::subsets::(?!%s\b)(\w+)" % (re.escape(s["artSlug"]), re.escape(s["stdSlug"]), re.escape(s["subSlug"])), A)
        for p in files:
            c = t.files[p]
            if p.endswith(".rs"):
                targets = set()
                for m in cross_sub.finditer(c):
                    targets.add(m.group(1))
                    out.append(row("subset-isolation-cross-subset-%s-%d" % (p, utf16_index(c, m.start())), "subset-isolation", c[:m.start()].count("\n") + 1))
                for m in cross_std.finditer(c):
                    if m.group(1) == s["stdSlug"] or m.group(1) in targets:
                        continue
                    out.append(row("subset-isolation-cross-standard-%s-%d" % (p, utf16_index(c, m.start())), "subset-isolation", c[:m.start()].count("\n") + 1))
            else:
                depth = len(p[len(s["sub"]) + 1:].split("/")) - 1
                for m in re.finditer(r"""from\s+["'](\.\.[^"']*)["']""", c):
                    spec = m.group(1)
                    if spec.count("../") <= depth:
                        continue
                    resolved = posixpath.normpath(posixpath.join(posixpath.dirname(p), spec))
                    if re.search(r"(^|/)🔨modules(/|$)", resolved) or resolved.startswith("🧰framework/") or (resolved.startswith(PLUGINS + "/") and not resolved.startswith(s["plugin"] + "/")):
                        continue
                    out.append(row("subset-isolation-ts-climb-%s-%d" % (p, utf16_index(c, m.start())), "subset-isolation", c[:m.start()].count("\n") + 1))
    return out


def find_dirs_named(t, roots, name):
    found = set()
    for root in roots:
        stack = [root]
        while stack:
            d = stack.pop()
            for n, is_dir in t.children(d).items():
                if not is_dir or n in SKIP_NAMES:
                    continue
                child = d + "/" + n
                if n == name:
                    found.add(child)
                else:
                    stack.append(child)
    return sorted(found)


def nearest_owner(path):
    for rx in (r"^(.*/🪆subsets/[^/]+)/", r"^(.*/🏅standards/[^/]+)/", r"^(.*/🗿artifacts/[^/]+)/"):
        m = re.match(rx, path)
        if m:
            return m.group(1)
    return "/".join(path.split("/")[:3])


def consumers(t):
    out = []
    for mdir in find_dirs_named(t, SCAN_ROOTS, "🔨modules"):
        subset_level = re.search(r"/🪆subsets/[^/]+/🔨modules$", mdir) is not None
        for unit, is_dir in sorted(t.children(mdir).items()):
            if not is_dir or unit in SKIP_NAMES:
                continue
            unit_rel = mdir + "/" + unit
            if subset_level:
                out.append(row("module-consumer-subset-level-" + unit_rel, "module-consumer-count"))
                continue
            token = "modules::" + module_slug(unit)
            roots = ["/".join(mdir.split("/")[:3])] if mdir.startswith(PLUGINS + "/") else SCAN_ROOTS
            match = t.walk(roots, lambda p, n: n.endswith(".rs") and not p.startswith(unit_rel + "/") and token in t.files[p])
            if len({nearest_owner(f) for f in match}) < 2:
                out.append(row("module-consumer-count-" + unit_rel, "module-consumer-count"))
    return out


def strip_cfg_test(content):
    marker, out, i = "#[cfg(test)]", "", 0
    while i < len(content):
        idx = content.find(marker, i)
        if idx == -1:
            out += content[i:]
            break
        out += content[i:idx]
        brace = content.find("{", idx)
        if brace == -1:
            out += content[idx:]
            break
        depth, j = 0, brace
        while j < len(content):
            if content[j] == "{":
                depth += 1
            elif content[j] == "}":
                depth -= 1
                if depth == 0:
                    j += 1
                    break
            j += 1
        i = j
    return out


def io_exclusivity(t):
    out = []
    for p in t.walk(SCAN_ROOTS, lambda p, n: n.endswith(".rs") and "/🚪io/" not in p and "/🧪tests/" not in p):
        c = strip_cfg_test(t.files[p])
        for rx, label in IO_PATTERNS:
            m = re.search(rx, c, A)
            if m:
                out.append(row("io-exclusivity-%s-%s" % (p, label), "io-exclusivity", c[:m.start()].count("\n") + 1))
    return out


def io_declaration(t, splits):
    out = []
    for s in splits:
        io = s["sub"] + "/🚪io"
        if not t.exists(io):
            continue
        root_file = io + "/" + RS
        root = t.files.get(root_file, "")
        for leaf in t.walk([io], lambda p, n: n == RS and p != root_file):
            d = leaf[:-(len(RS) + 1)]
            if d.split("/")[-1] not in root:
                out.append(row("io-declaration-unreachable-" + leaf, "io-declaration"))
            if d + "/" + TS not in t.files:
                out.append(row("io-declaration-missing-ts-twin-" + d, "io-declaration"))
    return out


def standalone(t, splits):
    out = []
    for s in splits:
        root = s["sub"] + "/🧬schema/" + RS
        if root not in t.files:
            continue
        c = t.files[root]
        for m in re.finditer(r"pub use\s+[\w:]*subsets::(\w+)[\w:]*::\*", c, A):
            if m.group(1) != s["subSlug"]:
                out.append(row("subset-standalone-bare-reexport-%s-%d" % (root, utf16_index(c, m.start())), "subset-standalone", c[:m.start()].count("\n") + 1))
        if not any(re.search(r"(?:^|[^\w])struct\s+\w*Snapshot\b", t.files[f], A) for f in t.walk([s["sub"] + "/🧬schema"], lambda p, n: n.endswith(".rs"))):
            out.append(row("subset-standalone-no-own-snapshot-" + s["sub"], "subset-standalone"))
    return out


def declaration_tree(t, splits):
    out = []
    for s in splits:
        art, std, sub = (t.files.get(s[k] + "/" + RS, "") for k in ("art", "std", "sub"))
        if re.search(r"pub fn artifact\(\)\s*->\s*ArtifactDeclaration", art) and re.search(r"pub fn standard\(\)\s*->\s*StandardDeclaration", std) and s["stdSlug"] not in art:
            out.append(row("declaration-tree-artifact-missing-standard-%s-%s" % (s["art"], s["stdSlug"]), "declaration-tree"))
        if re.search(r"pub fn standard\(\)\s*->\s*StandardDeclaration", std) and re.search(r"pub fn subset\(\)\s*->\s*SubsetDeclaration", sub) and s["subSlug"] not in std:
            out.append(row("declaration-tree-standard-missing-subset-%s-%s" % (s["std"], s["subSlug"]), "declaration-tree"))
    for plugin in sorted({s["plugin"] for s in splits}):
        glue = t.files.get(plugin + "/📦packages/🦀rust/🦀.rs", "")
        if not glue:
            continue
        for art in sorted({s["art"] for s in splits if s["plugin"] == plugin}):
            if not re.search(r"pub fn artifact\(\)\s*->\s*ArtifactDeclaration", t.files.get(art + "/" + RS, "")):
                continue
            slug = module_slug(art.split("/")[-1])
            if ".artifact(" not in glue or slug not in glue:
                out.append(row("declaration-tree-plugin-missing-artifact-%s-%s" % (plugin, slug), "declaration-tree"))
    return out


def clean_mechanism(t):
    splits = [split_subset(s) for s in top_level_subsets(t)]
    return owner_mounts(t, splits) + isolation(t, splits) + consumers(t) + io_exclusivity(t) + io_declaration(t, splits) + standalone(t, splits) + declaration_tree(t, splits)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: clean_mechanism(Tree(expand(c))) for c in cases})
