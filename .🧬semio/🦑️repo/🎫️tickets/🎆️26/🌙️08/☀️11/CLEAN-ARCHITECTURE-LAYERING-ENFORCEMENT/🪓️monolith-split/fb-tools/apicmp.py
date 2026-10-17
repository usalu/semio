"""🔎 FB: the public item names of the engine crate root before and after the carve (pub items, pub use leaves, pub mods) must be identical sets."""
import re, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from items import split_items
sys.path.insert(0, HERE + "/pos")
from posstub import expand_use

LIVE = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine"


def exports(path):
    out = {}
    t = open(path, encoding="utf-8").read()
    for kind, name, text in split_items(t):
        if kind != "item":
            continue
        body = [l for l in text.split("\n") if l.strip() and not l.strip().startswith("//") and not l.strip().startswith("#[")]
        if not body:
            continue
        head = body[0].strip()
        m = re.match(r"pub\s+(use)\s+(.*)", head)
        if head.startswith("pub use "):
            tree = " ".join(l.strip() for l in body).strip()[len("pub use "):].rstrip(";")
            for k, segs in expand_use(tree):
                alias = segs[-1]
                am = re.search(r"\bas\s+(\w+)\s*(?:[,}]|$)", tree)
                out[alias] = "use"
            continue
        m = re.match(r"pub\s+(?:async\s+|unsafe\s+|const\s+)*(fn|struct|enum|trait|type|const|static|mod|union)\s+(\w+)", head)
        if m:
            out[m.group(2)] = m.group(1)
    return out


old = exports(HERE + "/orig/engine/🦀️.rs")
new = exports(LIVE + "/🦀️.rs")
print("old", len(old), "new", len(new))
print("only old:", sorted(set(old) - set(new)))
print("only new:", sorted(set(new) - set(old)))
