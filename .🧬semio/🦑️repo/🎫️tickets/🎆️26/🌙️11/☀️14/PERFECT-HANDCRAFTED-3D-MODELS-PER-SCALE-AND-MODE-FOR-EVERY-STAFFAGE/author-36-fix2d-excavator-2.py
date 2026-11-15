"""🩹 Second excavator 2D repair (author 36): hood-rail front extent is the rail's own 50 mm (it was swept over 0.95 m of the roof), the 500 front no longer draws the whole house-counterweight profile at cab height (the lower house rect and the cab block carry the front), and at 100/200/500 the cab block reaches down to the lowered house bottom. Text-level, other bytes untouched; each step is skipped when already done."""
import glob, re
P = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/excavator/*.json")[0]
t = open(P, encoding="utf8").read()


def block_span(text, id_pos):
    start = text.rfind("\n      {", 0, id_pos) + 1
    depth = 0
    i = text.index("{", start)
    j = i
    while True:
        c = text[j]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                break
        j += 1
    end = j + 1
    if text[end] == ",":
        end += 1
    return start, end + 1


def front_range(text, scale):
    k = text.index(f'"denominator": {scale},', text.index('"representations"'))
    f = text.index('"front": [', k)
    nxt = text.index('"plan": [', f)
    return f, nxt


n = 0
f, e = front_range(t, 50)
seg = t[f:e]
if '"z0": -1.15,\n        "z1": -0.2' in seg and '"id": "hood-rail"' in seg:
    a = seg.index('"id": "hood-rail"')
    s0, s1 = block_span(seg, a)
    blk = seg[s0:s1].replace('"z0": -1.15,\n        "z1": -0.2', '"z0": -1.175,\n        "z1": -1.125')
    seg = seg[:s0] + blk + seg[s1:]
    t = t[:f] + seg + t[e:]
    n += 1
f, e = front_range(t, 500)
seg = t[f:e]
if '"id": "house-counterweight"' in seg:
    a = seg.index('"id": "house-counterweight"')
    s0, s1 = block_span(seg, a)
    seg = seg[:s0] + seg[s1:]
    t = t[:f] + seg + t[e:]
    n += 1
for sc in (100, 200, 500):
    f, e = front_range(t, sc)
    seg = t[f:e]
    a = seg.index('"id": "cab-block"')
    s0, s1 = block_span(seg, a)
    blk = seg[s0:s1]
    if '"h": 1.97' in blk:
        blk = blk.replace('"y": 0,\n       "w": 1.38,\n       "h": 1.97', '"y": -0.16,\n       "w": 1.38,\n       "h": 2.13')
        seg = seg[:s0] + blk + seg[s1:]
        t = t[:f] + seg + t[e:]
        n += 1
f, e = front_range(t, 500)
seg = t[f:e]
a = seg.index('"id": "house"')
s0, s1 = block_span(seg, a)
blk = seg[s0:s1]
if '"h": 1.25' in blk:
    blk = blk.replace('"y": 0,\n       "w": 4.6,\n       "h": 1.25', '"y": -0.16,\n       "w": 4.6,\n       "h": 1.41')
    seg = seg[:s0] + blk + seg[s1:]
    t = t[:f] + seg + t[e:]
    n += 1
print("edits", n)
open(P, "w", encoding="utf8").write(t)
