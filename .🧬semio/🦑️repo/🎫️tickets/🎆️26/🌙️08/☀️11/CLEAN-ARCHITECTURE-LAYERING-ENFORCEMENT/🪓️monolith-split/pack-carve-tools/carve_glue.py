"""🧩 Rewrites the pack facade glue and the component re-exports onto the carved crates."""
from lib import *
from owners import *
from crates import *
import usetree, textwrap

GLUE = P + "📦️packages/🦀️rust/🦀️.rs"

def fmt_use(crate_key, names, indent="    "):
    names = sorted(names, key=lambda s: s)
    lines = []
    cur = indent + "pub use " + ident(crate_key) + "::{"
    body = ", ".join(names)
    return indent + "pub use " + ident(crate_key) + "::{" + body + "};"

def format_module():
    base = read_base(P + "📐️format/🦀️.rs")
    pub = pub_names(base)
    owners = format_owners()
    by = {k: sorted(n for n in pub if owners.get(n) == k and n != "retained_inline_symbols") for k in ("format", "retained", "catalog")}
    assert "retained_inline_symbols" in pub
    out = ["pub mod format {"]
    for k in ("format", "retained", "catalog"):
        out.append(fmt_use(k, by[k]))
    out.append("    pub use " + ident("catalog") + "::retained_inline_symbols;")
    out.append("}")
    return "\n".join(out) + "\n"

def apply_format(glue):
    old = '#[path = "../../📐️format/🦀️.rs"]\npub mod format;\n'
    assert glue.count(old) == 1
    return glue.replace(old, format_module())

def run():
    glue = read_base(GLUE)
    glue = apply_format(glue)
    write(GLUE, glue)

if __name__ == "__main__":
    run(); print("glue ok")
