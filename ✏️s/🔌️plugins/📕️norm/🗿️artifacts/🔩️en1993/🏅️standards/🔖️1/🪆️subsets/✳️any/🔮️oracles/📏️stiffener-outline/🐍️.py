"""📏️ sectionproperties reference for the transverse stiffener outline of EN 1993-1-5 9.2.1(8): I_t, I_w and the polar moment I_p about the attachment point of a flat bar, tee and angle, from the solid outline meshed by the library (SI in, SI out). The mesh is refined until the last step moves I_t, I_p and I_w by less than 0.5 %; run with `python -I`, it prints the converged JSON that the Rust test `the_stiffener_outline_matches_the_converged_sectionproperties_oracle` embeds (tolerance 0.5 %, 1 % for I_w)."""

import json

from shapely.geometry import box
from shapely.ops import unary_union

from sectionproperties.analysis import Section
from sectionproperties.pre import Geometry
from sectionproperties.pre.pre import DEFAULT_MATERIAL

MM = 1.0e3
OUTLINES = {
    "flat": {"depth": 0.1, "thickness": 0.01, "flange_width": 0.0, "flange_thickness": 0.0},
    "tee": {"depth": 0.1, "thickness": 0.01, "flange_width": 0.08, "flange_thickness": 0.012},
    "angle": {"depth": 0.1, "thickness": 0.01, "flange_width": 0.06, "flange_thickness": 0.012},
}


def outline(shape, o):
    h, t, bf, tf = (o[k] * MM for k in ("depth", "thickness", "flange_width", "flange_thickness"))
    parts = [box(-t / 2, 0.0, t / 2, h)]
    if shape == "tee":
        parts.append(box(-bf / 2, h - tf / 2, bf / 2, h + tf / 2))
    if shape == "angle":
        parts.append(box(-t / 2, h - tf / 2, bf, h + tf / 2))
    return unary_union(parts)


def reference(shape, o, mesh=2.0e-7):
    geometry = Geometry(geom=outline(shape, o), material=DEFAULT_MATERIAL)
    geometry.create_mesh(mesh_sizes=[mesh * MM * MM])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    ixx, iyy, _ = section.get_ic()
    area, (cy, cz) = section.get_area(), section.get_c()
    return {"A": area / MM**2, "It": section.get_j() / MM**4, "Iw": section.get_gamma() / MM**6, "Ip": (ixx + iyy + area * (cy**2 + cz**2)) / MM**4}


MESHES = (2.0e-6, 5.0e-7, 2.0e-7)


def converged(tolerance=5.0e-3):
    """🔬️ Mesh refinement study: the last refinement step changes I_t, I_p and I_w by less than `tolerance`, so the pinned values are within that of the converged finite-element solution."""
    out = {}
    for shape, o in OUTLINES.items():
        coarse, fine = reference(shape, o, MESHES[-2]), reference(shape, o, MESHES[-1])
        for key in ("It", "Ip", "Iw"):
            step = abs(fine[key] - coarse[key]) / abs(fine[key])
            assert step < tolerance, f"{shape} {key} moved {step:.2e} in the last refinement"
        out[shape] = fine
    return out


if __name__ == "__main__":
    print(json.dumps(converged(), indent=1))
