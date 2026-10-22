def kind_file(kind):
    k = TAXONOMY["fileKinds"][kind]
    return k["emoji"] + k["extensionChains"][0]


MARKERS = {kind_file("antlr"): "DOCUMENT: 'schema' [ ]+", kind_file("ebnf"): "header = 'schema', space,", kind_file("grammar-semio"): "payload = *OCTET", kind_file("abnf"): "payload = *OCTET", kind_file("protocol-semio"): "payload = *OCTET", kind_file("kaitai"): "size-eos: true", kind_file("spicy"): "payload: bytes &eod;"}


def honesty(t):
    out = []
    for p in t.walk([STDIO], lambda p, n: n in MARKERS):
        if MARKERS[p.split("/")[-1]] in t.files[p]:
            out.append(["grammar-honesty-" + p, "stdio-artifacts/grammar-honesty", "low", 0])
    return out


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: honesty(Tree(expand(c))) for c in cases if c.get("oracle") is not False})
