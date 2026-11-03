"""🌍️ groundhog adapter: bearing capacity factors, earth pressure coefficients, CPT correlations and (GPL-3.0, test-only, never linked into shipped code); SI in (Pa, kg/m3, rad-free: angles in degrees), the library works in kPa, kN/m3 and degrees."""

import importlib.abc
import importlib.machinery
import math
import sys
import warnings
from unittest import mock

LIBRARY = "groundhog"
PLOTTING_ROOTS = ("plotly", "pyproj", "jinja2", "requests")


class PlottingStub(importlib.abc.MetaPathFinder, importlib.abc.Loader):
    """🔇️ groundhog 0.15.0 declares no dependencies yet imports plotly, pyproj and jinja2 at module level for plotting and `requests` for downloading soil data only; those roots resolve to inert mocks so the numeric functions import. Nothing else is stubbed."""

    def find_spec(self, name, path, target=None):
        return importlib.machinery.ModuleSpec(name, self, is_package=True) if name.split(".")[0] in PLOTTING_ROOTS else None

    def create_module(self, spec):
        module = mock.MagicMock()
        module.__path__, module.__name__, module.__spec__ = [], spec.name, spec
        return module

    def exec_module(self, module):
        return None


sys.meta_path.append(PlottingStub())
warnings.simplefilter("ignore", UserWarning)

from groundhog.excavations import basic as excavations  # noqa: E402
from groundhog.shallowfoundations import capacity, settlement, stressdistribution  # noqa: E402
from groundhog.siteinvestigation.classification import phaserelations  # noqa: E402
from groundhog.siteinvestigation.insitutests import pcpt_correlations, spt_correlations  # noqa: E402

KPA, MPA = 1.0e3, 1.0e6


def bearing_factors(i):
    """🏗️ Prandtl N_q and the three published N_gamma conventions; EN 1997-1 Annex D.4 uses 2 (N_q - 1) tan(phi'), reported as `Ng_ec7 = Ng_vesic - 4 tan(phi')` because Vesic is 2 (N_q + 1) tan(phi')."""
    phi = i["phi"]
    nq = float(capacity.nq_frictionangle_sand(phi)["Nq [-]"])
    vesic = float(capacity.ngamma_frictionangle_vesic(phi)["Ngamma [-]"])
    return {"Nq": nq, "Nc": (nq - 1.0) / math.tan(math.radians(phi)), "Ng_vesic": vesic, "Ng_meyerhof": float(capacity.ngamma_frictionangle_meyerhof(phi)["Ngamma [-]"]), "Ng_ec7": vesic - 4.0 * math.tan(math.radians(phi))}


def earth_pressure(i):
    """🧱️ Rankine coefficients of a smooth vertical wall with level ground and Poncelet/Coulomb coefficients with wall friction `delta` > 0 for the vertical wall (wall_angle and top_angle zero)."""
    rankine = excavations.earthpressurecoefficients_rankine(phi_eff=i["phi"], wall_angle=0.0, top_angle=0.0)
    out = {"Ka_rankine": float(rankine["KaR [-]"]), "Kp_rankine": float(rankine["KpR [-]"])}
    if i["delta"] > 0.0:
        coulomb = excavations.earthpressurecoefficients_poncelet(phi_eff=i["phi"], interface_friction_angle=i["delta"], wall_angle=0.0, top_angle=0.0)
        out |= {"Ka_coulomb": float(coulomb["KaC [-]"]), "Kp_coulomb": float(coulomb["KpC [-]"])}
    return out


def cpt_friction_angle(i):
    """📈️ Kulhawy and Mayne phi' = 17.6 + 11 log10((q_t/p_a)/sqrt(sigma'_v0/p_a)) from the library, with p_a = 100 kPa."""
    return {"phi_deg": float(pcpt_correlations.frictionangle_sand_kulhawymayne(qt=i["qc"] / MPA, sigma_vo_eff=i["sigma_v0_eff"] / KPA)["Phi [deg]"])}


def cpt_undrained_strength(i):
    """🧪️ s_u = (q_t - sigma_v0) / N_kt (Rad and Lunne), net resistance `q_net` in Pa."""
    return {"s_u": float(pcpt_correlations.undrainedshearstrength_clay_radlunne(qnet=i["q_net"] / MPA, Nk=i["N_k"])["Su [kPa]"]) * KPA}


def cpt_behaviour_index(i):
    """🧭️ Robertson and Wride soil behaviour type index I_c from q_t, f_s, total and effective vertical stress."""
    result = pcpt_correlations.behaviourindex_pcpt_robertsonwride(qt=i["qt"] / MPA, fs=i["fs"] / MPA, sigma_vo=i["sigma_v0"] / KPA, sigma_vo_eff=i["sigma_v0_eff"] / KPA)
    return {"Ic": float(result["Ic [-]"])}


def spt_correction(i):
    """🔨️ N60 = N E_r / 60 C_B C_S C_R (Skempton 1986 factors given as overrides, so no table lookup is hidden in the library) and (N1)60 = C_N N60 with the Liao and Whitman C_N = sqrt(p_a / sigma'_v0), p_a = 100 kPa; the library applies no upper bound to C_N."""
    n60 = spt_correlations.spt_N60_correction(N=i["n"], borehole_diameter=100.0, rod_length=10.0, country="Other", hammertype="Safety", hammerrelease="Free fall", eta_H=i["er"], eta_B=i["cb"], eta_S=i["cs"], eta_R=i["cr"])["N60 [-]"]
    return {"n60": float(n60), "n1_60": float(spt_correlations.overburdencorrection_spt_liaowhitman(N=n60, sigma_vo_eff=i["sigma_v0_eff"] / KPA)["N1 [-]"])}


def boussinesq(i):
    """🌍️ Elastic stress increase below a uniform load from the groundhog stress-distribution module: `strip` (per metre, `x` from the strip edge), `rectangle` (corner point) and `circle` (axis point, Poisson ratio irrelevant for the axial stress)."""
    q, z, shape = i["q"] / KPA, i["z"], i["shape"]
    if shape == "strip":
        value = stressdistribution.stresses_stripload(z=z, x=i.get("x", 0.0), width=i["width"], imposedstress=q)["delta sigma z [kPa]"]
    elif shape == "rectangle":
        value = stressdistribution.stresses_rectangle(imposedstress=q, length=i["length"], width=i["width"], z=z)["delta sigma z [kPa]"]
    else:
        value = stressdistribution.stresses_circle(z=z, footing_radius=i["radius"], imposedstress=q, poissonsratio=0.3)["delta sigma z [kPa]"]
    return {"sigma_z": float(value) * KPA}


def phase_relations(i):
    """💧️ Bulk unit weight of a saturated soil from void ratio and specific gravity, gamma = (G_s + S_r e) gamma_w / (1 + e), buoyant unit weight gamma' = gamma - gamma_w; densities in N/m3."""
    result = phaserelations.bulkunitweight(saturation=i["S_r"], voidratio=i["e"], specific_gravity=i["G_s"], unitweight_water=i["gamma_w"] / KPA)
    return {"gamma": float(result["bulk unit weight [kN/m3]"]) * KPA, "gamma_eff": float(result["effective unit weight [kN/m3]"]) * KPA}


CHECKS = {
    "en1997.D.4.bearing-factors": bearing_factors,
    "en1997.9.earth-pressure": earth_pressure,
    "en1997.2.cpt-friction-angle": cpt_friction_angle,
    "en1997.2.cpt-undrained-strength": cpt_undrained_strength,
    "en1997.2.cpt-behaviour-index": cpt_behaviour_index,
    "en1997.2.spt-correction": spt_correction,
    "en1997.F.boussinesq": boussinesq,
    "en1997.2.phase-relations": phase_relations,
}


def bearing_resistance(i):
    """🏗️ EN 1997-1 Annex D.4 drained bearing resistance: N_q from the library (Prandtl), N_gamma = 2 (N_q - 1) tan(phi') from the library's Vesic value, N_c = (N_q - 1) cot(phi'), the shape, base and load inclination factors of D.4 written out."""
    phi = i["phi"]
    t = math.tan(math.radians(phi))
    nq = float(capacity.nq_frictionangle_sand(phi)["Nq [-]"])
    ng = float(capacity.ngamma_frictionangle_vesic(phi)["Ngamma [-]"]) - 4.0 * t
    nc = (nq - 1.0) / t
    ratio, area = i["B"] / i["L"], i["B"] * i["L"]
    bq = (1.0 - math.radians(i.get("alpha", 0.0)) * t) ** 2
    bc = bq - (1.0 - bq) / (nc * t)
    sq, sg = 1.0 + ratio * math.sin(math.radians(phi)), 1.0 - 0.3 * ratio
    sc = (sq * nq - 1.0) / (nq - 1.0)
    m = (2.0 + ratio) / (1.0 + ratio)
    base = 1.0 - i["H"] / (i["V"] + area * i["c"] / t)
    iq, ig = base**m, base ** (m + 1.0)
    ic = iq - (1.0 - iq) / (nc * t)
    q_ult = i["c"] * nc * bc * sc * ic + i["q"] * nq * bq * sq * iq + 0.5 * i["gamma"] * i["B"] * ng * bq * sg * ig
    return {"N_q": nq, "N_c": nc, "N_gamma": ng, "s_q": sq, "s_gamma": sg, "s_c": sc, "i_q": iq, "i_gamma": ig, "i_c": ic, "b_q": bq, "q_ult": q_ult, "R_k": q_ult * area}


def stress_strain_settlement(i):
    """🏗️ EN 1997-1 Annex F.1 stress-strain settlement of a uniformly loaded rectangle below its centre: the library's rectangle-corner stress `stresses_rectangle` (quarter footing, four corners) summed over 200 midpoint layers down to `z_max`, s = sum sigma_z / E_m dz."""
    n, dz = 200, i["z_max"] / 200
    total = 0.0
    for k in range(n):
        z = (k + 0.5) * dz
        total += 4.0 * float(stressdistribution.stresses_rectangle(imposedstress=i["q"] / KPA, length=i["L"] / 2.0, width=i["B"] / 2.0, z=z)["delta sigma z [kPa]"]) * KPA / i["E_m"] * dz
    mid = 4.0 * float(stressdistribution.stresses_rectangle(imposedstress=i["q"] / KPA, length=i["L"] / 2.0, width=i["B"] / 2.0, z=i["z_max"] / 2.0)["delta sigma z [kPa]"]) * KPA
    return {"s": total, "sigma_z_mid": mid}


def horizontal_coefficient(phi, delta, side):
    """🧱️ Horizontal Coulomb coefficient K cos(delta) of a vertical wall with level ground from the library: `earthpressurecoefficients_poncelet` for delta > 0 and the Rankine coefficients for a smooth wall (delta = 0 is outside the Poncelet validity range)."""
    if delta <= 0.0:
        rankine = excavations.earthpressurecoefficients_rankine(phi_eff=phi, wall_angle=0.0, top_angle=0.0)
        return float(rankine["KaR [-]" if side == "a" else "KpR [-]"])
    coulomb = excavations.earthpressurecoefficients_poncelet(phi_eff=phi, interface_friction_angle=delta, wall_angle=0.0, top_angle=0.0)
    return float(coulomb["KaC [-]" if side == "a" else "KpC [-]"]) * math.cos(math.radians(delta))


def embedded_wall(i):
    """🧱️ Embedded wall with Coulomb coefficients of the library (`earthpressurecoefficients_poncelet`, horizontal component K cos delta) and the same moment balance solved by bisection on a trapezoid sum."""
    ka, kp = horizontal_coefficient(i["phi"], i["delta_a"], "a"), horizontal_coefficient(i["phi"], i["delta_p"], "p")
    h, gamma, q = i["h"], i["gamma"], i["q"]

    def moments(t):
        n = 4000
        ma = mp = 0.0
        for k in range(n):
            z = (k + 0.5) * (h + t) / n
            arm = (h + t - z) if i["kind"] == "cantilever" else (z - i["prop_depth"])
            ma += ka * (gamma * z + q) * arm * (h + t) / n
        for k in range(n):
            z = (k + 0.5) * t / n
            arm = (t - z) if i["kind"] == "cantilever" else (h + z - i["prop_depth"])
            mp += kp * gamma * z * arm * t / n
        return mp - ma

    lo, hi = 1e-3, 20.0 * h
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        lo, hi = (mid, hi) if moments(mid) < 0.0 else (lo, mid)
    return {"K_ah": ka, "K_ph": kp, "t": 0.5 * (lo + hi)}


def oedometer_settlement(i):
    """🧪️ EN 1997-1 Annex F.2 primary consolidation settlement of a normally consolidated layer from the library (`primaryconsolidationsettlement_nc`): s = H_0 / (1 + e_0) C_c log10((sigma'_v0 + d sigma) / sigma'_v0)."""
    result = settlement.primaryconsolidationsettlement_nc(initial_height=i["H0"], initial_voidratio=i["e0"], initial_effective_stress=i["sigma_v0"] / KPA, effective_stress_increase=i["d_sigma"] / KPA, compression_index=i["C_c"])
    return {"s": float(result["delta z [m]"]), "delta_e": float(result["delta e [-]"])}


CHECKS |= {
    "en1997.D.4.bearing-resistance": bearing_resistance,
    "en1997.F.stress-strain-settlement": stress_strain_settlement,
    "en1997.9.embedded-wall": embedded_wall,
    "en1997.F.oedometer-settlement": oedometer_settlement,
}


def settlement_pairs(i):
    """📏️ EN 1997-1 Annex H serviceability pairs: the stress-strain settlement of each footing (`stress_strain_settlement` of this adapter), the angular distortion between the first two footings and the gradient of the least-squares settlement plane (numpy.linalg.lstsq) through the footing positions."""
    import numpy as np

    s = np.array([stress_strain_settlement({"B": i["B"], "L": i["L"], "q": q, "E_m": i["E_m"], "z_max": i["z_max"]})["s"] for q in i["q"]])
    xy = np.array(i["xy"], dtype=float)
    design = np.column_stack([np.ones(len(s)), xy[:, 0], xy[:, 1]])
    plane = np.linalg.lstsq(design, s, rcond=None)[0]
    return {"s_1": float(s[0]), "beta_12": float(abs(s[1] - s[0]) / np.hypot(*(xy[1] - xy[0]))), "tilt": float(np.hypot(plane[1], plane[2]))}


CHECKS |= {"en1997.H.settlement-pairs": settlement_pairs}


def settlement_interaction(i):
    """🤝 EN 1997-1 Annex F.1 and H: the stress-strain settlement of each of several identical footings at its own centre with the stress of every other footing superposed. The stress at an offset point is the signed sum of four rectangle-corner stresses of the library (`stresses_rectangle`), 200 midpoint layers down to `z_max`; plus the angular distortion between the first two footings and the least-squares settlement plane gradient (numpy.linalg.lstsq)."""
    import numpy as np

    xy = np.array(i["xy"], dtype=float)
    a, b, n, dz = i["B"] / 2.0, i["L"] / 2.0, 200, i["z_max"] / 200

    def corner(q, u, v, z):
        if abs(u) < 1e-12 or abs(v) < 1e-12:
            return 0.0
        return math.copysign(1.0, u) * math.copysign(1.0, v) * float(stressdistribution.stresses_rectangle(imposedstress=q / KPA, length=abs(v), width=abs(u), z=z)["delta sigma z [kPa]"]) * KPA

    def at(receiver, loaded):
        px, py = xy[receiver] - xy[loaded]
        q = i["q"][loaded]
        total = 0.0
        for k in range(n):
            z = (k + 0.5) * dz
            total += (corner(q, a - px, b - py, z) - corner(q, -a - px, b - py, z) - corner(q, a - px, -b - py, z) + corner(q, -a - px, -b - py, z)) / i["E_m"] * dz
        return total

    s = np.array([sum(at(k, j) for j in range(len(xy))) for k in range(len(xy))])
    design = np.column_stack([np.ones(len(s)), xy[:, 0], xy[:, 1]])
    plane = np.linalg.lstsq(design, s, rcond=None)[0]
    return {"s_1": float(s[0]), "s_1_alone": at(0, 0), "beta_12": float(abs(s[1] - s[0]) / np.hypot(*(xy[1] - xy[0]))), "tilt": float(np.hypot(plane[1], plane[2]))}


CHECKS |= {"en1997.H.settlement-interaction": settlement_interaction}
