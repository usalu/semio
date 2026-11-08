"""🧱 Independent EN 1994-1-1 9.7 composite slab rows of a committed snapshot per metre width: bending with partial connection (9.7.2, rigid-plastic stress block, bond force from the m-k test values), longitudinal shear by the m-k method (9.7.3) and vertical shear (9.7.5 through EN 1992-1-1 6.2.2). The design moment and shear come from the caller (EN 1990 6.10 on the PyNite statics). SI throughout. Unverified catalogue assumptions, not clauses: the sheeting area 1.2 t per metre, the rib pitch 0.25 m, f_yp = 280 MPa with gamma_ap 1.0, the bar lever arm 0.9 d with d = h - 30 mm."""

import importlib.util
import math
import pathlib

_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in pathlib.Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)
SHEETING_FYP, REBAR_FSK = 280e6, 500e6
GAMMA_AP, GAMMA_VS, GAMMA_C, GAMMA_S = (FACTORS.ndp(row, "en") for row in ("ndp.en1994.gamma-ap", "ndp.en1994.gamma-vs", "ndp.en1994.gamma-c", "ndp.en1994.gamma-s"))
DEVELOPED, RIB_PITCH = 1.2, 0.25


class Slab:
    """🧩 Section constants of one metre of a composite slab."""

    def __init__(self, slab):
        self.s = slab
        sheeting = slab["sheeting"]
        self.h, self.hp = slab["concreteThicknessM"], sheeting["heightM"]
        self.ap = DEVELOPED * sheeting["thicknessM"]
        self.e = self.ep = self.hp / 2
        self.fyp = SHEETING_FYP / GAMMA_AP
        self.fcd = slab["fCkPa"] / GAMMA_C
        self.dp = self.h - self.e
        self.ncf = self.ap * self.fyp
        self.mpa = 0.5 * self.ap * 0.9 * self.hp * self.fyp
        self.ls = slab["spanM"] / 4
        self.bo = min(sheeting["ribWidthM"] / RIB_PITCH, 1.0)

    def bending(self, nc):
        """📐 Sagging resistance of the sheeting with the concrete force nc (the lever arm z, the sheeting moment M_pr)."""
        nc = min(max(nc, 0.0), self.ncf)
        x = nc / (0.85 * self.fcd)
        z = self.h - 0.5 * x - self.ep
        mpr = 0.0 if nc >= self.ncf - 1e-9 else min(1.25 * self.mpa * (1 - nc / self.ncf), self.mpa)
        return nc * z + mpr, z

    def longitudinal_shear(self):
        s = self.s
        return self.dp / GAMMA_VS * (s["mFactor"] * 1e6 * self.ap / self.ls + s["kFactor"] * 1e6)

    def bond_force(self):
        if self.s["sheeting"]["ribsParallelToBeam"]:
            return 0.0
        return self.longitudinal_shear() / self.bending(self.ncf)[1] * self.s["spanM"] / 2

    def vertical_shear(self):
        fck = self.s["fCkPa"] / 1e6
        k = min(1 + math.sqrt(200 / (self.dp * 1e3)), 2.0)
        rho = min(self.ap / (self.bo * self.dp), 0.02)
        v_c = 0.18 / GAMMA_C * k * (100 * rho * fck) ** (1 / 3)
        v_min = 0.035 * k ** 1.5 * math.sqrt(fck)
        return max(v_c, v_min) * 1e6 * self.bo * self.dp


def checks(slab, m_ed, v_ed):
    """🧩 The bending, m-k longitudinal shear and vertical shear utilisations of one composite slab."""
    q = Slab(slab)
    d = max(slab["concreteThicknessM"] - 0.03, 0.05)
    m_rd = q.bending(q.bond_force())[0] + slab["asM2PerM"] * REBAR_FSK / GAMMA_S * 0.9 * d
    sid = slab["id"]
    return [
        (f"en1994.9.7.2.mrd.{sid}", m_ed / m_rd),
        (f"en1994.9.7.3.mk.{sid}", v_ed / q.longitudinal_shear()),
        (f"en1994.9.7.5.vrd.{sid}", v_ed / q.vertical_shear()),
    ]


def third_party():
    """🎯️ Spec rows C-34, C-35, C-36 and C-37 against third-party code: the vertical shear V_v,Rd against `structuralcodes` EN 1992-1-1 6.2.2 (`ec2_2004.VRdc`, N_Ed = 0), the sagging resistance with partial connection against the sympy stress block z = h - x/2 - e_p, M_pr = min(1.25 M_pa (1 - N_c/N_cf), M_pa) and the m-k longitudinal shear against sympy; returns the relative deviations, all of which must stay below 1e-9 except the shear (below 1e-9 as well, same formula, other implementation)."""
    import sympy
    from structuralcodes.codes import ec2_2004

    slab = {"id": "s", "concreteThicknessM": 0.13, "fCkPa": 30e6, "spanM": 3.2, "mFactor": 160.0, "kFactor": 0.05, "asM2PerM": 0.0, "sheeting": {"heightM": 0.06, "thicknessM": 0.001, "ribWidthM": 0.12, "ribsParallelToBeam": False}}
    q = Slab(slab)
    out = {}
    fck, d_mm, b_mm = slab["fCkPa"] / 1e6, q.dp * 1e3, q.bo * 1e3
    v_library = ec2_2004.VRdc(fck=fck, d=d_mm, Asl=q.ap * 1e6, bw=b_mm, NEd=0.0, Ac=b_mm * slab["concreteThicknessM"] * 1e3, fcd=fck / 1.5, gamma_c=GAMMA_C)
    out["c37_vrd"] = abs(q.vertical_shear() - v_library) / v_library
    h, e_p, ap, hp = sympy.symbols("h e_p A_p h_p", positive=True)
    m_pa = 0.5 * ap * 0.9 * hp * SHEETING_FYP / GAMMA_AP
    point = {h: q.h, e_p: q.ep, ap: q.ap, hp: q.hp}
    for share in (0.0, 0.3, 0.7, 1.0):
        n_c = share * q.ncf
        x = n_c / (0.85 * q.fcd)
        m_pr = sympy.Min(1.25 * m_pa * (1 - n_c / q.ncf), m_pa).subs(point)
        reference = float(n_c * (q.h - x / 2 - q.ep) + m_pr)
        out[f"c34_c36_mrd_{share}"] = abs(q.bending(n_c)[0] - reference) / max(abs(reference), 1e-9)
    m, k, ap_, ls, dp = sympy.symbols("m k A_p L_s d_p", positive=True)
    shear = dp / GAMMA_VS * (m * 1e6 * ap_ / ls + k * 1e6)
    reference = float(shear.subs({m: slab["mFactor"], k: slab["kFactor"], ap_: q.ap, ls: q.ls, dp: q.dp}))
    out["c35_vlrd"] = abs(q.longitudinal_shear() - reference) / reference
    return out


if __name__ == "__main__":
    import json
    import sys

    deviations = third_party()
    print(json.dumps({"specRows": {"C-34": ["en1994.9.7.2.mrd"], "C-35": ["en1994.9.7.3.mk"], "C-36": ["en1994.9.7.2.mrd"], "C-37": ["en1994.9.7.5", "en1994.9.7.5.vrd"]}, "deviations": deviations}))
    sys.exit(0 if max(deviations.values()) < 1e-9 else 2)
