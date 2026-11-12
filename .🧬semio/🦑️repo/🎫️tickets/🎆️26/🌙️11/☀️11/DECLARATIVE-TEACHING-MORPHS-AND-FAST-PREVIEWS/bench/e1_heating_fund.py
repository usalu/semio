import sys
sys.path.insert(0, ".")
from migrate import T, run
H = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/"
S = "heating.fundamentals."
M = "morphs.transition"
tr = lambda i, *a, **k: T(S + i, *a, **k)
DIM_OLD = '''    target = mobs.copy()
    for m in target.get_family():
        m.set_stroke(opacity=min(opacity, m.get_stroke_opacity()))
        if m.get_fill_opacity() > 0:
            m.set_fill(opacity=min(opacity, m.get_fill_opacity()))
    return Transform(mobs, target)'''
DIM_NEW = '''    members = [(m, m.get_stroke_opacity(), m.get_fill_opacity()) for m in mobs.get_family()]

    def blend(_, alpha):
        for m, stroke, fill in members:
            m.set_stroke(opacity=stroke + (min(opacity, stroke) - stroke) * alpha)
            if fill > 0:
                m.set_fill(opacity=fill + (min(opacity, fill) - fill) * alpha)

    return UpdateFromAlphaFunc(mobs, blend)'''
run(H + "🧲️fundamentals",
    [(DIM_OLD, DIM_NEW),
     ('ReplacementTransform(stage["ti"], ti_cool), ReplacementTransform(stage["te"], te_warm),', f'{M}("{S}direction.indoor-readout", source=stage["ti"], target=ti_cool), {M}("{S}direction.outdoor-readout", source=stage["te"], target=te_warm),'),
     ('self.play(ReplacementTransform(marker, lens["rim"]),', f'self.play({M}("{S}lattice.marker-to-lens", source=marker, target=lens["rim"]),'),
     ('ReplacementTransform(lat["group"], token),', f'{M}("{S}symbol.lattice-to-k", source=lat["group"], target=token),'),
     ("ReplacementTransform(energy, energy_k), run_time=1.4)", f'{M}("{S}symbol.energy-to-k-label", source=energy, target=energy_k), run_time=1.4)'),
     ("self.play(ReplacementTransform(rise_guide, loop_guide), FadeIn(cold_tag),", f'self.play({M}("{S}wall.rise-to-loop", source=rise_guide, target=loop_guide), FadeIn(cold_tag),'),
     ("self.play(ReplacementTransform(loop_guide, token), FadeOut(warm_tag)", f'self.play({M}("{S}symbol.loop-to-c", source=loop_guide, target=token), FadeOut(warm_tag)'),
     ("ReplacementTransform(air_tag, no_air), *glow(1.6, pulses=False)", f'{M}("{S}vacuum.air-tag-to-none", source=air_tag, target=no_air), *glow(1.6, pulses=False)'),
     ("self.play(ReplacementTransform(VGroup(inner, outer), token),", f'self.play({M}("{S}symbol.surfaces-to-r", source=VGroup(inner, outer), target=token),'),
     ("ReplacementTransform(mat, ins_tag),", f'{M}("{S}lam.material-to-insulation", source=mat, target=ins_tag),'),
     ("ReplacementTransform(r_eq, u_eq), ReplacementTransform(r_box, u_box),", f'{M}("{S}flip.r-equation-to-u", source=r_eq, target=u_eq), {M}("{S}flip.r-box-to-u", source=r_box, target=u_box),'),
     ("ReplacementTransform(te, te_cold),", f'{M}("{S}dt.outdoor-to-cold", source=te, target=te_cold),'),
     ("ReplacementTransform(te_cold, te_back),", f'{M}("{S}dt.cold-back", source=te_cold, target=te_back),')],
    [tr("direction.indoor-readout", "transformation", S + "direction.indoor-readout", S + "direction.indoor-cooled", "replace", 2.4),
     tr("direction.outdoor-readout", "transformation", S + "direction.outdoor-readout", S + "direction.outdoor-warmed", "replace", 2.4),
     tr("lattice.marker-to-lens", "concretization", S + "lattice.marker", S + "lattice.lens-rim", "replace", 1.0),
     tr("symbol.lattice-to-k", "abstraction", S + "symbol.lattice", S + "symbol.k-token", "replace", 1.4),
     tr("symbol.energy-to-k-label", "abstraction", S + "symbol.energy-chain", S + "symbol.k-chain", "replace", 1.4),
     tr("wall.rise-to-loop", "transformation", S + "wall.rise-guide", S + "wall.loop-guide", "replace", 0.8),
     tr("symbol.loop-to-c", "abstraction", S + "symbol.loop-guide", S + "symbol.c-token", "replace", 1.2),
     tr("vacuum.air-tag-to-none", "transformation", S + "vacuum.air-tag", S + "vacuum.no-air-tag", "replace", 1.6),
     tr("symbol.surfaces-to-r", "abstraction", S + "symbol.surfaces", S + "symbol.r-token", "replace", 1.2),
     tr("lam.material-to-insulation", "transformation", S + "lam.material-tag", S + "lam.insulation-tag", "replace", 1.9),
     tr("flip.r-equation-to-u", "transformation", S + "flip.r-equation", S + "flip.u-equation", "replace", 1.3),
     tr("flip.r-box-to-u", "continuation", S + "flip.r-box", S + "flip.u-box", "replace", 1.3),
     tr("dt.outdoor-to-cold", "transformation", S + "dt.outdoor-value", S + "dt.cold-value", "replace", 1.4),
     tr("dt.cold-back", "transformation", S + "dt.cold-value", S + "dt.restored-value", "replace", 1.0)],
    ["Beat1_DreiWegeDerWaerme", "Beat2_Waermeleitung", "Beat3_Konvektion", "Beat4_Strahlung", "Beat7_Waermedurchlasswiderstand", "Beat8_UWert", "Beat9_WaermestromFormel"], var="morphs")
