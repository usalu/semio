from pathlib import Path
import ast

ROOT=Path("🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip")


def replace_function(text, name, source):
    tree=ast.parse(text)
    node=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name==name)
    lines=text.splitlines(keepends=True)
    return "".join(lines[:node.lineno-1])+source.rstrip()+"\n"+"".join(lines[node.end_lineno:])


ROOM='''def _room(center, width, height, color=P_WHITE, t: float = 0.16, *,
          window_side="none", window_y=None, window_h=1.15):
    """🏠 Closed room material around declared openings, with slabs centred on the outline."""
    inner_h = height - t
    floor_y = float(center[1]) - inner_h / 2
    mid = float(center[1]) if window_y is None else window_y
    opening = ((mid - window_h / 2 - floor_y) / inner_h, (mid + window_h / 2 - floor_y) / inner_h)
    section = room_section(center, w=width - t, h=inner_h, wall=t, slab=t,
                           window=opening, window_side=window_side)
    body = VGroup(section["ceiling"], *[m for m in section["shell"] if m is not section["floor"] and m is not section["ceiling"]])
    return VGroup(body, section["floor"]).set_stroke(color).set_fill(color, opacity=1)
'''
WINDOW='''def _window(pos, height=1.15, width=0.16, color=P_CYAN, inward=1):
    """🪟 Recessed sash projected inward within a structural wall aperture."""
    win = window_glyph(float(pos[0]), float(pos[1]) - height / 2, float(pos[1]) + height / 2,
                       depth=0.8 * height, wall=width, color=color, inward=inward)
    win["sash"].become(window_opening(win))
    win["group"].window = win
    return win["group"]
'''
STAND='''def _stand(room_c, room_h, x, color=P_ORANGE, t: float = 0.16):
    """🧍 A 1.75 m occupant in a 2.5 m clear room, anchored to the slab top."""
    height = 0.7 * (room_h - t)
    floor_y = float(room_c[1]) - room_h / 2 + t / 2
    return person_glyph([x, floor_y + height / 2, 0], color=color, scale=height / 0.66)
'''
OPEN='''def _open_sash(window, *, run_time: float = 0.9, close: bool = False):
    """🚪 Project the shared sash open or closed around its recessed hinge."""
    return open_window(window, run_time=run_time, close=close)
'''
for name in ("🪟️natural", "❓️need", "⚙️mechanical", "📦️volume"):
    p=ROOT/name/"🐍️.py"
    text=p.read_text(encoding="utf-8")
    for alias in ("room_section", "window_glyph", "window_opening", "open_window"):
        if f"{alias} = _manim_module.{alias}" not in text:
            text=text.replace("person_glyph = _manim_module.person_glyph",f"{alias} = _manim_module.{alias}\nperson_glyph = _manim_module.person_glyph")
    text=replace_function(text,"_room",ROOM)
    text=replace_function(text,"_stand",STAND)
    if name in ("🪟️natural", "❓️need"):
        text=replace_function(text,"_window",WINDOW)
    if name in ("🪟️natural", "📦️volume"):
        text=replace_function(text,"_open_sash",OPEN)
    if name=="❓️need":
        text=text.replace("shell = _room(room_c, room_w, room_h)","shell = _room(room_c, room_w, room_h, window_side=\"both\", window_y=room_c[1] + 0.3)")
        text=text.replace("right_win = _window(np.array([room_c[0] + room_w / 2, win_y, 0.0]))", "right_win = _window(np.array([room_c[0] + room_w / 2, win_y, 0.0]), inward=-1)")
    if name=="🪟️natural":
        text=text.replace("return person_glyph(pos, color=color, scale=1.09 * scale)","return person_glyph(pos, color=color, scale=scale)")
        text=text.replace("shell = _room(room_c, 5.6, 2.5)", "shell = _room(room_c, 5.6, 2.5, window_side=\"left\", window_y=0.30, window_h=1.42)")
        start=text.index("        win_h = 1.42")
        end=text.index("        _shape_window()",start)
        text=text[:start]+'''        win_h = 1.42
        window = window_glyph(room_c[0] - 2.8, 0.30 - win_h / 2, 0.30 + win_h / 2,
                              depth=0.8 * win_h, wall=0.16, color=P_CYAN)
        frame, sash, gap = window["reveal"], window["sash"], window["reference"]
        opening = ValueTracker(0.0)

        def _shape_window(_m=None):
            sash.become(window_opening(window, fraction=opening.get_value()))

'''+text[end:]
        text=text.replace("shell = _room(room_c, 6.4, 2.2)", "shell = _room(room_c, 6.4, 2.2, window_side=\"both\", window_y=0.55)")
        text=text.replace("_window(np.array([room.get_left()[0], 0.55, 0.0]), height=1.15)", "_window(np.array([room_c[0] - 3.2, 0.55, 0.0]), height=1.15)")
        text=text.replace("_window(np.array([room.get_right()[0], 0.55, 0.0]), height=1.15)", "_window(np.array([room_c[0] + 3.2, 0.55, 0.0]), height=1.15, inward=-1)")
        text=text.replace("right_win.animate.stretch_to_fit_height(0.30)", "open_window(right_win.window, fraction=0.26)")
        text=text.replace("right_win.animate.stretch_to_fit_height(1.15)", "open_window(right_win.window)")
        text=text.replace("shell = _room(room_c, 4.2, 1.7)", "shell = _room(room_c, 4.2, 1.7, window_side=\"left\", window_y=-0.95, window_h=0.60)")
        text=text.replace("_window(np.array([room.get_left()[0], -0.95, 0.0]), height=0.60)", "_window(np.array([room_c[0] - 2.1, -0.95, 0.0]), height=0.60)")
        text=text.replace("icon_room = _room(room_c, 2.1, 1.25, t=0.12)", "icon_room = _room(room_c, 2.1, 1.25, t=0.12, window_side=\"both\", window_y=room_c[1] + 0.28, window_h=0.45)")
        text=text.replace("_window(icon.get_left() + UP * 0.28, height=0.45, width=0.16)", "_window(icon.get_left() + UP * 0.28, height=0.45, width=0.12)")
        text=text.replace("_window(icon.get_right() + UP * 0.28, height=0.45, width=0.16)", "_window(icon.get_right() + UP * 0.28, height=0.45, width=0.12, inward=-1)")
        text=text.replace("shell = _room(room_c, 4.4, 2.2)", "shell = _room(room_c, 4.4, 2.2, window_side=\"left\", window_y=0.55, window_h=1.0)")
        text=text.replace("_window(np.array([room.get_left()[0], 0.55, 0.0]), height=1.0)", "_window(np.array([room_c[0] - 2.2, 0.55, 0.0]), height=1.0)")
        text=text.replace("hinge = rk[\"sash\"].get_bottom()\n", "")
        text=text.replace("Rotate(rk[\"sash\"], angle=0.17, about_point=hinge)", "open_window(rk[\"window\"], fraction=0.17, tilt=True)")
        text=text.replace("window = window_glyph((x_in + x_out) / 2, sill, lintel, depth=0.42, color=P_CYAN)\n    window[\"side\"] = \"r\"", "window = window_glyph((x_in + x_out) / 2, sill, lintel, depth=0.9 * k, wall=t, inward=-1, color=P_CYAN)")
        text=text.replace("rad = radiator([x_in - 0.62, floor_y + 0.33, 0], color=P_RED)", "rad = radiator(ORIGIN, color=P_RED)\n    rad.stretch_to_fit_width(1.1 * k).stretch_to_fit_height(0.6 * k)\n    rad.move_to([x_in - 0.65 * k, floor_y + 0.42 * k, 0])")
        text=text.replace("height=0.16, **style", "height=0.20 * k, **style")
        text=text.replace("floor_y - 0.08", "floor_y - 0.10 * k").replace("ceil + 0.08", "ceil + 0.10 * k")
        text=text.replace("fill_opacity=0.14)\n    floor = Rectangle", "fill_opacity=1.0)\n    floor = Rectangle")
    if name=="⚙️mechanical":
        text=text.replace("part.set_stroke(P_WHITE).set_fill(P_WHITE, opacity=0.14)","part.set_stroke(P_WHITE).set_fill(P_WHITE, opacity=1)")
        text=text.replace("win1 = window_glyph(room1[\"glass_x\"], lo, hi, depth=0.4, color=P_CYAN)", "win1 = window_glyph(room1[\"glass_x\"], lo, hi, depth=0.4, wall=0.13, color=P_CYAN)")
    p.write_text(text,encoding="utf-8")
