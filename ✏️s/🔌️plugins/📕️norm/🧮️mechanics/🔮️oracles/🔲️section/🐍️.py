"""🔮️ Third-party oracle for the section kernel: the `sectionproperties` finite-element library solves the blessed cross-sections independently of the Rust sources.

Prints `case.quantity value` lines in SI (m): area, centroid (cy, cz), iy = ∫z², iz = ∫y², iyz, j, iw (about the shear centre), shear centre (scy, scz), shear areas (avy, avz), plastic moduli (wply, wplz), elastic moduli. Polygon cases are given as explicit vertex lists (y horizontal, z up); library cases use the `sectionproperties` steel-section constructors (n_r = 16) so that the Rust profile builders are checked against them.
"""
import math
import warnings

from shapely.geometry import Polygon

from sectionproperties.analysis import Section
from sectionproperties.pre import Geometry, Material
from sectionproperties.pre.library import angle_section, channel_section, elliptical_hollow_section, i_section, mono_i_section, rectangular_hollow_section, tee_section

warnings.simplefilter("ignore")
OUT = []


def emit(key, value):
    OUT.append(f"{key} {float(value)!r}")


def analyse(name, geometry, mesh_cells=2500, nu=0.0):
    geometry.create_mesh(mesh_sizes=[geometry.calculate_area() / mesh_cells])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    section.calculate_plastic_properties()
    if nu:
        e = 1.0 + nu
        area, (iy, iz, iyz), j, gamma, (avy, avz) = section.get_ea(e_ref=e), section.get_eic(e_ref=e), section.get_ej(e_ref=e), section.get_egamma(e_ref=e), section.get_eas(e_ref=e)
    else:
        area, (iy, iz, iyz), j, gamma, (avy, avz) = section.get_area(), section.get_ic(), section.get_j(), section.get_gamma(), section.get_as()
    cy, cz = section.get_c()
    scy, scz = section.get_sc()
    zy_pos, zy_neg, zz_pos, zz_neg = section.get_z() if not nu else (0.0, 0.0, 0.0, 0.0)
    wply, wplz = section.get_s() if not nu else (0.0, 0.0)
    for key, value in [("area", area), ("cy", cy), ("cz", cz), ("iy", iy), ("iz", iz), ("iyz", iyz), ("j", j), ("iw", gamma), ("scy", scy), ("scz", scz), ("avy", avy), ("avz", avz)]:
        emit(f"{name}.{key}", value)
    if not nu:
        for key, value in [("wply", wply), ("wplz", wplz), ("wel_y_pos", zy_pos), ("wel_y_neg", zy_neg), ("wel_z_pos", zz_pos), ("wel_z_neg", zz_neg)]:
            emit(f"{name}.{key}", value)


def polygon(outer, holes=()):
    return Geometry(Polygon(outer, list(holes)))


def ring(d, n):
    return [(d / 2 * math.cos(2 * math.pi * k / n), d / 2 * math.sin(2 * math.pi * k / n)) for k in range(n)]


def rect(w, h):
    return [(0, 0), (w, 0), (w, h), (0, h)]


def main():
    analyse("rectangle", polygon(rect(0.07, 0.14)))
    material = Material(name="m", elastic_modulus=1.0 + 0.3, poissons_ratio=0.3, yield_strength=1.0, density=1.0, color="w")
    analyse("rectangle-nu", Geometry(Polygon(rect(0.07, 0.14)), material=material), nu=0.3)
    b, h, tf, tw = 0.1, 0.12, 0.008, 0.006
    x0, x1 = (b - tw) / 2, (b + tw) / 2
    analyse("i-sharp", polygon([(0, 0), (b, 0), (b, tf), (x1, tf), (x1, h - tf), (b, h - tf), (b, h), (0, h), (0, h - tf), (x0, h - tf), (x0, tf), (0, tf)]))
    analyse("channel-sharp", polygon([(0, 0), (0.075, 0), (0.075, 0.011), (0.0085, 0.011), (0.0085, 0.189), (0.075, 0.189), (0.075, 0.2), (0, 0.2)]))
    analyse("angle-sharp", polygon([(0, 0), (0.1, 0), (0.1, 0.01), (0.01, 0.01), (0.01, 0.1), (0, 0.1)]))
    analyse("rhs-sharp", polygon(rect(0.1, 0.06), [[(0.004, 0.004), (0.096, 0.004), (0.096, 0.056), (0.004, 0.056)]]))
    analyse("chs-96", polygon(ring(0.1, 96), [ring(0.09, 96)]))
    analyse("hexagon-hole", polygon(rect(0.1, 0.08), [[(0.02 + 0.015 * math.cos(2 * math.pi * k / 6), 0.04 + 0.015 * math.sin(2 * math.pi * k / 6)) for k in range(6)]]))
    analyse("ipe300", i_section(d=0.3, b=0.15, t_f=0.0107, t_w=0.0071, r=0.015, n_r=16))
    analyse("pfc200", channel_section(d=0.2, b=0.075, t_f=0.0115, t_w=0.0085, r=0.0115, n_r=16))
    analyse("l100x10", angle_section(d=0.1, b=0.1, t=0.01, r_r=0.012, r_t=0.006, n_r=16))
    analyse("t150", tee_section(d=0.15, b=0.15, t_f=0.0107, t_w=0.0071, r=0.012, n_r=16))
    analyse("plate-girder", mono_i_section(d=0.8, b_t=0.3, b_b=0.4, t_ft=0.02, t_fb=0.03, t_w=0.012, r=0.0, n_r=1))
    analyse("rhs-200x100x8", rectangular_hollow_section(d=0.2, b=0.1, t=0.008, r_out=0.016, n_r=16))
    analyse("ehs-300x200x10", elliptical_hollow_section(d_x=0.3, d_y=0.2, t=0.01, n=128), mesh_cells=4000)
    print("\n".join(OUT))


main()
