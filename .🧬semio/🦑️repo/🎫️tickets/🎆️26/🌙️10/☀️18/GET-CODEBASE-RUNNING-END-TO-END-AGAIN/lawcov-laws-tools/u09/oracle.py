DOMAIN_ALLOW = {"flow", "trinity_jack", "trinity_ram", "mathematical_graph_drawing", "mathematical_geometry"}
JSON_ALLOW_PREFIXES = ("coda/",)
STRUCTURAL = ("@doc", "@edit", "@change", "@checkpoint", "@alternative", "@active")


def strip_non_ascii(s):
    return re.sub(r"[^\x00-\x7f]", "", s)


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def plugin_owners(t):
    owners = []
    for p, c in t.files.items():
        if p.endswith("/📦packages/🦀rust/Cargo.toml") and re.search(r'^role\s*=\s*"plugin"', c, re.M):
            owners.append(p[: -len("/📦packages/🦀rust/Cargo.toml")])
    return sorted(set(owners), key=lambda o: -len(o))


def scope_key(t, rel):
    for o in plugin_owners(t):
        if rel == o or rel.startswith(o + "/"):
            return strip_non_ascii(o.split("/")[-1])
    return ""


def priority(crate, p):
    return p if crate["pluginId"] else "low"


def cargo_artifacts(t, crates):
    return [row("stray-cargo-artifact-%s-Cargo.lock" % c["dir"], "app-plugin/stray-cargo-artifact", priority(c, "high")) for c in crates if (c["dir"] + "/Cargo.lock") in t.files]


def app_coupling(t, crates):
    dirs = {c["dir"] for c in crates}
    out = []
    for c in crates:
        manifest = c["dir"] + "/Cargo.toml"
        if manifest not in t.files:
            continue
        for m in re.finditer(r'^([\w.-]+)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$', t.files[manifest], re.M | A):
            name, dep = m.group(1), m.group(2)
            resolved = posixpath.normpath(posixpath.join(c["dir"], dep))
            other = scope_key(t, resolved)
            if not other or other == c["pluginId"]:
                continue
            if resolved in dirs:
                out.append(row("app-coupling-plugin-%s-%s" % (c["dir"], name), "app-plugin/app-coupling", priority(c, "medium")))
                continue
            if name in DOMAIN_ALLOW:
                continue
            out.append(row("app-coupling-domain-%s-%s" % (c["dir"], name), "app-plugin/app-coupling", "low"))
    return out


def json_fixtures(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".json") and p.split("/")[-2:-1] == ["example"]):
        if p.startswith(JSON_ALLOW_PREFIXES):
            continue
        out.append(row("no-json-fixture-" + p, "dsl-migration/no-json-fixture", "high"))
    return out


def ops_grammar(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".ops")):
        raw = split_lines(t.files[p])
        lines = raw[:-1] if raw and raw[-1] == "" else raw
        for i, line in enumerate(lines):
            no = i + 1
            if not line.strip():
                out.append(row("ops-grammar-blank-%s-%d" % (p, no), "dsl-migration/ops-grammar", "medium", no))
                continue
            if not (line.startswith(STRUCTURAL) or (line.startswith("  ") and line.strip())):
                out.append(row("ops-grammar-line-%s-%d" % (p, no), "dsl-migration/ops-grammar", "medium", no))
    return out


def run(c):
    t = Tree(expand(c))
    crates = c.get("crates", [])
    return {"policyCargoArtifactBreaches": lambda: cargo_artifacts(t, crates), "policyAppCouplingBreaches": lambda: app_coupling(t, crates), "policyJsonFixtureBreaches": lambda: json_fixtures(t), "policyOpsGrammarBreaches": lambda: ops_grammar(t)}[c["rule"]]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
