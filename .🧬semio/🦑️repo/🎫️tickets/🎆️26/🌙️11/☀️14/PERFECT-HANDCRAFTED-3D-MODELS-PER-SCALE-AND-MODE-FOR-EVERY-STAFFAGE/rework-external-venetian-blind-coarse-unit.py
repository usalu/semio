"""🪟 components/external-venetian-blind: the sealed unit of 1:200 and 1:500 without the 16 mm edge spacer ring of 1:50.

A 12 x 16 mm spacer bar is below the smallest feature of both scales (30 and 75 mm) and hidden in the frame rebate. At 1:200 the unit is two 4 mm panes around one cavity body that runs to the edge of the glass (1.13 x 1.18 m: gas space and edge seal as one); at 1:500 the cavity stays the gas space alone (1.106 x 1.156 m) and the 12 mm rim in the frame rebate is left to the frame. One-shot on `volumes/components/external-venetian-blind.json`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, drop, load, save, say

ASSET = "components/external-venetian-blind"
OLD = "The sealed unit retains two 4 mm panes and the 16 mm cavity and edge spacer across its 1.13 × 1.18 m opening."
TEXT = {
    "200": ("The sealed unit is two 4 mm panes around one 16 mm cavity body that runs to the glass edge (1.13 × 1.18 m): gas space and edge seal are one body, the 12 mm spacer bar is below the smallest feature of this scale.",
            "Die Isolierglasscheibe besteht aus zwei 4-mm-Gläsern um einen 16 mm tiefen Zwischenraumkörper, der bis zur Glaskante reicht (1,13 × 1,18 m): Gasraum und Randverbund sind ein Körper, der 12 mm breite Abstandhalter liegt unter dem kleinsten Merkmal dieses Maßstabs"),
    "500": ("The sealed unit is two 4 mm panes around the 16 mm gas space (1.106 × 1.156 m); the 12 mm rim of the edge seal lies in the frame rebate and is not modelled at this scale.",
            "Die Isolierglasscheibe besteht aus zwei 4-mm-Gläsern um den 16 mm tiefen Gasraum (1,106 × 1,156 m); der 12 mm breite Rand des Randverbunds liegt im Rahmenfalz und wird in diesem Maßstab nicht modelliert"),
}


def main():
    """▶️ Drops the spacer ring from body and thermal, widens the cavity to the glass edge and rewrites the sentence."""
    document, indented = load(ASSET)
    for scale in ("200", "500"):
        volumes = document["volumes"][scale]
        drop(volumes["body"], {"edge-spacer"})
        drop(volumes["thermal"], {"th-edge-spacer"})
        cavity = by_id(volumes["thermal"])["th-cavity"]
        if scale == "200":
            for key in ("profile", "plane", "from", "to", "holes"):
                cavity.pop(key, None)
            cavity.update({"kind": "box", "size": [0.016, 1.18, 1.13], "transform": {"translation": [-0.188, 1.625, 0.0]}})
        elif cavity["size"] != [0.016, 1.156, 1.106]:
            raise SystemExit("500: cavity is not the gas space")
        hit = [d for d in volumes["decisions"] if d["en"].startswith(OLD)]
        if len(hit) != 1:
            raise SystemExit(scale + ": sentence not found")
        hit[0]["en"] = hit[0]["en"].replace(OLD, TEXT[scale][0])
        hit[0]["de"] = TEXT[scale][1] + hit[0]["de"][hit[0]["de"].find(". "):]
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
