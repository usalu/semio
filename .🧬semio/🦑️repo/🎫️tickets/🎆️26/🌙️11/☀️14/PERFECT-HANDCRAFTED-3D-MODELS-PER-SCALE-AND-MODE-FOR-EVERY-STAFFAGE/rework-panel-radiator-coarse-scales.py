"""♨️ components/panel-radiator: 1:100, 1:200 and 1:500 modelled for their smallest feature instead of the 1:50 parts list.

1:100 (15 mm): the side covers are 15 mm slabs (were 10), the pierced top plate is 15 mm (was 7.5).
1:200 (30 mm): each water panel is one flat 30 mm slab over the full height that takes its two headers in (the 22 mm pressed profile and the header rails are gone), the side covers are 30 mm, rim and plate of the grille are one 30 mm plate with the sixteen typified slots of 8 x 100 mm that the plan of 1:200 draws instead of 32 slots, the union nuts and nipples are part of the connection block and the two 15 mm pipes are one 30 x 80 mm pair body. Thermal: the three front bands are 30 mm slabs of the same panel.
1:500 (3D only, 75 mm): as 1:200 but without side covers and slots (closed 30 mm top plate), the whole connection with its pipes is one block, and each pair of wall clips is one bracket rail from the rest to the hook.
Load path (measured by `volumes-connected`, open before this pass): at 1:50 and 1:100 the four nipple joints run through both headers (they ended in the convector gap, 6.6 mm from the pressed panels) and the headers are members, and at 1:50 the joint of the valve head reaches the upper nipple (it ended 3.3 mm short); at 1:100 the valve set meets the lower headers in a tee body.
One-shot on `volumes/components/panel-radiator.json`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, drop, load, save

ASSET = "components/panel-radiator"
OUTER = [[-0.1015, -0.5015], [0.0015, -0.5015], [0.0015, 0.5015], [-0.1015, 0.5015]]
SLOTS = [[[y0, zc - 0.05], [y0, zc + 0.05], [y0 + 0.008, zc + 0.05], [y0 + 0.008, zc - 0.05]] for y0 in (-0.044, -0.064) for zc in (-0.405, -0.2893, -0.1736, -0.0579, 0.0579, 0.1736, 0.2893, 0.405)]
BANDS = {"th-band-lower": 0.0975, "th-band-mid": 0.2925, "th-band-upper": 0.4875}
TEXT = {
    "100": [("Side covers and the pierced top plate are 15 mm, the smallest feature of 1:100; the valve set meets the two lower headers in a tee body, and the nipple joints of the structure run through both headers, which are members.", "Seitenblenden und gelochte Abdeckplatte messen 15 mm, das kleinste Merkmal von 1:100; die Ventilgarnitur trifft die zwei unteren Sammler in einem T-Stück, und die Nippelgelenke der Struktur laufen durch beide Sammler, die Stäbe sind.")],
    "200": [("Authored for the 30 mm feature: two flat 30 mm water panels over the full height (headers taken in) flank the two folded convectors, closed by 30 mm side covers and one 30 mm top plate with the sixteen typified slots of the plan drawing. Brackets tie the panels back to the wall.", "Für das 30-mm-Merkmal modelliert: zwei ebene 30-mm-Heizplatten über die volle Höhe (Sammler eingeschlossen) fassen die zwei gefalteten Konvektoren, geschlossen durch 30-mm-Seitenblenden und eine 30-mm-Abdeckplatte mit den sechzehn typisierten Schlitzen der Grundrisszeichnung. Konsolen binden die Platten an die Wand."),
            ("The front temperature uses three height bands of the 30 mm front slab. The forces scene carries both panels, their nipples and the wall contacts.", "Die Fronttemperatur nutzt drei Höhenbänder der 30-mm-Frontplatte. Die Kräfteszene trägt beide Platten, ihre Nippel und die Wandauflager."),
            ("Union nuts and nipples are part of the brass connection block; the two 15 mm pipes on 50 mm centres are one pair body of 30 x 80 mm.", "Überwurfmuttern und Nippel gehören zum Messing-Anschlussblock; die zwei 15-mm-Rohre im Abstand von 50 mm sind ein Paarkörper von 30 x 80 mm.")],
    "500": [("3D only, a massing of the 1:200 parts: two flat 30 mm water panels flank the convectors under a closed 30 mm top plate; no side covers and no slots. Brackets tie the panels back to the wall.", "Nur 3D, eine Massierung der Teile von 1:200: zwei ebene 30-mm-Heizplatten fassen die Konvektoren unter einer geschlossenen 30-mm-Abdeckplatte; ohne Seitenblenden und Schlitze. Eine Konsolenschiene je Seite bindet die Platten an die Wand."),
            ("The front temperature uses three height bands of the front slab. The forces scene carries both panels, their nipples and the wall contacts.", "Die Fronttemperatur nutzt drei Höhenbänder der Frontplatte. Die Kräfteszene trägt beide Platten, ihre Nippel und die Wandauflager."),
            ("The connection with its union nuts, nipples and both pipes is one block below the panel end.", "Der Anschluss mit Überwurfmuttern, Nippeln und beiden Rohren ist ein Block unter dem Plattenende.")],
}


def box(solid, size, at):
    """📦 Turns a solid into a plain box, keeping id, bone, material, role, tone and thermal members."""
    for key in ("profile", "holes", "plane", "from", "to", "cornerRadius", "repeat", "axis", "origin"):
        solid.pop(key, None)
    solid.update({"kind": "box", "size": size, "transform": {"translation": at}})


def coarse(volumes, covers):
    """🧱 The merged model of 1:200 and 1:500."""
    body, thermal = by_id(volumes["body"]), by_id(volumes["thermal"])
    if body["panel-front"]["kind"] == "box":
        raise SystemExit("already applied")
    box(body["panel-front"], [0.585, 0.03, 1.0], [0.2925, -0.085, 0.0])
    box(body["panel-rear"], [0.585, 0.03, 1.0], [0.2925, -0.015, 0.0])
    for name, at in BANDS.items():
        box(thermal[name], [0.195, 0.03, 1.0], [at, -0.085, 0.0])
    plate = body["grille-plate"]
    plate.update({"profile": OUTER, "from": 0.57, "to": 0.6})
    if covers:
        plate["holes"] = SLOTS
        box(body["side-cover-left"], [0.585, 0.04, 0.03], [0.2925, -0.05, 0.485])
        box(body["side-cover-right"], [0.585, 0.04, 0.03], [0.2925, -0.05, -0.485])
        box(body["connection-block"], [0.083, 0.036, 0.1], [-0.0385, -0.05, -0.44])
        box(body["pipe"], [0.03, 0.03, 0.08], [-0.095, -0.05, -0.44])
    else:
        plate.pop("holes", None)
        box(body["connection-block"], [0.113, 0.036, 0.1], [-0.0535, -0.05, -0.44])
    gone = ["header-rear", "header-front", "grille-rim", "connection-nut-upper", "connection-nipple", "connection-nut-lower"] + ([] if covers else ["side-cover-left", "side-cover-right", "pipe"])
    drop(volumes["body"], set(gone))
    drop(volumes["thermal"], {"th-" + name for name in gone if name != "header-front"} | {"th-band-lower-header", "th-band-upper-header"})


def load_path(volumes):
    """🔗 Nipple joints through both headers, headers as members."""
    for side in ("left", "right"):
        joint = by_id(volumes["structure"])["joint-nipple-" + side]
        radius = joint["profile"][1][1]
        joint["profile"] = [[-0.1, 0.0], [-0.1, radius], [0.0, radius], [0.0, 0.0]]
    for name in ("header-front", "header-rear"):
        volumes["structure"].append({"id": "member-" + name, "of": name, "member": "radiator", "part": "member"})


def add_tee(volumes):
    """🚰 The tee of the valve set between the two lower headers of 1:100."""
    block = by_id(volumes["body"])["connection-block"]
    volumes["body"].append({"id": "connection-tee", "kind": "box", "bone": "radiator", "material": block["material"], "role": "body", "tone": block["tone"], "size": [0.021, 0.078, 0.071], "transform": {"translation": [0.0105, -0.05, -0.44]}})
    volumes["thermal"].append({"id": "th-connection-tee", "of": "connection-tee", "region": "valve"})


def rail(volumes):
    """🪜 One bracket rail per side at 1:500 instead of two clips."""
    upper = by_id(volumes["body"])["bracket-upper"]
    upper["profile"] = [[0.025, 0.0], [0.494, 0.0], [0.494, 0.035], [0.025, 0.035]]
    drop(volumes["body"], {"bracket-lower"})
    drop(volumes["thermal"], {"th-bracket-lower"})
    drop(volumes["structure"], {"member-bracket-lower"})
    for side, z in (("left", 0.47), ("right", -0.47)):
        joint = by_id(volumes["structure"])["joint-nipple-" + side]
        box(joint, [0.03, 0.1, 0.03], [0.0175, -0.05, z])
        joint["repeat"] = {"count": 2, "translation": [0.55, 0, 0]}


def main():
    """▶️ Rewrites the three coarse scales and their decisions."""
    document, indented = load(ASSET)
    fine = by_id(document["volumes"]["100"]["body"])
    fine["side-cover-left"]["profile"] = [[-0.078, 0.485], [-0.022, 0.485], [-0.022, 0.5], [-0.078, 0.5]]
    fine["side-cover-right"]["profile"] = [[-0.078, -0.5], [-0.022, -0.5], [-0.022, -0.485], [-0.078, -0.485]]
    fine["grille-plate"]["from"] = 0.585
    add_tee(document["volumes"]["100"])
    for scale in ("50", "100"):
        load_path(document["volumes"][scale])
    valve = by_id(document["volumes"]["50"]["structure"])["joint-valve-head"]
    valve["profile"] = [[-0.47 if point[0] == -0.48 else point[0], point[1]] for point in valve["profile"]]
    coarse(document["volumes"]["200"], True)
    coarse(document["volumes"]["500"], False)
    rail(document["volumes"]["500"])
    for scale, rows in TEXT.items():
        decisions = document["volumes"][scale]["decisions"]
        if scale == "100":
            document["volumes"]["50"]["decisions"].append({"en": "The four nipple joints of the structure run through both headers, and the headers are members, so the front panel has its load path to the wall brackets.", "de": "Die vier Nippelgelenke der Struktur laufen durch beide Sammler, und die Sammler sind Stäbe; so hat die Frontplatte ihren Lastpfad zu den Wandkonsolen."})
            decisions.append({"en": rows[0][0], "de": rows[0][1]})
        else:
            if len(decisions) != 3:
                raise SystemExit(scale + ": unexpected decisions")
            document["volumes"][scale]["decisions"] = [{"en": en, "de": de} for en, de in rows]
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
