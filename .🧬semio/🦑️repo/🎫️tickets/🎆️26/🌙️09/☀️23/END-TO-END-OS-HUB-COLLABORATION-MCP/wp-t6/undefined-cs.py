"""🔍️ Lists expl3/LaTeX control sequences that print packages and fixtures use but no print package defines."""
import re, pathlib, sys, collections
root = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print")
latex = root / "🖋️latex"
defs = set()
defpat = re.compile(r"\\(?:cs_(?:new|set|gset)(?:_protected)?(?:_nopar)?:(?:Npn|Npx|Nn|Nx|cpn|cpx|cn|cx)|cs_generate_variant:Nn|cs_new_eq:NN|cs_set_eq:NN|cs_gset_eq:NN|NewDocumentCommand|DeclareDocumentCommand|ProvideDocumentCommand|newcommand|providecommand|def|gdef|edef|xdef|let|(?:tl|seq|clist|prop|int|fp|dim|skip|bool|str|box|coffin|muskip|intarray|fparray|toks|regex|ior|iow)_(?:new|const):(?:N|c|Nn|cn|Nx|cx)|newcounter|newlength|newif|newdimen|newcount|newtoks|newbox|colorlet|definecolor)\s*\{?\s*\\([A-Za-z_@:]+)")
for f in latex.glob("*.sty"):
    for m in defpat.finditer(f.read_text(errors="replace")):
        defs.add(m.group(1))
for f in list(latex.glob("*.sty")) + list(latex.glob("*.cls")):
    t = f.read_text(errors="replace")
    for m in re.finditer(r"\\cs_generate_variant:Nn\s*\\([A-Za-z_@]+):([A-Za-z]+)\s*\{([^}]*)\}", t):
        base, sig, vs = m.groups()
        for v in re.split(r"\s*,\s*", vs.strip()):
            if v: defs.add(f"{base}:{v}")
    for m in re.finditer(r"\\NewDocumentEnvironment\s*\{([^}]+)\}", t):
        defs.add("env:" + m.group(1))
targets = [pathlib.Path(p) for p in sys.argv[1:]]
uses = collections.defaultdict(set)
for f in targets:
    t = f.read_text(errors="replace")
    for m in re.finditer(r"\\((?:semio|SemioViz|g_semio|l_semio|c_semio|__semio)[A-Za-z_@:]*)", t):
        uses[m.group(1)].add(f.name)
missing = {k: v for k, v in uses.items() if k not in defs}
for k in sorted(missing):
    print(k, "\t", ",".join(sorted(missing[k])))
print(len(missing), "missing of", len(uses), file=sys.stderr)
