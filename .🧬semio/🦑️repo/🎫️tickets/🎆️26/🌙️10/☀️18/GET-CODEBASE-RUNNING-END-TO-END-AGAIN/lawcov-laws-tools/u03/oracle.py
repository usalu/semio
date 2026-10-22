import hashlib, posixpath

FAMILY_ROOT = "🧰framework/🛍products/💻os/🔨modules/🗣dsl/👪family"
GRAMMAR, PROTOCOL, RS = "📖.grammar.semio", "📡.protocol.semio", "🦀.rs"
FACETS = ["🗣dsl", "🔧op", "🔺diff", "🎒pack", "📡spr", "🧬mutations"]
VERBS = set("create delete insert remove add rename change update move drag resize rotate scale reorder edit replace duplicate connect disconnect bind unbind group ungroup flatten unflatten split merge extract inline clear fix toggle apply set format open load save import export lint request select hover evaluate run retry calibrate place commit input submit abort patch paint nudge reset focus step query fill snap ingest".split())
RESERVED = {"grammar", "start", "extension", "dialect", "use", "schema", "protocol"}


def hb(rule, id_, kind, line=0, priority="high"):
    return [id_, kind, priority, line]


def normalize(raw):
    out = []
    for line in split_lines(raw):
        if re.match(r"^(grammar|protocol|extension|schema)\s+\S+", line):
            line = re.sub(r"^(grammar|protocol|extension|schema)\s+\S+", r"\1 __NAME__", line)
        elif re.match(r"^start\s+\S+", line):
            line = "start __START__"
        out.append(line)
    return "\n".join(out)


def distinctness(t):
    groups = {}
    for p in t.walk(["✏s", "🧰framework"], lambda p, n: n.endswith(".grammar.semio") or n.endswith(".protocol.semio")):
        groups.setdefault(hashlib.sha256(normalize(t.files[p]).encode("utf-8")).hexdigest(), []).append(p)
    out = []
    for h, g in groups.items():
        s = jssort(g)
        for i in range(len(s)):
            for j in range(i + 1, len(s)):
                out.append(["spec-distinctness-%s-%s-%s" % (h[:12], s[i], s[j]), "handcrafted-grammar/spec-distinctness", "high", 0])
    return out


def generic_spec(t):
    out = []
    for p in t.walk(["✏s"], lambda p, n: n.endswith(".grammar.semio")):
        c = t.files[p]
        reasons = []
        if re.search(r'prop\s*=\s*IDENT\s*"="\s*\(', c) or re.search(r"prop\s*=\s*IDENT\s*=", c):
            reasons.append("catch-all")
        untyped = [n for n in ("list", "map", "value") if re.search(r"^\s*%s\s*=" % n, c, re.M)]
        if re.search(r"\bprop\b", c) and untyped:
            reasons.append("untyped")
        if re.search(r"=\s*IDENT\s+assign\*\s+block\?", c) or re.search(r"=\s*IDENT\s+assign\*", c):
            reasons.append("shell")
        if re.search(r'-(json|blob|base64|payload)"', c) or re.search(r"-(json|blob|base64|payload)\b", c):
            reasons.append("field")
        if reasons:
            out.append(["generic-spec-" + p, "handcrafted-grammar/generic-spec", "high", 0])
    return out


def declared_use(t):
    fams = {}
    for p in t.walk([FAMILY_ROOT], lambda p, n: n.startswith("📖family-") and n.endswith(".grammar.semio")):
        fid = re.sub(r"\.grammar\.semio$", "", re.sub(r"^📖", "", p.split("/")[-1]))
        prods = [m.group(1) for m in re.finditer(r"^([A-Za-z_][\w-]*)\s*=", t.files[p], re.M | A) if m.group(1) not in RESERVED]
        fams[fid] = (p, prods)
    out = []
    for p in t.walk(["✏s"], lambda p, n: n.endswith(".grammar.semio")):
        c = t.files[p]
        for fam in [m.group(1) for m in re.finditer(r"^use\s+(family-[\w-]+)", c, re.M | A)]:
            if fam not in fams:
                out.append(["declared-use-missing-%s-%s" % (p, fam), "handcrafted-grammar/declared-use", "high", 0])
                continue
            prods = fams[fam][1]
            if not prods:
                continue
            if any(re.search(r"\b%s\b" % re.escape(x), c) for x in prods):
                continue
            out.append(["declared-use-%s-%s" % (p, fam), "handcrafted-grammar/declared-use", "high", 0])
    return out


def has_register(t, art):
    return any("register_language" in t.files[p] for p in t.walk([art], lambda p, n: n.endswith(".rs")))


def artifact_dirs(t):
    out = []
    for plugin, is_dir in t.children("✏s/🔌plugins").items():
        if not is_dir:
            continue
        for art, art_dir in t.children("✏s/🔌plugins/%s/🗿artifacts" % plugin).items():
            if art_dir:
                out.append("✏s/🔌plugins/%s/🗿artifacts/%s" % (plugin, art))
    return jssort(out)


def spec_wiring(t):
    out = []
    for art in artifact_dirs(t):
        any_spec = False
        for facet in FACETS:
            facet_rel = art + "/" + facet
            if not t.exists(facet_rel):
                continue
            has_g, has_p = t.exists(facet_rel + "/" + GRAMMAR), t.exists(facet_rel + "/" + PROTOCOL)
            if not has_g and not has_p:
                continue
            any_spec = True
            rs = facet_rel + "/" + RS
            if not t.exists(rs):
                continue
            body = t.files[rs]
            for leaf in ([GRAMMAR] if has_g else []) + ([PROTOCOL] if has_p else []):
                if not ("include_str!" in body and leaf in body):
                    out.append(["spec-wiring-include-%s-%s" % (rs, leaf), "handcrafted-grammar/spec-wiring-include", "high", 0])
        if any_spec and not has_register(t, art):
            out.append(["spec-wiring-register-" + art, "handcrafted-grammar/spec-wiring-register", "high", 0])
    return out


def empty_example(t):
    out = []
    pred = lambda p, n: n.endswith(".semio") and ("/📚examples/" in p or p.startswith("📚examples/")) and ("/🖼assets/" in p)
    for p in t.walk([""], pred):
        size = len(t.files[p].encode("utf-8"))
        if size <= 64:
            out.append(["empty-example-" + p, "handcrafted-grammar/empty-example", "high", 0])
    return out


def codec_derive(t):
    out = []
    banned = [r"dsl::__rt::parse_document_record|dsl::__rt::print_document_record|dsl::__rt::parse_inline_record|dsl::__rt::print_inline_record", r"dsl::op_rt::|store::op_rt::"]
    for p in t.walk(["✏s/🔌plugins"], lambda p, n: "/🗿artifacts/" in p and n.endswith(".rs")):
        for rx in banned:
            for m in re.finditer(rx, t.files[p]):
                line = len(re.split(r"\r?\n", t.files[p][:m.start()]))
                out.append(["generic-codec-runtime-%s-%d" % (p, line), "handcrafted-grammar/generic-codec-derive", "high", line])
    return out


def p3(t):
    return distinctness(t) + generic_spec(t) + declared_use(t) + spec_wiring(t) + empty_example(t) + codec_derive(t)


def strip_non_ascii(s):
    return re.sub(r"[^\x00-\x7f]", "", s)


def dependency_owner_roots(t):
    roots = []
    for plugin, is_dir in sorted(t.children("✏s/🔌plugins").items()):
        if not is_dir or plugin.startswith("."):
            continue
        rel = "✏s/🔌plugins/" + plugin
        roots.append(rel)
        for ext, e_dir in sorted(t.children(rel + "/🧩extensions").items()):
            if e_dir and not ext.startswith("."):
                roots.append(rel + "/🧩extensions/" + ext)
    return roots


def own_component_files(t, owner):
    return t.walk([owner], lambda p, n: n == RS, skip=lambda rel, name: name == "🧩extensions")


def semio_metadata(t, owner):
    manifest = t.files.get(owner + "/📦packages/🦀rust/Cargo.toml", "")
    block = next((b for b in re.split(r"(?=\n\[)", manifest) if b.lstrip().startswith("[package.metadata.semio]")), "")
    ext = re.search(r'^extends\s*=\s*"([a-z0-9-]+)"', block, re.M)
    lst = re.search(r"^depends-on\s*=\s*\[([^\]]*)\]", block, re.M)
    return set(re.findall(r'"([a-z0-9-]+)"', lst.group(1) if lst else "")), (ext.group(1) if ext else None)


def dependency_parity(t):
    out = []
    for owner in dependency_owner_roots(t):
        declared = set()
        for p in own_component_files(t, owner):
            declared |= set(re.findall(r'\.depends_on\s*\(\s*"([a-z0-9-]+)"', t.files[p]))
        meta, extends = semio_metadata(t, owner)
        own = strip_non_ascii(owner.split("/")[-1])
        for i in jssort(declared):
            if i in meta or i == extends:
                continue
            out.append(["plugin-dependency-undeclared-metadata-%s-%s" % (owner, i), "plugin-dependency/parity", "high", 0])
        for i in jssort(meta):
            if i == own or i in declared:
                continue
            out.append(["plugin-dependency-undeclared-runtime-%s-%s" % (owner, i), "plugin-dependency/parity", "high", 0])
    return out


def contribution_targets(t):
    out = []
    for owner in dependency_owner_roots(t):
        declared, targets = set(), {}
        for p in own_component_files(t, owner):
            c = t.files[p]
            declared |= set(re.findall(r'\.depends_on\s*\(\s*"([a-z0-9-]+)"', c))
            for m in re.finditer(r'ArtifactContribution::builder\s*\(\s*"([^"]+)"', c):
                targets[m.group(1)] = p
        for kind, p in targets.items():
            parts = kind.split(".")
            if len(parts) < 2 or not parts[1]:
                out.append(["contribution-target-ungrammatical-%s-%s" % (p, kind), "plugin-dependency/contribution-target", "high", 0])
                continue
            if parts[1] in declared:
                continue
            out.append(["contribution-target-undeclared-%s-%s" % (owner, kind), "plugin-dependency/contribution-target", "high", 0])
    return out


def owned_roots():
    tx = json.load(open(taxonomy_file, encoding="utf8"))
    cands = {"✏s", "🧰framework", "🌎hub", "♻mit-bestand"} | {a for a, s in tx["areas"].items() if s == "clean"}
    cands = [c for c in cands if c != "compose" and not c.startswith("compose/")]
    return sorted(c for c in cands if not any(o != c and c.startswith(o + "/") for o in cands))


taxonomy_file = os.path.dirname(os.path.abspath(__file__))
while not os.path.exists(os.path.join(taxonomy_file, "🔣taxonomy.json")):
    taxonomy_file = os.path.dirname(taxonomy_file)
taxonomy_file = os.path.join(taxonomy_file, "🔣taxonomy.json")


def vocabulary(t):
    out = []
    pred = lambda p, n: n.endswith(".rs") and ("/🧬mutations/" in p or "/🎮commands/" in p)
    for p in t.walk(owned_roots(), pred):
        c = t.files[p]
        if re.search(r"SetSnapshot", c) or re.search(r"NoMutation", c) or re.search(r"CollectionMutation(<|::)", c):
            out.append(["semantic-vocabulary-banned-" + p, "mutation-migration/semantic-vocabulary", "high", 0])
        if any(n != "SetSnapshot" for n in re.findall(r"\bSet[A-Z]\w*", c, A)):
            out.append(["semantic-vocabulary-bare-set-" + p, "mutation-migration/semantic-vocabulary", "medium", 0])
    return out


def collect_glue(t, glue):
    declared, lib_dir = set(), posixpath.dirname(glue)
    stack, pending = [lib_dir], None
    for raw in split_lines(t.files[glue]):
        line = raw.strip()
        pm = re.search(r'#\[path\s*=\s*"([^"]+)"\]', line)
        if pm:
            pending = pm.group(1)
            continue
        mm = re.match(r"^(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)", line)
        if mm:
            base = stack[-1] if stack else lib_dir
            if pending is None:
                resolved = posixpath.normpath(posixpath.join(base, mm.group(1)))
            elif pending == ".":
                resolved = base
            else:
                resolved = posixpath.normpath(posixpath.join(base, pending))
            pending = None
            as_file = resolved if resolved.endswith(".rs") else resolved + ".rs"
            as_mod = resolved + "/mod.rs"
            if as_file in t.files:
                declared.add(as_file)
            elif as_mod in t.files:
                declared.add(as_mod)
            else:
                declared.add(as_file)
            if "{" in line:
                stack.append(posixpath.dirname(resolved) if resolved.endswith(".rs") else resolved)
            continue
        pending = None
        for _ in range(line.count("{")):
            stack.append(stack[-1] if stack else lib_dir)
        for _ in range(line.count("}")):
            if len(stack) > 1:
                stack.pop()
    return declared


def dead_example(t):
    reachable = set()
    for glue in t.walk(["✏s/🔌plugins"], lambda p, n: n == RS):
        reachable |= collect_glue(t, glue)
    fw = next((n for n in sorted(t.children("")) if n.endswith("framework")), None)
    roots = ["✏s/🔌plugins"] + ([fw] if fw else [])
    out = []
    for p in t.walk(roots, lambda p, n: n.endswith(".rs") and "/📚examples/" in p):
        if p not in reachable:
            out.append(["dead-example-leaf-" + p, "taxonomy/dead-example-leaf", "high", 0])
    return out


def op_and_commands(t):
    out = []
    for p in t.walk(["✏s"], lambda p, n: n.endswith(".grammar.semio") and "/🔧op/" in p):
        c = t.files[p]
        if re.search(r"^start\s+mutation\b", c, re.M | A):
            continue
        out.append(["op-grammar-start-mutation-" + p, "mutation-migration/op-grammar-start", "high", 0])

    def walk(d):
        for name, is_dir in sorted(t.children(d).items()):
            if not is_dir or name.startswith(".") or name in SKIP_NAMES:
                continue
            child = d + "/" + name if d else name
            if name == "🎮commands":
                for cmd, c_dir in sorted(t.children(child).items()):
                    if not c_dir or cmd in SKIP_NAMES:
                        continue
                    ascii_ = strip_non_ascii(cmd)
                    emoji = cmd if not ascii_ else (cmd[:cmd.index(ascii_)] if cmd.index(ascii_) > 0 else "")
                    slug = cmd[len(emoji):] if emoji else cmd
                    if slug.split("-")[0] in VERBS:
                        continue
                    out.append(["command-folder-semantic-%s/%s" % (child, cmd), "taxonomy/command-folder-semantic", "medium", 0])
                continue
            walk(child)

    walk("✏s")
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    if r == "policyOwnerOwnComponentFiles":
        return own_component_files(t, c["ownerRel"])
    return {
        "policyHandcraftedSpecP3Breaches": p3, "policyEmptyExampleBreaches": empty_example, "policyPluginDependencyParityBreaches": dependency_parity,
        "policyContributionTargetBreaches": contribution_targets, "policySemanticVocabularyBreaches": vocabulary, "policyDeadExampleLeafBreaches": dead_example,
        "policyMutationArtifactEngineBreaches": op_and_commands,
    }[r](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
