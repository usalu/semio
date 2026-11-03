"""📐️ sectionproperties adapter: FE section constants (A, I, J, warping I_w, elastic and plastic moduli, shear centre) for I, RHS, CHS, channel and angle shapes, and M_cr of a doubly symmetric I-beam from those constants; SI in (m), the library works in mm."""

import math

from shapely.geometry import Polygon

from sectionproperties.analysis import Section
from sectionproperties.pre import Geometry, Material
from sectionproperties.pre.pre import DEFAULT_MATERIAL
from sectionproperties.pre.library import angle_section, channel_section, circular_hollow_section, elliptical_hollow_section, i_section, mono_i_section, rectangular_hollow_section, rectangular_section, tee_section

LIBRARY = "sectionproperties"
MM = 1.0e3


def solved(geometry, mesh):
    """🧮️ Mesh the geometry and run the geometric, warping and plastic analyses."""
    geometry.create_mesh(mesh_sizes=[mesh * MM * MM])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    section.calculate_plastic_properties()
    return section


def constants(section):
    """📊️ Constants in SI: m^2, m^4, m^6, m^3 and the principal axes quantities of the centroidal system; `beta_11_*` and `beta_22_*` are the Wagner monosymmetry constants (m) about the principal axes, `pos` with the top and `neg` with the bottom flange in compression."""
    ixx, iyy, ixy = section.get_ic()
    i11, i22 = section.get_ip()
    zxx, _, zyy, _ = section.get_z()
    sxx, syy = section.get_s()
    b11p, b11m, b22p, b22m = section.get_beta_p()
    return {"beta_11_pos": b11p / MM, "beta_11_neg": b11m / MM, "beta_22_pos": b22p / MM, "beta_22_neg": b22m / MM, "A": section.get_area() / MM**2, "Iy": ixx / MM**4, "Iz": iyy / MM**4, "Iyz": ixy / MM**4, "I_major": i11 / MM**4, "I_minor": i22 / MM**4, "It": section.get_j() / MM**4, "Iw": section.get_gamma() / MM**6, "Wel_y": zxx / MM**3, "Wel_z": zyy / MM**3, "Wpl_y": sxx / MM**3, "Wpl_z": syy / MM**3, "y_sc": (section.get_sc()[0] - section.get_c()[0]) / MM, "z_sc": (section.get_sc()[1] - section.get_c()[1]) / MM}


def i_rolled(i):
    """🏗️ Doubly symmetric rolled I or H section with root radius."""
    mm = {k: i[k] * MM for k in ("h", "b", "tf", "tw", "r")}
    return constants(solved(i_section(d=mm["h"], b=mm["b"], t_f=mm["tf"], t_w=mm["tw"], r=mm["r"], n_r=16), i.get("mesh", 1.0e-5)))


def rhs(i):
    """🔲️ Rectangular hollow section with outer corner radius `r_out` and inner radius `r_in`."""
    return constants(solved(rectangular_hollow_section(d=i["h"] * MM, b=i["b"] * MM, t=i["t"] * MM, r_out=i["r_out"] * MM, n_r=16), i.get("mesh", 1.0e-5)))


def chs(i):
    """⭕️ Circular hollow section."""
    return constants(solved(circular_hollow_section(d=i["d"] * MM, t=i["t"] * MM, n=128), i.get("mesh", 1.0e-5)))


def channel(i):
    """🧲️ Parallel-flange channel; the shear centre is measured from the centroid."""
    return constants(solved(channel_section(d=i["h"] * MM, b=i["b"] * MM, t_f=i["tf"] * MM, t_w=i["tw"] * MM, r=i["r"] * MM, n_r=16), i.get("mesh", 1.0e-5)))


def angle(i):
    """📐️ Equal or unequal angle with root radius `r_root` and toe radius `r_toe`; principal moments in `I_major` and `I_minor`."""
    return constants(solved(angle_section(d=i["h"] * MM, b=i["b"] * MM, t=i["t"] * MM, r_r=i["r_root"] * MM, r_t=i["r_toe"] * MM, n_r=16), i.get("mesh", 1.0e-5)))


def mcr_i(i):
    """🦋️ EN 1993-1-1 NCCI SN003 elastic critical moment of a simply supported doubly symmetric I-beam, M_cr = C1 pi^2 E I_z / L^2 sqrt(I_w / I_z + L^2 G I_t / (pi^2 E I_z)), with I_z, I_t, I_w from the FE analysis."""
    c = i_rolled(i)
    e, g, length = i["E"], i["G"], i["L"]
    pi2 = math.pi**2
    return {**{k: c[k] for k in ("Iz", "It", "Iw")}, "M_cr": i["C1"] * pi2 * e * c["Iz"] / length**2 * math.sqrt(c["Iw"] / c["Iz"] + length**2 * g * c["It"] / (pi2 * e * c["Iz"]))}


def tee(i):
    """🔱️ Tee section with flange b x tf on top, stem tw and root radius r; the stem tip is the lowest point."""
    return constants(solved(tee_section(d=i["h"] * MM, b=i["b"] * MM, t_f=i["tf"] * MM, t_w=i["tw"] * MM, r=i["r"] * MM, n_r=16), i.get("mesh", 1.0e-5)))


def mono_i(i):
    """🏗️ Welded mono-symmetric I section: top flange b_t x t_ft, bottom flange b_b x t_fb, web tw, root radius r (0 for sharp welds)."""
    mm = {k: i[k] * MM for k in ("h", "b_t", "b_b", "t_ft", "t_fb", "tw", "r")}
    return constants(solved(mono_i_section(d=mm["h"], b_t=mm["b_t"], b_b=mm["b_b"], t_ft=mm["t_ft"], t_fb=mm["t_fb"], t_w=mm["tw"], r=mm["r"], n_r=16), i.get("mesh", 1.0e-5)))


def ehs(i):
    """🥚️ Elliptical hollow section with outer axes d_x x d_y and wall t."""
    return constants(solved(elliptical_hollow_section(d_x=i["d_x"] * MM, d_y=i["d_y"] * MM, t=i["t"] * MM, n=128), i.get("mesh", 1.0e-5)))


def composite_beam(i):
    """🧩️ EN 1994-1-1 5.4.1 elastic composite section: rolled I beam plus slab of width b_eff and thickness h_c on top, the slab weighted by E_c / E_a (modular ratio n = E_a / E_c); results referred to steel: A_eff, I_eff about the elastic centroid, distance z_na of the elastic neutral axis from the steel bottom fibre."""
    steel = Material("steel", i["E_a"] / 1.0e6, 0.3, 355.0, 7.85e-6, "grey")
    concrete = Material("concrete", i["E_c"] / 1.0e6, 0.2, 30.0, 2.4e-6, "lightgrey")
    mm = {k: i[k] * MM for k in ("h", "b", "tf", "tw", "r", "b_eff", "h_c")}
    beam = i_section(d=mm["h"], b=mm["b"], t_f=mm["tf"], t_w=mm["tw"], r=mm["r"], n_r=16, material=steel)
    slab = rectangular_section(d=mm["h_c"], b=mm["b_eff"], material=concrete).shift_section(x_offset=(mm["b"] - mm["b_eff"]) / 2.0, y_offset=mm["h"])
    geometry = beam + slab
    geometry.create_mesh(mesh_sizes=[i.get("mesh", 1.0e-5) * MM * MM])
    section = Section(geometry)
    section.calculate_geometric_properties()
    ea, (eixx, _, _) = section.get_ea(), section.get_eic()
    return {"n": i["E_a"] / i["E_c"], "A_eff": ea / (i["E_a"] / 1.0e6) / MM**2, "z_na": section.get_c()[1] / MM, "I_eff": eixx / (i["E_a"] / 1.0e6) / MM**4}


def polygon_holes(i):
    """🔷️ Free outline: outer ring and hole rings given as (y, z) vertex lists in m, meshed as one shapely polygon with holes."""
    outer = [(y * MM, z * MM) for y, z in i["outer"]]
    holes = [[(y * MM, z * MM) for y, z in ring] for ring in i["holes"]]
    return constants(solved(Geometry(geom=Polygon(outer, holes), material=DEFAULT_MATERIAL), i.get("mesh", 1.0e-5)))


CHECKS = {
    "en1993.section.i-rolled": i_rolled,
    "en1993.section.rhs": rhs,
    "en1993.section.chs": chs,
    "en1993.section.channel": channel,
    "en1993.section.angle": angle,
    "en1993.6.3.2.mcr-i": mcr_i,
    "en1993.section.tee": tee,
    "en1993.section.mono-i": mono_i,
    "en1993.section.ehs": ehs,
    "en1994.5.4.composite-elastic": composite_beam,
    "en1993.section.rhs-sharp": rhs,
    "en1999.section.rhs-sharp": rhs,
    "en1999.section.polygon-holes": polygon_holes,
    "en1999.section.i": mono_i,
    "en1999.section.rhs": rhs,
    "en1999.section.chs": chs,
    "en1999.section.channel": channel,
    "en1999.section.tee": tee,
    "en1999.section.angle": angle,
    "en1999.section.ehs": ehs,
}


def i_section_free(i):
    """📐️ Doubly symmetric I section without root radius (n_r = 1, r = 0) meshed by the library."""
    mm = {k: i[k] * MM for k in ("h", "b", "tf", "tw")}
    return constants(solved(i_section(d=mm["h"], b=mm["b"], t_f=mm["tf"], t_w=mm["tw"], r=0.0, n_r=1), i.get("mesh", 1.0e-6)))


def buckling_chi(lam, alpha, lam0):
    """📉️ chi = 1 / (phi + sqrt(phi^2 - lambda^2)) capped at 1, phi = (1 + alpha (lambda - lambda_0) + lambda^2) / 2."""
    phi = 0.5 * (1.0 + alpha * (lam - lam0) + lam * lam)
    return min(1.0, 1.0 / (phi + math.sqrt(phi * phi - lam * lam)))


def flexural_buckling(i):
    """🪶️ EN 1999-1-1 6.3.1: A and I_z from the FE section analysis, N_cr = pi^2 E I_z / L^2, lambda and chi from the curve."""
    c = i_section_free(i)
    n_cr = math.pi**2 * i["E"] * c["Iz"] / i["L"] ** 2
    lam = math.sqrt(c["A"] * i["f_o"] / n_cr)
    chi = buckling_chi(lam, i["alpha"], i["lambda_0"])
    return {"A": c["A"], "Iz": c["Iz"], "N_cr": n_cr, "lambda": lam, "chi": chi, "N_b_Rd": i.get("kappa", 1.0) * chi * c["A"] * i["f_o"] / i["gamma_M1"]}


def ltb(i):
    """🦋️ EN 1999-1-1 6.3.2 and EN 1994-1-1 6.4: I_z, I_t, I_w and W_el,y from the FE section analysis (St Venant torsion and warping solved on the mesh), M_cr = sqrt(E I_z (E I_w a^4 + G I_t a^2 + k_s)) with a = pi / L, and chi_LT."""
    c = i_section_free(i)
    a = math.pi / i["L"]
    m_cr = math.sqrt(i["E"] * c["Iz"] * (i["E"] * c["Iw"] * a**4 + i["G"] * c["It"] * a * a + i.get("k_s", 0.0)))
    lam = math.sqrt(c["Wel_y"] * i["f_o"] / m_cr)
    chi = buckling_chi(lam, i["alpha_LT"], i["lambda_0"])
    return {"Iz": c["Iz"], "It": c["It"], "Iw": c["Iw"], "M_cr": m_cr, "lambda_LT": lam, "chi_LT": chi, "M_b_Rd": chi * c["Wel_y"] * i["f_o"] / i["gamma_M1"]}


def weld_group_box(i):
    """🧵️ Box weld group as a thin sharp-cornered rectangular hollow section of wall `t_weld` meshed by the library: L_w = A / t, I_p = (I_xx + I_yy) / t, resultant per length at the corner from the vector sum."""
    t = i.get("t_weld", 2.0e-3)
    c = constants(solved(rectangular_hollow_section(d=(i["d"] + t) * MM, b=(i["b"] + t) * MM, t=t * MM, r_out=0.0, n_r=1), i.get("mesh", 2.0e-6)))
    lw, ip = c["A"] / t, (c["Iy"] + c["Iz"]) / t
    f, e, b, d = i["F"], i["e"], i["b"], i["d"]
    return {"L_w": lw, "I_p": ip, "f_max": math.hypot(-f * e * (d / 2.0) / ip, f / lw + f * e * (b / 2.0) / ip)}


CHECKS |= {
    "en1999.6.3.1.flexural-buckling": flexural_buckling,
    "en1999.6.3.2.ltb": ltb,
    "en1994.6.4.ltb-restrained": ltb,
    "en1993.1-8.4.5.weld-group-box": weld_group_box,
}


def mkappa_elastic(i):
    """📉️ EN 1994-1-1 6.2.1.4 moment-curvature point in the fully elastic range: M = E_a I_eff kappa with I_eff of the uncracked transformed composite section from the library (slab fully compressed, neutral axis `z_na` above the slab bottom is verified by the caller)."""
    c = composite_beam(i)
    return {"M": i["E_a"] * c["I_eff"] * i["kappa"], "z_na": c["z_na"]}


CHECKS |= {"en1994.6.2.1.4.mkappa-elastic": mkappa_elastic}


def drylib():
    """🧭️ The pure-Python dry-run module of this adapter (geometry validation and mesh planning), loaded by path so the validation of the `mechanics.*` cases and the solver share one reading of the inputs."""
    import importlib.util
    from pathlib import Path

    path = Path(__file__).resolve().parent / "🧭️dry-run" / "🐍️.py"
    spec = importlib.util.spec_from_file_location("norm_oracle_sectionproperties_dry_run", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def principal(iy, iz, iyz):
    """🧭️ Major and minor principal second moments by Mohr's circle."""
    centre, radius = 0.5 * (iy + iz), math.hypot(0.5 * (iy - iz), iyz)
    return centre + radius, centre - radius


def solved_polygon(geometry, cells, nu):
    """🧮️ Mesh `geometry` with `cells` cells, run the geometric, warping and plastic analyses and return the constants under the field names of the Rust `SectionConstants`; with `nu > 0` the shear functions use E = 1 + nu so that G = 1 / 2 and the constants stay geometric. Elastic and plastic moduli are only given for nu = 0 (the library does not weight them)."""
    geometry.create_mesh(mesh_sizes=[geometry.calculate_area() / cells])
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
    major, minor = principal(iy, iz, iyz)
    out = {"area": area, "centroid_y": cy, "centroid_z": cz, "iy": iy, "iz": iz, "iyz": iyz, "i_major": major, "i_minor": minor, "j": j, "iw": gamma, "shear_centre_y": scy, "shear_centre_z": scz, "a_vy": avy, "a_vz": avz}
    if not nu:
        zy_pos, zy_neg, zz_pos, zz_neg = section.get_z()
        wply, wplz = section.get_s()
        out |= {"w_el_y_pos": zy_pos, "w_el_y_neg": zy_neg, "w_el_z_pos": zz_pos, "w_el_z_neg": zz_neg, "w_pl_y": wply, "w_pl_z": wplz}
    return out


def material_of(nu):
    """🧪️ A unit material with Poisson's ratio `nu` for the shear functions (the default material when `nu` is 0)."""
    return Material(name="m", elastic_modulus=1.0 + nu, poissons_ratio=nu, yield_strength=1.0, density=1.0, color="w") if nu else DEFAULT_MATERIAL


def analyse_section(i):
    """🔲️ Reference for `semio_s_plugin_norm_mechanics::analyse_section`: the outer ring and hole rings (`outer`, `holes` as [y, z] lists in m, any orientation), Poisson's ratio `nu` and the mesh cell count `cells` solved by the library; quantities carry the Rust field names (SI: m^2, m^4, m^6, m^3). The inputs are validated by the dry run first."""
    dry = drylib()
    plan = dry.plan_analyse_section(i)
    outer, holes = dry.rings_of(i)
    return solved_polygon(Geometry(geom=Polygon(outer, holes), material=material_of(plan["nu"])), plan["cells"], plan["nu"])


def strip(wall):
    """🪶️ The solid strip of one centreline wall: a rectangle of the wall length and thickness centred on the centreline."""
    y1, z1, y2, z2, t = wall
    length = math.hypot(y2 - y1, z2 - z1)
    ny, nz = -(z2 - z1) / length * t / 2.0, (y2 - y1) / length * t / 2.0
    return Polygon([(y1 + ny, z1 + nz), (y2 + ny, z2 + nz), (y2 - ny, z2 - nz), (y1 - ny, z1 - nz)])


THIN_EXPONENTS = {"area": 1, "iy": 1, "iz": 1, "iyz": 1, "i_major": 1, "i_minor": 1, "j": 3, "iw": 1, "a_vy": 1, "a_vz": 1, "centroid_y": 0, "centroid_z": 0, "shear_centre_y": 0, "shear_centre_z": 0}
THIN_SCALES = (1.0, 0.5, 0.25)


def thin_limit(samples):
    """📉️ Second-order Richardson extrapolation of the thin-wall limit: a quantity of order `k` in the wall thickness behaves as q(s) = s^k q0 (1 + c1 s + c2 s^2) for the thickness scale s, so (8 f(1/4) - 6 f(1/2) + f(1)) / 3 with f = q / s^k leaves an error of order s^3."""
    coarse, middle, fine = samples
    return {key: (8.0 * fine[key] / THIN_SCALES[2] ** k - 6.0 * middle[key] / THIN_SCALES[1] ** k + coarse[key] / THIN_SCALES[0] ** k) / 3.0 for key, k in THIN_EXPONENTS.items()}


def analyse_thin_walled(i):
    """🪶️ Reference for `semio_s_plugin_norm_mechanics::analyse_thin_walled`: every centreline wall becomes a solid strip of its thickness, the strips are united (one polygon) and solved by the library. The thin-wall idealisation (centreline, J = sum l t^3 / 3, sectorial coordinates) differs from the solid strips by O(t / l) at the corners and in J, so the strips are solved at the thickness scales 1, 1/2 and 1/4 and extrapolated to the thin-wall limit (`thin_limit`); the limit is what the centreline model computes, so cases compare under the profile tolerance. Returns the constants the Rust kernel derives for walls: area, centroid, second moments, principal moments, J, I_w, shear centre and shear areas, each in its thin-wall limit."""
    from shapely.ops import unary_union

    dry = drylib()
    plan = dry.plan_analyse_thin_walled(i)
    samples = []
    for scale in THIN_SCALES:
        union = unary_union([strip((y1, z1, y2, z2, t * scale)) for y1, z1, y2, z2, t in dry.walls_of(i)])
        if union.geom_type != "Polygon":
            raise ValueError("the wall strips do not unite to one polygon")
        samples.append(solved_polygon(Geometry(geom=union, material=material_of(plan["nu"])), plan["cells"], plan["nu"]))
    return thin_limit(samples)


CHECKS |= {"mechanics.analyse-section": analyse_section, "mechanics.analyse-thin-walled": analyse_thin_walled}


def web_stiffener(i):
    """🦴️ EN 1999-1-1 6.7.6 transverse web stiffener: the effective cross-section (web strip of the length `strip` and thickness `tw` plus one flat bar `width` x `thickness` or two, symmetric) united to one polygon and solved by the library; I_st is the second moment about the centroidal axis along the web, and the stiffness ratio I_st,min / I_st follows the required inertia 1.5 h_w^3 t_w^3 / a^2 below the aspect limit a / h_w and 0.75 h_w t_w^3 at or above it (recalled factors of the input)."""
    from shapely.geometry import box
    from shapely.ops import unary_union

    b, tw, w, t = (i[k] * MM for k in ("strip", "tw", "width", "thickness"))
    parts = [box(-b / 2.0, -tw / 2.0, b / 2.0, tw / 2.0), box(-t / 2.0, tw / 2.0, t / 2.0, tw / 2.0 + w)]
    if i["sides"] == "two":
        parts.append(box(-t / 2.0, -tw / 2.0 - w, t / 2.0, -tw / 2.0))
    geometry = Geometry(geom=unary_union(parts), material=DEFAULT_MATERIAL)
    geometry.create_mesh(mesh_sizes=[i.get("mesh", 1.0e-6) * MM * MM])
    section = Section(geometry)
    section.calculate_geometric_properties()
    inertia = section.get_ic()[0] / MM**4
    hw, a, twm = i["h_w"], i["a"], i["tw"]
    required = i["factor_short"] * hw**3 * twm**3 / a**2 if a / hw < i["aspect"] else i["factor_long"] * hw * twm**3
    return {"A": section.get_area() / MM**2, "centroid": section.get_c()[1] / MM, "I_st": inertia, "I_min": required, "stiffness_ratio": required / inertia}


CHECKS |= {"en1999.6.7.6.web-stiffener": web_stiffener}
