"""📐️ Third-party oracle of the angle and circular hollow section properties: sectionproperties (finite elements, sharp corners) for A, I_u, I_v, the principal angle, W_el,u/v, W_pl,u/v and I_t of angles and CHS. Writes `🔣️.json` beside it (run `python3 -I` with the sectionproperties venv; the Rust unit tests in `parametric_sections` embed the same values)."""
import json
import math
import pathlib

from sectionproperties.analysis import Section
from sectionproperties.pre.library import angle_section, circular_hollow_section

ANGLES = [(100.0, 65.0, 8.0), (100.0, 100.0, 10.0), (150.0, 90.0, 12.0)]
CHS = [(168.3, 6.3), (114.3, 4.0), (60.3, 2.9)]


def analyse(geometry, mesh):
    geometry.create_mesh(mesh_sizes=[mesh])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    section.calculate_plastic_properties()
    return section


def main():
    rows = []
    for h, b, t in ANGLES:
        s = analyse(angle_section(d=h, b=b, t=t, r_r=0, r_t=0, n_r=1), t * t / 16)
        i11, i22 = s.get_ip()
        zp = s.get_zp()
        sp = s.get_sp()
        rows.append({"kind": "angle", "h": h, "b": b, "t": t, "area": s.get_area(), "iu": i11, "iv": i22, "phiDeg": s.get_phi(), "wElU": min(abs(zp[0]), abs(zp[1])), "wElV": min(abs(zp[2]), abs(zp[3])), "wPlU": sp[0], "wPlV": sp[1], "it": s.get_j()})
    for d, t in CHS:
        s = analyse(circular_hollow_section(d=d, t=t, n=128), t / 3)
        ix, _, _ = s.get_ic()
        z = s.get_z()
        sx = s.get_s()
        rows.append({"kind": "chs", "d": d, "t": t, "area": s.get_area(), "i": ix, "wEl": z[0], "wPl": sx[0], "it": s.get_j()})
    out = pathlib.Path(__file__).with_name("🔣️.json")
    out.write_text(json.dumps({"units": "mm", "source": "sectionproperties", "rows": rows}, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    print(json.dumps(rows, indent=1))


if __name__ == "__main__":
    main()
