"""🏛️ Ops writer of `steel-column` (the leftover of batch MC, 21 `derived-view-in-band` cells; the asset went to the shared-code owner MX-2). At 1:100 the base and head plates were drawn 35 mm thick, so the edge of the near gusset stood 75 mm above the drawn plate in the front elevation where the view onto the web has nothing: 75.7 mm between the two views against the 90 mm the measure asks on the 3 m column. The body of the band has 20 mm plates under a gusset that reaches 0.11 m: drawn as the body is, the gusset stands 90 mm above the plate, 0.9 mm on paper. With the gusset edge at 20 mm the two views then differ by exactly the 90 mm asked, a tie (the corner of the edge is 90 mm from the gusset tip of the side elevation); with the edge at its true 16 mm, as at 1:50, the corner is 90.35 mm from its nearest point and the measure is 0.03012 against 0.03. This writer draws the plates of 1:100 at `PLATE` metres (default the true 0.02) and the gusset edges at `EDGE` metres (default the true 0.016), with bar, gussets and anchor studs standing on the plate, and puts the decision texts in step. It reads neither dimension from what it changes, so a second run writes the same ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
T = float(os.environ.get("PLATE", "0.02"))
MM = round(T * 1000)
EDGE = float(os.environ.get("EDGE", "0.016"))
a = asset("steel-column")


def along(sid, low, high):
    """📏️ The op that puts the lower plate face of a side elevation polygon at T and its upper plate face at 3 - T: every point on the old bottom face `low` or the old top face `high` moves, the others stay."""
    s = shape(a, 100, "elevation", sid)
    old = sorted({p[0] for p in s["points"]})
    return op(100, "elevation", sid, like(s, points=[[T if low and p[0] == old[0] else 3 - T if high and p[0] == old[-1] else p[0], p[1]] for p in s["points"]]))


def base(sid, top, **across):
    """🧱️ The op that lets a front rectangle stand on the base plate and reach up to `top` (`across`: its x and width)."""
    return op(100, "front", sid, like(shape(a, 100, "front", sid), y=T, h=round(top - T, 6), **across))


ops = [
    along("column-web-bar", True, True),
    op(100, "elevation", "base-plate-block", like(shape(a, 100, "elevation", "base-plate-block"), points=[[0, 0.15], [0, -0.15], [T, -0.15], [T, 0.15]])),
    op(100, "elevation", "cap-plate-block", like(shape(a, 100, "elevation", "cap-plate-block"), points=[[3 - T, 0.12], [3 - T, -0.12], [3, -0.12], [3, 0.12]])),
    along("stiffener-left", True, False),
    along("stiffener-right", True, False),
    along("anchor-a", True, False),
    along("anchor-b", True, False),
    op(100, "front", "column-flange-bar", like(shape(a, 100, "front", "column-flange-bar"), y=T, h=round(3 - 2 * T, 6))),
    op(100, "front", "base-plate-block", like(shape(a, 100, "front", "base-plate-block"), y=0, h=T)),
    op(100, "front", "cap-plate-block", like(shape(a, 100, "front", "cap-plate-block"), y=round(3 - T, 6), h=T)),
    base("stiffener-edge", 0.11, x=-EDGE / 2, w=EDGE),
    base("anchor-a", 0.075),
    base("anchor-b", 0.075),
    *[op(100, "plan", sid, like(shape(a, 100, "plan", sid), y=-EDGE / 2, h=EDGE)) for sid in ("stiffener-a", "stiffener-b")],
]
if MM == 20 and EDGE == 0.016:
    thin = {
        "en": "The 20 mm plates and the 16 mm gussets are drawn at their true thickness (0.2 mm and 0.16 mm, with the 0.1 mm pen 0.3 mm and 0.26 mm of ink), so the gusset stands its true 90 mm above the base plate (0.9 mm) and its edge tells the front from the view onto the web. The gussets are drawn with a 20 mm nose and a 15 mm flat top, so no filled tip is thinner than the printable feature; washer, nut and bolt end of each anchor are one 24 mm stud. The model has the same plates and gussets.",
        "de": "Die 20-mm-Platten und die 16-mm-Rippen sind in ihrer wahren Dicke gezeichnet (0,2 mm und 0,16 mm, mit der 0,1-mm-Feder 0,3 mm und 0,26 mm Farbe), sodass die Rippe ihre wahren 90 mm über der Fußplatte steht (0,9 mm) und ihre Kante die Vorderansicht von der Ansicht auf den Steg unterscheidet. Die Rippen sind mit 20-mm-Nase und 15 mm breiter Oberkante gezeichnet, sodass keine gefüllte Spitze dünner ist als das druckbare Merkmal; Scheibe, Mutter und Bolzenende jedes Ankers sind ein 24-mm-Bolzen. Das Modell hat dieselben Platten und Rippen.",
    }
    retained = {
        "en": "Plates, two gussets of the true 50 mm width up to the true height of 0.11 m (0.5 x 0.9 mm above the plate) and four anchor studs of 24 x 55 mm (0.24 x 0.55 mm).",
        "de": "Platten, zwei Rippen der wahren Breite von 50 mm bis zur wahren Höhe von 0,11 m (0,5 x 0,9 mm über der Platte) und vier Ankerbolzen von 24 x 55 mm (0,24 x 0,55 mm).",
    }
    body = a["volumes"]["100"]["decisions"][0]
    old = {
        "en": "(above the 15 mm feature; only the drawing thickens them to 35 mm so that they still print at 1:150)",
        "de": "(über dem 15-mm-Merkmal; nur die Zeichnung verdickt sie auf 35 mm, damit sie auch bei 1:150 drucken)",
    }
    new = {
        "en": "(above the 15 mm feature; the drawing shows them at the same 20 mm)",
        "de": "(über dem 15-mm-Merkmal; die Zeichnung zeigt sie mit denselben 20 mm)",
    }
    shown = a["representations"]["100"]["decisions"]["silhouette"]
    ops += [
        put(["representations", "100", "decisions", "silhouette"], {"en": shown["en"].replace("the near gusset (20 mm)", "the near gusset (16 mm)"), "de": shown["de"].replace("der vorderen Rippe (20 mm)", "der vorderen Rippe (16 mm)")}),
        put(["representations", "100", "decisions", "thinElements"], thin),
        put(["representations", "100", "decisions", "retained"], retained),
        put(["volumes", "100", "decisions", 0], {language: body[language].replace(old[language], new[language]) for language in ("en", "de")}),
    ]
write("steel-column", ops)
