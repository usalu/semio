"""number: lift the eight inline `pub mod` blocks of the lib file into `<emoji>️<kebab>/🦀️.rs` children."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
from rslex import outline, multiline_literal_lines

NAME = "number"
T = TARGETS[NAME]
SRC = ROOT + "/" + T["file"]
SNAP = snap_dir(NAME) + "/" + T["file"]

MODS = {
    "traits": "💍️traits",
    "natural": "🧱️natural",
    "integer": "➕️integer",
    "rational": "➗️rational",
    "modular": "🕰️modular",
    "primes": "🔑️primes",
    "interval": "📏️interval",
    "algebra": "🧮️algebra",
}
DOC_FIX = {
    ("natural", "//! 🔢️ "): "//! 🧱️ ",
    ("primes", "//! 🧮️ "): "//! 🔑️ ",
}


def build():
    text = read(SNAP)
    ls = lines_of(text)
    assert not multiline_literal_lines(text), "multi-line literals present: dedent unsafe"
    items = outline(text)
    cov = Coverage(len(ls))
    out = {}
    parent = []
    pos = 1
    mods = [it for it in items if it["kind"] == "mod"]
    assert [m["name"] for m in mods] == list(MODS), [m["name"] for m in mods]
    for m in mods:
        name = m["name"]
        s0 = next(i + 1 for i in range(m["start"], m["end"] + 1) if ls[i] == f"pub mod {name} {{")
        e0 = m["end"] + 1
        assert ls[s0 - 1].startswith("// #region") or name == "traits" or True
        rs = s0 - 1
        assert ls[rs - 1].startswith("// #region 🔖️"), (name, ls[rs - 1])
        assert ls[e0].startswith("// #endregion 🔖️"), (name, ls[e0])
        assert ls[s0 - 1] == f"pub mod {name} {{"
        assert ls[e0 - 1] == "}"
        cov.claim("parent", pos, rs - 1)
        parent.extend(ls[pos - 1:rs - 1])
        folder = MODS[name]
        parent.append(ls[rs - 1])
        parent.append(f'#[path = "{folder}/🦀️.rs"]')
        parent.append(f"pub mod {name};")
        parent.append(ls[e0])
        cov.claim("parent", rs, rs)
        cov.claim("parent-mount", s0, s0)
        cov.claim("parent-mount", e0, e0)
        cov.claim("parent", e0 + 1, e0 + 1)
        body = ls[s0:e0 - 1]
        cov.claim(name, s0 + 1, e0 - 1)
        body = dedent(body, 4)
        for i, l in enumerate(body):
            m2 = re.match(r'^(\s*)include!\("(🧪️tests/[^"]+)"\);$', l)
            if m2:
                body[i] = f'{m2.group(1)}include!("../{m2.group(2)}");'
        for (nm, pre), repl in DOC_FIX.items():
            if nm == name:
                assert body[0].startswith(pre), body[0][:30]
                body[0] = repl + body[0][len(pre):]
        out[f"{folder}/🦀️.rs"] = "\n".join(body) + "\n"
        pos = e0 + 2
    cov.claim("parent", pos, len(ls))
    parent.extend(ls[pos - 1:])
    cov.check()
    out["🦀️.rs"] = "\n".join(parent) + "\n"
    return ls, out


def main():
    ls, out = build()
    sd = stage_dir(NAME)
    if os.path.exists(sd):
        import shutil
        shutil.rmtree(sd)
    base = os.path.dirname(T["file"])
    for rel, txt in out.items():
        write(f"{sd}/{base}/{rel}", txt)
    new = []
    for txt in out.values():
        new.extend(lines_of(txt))
    gone, added = diff_multiset(ls, new)
    print("files:", {k: len(lines_of(v)) for k, v in out.items()})
    print("removed (stripped multiset):", dict(gone))
    print("added   (stripped multiset):", dict(added))


if __name__ == "__main__":
    main()
