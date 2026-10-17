"""📦 rw2t manifest editor: adds dependency lines to a table in sorted position (first-party renderer crates first), or sets features on an existing line."""
import re, os, sys
from ed import *
FP = "semio-framework-os-renderer-wgpu-"
TG = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"

def key_of(line):
    return line.split("=", 1)[0].strip()

def sortkey(line):
    k = key_of(line)
    return (0 if k.startswith(FP) else 1, k)

def add_lines(path, table, lines):
    text = read(path)
    m = re.search(r"(?m)^\[" + re.escape(table) + r"\]\n((?:[^\[\n][^\n]*\n)*)", text)
    if not m:
        sep = "" if text.endswith("\n\n") else "\n"
        new = text + sep + f"[{table}]\n" + "".join(sorted(set(lines), key=sortkey)) if False else None
        body = "".join(l.rstrip("\n") + "\n" for l in sorted(lines, key=sortkey))
        out = text.rstrip("\n") + f"\n\n[{table}]\n" + body
        return replace_text(path, text, out)
    body = m.group(1)
    have = [l for l in body.split("\n") if l.strip()]
    keys = {key_of(l) for l in have}
    add = [l.rstrip("\n") for l in lines if key_of(l) not in keys]
    if not add:
        print("no change", path[-60:], table)
        return
    allv = sorted(have + add, key=sortkey)
    out = text[:m.start(1)] + "\n".join(allv) + "\n" + text[m.end(1):]
    replace_text(path, text, out)

def set_features(path, key, features):
    text = read(path)
    pat = re.compile(r"(?m)^(" + re.escape(key) + r" = \{ path = \"[^\"]+\")((?:, [^\n]*?)?) \}$")
    ms = list(pat.finditer(text))
    assert len(ms) == 1, (path[-60:], key, len(ms))
    m = ms[0]
    rest = m.group(2)
    if "features" in rest:
        print("has features", key); return
    new = m.group(1) + ', features = [' + ", ".join(f'"{f}"' for f in features) + "] }"
    replace_text(path, text, text[:m.start()] + new + text[m.end():])

def replace_text(path, old, new):
    backup(path)
    assert read(path) == old, "changed: " + path
    with open(path, "w", encoding="utf8") as h:
        h.write(new)
    note(path)
    print("edited", path[-70:])

def mpath(folder):
    return TG + "/" + folder + "/📦️packages/🦀️rust/Cargo.toml"
