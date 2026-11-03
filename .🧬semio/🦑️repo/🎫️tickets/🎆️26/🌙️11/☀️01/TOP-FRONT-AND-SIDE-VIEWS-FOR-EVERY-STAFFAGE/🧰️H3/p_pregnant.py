import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from human import *

P = dict(hl=.09, t0=.085, t1=.063, s0=.062, s1=.042, fw=.052, ww=.145, cw=.16, sw=.175, nw=.045, ac=.152, a0=.048, a1=.04, f0=.04, f1=.032, hr=.03, hw=.075, pd=.115, tl=.055, cd=.115, sd=.1)
KEY = {"thigh": "thigh-{s}", "shank": "shank-{s}", "shoe": "boot-{s}", "toe": "boot-toe-{s}", "shoe-toe": "boot-toe-{s}", "hips": "thigh-l", "pelvis": "sweater", "torso": "sweater",
       "sleeve": "sleeve-{s}", "forearm": "forearm-{s}", "hand": "hand-{s}", "neck": "neck", "skin": "head", "hair": "hair", "skull": "head", "cuff": "cuff-rib-l", "hem": "sweater-hem-rib"}


def build(a, sc, of, op):
    h = Human(a, coarse(P, sc), KEY); R = h.R; t = tier(sc)
    h.front(sc, of); h.plan(sc, op)
    bx, by = R.o("belly")
    ell(of, "belly", "belly", 0.0, by + 0.015, 0.152, 0.19, 4.4, h.st.get(sc, "belly-cover"))
    ell(op, "belly", "belly", bx + 0.118, 0.0, 0.122, 0.15, 3.92, h.st.get(sc, "belly-cover"))
    if t >= 2:
        line(of, "linea-fold", "belly", [(0.0, by + 0.19), (0.0, by - 0.17)], 4.5, 0.15)
    if t >= 3:
        line(of, "belly-stretch-1", "belly", [(-0.1, by + 0.07), (0.1, by + 0.07)], 4.5, 0.15, role="texture")
        line(of, "belly-stretch-2", "belly", [(-0.11, by - 0.03), (0.11, by - 0.03)], 4.5, 0.15, role="texture")
        line(op, "belly-stretch", "belly", [(bx + 0.05, -0.1), (bx + 0.2, -0.02)], 3.93, 0.15, role="texture")


NOTES = {
    "500": {"en": "head, body with the belly, two arms and two legs as flat masses on the ground line. Plan: head, shoulders, belly bulge, arms and legs as masses.",
            "de": "Kopf, Körper mit dem Bauch, zwei Arme und zwei Beine als flache Massen auf der Standlinie. Grundriss: Kopf, Schultern, Bauchwölbung, Arme und Beine als Massen."},
    "200": {"en": "head with hair cap, neck, sweater body with the rounded belly, sleeves with shoulder and elbow joints, hands, leggings with knee joints and boots. Plan: head with face, neck, shoulders, belly, arms, hands, legs and boots from above.",
            "de": "Kopf mit Haarkappe, Hals, Pulloverkörper mit dem runden Bauch, Ärmel mit Schulter- und Ellbogengelenken, Hände, Leggings mit Kniegelenken und Stiefel. Grundriss: Kopf mit Gesicht, Hals, Schultern, Bauch, Arme, Hände, Beine und Stiefel von oben."},
    "100": {"en": "head with hair, ears and face, neck, sweater with hip line, belly with its centre fold, shoulder, elbow and wrist joints, leggings with knee joints, boots with toe caps. Plan: head with face, shoulders, trunk, belly, arms with hands, legs and boots.",
            "de": "Kopf mit Haar, Ohren und Gesicht, Hals, Pullover mit Hüftlinie, Bauch mit Mittelfalte, Schulter-, Ellbogen- und Handgelenke, Leggings mit Kniegelenken, Stiefel mit Kappen. Grundriss: Kopf mit Gesicht, Schultern, Rumpf, Bauch, Arme mit Händen, Beine und Stiefel."},
    "50": {"en": "eyes, brows, nose and mouth, ears, hair cap with fringe, sweater hem, belly with centre fold and stretch lines, cuffs, boot soles and leggings hems. Plan: nose, ears, parting, shoulder seams, belly stretch lines and boot laces besides everything of 1:100. Limbs follow the pose by the bone travel, so strides, raised hands and the leaned-back torso change both views.",
           "de": "Augen, Brauen, Nase und Mund, Ohren, Haarkappe mit Pony, Pulloversaum, Bauch mit Mittelfalte und Dehnungslinien, Bündchen, Stiefelsohlen und Leggings-Säume. Grundriss: Nase, Ohren, Scheitel, Schulternähte, Bauch-Dehnungslinien und Stiefelschnürung zusätzlich zu allem aus 1:100. Die Glieder folgen der Pose über den Knochenweg, sodass Schritte, erhobene Hände und der zurückgelehnte Rumpf beide Ansichten verändern."},
}

if __name__ == "__main__":
    print(write("pregnant-person", build, NOTES))
