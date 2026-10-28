"""🛠️ Apply bounded section material changes without touching concurrent files."""

import ast
from pathlib import Path

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
ENERGY = ROOT / "🎓️teaching/🏛️architecture/⚡️energy"


def edit(relative, transform):
    path = ENERGY / relative
    source = path.read_text(encoding="utf-8")
    changed = transform(source)
    if "section_material(" in changed and "section_material = _manim_module.section_material" not in changed:
        marker = '_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")'
        assert marker in changed, relative
        changed = changed.replace(marker, marker + "\nsection_material = _manim_module.section_material", 1)
    ast.parse(changed)
    path.write_text(changed, encoding="utf-8")


def wrap(source, predicate):
    lines = source.splitlines(keepends=True)
    offsets = [0]
    for line in lines:
        offsets.append(offsets[-1] + len(line.encode("utf-8")))
    data = source.encode("utf-8")
    edits = []
    for node in ast.walk(ast.parse(source)):
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and predicate(node):
            node.keywords = [kw for kw in node.keywords if kw.arg not in {"color", "fill_color", "fill_opacity", "stroke_color", "stroke_width", "stroke_opacity"} and not (kw.arg is None and isinstance(kw.value, ast.Name) and kw.value.id in {"style", "slab", "wall_style", "_SLAB", "SHELL_STYLE"})]
            replacement = "section_material(" + ast.unparse(node) + ")"
            edits.append((offsets[node.lineno - 1] + node.col_offset, offsets[node.end_lineno - 1] + node.end_col_offset, replacement.encode("utf-8")))
    for start, end, replacement in sorted(edits, reverse=True):
        data = data[:start] + replacement + data[end:]
    return data.decode("utf-8")


def styled(source, style_names):
    return wrap(source, lambda node: node.func.id in {"Rectangle", "Polygon", "_box"} and any(kw.arg is None and isinstance(kw.value, ast.Name) and kw.value.id in style_names for kw in node.keywords))


def calculation(source):
    source = styled(source, {"_SLAB"})
    source = source.replace('_SLAB = dict(color=PASTEL_WHITE, stroke_width=1.3, fill_color=PASTEL_WHITE, fill_opacity=0.7)\n', '')
    source = source.replace('VGroup(h["ground"], h["floor"]).set_color(PASTEL_TEAL)', 'h["ground"].set_color(PASTEL_TEAL)')
    source = source.replace('room["shell"].set_stroke(PASTEL_WHITE).set_fill(PASTEL_WHITE)', 'section_material(room["shell"])')
    source = wrap(source, lambda node: node.func.id == "Polygon" and node.lineno > source[:source.index("def _plant_house")].count("\n"))
    return source


edit("🔥️heating/🎬️clip/🧮️calculation/🐍️.py", calculation)


def fundamentals(source):
    start, end = source.index("def _brick_wall():"), source.index("\n\ndef _radiator():")
    source = source[:start] + '''def _brick_wall():
    """🧱 Uniform exterior cut material with a separate thermal analysis tint."""
    y0, y1 = FLOOR_Y - SLAB, CEIL_Y + SLAB
    core = section_material(_box(WALL_X0, y0, WALL_X1, y1))
    tint = VGroup(*[
        _box(WALL_X0 + k * (WALL_X1 - WALL_X0) / 16, y0,
             WALL_X0 + (k + 1) * (WALL_X1 - WALL_X0) / 16, y1,
             stroke_width=0, fill_color=interpolate_color(ManimColor(HOT), ManimColor(PASTEL_BLUE), (k + 0.5) / 16),
             fill_opacity=0.0)
        for k in range(16)
    ])
    return {"core": core, "joints": VGroup(), "inner": VGroup(), "outer": VGroup(),
            "outline": VGroup().move_to(core), "tint": tint, "group": VGroup(core, tint)}
''' + source[end:]
    source = styled(source, {"SHELL_STYLE"})
    for line in ['BRICK_FILL = semio_color("dark-5-7")\n', 'MORTAR = semio_color("gray-600")\n', 'SHELL_STYLE = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)\n']:
        source = source.replace(line, '')
    return source


edit("🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py", fundamentals)
for chapter in ("❓️need", "⚙️mechanical", "📦️volume", "🪟️natural"):
    edit(f"🌬️ventilation/🎬️clip/{chapter}/🐍️.py", lambda source: source.replace('return VGroup(body, section["floor"]).set_stroke(color).set_fill(color, opacity=1)', 'return section_material(VGroup(body, section["floor"]))'))


def volume(source):
    source = source.replace('    style = dict(color=P_WHITE, stroke_width=1.1, fill_color=P_WHITE, fill_opacity=1)\n', '')
    source = source.replace('_poly(cut, o, s, **style)', 'section_material(_poly(cut, o, s))').replace('_poly(points, o, s, **style)', 'section_material(_poly(points, o, s))')
    return source


edit("🌬️ventilation/🎬️clip/📦️volume/🐍️.py", volume)
for relative in ("📊️demand/🎬️clip/♨️system-losses/🐍️.py", "📊️demand/🎬️clip/🧪️practice/🐍️.py", "📊️demand/🎬️clip/⚖️regulations/🐍️.py"):
    edit(relative, lambda source: styled(source, {"style"}))


def solar(source):
    start, end = source.index("def _hatch("), source.index("class Beat5_Verschattung")
    source = source[:start] + source[end:]
    source = styled(source, {"slab"})
    source = source.replace('VGroup(lintel, sill, _hatch(lintel), _hatch(sill))', 'VGroup(lintel, sill)')
    return source


edit("🔥️heating/🎬️clip/☀️solar-gains/🐍️.py", solar)


def natural(source):
    source = styled(source, {"style"})
    source = source.replace('Rectangle(width=t, height=ceil - lintel, color=P_WHITE, stroke_width=2, fill_color=P_ORANGE,\n                      fill_opacity=1)', 'section_material(Rectangle(width=t, height=ceil - lintel))')
    return source


edit("🌬️ventilation/🎬️clip/🪟️natural/🐍️.py", natural)


def conduction(source):
    source = source.replace('outer_top, inner_top, color=col["Außenwand"], stroke_width=1.3,\n                                    fill_color=col["Außenwand"], fill_opacity=0.7))', 'outer_top, inner_top)))')
    source = source.replace('h["wall_right"].become(Polygon(', 'h["wall_right"].become(section_material(Polygon(')
    start, end = source.index('    h["roof"].set_color(col["Dach"])'), source.index('\n#endregion', source.index('def _envelope_house'))
    source = source[:start] + '''    parts = {"Dach": h["roof"].copy(), "Außenwand": VGroup(h["walls"][0], h["wall_right"]).copy(),
             "Fenster": VGroup(*[w["sash"] for w in h["windows"]]), "Haustür": door["sash"], "Boden": h["floor"].copy()}
    overlays = VGroup(*[parts[name].set_fill(col[name], opacity=0.35).set_stroke(width=0, opacity=0)
                        for name in ("Dach", "Außenwand", "Boden")])
    return {**h, "door": door, "door_top": door_top, "parts": parts,
            "group": VGroup(h["group"], overlays, door["group"])}
''' + source[end:]
    return source


edit("🔥️heating/🎬️clip/🧱️conduction/🐍️.py", conduction)


def physics(source):
    source = styled(source, {"wall_style"})
    source = source.replace('Rectangle(width=house2["level_1"].width, height=0.18, color=P_ORANGE, stroke_width=2,\n                         fill_color=P_ORANGE, fill_opacity=0.5)', 'section_material(Rectangle(width=house2["level_1"].width, height=0.18))')
    source = source.replace('        self.play(FadeIn(e_out), FadeIn(out_bar), FadeIn(st_bar), FadeIn(out_t), FadeIn(st_t),', '        ceiling_heat = room["ceiling"].copy().set_fill(P_ORANGE, opacity=0)\n        room["group"].add(ceiling_heat)\n        self.add(ceiling_heat)\n        self.play(FadeIn(e_out), FadeIn(out_bar), FadeIn(st_bar), FadeIn(out_t), FadeIn(st_t),')
    source = source.replace('room["ceiling"].animate.set_fill(P_ORANGE, opacity=0.55)', 'ceiling_heat.animate.set_fill(opacity=0.55)').replace('room["ceiling"].animate.set_fill(P_WHITE, opacity=0.14)', 'ceiling_heat.animate.set_fill(opacity=0)')
    marker = '        #region absorb, long-wave and trap\n'
    source = source.replace(marker, marker + '        surface_heat = {key: room[key].copy().set_fill(P_ORANGE, opacity=0) for key in ("floor", "wall_lo", "wall_hi")}\n        room["group"].add(*surface_heat.values())\n        self.add(*surface_heat.values())\n')
    for key in ("floor", "wall_lo", "wall_hi"):
        source = source.replace(f'room["{key}"].animate.set_fill(P_ORANGE, opacity=0.4)', f'surface_heat["{key}"].animate.set_fill(opacity=0.4)').replace(f'room["{key}"].animate.set_fill(P_WHITE, opacity=0.14)', f'surface_heat["{key}"].animate.set_fill(opacity=0)')
    source = source.replace('        big["ceiling"].add_updater(lambda m: m.set_fill(P_ORANGE, opacity=ceil_fill.get_value()))', '        ceiling_heat = big["ceiling"].copy().set_fill(P_ORANGE, opacity=0)\n        floor_heat = big["floor"].copy().set_fill(P_ORANGE, opacity=0)\n        big["group"].add(ceiling_heat, floor_heat)\n        self.add(ceiling_heat, floor_heat)\n        ceiling_heat.add_updater(lambda m: m.set_fill(P_ORANGE, opacity=ceil_fill.get_value()))')
    source = source.replace('big["floor"].animate.set_fill(P_ORANGE, opacity=0.5)', 'floor_heat.animate.set_fill(opacity=0.5)').replace('big["floor"].animate.set_fill(P_WHITE, opacity=0.14)', 'floor_heat.animate.set_fill(opacity=0)').replace('big["ceiling"].clear_updaters()', 'ceiling_heat.clear_updaters()')
    return source


edit("🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py", physics)
