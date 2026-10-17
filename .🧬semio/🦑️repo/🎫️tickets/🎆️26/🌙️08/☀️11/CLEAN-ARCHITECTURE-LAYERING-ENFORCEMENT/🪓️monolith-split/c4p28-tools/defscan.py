"""🔎 C4P28 definition scan (the blind spot of the typeck oracle: E0616/E0624 can be under-reported): for every module-split child, each private top-level item, brace-struct field and inherent member whose
name also occurs as an identifier in another file of the tree (siblings, the root, the descendants and, for lib roots, nothing else) must carry a visibility. Identifiers are taken from the code with comments and
string literals masked. usage: defscan.py stage|live"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
REPO = "/Users/ueli/Documents/semio"
mode = sys.argv[1]
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
RX_TOP = re.compile(r"^(?:(?:async|unsafe|const|extern(?: \"[^\"]*\")?)\s+)*(fn|const|static|struct|enum|trait|type|union|mod)\s+(\w+)")
RX_FIELD = re.compile(r"^ {4}(\w+)\s*:\s")
RX_MEM = re.compile(r"^ {4}(?:(?:async|unsafe|const|default)\s+)*(fn|const|type)\s+(\w+)")


def mask(text):
    text = re.sub(r"/\*.*?\*/", lambda m: re.sub(r"[^\n]", " ", m.group(0)), text, flags=re.S)
    text = re.sub(r"//[^\n]*", "", text)
    text = re.sub(r'r(#*)".*?"\1', '""', text, flags=re.S)
    return re.sub(r'"(?:[^"\\\n]|\\.)*"', '""', text)


def idents(text):
    return set(IDENT.findall(mask(text)))


bad = 0
for path in sorted(glob.glob(HERE + "/specs/*.json")):
    n = int(os.path.basename(path).split(".")[0])
    spec = json.load(open(path, encoding="utf-8"))
    if "segments" not in spec["chunks"][0]:
        continue
    base = REPO if mode == "live" else spec["stage"]
    d = os.path.dirname(spec["file"])
    texts = {c["folder"]: open(f"{base}/{d}/{c['folder']}/🦀️.rs", encoding="utf-8").read() for c in spec["chunks"]}
    root = open(f"{base}/{spec['file']}", encoding="utf-8").read()
    ext = {rel: idents(open(f"{REPO}/{rel}", encoding="utf-8").read()) for rel in spec.get("descendants", []) if os.path.isfile(f"{REPO}/{rel}")}
    ids = {k: idents(t) for k, t in texts.items()}
    ids["__root__"] = idents(root)
    hits = []
    count = 0
    for k, t in texts.items():
        others = set()
        for g, s in ids.items():
            if g != k:
                others |= s
        for s in ext.values():
            others |= s
        mode_ = None
        for ln, line in enumerate(mask(t).split("\n"), 1):
            if re.match(r"^\S", line) and not line.startswith("#["):
                mode_ = None
                m = RX_TOP.match(line)
                if m:
                    count += 1
                    if m.group(2) in others and not line.startswith("pub"):
                        hits.append((k, ln, "item", m.group(2)))
                if re.match(r"^struct\s+\w+[^;]*\{\s*$", line):
                    mode_ = "struct"
                elif re.match(r"^impl\b", line) and " for " not in line.split("{")[0] and line.rstrip().endswith("{"):
                    mode_ = "impl"
                elif re.match(r"^impl\b", line) and " for " not in line and not line.rstrip().endswith("{"):
                    mode_ = "impl-open"
                continue
            if mode_ == "impl-open":
                if line.rstrip().endswith("{"):
                    mode_ = "impl"
                continue
            if mode_ == "struct":
                m = RX_FIELD.match(line)
                if m:
                    count += 1
                    if m.group(1) in others:
                        hits.append((k, ln, "field", m.group(1)))
            elif mode_ == "impl":
                m = RX_MEM.match(line)
                if m:
                    count += 1
                    if m.group(2) in others:
                        hits.append((k, ln, "member", m.group(2)))
    bad += len(hits)
    print(f"[{n}] private declarations scanned: {count}, cross-file leftovers without a visibility: {len(hits)}")
    for h in hits[:30]:
        print("    ", h)
print("DEFSCAN", "OK" if bad == 0 else f"{bad} leftovers")
sys.exit(1 if bad else 0)
