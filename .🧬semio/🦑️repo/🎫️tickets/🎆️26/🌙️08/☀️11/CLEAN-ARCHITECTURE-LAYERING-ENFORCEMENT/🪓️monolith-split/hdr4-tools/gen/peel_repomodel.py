import sys, os, re, json, shutil
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
write = "--write" in sys.argv
MOD = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model"
OLD = MOD + "/" + PKG
NODE = MOD + "/🪪️entity"
NEW = NODE + "/" + PKG
CRATE = "semio-framework-repo-model-entity"
IDENT = "semio_framework_repo_model_entity"
consumers = {
 "event-stream": "⌨️cli/📡️event-stream",
 "tree-source-and-search": "⌨️cli/🧭️repo-cli/🗂️tree-source-and-search",
 "verbs": "⌨️cli/🧭️repo-cli/🚦️verbs",
 "tree": "🌳️tree",
}
RM = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
files = [
 "⌨️cli/📡️event-stream/🦀️.rs", "⌨️cli/🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs",
 "⌨️cli/🧭️repo-cli/🚦️verbs/⚖️statute-verbs/🦀️.rs", "⌨️cli/🧭️repo-cli/🚦️verbs/📊️code-metric-verbs/🦀️.rs",
 "⌨️cli/🧭️repo-cli/🚦️verbs/📜️history-verbs/🦀️.rs", "⌨️cli/🧭️repo-cli/🚦️verbs/🔎️query-verbs/🦀️.rs",
 "🌳️tree/📦️packages/🦀️rust/🧱️tree-model/🦀️.rs",
]
def rewrite(s):
    def brace(m):
        items = [x.strip() for x in m.group(1).split(",") if x.strip()]
        mine = [x for x in items if x in ("entity", "ansi")]
        rest = [x for x in items if x not in ("entity", "ansi")]
        if not mine: return m.group(0)
        out = ""
        if rest: out += "use semio_framework_repo_model::{%s};\n" % ", ".join(rest)
        out += "use %s::{%s};" % (IDENT, ", ".join(mine)) if len(mine) > 1 else "use %s::%s;" % (IDENT, mine[0])
        return out
    s = re.sub(r"use semio_framework_repo_model::\{([^}]*)\};", brace, s)
    s = re.sub(r"semio_framework_repo_model::(entity|ansi)::", IDENT + r"::\1::", s)
    return s
for f in files:
    p = RM + f
    s = rd(p); s2 = rewrite(s)
    print(("CHANGE " if s != s2 else "same   ") + f, sum(1 for a, b in zip(s.split("\n"), s2.split("\n")) if a != b))
if not write: sys.exit(0)
assert not os.path.exists(NODE)
os.makedirs(NEW)
for d in ("🎨️ansi", "🪪️entity"):
    shutil.move(OLD + "/" + d, NEW + "/" + d)
open(NEW + "/🦀️.rs", "w").write('//! 🪪️ Artifact identities, property projection, entity templates and the ANSI escapes of the human renderer of the `🦑️repo` product, carved out of `semio-framework-repo-model`.\n\n#[path = "🎨️ansi/🦀️.rs"]\npub mod ansi;\n\n#[path = "🪪️entity/🦀️.rs"]\npub mod entity;\n')
wsrel = os.path.relpath(R, NEW)
open(NEW + "/Cargo.toml", "w").write('[package]\nworkspace = "%s"\nname = "%s"\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.88"\ndescription = "semio repo entity templates: artifact identities, property projection and the ANSI escapes of the human renderer"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "🦀️.rs"\n\n[dependencies]\nserde_json = { workspace = true }\nsemio-framework-repo-identity = { path = "%s" }\n' % (wsrel, CRATE, os.path.relpath(MOD + "/../🪪️identity/" + PKG, NEW)))
t = rd(OLD + "/🦀️.rs")
t = t.replace('#[path = "🎨️ansi/🦀️.rs"]\npub mod ansi;\n\n#[path = "🪪️entity/🦀️.rs"]\npub mod entity;\n\n', "", 1)
assert "pub mod entity" not in t
wr(OLD + "/🦀️.rs", t)
add_member(R, NEW, OLD)
add_owner(CRATE, NEW)
for f in files:
    p = RM + f; wr(p, rewrite(rd(p)))
for name, rel in consumers.items():
    d = RM + rel
    man = d + "/Cargo.toml" if os.path.exists(d + "/Cargo.toml") else d + "/" + PKG + "/Cargo.toml"
    mt = rd(man)
    m = re.search(r'(?m)^semio-framework-repo-model = \{ path = "([^"]+)"[^\n]*\}\n', mt)
    assert m, man
    newp = os.path.relpath(NEW, os.path.dirname(man))
    mt = mt[:m.end()] + '%s = { path = "%s" }\n' % (CRATE, newp) + mt[m.end():]
    wr(man, mt); print("dep added", os.path.relpath(man, R))
