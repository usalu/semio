"""🧰 rw2t step A: the Scenes standalone helpers other laws name move to the crates that own what they read (cfg(any(test, feature = "testkit")) items)."""
import re, sys
from ed import *
E_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
STAND = E_ + "/🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs"
TBL = E_ + "/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧮️table-and-paint2d/🦀️.rs"
INK = E_ + "/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖋️ink-canvas-render/🦀️.rs"
TXT = E_ + "/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs"
DRY = "--dry" in sys.argv

def take(text, name):
    ms = list(re.finditer(r"#\[cfg\(test\)\]\npub fn " + name + r"[<(]", text))
    assert len(ms) == 1, name
    s = ms[0].start()
    nm = re.compile(r"\n(?=#\[cfg\(test\)\]|#\[test\])").search(text, ms[0].end())
    nxt = nm.start()
    block = text[s:nxt + 1]
    return block, text[:s] + text[nxt + 1:]

def gated(block):
    return block.replace("#[cfg(test)]\npub fn", '#[cfg(any(test, feature = "testkit"))]\npub fn', 1).rstrip("\n") + "\n"

stand = read(STAND)
moved = {}
for n in ["paint2d_navigator_fit_viewport", "paint2d_navigator_overlay_rect", "ink_items_at_point", "cursor_from_click", "line_col_at"]:
    b, stand = take(stand, n)
    moved[n] = gated(b)
stand = re.sub(r"\n{4,}", "\n\n\n", stand)

tbl = read(TBL)
a1 = "document has no pixel content. */\n\n"
a2 = "hasn't been reported yet. */\n\n"
assert tbl.count(a1) == 1 and tbl.count(a2) == 1
tbl = tbl.replace(a1, a1 + moved["paint2d_navigator_fit_viewport"] + "\n").replace(a2, a2 + moved["paint2d_navigator_overlay_rect"] + "\n")

ink = read(INK)
a3 = "pub fn ink_selection_bounds("
assert ink.count(a3) == 1
ink = ink.replace(a3, moved["ink_items_at_point"] + "\n" + a3)

txt = read(TXT)
a4 = "//#endregion State\n"
assert txt.count(a4) == 1
txt = txt.replace(a4, moved["cursor_from_click"] + "\n" + moved["line_col_at"] + "\n" + a4)

if DRY:
    print("dry ok", len(stand), len(tbl), len(ink), len(txt))
    for k, v in moved.items(): print(k, len(v.split("\n")))
    sys.exit(0)
for p, t in [(STAND, stand), (TBL, tbl), (INK, ink), (TXT, txt)]:
    backup(p)
    assert read(p) != t
    with open(p, "w", encoding="utf8") as h:
        h.write(t)
    note(p)
    print("edited", p[-70:])
