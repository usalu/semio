ROOT_DIRS = ["🧰framework/🛍products/🦑repo/🔨modules/📚library/" + d for d in ("🎛commands", "⚖laws/🎯tool-job-coverage", "⚖laws/🔬interactivity-audit", "⚖laws/🧩plugin-consistency", "⚖laws/🧪self-test-lanes", "🗄stdio-ledger", "🧹normalization/🔣taxonomy/🏷filenames", "🔍discovery/📖source-access/🦀rust-evidence", "🗂workspaces/📍checkout-root")]
PKG_DIRS = ["🧰framework/🛍products/💻os/📦packages/🦀rust/🎛commands"]

def owned_modules(t, dirs):
    found = set()
    for d in dirs:
        found |= set(t.walk([d], lambda p, n: n == "🟦.ts", skip=lambda rel, name: name == "🧪tests"))
    return jssort(found)


def is_root_script(p):
    return p == "📜script.ts" or any(p.startswith(d + "/") for d in ROOT_DIRS)


def run(c):
    a = c.get("args", [])
    t = Tree(expand(c)) if "files" in c else None
    return {
        "policyAppIdFromCrateDir": lambda: app_id(a[0]), "policyNormalizeRelPath": lambda: normalize_rel(a[0]),
        "policyNewSurfacePriority": lambda: a[1] if a[0].get("pluginId") else "low",
        "policyParseRegionEvents": lambda: region_events(a[0]), "policyPairRegionSpans": lambda: pair_spans(a[0]),
        "policyParseModSpans": lambda: mod_spans(a[0]), "policyModAtLine": lambda: mod_at(a[0], a[1]),
        "policyLabelName": lambda: label_name(a[0]), "policyExtractFnBody": lambda: extract_body(a[0], a[1]),
        "policyPascalAppStructName": lambda: pascal(a[0]), "policyIsRootScriptSource": lambda: is_root_script(a[0]),
        "policyRootScriptModulePaths": lambda: ["📜script.ts"] + owned_modules(t, ROOT_DIRS),
        "policyPackageScriptModulePaths": lambda: owned_modules(t, PKG_DIRS),
        "policyAllRustFiles": lambda: t.walk([""], lambda p, n: n.endswith(".rs")),
    }[c["rule"]]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases if c.get("oracle") is not False})
