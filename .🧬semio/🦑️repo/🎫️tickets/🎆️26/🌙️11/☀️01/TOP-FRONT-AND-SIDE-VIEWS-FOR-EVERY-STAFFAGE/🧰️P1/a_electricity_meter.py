from lib import *

ID = "electricity-meter"
B = "body"
AB, PC = "abs-plastic", "polycarbonate"

SPEC = {
    50: {
        "front": [
            rect("back-plate", B, 0.035, -0.1425, 0.03, 0.285, 3, 0.78, "casing", AB, r=0.004),
            rect("body", B, -0.058, -0.1, 0.093, 0.2425, 3.1, 0.88, "casing", AB, r=0.012),
            rect("terminals", B, -0.058, -0.1425, 0.093, 0.0425, 3.2, 0.55, "casing", AB, r=0.004),
            line("seal-seam", B, [[0.035, -0.1], [0.035, 0.1425]], 3.4, 0.3, "casing", AB),
            rect("rim", B, -0.065, -0.04, 0.007, 0.1, 3.15, 0.25, "window", PC),
            rect("counter", B, -0.065, 0.078, 0.007, 0.026, 3.2, 0.1, "window", PC),
            rect("eye", B, -0.062, 0.115, 0.004, 0.016, 3.3, 0.15, "casing", AB, role="detail"),
            rect("plate-edge", B, -0.065, -0.085, 0.007, 0.04, 3.12, 0.78, "plate", AB),
        ],
        "plan": [
            rect("back-plate", B, -0.085, 0.035, 0.17, 0.03, 3, 0.78, "casing", AB, r=0.004),
            rect("body", B, -0.085, -0.058, 0.17, 0.093, 3.1, 0.88, "casing", AB, r=0.012),
            line("seal-seam", B, [[-0.085, 0.035], [0.085, 0.035]], 3.4, 0.3, "casing", AB),
            rect("rim", B, -0.05, -0.065, 0.1, 0.007, 3.15, 0.25, "window", PC),
            rect("counter", B, -0.045, -0.065, 0.09, 0.007, 3.2, 0.1, "window", PC),
            rect("eye", B, -0.008, -0.062, 0.016, 0.004, 3.3, 0.15, "casing", AB, role="detail"),
            rect("plate-edge", B, -0.065, -0.065, 0.13, 0.007, 3.12, 0.78, "plate", AB),
            line("screw-left", B, [[-0.07, -0.058], [-0.07, 0.035]], 3.4, 0.5, "casing", AB),
            line("screw-right", B, [[0.07, -0.058], [0.07, 0.035]], 3.4, 0.5, "casing", AB),
        ],
    },
    100: {
        "front": [
            rect("back-plate", B, 0.035, -0.1425, 0.03, 0.285, 3, 0.78, "casing", AB),
            rect("body", B, -0.058, -0.1, 0.093, 0.2425, 3.1, 0.88, "casing", AB),
            rect("terminals", B, -0.058, -0.1425, 0.093, 0.0425, 3.2, 0.55, "casing", AB),
            rect("plate-edge", B, -0.065, -0.085, 0.007, 0.04, 3.12, 0.78, "plate", AB),
            rect("rim", B, -0.065, -0.04, 0.007, 0.1, 3.15, 0.25, "window", PC),
            rect("counter", B, -0.065, 0.076, 0.007, 0.03, 3.2, 0.1, "window", PC),
        ],
        "plan": [
            rect("back-plate", B, -0.085, 0.035, 0.17, 0.03, 3, 0.78, "casing", AB),
            rect("body", B, -0.085, -0.058, 0.17, 0.093, 3.1, 0.88, "casing", AB),
            rect("plate-edge", B, -0.065, -0.065, 0.13, 0.007, 3.12, 0.78, "plate", AB),
            rect("rim", B, -0.05, -0.065, 0.1, 0.007, 3.15, 0.25, "window", PC),
        ],
    },
    200: {
        "front": [
            rect("body", B, -0.045, -0.1425, 0.11, 0.285, 3, 0.88, "casing", AB),
            rect("terminals", B, -0.058, -0.1425, 0.093, 0.0425, 3.2, 0.55, regions=["casing", "plate"], material=AB),
            rect("dome", B, -0.065, -0.042, 0.02, 0.104, 3.1, 0.25, "window", PC),
        ],
        "plan": [
            rect("body", B, -0.085, -0.045, 0.17, 0.11, 3, 0.88, "casing", AB),
            rect("dome", B, -0.052, -0.065, 0.104, 0.02, 3.1, 0.25, regions=["window", "plate"], material=PC),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, 2.6 mm by 5.7 mm on paper: the meter side-on with the flat back plate against the wall, the rounded cover in front of it, the dark terminal cover at the foot and the two window domes, the long one of the disc and the short one of the counter, standing out of the cover; a small optical eye sits above. Plan, 3.4 mm by 2.6 mm: back plate and cover with the seal seam, the two window domes and the optical eye at the front, and the two sealing screws.",
            "de": "Vorderansicht, 2,6 mm auf 5,7 mm: der Zähler von der Seite mit der flachen Rückplatte an der Wand, davor der gerundete Deckel, am Fuß die dunkle Klemmenabdeckung und die beiden Fensterwölbungen, die lange der Scheibe und die kurze des Zählwerks, die aus dem Deckel treten; darüber sitzt das kleine optische Auge. Grundriss, 3,4 mm auf 2,6 mm: Rückplatte und Deckel mit der Plombenfuge, vorn die beiden Fensterwölbungen und das optische Auge sowie die beiden Plombierschrauben.",
        },
        "omitted": {
            "en": "Digits, disc, hub and rating-plate lines are not seen from the side or from above; the rating plate shows as a 7 mm raised strip: in front as its edge, from above as two stubs left and right of the window dome, so that its thermal region reads in every view.",
            "de": "Ziffern, Scheibe, Nabe und Typenschildlinien sind von der Seite und von oben nicht zu sehen; das Typenschild zeigt sich als 7 mm hoher Streifen: vorn als Kante, von oben als zwei Stummel links und rechts der Fensterwölbung, sodass sein thermischer Bereich in jeder Ansicht lesbar ist.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 1.3 mm by 2.85 mm: back plate, cover, terminal cover and the two window domes. Plan, 1.7 mm by 1.3 mm: back plate, cover, the raised rating plate and the window dome.",
            "de": "Vorderansicht, 1,3 mm auf 2,85 mm: Rückplatte, Deckel, Klemmenabdeckung und die beiden Fensterwölbungen. Grundriss, 1,7 mm auf 1,3 mm: Rückplatte, Deckel, das erhabene Typenschild und die Fensterwölbung.",
        },
        "omitted": {
            "en": "Seal seam, optical eye and screws.",
            "de": "Plombenfuge, optisches Auge und Schrauben.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 0.65 mm by 1.4 mm: one body with the dark terminal cover and the window dome. Plan, 0.85 mm by 0.65 mm: one body and the dome, which carries the window and the rating plate as one face layer.",
            "de": "Vorderansicht, 0,65 mm auf 1,4 mm: ein Körper mit der dunklen Klemmenabdeckung und der Fensterwölbung. Grundriss, 0,85 mm auf 0,65 mm: ein Körper und die Wölbung, die Fenster und Typenschild als eine Frontschicht trägt.",
        },
        "omitted": {
            "en": "Counter dome, back plate and cover are one mass; the window dome is drawn 0.02 m deep.",
            "de": "Zählwerkwölbung, Rückplatte und Deckel sind eine Masse; die Fensterwölbung ist 0,02 m tief gezeichnet.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
