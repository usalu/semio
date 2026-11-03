"""📏 pa: OI-33 follow-up - the catalog check ties heightM to the default pose (mid-stroke, 5.57 m), so heightM is 5.57 m and the 6.27 m of the stopped pose is documented - run once."""
from jio import load, save


def swap(holder, pairs):
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in holder[lang]:
                holder[lang] = holder[lang].replace(old, new)


path, data = load("oil-pumpjack")
a = data["asset"]
a["dimensions"]["heightM"] = 5.57
swap(a["provenance"]["notes"], {
    "en": [("The unit is 6.3 m high at the top of the stroke with the horsehead up (the stopped pose) and 5.5 m at mid-stroke.", "The unit is 5.57 m high at mid-stroke (the default pose, heightM) and 6.27 m with the horsehead up (the stopped pose): the usual 6 to 8 m of conventional units belong to larger sizes than this API 114 class.")],
    "de": [("Die Einheit ist am oberen Hubende mit hochstehendem Pferdekopf (Pose gestoppt) 6,3 m hoch und in Hubmitte 5,5 m", "Die Einheit ist in Hubmitte (Grundpose, heightM) 5,57 m hoch und mit hochstehendem Pferdekopf (Pose gestoppt) 6,27 m: die üblichen 6 bis 8 m klassischer Einheiten gehören zu größeren Baugrößen als dieser Klasse API 114")],
})
swap(a["representations"]["50"]["decisions"]["silhouette"], {
    "en": [("A 9 m wide, 6.3 m tall conventional oil pumpjack (horsehead up), 18 cm by 13 cm on paper:", "A 9 m wide, 5.6 m tall conventional oil pumpjack (6.3 m with the horsehead up), 18 cm by 11 cm on paper:")],
    "de": [("Eine 9 m breite und 6,3 m hohe klassische Erdölpumpe (Pferdekopf oben), auf Papier 18 cm mal 13 cm:", "Eine 9 m breite und 5,6 m hohe klassische Erdölpumpe (6,3 m mit hochstehendem Pferdekopf), auf Papier 18 cm mal 11 cm:")],
})
save(path, data)
print("ok")
