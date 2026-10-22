REGISTRY = "📋build-owners.json"
AUTHORED = {"registry-invalid", "missing-native-root", "row-in-excluded-tree", "provider-owner-root", "provider-source", "provider-consumer-root"}


def finding_row(kind, key, scope):
    return ["build-owner-registry-%s-%s" % (kind, key), "repo/build-owner-registry", "high", 0, kind not in AUTHORED, scope]


def cargo_name(text):
    m = re.search(r"^\[package\]\s*$([\s\S]*?)(?=^\[|\Z)", text, re.M)
    n = re.search(r'^name\s*=\s*"([^"]+)"', m.group(1), re.M) if m else None
    return n.group(1) if n else None


def findings(t):
    if REGISTRY not in t.files:
        return [finding_row("registry-invalid", REGISTRY, REGISTRY)]
    try:
        reg = json.loads(t.files[REGISTRY])
    except ValueError:
        return [finding_row("registry-invalid", REGISTRY, REGISTRY)]
    out = []
    covered = {r for o in reg["owners"] for r in [o["root"]] + o.get("nativeRoots", [])}
    for o in reg["owners"]:
        if not t.exists(o["root"]):
            out.append(finding_row("missing-root", o["name"], o["root"]))
            continue
        if not t.exists(o["manifest"]):
            out.append(finding_row("missing-manifest", o["name"], o["manifest"]))
            continue
        actual = cargo_name(t.files[o["manifest"]]) if o["manifest"].endswith("Cargo.toml") else None
        if actual != o["name"]:
            out.append(finding_row("name-mismatch", o["name"], o["manifest"]))
    for p in reg["catalogProviders"]:
        if not t.exists(p["ownerRoot"]):
            out.append(finding_row("provider-owner-root", p["ownerRoot"], p["ownerRoot"]))
        if not t.exists(p["sourcePath"]):
            out.append(finding_row("provider-source", p["sourcePath"], p["sourcePath"]))
        for c in p["consumerRoots"]:
            if not t.exists(c):
                out.append(finding_row("provider-consumer-root", c, c))
    for p in sorted(t.files):
        if p.endswith("/Cargo.toml") or p == "Cargo.toml":
            d = posixpath.dirname(p)
            if d not in covered and cargo_name(t.files[p]) is not None:
                out.append(finding_row("cargo-without-row", p, p))
    return out


def run(c):
    rows = findings(Tree(expand(c)))
    return sorted([r[:5] + [r[5]] for r in rows], key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
