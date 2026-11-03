from pathlib import Path

path = Path("🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py")
text = open(str(path), encoding="utf8", newline="").read()


def swap(old, new):
    global text
    assert text.count(old) == 1, old
    text = text.replace(old, new)


start = text.index("def _heat_pump(center):")
end = text.index("def _meter_glyph(")
text = text[:start] + '''def _heat_pump(center):
    """♻️ Air-source heat pump outdoor unit in winter air, fitted into a 0.8 × 0.56 box; it stands still until ``rate`` is raised."""
    return fitted_staffage("demand-primary-energy-heat-pump", width=0.8, height=0.56, rate=0.0).move_to(_pt(center))


''' + text[end:]
swap('''        heat_pump = _heat_pump(np.array([xs["plant"], G3 + 0.31, 0.0]))
        plant, fan = heat_pump["group"], heat_pump["fan"]
''', '''        plant = fan = _heat_pump(np.array([xs["plant"], G3 + 0.31, 0.0]))
''')
swap('''            streams.append(lambda rt, p=p: [Rotate(p["fan"][1], angle=-TAU * rt / 0.9,
                                                  about_point=p["fan"][0].get_center(), rate_func=linear)])
''', '''            streams.append(lambda rt, p=p: setattr(p["fan"], "rate", 1.0) or [])
''')
swap("staffage_height = _manim_module.staffage_height\n", "staffage_height = _manim_module.staffage_height\nfitted_staffage = _manim_module.fitted_staffage\n")
open(str(path), "w", encoding="utf8", newline="").write(text)
print("ok")
