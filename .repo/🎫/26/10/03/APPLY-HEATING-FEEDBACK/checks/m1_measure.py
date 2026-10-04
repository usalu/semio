import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[7] / "tutorial"
sys.path.insert(0, str(ROOT))
from manim_fonts import BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE, scene_title, beat_subtitle
from manim_visuals import math_label
t = scene_title("Modul 1: Die Grundlagen der Bauphysik")
s = beat_subtitle("Wärmedurchlasswiderstand R", t)
print("[DEBUG] subtitle bottom", s.get_bottom()[1], "title bottom", t.get_bottom()[1])
for src, size in [
    (r"\lambda = 1{,}0\,\mathrm{W/(m\,K)}", LABEL_FONT_SIZE),
    (r"d = 0{,}24\,\mathrm{m}", BODY_FONT_SIZE),
    (r"R = \frac{d}{\lambda} = 13{,}7\,\mathrm{m^{2}K/W}", BODY_FONT_SIZE),
    (r"R_{\mathrm{ges}} = R_{1} + R_{2} + R_{3} + \ldots", BODY_FONT_SIZE),
    (r"R_{\mathrm{ges}} = 0{,}13 + 0{,}02 + 0{,}49 + 5{,}71 + 0{,}02 + 0{,}04 = 6{,}41", LABEL_FONT_SIZE),
    (r"U = \frac{1}{R_{\mathrm{ges}}} = 1{,}43\,\mathrm{W/(m^{2}K)}", BODY_FONT_SIZE),
    (r"\dot{Q} = U \cdot A \cdot \Delta\theta = 2\,059\,\mathrm{W}", BODY_FONT_SIZE),
    (r"\Phi_{\mathrm{int}} = 290\,\mathrm{W}", BODY_FONT_SIZE),
    (r"q_{\mathrm{int}} = \frac{\Phi_{\mathrm{int}}}{A_{\mathrm{N}}} = \frac{290\,\mathrm{W}}{20\,\mathrm{m^{2}}} = 14{,}5\,\mathrm{W/m^{2}}", FORMULA_FONT_SIZE),
]:
    m = math_label(src, size=size)
    print(f"[DEBUG] {m.width:5.2f} x {m.height:4.2f}  {src}")
from manim_visuals import math_panel, P_WHITE
for parts in ([("r", "R", P_WHITE), (None, "=", P_WHITE), ("f", r"\frac{d}{\lambda}", P_WHITE), (None, r"\;[\mathrm{m^{2}K/W}]", P_WHITE)],
              [("q", r"\dot{Q}", P_WHITE), (None, "=", P_WHITE), ("u", "U", P_WHITE)]):
    row, box, items = math_panel(parts)
    print("[DEBUG] panel row", round(row.get_bottom()[1], 2), round(row.get_top()[1], 2), "box", round(box.get_bottom()[1], 2), round(box.get_top()[1], 2))
    row, box, items = math_panel(parts, edge_buff=1.2)
    print("[DEBUG] panel1.2 row", round(row.get_bottom()[1], 2), round(row.get_top()[1], 2), "box", round(box.get_bottom()[1], 2), round(box.get_top()[1], 2))
