# 🔍 Reports every catalogue option key that its family's l3keys module does not declare, which is
# what makes a gallery section stop with "The key 'semio/viz/family/<f>/<k>' is unknown".
# A family with an `unknown .code:n` handler swallows anything, so it is reported as tolerant.
import io, json, os, re, sys, collections

PRINT = u"C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint"
LATEX = os.path.join(PRINT, u"\U0001f58b\ufe0flatex")
CATALOG = os.path.join(PRINT, u"\U0001f5bc\ufe0fassets", u"\U0001f523\ufe0fviz-catalog.json")

MODULE = re.compile(r"\\keys_define:nn\s*\{\s*semio\s*/\s*viz\s*/\s*([A-Za-z0-9 /\-]+?)\s*\}\s*\{")
KEY = re.compile(r"^\s*([A-Za-z][A-Za-z0-9-]*)\s*\.", re.M)
ADD = re.compile(r"\\exp_args:NV\s+\\keys_define:nn\s+(\\[A-Za-z_:]+)\s*\{")


def blocks(src):
    """Yield (module, body) for every \\keys_define:nn block, matching braces."""
    for match in MODULE.finditer(src):
        start = match.end()
        depth = 1
        index = start
        while index < len(src) and depth > 0:
            if src[index] == "{":
                depth += 1
            elif src[index] == "}":
                depth -= 1
            index += 1
        yield re.sub(r"\s+", "", match.group(1)), src[start:index - 1]


declared = collections.defaultdict(set)
tolerant = set()
shared = {}
for name in sorted(os.listdir(LATEX)):
    if not name.endswith(".sty"):
        continue
    src = io.open(os.path.join(LATEX, name), encoding="utf-8").read()
    for module, body in blocks(src):
        keys = set(KEY.findall(body))
        declared[module] |= keys
        if "unknown" in keys:
            tolerant.add(module)
    # a shared vocabulary added into a family module through a tl-valued module name
    for match in ADD.finditer(src):
        start = match.end()
        depth = 1
        index = start
        while index < len(src) and depth > 0:
            if src[index] == "{":
                depth += 1
            elif src[index] == "}":
                depth -= 1
            index += 1
        shared[name] = shared.get(name, set()) | set(KEY.findall(src[start:index - 1]))

inherited = set().union(*shared.values()) if shared else set()
catalog = json.load(io.open(CATALOG, encoding="utf-8"))
missing = collections.defaultdict(set)
for kind in catalog["kinds"]:
    module = "family/" + kind["family"]
    if module in tolerant:
        continue
    known = declared[module] | declared["family/common"] | inherited | {"variant", "data"}
    for key in kind["options"]:
        if key not in known:
            missing[kind["family"]].add(key)

want = sys.argv[1] if len(sys.argv) > 1 else ""
for family in sorted(missing):
    if want and want != family:
        continue
    module = "family/" + family
    state = "declares nothing" if not declared[module] else ""
    print(family, "->", " ".join(sorted(missing[family])), state)
print("families with undeclared keys:", len(missing))
