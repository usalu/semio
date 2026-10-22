SPLIT = set(json.loads(@@SPLIT@@))
PATH_MOD = re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')
HERE = os.path.dirname(os.path.abspath(sys.argv[1]))
ROOT = HERE[:HERE.index(os.sep + "🧰framework" + os.sep)]


def raw(rel):
    try:
        with open(os.path.join(ROOT, rel), encoding="utf8") as fh:
            return fh.read()
    except OSError:
        return ""


def children(rel, text):
    return [os.path.normpath(os.path.join(os.path.dirname(rel), m.group(1))).replace(os.sep, "/") for m in PATH_MOD.finditer(text)]


def tree(rel, seen=None):
    seen = set() if seen is None else seen
    if rel in seen:
        return ""
    seen.add(rel)
    text = raw(rel)
    return "\n".join([text] + [tree(child, seen) for child in children(rel, text)])


def tests_of(rel, text):
    return [c for c in children(rel, text) if "/🧪tests/" in c]


def rust_source(rel):
    text = raw(rel)
    if not text:
        return text
    if rel in SPLIT:
        return tree(rel)
    return text + "".join("\n" + (tree(t) if t in SPLIT else raw(t)) for t in tests_of(rel, text))


def family_dir(rel, tests):
    out = []
    base = os.path.join(ROOT, rel)

    def visit(d):
        try:
            entries = sorted(os.scandir(d), key=lambda e: e.name.encode("utf-16-be"))
        except OSError:
            return
        for e in entries:
            if e.is_dir():
                if (tests or (e.name != "🧪tests" and e.name != "🧪testing")) and e.name != "node_modules":
                    visit(e.path)
            elif e.is_file() and e.name.endswith(".rs"):
                with open(e.path, encoding="utf8") as fh:
                    text = fh.read()
                out.append(text if text.endswith("\n") else text + "\n")

    visit(base)
    return "".join(out)


def js_slice(text, a, b):
    i, j = text.find(a), text.find(b)
    n = len(text)
    i = max(n + i, 0) if i < 0 else min(i, n)
    j = max(n + j, 0) if j < 0 else min(j, n)
    return text[i:j] if i < j else ""


def node(p):
    if "parts" in p:
        return p.get("join", "\n").join(node(q) for q in p["parts"])
    paths = p["paths"]
    if p["reader"] == "rust-family":
        text = "\n".join(rust_source(x) for x in paths)
    elif p["reader"] == "rust-source":
        text = rust_source(paths[0])
    elif p["reader"] == "module-tree":
        text = tree(paths[0])
    elif p["reader"] == "wit-package":
        d = os.path.join(ROOT, paths[0])
        text = "\n".join(open(os.path.join(d, n), encoding="utf8").read() for n in sorted((n for n in os.listdir(d) if n.endswith(".wit") and os.path.isfile(os.path.join(d, n))), key=lambda s: s.encode("utf-16-be"))) if os.path.isdir(d) else ""
    elif p["reader"] == "source-family":
        text = family_dir(paths[0], p.get("tests") is True)
    else:
        text = raw(paths[0])
    if "names" in p:
        return "\n".join(n for n in p["names"] if p["prefix"] + n in text)
    if "slice" in p:
        return js_slice(text, p["slice"][0], p["slice"][1])
    return text


doc = json.load(open(sys.argv[1], encoding="utf8"))
cache = {}
result = {}
for case in doc["cases"]:
    if case.get("oracle") is False:
        continue
    m = case["mutation"]
    if m["op"] == "none":
        result[case["id"]] = {"present": True}
        continue
    name = m["target"]
    if name not in cache:
        cache[name] = node(doc["sources"][name])
    result[case["id"]] = {"present": m["text"] in cache[name]}
output(result)
