ROOTS = ["🧰framework/🔨modules/🔺mesh", "🧰framework/🛍products/💻os", "🧰framework/🛍products/💻os/🖥host", "✏s/🔌plugins"]
MARKERS = ["SRAS", "IFCCARTOONMESH", 'b"minimal"', "stub codec", "minimal stub codec"]


def codec_fidelity(t):
    out = []
    for p in t.walk(ROOTS, lambda p, n: n.endswith(".rs")):
        for m in MARKERS:
            if m in t.files[p]:
                out.append(["stdio-codec-ban-%s-%s" % (p, m), "stdio-artifacts/codec-fidelity", "high", 0])
    return sorted(out, key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


def run(c):
    t = Tree(expand(c))
    return codec_fidelity(t) if c["rule"] == "policyCodecFidelityBreaches" else []


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
