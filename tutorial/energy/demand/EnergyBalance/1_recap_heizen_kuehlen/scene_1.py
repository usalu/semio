import numpy as np
from manim import *

from pathlib import Path as _Path
import sys as _sys

_TUTORIAL_ROOT = next(
    p for p in _Path(__file__).resolve().parents
    if (p / "manim_fonts.py").is_file()
)
if str(_TUTORIAL_ROOT) not in _sys.path:
    _sys.path.insert(0, str(_TUTORIAL_ROOT))

from manim_fonts import (
    apply_scene_style, scene_title, play_scene_title, TITLE_RUN_TIME,
    beat_subtitle, BEAT_SUBTITLE_FADE,
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE, body_text,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, CONTENT_TOP,
    chip, stacked_bar, side_labels,
    smooth_path, flow_guides, flow_animation,
    house_section, person_glyph, sun_glyph, sun_rays, shine, pulse_flashes, ripples,
    lamp_glyph, radiator, open_window, thermometer_glyph, clock_glyph, meter, set_meter, bind_meter,
    math_label, math_panel, math_readout, de_num,
    dim_arrow, highlight_param, caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ One persistent chapter title — written once on Beat 1, re-added on later beats.
TITLE_DE = "Die Energiebilanz des Gebäudes"

# Carried over unchanged from the Heating and Cooling series so a viewer who
# watched those recognises each flow by its colour before reading its label.
LOSS_BLUE = "#38BDF8"     # Transmission — Heating/2_conduction
VENT_BLUE = "#60A5FA"     # Lüftung — Heating/3_convection
SOLAR_YELLOW = "#FDE047"  # Solare Gewinne — Heating/5_solar_heat_gain
INT_ORANGE = "#F59E0B"    # Interne Gewinne — Heating/4_internal_heat_gain
ETA_GREEN = "#22C55E"     # Ausnutzungsgrad
DEMAND_RED = "#EF4444"    # Was die Anlage liefern muss

HOUSE_SCALE = 0.8


#region Example house
# One set of numbers for the whole chapter — the same example house the Heating
# series calculates (areas, U-values, volume, air change), so every bar and
# readout below is a product of drawn quantities.
SURFACES = [
    ("Außenwand", 209.0, 0.24, 20.0, LOSS_BLUE),
    ("Dach", 89.0, 0.20, 20.0, "#A78BFA"),
    ("Fenster", 30.0, 1.10, 20.0, P_CYAN),
    ("Boden", 80.0, 0.30, 10.0, P_TEAL),
]
A_HUELLE = sum(a for _n, a, _u, _d, _c in SURFACES)
DU_WB = 0.05
V_NET, N_AIR, RHO_C, D_THETA = 440.0, 0.5, 0.34, 20.0
T_MONTH = 744.0
PHI_SURFACES = [u * a * d for _n, a, u, d, _c in SURFACES]
PHI_WB = DU_WB * A_HUELLE * D_THETA
PHI_V = V_NET * N_AIR * RHO_C * D_THETA
BAR_UNIT = 3.0 / (sum(PHI_SURFACES) + PHI_WB + PHI_V)

A_WINDOW, G_VALUE, F_S, I_S = 30.0, 0.6, 0.7, 26.0
Q_I_SPEC, A_NET, ETA_H = 5.0, 60.0, 0.8
Q_SOLAR = A_WINDOW * G_VALUE * F_S * I_S
Q_INTERN = Q_I_SPEC * A_NET * T_MONTH / 1000.0
Q_QUELLE = Q_SOLAR + Q_INTERN
Q_USABLE = ETA_H * Q_QUELLE
Q_SENKE = 840.0
D_THETA_JUL, I_S_JUL, ETA_K = 6.0, 54.0, 0.9
Q_SENKE_JUL = Q_SENKE * D_THETA_JUL / D_THETA
Q_QUELLE_JUL = A_WINDOW * G_VALUE * F_S * I_S_JUL + Q_INTERN
#endregion


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as every Heating/Cooling beat
    (``Cooling/3_transmission_humidity/scene_3.py``) — a dim footnote that
    never competes with the diagram.
    """
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs
# The through-line house: the Physical Fundamentals section every Heating and
# Cooling video draws, now carrying the four balance flows across one dashed
# envelope. Every beat of this chapter builds on the same geometry.

def _pts(*coords):
    """📍 Points on the z = 0 plane."""
    return [np.array([x, y, 0.0]) for x, y in coords]


def _home(center, scale: float = HOUSE_SCALE):
    """🏠 Section house plus the geometry every flow, person and label hangs on."""
    center = np.array(center, dtype=float)
    hs = house_section(center, scale)
    bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
    eave_l, eave_r = tl + LEFT * 0.26 * scale, tr + RIGHT * 0.26 * scale
    inner_peak = peak + DOWN * 0.12
    air = Polygon(
        bl + RIGHT * 0.08 + UP * 0.02, br + LEFT * 0.08 + UP * 0.02, tr + LEFT * 0.08 + DOWN * 0.06,
        inner_peak, tl + RIGHT * 0.08 + DOWN * 0.06,
        stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.0,
    ).set_z_index(-1)
    return {
        "hs": hs, "group": hs["group"], "air": air, "scale": scale, "cx": float(center[0]),
        "x_l": float(bl[0]), "x_r": float(br[0]), "floor_y": float(bl[1]),
        "mid_y": float(hs["level_1"].get_center()[1]), "top_y": float(tl[1]),
        "peak": peak, "eave_l": eave_l, "eave_r": eave_r,
        "lo": hs["windows"][0], "up": hs["windows"][1],
    }


def _roof(H, side: str, u: float):
    """📐 Point on a roof slope (``u`` runs eave → ridge) and the unit normal pointing out of the house."""
    eave = H["eave_l"] if side == "l" else H["eave_r"]
    p = eave + (H["peak"] - eave) * u
    d = (H["peak"] - eave) if side == "l" else (eave - H["peak"])
    d = d / np.linalg.norm(d)
    return p, np.array([-d[1], d[0], 0.0])


def _envelope(H, off: float = 0.26, color=P_TEAL):
    """⭕ Dashed thermal envelope hugging the house — the boundary every balance flow crosses."""
    nl, nr = _roof(H, "l", 0.0)[1], _roof(H, "r", 0.0)[1]
    bottom = H["floor_y"] - 0.22
    corners = [
        np.array([H["x_l"] - off, bottom, 0.0]), np.array([H["x_l"] - off, H["top_y"], 0.0]),
        H["eave_l"] + nl * off, H["peak"] + UP * off * 1.15, H["eave_r"] + nr * off,
        np.array([H["x_r"] + off, H["top_y"], 0.0]), np.array([H["x_r"] + off, bottom, 0.0]),
    ]
    return DashedVMobject(Polygon(*corners, color=color, stroke_width=2.6), num_dashes=70, dashed_ratio=0.55)


def _stand(H, x: float, color=P_ORANGE):
    """🧍 Occupant at true scale — 70 % of the storey height, feet on the slab's top edge at ``x``."""
    storey = H["mid_y"] - H["floor_y"]
    height = 0.7 * (storey - 0.04)
    return person_glyph(np.array([x, H["floor_y"] + 0.02 + height / 2, 0.0]), color=color, scale=height / 0.66)


def _gains(H):
    """💡 Occupant, pendant lamp, desk and laptop — the internal sources inside the house."""
    cx, fy, my = H["cx"], H["floor_y"], H["mid_y"]
    person = _stand(H, cx + 0.05)
    lamp = lamp_glyph(np.array([cx - 0.4, my, 0.0]), drop=0.1)
    desk_y = my + 0.3
    desk = VGroup(
        Line(np.array([cx + 0.6, desk_y, 0.0]), np.array([cx + 1.4, desk_y, 0.0]), color=P_WHITE, stroke_width=1.5),
        Line(np.array([cx + 0.7, desk_y, 0.0]), np.array([cx + 0.7, my, 0.0]), color=P_WHITE, stroke_width=1.5),
        Line(np.array([cx + 1.3, desk_y, 0.0]), np.array([cx + 1.3, my, 0.0]), color=P_WHITE, stroke_width=1.5),
    )
    laptop = VGroup(
        Line(np.array([cx + 0.85, desk_y + 0.02, 0.0]), np.array([cx + 1.13, desk_y + 0.02, 0.0]), color=P_CYAN, stroke_width=1.8),
        Line(np.array([cx + 1.13, desk_y + 0.02, 0.0]), np.array([cx + 1.08, desk_y + 0.22, 0.0]), color=P_CYAN, stroke_width=1.8),
    )
    return {
        "person": person, "lamp": lamp["group"], "device": VGroup(desk, laptop),
        "sources": VGroup(person, lamp["group"], desk, laptop),
        "warm": [person.get_center() + UP * 0.08, laptop[1].get_center()],
        "bulbs": [lamp["bulb"].get_center()],
    }


def _gain_heat(G, rt: float, *, color=INT_ORANGE, r_max: float = 0.42):
    """🌡️ Long-wave heat from every internal source — ripples from person and laptop, down from the lamp."""
    cycles = max(1.0, rt / 1.3)
    return [ripples(G["warm"], r_max=r_max, color=color, cycles=cycles),
            ripples(G["bulbs"], r_max=r_max * 0.95, color=color, down=True, cycles=cycles)]


def _paths(H, reach: float = 1.15):
    """🧭 Guide curves of the balance flows — each one crosses the envelope exactly where its physics does."""
    xl, xr, fy, my = H["x_l"], H["x_r"], H["floor_y"], H["mid_y"]
    lo_y, up_y = H["lo"]["center"][1], H["up"]["center"][1]
    walls = [smooth_path(_pts((xr - 0.6, y - 0.04), (xr, y), (xr + reach * 0.5, y + 0.04), (xr + reach, y + 0.1)))
             for y in (my - 0.2, my + 0.68)]
    roofs = []
    for u in (0.3, 0.72):
        p, n = _roof(H, "r", u)
        roofs.append(smooth_path([p - n * 0.6, p - n * 0.1, p + n * 0.4, p + n * 0.8]))
    pl, nl = _roof(H, "l", 0.55)
    return {
        "trans": walls + roofs, "wall": walls, "roof": roofs,
        "win": [smooth_path(_pts((xl + 0.55, y), (xl, y + 0.02), (xl - reach * 0.5, y - 0.02), (xl - reach, y + 0.06)))
                for y in (lo_y, up_y)],
        "floor": [smooth_path(_pts((x, fy + 0.4), (x, fy + 0.05), (x + 0.04, fy - 0.18), (x, fy - 0.38)))
                  for x in (H["cx"] - 0.5, H["cx"] + 0.5)],
        "vent_in": [smooth_path(_pts((xl - reach, lo_y - 0.1), (xl - reach * 0.55, lo_y - 0.03), (xl, lo_y),
                                     (xl + 0.5, lo_y - 0.12), (xl + 0.9, lo_y - 0.4)))],
        "vent_out": [smooth_path([pl - nl * 0.85, pl - nl * 0.2, pl + nl * 0.4, pl + nl * 0.8])],
        "supply": [smooth_path(_pts((xr + reach, fy + 0.2), (xr + reach * 0.5, fy + 0.2), (xr, fy + 0.2), (xr - 0.2, fy + 0.2)))],
    }


def _stream(paths, color, rt: float, *, color_end=None, period: float = 1.5, waves: int = 3):
    """💨 One coloured particle stream for ``rt`` seconds."""
    return flow_animation([(list(paths), color, color_end)], waves=waves, radius=0.06, cycles=max(1.0, rt / period))


def _solar(H, sun_c, windows=("up",)):
    """☀️ Sun plus parallel straight rays through the chosen windows onto the floor behind each."""
    lands = {"up": H["mid_y"], "lo": H["floor_y"]}
    sets = [sun_rays(sun_c, H[k]["x"], [H[k]["center"][1] + 0.09, H[k]["center"][1] - 0.09], lands[k], gap=0.5)
            for k in windows]
    return {"sun": sun_glyph(sun_c).scale(0.42), "sets": sets, "group": VGroup(*[r["group"] for r in sets]),
            "paths": [[s, h, p] for r in sets for s, h, p in zip(r["starts"], r["hits"], r["lands"])]}


def _shine(S, lag: float = 0.25):
    """🔦 Draw every ray of the sun, outside the glass first, then on through it."""
    return AnimationGroup(*[shine(r, lag=lag) for r in S["sets"]])


def _sunshine(S, rt: float):
    """⚡ Light pulses running down the rays while a subtitle is read."""
    return pulse_flashes(S["paths"], SOLAR_YELLOW, repeats=max(1, int(rt / 1.5)))


def _overhangs(H):
    """🪟 Exterior shading slats above each window — the outer end lower than the wall end."""
    return VGroup(*[Line(np.array([w["x"], w["lintel"] + 0.05, 0.0]), np.array([w["x"] - 0.42, w["lintel"] - 0.03, 0.0]),
                         color=P_WHITE, stroke_width=3.2) for w in (H["lo"], H["up"])])


def _tag(name: str, sym: str, color):
    """🏷️ Flow label — a short word plus the balance symbol that later lands in the formula."""
    word = body_text(name, font_size=LABEL_FONT_SIZE, color=color)
    symbol = math_label(sym, size=BODY_FONT_SIZE, color=color)
    return VGroup(word, symbol).arrange(RIGHT, buff=0.14)


def _sym(sym: str, color):
    """🔣 Bare balance symbol that travels from its flow into the formula."""
    return math_label(sym, size=BODY_FONT_SIZE, color=color)


def _lead(tag, point, *, side: str, color):
    """➖ Thin leader from a tag's inner edge to the physical element it names."""
    start = tag.get_right() + RIGHT * 0.08 if side == "left" else tag.get_left() + LEFT * 0.08
    return Line(start, np.array(point, dtype=float), color=color, stroke_width=1.4, stroke_opacity=0.55)


def _merge(sources, target):
    """🔀 Morph several flow symbols into one formula term — the first becomes it, the rest fold into it and are returned to remove."""
    anims = [ReplacementTransform(sources[0], target)]
    anims += [src.animate.move_to(target).scale(0.4).set_opacity(0) for src in sources[1:]]
    return anims, list(sources[1:])


def _surfaces(H):
    """🧱 Heavy colour overlays on the four envelope surfaces — the areas that become U · A."""
    hs = H["hs"]

    def thick(mob, color):
        return mob.copy().set_color(color).set_stroke(width=6.5, opacity=0.95)

    colors = {n: c for n, _a, _u, _d, c in SURFACES}
    return {
        "Außenwand": VGroup(*[thick(hs["walls"][i], colors["Außenwand"]) for i in (0, 1, 2, 3)]),
        "Dach": thick(hs["roof"][0], colors["Dach"]),
        "Fenster": VGroup(*[Line(np.array([w["x"], w["sill"], 0.0]), np.array([w["x"], w["lintel"], 0.0]),
                                 color=colors["Fenster"], stroke_width=7.5) for w in hs["windows"]]),
        "Boden": thick(hs["floor"], colors["Boden"]),
    }


def _card(title: str, color, specs, *, x: float = 4.5, top: float = 1.85, width: float = 2.3, gap: float = 0.44):
    """🪪 Factor card — a framed title over live readouts that count up from the physical picture."""
    frame = RoundedRectangle(
        width=width, height=0.66 + gap * len(specs), corner_radius=0.1, color=color, stroke_width=1.8,
        fill_color=P_DEEP_DARK, fill_opacity=0.85,
    )
    frame.move_to(np.array([x + width / 2, top - frame.height / 2, 0.0])).set_z_index(-2)
    head_at = np.array([x + width / 2, top - 0.3, 0.0])
    rows = [math_readout(fn, np.array([x + 0.18, top - 0.62 - gap * (k + 0.55), 0.0]), size=LABEL_FONT_SIZE,
                         color=c, edge="left") for k, (fn, c) in enumerate(specs)]
    return {"frame": frame, "head_at": head_at, "rows": rows}


def _gauge(sym: str, color, at, tracker, *, length: float = 1.3, digits: int = 2):
    """📏 Horizontal 0–1 factor gauge — a typeset symbol, a track that fills with the tracker and its live value."""
    g = meter("x", length=length, thickness=0.2, color=color, vertical=False)
    g["track"].move_to(np.array(at, dtype=float))
    set_meter(g, 0.0)
    bind_meter(g, tracker)
    label = math_label(sym, size=BODY_FONT_SIZE, color=color).next_to(g["track"], LEFT, buff=0.14)
    read = math_readout(lambda: de_num(tracker.get_value(), digits),
                        np.array([g["track"].get_right()[0] + 0.14, float(at[1]) - 0.08, 0.0]),
                        size=LABEL_FONT_SIZE + 1, color=color, edge="left")
    return {"track": g["track"], "fill": g["fill"], "label": label, "read": read,
            "static": VGroup(g["track"], g["fill"], label)}


def _thermo_pair(x: float, bottom: float, *, height: float = 1.5, gap: float = 0.8):
    """🌡️ Indoor and outdoor thermometer — the Δθ span is the gap between their two column tops, read live."""
    t_in = thermometer_glyph(np.array([x, bottom, 0.0]), height=height, color=INT_ORANGE, level=0.0)
    t_out = thermometer_glyph(np.array([x + gap, bottom, 0.0]), height=height, color=LOSS_BLUE, level=0.0)
    names = VGroup(body_text("innen", font_size=LABEL_FONT_SIZE, color=INT_ORANGE),
                   body_text("außen", font_size=LABEL_FONT_SIZE, color=LOSS_BLUE))
    names[0].move_to(np.array([x, bottom - 0.33, 0.0]))
    names[1].move_to(np.array([x + gap, bottom - 0.33, 0.0]))
    span = height - 0.12

    def top(t):
        return bottom + 0.05 + span * float(np.clip(t["level"].get_value(), 0.0, 1.0))

    def d_theta():
        return 40.0 * (t_in["level"].get_value() - t_out["level"].get_value())

    arrow = always_redraw(lambda: dim_arrow(
        np.array([x + gap / 2, top(t_out), 0.0]), np.array([x + gap / 2, top(t_out) + max(0.06, top(t_in) - top(t_out)), 0.0]),
        color=P_BLUE))
    return {"in": t_in, "out": t_out, "names": names, "arrow": arrow, "d_theta": d_theta, "x_mid": x + gap / 2,
            "static": VGroup(t_in["group"], t_out["group"], names), "top": top, "bottom": bottom}
#endregion


#region Beat 1 – Bilanzgrenze & Energieerhaltung

class Beat1_Bilanzgrenze(Scene):
    NARRATION = [
        ("intro",
         "In three videos we calculated single heat flows. Now we assemble them into one balance.",
         "In drei Videos haben wir einzelne Wärmeströme berechnet. Jetzt fügen wir sie zu einer Bilanz zusammen."),
        ("huelle",
         "A balance needs a boundary: the thermal envelope. Everything that crosses it over the year is an energy flow we can add up.",
         "Eine Bilanz braucht eine Grenze: die thermische Hülle. Alles, was sie übers Jahr kreuzt, ist ein Energiefluss, den wir aufsummieren."),
        ("erhaltung",
         "From the fundamentals we know energy is conserved — so at a constant indoor temperature, everything leaving must be replaced.",
         "Aus den Grundlagen wissen wir: Energie bleibt erhalten. Bei konstanter Innentemperatur muss alles Abfließende ersetzt werden."),
        ("bedarf",
         "What the free gains don't cover, the heating has to deliver. That remainder is the useful heating demand — Heizwärmebedarf.",
         "Was die freien Gewinne nicht decken, muss die Heizung liefern. Dieser Rest ist der Heizwärmebedarf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Bilanzgrenze & Energieerhaltung", title)
        din = _din_ref("DIN V 18599-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        H = _home(np.array([0.3, 0.0, 0.0]), 0.9)
        G = _gains(H)
        P = _paths(H)
        S = _solar(H, np.array([-4.3, 1.3, 0.0]))
        self.add(H["air"])
        self.play(Create(H["group"]), run_time=1.4)
        self.play(LaggedStart(*[FadeIn(m) for m in G["sources"]], lag_ratio=0.2),
                  H["air"].animate.set_fill(P_ORANGE, opacity=0.07), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 1.0,
                 during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "huelle"))
        envelope = _envelope(H)
        self.play(Create(envelope), *_gain_heat(G, 1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "huelle", during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "erhaltung"))
        trans_g = flow_guides(P["trans"], LOSS_BLUE, opacity=0.2)
        vent_g = flow_guides(P["vent_in"] + P["vent_out"], VENT_BLUE, opacity=0.2)
        tag_t = _tag("Transmission", r"Q_{T}", LOSS_BLUE)
        tag_v = _tag("Lüftung", r"Q_{V}", VENT_BLUE)
        tag_s = _tag("Solare Gewinne", r"Q_{S}", SOLAR_YELLOW)
        tag_i = _tag("Interne Gewinne", r"Q_{I}", INT_ORANGE)
        tag_t.move_to(np.array([4.6, 0.55, 0.0]))
        tag_v.move_to(np.array([-4.45, -0.95, 0.0]))
        tag_s.move_to(S["sun"].get_center() + np.array([0.0, -0.75, 0.0]))
        tag_i.move_to(np.array([4.5, -0.5, 0.0]))
        leads = VGroup(
            _lead(tag_t, P["trans"][0].point_from_proportion(0.85), side="right", color=LOSS_BLUE),
            _lead(tag_v, P["vent_in"][0].point_from_proportion(0.1), side="left", color=VENT_BLUE),
            _lead(tag_i, G["person"].get_center() + RIGHT * 0.12 + UP * 0.2, side="right", color=INT_ORANGE),
        )

        def flows(rt, kinds=("trans", "vent", "sun", "int")):
            out = []
            if "trans" in kinds:
                out.append(_stream(P["trans"], LOSS_BLUE, rt, color_end=LOSS_BLUE))
            if "vent" in kinds:
                out += [_stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE),
                        _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE)]
            if "sun" in kinds:
                out.append(_sunshine(S, rt))
            if "int" in kinds:
                out += _gain_heat(G, rt)
            return out

        self.play(FadeIn(S["sun"], scale=0.7), _shine(S), FadeIn(tag_s), run_time=1.3)
        self.play(Create(trans_g), FadeIn(tag_t), Create(leads[0]), *flows(1.5, ("trans", "sun", "int")), run_time=1.5)
        self.play(Create(vent_g), FadeIn(tag_v), Create(leads[1]), open_window(H["lo"], run_time=0.8),
                  *flows(1.5, ("trans", "vent", "sun", "int")), run_time=1.5)
        self.play(FadeIn(tag_i), Create(leads[2]), *flows(1.5), run_time=1.5)
        hold_for(self, self.NARRATION, "erhaltung", during=flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf"))
        rad = radiator(np.array([H["x_r"] - 0.6, H["floor_y"] + 0.28, 0.0]), color=DEMAND_RED).scale(0.9)
        tag_h = _tag("Heizwärmebedarf", r"Q_{h}", DEMAND_RED)
        tag_h.move_to(np.array([4.95, H["floor_y"] + 0.2, 0.0]))
        lead_h = _lead(tag_h, P["supply"][0].point_from_proportion(0.05), side="right", color=DEMAND_RED)
        heat_g = flow_guides(P["supply"], DEMAND_RED, opacity=0.3)
        rad_spots = [rad.get_top() + UP * 0.02]

        def heating(rt):
            return [_stream(P["supply"], DEMAND_RED, rt, period=1.2),
                    ripples(rad_spots, r_max=0.5, color=DEMAND_RED, cycles=max(1.0, rt / 1.3))]

        row, box, items = math_panel([
            ("qh", r"Q_{h}", DEMAND_RED), (None, "+", P_WHITE),
            ("quelle", r"Q_{\mathrm{Quelle}}", SOLAR_YELLOW), (None, "=", P_WHITE),
            ("senke", r"Q_{\mathrm{Senke}}", LOSS_BLUE), (None, r"\;[\mathrm{kWh/a}]", P_TEAL),
        ])
        glue = [m for m in row if all(m is not v for v in items.values())]
        self.play(FadeIn(rad), Create(heat_g), FadeIn(tag_h), Create(lead_h), *flows(1.3), *heating(1.3), run_time=1.3)
        sink_anims, sink_spare = _merge([tag_t[1], tag_v[1]], items["senke"])
        src_anims, src_spare = _merge([tag_s[1], tag_i[1]], items["quelle"])
        self.play(
            Create(box), *[FadeIn(g) for g in glue],
            *sink_anims, *src_anims, ReplacementTransform(tag_h[1], items["qh"]),
            FadeOut(VGroup(tag_t[0], tag_v[0], tag_s[0], tag_i[0], tag_h[0], leads, lead_h)),
            *flows(1.4), *heating(1.4), run_time=1.4,
        )
        self.remove(*sink_spare, *src_spare)
        ring = highlight_param(items, "qh", color=DEMAND_RED)
        self.play(Create(ring), *flows(0.6), *heating(0.6), run_time=0.6)
        hold_for(self, self.NARRATION, "bedarf",
                 during=lambda rt: [*flows(rt), *heating(rt)])

        self.play(FadeOut(VGroup(caption, ring, box, row, trans_g, vent_g, heat_g, envelope, rad,
                                 S["sun"], S["group"], G["sources"], H["group"], H["air"])),
                  run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 2 – Die Verlustseite (Q_T + Q_V)

class Beat2_Verlustseite(Scene):
    NARRATION = [
        ("intro",
         "The loss side first — and it is exactly the two mechanisms the Heating video derived.",
         "Zuerst die Verlustseite — genau die beiden Mechanismen aus dem Heizen-Video."),
        ("transmission",
         "Transmission through every envelope surface: U times area times the mean temperature difference, summed over the hours of the month — a schematic form of H times degree-hours.",
         "Transmission durch jede Hüllfläche: U mal Fläche mal mittlere Temperaturdifferenz, über die Stunden des Monats — schematisch wie H mal Gradstunden."),
        ("bruecken",
         "Thermal bridges are added as a lump surcharge on the U-values — 0.05 watts per square metre kelvin when a thermal-bridge check is available, more without one.",
         "Wärmebrücken kommen als Pauschalzuschlag auf die U-Werte — 0,05 W/(m²·K) mit Wärmebrückennachweis, sonst höher."),
        ("lueftung",
         "Ventilation carries the second stream out: the air change rate replaces warm indoor air with cold outdoor air.",
         "Die Lüftung trägt den zweiten Strom hinaus: Der Luftwechsel ersetzt warme Innenluft durch kalte Außenluft."),
        ("summe",
         "Together they form the sink side of the balance — every kilowatt-hour the building loses over the year.",
         "Zusammen bilden sie die Senkenseite der Bilanz — jede Kilowattstunde, die das Gebäude im Jahr verliert."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Verlustseite: Transmission & Lüftung", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        H = _home(np.array([-3.7, 0.0, 0.0]), 0.9)
        P = _paths(H, reach=0.8)
        hl = _surfaces(H)
        envelope = _envelope(H)
        by_surface = {"Außenwand": P["wall"], "Dach": P["roof"], "Fenster": P["win"], "Boden": P["floor"]}

        segments = [(n, phi, c) for (n, _a, _u, _d, c), phi in zip(SURFACES, PHI_SURFACES)]
        segments += [("Wärmebrücken", PHI_WB, P_ORANGE), ("Lüftung", PHI_V, VENT_BLUE)]
        stack = stacked_bar(segments, base=np.array([1.35, -1.2, 0.0]), width=1.0, unit=BAR_UNIT)
        labels, leaders = side_labels(
            [(a, name, col) for a, (name, _v, col) in zip(stack["anchors"], segments)],
            x=2.3, align="left", min_gap=0.28, pad=0.06,
        )

        def loss_flows(rt):
            return [_stream(P["trans"], LOSS_BLUE, rt),
                    _stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE),
                    _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE)]

        ledger = DashedVMobject(Rectangle(width=1.0, height=3.0, color=P_WHITE, stroke_width=1.6, stroke_opacity=0.5)
                                .move_to(np.array([1.35, -1.2 + 1.5, 0.0])), num_dashes=40)
        sym_t = _sym(r"Q_{T}", LOSS_BLUE).next_to(P["wall"][0].get_end(), RIGHT, buff=0.12)
        sym_v = _sym(r"Q_{V}", VENT_BLUE).next_to(P["vent_in"][0].get_start(), LEFT, buff=0.1)

        self.add(H["air"])
        self.play(Create(H["group"]), run_time=1.2)
        self.play(Create(envelope), H["air"].animate.set_fill(P_ORANGE, opacity=0.07),
                  open_window(H["lo"], run_time=0.8), FadeIn(sym_t), FadeIn(sym_v), FadeIn(ledger), *loss_flows(1.6),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 1.6, during=loss_flows)

        # —— Transmission: every surface is one U · A block of the loss column ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "transmission"))
        row_t, box_t, it_t = math_panel([
            ("qt", r"Q_{T}", LOSS_BLUE), (None, "=", P_WHITE),
            ("ua", r"\Sigma\,(U\cdot A)", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_BLUE), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_TEAL), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])
        u_t, a_t, d_t = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        card = _card("", P_ORANGE, [
            (lambda: rf"U = {de_num(u_t.get_value(), 2)}\,\mathrm{{W/(m^{{2}}K)}}", P_ORANGE),
            (lambda: rf"A = {de_num(a_t.get_value())}\,\mathrm{{m^{{2}}}}", P_CYAN),
            (lambda: rf"\Delta\theta = {de_num(d_t.get_value())}\,\mathrm{{K}}", P_BLUE),
            (lambda: rf"\Phi = {de_num(u_t.get_value() * a_t.get_value() * d_t.get_value())}\,\mathrm{{W}}", P_WHITE),
        ])
        heads = [body_text(n, font_size=LABEL_FONT_SIZE, color=c).move_to(card["head_at"]) for n, _a, _u, _d, c in SURFACES]
        ring_ua = highlight_param(it_t, "ua", color=P_ORANGE)
        self.add(*card["rows"])
        self.play(Create(box_t), FadeIn(row_t), FadeIn(card["frame"]), Create(ring_ua), run_time=0.7)
        for k, (name, area, u, d_k, color) in enumerate(SURFACES):
            u_t.set_value(0.0)
            a_t.set_value(0.0)
            d_t.set_value(0.0)
            head = FadeIn(heads[0]) if k == 0 else ReplacementTransform(heads[k - 1], heads[k])
            self.play(Create(hl[name]), head, u_t.animate.set_value(u), a_t.animate.set_value(area),
                      d_t.animate.set_value(d_k), _stream(by_surface[name], color, 0.9), run_time=0.9)
            self.play(TransformFromCopy(hl[name], stack["bars"][k]), FadeIn(labels[k]), Create(leaders[k]),
                      _stream(by_surface[name], color, 0.7), run_time=0.7)
        th = _thermo_pair(-0.5, -1.0, height=1.5, gap=0.85)
        dth_read = math_readout(lambda: rf"\Delta\theta = {de_num(th['d_theta']())}\,\mathrm{{K}}",
                                np.array([th["x_mid"], 0.82, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE, edge="center")
        ring_dt = highlight_param(it_t, "dt", color=P_BLUE)
        self.remove(*card["rows"])
        self.play(FadeOut(VGroup(card["frame"], heads[-1])), FadeOut(ring_ua), FadeIn(th["static"]), Create(ring_dt),
                  run_time=0.4)
        self.add(th["in"]["column"], th["out"]["column"], dth_read)
        self.play(th["in"]["level"].animate.set_value(0.75), th["out"]["level"].animate.set_value(0.25), run_time=1.0)
        self.add(th["arrow"])
        clk = clock_glyph(np.array([5.65, 1.3, 0.0]), r=0.42, color=P_TEAL)
        t_tr = ValueTracker(0.0)
        t_card = _card("Stunden im Monat", P_TEAL, [
            (lambda: rf"t = {de_num(t_tr.get_value())}\,\mathrm{{h}}", P_TEAL)], top=0.62)
        t_head = body_text("Stunden im Monat", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(t_card["head_at"])
        ring_t = highlight_param(it_t, "t", color=P_TEAL)
        self.add(*t_card["rows"])
        self.play(FadeOut(ring_dt), Create(ring_t), FadeIn(clk["group"]), FadeIn(VGroup(t_card["frame"], t_head)),
                  run_time=0.4)
        self.play(Rotate(clk["hand"], angle=-TAU * 3, about_point=clk["center"], rate_func=linear),
                  t_tr.animate.set_value(T_MONTH), run_time=1.2)
        hold_for(self, self.NARRATION, "transmission", during=lambda rt: [
            _stream(P["trans"], LOSS_BLUE, rt), _stream(P["win"], P_CYAN, rt), _stream(P["floor"], P_TEAL, rt),
            Rotate(clk["hand"], angle=-TAU * rt / 0.5, about_point=clk["center"], rate_func=linear)])

        # —— Thermal bridges: a lump surcharge on every U ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bruecken"))
        xl, xr = H["x_l"], H["x_r"]
        spots = [H["hs"]["bottom_left"], H["hs"]["bottom_right"], H["hs"]["top_left"], H["hs"]["top_right"],
                 np.array([xl, H["mid_y"], 0.0]), np.array([xr, H["mid_y"], 0.0]), H["peak"],
                 *[np.array([w["x"], y, 0.0]) for w in H["hs"]["windows"] for y in (w["sill"], w["lintel"])]]
        dots = VGroup(*[Dot(np.array(p, dtype=float), radius=0.075, color=P_ORANGE) for p in spots]).set_z_index(5)
        du_t = ValueTracker(0.0)
        du_g = meter("Wärmebrücken", length=2.2, thickness=0.26, color=P_ORANGE, vertical=False)
        du_g["group"].move_to(np.array([5.65, 1.2, 0.0]))
        bind_meter(du_g, du_t)
        du_read = math_readout(lambda: rf"\Delta U_{{WB}} = {de_num(du_t.get_value() * 0.15, 2)}",
                               np.array([5.65, 0.52, 0.0]), size=LABEL_FONT_SIZE + 1, color=P_ORANGE, edge="center")
        du_unit = math_label(r"\mathrm{W/(m^{2}K)}", np.array([5.65, 0.16, 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        row_t2, box_t2, it_t2 = math_panel([
            ("qt", r"Q_{T}", LOSS_BLUE), (None, "=", P_WHITE),
            ("ua", r"\Sigma\,((U+\Delta U_{WB})\cdot A)", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_BLUE), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_TEAL), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])
        self.remove(*t_card["rows"])
        self.play(FadeOut(VGroup(clk["group"], t_card["frame"], t_head, ring_t)), LaggedStart(*[GrowFromCenter(d) for d in dots],
                  lag_ratio=0.08), run_time=1.0)
        self.add(du_read)
        self.play(FadeIn(du_g["group"]), FadeIn(du_unit), du_t.animate.set_value(1 / 3), run_time=0.9)
        self.play(ReplacementTransform(dots, stack["bars"][4]), FadeIn(labels[4]), Create(leaders[4]),
                  ReplacementTransform(box_t, box_t2), ReplacementTransform(row_t, row_t2), run_time=1.2)
        hold_for(self, self.NARRATION, "bruecken", during=lambda rt: [
            ripples([np.array(p, dtype=float) for p in spots[:7]], r_max=0.3, color=P_ORANGE, cycles=max(1.0, rt / 1.4)),
            _stream(P["trans"], LOSS_BLUE, rt)])

        # —— Ventilation: the interior volume becomes the second block ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lueftung"))
        n_t, v_t, rc_t = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        n_g = meter("Luftwechsel", length=2.2, thickness=0.26, color=VENT_BLUE, vertical=False)
        n_g["group"].move_to(np.array([5.65, 1.72, 0.0]))
        bind_meter(n_g, n_t)
        v_card = _card("", VENT_BLUE, [
            (lambda: rf"n = {de_num(n_t.get_value(), 1)}\,\mathrm{{h^{{-1}}}}", P_YELLOW),
            (lambda: rf"V = {de_num(v_t.get_value())}\,\mathrm{{m^{{3}}}}", P_YELLOW),
            (lambda: rf"\rho c = {de_num(rc_t.get_value(), 2)}\,\mathrm{{Wh/m^{{3}}K}}", P_GREEN),
            (lambda: rf"\Phi_{{V}} = {de_num(rc_t.get_value() * n_t.get_value() * v_t.get_value() * D_THETA)}\,\mathrm{{W}}", P_WHITE),
        ], top=1.18)
        vol = H["air"].copy().set_fill(VENT_BLUE, opacity=0.3).set_z_index(0)
        row_v, box_v, it_v = math_panel([
            ("qv", r"Q_{V}", VENT_BLUE), (None, "=", P_WHITE),
            ("rc", r"\rho\,c", P_GREEN), (None, r"\cdot", P_WHITE),
            ("nv", r"n\cdot V", P_YELLOW), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_BLUE), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_TEAL), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])
        ring_nv = highlight_param(it_v, "nv", color=P_YELLOW)
        self.remove(du_read)
        self.play(FadeOut(VGroup(du_g["group"], du_unit)), run_time=0.3)
        self.add(*v_card["rows"])
        self.play(FadeIn(vol), FadeIn(n_g["group"]), FadeIn(v_card["frame"]),
                  n_t.animate.set_value(N_AIR), v_t.animate.set_value(V_NET), rc_t.animate.set_value(RHO_C),
                  ReplacementTransform(box_t2, box_v), ReplacementTransform(row_t2, row_v), Create(ring_nv),
                  *loss_flows(1.5), run_time=1.5)
        self.play(TransformFromCopy(vol, stack["bars"][5]), FadeIn(labels[5]), Create(leaders[5]),
                  *loss_flows(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "lueftung", during=loss_flows)

        # —— The sink side: both blocks form Q_Senke ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summe"))
        brace_t = Brace(VGroup(*stack["bars"][:5]), LEFT, color=LOSS_BLUE, buff=0.12)
        brace_v = Brace(stack["bars"][5], LEFT, color=VENT_BLUE, buff=0.12)
        brace_all = Brace(stack["bars"], LEFT, color=LOSS_BLUE, buff=0.95)
        lab_t = _sym(r"Q_{T}", LOSS_BLUE).next_to(brace_t, LEFT, buff=0.1)
        lab_v = _sym(r"Q_{V}", VENT_BLUE).next_to(brace_v, LEFT, buff=0.1)
        lab_all = _sym(r"Q_{\mathrm{Senke}}", LOSS_BLUE).next_to(brace_all, LEFT, buff=0.1)
        row_s, box_s, it_s = math_panel([
            ("senke", r"Q_{\mathrm{Senke}}", LOSS_BLUE), (None, "=", P_WHITE),
            ("qt", r"Q_{T}", LOSS_BLUE), (None, "+", P_WHITE),
            ("qv", r"Q_{V}", VENT_BLUE), (None, r"\;[\mathrm{kWh/a}]", P_TEAL),
        ])
        self.remove(*v_card["rows"])
        self.play(FadeOut(VGroup(th["static"], vol, v_card["frame"], n_g["group"], ring_nv, dth_read, th["arrow"],
                                 sym_t, sym_v)),
                  GrowFromCenter(brace_t), GrowFromCenter(brace_v), FadeIn(lab_t), FadeIn(lab_v), run_time=0.8)
        self.remove(th["in"]["column"], th["out"]["column"])
        glue = [m for m in row_s if all(m is not v for v in it_s.values())]
        self.play(GrowFromCenter(brace_all), FadeIn(lab_all), run_time=0.6)
        self.play(ReplacementTransform(box_v, box_s), FadeOut(row_v), *[FadeIn(g) for g in glue],
                  TransformFromCopy(lab_all, it_s["senke"]), TransformFromCopy(lab_t, it_s["qt"]),
                  TransformFromCopy(lab_v, it_s["qv"]), run_time=1.2)
        hold_for(self, self.NARRATION, "summe", during=loss_flows)

        self.play(FadeOut(VGroup(caption, box_s, row_s, brace_t, brace_v, brace_all, lab_t, lab_v, lab_all, stack["bars"], labels,
                                 leaders, envelope, ledger, H["group"], H["air"])), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 3 – Die Gewinnseite & der Ausnutzungsgrad

class Beat3_Gewinnseite(Scene):
    NARRATION = [
        ("intro",
         "Now the other pan of the scale: the heat the building gets for free.",
         "Jetzt die andere Waagschale: die Wärme, die das Gebäude kostenlos bekommt."),
        ("solar",
         "Solar gains through the glazing — window area times the g-value times the irradiation on that facade.",
         "Solare Gewinne durch die Verglasung — Fensterfläche mal g-Wert mal Einstrahlung auf diese Fassade."),
        ("intern",
         "Internal gains from people, appliances and lighting: DIN V 18599-10 tables them by use, here as a watt per square metre.",
         "Interne Gewinne von Personen, Geräten und Licht: DIN V 18599-10 tabelliert sie nutzungsabhängig, hier als Wert je Quadratmeter."),
        ("eta",
         "But not every free kilowatt-hour helps. The utilization factor eta_h depends on the gain-to-loss ratio — here a schematic 0.8. Gains that arrive when no heat is needed only overheat the room.",
         "Doch nicht jede freie Kilowattstunde hilft. Der Ausnutzungsgrad η_h hängt vom Gewinn-Verlust-Verhältnis ab — hier schematisch 0,8. Gewinne ohne Wärmebedarf überhitzen nur den Raum."),
        ("bilanz",
         "So the losses are covered by the usable gains plus whatever the heating adds. That remainder is the heating demand Q_h.",
         "Die Verluste werden also von den nutzbaren Gewinnen plus der Heizung gedeckt. Dieser Rest ist der Heizwärmebedarf Q_h."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Gewinnseite & der Ausnutzungsgrad", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        H = _home(np.array([-2.5, 0.0, 0.0]), 0.9)
        P = _paths(H, reach=0.8)
        G = _gains(H)
        S = _solar(H, np.array([-5.9, 1.35, 0.0]), ("up", "lo"))
        envelope = _envelope(H)
        slats = _overhangs(H)

        base_y, unit, width = -1.0, 2.9 / Q_SENKE, 1.0
        x_sink, x_gain = 0.95, 2.35

        def block(x, y0, value, color, opacity=0.85):
            h = value * unit
            return Rectangle(width=width, height=h, color=color, fill_color=color, fill_opacity=opacity,
                             stroke_width=1.2).move_to(np.array([x, base_y + y0 * unit + h / 2, 0.0]))

        sink = block(x_sink, 0.0, Q_SENKE, LOSS_BLUE, 0.55)
        ground = Line(np.array([x_sink - 0.75, base_y, 0.0]), np.array([x_gain + 0.75, base_y, 0.0]),
                      color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        cap_sink = math_label(r"Q_{\mathrm{Senke}}", np.array([x_sink, base_y - 0.38, 0.0]), size=BODY_FONT_SIZE, color=LOSS_BLUE)
        cap_gain = math_label(r"Q_{\mathrm{Quelle}}", np.array([x_gain, base_y - 0.38, 0.0]), size=BODY_FONT_SIZE, color=SOLAR_YELLOW)

        def loss_flows(rt):
            return [_stream(P["trans"], LOSS_BLUE, rt),
                    _stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE),
                    _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE)]

        self.add(H["air"])
        self.play(Create(H["group"]), run_time=1.2)
        self.play(Create(envelope), Create(ground), GrowFromEdge(sink, DOWN), FadeIn(cap_sink),
                  H["air"].animate.set_fill(P_ORANGE, opacity=0.07), open_window(H["lo"], run_time=0.8),
                  *loss_flows(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 1.4, during=loss_flows)

        # —— Solar gains: A_W · g · F_S · I_S, each one drawn before it is multiplied ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar"))
        g_t, fs_t, aw_t, is_t, qs_t = [ValueTracker(0.0) for _ in range(5)]
        g_g = _gauge("g", P_ORANGE, np.array([5.45, 1.85, 0.0]), g_t)
        fs_g = _gauge(r"F_{S}", P_TEAL, np.array([5.45, 1.4, 0.0]), fs_t)
        s_card = _card("", P_YELLOW, [
            (lambda: rf"A_{{W}} = {de_num(aw_t.get_value())}\,\mathrm{{m^{{2}}}}", P_CYAN),
            (lambda: rf"I_{{S}} = {de_num(is_t.get_value())}\,\mathrm{{kWh/m^{{2}}}}", P_YELLOW),
            (lambda: rf"Q_{{S}} = {de_num(qs_t.get_value())}\,\mathrm{{kWh}}", SOLAR_YELLOW),
        ], top=0.95)
        glass = VGroup(*[Line(np.array([w["x"], w["sill"], 0.0]), np.array([w["x"], w["lintel"], 0.0]),
                              color=P_CYAN, stroke_width=7.5) for w in (H["lo"], H["up"])])
        patches = VGroup(*[Rectangle(width=0.55, height=0.07, stroke_width=0, fill_color=SOLAR_YELLOW, fill_opacity=0.9)
                           .move_to(np.array([float(np.mean([p[0] for p in r["lands"]])), y + 0.04, 0.0]))
                           for r, y in zip(S["sets"], (H["mid_y"], H["floor_y"]))])
        solar_block = block(x_gain, 0.0, Q_SOLAR, SOLAR_YELLOW)
        row_s, box_s, it_s = math_panel([
            ("qs", r"Q_{S}", SOLAR_YELLOW), (None, "=", P_WHITE),
            ("a", r"A_{W}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("g", r"g", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("f", r"F_{S}", P_TEAL), (None, r"\cdot", P_WHITE),
            ("i", r"I_{S}", P_YELLOW), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])
        ring_a = highlight_param(it_s, "a", color=P_CYAN)
        ring_i = highlight_param(it_s, "i", color=P_YELLOW)
        ring_g = highlight_param(it_s, "g", color=P_ORANGE)
        ring_f = highlight_param(it_s, "f", color=P_TEAL)
        sunny = lambda rt: [_sunshine(S, rt)]
        self.add(*s_card["rows"])
        self.play(FadeIn(S["sun"], scale=0.7), Create(glass), FadeIn(s_card["frame"]), Create(box_s), FadeIn(row_s),
                  Create(ring_a), aw_t.animate.set_value(A_WINDOW), FadeIn(cap_gain), run_time=0.9)
        self.play(_shine(S), is_t.animate.set_value(I_S), ReplacementTransform(ring_a, ring_i), run_time=1.1)
        self.add(g_g["read"])
        self.play(FadeIn(g_g["static"]), g_t.animate.set_value(G_VALUE), ReplacementTransform(ring_i, ring_g),
                  *sunny(1.0), run_time=1.0)
        self.add(fs_g["read"])
        self.play(FadeIn(fs_g["static"]), Create(slats), fs_t.animate.set_value(F_S), ReplacementTransform(ring_g, ring_f),
                  *sunny(1.0), run_time=1.0)
        self.play(FadeOut(ring_f), FadeIn(patches), run_time=0.3)
        self.play(TransformFromCopy(patches, solar_block), qs_t.animate.set_value(Q_SOLAR), *sunny(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "solar", during=sunny)

        # —— Internal gains: q_I · A_N · t ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "intern"))
        q_t, an_t, t_t, qi_t = [ValueTracker(0.0) for _ in range(4)]
        i_card = _card("", INT_ORANGE, [
            (lambda: rf"q_{{I}} = {de_num(q_t.get_value())}\,\mathrm{{W/m^{{2}}}}", P_ORANGE),
            (lambda: rf"A_{{N}} = {de_num(an_t.get_value())}\,\mathrm{{m^{{2}}}}", P_CYAN),
            (lambda: rf"t = {de_num(t_t.get_value())}\,\mathrm{{h}}", P_TEAL),
            (lambda: rf"Q_{{I}} = {de_num(qi_t.get_value())}\,\mathrm{{kWh}}", INT_ORANGE),
        ], top=1.85)
        floors = VGroup(*[Line(np.array([H["x_l"] + 0.12, y + 0.04, 0.0]), np.array([H["x_r"] - 0.12, y + 0.04, 0.0]),
                               color=P_CYAN, stroke_width=6) for y in (H["floor_y"], H["mid_y"])])
        heat_pack = VGroup(*[Rectangle(width=0.5, height=0.07, stroke_width=0, fill_color=INT_ORANGE, fill_opacity=0.9)
                             .move_to(np.array([G["person"].get_center()[0], y + 0.1, 0.0])) for y in (H["floor_y"], H["mid_y"])])
        intern_block = block(x_gain, 0.0, Q_INTERN, INT_ORANGE)
        row_i, box_i, it_i = math_panel([
            ("qi", r"Q_{I}", INT_ORANGE), (None, "=", P_WHITE),
            ("q", r"q_{I}", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", r"A_{N}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_TEAL), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])
        ring_q = highlight_param(it_i, "q", color=P_ORANGE)
        ring_an = highlight_param(it_i, "a", color=P_CYAN)
        ring_t = highlight_param(it_i, "t", color=P_TEAL)
        warm = lambda rt: [*_gain_heat(G, rt), _sunshine(S, rt)]
        self.remove(*s_card["rows"], g_g["read"], fs_g["read"])
        self.play(FadeOut(VGroup(s_card["frame"], g_g["static"], fs_g["static"], glass, slats, patches, S["sun"],
                                 S["group"])),
                  ReplacementTransform(box_s, box_i), ReplacementTransform(row_s, row_i),
                  solar_block.animate.move_to(np.array([x_gain, base_y + (Q_INTERN + Q_SOLAR / 2) * unit, 0.0])),
                  LaggedStart(*[FadeIn(m) for m in G["sources"]], lag_ratio=0.15), run_time=1.0)
        self.add(*i_card["rows"])
        self.play(FadeIn(i_card["frame"]), Create(ring_q), q_t.animate.set_value(Q_I_SPEC), *_gain_heat(G, 0.9), run_time=0.9)
        self.play(ReplacementTransform(ring_q, ring_an), Create(floors), an_t.animate.set_value(A_NET), *_gain_heat(G, 0.9),
                  run_time=0.9)
        self.play(ReplacementTransform(ring_an, ring_t), t_t.animate.set_value(T_MONTH), *_gain_heat(G, 0.9), run_time=0.9)
        self.play(FadeOut(ring_t), FadeIn(heat_pack), run_time=0.3)
        self.play(TransformFromCopy(heat_pack, intern_block), qi_t.animate.set_value(Q_INTERN), *_gain_heat(G, 1.0),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "intern", during=lambda rt: _gain_heat(G, rt))

        # —— The utilisation factor splits the free heat ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "eta"))
        eta_t = ValueTracker(0.0)
        eta_g = _gauge(r"\eta_{h}", ETA_GREEN, np.array([5.45, 1.5, 0.0]), eta_t)
        usable = block(x_gain, 0.0, Q_USABLE, ETA_GREEN)
        waste = block(x_gain, Q_USABLE, Q_QUELLE - Q_USABLE, P_RED, 0.6)
        spill = DashedLine(np.array([x_gain - 0.65, base_y + Q_QUELLE * unit, 0.0]),
                           np.array([x_gain + 0.65, base_y + Q_QUELLE * unit, 0.0]), color=P_RED, stroke_width=2)
        spill_note = body_text("Überhitzung", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([3.95, base_y + (Q_USABLE + (Q_QUELLE - Q_USABLE) / 2) * unit, 0.0]))
        row_e, box_e, it_e = math_panel([
            ("eta", r"\eta_{h}", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("quelle", r"Q_{\mathrm{Quelle}}", SOLAR_YELLOW),
            (None, rf"\approx {de_num(ETA_H, 1)}\cdot {de_num(Q_QUELLE)} \approx {de_num(Q_USABLE)}\;\mathrm{{kWh}}", P_WHITE),
        ])
        ring_eta = highlight_param(it_e, "eta", color=ETA_GREEN)
        self.remove(*i_card["rows"])
        self.add(eta_g["read"])
        self.play(FadeOut(VGroup(i_card["frame"], floors, heat_pack)), FadeIn(eta_g["static"]),
                  ReplacementTransform(box_i, box_e), ReplacementTransform(row_i, row_e),
                  eta_t.animate.set_value(ETA_H), *_gain_heat(G, 1.1), run_time=1.1)
        self.play(ReplacementTransform(VGroup(intern_block, solar_block), VGroup(usable, waste)), Create(spill),
                  Create(ring_eta), FadeIn(spill_note), H["air"].animate.set_fill(P_RED, opacity=0.16),
                  *_gain_heat(G, 1.2, color=P_RED), run_time=1.2)
        hold_for(self, self.NARRATION, "eta", during=lambda rt: _gain_heat(G, rt, color=P_RED))

        # —— The balance closes: usable gains + heating = losses ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bilanz"))
        rad = radiator(np.array([H["x_r"] - 0.6, H["floor_y"] + 0.28, 0.0]), color=DEMAND_RED).scale(0.9)
        supply = _paths(H, reach=0.8)["supply"]
        demand = block(x_gain, Q_USABLE, Q_SENKE - Q_USABLE, DEMAND_RED)
        level = DashedLine(np.array([x_sink + 0.65, base_y + Q_SENKE * unit, 0.0]),
                           np.array([x_gain + 0.65, base_y + Q_SENKE * unit, 0.0]), color=P_WHITE, stroke_width=1.8,
                           stroke_opacity=0.55)
        lab_use = math_label(rf"\eta_{{h}}\cdot Q_{{\mathrm{{Quelle}}}}", np.array([3.0, base_y + Q_USABLE / 2 * unit - 0.07, 0.0]),
                             size=LABEL_FONT_SIZE + 1, color=ETA_GREEN, edge="left")
        lab_dem = math_label(r"Q_{h}", np.array([3.0, base_y + (Q_USABLE + (Q_SENKE - Q_USABLE) / 2) * unit - 0.07, 0.0]),
                             size=BODY_FONT_SIZE, color=DEMAND_RED, edge="left")
        row_b, box_b, it_b = math_panel([
            ("qh", r"Q_{h}", DEMAND_RED), (None, "=", P_WHITE),
            ("senke", r"Q_{\mathrm{Senke}}", LOSS_BLUE), (None, "−", P_WHITE),
            ("eta", r"\eta_{h}", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("quelle", r"Q_{\mathrm{Quelle}}", SOLAR_YELLOW), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])
        ring_h = highlight_param(it_b, "qh", color=DEMAND_RED)
        warm_in = lambda rt: [_stream(supply, DEMAND_RED, rt, period=1.2),
                              ripples([rad.get_top() + UP * 0.02], r_max=0.5, color=DEMAND_RED, cycles=max(1.0, rt / 1.3))]
        self.remove(eta_g["read"])
        self.play(FadeOut(VGroup(spill, spill_note, waste, eta_g["static"], ring_eta)),
                  H["air"].animate.set_fill(P_ORANGE, opacity=0.07), run_time=0.5)
        self.play(FadeIn(rad), TransformFromCopy(rad, demand), Create(level), *warm_in(1.3), run_time=1.3)
        self.play(FadeIn(lab_use), FadeIn(lab_dem), *warm_in(0.8), run_time=0.8)
        self.play(ReplacementTransform(box_e, box_b), ReplacementTransform(row_e, row_b),
                  TransformFromCopy(lab_dem, it_b["qh"]), *warm_in(1.0), run_time=1.0)
        self.play(Create(ring_h), *warm_in(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "bilanz", during=warm_in)

        self.play(FadeOut(VGroup(caption, ring_h, box_b, row_b, sink, cap_sink, cap_gain, usable, demand, lab_use, lab_dem,
                                 level, ground, rad, envelope, G["sources"], H["group"], H["air"])), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 4 – Dieselbe Bilanz im Sommer

class Beat4_WinterUndSommer(Scene):
    NARRATION = [
        ("intro",
         "The same balance runs all year — only the two pans change height.",
         "Dieselbe Bilanz läuft das ganze Jahr — nur die beiden Waagschalen ändern ihre Höhe."),
        ("winter",
         "In January losses dominate: the gain pan is small, and the gap is heating demand.",
         "Im Januar dominieren die Verluste: Die Gewinnschale ist klein, und die Lücke ist Heizwärmebedarf."),
        ("sommer",
         "In July it flips. Sun and internal loads exceed the losses, and the surplus is cooling energy demand that has to be removed.",
         "Im Juli kippt es. Sonne und interne Lasten übersteigen die Verluste — der Überschuss ist Kühlenergiebedarf, den wir abführen müssen."),
        ("beide",
         "Same building, same physics, opposite sign. Heating counted the annual demand, cooling the design load — two quantities, one envelope.",
         "Gleiches Gebäude, gleiche Physik, umgekehrtes Vorzeichen. Heizen zählte den Jahresbedarf, Kühlen die Auslegungslast — zwei Größen, eine Hülle."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Dieselbe Bilanz, zwei Jahreszeiten", title)
        din = _din_ref("DIN V 18599-2 · VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        H = _home(np.array([-2.9, 0.0, 0.0]), 0.9)
        G = _gains(H)
        P = _paths(H, reach=0.8)
        envelope = _envelope(H)
        season, eta_t = ValueTracker(0.0), ValueTracker(0.0)

        def mix(a, b):
            return a + (b - a) * season.get_value()

        def d_theta():
            return mix(D_THETA, D_THETA_JUL)

        base_y, unit, width = -1.0, 2.9 / Q_SENKE, 1.0
        x_loss, x_gain = 0.4, 2.4

        def loss_h():
            return mix(Q_SENKE, ETA_K * Q_SENKE_JUL)

        def gain_h():
            return mix(Q_USABLE, Q_QUELLE_JUL)

        def column(x, height, color_fn, opacity):
            def build():
                h = max(0.02, height() * unit)
                c = color_fn()
                return Rectangle(width=width, height=h, color=c, fill_color=c, fill_opacity=opacity,
                                 stroke_width=1.2).move_to(np.array([x, base_y + h / 2, 0.0]))
            return always_redraw(build)

        loss_col = column(x_loss, loss_h, lambda: LOSS_BLUE, 0.55)
        gain_col = column(x_gain, gain_h,
                          lambda: interpolate_color(ManimColor(ETA_GREEN), ManimColor(SOLAR_YELLOW), season.get_value()), 0.85)
        mid_x = (x_loss + x_gain) / 2

        def gap():
            lo, hi = sorted((loss_h(), gain_h()))
            y0, y1 = base_y + lo * unit, base_y + hi * unit
            if y1 - y0 < 0.08:
                return VGroup()
            return VGroup(
                Rectangle(width=x_gain - x_loss - width, height=y1 - y0, stroke_width=0, fill_color=DEMAND_RED,
                          fill_opacity=0.3).move_to(np.array([mid_x, (y0 + y1) / 2, 0.0])),
                DashedLine(np.array([x_loss + width / 2, y1, 0.0]), np.array([x_gain - width / 2, y1, 0.0]),
                           color=P_WHITE, stroke_width=1.6, stroke_opacity=0.6),
                DashedLine(np.array([x_loss + width / 2, y0, 0.0]), np.array([x_gain - width / 2, y0, 0.0]),
                           color=P_WHITE, stroke_width=1.6, stroke_opacity=0.6),
                dim_arrow(np.array([mid_x + 0.2, y0, 0.0]), np.array([mid_x + 0.2, y1, 0.0]), color=DEMAND_RED),
            )

        gap_mob = always_redraw(gap)
        ground = Line(np.array([x_loss - 0.75, base_y, 0.0]), np.array([x_gain + 0.75, base_y, 0.0]),
                      color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        cap_loss = math_label(r"Q_{\mathrm{Senke}}", np.array([x_loss, base_y - 0.38, 0.0]), size=BODY_FONT_SIZE, color=LOSS_BLUE)
        cap_gain = math_label(r"\eta_{h}\cdot Q_{\mathrm{Quelle}}", np.array([x_gain, base_y - 0.38, 0.0]),
                              size=BODY_FONT_SIZE, color=ETA_GREEN)
        cap_loss_k = math_label(r"\eta_{k}\cdot Q_{\mathrm{Senke}}", np.array([x_loss, base_y - 0.38, 0.0]),
                                size=BODY_FONT_SIZE, color=LOSS_BLUE)
        cap_gain_k = math_label(r"Q_{\mathrm{Quelle}}", np.array([x_gain, base_y - 0.38, 0.0]),
                                size=BODY_FONT_SIZE, color=SOLAR_YELLOW)
        gap_lab_h = math_label(r"Q_{h}", np.array([mid_x - 0.2, base_y + (loss_h() + gain_h()) / 2 * unit - 0.07, 0.0]),
                               size=BODY_FONT_SIZE, color=DEMAND_RED)
        gap_lab_k = math_label(r"Q_{K}", np.array([mid_x - 0.2, base_y + (mix(Q_SENKE, ETA_K * Q_SENKE_JUL) * 0 + 0.5 * (ETA_K * Q_SENKE_JUL + Q_QUELLE_JUL)) * unit - 0.07, 0.0]),
                               size=BODY_FONT_SIZE, color=DEMAND_RED)

        def sun_rays_now(rt=None):
            m = season.get_value()
            c = np.array([-6.0 + 0.4 * m, 0.95 + 1.05 * m, 0.0])
            return c, _solar(H, c, ("up", "lo"))

        def sun_mob():
            m = season.get_value()
            c, S = sun_rays_now()
            sun = sun_glyph(c).scale(0.28 + 0.2 * m, about_point=c)
            sun.set_opacity(0.55 + 0.45 * m)
            for r in S["sets"]:
                r["out"].set_stroke(width=1.6 + 2.4 * m, opacity=0.45 + 0.45 * m)
                r["in"].set_stroke(width=1.4 + 1.4 * m, opacity=0.3 + 0.3 * m)
            return VGroup(sun, S["group"])

        sun_live = always_redraw(sun_mob)
        plant = radiator(np.array([H["x_r"] - 0.6, H["floor_y"] + 0.28, 0.0]), color=DEMAND_RED).scale(0.9)
        supply = P["supply"]
        removal = [smooth_path([supply[0].point_from_proportion(u) for u in np.linspace(1.0, 0.0, 6)])]

        season_chip = chip("Januar", LOSS_BLUE).move_to(np.array([5.6, 2.0, 0.0]))
        result_chip = chip("Heizwärmebedarf", DEMAND_RED).move_to(np.array([5.6, 1.4, 0.0]))
        season_chip_j = chip("Juli", P_RED).move_to(season_chip)
        result_chip_k = chip("Kühlbedarf", DEMAND_RED).move_to(result_chip)
        eta_g = _gauge(r"\eta", ETA_GREEN, np.array([5.45, 0.75, 0.0]), eta_t)
        card = _card("", P_TEAL, [
            (lambda: rf"\Delta\theta = {de_num(d_theta())}\,\mathrm{{K}}", P_BLUE),
            (lambda: rf"I_{{S}} = {de_num(mix(I_S, I_S_JUL))}\,\mathrm{{kWh/m^{{2}}}}", P_YELLOW),
        ], top=0.25)

        def winter_flows(rt):
            return [_stream(P["trans"], LOSS_BLUE, rt, waves=4), _stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE, waves=3),
                    _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE, waves=3),
                    *_gain_heat(G, rt), pulse_flashes(sun_rays_now()[1]["paths"], SOLAR_YELLOW, repeats=max(1, int(rt / 1.5)))]

        def heating(rt):
            return [_stream(supply, DEMAND_RED, rt, period=1.2),
                    ripples([plant.get_top() + UP * 0.02], r_max=0.5, color=DEMAND_RED, cycles=max(1.0, rt / 1.3))]

        def summer_flows(rt):
            return [_stream(P["trans"], LOSS_BLUE, rt, waves=1), _stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE, waves=1),
                    _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE, waves=1),
                    *_gain_heat(G, rt, color=P_RED, r_max=0.5),
                    pulse_flashes(sun_rays_now()[1]["paths"], SOLAR_YELLOW, repeats=max(1, int(rt / 1.2)))]

        def cooling(rt):
            return [_stream(removal, P_CYAN, rt, period=1.2)]

        row_w, box_w, it_w = math_panel([
            ("qh", r"Q_{h}", DEMAND_RED), (None, "=", P_WHITE),
            ("senke", r"Q_{\mathrm{Senke}}", LOSS_BLUE), (None, "−", P_WHITE),
            ("eta", r"\eta_{h}", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("quelle", r"Q_{\mathrm{Quelle}}", SOLAR_YELLOW), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])
        row_k, box_k, it_k = math_panel([
            ("qk", r"Q_{K}", DEMAND_RED), (None, "=", P_WHITE),
            ("quelle", r"Q_{\mathrm{Quelle}}", SOLAR_YELLOW), (None, "−", P_WHITE),
            ("eta", r"\eta_{k}", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("senke", r"Q_{\mathrm{Senke}}", LOSS_BLUE), (None, r"\;[\mathrm{kWh}]", P_TEAL),
        ])

        self.add(H["air"])
        self.play(Create(H["group"]), Create(envelope), run_time=1.2)
        self.add(loss_col, gain_col)
        self.play(Create(ground), FadeIn(cap_loss), FadeIn(cap_gain), LaggedStart(*[FadeIn(m) for m in G["sources"]], lag_ratio=0.15),
                  H["air"].animate.set_fill(P_ORANGE, opacity=0.07), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 1.2, during=lambda rt: _gain_heat(G, rt))

        # —— Winter ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        ring_h = highlight_param(it_w, "qh", color=DEMAND_RED)
        self.add(sun_live, *card["rows"], eta_g["read"])
        self.play(FadeIn(season_chip), FadeIn(result_chip), FadeIn(card["frame"]), FadeIn(eta_g["static"]),
                  eta_t.animate.set_value(ETA_H), FadeIn(plant), *winter_flows(1.2), *heating(1.2), run_time=1.2)
        self.play(FadeIn(gap_mob), FadeIn(gap_lab_h), Create(box_w), FadeIn(row_w), Create(ring_h), *winter_flows(1.2), *heating(1.2),
                  run_time=1.2)
        hold_for(self, self.NARRATION, "winter", during=lambda rt: [*winter_flows(rt), *heating(rt)])

        # —— Summer: the same columns swap magnitude ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sommer"))
        self.play(FadeOut(ring_h), run_time=0.2)
        self.play(
            season.animate.set_value(1.0), eta_t.animate.set_value(ETA_K),
            ReplacementTransform(season_chip, season_chip_j), ReplacementTransform(result_chip, result_chip_k),
            ReplacementTransform(cap_loss, cap_loss_k), ReplacementTransform(cap_gain, cap_gain_k),
            ReplacementTransform(gap_lab_h, gap_lab_k),
            ReplacementTransform(box_w, box_k), ReplacementTransform(row_w, row_k),
            plant.animate.set_color(P_CYAN), H["air"].animate.set_fill(P_RED, opacity=0.2),
            *summer_flows(2.4), *cooling(2.4), run_time=2.4, rate_func=smooth,
        )
        hold_for(self, self.NARRATION, "sommer", during=lambda rt: [*summer_flows(rt), *cooling(rt)])

        # —— One envelope, two signs ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "beide"))
        sign_h = chip("Heizen: Jahresbedarf", DEMAND_RED).move_to(np.array([5.45, 0.75, 0.0]))
        sign_k = chip("Kühlen: Auslegungslast", P_CYAN).move_to(np.array([5.45, 0.1, 0.0]))

        def both(rt):
            return [*summer_flows(rt), *cooling(rt), _stream(supply, DEMAND_RED, rt, period=1.2)]

        self.remove(*card["rows"], eta_g["read"])
        self.play(FadeOut(VGroup(card["frame"], eta_g["static"])), FadeIn(sign_h), FadeIn(sign_k),
                  Indicate(envelope, color=P_TEAL, scale_factor=1.0), *both(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "beide", during=both)

        self.remove(sun_live, loss_col, gain_col, gap_mob)
        self.play(FadeOut(VGroup(caption, box_k, row_k, season_chip_j, result_chip_k,
                                 sign_h, sign_k, cap_loss_k, cap_gain_k, gap_lab_k, ground, plant, envelope,
                                 G["sources"], H["group"], H["air"])), run_time=0.6)
        self.wait(0.4)

#endregion
