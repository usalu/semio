"""📦️ Reports every `semio-viz-*.sty` that CALLS a kernel macro it neither defines itself nor gets
through its transitive RequirePackage closure. A family package that forgets a requirement works
inside a document that happens to load the missing package first and dies in a minimal one — which is
exactly what a probe fixture and a gallery section are."""
import io, os, re, sys, collections

LATEX = os.path.join(sys.argv[1] if len(sys.argv) > 1 else "C:/git/semio",
                     "\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint/\U0001f58b\ufe0flatex")
DEF = re.compile(r"\\(?:cs_new(?:_protected)?(?:_nopar)?:(?:Npn|Npx|cpn)|cs_set(?:_protected)?:Npn|cs_new_eq:NN|cs_gset(?:_protected)?:Npn)\s*\\(semio_viz_[a-zA-Z_]+:[a-zA-Z]*)")
VARIANT = re.compile(r"\\cs_generate_variant:Nn\s*\\(semio_viz_[a-zA-Z_]+):([a-zA-Z]*)\s*\{([^}]*)\}")
USE = re.compile(r"\\(semio_viz_[a-zA-Z_]+:[a-zA-Z]+)")
REQ = re.compile(r"\\RequirePackage\{(semio-[a-z0-9-]+)\}")

defines, uses, requires = {}, {}, {}
for name in sorted(os.listdir(LATEX)):
    if not name.endswith(".sty"):
        continue
    text = io.open(os.path.join(LATEX, name), encoding="utf-8", errors="replace").read()
    own = set(DEF.findall(text))
    for base, sig, variants in VARIANT.findall(text):
        for variant in variants.split(","):
            variant = variant.strip()
            if variant:
                own.add(base + ":" + variant)
    defines[name] = own
    uses[name] = set(USE.findall(text))
    requires[name] = set(package + ".sty" for package in REQ.findall(text))

owner = {}
for name, own in defines.items():
    for macro in own:
        owner.setdefault(macro, name)


def closure(name, seen=None):
    seen = seen if seen is not None else set()
    for dep in requires.get(name, ()):
        if dep not in seen and dep in defines:
            seen.add(dep)
            closure(dep, seen)
    return seen


report = collections.defaultdict(lambda: collections.defaultdict(list))
for name in sorted(defines):
    reach = closure(name) | {name}
    for macro in sorted(uses[name]):
        home = owner.get(macro)
        if home is None or home in reach:
            continue
        report[name][home].append(macro)

for name in sorted(report):
    for home, macros in sorted(report[name].items(), key=lambda item: -len(item[1])):
        print(name + "\tneeds " + home[:-4] + "\t" + str(len(macros)) + " macro(s): " + ", ".join(macros[:4]))
print("packages with a gap:", len(report))
