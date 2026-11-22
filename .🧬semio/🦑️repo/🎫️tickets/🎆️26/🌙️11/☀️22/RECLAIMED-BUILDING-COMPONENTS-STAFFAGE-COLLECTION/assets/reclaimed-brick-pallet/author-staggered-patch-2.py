"""🔧️ Second one-off edit: per-layer slabs at 1:100, joints of staggered rows, true-width courses at every scale."""
import re
p = "author-brick-pallet.py"
s = open(p).read()
def rep(a, b):
    global s
    assert a in s, a[:90]
    s = s.replace(a, b)
start = s.index("def lines_of_stack(")
end = s.index("def corner_shapes(")
s = s[:start] + '''def layer_slab_shapes(bone, layer, out, scale):
    """〰️ One layer as a slab of its own extent with its joints as texture lines (1:100): in the side elevation the head joints of the right row (A) or the row ends (B), in the front the row ends (A) or the head joints of the front row (B)."""
    kind = kind_of(layer)
    ex0, ex1, ez0, ez1 = extent(kind)
    yb, yt = layer_y(layer), layer_y(layer) + BH
    pen = BRICK_PEN[scale]
    out["elevation"].append(shape(f"layer-{layer + 1:02d}-side", bone, "body", 2, tone_of(layer), erect(bone, ex0, ex1, yb, yt), region=R_BRICK, material=CLAY, pen=pen))
    out["front"].append(shape(f"layer-{layer + 1:02d}-front", bone, "body", 2, tone_of(layer, "f"), frect(ez0, ez1, yb, yt), region=R_BRICK, material=CLAY, pen=pen))
    xs = [a_x0(0) + PITCH_L * i + BL + GAP / 2 for i in range(3)] if kind == "A" else [b_x0(i) + BW for i in range(7)]
    zs = [a_z0(j) + BW for j in range(5)] if kind == "A" else [b_z0(7) + PITCH_L * j + BL + GAP / 2 for j in range(2)]
    for n, xj in enumerate(xs):
        out["elevation"].append(shape(f"joint-l{layer + 1:02d}-s{n + 1}", bone, "texture", 2.5, 0.3, epoly(bone, [(xj, yb), (xj, yt)], closed=False), region=R_BRICK, filled=False))
    for n, zj in enumerate(zs):
        out["front"].append(shape(f"joint-l{layer + 1:02d}-f{n + 1}", bone, "texture", 2.5, 0.3, fpoly([(zj, yb), (zj, yt)], closed=False), region=R_BRICK, filled=False))


''' + s[end:]
old_start = s.index("    else:\n        regions = [R_BRICK] if scale == 100")
old_end = s.index('    out["plan"].append(shape("stack-top"')
s = s[:old_start] + '''    elif scale == 100:
        for L in stack_layers:
            layer_slab_shapes("stack", L, out, scale)
        layer_slab_shapes("layer-14", 13, out, scale)
        layer_slab_shapes("layer-16", 15, out, scale)
    else:
        regions = [R_BRICK, R_MORTAR] if scale == 200 else [R_BRICK, R_MORTAR, R_CORNER, R_STRAP]
        hatch = "coursing" if scale == 200 else None
        out["elevation"].append(shape("stack-side", "stack", "body", 2, 0.43, erect("stack", SX0, SX1, SY0, layer_y(12)), regions=regions, material=CLAY, hatch=hatch, pen=BRICK_PEN[scale]))
        out["front"].append(shape("stack-front", "stack", "body", 2, 0.45, frect(SZ0, SZ1, SY0, layer_y(12)), regions=regions, material=CLAY, hatch=hatch, pen=BRICK_PEN[scale]))
        bx0, bx1, bz0, bz1 = extent("B")
        for name, L in (("layer-14", 13), ("layer-16", 15)):
            out["elevation"].append(shape(f"{name}-side", name, "body", 2, tone_of(L), erect(name, bx0, bx1, layer_y(L), layer_y(L) + BH), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
            out["front"].append(shape(f"{name}-front", name, "body", 2, tone_of(L, "f"), frect(bz0, bz1, layer_y(L), layer_y(L) + BH), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
''' + s[old_end:]
rep('''    out["plan"].append(shape("stack-top", "stack", "body", 2, 0.43, prect(SX0, SX1, SZ0, SZ1), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
    out["plan"].append(shape("layer-14-top", "layer-14", "body", 2.1, tone_of(13), prect(SX0, SX1, SZ0, SZ1), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))''', '''    top = extent("B") if scale <= 100 else (SX0, SX1, SZ0, SZ1)
    out["plan"].append(shape("stack-top", "stack", "body", 2, 0.43, prect(*top), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
    out["plan"].append(shape("layer-14-top", "layer-14", "body", 2.1, tone_of(13), prect(*extent("B")), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))''')
rep('''        out["plan"].append(shape("layer-16-top", "layer-16", "body", 2.2, tone_of(15, "t"), prect(SX0, SX1, SZ0, SZ1), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
        if scale == 100:
            for i in range(7):
                xj = SX0 + PITCH_W * i + BW + GAP / 2
                out["plan"].append(shape(f"layer-16-row-joint-{i + 1}", "layer-16", "texture", 2.5, 0.3, ppoly([(xj, SZ0), (xj, SZ1)], closed=False), region=R_BRICK, filled=False))
            for j in range(2):
                zj = SZ0 + PITCH_L * j + BL + GAP / 2
                out["plan"].append(shape(f"layer-16-head-joint-{j + 1}", "layer-16", "texture", 2.5, 0.3, ppoly([(SX0, zj), (SX1, zj)], closed=False), region=R_BRICK, filled=False))''', '''        out["plan"].append(shape("layer-16-top", "layer-16", "body", 2.2, tone_of(15, "t"), prect(*extent("B")), region=R_BRICK, material=CLAY, pen=BRICK_PEN[scale]))
        if scale == 100:
            for i in range(7):
                xj = b_x0(i) + BW
                out["plan"].append(shape(f"layer-16-row-joint-{i + 1}", "layer-16", "texture", 2.5, 0.3, ppoly([(xj, SZ0), (xj, SZ1)], closed=False), region=R_BRICK, filled=False))
            for i in range(8):
                for j in range(2):
                    zj = b_z0(i) + PITCH_L * j + BL + GAP / 2
                    out["plan"].append(shape(f"layer-16-head-joint-{i + 1}-{j + 1}", "layer-16", "texture", 2.5, 0.3, ppoly([(b_x0(i), zj), (b_x0(i) + BW, zj)], closed=False), region=R_BRICK, filled=False))''')
rep('''                if (L, key) in CHIPS and abs(z0 - SZ0) < 1e-9:''', '''                if (L, key) in CHIPS and ((kind == "A" and key[0] == 0) or (kind == "B" and key[1] == 0)):''')
rep('''        return wbox(f"{cid}-bricks", cid, SX0, SX0 + BL,''', '''        return wbox(f"{cid}-bricks", cid, c["x0"], c["x0"] + BL,''')
rep('''    w = {100: BW, 200: 0.125, 500: 0.130}[scale]
    return wbox(f"{cid}-row", cid, SX0, SX1, y0, y0 + BH, c["zc"] - w / 2, c["zc"] + w / 2, CLAY, tone_of("c", k))''', '''    return wbox(f"{cid}-row", cid, c["x0"], c["x0"] + ROW_L, y0, y0 + BH, c["zc"] - BW / 2, c["zc"] + BW / 2, CLAY, tone_of("c", k, scale))''')
start = s.index("def stack_block(scale):")
end = s.index("def boxes(")
s = s[:start] + '''def stack_block(scale):
    """🧊️ The twelve stack layers as slabs of their own extent at 1:100 (the bricks of a layer merged), as one block of the stack's envelope at 1:200 and 1:500; the two set-down layers as slabs."""
    out = []
    if scale == 100:
        for L in range(12):
            x0, x1, z0, z1 = extent(kind_of(L))
            out.append(wbox(f"stack-layer-{L + 1:02d}", "stack", x0, x1, layer_y(L), layer_y(L) + BH, z0, z1, CLAY, tone_of(L, "slab")))
    else:
        out.append(wbox("stack-block", "stack", SX0, SX1, SY0, layer_y(12), SZ0, SZ1, CLAY, 0.44))
    bx0, bx1, bz0, bz1 = extent("B")
    out += [wbox("layer-14-slab", "layer-14", bx0, bx1, layer_y(13), layer_y(13) + BH, bz0, bz1, CLAY, tone_of(13)),
            wbox("layer-16-slab", "layer-16", bx0, bx1, layer_y(15), layer_y(15) + BH, bz0, bz1, CLAY, tone_of(15))]
    return out


''' + s[end:]
rep('''    structure.append(dict(wbox("st-stack", "stack", SX0, SX1, SY0, layer_y(12), SZ0, SZ1, CLAY, 0.44), member="stack", part="member"))
    for name, L in (("layer-14", 13), ("layer-16", 15)):
        structure.append(dict(wbox(f"st-{name}", name, SX0, SX1, layer_y(L), layer_y(L) + BH, SZ0, SZ1, CLAY, 0.44), member=name, part="member"))
    for k in range(12):
        c = course_rows()[k]
        w = {50: BW, 100: BW, 200: 0.125, 500: 0.130}[scale]
        structure.append(dict(wbox(f"st-{COURSE_IDS[k]}", COURSE_IDS[k], SX0, SX1, layer_y(c["layer"]), layer_y(c["layer"]) + BH, c["zc"] - w / 2, c["zc"] + w / 2, CLAY, 0.44), member=COURSE_IDS[k], part="member"))''', '''    structure.append(dict(wbox("st-stack", "stack", 0.12, 1.08, SY0, layer_y(12), -0.36, 0.36, CLAY, 0.44), member="stack", part="member"))
    for name, L in (("layer-14", 13), ("layer-16", 15)):
        structure.append(dict(wbox(f"st-{name}", name, *extent("B")[:2], layer_y(L), layer_y(L) + BH, *extent("B")[2:], CLAY, 0.44), member=name, part="member"))
    for k in range(12):
        c = course_rows()[k]
        structure.append(dict(wbox(f"st-{COURSE_IDS[k]}", COURSE_IDS[k], c["x0"], c["x0"] + ROW_L, layer_y(c["layer"]), layer_y(c["layer"]) + BH, c["zc"] - BW / 2, c["zc"] + BW / 2, CLAY, 0.44), member=COURSE_IDS[k], part="member"))''')
rep('''PLATE_LOAD_N = 3000.0 * (SX1 - SX0)
WIND_N = 600.0 * (SX1 - SX0 + 0.13) * (11 * COURSE + BH)''', '''PLATE_LOAD_N = 3000.0 * ROW_L
WIND_N = 600.0 * (ROW_L + 0.13) * (11 * COURSE + BH)''')
open(p, "w").write(s)
print("ok")
