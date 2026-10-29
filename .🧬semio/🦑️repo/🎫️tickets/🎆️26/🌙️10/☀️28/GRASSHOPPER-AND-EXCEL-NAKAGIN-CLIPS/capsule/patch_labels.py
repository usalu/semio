import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b, count=1):
    global s
    if a not in s:
        print("MISSING", a[:70])
        return
    s = s.replace(a, b, count)


rep('NAMES = {"w": "Width", "l": "Length", "h": "Height", "r": "Radius", "d": "Window Ø", "wu": "Offset", "bu": "Bolt x", "bvl": "Bolt low", "bvh": "Bolt high", "an": "Angle"}',
    'NAMES = {"w": ("Width", "Breite"), "l": ("Length", "Länge"), "h": ("Height", "Höhe"), "r": ("Radius", "Radius"), "d": ("Window Ø", "Fenster Ø"), "wu": ("Offset", "Versatz"),\n         "bu": ("Bolt x", "Bolzen x"), "bvl": ("Bolt low", "Bolzen unten"), "bvh": ("Bolt high", "Bolzen oben"), "an": ("Angle", "Winkel")}')
rep('self.gh_slider(NAMES.get(oid, s["label"]),', 'self.gh_slider(self.tr(*NAMES[oid]) if oid in NAMES else s["label"],')
rep('self.gh_slider(NAMES["w"],', 'self.gh_slider(self.tr(*NAMES["w"]),')
rep('self._say("Perspective",', 'self._say(self.tr("Perspective", "Perspektive"),')
rep('"settings": {"label": "Base A", "data": [plane_a]}', '"settings": {"label": self.tr("Base A", "Basis A"), "data": [plane_a]}', 2)
rep('"settings": {"label": "Target B"', '"settings": {"label": self.tr("Target B", "Ziel B")')
rep('"settings": {"label": "Capsule"', '"settings": {"label": self.tr("Capsule", "Kapsel")')
rep('"settings": {"label": "Core axis"', '"settings": {"label": self.tr("Core axis", "Kernachse")')
rep('"settings": {"label": "Angle", "min"', '"settings": {"label": self.tr("Angle", "Winkel"), "min"')
rep('"settings": {"label": "Types"', '"settings": {"label": self.tr("Types", "Typen")')
rep('"settings": {"label": "Planes"', '"settings": {"label": self.tr("Planes", "Ebenen")')
open(p, "w", encoding="utf-8").write(s)
