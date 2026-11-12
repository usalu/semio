import sys
sys.path.insert(0, ".")
from migrate import T, run
H = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/"
S = "physics.fundamentals."
X = "stage.transition"
tr = lambda i, *a, **k: T(S + i, *a, **k)


def fly(prefix, key):
    return T(f"{S}{prefix}.fly.{key}", "abstraction", f"{S}{prefix}.fly.{key}.readout", f"{S}{prefix}.fly.{key}.slot", "fade", 0.48)


KETTLE_OLD = '''        self.play(Transform(name, Text(tr("Kettle", "Wasserkocher"), font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(name)),
                  FadeOut(device_icon), FadeIn(kettle_icon), run_time=0.5)
'''
KETTLE_NEW = f'''        kettle_name = Text(tr("Kettle", "Wasserkocher"), font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(name)
        self.play({X}("{S}devices.vacuum-to-kettle-name", source=name, target=kettle_name),
                  FadeOut(device_icon), FadeIn(kettle_icon), run_time=0.5)
        name = kettle_name
'''
SHOWER_OLD = '''        self.play(Transform(name, Text(tr("Shower (instantaneous heater)", "Dusche (Durchlauferhitzer)"), font_size=BODY_FONT_SIZE,
                                       color=P_CYAN).move_to(name)), FadeOut(kettle_icon), FadeIn(shower_icon), run_time=0.5)
'''
SHOWER_NEW = f'''        shower_name = Text(tr("Shower (instantaneous heater)", "Dusche (Durchlauferhitzer)"), font_size=BODY_FONT_SIZE,
                           color=P_CYAN).move_to(name)
        self.play({X}("{S}devices.kettle-to-shower-name", source=name, target=shower_name), FadeOut(kettle_icon), FadeIn(shower_icon), run_time=0.5)
        name = shower_name
'''
WATT_OLD = '''self.play(ReplacementTransform(ax["xl"], new_xl), ReplacementTransform(ax["yl"], new_yl),
                  ReplacementTransform(t_lbls, new_t), ReplacementTransform(p_lbls, new_p),
                  ReplacementTransform(h_lbl, new_h), ReplacementTransform(v_lbl, new_v), run_time=1.2)'''
WATT_NEW = f'''self.play({X}("{S}watt.x-label", source=ax["xl"], target=new_xl), {X}("{S}watt.y-label", source=ax["yl"], target=new_yl),
                  {X}("{S}watt.time-ticks", source=t_lbls, target=new_t), {X}("{S}watt.power-ticks", source=p_lbls, target=new_p),
                  {X}("{S}watt.time-brace", source=h_lbl, target=new_h), {X}("{S}watt.power-brace", source=v_lbl, target=new_v), run_time=1.2)'''
CLOUD_OLD = '''kw.animate.set_color(P_CYAN), Transform(cloud, wide),
                  run_time=0.7)'''
CLOUD_NEW = f'''kw.animate.set_color(P_CYAN), {X}("{S}said.cloud-widens", source=cloud, target=wide),
                  run_time=0.7)
        cloud = wide'''

run(H + "🧲️fundamentals",
    [("def _fill_panel(scene, panel, sources, *, run_time: float = 1.2, fly: bool = True):", "def _fill_panel(scene, stage, prefix, panel, sources, *, run_time: float = 1.2, fly: bool = True):"),
     ("scene.play(*[FadeTransform(copies[key], items[key]) for key in sources], run_time=run_time * 0.4)", 'scene.play(*[stage.transition(f"{prefix}.{key}", source=copies[key], target=items[key]) for key in sources], run_time=run_time * 0.4)'),
     ('_fill_panel(self, panel, {"kw": kw, "h": h_glyph})', f'_fill_panel(self, stage, "{S}said.fly", panel, {{"kw": kw, "h": h_glyph}})'),
     ('_fill_panel(self, heat_panel, {"m": litre_read, "dt": temp_read})', f'_fill_panel(self, stage, "{S}heat.fly", heat_panel, {{"m": litre_read, "dt": temp_read}})'),
     ('_fill_panel(self, (row, box, items), {"gv": g_lbl, "av": a_read, "iv": i_lbl}, fly=False)', f'_fill_panel(self, stage, "{S}gain.fly", (row, box, items), {{"gv": g_lbl, "av": a_read, "iv": i_lbl}}, fly=False)'),
     ("self.play(ReplacementTransform(power_unit, unit_definition), run_time=0.8)", f'self.play({X}("{S}unit.watt-name-to-definition", source=power_unit, target=unit_definition), run_time=0.8)'),
     ("ReplacementTransform(tank_static, target), tank_lbl.animate.move_to(pt(0.5, 0.5)), run_time=1.4)", f'{X}("{S}power.tank-to-area", source=tank_static, target=target), tank_lbl.animate.move_to(pt(0.5, 0.5)), run_time=1.4)'),
     (KETTLE_OLD, KETTLE_NEW),
     (SHOWER_OLD, SHOWER_NEW),
     (WATT_OLD, WATT_NEW),
     ("self.play(ReplacementTransform(q_read, year_lbl), run_time=0.6)", f'self.play({X}("{S}demand.readout-to-year", source=q_read, target=year_lbl), run_time=0.6)'),
     (CLOUD_OLD, CLOUD_NEW),
     ("ReplacementTransform(tank_static, dots[1]),", f'{X}("{S}zeros.tank-to-dot", source=tank_static, target=dots[1]),'),
     ('self.play(ReplacementTransform(house2["level_1"].copy(), slab), FadeIn(right_tags[0]),', f'self.play({X}("{S}house.slab-from-level", source=house2["level_1"], target=slab), FadeIn(right_tags[0]),'),
     ("self.play(ReplacementTransform(forms[0], forms[1], rate_func=linear), FadeIn(law_glow, scale=0.4),", f'self.play({X}("{S}law.sum-to-heat", source=forms[0], target=forms[1]), FadeIn(law_glow, scale=0.4),'),
     ("self.play(ReplacementTransform(forms[1], forms[2], rate_func=linear), law_rays.animate.set_color(P_ORANGE),", f'self.play({X}("{S}law.heat-to-room", source=forms[1], target=forms[2]), law_rays.animate.set_color(P_ORANGE),'),
     ("ReplacementTransform(light_read, light_heat), run_time=1.0)", f'{X}("{S}absorb.light-to-heat", source=light_read, target=light_heat), run_time=1.0)'),
     ("self.play(TransformFromCopy(w_bar, tower[k]), count.animate.set_value(k + 1), run_time=0.45)", f'self.play({X}("{S}cop.bar-copy", source=w_bar, target=tower[k]), count.animate.set_value(k + 1), run_time=0.45)'),
     ("self.play(ReplacementTransform(beams_out, stopped), shaft.animate.set_fill(opacity=0.03)", f'self.play({X}("{S}shade.beams-stopped", source=beams_out, target=stopped), shaft.animate.set_fill(opacity=0.03)'),
     ("self.play(ReplacementTransform(slab, tiles[0]), FadeOut(kwh_read), run_time=1.0)", f'self.play({X}("{S}tiles.slab-to-first", source=slab, target=tiles[0]), FadeOut(kwh_read), run_time=1.0)'),
     ("self.play(LaggedStart(*[TransformFromCopy(tiles[0], t) for t in tiles[1:]], lag_ratio=0.08),", f'self.play({X}("{S}tiles.first-to-grid", source=[tiles[0]], target=list(tiles[1:])),'),
     ("self.play(ReplacementTransform(sun, moon), _open_window(night_window, tilt=True)", f'self.play({X}("{S}night.sun-to-moon", source=sun, target=moon), _open_window(night_window, tilt=True)'),
     ("self.play(ReplacementTransform(moon, sun2), _open_window(night_window, close=True)", f'self.play({X}("{S}night.moon-to-sun", source=moon, target=sun2), _open_window(night_window, close=True)'),
     ("self.play(ReplacementTransform(sun2, evening), FadeOut(winter_rays", f'self.play({X}("{S}night.sun-to-evening", source=sun2, target=evening), FadeOut(winter_rays'),
     ("self.play(*[ReplacementTransform(m, d) for m, d in zip(movers, drops)], FadeIn(drop_tag)", f'self.play({X}("{S}condensation.movers-to-drops", source=movers, target=drops), FadeIn(drop_tag)')],
    [fly("said", "kw"), fly("said", "h"), fly("heat", "m"), fly("heat", "dt"),
     tr("unit.watt-name-to-definition", "abstraction", S + "unit.watt-name", S + "unit.watt-definition", "replace", 0.8),
     tr("power.tank-to-area", "abstraction", S + "power.tank", S + "power.area", "replace", 1.4),
     tr("devices.vacuum-to-kettle-name", "transformation", S + "devices.vacuum-name", S + "devices.kettle-name", "replace", 0.5),
     tr("devices.kettle-to-shower-name", "transformation", S + "devices.kettle-name", S + "devices.shower-name", "replace", 0.5),
     tr("watt.x-label", "transformation", S + "watt.x-label", S + "watt.x-label-seconds", "replace", 1.2),
     tr("watt.y-label", "transformation", S + "watt.y-label", S + "watt.y-label-watt", "replace", 1.2),
     tr("watt.time-ticks", "transformation", S + "watt.time-ticks", S + "watt.time-ticks-seconds", "replace", 1.2),
     tr("watt.power-ticks", "transformation", S + "watt.power-ticks", S + "watt.power-ticks-watt", "replace", 1.2),
     tr("watt.time-brace", "transformation", S + "watt.time-brace", S + "watt.time-brace-seconds", "replace", 1.2),
     tr("watt.power-brace", "transformation", S + "watt.power-brace", S + "watt.power-brace-watt", "replace", 1.2),
     tr("demand.readout-to-year", "transformation", S + "demand.readout", S + "demand.year-label", "replace", 0.6),
     tr("said.cloud-widens", "continuation", S + "said.cloud", S + "said.wide-cloud", "replace", 0.7),
     tr("zeros.tank-to-dot", "abstraction", S + "zeros.tank", S + "zeros.dot", "replace", 1.2),
     tr("house.slab-from-level", "concretization", S + "house.level", S + "house.slab", "replace", 1.0, copy=True),
     tr("law.sum-to-heat", "transformation", S + "law.sum-form", S + "law.heat-form", "replace", 4.0, easing="linear"),
     tr("law.heat-to-room", "transformation", S + "law.heat-form", S + "law.room-form", "replace", 4.0, easing="linear"),
     tr("absorb.light-to-heat", "transformation", S + "absorb.light-readout", S + "absorb.heat-readout", "replace", 1.0),
     tr("cop.bar-copy", "concretization", S + "cop.work-bar", S + "cop.tower-bar", "replace", 0.45, copy=True),
     tr("shade.beams-stopped", "transformation", S + "shade.beams", S + "shade.stopped-beams", "replace", 1.4),
     tr("tiles.slab-to-first", "abstraction", S + "tiles.slab", S + "tiles.first-tile", "replace", 1.0),
     tr("tiles.first-to-grid", "concretization", S + "tiles.first-tile-seed", S + "tiles.grid-tiles", "split", 2.4, relation="one-to-many", corr="pairs", pairs=[["0", "0"]], lag=0.192, copy=True),
     tr("night.sun-to-moon", "transformation", S + "night.sun", S + "night.moon", "replace", 0.9),
     tr("night.moon-to-sun", "transformation", S + "night.moon", S + "night.winter-sun", "replace", 0.8),
     tr("night.sun-to-evening", "transformation", S + "night.winter-sun", S + "night.evening-moon", "replace", 0.7),
     tr("condensation.movers-to-drops", "concretization", S + "condensation.movers", S + "condensation.drops", "replace", 0.8, relation="many-to-many")],
    ["Beat1_Leistung", "Beat2_EnergieImAlltag", "Beat3_Energieerhaltung", "Beat4_Waermepumpe", "Beat5_Strahlung", "Beat6_ThermischeMasse", "Beat7_SensibelLatent"])
