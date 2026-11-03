p = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🌬️ventilation\🎬️clip\⚙️mechanical\🐍️.py"
s = open(p, encoding="utf-8", newline="").read()
nl = "\r\n" if "\r\n" in s else "\n"
s = s.replace("\r\n", "\n")
BS = chr(92)


def rep(a, b):
    global s
    assert s.count(a) == 1, a
    s = s.replace(a, b)


rep('''        mvhr_box = RoundedRectangle(width=1.5, height=0.5, corner_radius=0.05, color=P_WHITE, fill_color=semio_color("dark-5-7"),
                                    fill_opacity=0.95, stroke_width=2).move_to([p2 + 0.15, 0.6, 0])
        lbl_mvhr = Text("WRG''' + BS + '''nZentral", font_size=LABEL_FONT_SIZE, color=P_YELLOW).scale(0.65)
        lbl_mvhr.move_to(mvhr_box.get_center() + LEFT * 0.33)
        fan2 = _fan(mvhr_box.get_center() + RIGHT * 0.42, radius=0.15)
        x_s, x_e = p2 - 0.35, p2 + 0.6
''', '''        mvhr_box = _mvhr(np.array([p2 + 0.15, 0.6, 0.0]))
        lbl_mvhr = Text("WRG Zentral", font_size=LABEL_FONT_SIZE, color=P_YELLOW).scale(0.65)
        lbl_mvhr.next_to(mvhr_box, UP, buff=0.08)
        x_s, x_e = mvhr_box.get_left()[0] + 0.1, mvhr_box.get_right()[0] - 0.1
''')
rep("(mvhr_box.get_left()[0] + 0.05, 0.62))]", "(mvhr_box.get_left()[0] + 0.02, 0.62))]")
rep("(mvhr_box.get_right()[0] - 0.05, 0.62), (p2 + 1.5, 0.95)", "(mvhr_box.get_right()[0] - 0.02, 0.62), (p2 + 1.5, 0.95)")
rep("mvhr_box, lbl_mvhr, fan2, lbl_supply,", "mvhr_box, lbl_mvhr, lbl_supply,")
rep("speed=0.55, waves=3), *_spin([fan2], rt)]", "speed=0.55, waves=3), *_spin([mvhr_box], rt)]")
rep('''        core = Rectangle(width=0.37, height=0.22, color=P_YELLOW, stroke_width=1.5, fill_color=P_ORANGE,
                         fill_opacity=0.9).move_to([gx3, y_u, 0])
        fan3 = _fan([x_l3 + 0.24, y_u, 0], radius=0.11)
''', '''        unit3 = _pendulum_unit(np.array([gx3, y_u, 0.0]))
''')
rep("person3, core, fan3, lbl_cyc1", "person3, unit3, lbl_cyc1")
rep('''                turn = _spin([fan3], rt / n)[0]
                turn.run_time = rt / n
''', '''                turn = _pendulum_half(unit3, pendulum["half"], rt / n)
''')
rep("                pendulum[\"out\"] = not out\n", "                pendulum[\"out\"] = not out\n                pendulum[\"half\"] += 1\n")
rep('pendulum = {"out": True}', 'pendulum = {"out": True, "half": 0}')
rep('''def _spin(fans, rt: float):''', '''PENDULUM_PHASES = 6
PENDULUM_PHASE_S = 70.0


def _mvhr(pos, height=0.62):
    """🌀 Handcrafted heat-recovery unit in thermal view, ``height`` tall, centred on ``pos``; its impeller turns once ``_spin`` starts it."""
    return fitted_staffage("ventilation-mechanical-mvhr", height=height, rate=0.0, loop=True).move_to(pos)


def _pendulum_unit(pos, size=0.4):
    """🔄 Handcrafted decentralised push-pull fan with its ceramic core in thermal view, fitted into a square of ``size``; ``_pendulum_half`` drives its clip."""
    return fitted_staffage("ventilation-mechanical-pendulum-fan", width=size, height=size, rate=0.0).move_to(pos)


def _pendulum_half(unit, half: int, rt: float):
    """↔️ Play one 70 s phase of the pendulum clip in ``rt`` seconds: the rotor turns one way and back while the core stores or gives up heat; from the seventh phase on the last two repeat."""
    index = half if half < PENDULUM_PHASES else PENDULUM_PHASES - 2 + half % 2
    start = index * PENDULUM_PHASE_S
    return UpdateFromAlphaFunc(unit, lambda m, a: m.seek(start + a * PENDULUM_PHASE_S), run_time=rt, rate_func=linear)


def _spin(fans, rt: float):''')
open(p, "w", encoding="utf-8", newline="").write(s.replace("\n", nl))
print("ok")
