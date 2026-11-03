#!/usr/bin/env python3
"""🌐️ Third-party validation of the EN 1994 wave-3 catalogue quantities (E1-23..E1-26): numpy eigenvalues for the floor frequency, sectionproperties for the composite section I(n), the shrinkage curvature, the plastic modulus and Iz/J of the I-section. Run with the repository venv: `python -I 🐍️.py`."""
from __future__ import annotations

import importlib.util
import math
import sys
from pathlib import Path

import numpy as np
from sectionproperties.analysis import Section
from sectionproperties.pre import Material
from sectionproperties.pre.library import i_section, rectangular_section

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("catalogue_oracle", HERE.parent / "🧮️catalogue" / "🐍️.py")
oracle = importlib.util.module_from_spec(spec)
sys.modules["catalogue_oracle"] = oracle
spec.loader.exec_module(oracle)

D, B, TF, TW, ECM = 0.300, 0.150, 0.0107, 0.0071, 22e9 * (3.8) ** 0.3


def close(name: str, ours: float, theirs: float, tol: float) -> None:
    rel = abs(ours - theirs) / abs(theirs)
    assert rel <= tol, f"{name}: {ours} vs {theirs} (rel {rel:.2e} > {tol})"
    print(f"ok {name}: {ours:.8g} vs {theirs:.8g} (rel {rel:.1e})")


def frequency_numpy(length: float, ei: float, mass: float, n: int = 600) -> float:
    h = length / (n + 1)
    k = (2 * np.eye(n) - np.eye(n, k=1) - np.eye(n, k=-1)) / (h * h)
    return math.sqrt(np.linalg.eigvalsh(k @ k)[0] * ei / mass) / (2 * math.pi)


def composite_inertia(n: float) -> float:
    steel = i_section(d=D, b=B, t_f=TF, t_w=TW, r=0.0, n_r=2, material=Material("steel", 210e9, 0.3, 0.0, 355e6, "grey"))
    slab = rectangular_section(d=0.100, b=2.0, material=Material("concrete", 210e9 / n, 0.2, 0.0, 30e6, "lightblue")).shift_section(x_offset=B / 2 - 1.0, y_offset=D)
    joined = steel + slab
    joined.create_mesh(mesh_sizes=[2e-4, 4e-3])
    section = Section(geometry=joined, time_info=False)
    section.calculate_geometric_properties()
    return section.get_eic()[0] / 210e9


def main() -> None:
    v = oracle.vectors()
    mass = v["vibration_mass"]
    close("E1-23 f1 numpy eigenvalue", v["vibration_f1"], frequency_numpy(8.0, 210e9 * 6e-4, mass), 1e-4)
    close("E1-23 f1 closed form", math.pi / (2 * 64) * math.sqrt(210e9 * 6e-4 / mass), frequency_numpy(8.0, 210e9 * 6e-4, mass), 1e-4)

    steel = i_section(d=D, b=B, t_f=TF, t_w=TW, r=0.0, n_r=2)
    steel.create_mesh(mesh_sizes=[2e-5])
    s = Section(geometry=steel, time_info=False)
    s.calculate_geometric_properties()
    s.calculate_plastic_properties()
    s.calculate_warping_properties()
    sec = oracle.ipe300()
    close("E1-24 Iz flange pair + web", oracle.flange_inertia_z(sec), s.get_ic()[1], 2e-2)
    close("E1-24 J vs thin-walled sum", (2 * B * TF ** 3 + (D - 2 * TF) * TW ** 3) / 3, s.get_j(), 6e-2)
    close("E1-24 W_pl vs sectionproperties (root radius neglected)", oracle.plastic_modulus(sec), s.get_s()[0], 5e-2)

    bare = dict(sec, a=s.get_area(), iy=s.get_ic()[0])
    for label, n in (("n0", 210e9 / ECM), ("nL", 210e9 / ECM * (1 + 1.1 * 2.5)), ("ns", v["creep_ns"])):
        from_oracle, _ = oracle.second_moment(bare, 0.100, 2.0, 0.100, n)
        close(f"E1-25 composite I({label})", from_oracle, composite_inertia(n), 5e-3)

    close("E1-26 critical temperature, inversion of the ky table", v["fire_crit_mu05"], v["fire_crit_inversion_mu05"], 1.2e-2)
    print("third-party validation OK")


if __name__ == "__main__":
    main()
