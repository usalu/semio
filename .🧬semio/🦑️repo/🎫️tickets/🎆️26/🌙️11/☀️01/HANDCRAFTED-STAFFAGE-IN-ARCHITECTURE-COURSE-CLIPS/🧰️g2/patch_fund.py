import time

F = "🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py"
s = open(F, encoding="utf-8", newline="").read()
nl = "\r\n" if "\r\n" in s else "\n"
s = s.replace("\r\n", "\n")


def between(start, end, new, keep_end=True):
    global s
    assert s.count(start) == 1, start
    a = s.index(start)
    b = s.index(end, a)
    s = s[:a] + new + (s[b:] if keep_end else s[b + len(end):])


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:80]
    s = s.replace(old, new)


between("def _blind(x: float", "#endregion\n\n\n#region Glyphs — buildings", '''BLIND_CAST = "physics-fundamentals-blind"


def _blind(x: float, y_top: float, y_bot: float):
    """🪟 External venetian blind of the staffage archive fitted between ``y_top`` and ``y_bot`` and centred on ``x``.

    ``slats`` holds the sun-facing face of every lamella as (outer end, inner end), ``ys`` their centres, ``angle`` the lamella angle and ``group`` the drawn cast.

    @see ../../../../../🧍️staffage/🗂️catalog/🧱️components/external-venetian-blind/🔣️.json — the cast the lamellae are read from
    """
    top = float(_staffage_parts(BLIND_CAST, "box")["box"][:, 1].max())
    bottom = float(_staffage_parts(BLIND_CAST, "bottom-rail")["bottom-rail"][:, 1].min())
    unit = (y_top - y_bot) / (top - bottom)
    anchor = np.array([x + 0.075 * unit, y_bot - bottom * unit, 0.0])
    group = _staffage(BLIND_CAST, unit=unit, anchor=anchor, window=(-1.0, bottom - 1e-3, 0.0, top + 1e-3))
    slats = []
    for _, polygon in sorted(_staffage_parts(BLIND_CAST, "slat-").items(), key=lambda item: int(item[0].rsplit("-", 1)[1])):
        edges = sorted(((polygon[i], polygon[(i + 1) % len(polygon)]) for i in range(len(polygon))),
                       key=lambda edge: -float(np.linalg.norm(edge[1] - edge[0])))[:2]
        upper = max(edges, key=lambda edge: edge[0][1] + edge[1][1])
        outer, inner = sorted(upper, key=lambda point: float(point[0]))
        slats.append((np.append(anchor[:2] + outer * unit, 0.0), np.append(anchor[:2] + inner * unit, 0.0)))
    axis = slats[0][1] - slats[0][0]
    return {"ys": np.array([(a[1] + b[1]) / 2 for a, b in slats]), "slats": slats, "angle": float(np.arctan2(axis[1], axis[0])),
            "group": group.set_z_index(4)}


def _blind_hit(blind, start, direction):
    """🎯 First point where a straight ray meets a lamella of ``blind`` — rays end on the lamella faces, not on a flat line."""
    start, direction = np.array(start, dtype=float), np.array(direction, dtype=float)
    best = None
    for outer, inner in blind["slats"]:
        axis = inner - outer
        matrix = np.array([[direction[0], -axis[0]], [direction[1], -axis[1]]])
        if abs(np.linalg.det(matrix)) < 1e-9:
            continue
        along, position = np.linalg.solve(matrix, (outer - start)[:2])
        if along > 0 and 0.0 <= position <= 1.0 and (best is None or along < best):
            best = along
    if best is None:
        best = (blind["slats"][0][0][0] - start[0]) / direction[0]
    return start + direction * best
''')

between("def _flame(base", "def _clock(", "")
between("def _clock(", "def _vacuum(", "_clock = _manim_module.clock_glyph\n\n\n")

sub("_staffage_height = _manim_module.staffage_height\n",
    "_staffage_height = _manim_module.staffage_height\n_staffage_parts = _manim_module.staffage_parts\n_fitted_staffage = _manim_module.fitted_staffage\n"
    "HOB_UNIT = 6.0\nHOB_PAN = 0.088\nPOT_INNER = 0.22\nPOT_HEIGHT = 0.13\nPOT_BOIL_RATE = 66.0\nKETTLE_RATE = 60.0\n")

between("        hob = _hob(4.6, QO[1], width=1.4)", "        col_x = bl[0] - 0.35", '''        hob = _staffage_motion("physics-fundamentals-hob", unit=HOB_UNIT, anchor=np.array([4.7, QO[1], 0.0]))
        pan_y = QO[1] + HOB_PAN * HOB_UNIT
        pot_w, pot_h = POT_INNER * HOB_UNIT, POT_HEIGHT * HOB_UNIT
        pot_c = np.array([4.7, pan_y + pot_h / 2, 0.0])
        bl, br = pot_c + np.array([-pot_w / 2, -pot_h / 2, 0.0]), pot_c + np.array([pot_w / 2, -pot_h / 2, 0.0])
        pot = _staffage_motion("physics-fundamentals-pot", unit=HOB_UNIT, anchor=np.array([4.7, pan_y, 0.0]), rate=0.0)
''')
sub('''        self.play(FadeIn(qax["group"]), FadeIn(t_ticks), FadeIn(pot), FadeIn(water), FadeIn(hob["head"]),
                  FadeIn(hob["supports"]), FadeIn(mass_tag), run_time=1.0)
        self.play(FadeIn(hob["flames"], scale=0.3, shift=DOWN * 0.1), run_time=0.5)
''', '''        self.play(FadeIn(qax["group"]), FadeIn(t_ticks), FadeIn(pot), FadeIn(hob), FadeIn(mass_tag), run_time=1.0)
''')
sub('''        self.play(Q.animate.set_value(210.0), water.animate.set_fill(interpolate_color(
            ManimColor(P_BLUE), ManimColor(P_RED), 0.5)), run_time=2.0, rate_func=linear)''',
    '''        pot.rate = POT_BOIL_RATE
        self.play(Q.animate.set_value(210.0), run_time=2.0, rate_func=linear)''')
sub('''        self.play(Q.animate.set_value(419.0), water.animate.set_fill(P_RED, opacity=0.5), run_time=1.6,
                  rate_func=linear)''', '''        self.play(Q.animate.set_value(419.0), run_time=1.6, rate_func=linear)''')
sub('''                    extra=[Q.animate.set_value(2676.0),
                           water.animate.stretch_to_fit_height(0.03, about_edge=DOWN).set_fill(opacity=0.3)])''',
    '''                    extra=[Q.animate.set_value(2676.0)])''')

sub('''        out_blind = _blind(gx - 0.3, room["win_hi"] + 0.06, room["win_lo"] + 0.18)
        rail, cords, slats = out_blind["rail"], out_blind["cords"], out_blind["slats"]
''', '''        out_blind = _blind(gx - 0.3, room["win_hi"] + 0.06, room["win_lo"] + 0.18)
        blind_group = out_blind["group"]
''')
sub('''        self.play(FadeIn(rail), Create(cords), LaggedStart(*[FadeIn(s, shift=DOWN * 0.25) for s in slats],
                                                            lag_ratio=0.12),
                  FadeIn(blind_lbl), run_time=1.2)''', '''        self.play(FadeIn(blind_group, shift=DOWN * 0.25), FadeIn(blind_lbl), run_time=1.2)''')
sub('''                  slats.animate.set_stroke(color=interpolate_color(ManimColor(P_TEAL), ManimColor(P_ORANGE), 0.45)),''',
    '''                  blind_group.animate.set_stroke(color=interpolate_color(ManimColor(P_TEAL), ManimColor(P_ORANGE), 0.45)),''')
sub('''        in_rail, in_cords, in_slats = in_blind["rail"], in_blind["cords"], in_blind["slats"]
''', '''        in_group = in_blind["group"]
''')
sub('''        self.play(FadeOut(VGroup(rail, cords, slats, blind_lbl, norm2)), FadeOut(stopped), run_time=0.6)
        self.play(FadeIn(in_rail), Create(in_cords), LaggedStart(*[FadeIn(s_, shift=DOWN * 0.2) for s_ in in_slats],
                                                                  lag_ratio=0.1), FadeIn(in_lbl),''',
    '''        self.play(FadeOut(VGroup(blind_group, blind_lbl, norm2)), FadeOut(stopped), run_time=0.6)
        self.play(FadeIn(in_group, shift=DOWN * 0.2), FadeIn(in_lbl),''')
sub("in_slats.animate.set_stroke(color=warm_slats)", "in_group.animate.set_stroke(color=warm_slats)")

between("        def exchanger(x, color):", "        lbl_evap = Text(", '''        evap = _fitted_staffage("physics-fundamentals-evaporator", width=0.9, height=2.25).move_to(np.array([-3.2, 0.375, 0.0]))
        cond = _fitted_staffage("physics-fundamentals-condenser", width=0.9, height=2.25).move_to(np.array([3.0, 0.375, 0.0]))
        comp = _fitted_staffage("physics-fundamentals-compressor", height=0.9, rate=0.0, loop=True).move_to(comp_c)
        valve = _fitted_staffage("physics-fundamentals-valve", width=0.9).move_to(valve_c)
''')
sub("        cycle(extra=[Rotate(comp[1:], angle=TAU * 2, about_point=comp_c)])", "        comp.rate = 1.0\n        cycle()")

sub('''        self.play(FadeOut(tank_lbl), FadeIn(device_frame), FadeIn(name), FadeIn(power_control), run_time=0.5)''',
    '''        device_icon = _fitted_staffage("physics-fundamentals-vacuum", width=0.9, height=0.62, rate=1.0, loop=True).move_to(np.array([0.95, 1.95, 0.0]))
        self.play(FadeOut(tank_lbl), FadeIn(device_frame), FadeIn(name), FadeIn(device_icon), FadeIn(power_control),
                  run_time=0.5)''')
sub('''        self.play(Transform(name, Text("Wasserkocher", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(name)),
                  run_time=0.5)''', '''        kettle_icon = _fitted_staffage("physics-fundamentals-kettle", width=0.9, height=0.62, rate=KETTLE_RATE).move_to(np.array([0.95, 1.95, 0.0]))
        self.play(Transform(name, Text("Wasserkocher", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(name)),
                  FadeOut(device_icon), FadeIn(kettle_icon), run_time=0.5)''')

s = s.replace("\n", nl)
for _ in range(60):
    try:
        open(F, "w", encoding="utf-8", newline="").write(s)
        break
    except OSError:
        time.sleep(0.5)
print("patched")
