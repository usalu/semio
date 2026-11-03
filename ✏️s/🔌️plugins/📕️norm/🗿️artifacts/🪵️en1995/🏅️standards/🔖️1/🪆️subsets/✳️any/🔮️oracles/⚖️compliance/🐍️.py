#!/usr/bin/env python3
"""🪵️ Independent EN 1995 evaluate oracle over the hierarchical SI snapshot.

Reads `{annex, members[], connections[]}` (camelCase, SI base units, `actions[]` per item), derives
characteristic internals per action, builds EN 1990 eq. 6.10 ULS plus SLS characteristic and
quasi-permanent combinations, and emits `{"checks":[{"id","utilization"}]}` with the same check ids as
the Rust evaluator. Written from EN 1995-1-1, EN 1995-1-2, EN 1995-2 and EN 1990 Annex A1 — it imports
nothing from the Rust it is compared against.

Links: https://eurocodes.jrc.ec.europa.eu/EN-Eurocodes/eurocode-5-design-timber-structures
"""
from __future__ import annotations

import importlib.util
import json
import math
import sys
from pathlib import Path

MPA = 1e6
GPA = 1e9
REGISTRY = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())
_NDP: dict = {}


def ndp(row: str, annex: str) -> float:
    """📖 Value of the registry row `ndp.en1995.<row>` for the annex: the very row the Rust reads, so a constant resting on a recollection is decided in one place and stays `unverified` there."""
    if not _NDP:
        _NDP.update({e["id"]: e for e in json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]})
    entry = _NDP[f"ndp.en1995.{row}"]
    return float(entry["naDe"] if annex == "de" else entry["recommended"])


def _solid(fm, ft0, ft90, fc0, fc90, fv, e0, e05, e90, g, rho, product):
    """🧱️ One EN 338:2016 / EN 14080 row in SI units."""
    return dict(fm=fm * MPA, ft0=ft0 * MPA, ft90=ft90 * MPA, fc0=fc0 * MPA, fc90=fc90 * MPA, fv=fv * MPA,
                e0=e0 * GPA, e05=e05 * GPA, e90=e90 * GPA, g=g * GPA, rho=rho, product=product)


ROWS = {
    "C14": (14, 7.2, 0.4, 16, 2.0, 3.0, 7.0, 4.7, 0.23, 0.44, 290, "solid"),
    "C16": (16, 8.5, 0.4, 17, 2.2, 3.2, 8.0, 5.4, 0.27, 0.50, 310, "solid"),
    "C18": (18, 10, 0.4, 18, 2.2, 3.4, 9.0, 6.0, 0.30, 0.56, 320, "solid"),
    "C20": (20, 11.5, 0.4, 19, 2.3, 3.6, 9.5, 6.4, 0.32, 0.59, 330, "solid"),
    "C22": (22, 13, 0.4, 20, 2.4, 3.8, 10.0, 6.7, 0.33, 0.63, 340, "solid"),
    "C24": (24, 14.5, 0.4, 21, 2.5, 4.0, 11.0, 7.4, 0.37, 0.69, 350, "solid"),
    "C27": (27, 16.5, 0.4, 22, 2.5, 4.0, 11.5, 7.7, 0.38, 0.72, 360, "solid"),
    "C30": (30, 19, 0.4, 24, 2.7, 4.0, 12.0, 8.0, 0.40, 0.75, 380, "solid"),
    "C35": (35, 22.5, 0.4, 25, 2.7, 4.0, 13.0, 8.7, 0.43, 0.81, 390, "solid"),
    "C40": (40, 26, 0.4, 27, 2.8, 4.0, 14.0, 9.4, 0.47, 0.88, 400, "solid"),
    "C45": (45, 30, 0.4, 29, 2.9, 4.0, 15.0, 10.1, 0.50, 0.94, 410, "solid"),
    "C50": (50, 33.5, 0.4, 30, 3.0, 4.0, 16.0, 10.7, 0.53, 1.00, 430, "solid"),
    "GL20H": (20, 16, 0.5, 20, 2.5, 3.5, 8.4, 7.0, 0.30, 0.65, 340, "glulam"),
    "GL22H": (22, 17.6, 0.5, 22, 2.5, 3.5, 10.5, 8.8, 0.30, 0.65, 370, "glulam"),
    "GL24H": (24, 19.2, 0.5, 24, 2.5, 3.5, 11.5, 9.6, 0.30, 0.65, 385, "glulam"),
    "GL26H": (26, 20.8, 0.5, 26, 2.5, 3.5, 12.1, 10.1, 0.30, 0.65, 405, "glulam"),
    "GL28H": (28, 22.3, 0.5, 28, 2.5, 3.5, 12.6, 10.5, 0.30, 0.65, 425, "glulam"),
    "GL30H": (30, 24, 0.5, 30, 2.5, 3.5, 13.6, 11.3, 0.30, 0.65, 430, "glulam"),
    "GL32H": (32, 25.6, 0.5, 32, 2.5, 3.5, 14.2, 11.8, 0.30, 0.65, 440, "glulam"),
    "GL20C": (20, 11.5, 0.35, 15.5, 2.4, 3.5, 10.4, 8.6, 0.30, 0.65, 335, "glulam"),
    "GL22C": (22, 13, 0.35, 18.5, 2.4, 3.5, 10.4, 8.6, 0.30, 0.65, 355, "glulam"),
    "GL24C": (24, 17, 0.5, 21.5, 2.5, 3.5, 11.0, 9.1, 0.30, 0.65, 350, "glulam"),
    "GL26C": (26, 19, 0.5, 23, 2.5, 3.5, 11.5, 9.6, 0.30, 0.65, 385, "glulam"),
    "GL28C": (28, 19.5, 0.5, 24, 2.5, 3.5, 12.5, 10.4, 0.30, 0.65, 390, "glulam"),
    "GL30C": (30, 19.5, 0.5, 24.5, 2.5, 3.5, 13.0, 10.8, 0.30, 0.65, 390, "glulam"),
    "GL32C": (32, 19.5, 0.5, 24.5, 2.5, 3.5, 13.5, 11.2, 0.30, 0.65, 400, "glulam"),
    "LVL32": (36, 26, 0.9, 35, 6.0, 4.5, 14.0, 11.6, 0.5, 0.6, 510, "lvl"),
    "CLT100": (24, 14.5, 0.4, 21, 2.5, 4.0, 11.0, 7.4, 0.37, 0.69, 350, "clt"),
}
KEYS = ("fm", "ft0", "ft90", "fc0", "fc90", "fv", "e0", "e05", "e90", "g", "rho", "product")
STRENGTH = {}
for _name, _row in ROWS.items():
    _d = dict(zip(KEYS, _row))
    for _k in ("fm", "ft0", "ft90", "fc0", "fc90", "fv"):
        _d[_k] *= MPA
    for _k in ("e0", "e05", "e90", "g"):
        _d[_k] *= 1e9 if _k != "g" else 1e9
    STRENGTH[_name] = _d
ALIASES = {"LVL-32": "LVL32", "CLT": "CLT100"}

DURATIONS = ("permanent", "long", "medium", "short", "instantaneous")
K_MOD = {1: (0.60, 0.70, 0.80, 0.90, 1.10), 2: (0.60, 0.70, 0.80, 0.90, 1.10), 3: (0.50, 0.55, 0.65, 0.70, 0.90)}
K_DEF = {1: 0.60, 2: 0.80, 3: 2.00}
K_DEF_PRODUCT = {"solid": (0.6, 0.8, 2.0), "glulam": (0.6, 0.8, 2.0), "lvl": (0.6, 0.8, 2.0), "clt": (0.6, 0.8, 2.0)}
K_FI = {"solid": 1.25, "glulam": 1.15, "lvl": 1.1, "clt": 1.15}
BETA_N_MM_PER_MIN = {"solid": 0.80, "glulam": 0.70, "lvl": 0.70, "clt": 0.65}
_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)


def props(cls):
    """🔎️ Tabulated properties for a strength class, `None` when unknown."""
    key = str(cls).strip().upper().replace(" ", "")
    return STRENGTH.get(ALIASES.get(key, key))


def annex_of(doc):
    """🌍️ `en` or `de` from the snapshot's annex spelling."""
    return "de" if str(doc.get("annex", "En")).strip().lower() == "de" else "en"


def num(obj, key, default=0.0):
    """🔢️ Float field with a default for absent or null values."""
    value = obj.get(key)
    return default if value is None else float(value)


def duration_rank(text):
    """⏱️ Index into `DURATIONS`; unknown spellings read as medium."""
    t = str(text or "").strip().lower()
    t = {"perm": "permanent", "inst": "instantaneous"}.get(t, t)
    return DURATIONS.index(t) if t in DURATIONS else DURATIONS.index("medium")


def service_class(value):
    """🌧️ SC1–SC3, anything else reads as SC1."""
    v = int(value or 1)
    return v if v in (2, 3) else 1


def k_mod(sc, rank):
    """🧮️ EN 1995-1-1 Table 3.1."""
    return K_MOD[service_class(sc)][rank]


def gamma_m(annex, product):
    if annex == "de":
        return 1.3
    """🛡️ EN 1995-1-1 Table 2.3 and DIN EN 1995-1-1/NA: solid 1.3, glulam 1.25, LVL 1.2, CLT 1.25 (annex-independent for these products)."""
    return {"glulam": 1.25, "lvl": 1.2, "clt": 1.25}.get(product, 1.3)


def k_cr(annex, fv, product="solid"):
    """✂️ Crack factor: 1.0 for LVL/CLT; solid and glulam EN 0.67, DIN EN 1995-1-1/NA min(1, 2.5/f_v,k[MPa])."""
    if product in ("lvl", "clt"):
        return 1.0
    if annex == "en":
        return 0.67
    f = fv / MPA
    return 1.0 if f <= 0 else min(1.0, 2.5 / f)


def k_h(h, product, rho):
    """📏️ Depth factor §3.2 / §3.3 / §3.4 (LVL exponent 0.12); solid timber above the registry density limit `ndp.en1995.kh.density-limit` (700 kg/m3) has none."""
    hmm = max(h * 1000.0, 1.0)
    if product == "solid" and rho > ndp("kh.density-limit", "en"):
        return 1.0
    if product == "solid":
        return min(max((150.0 / hmm) ** 0.2, 1.0), 1.3)
    if product == "lvl":
        return min(max((300.0 / hmm) ** 0.12, 1.0), 1.2)
    if product == "clt":
        return 1.0
    return min(max((600.0 / hmm) ** 0.1, 1.0), 1.1)


def k_crit(lam):
    """🌀️ Lateral torsional buckling §6.3.3 eq. 6.34."""
    if lam <= 0.75:
        return 1.0
    if lam <= 1.4:
        return 1.56 - 0.75 * lam
    return 1.0 / (lam * lam)


def lambda_rel_m(w, fm, mcrit):
    """📐️ Relative slenderness for bending."""
    return 0.0 if mcrit <= 0 or w <= 0 or fm <= 0 else math.sqrt(w * fm / mcrit)


def effective_m_crit_nm(member, p):
    """🪢 M_crit = π√(E0,05·Iz·G0,05·Itor)/(l_ef·Wy) with G0,05 = E0,05/16 and the Saint-Venant constant of a rectangle Itor = h·b³·(1/3 − 0.21·b/h·(1 − b⁴/(12·h⁴)))."""
    b, h, span = num(member, "bM"), num(member, "hM"), max(num(member, "spanM"), 1e-6)
    spacing = num(member, "lateralRestraintSpacingM")
    l_ef = min(spacing, span) if spacing > 0 else span
    e = p["e05"]
    sigma = math.pi * math.sqrt(e * (h * b ** 3 / 12) * (e / 16) * (h * b ** 3 * (1 / 3 - 0.21 * b / h * (1 - b ** 4 / (12 * h ** 4))))) / (l_ef * (b * h * h / 6))
    return sigma * b * h * h / 6


def k_c(lam_rel, beta_c=0.2):
    """🏛️ Column buckling §6.3.2 with imperfection factor β_c."""
    if lam_rel <= 0.3:
        return 1.0
    k = 0.5 * (1 + beta_c * (lam_rel - 0.3) + lam_rel ** 2)
    return 1.0 / (k + math.sqrt(max(k * k - lam_rel ** 2, 0.0)))


def lambda_rel_c(lam, fc0, e05):
    """📐️ Relative slenderness for compression."""
    return 0.0 if e05 <= 0 else lam / math.pi * math.sqrt(fc0 / e05)


def contact_length(l_mm, overhang_mm):
    """🦶️ §6.1.5 (1): l_ef = l + a1 + a2 with each spread min(30 mm, a, l)."""
    return l_mm + min(30.0, overhang_mm, l_mm) + min(30.0, l_mm)


def k_c90(product, l_mm, h_mm):
    """🧱️ §6.1.5 (4): discrete support — solid 1.5, glulam 1.75 (l ≤ 400 mm); else 1.0."""
    if l_mm <= 400.0 and product == "solid":
        return 1.5
    if l_mm <= 400.0 and product == "glulam":
        return 1.75
    return 1.0


def k_v(h, hn, x, product):
    """🪚️ Notch factor §6.5.2 (6.62), support-side notch: k_n = 4.5 LVL, 5 solid, 6.5 glulam."""
    if hn <= 0 or h <= 0:
        return 1.0
    alpha = (h - hn) / h
    hmm = h * 1000.0
    kn = {"lvl": 4.5, "glulam": 6.5}.get(product, 5.0)
    xr = max(x, 0.0) * 1000.0 / hmm
    den = math.sqrt(hmm) * (math.sqrt(alpha * (1 - alpha)) + 0.8 * xr * math.sqrt(1 / alpha - alpha * alpha))
    return min(1.0, kn / den)


def psi(kind, category, factors):
    """🪵️ (ψ₀, ψ₁, ψ₂) of the registry rows `ndp.en1990.psi.*`: snow by kind and category (`snowHigh`: the row `snow_high` above 1000 m a.s.l.), wind by kind, imposed actions by category letter, anything else or an unknown category follows offices; permanent and accidental actions carry (1, 1, 1)."""
    k = str(kind or "").strip().lower()
    if k in ("permanent", "accidental"):
        return 1.0, 1.0, 1.0
    c = str(category or "").strip().lower()
    key = ("snow_high" if c == "snowhigh" else "snow") if k == "snow" else "wind" if k == "wind" else c.upper() if k == "imposed" and c != "snowhigh" else "office"
    return factors.psi_known(key) or factors.psi("office")


def family(kind):
    """👪️ permanent | accidental | variable."""
    k = str(kind or "").strip().lower()
    if k in ("permanent", "g", "dead"):
        return "permanent"
    if k in ("accidental", "a"):
        return "accidental"
    return "variable"


FORCES = ("m", "v", "n", "nt", "fc90", "q", "f")


def internals(member, action):
    """🧮️ Characteristic internals from q/F and the support, else the analysed forces."""
    q, f = num(action, "qLineNPerM"), num(action, "fPointN")
    l = max(num(member, "spanM"), 0.0)
    if q != 0 or f != 0:
        support = member.get("support", "simplySupported")
        column = member.get("role") == "column"
        qt, ft = q, (0.0 if column else f)
        if support == "cantilever":
            m, v = qt * l * l / 2 + ft * l, qt * l + ft
        elif support == "continuousTwoSpan":
            m, v = qt * l * l / 8 + 3 * ft * l / 16, 5 * qt * l / 8 + 11 * ft / 16
        else:
            m, v = qt * l * l / 8 + ft * l / 4, qt * l / 2 + ft / 2
        n = f if column else 0.0
        return dict(m=m, v=v, n=n, nt=0.0, fc90=max(v, 0.0), q=q, f=ft)
    return dict(m=num(action, "mKNm"), v=num(action, "vKN"), n=num(action, "nKN"), nt=num(action, "nTKN"),
                fc90=num(action, "fC90KN"), q=0.0, f=0.0)


def combine(terms):
    """➕️ Σ factor·internals."""
    out = dict.fromkeys(FORCES, 0.0)
    for factor, i in terms:
        for key in FORCES:
            out[key] += factor * i[key]
    return out


def combinations(member, factors, ab=False):
    """🔀️ EN 1990 eq. 6.10 (or 6.10a and 6.10b with ξ = 0.85 when `ab`) ULS, SLS characteristic / frequent(ψ₁) / quasi-permanent and accidental combinations."""
    actions = member.get("actions") or []
    if not actions:
        return []
    ins = [internals(member, a) for a in actions]
    perm = [i for i, a in enumerate(actions) if family(a.get("kind")) == "permanent"]
    acc = [i for i, a in enumerate(actions) if family(a.get("kind")) == "accidental"]
    var = [i for i, a in enumerate(actions) if family(a.get("kind")) == "variable"]
    gamma_g, gamma_q, xi = factors.gamma_g_sup(), factors.gamma_q(), factors.xi()
    psis = {i: psi(actions[i].get("kind"), actions[i].get("category"), factors) for i in var}

    def shortest(idx):
        return max((duration_rank(actions[i].get("loadDuration")) for i in idx), default=DURATIONS.index("medium"))

    out = []
    if not var:
        out.append(dict(id="uls.g", kind="uls", rank=shortest(perm), psi2=1.0, **combine([(gamma_g, ins[i]) for i in perm])))
        g = combine([(1.0, ins[i]) for i in perm])
        out.append(dict(id="sls.char.g", kind="char", rank=shortest(perm), psi2=1.0, **g))
        out.append(dict(id="sls.freq.g", kind="freq", rank=shortest(perm), psi2=1.0, **g))
        out.append(dict(id="sls.qp.g", kind="qp", rank=shortest(perm), psi2=1.0, **g))
    else:
        rank = shortest(perm + var)
        if ab:
            terms = [(gamma_g, ins[i]) for i in perm] + [(gamma_q * psis[v][0], ins[v]) for v in var]
            out.append(dict(id="uls.6.10a", kind="uls", rank=rank, psi2=0.3, **combine(terms)))
        for lead in var:
            others = [v for v in var if v != lead]
            uls = [(gamma_g * (xi if ab else 1.0), ins[i]) for i in perm] + [(gamma_q, ins[lead])] + [(gamma_q * psis[v][0], ins[v]) for v in others]
            out.append(dict(id=f"uls.{'6.10b' if ab else '6.10'}.lead.{actions[lead]['id']}", kind="uls", rank=rank, psi2=psis[lead][2], **combine(uls)))
        for d in sorted({duration_rank(actions[i].get("loadDuration")) for i in perm + var}):
            sub = [v for v in var if duration_rank(actions[v].get("loadDuration")) <= d]
            if len(sub) == len(var):
                continue
            if not sub:
                out.append(dict(id=f"uls.dur.{d}.g", kind="uls", rank=d, psi2=1.0, **combine([(gamma_g, ins[i]) for i in perm])))
                continue
            for lead in sub:
                terms = [(gamma_g * (xi if ab else 1.0), ins[i]) for i in perm] + [(gamma_q, ins[lead])] + [(gamma_q * psis[v][0], ins[v]) for v in sub if v != lead]
                out.append(dict(id=f"uls.dur.{d}.lead.{actions[lead]['id']}", kind="uls", rank=d, psi2=psis[lead][2], **combine(terms)))
        for lead in var:
            others = [v for v in var if v != lead]
            char = [(1.0, ins[i]) for i in perm] + [(1.0, ins[lead])] + [(psis[v][0], ins[v]) for v in others]
            out.append(dict(id=f"sls.char.lead.{actions[lead]['id']}", kind="char", rank=rank, psi2=psis[lead][2], **combine(char)))
        for lead in var:
            others = [v for v in var if v != lead]
            freq = [(1.0, ins[i]) for i in perm] + [(psis[lead][1], ins[lead])] + [(psis[v][2], ins[v]) for v in others]
            out.append(dict(id=f"sls.freq.lead.{actions[lead]['id']}", kind="freq", rank=rank, psi2=psis[lead][1], **combine(freq)))
        qp = [(1.0, ins[i]) for i in perm] + [(psis[v][2], ins[v]) for v in var]
        out.append(dict(id="sls.qp", kind="qp", rank=rank, psi2=max(psis[v][2] for v in var), **combine(qp)))
    if acc:
        terms = [(1.0, ins[i]) for i in perm + acc] + [(psis[v][2], ins[v]) for v in var]
        out.append(dict(id="accidental", kind="acc", rank=DURATIONS.index("instantaneous"), psi2=0.0, **combine(terms)))
    return out


def last_max(items, key):
    """🏁️ The LAST item with the largest key — ties resolve to the later combination."""
    best = None
    for item in items:
        if best is None or key(item) >= key(best):
            best = item
    return best


def connection_uls(conn, factors):
    """🔗️ Governing (rank, F_Ed) over the connection's ULS combinations, `None` without actions."""
    actions = conn.get("actions") or []
    perm = [a for a in actions if family(a.get("kind")) == "permanent"]
    var = [a for a in actions if family(a.get("kind")) == "variable"]
    if not actions:
        return None
    g = sum(num(a, "fKN") for a in perm)
    rank = max((duration_rank(a.get("loadDuration")) for a in perm + var), default=DURATIONS.index("medium"))
    if not var:
        return rank, factors.gamma_g_sup() * g
    candidates = []
    for lead in var:
        fed = factors.gamma_g_sup() * g + factors.gamma_q() * num(lead, "fKN")
        fed += sum(factors.gamma_q() * psi(v.get("kind"), "", factors)[0] * num(v, "fKN") for v in var if v is not lead)
        candidates.append(fed)
    return rank, last_max(candidates, abs)


def deflection(member, p, q, f):
    """📉️ Mid-span / tip deflection for the member's support idealisation."""
    ei = p["e0"] * num(member, "bM") * num(member, "hM") ** 3 / 12
    l = num(member, "spanM")
    if ei <= 0 or l <= 0:
        return 0.0
    support = member.get("support", "simplySupported")
    if support == "cantilever":
        return q * l ** 4 / (8 * ei) + f * l ** 3 / (3 * ei)
    if support == "continuousTwoSpan":
        return q * l ** 4 / (185 * ei) + f * l ** 3 / (100 * ei)
    return 5 * q * l ** 4 / (384 * ei) + f * l ** 3 / (48 * ei)


def util(demand, capacity):
    """⚖️ Demand over capacity."""
    return demand / capacity if capacity else float("inf")


def fire_check(member, p, m_ed):
    """🔥️ EN 1995-1-2 §4.2.2 reduced cross-section, k_mod,fi = γ_M,fi = 1, d_ef = β_n·t + k0·d0."""
    t_min = num(member, "fireDurationS") / 60.0
    d_ef = (BETA_N_MM_PER_MIN[p["product"]] * t_min + min(1.0, t_min / 20.0) * 7.0) / 1000.0
    b_fi, h_fi = max(num(member, "bM") - 2 * d_ef, 0.0), max(num(member, "hM") - 2 * d_ef, 0.0)
    m_rd = K_FI[p["product"]] * p["fm"] * b_fi * h_fi ** 2 / 6
    return {"id": f"en1995.1-2.4.fire.{member['id']}", "utilization": util(abs(m_ed), max(m_rd, 1e-9))}


def single_pedestrian(f1, modal_mass, zeta, vertical):
    """🚶️ EN 1995-2 §7.3 acceleration of one pedestrian."""
    k = (200.0 if f1 <= 2.5 else 100.0) if vertical else 50.0
    return k / (modal_mass * zeta)


def bridge_checks(annex, reliability_class, member, p):
    """🌉️ EN 1995-2 §6.2 Miner fatigue, §7.3 pedestrian comfort, ULS bending and SLS deflection."""
    mid = member["id"]
    b, h, l = num(member, "bM"), num(member, "hM"), num(member, "spanM")
    w = b * h * h / 6
    km = k_mod(member.get("serviceClass"), DURATIONS.index("medium"))
    g = gamma_m(annex, p["product"])
    q = max(num(member, "bridgeCrowdPerM2"), 0.0) * 800 * max(b, 0.05)
    coefficient = {"cantilever": 1 / 2}.get(member.get("support"), 1 / 8)
    m_crowd = q * l * l * coefficient
    m_ed = FACTORS.ActionFactors(annex, reliability_class=reliability_class, domain="A2").gamma_q("footbridge_crowd") * m_crowd
    fmd = km * k_h(h, p["product"], p["rho"]) * k_crit(lambda_rel_m(w, p["fm"], effective_m_crit_nm(member, p))) * p["fm"] / g
    sigma_m = m_ed / w if w > 0 else 0.0
    delta = 0.4 * m_crowd / w if w > 0 else 0.0
    cycles = max(num(member, "bridgeNObs") * num(member, "bridgeTLYears"), 1.0)
    a_w = num(member, "bridgeA") if num(member, "bridgeA") > 0 else 15.0
    b_w = num(member, "bridgeB") if num(member, "bridgeB") > 0 else 4.0
    n_r = max(10 ** (a_w - b_w * math.log10(max(delta / MPA, 1e-6))), 1.0)
    ei = p["e0"] * b * h ** 3 / 12
    mu = num(member, "massKgPerM") if num(member, "massKgPerM") > 0 else p["rho"] * b * h
    f1 = math.pi / (2 * l * l) * math.sqrt(ei / mu) if mu > 0 and ei > 0 and l > 0 else 1.0
    f_lat = math.pi / (2 * l * l) * math.sqrt(p["e0"] * h * b ** 3 / 12 / mu) if mu > 0 and l > 0 else 1.0
    zeta = num(member, "dampingXi") if num(member, "dampingXi") > 0 else 0.01
    n_peds = max(num(member, "bridgeCrowdPerM2"), 0.0) * l * max(b, 0.05)
    modal = mu * l / 2
    crowd = lambda a1, factor: factor * a1 * n_peds if n_peds > 1.0 else a1
    a_vert = 0.0 if f1 > 5.0 else crowd(single_pedestrian(f1, modal, zeta, True), 0.23)
    a_hor = 0.0 if f_lat > 2.5 else crowd(single_pedestrian(f_lat, modal, zeta, False), 0.18)
    out = [
        {"id": f"en1995.2.a.fatigue.{mid}", "utilization": cycles / n_r},
        {"id": f"en1995.2.b.avert.{mid}", "utilization": util(a_vert, 0.7)},
        {"id": f"en1995.2.b.ahor.{mid}", "utilization": util(a_hor, 0.2)},
        {"id": f"en1995.2.uls.bending.{mid}", "utilization": util(sigma_m, max(fmd, 1e-9))},
    ]
    if l > 0:
        out.append({"id": f"en1995.2.sls.deflection.{mid}", "utilization": util(deflection(member, p, q, 0.0), l / 400)})
    return out


def worst(items):
    """🏁️ Largest demand/capacity ratio of (demand, capacity) pairs; the last wins ties."""
    best = None
    for demand, capacity in items:
        r = demand / max(capacity, 1e-9)
        if best is None or r >= best:
            best = r
    return best


def member_checks(annex, reliability_class, member, ab=False):
    """🪵️ All EN 1995-1-1 / 1-2 / 2 checks one member earns, every ULS combination checked per effect."""
    mid = member["id"]
    p = props(member.get("strengthClass", ""))
    if p is None:
        return [{"id": f"en1995.material.unknown.{mid}", "utilization": None}]
    if member.get("role") == "bridge":
        out = bridge_checks(annex, reliability_class, member, p)
        if num(member, "fireDurationS") > 0:
            ulses = [abs(c["m"]) for c in combinations(member, FACTORS.ActionFactors(annex, reliability_class=reliability_class), ab) if c["kind"] == "uls"]
            out.append(fire_check(member, p, max(ulses, default=0.0)))
        return out
    combos = combinations(member, FACTORS.ActionFactors(annex, reliability_class=reliability_class), ab)
    ulses = [c for c in combos if c["kind"] == "uls"]
    if not ulses:
        return []
    b, h = num(member, "bM"), num(member, "hM")
    sc = service_class(member.get("serviceClass"))
    g = gamma_m(annex, p["product"])
    area, w = b * h, b * h * h / 6
    kh_b = k_h(h, p["product"], p["rho"])
    kh_t = k_h(max(b, h) if p["product"] == "solid" else b, p["product"], p["rho"])
    kcrit = k_crit(lambda_rel_m(w, p["fm"], effective_m_crit_nm(member, p)))
    kcr = k_cr(annex, p["fv"], p["product"])
    notch = num(member, "notchDepthM")
    hef = max(h - notch, 1e-6)
    kv = k_v(h, notch, num(member, "notchDistanceM"), p["product"])
    beta_c = 0.2 if p["product"] == "solid" else 0.1
    iy, iz = math.sqrt(b * h ** 3 / 12 / max(area, 1e-12)), math.sqrt(h * b ** 3 / 12 / max(area, 1e-12))
    rel_y = lambda_rel_c(num(member, "bucklingLengthYM") / max(iy, 1e-12), p["fc0"], p["e05"])
    rel_z = lambda_rel_c(num(member, "bucklingLengthZM") / max(iz, 1e-12), p["fc0"], p["e05"])
    kcy, kcz = k_c(rel_y, beta_c), k_c(rel_z, beta_c)
    km = lambda c: k_mod(sc, c["rank"])
    out = [{"id": f"en1995.6.1.6.bending.{mid}", "utilization": worst([(abs(c["m"]) / w, km(c) * kh_b * kcrit * p["fm"] / g) for c in ulses])}]
    out.append({"id": f"en1995.6.1.7.shear.{mid}", "utilization": worst([(1.5 * abs(c["v"]) / (kcr * b * hef), kv * km(c) * p["fv"] / g) for c in ulses])})
    if any(c["nt"] > 0 for c in ulses):
        out.append({"id": f"en1995.6.2.3.tension.{mid}", "utilization": worst([(c["nt"] / area / (km(c) * kh_t * p["ft0"] / g) + abs(c["m"]) / w / (km(c) * kh_b * p["fm"] / g), 1.0) for c in ulses])})
    if any(c["n"] > 0 for c in ulses):
        kc = min(kcy, kcz)
        out.append({"id": f"en1995.6.3.2.compression.{mid}", "utilization": worst([(c["n"] / area, kc * km(c) * p["fc0"] / g) for c in ulses])})

        def combined(c):
            fc, fm = km(c) * p["fc0"] / g, km(c) * kh_b * p["fm"] / g
            sc_d, sm = c["n"] / area, abs(c["m"]) / w
            ltb = (sm / (kcrit * fm)) ** 2 + sc_d / (kcz * fc)
            if rel_y <= 0.3 and rel_z <= 0.3:
                return max((sc_d / fc) ** 2 + sm / fm, ltb)
            return max(sc_d / (kcy * fc) + sm / fm, sc_d / (kcz * fc) + 0.7 * sm / fm, ltb)

        out.append({"id": f"en1995.6.3.2.c.combined.{mid}", "utilization": worst([(combined(c), 1.0) for c in ulses])})
    bearing = num(member, "bearingLengthM")
    if any(c["fc90"] > 0 for c in ulses) and bearing > 0:
        l_mm = bearing * 1000.0
        overhang = max(num(member, "supportLengthM") - bearing, 0.0) * 1000.0
        lef = contact_length(l_mm, overhang)
        k90 = k_c90(p["product"], l_mm, h * 1000.0)
        out.append({"id": f"en1995.6.1.5.c90.{mid}", "utilization": worst([(c["fc90"] / (b * lef / 1000.0), k90 * km(c) * p["fc90"] / g) for c in ulses])})
    if annex == "de" and notch > 0:
        out.append({"id": f"en1995.6.5.2.notch-na.{mid}", "utilization": util(0.5, hef / h)})
    chars = [c for c in combos if c["kind"] == "char"]
    span = num(member, "spanM")
    if chars:
        sls = last_max(chars, lambda c: abs(c["q"]))
        if span > 0 and (sls["q"] != 0 or sls["f"] != 0):
            perm_q = sum(num(a, "qLineNPerM") for a in member.get("actions") or [] if family(a.get("kind")) == "permanent")
            perm_f = sum(num(a, "fPointN") for a in member.get("actions") or [] if family(a.get("kind")) == "permanent")
            w_g = deflection(member, p, perm_q, perm_f)
            w_q = max(deflection(member, p, sls["q"], sls["f"]) - w_g, 0.0)
            kdef = K_DEF_PRODUCT[p["product"]][sc - 1]
            w_fin = w_g * (1 + kdef) + w_q * (1 + sls["psi2"] * kdef)
            leff = 2 * span if member.get("support") == "cantilever" else span
            out.append({"id": f"en1995.7.2.winst.{mid}", "utilization": util(w_q, leff / 300)})
            out.append({"id": f"en1995.7.2.wfin.{mid}", "utilization": util(w_fin, leff / (200 if annex == "de" else 250))})
    freqs = [c for c in combos if c["kind"] == "freq"]
    if freqs:
        freq = last_max(freqs, lambda c: abs(c["q"]))
        if span > 0 and (freq["q"] != 0 or freq["f"] != 0):
            out.append({"id": f"en1995.7.2.wfreq.{mid}", "utilization": util(deflection(member, p, freq["q"], freq["f"]), span / 300)})
    if member.get("role") == "floor":
        out.extend(floor_checks(annex, member, p))
    if num(member, "fireDurationS") > 0:
        out.append(fire_check(member, p, last_max(ulses, lambda c: abs(c["m"]))["m"]))
    return out


def floor_checks(annex, member, p):
    """🏠️ EN 1995-1-1 §7.3: f1, w under 1 kN on a square strip-panel, and the unit-impulse velocity for f1 ≥ 8 Hz."""
    b, h = num(member, "bM"), num(member, "hM")
    l = max(num(member, "spanM"), 1e-6)
    m1, m2 = num(member, "massKgPerM"), num(member, "massKgPerM2")
    strip = m1 / m2 if m1 > 0 and m2 > 0 else max(b, 0.05)
    ei_l = p["e0"] * b * h ** 3 / 12 / strip
    ei_b = p["e0"] * 0.022 ** 3 / 12
    mass = m2 if m2 > 0 else (m1 if m1 > 0 else p["rho"] * b * h) / strip
    f1 = math.pi / (2 * l * l) * math.sqrt(ei_l / mass)
    bef = min(l, l / 1.1 * (ei_b / ei_l) ** 0.25)
    w_unit = 1000.0 * l ** 3 / (48 * ei_l * bef) * 1000.0
    zeta = num(member, "dampingXi") if num(member, "dampingXi") > 0 else 0.01
    f1_lim, a_lim = (6.0, 0.5) if annex == "de" else (8.0, 1.5)
    out = [
        {"id": f"en1995.7.3.f1.{member['id']}", "utilization": util(f1_lim, max(f1, 1e-9))},
        {"id": f"en1995.7.3.stiffness.{member['id']}", "utilization": util(w_unit / 1000.0, a_lim / 1000.0)},
    ]
    if f1 >= 8.0:
        n40 = (max((40.0 / f1) ** 2 - 1.0, 0.0) * 1.0 * ei_l / ei_b) ** 0.25
        v = 4 * (0.4 + 0.6 * n40) / (mass * l * l + 200.0)
        out.append({"id": f"en1995.7.3.velocity.{member['id']}", "utilization": util(v, 100.0 ** (f1 * zeta - 1.0))})
    return out


def floor_entity_checks(annex, doc, floor):
    """🏠️ The typed floor entity: final deflection with creep, w_fin = w_G (1 + k_def) + w_Q (1 + ψ₂ k_def) of the one-way strip of width 1 m against L/250 (EN) or L/200 (NA-DE), and f1 = π/(2 L²)·√((EI)_l/m) against the lower limit (8 Hz EN, 6 Hz NA-DE, unverified)."""
    member = next((m for m in doc.get("members") or [] if m.get("id") == floor.get("memberId")), None)
    if member is None or num(floor, "spanM") <= 0 or num(floor, "eiLNM2PerM") <= 0 or num(floor, "massKgPerM2") <= 0:
        return []
    p = props(member.get("strengthClass"))
    kdef = K_DEF_PRODUCT[p["product"]][service_class(member.get("serviceClass")) - 1]
    span, ei = num(floor, "spanM"), num(floor, "eiLNM2PerM")
    strip = lambda q: 5 * q * span**4 / (384 * ei)
    w_fin = strip(num(floor, "permanentLoadPa")) * (1 + kdef) + strip(num(floor, "imposedLoadPa")) * (1 + num(floor, "psi2") * kdef)
    f1 = math.pi / (2 * span * span) * math.sqrt(ei / num(floor, "massKgPerM2"))
    return [
        {"id": f"en1995.7.2.floor-creep.{floor['id']}", "utilization": util(w_fin, span / (200 if annex == "de" else 250))},
        {"id": f"en1995.t-63.f1.floors:{floor['id']}:T-63", "utilization": util(6.0 if annex == "de" else 8.0, f1)},
    ]


def panel_entity_checks(annex, panel):
    """🧱️ The typed CLT panel entity as a three-layer panel (outer layers longitudinal and equal, cross layer between): bending, rolling shear and gross shear of T-97 by the shear analogy [`cat_t97`] under the uniform load p L^2/8 and p L/2 per metre; other layups have no closed shear-analogy form and earn no row."""
    layers = panel.get("layers") or []
    if len(layers) != 3 or [x.get("orientation") for x in layers] != ["longitudinal", "transverse", "longitudinal"] or num(layers[0], "thicknessM") != num(layers[2], "thicknessM") or props(panel.get("strengthClass")) is None:
        return []
    span, load = num(panel, "spanM"), num(panel, "loadPa")
    case = {"annex": annex, "num": {"t1": num(layers[0], "thicknessM") * 1000.0, "t2": num(layers[1], "thicknessM") * 1000.0, "l": span * 1000.0, "m_d": load * span * span / 8.0, "v_d": load * span / 2.0 / 1000.0}, "text": {"class": panel.get("strengthClass")}}
    return [{"id": f"en1995.t-97.{name}.panels:{panel['id']}:T-97", "utilization": value} for name, value in cat_t97(case).items() if name != "ei_ef"]


def deck_entity_checks(annex, deck):
    """🌉️ The typed stress-laminated deck as the T-122 case in N, mm and N/mm2 (bar area from the bar diameter, plate area from the plate side) through [`cat_t122`]; a deck with a non-positive dimension, prestress or bar resistance or a loss outside [0, 1) earns no verdict, as in the production layout check."""
    positive = [num(deck, k) for k in ("spanM", "deckWidthM", "deckDepthM", "laminationWidthM", "barSpacingM", "barDiameterM", "plateSideM", "initialPrestressPa", "barResistancePa")]
    losses = [num(deck, k) for k in ("lossRelaxation", "lossCreep", "lossMoisture")]
    if any(v <= 0 for v in positive) or any(not 0 <= v < 1 for v in losses):
        return []
    bar_d, side, depth = num(deck, "barDiameterM") * 1000.0, num(deck, "plateSideM") * 1000.0, num(deck, "deckDepthM") * 1000.0
    n = {"sigma_p_0": num(deck, "initialPrestressPa") / MPA, "loss_relax": losses[0], "loss_creep": losses[1], "loss_mc": losses[2], "tau_h": num(deck, "shearStressPa") / MPA, "bar_spacing": num(deck, "barSpacingM") * 1000.0, "deck_depth": depth, "bar_area": math.pi / 4 * bar_d**2, "f_bar_d": num(deck, "barResistancePa") / MPA, "plate_a": side**2, "wheel_n": num(deck, "wheelLoadN"), "contact_mm": num(deck, "wheelContactM") * 1000.0, "h": depth, "f_c_90_d": num(deck, "compressionPerpendicularPa") / MPA}
    return [{"id": f"en1995.t-122.{name}.decks:{deck['id']}:T-122", "utilization": value} for name, value in cat_t122({"annex": annex, "num": n, "text": {}}).items()]


def rope_plus(j, fax, share):
    """🪢️ Rope effect: add min(F_ax/4, share·J)."""
    return j + min(fax / 4.0, share * j)


def thick_steel(t, d, fh, my, fax, share):
    """🛡️ Thick steel plate or central plate: bearing, one hinge, two hinges."""
    return min(fh * t * d, rope_plus(fh * t * d * (math.sqrt(2 + 4 * my / (fh * d * t * t)) - 1), fax, share), rope_plus(2.3 * math.sqrt(my * fh * d), fax, share))


def connection_modes(kind, steel, planes, t1, t2, ts, d, fh, my, fax):
    """🔩️ Governing characteristic load per shear plane for the four connection configurations."""
    share = {"nail": 0.15, "screw": 1.0, "bolt": 0.25}.get(kind, 0.0)
    if steel and planes >= 2:
        return thick_steel(t1, d, fh, my, fax, share)
    if steel:
        thin = min(0.4 * fh * t1 * d, rope_plus(1.15 * math.sqrt(2 * my * fh * d), fax, share))
        ratio = min(max((ts / d - 0.5) / 0.5, 0.0), 1.0)
        if ratio <= 0.0:
            return thin
        thick = thick_steel(t1, d, fh, my, fax, share)
        return thin if ratio <= 0.0 else (thick if ratio >= 1.0 else thin + ratio * (thick - thin))
    beta = 1.0
    if planes >= 2:
        j = 1.05 * fh * t1 * d / 3 * (math.sqrt(4 + 12 * my / (fh * d * t1 * t1)) - 1)
        k = 1.15 * math.sqrt(2 * my * fh * d)
        return min(fh * t1 * d, 0.5 * fh * t2 * d, rope_plus(j, fax, share), rope_plus(k * 1.0, fax, share))
    r = t2 / t1
    c = fh * t1 * d / 2 * (math.sqrt(1 + 2 * (1 + r + r * r) + r * r) - (1 + r))
    dm = 1.05 * fh * t1 * d / 3 * (math.sqrt(4 + 12 * my / (fh * d * t1 * t1)) - 1)
    e = 1.05 * fh * t2 * d / 3 * (math.sqrt(4 + 12 * my / (fh * d * t2 * t2)) - 1)
    f = 1.15 * math.sqrt(2 * my * fh * d)
    return min(fh * t1 * d, fh * t2 * d, rope_plus(c, fax, share), rope_plus(dm, fax, share), rope_plus(e, fax, share), rope_plus(f, fax, share))


def spacing_minima(kind, d, rho):
    """📏️ Tables 8.2 – 8.5 at α = 0: (a1, a3,t, a4,t) in mm."""
    if kind == "nail":
        k = 1.5 if rho > 420 else 1.0
        return ((10.0 if d < 5 else 12.0) * d * k, 15.0 * d * k, (5.0 + (0.0 if d < 5 else 0.0)) * d * k)
    if kind == "bolt":
        return 5.0 * d, max(7.0 * d, 80.0), max(2.0 * d, 3.0 * d)
    return 5.0 * d, max(7.0 * d, 80.0), max(2.0 * d, 3.0 * d)


def connection_checks(annex, reliability_class, conn):
    """🔗️ Johansen capacity and Table 8.2–8.5 spacing for one dowel-type connection."""
    cid = conn["id"]
    p = props(conn.get("strengthClass", ""))
    if p is None:
        return [{"id": f"en1995.conn.material.{cid}", "utilization": None}]
    governing = connection_uls(conn, FACTORS.ActionFactors(annex, reliability_class=reliability_class))
    if governing is None:
        return []
    rank, fed = governing
    ftype = str(conn.get("fastenerType", "")).lower()
    kind = "nail" if "nail" in ftype else "screw" if "screw" in ftype else "bolt" if "bolt" in ftype else "dowel"
    d = num(conn, "diameterM") * 1000
    t1, t2, ts = num(conn, "t1M") * 1000, num(conn, "t2M") * 1000, num(conn, "steelPlateThicknessM") * 1000
    rho = p["rho"]
    if (kind == "nail" and d <= 8) or (kind == "screw" and d <= 6):
        fh = 0.082 * rho * d ** -0.3
    else:
        fh = 0.082 * (1 - 0.01 * d) * rho
    fu = num(conn, "fUK") / MPA
    my = 0.15 * fu * (1.1 * 0.7 * d) ** 2.6 if kind == "screw" and d <= 6 else 0.3 * fu * d ** 2.6
    fax = 0.52 * d ** -0.5 * t2 ** -0.1 * 350 ** 0.8 * d * t2 * min(d / 8, 1.0) * (rho / 350) ** 0.8 if kind == "screw" else 0.0
    planes_count = int(conn.get("shearPlanes") or 0)
    fvrk = connection_modes(kind, bool(conn.get("steelPlate")), planes_count, t1, t2, ts, d, fh, my, fax)
    planes = float(max(planes_count, 1))
    rows = max(int(conn.get("rows") or 0), 1)
    n = max(round(int(conn.get("number") or 0) / rows), 1)
    a1_prov = num(conn, "spacingM") * 1000
    if kind in ("nail", "screw"):
        r = a1_prov / d
        kef = 1.0 if r >= 14 else 0.85 + 0.15 * (r - 10) / 4 if r >= 10 else 0.7 + 0.15 * (r - 7) / 3 if r >= 7 else 0.5 + 0.2 * (r - 4) / 3 if r >= 4 else 0.5
        nef_row = n ** kef
    else:
        nef_row = float(n) if n <= 1 else min(n, n ** 0.9 * (max(a1_prov, 1e-6) / (13 * d)) ** 0.25)
    fvrd = k_mod(conn.get("serviceClass"), rank) * nef_row * rows * planes * fvrk / 1.3
    clause = "8.2.3" if conn.get("steelPlate") else "8.2.2"
    a1, a3, a4 = spacing_minima("nail" if kind == "screw" else kind, d, rho)
    return [
        {"id": f"en1995.{clause}.johansen.{cid}", "utilization": util(fed, fvrd)},
        {"id": f"en1995.8.spacing.a1.{cid}", "utilization": a1 / max(a1_prov, 1e-9)},
        {"id": f"en1995.8.spacing.a3t.{cid}", "utilization": a3 / max(num(conn, "endDistanceM") * 1000, 1e-9)},
        {"id": f"en1995.8.spacing.a4t.{cid}", "utilization": a4 / max(num(conn, "edgeDistanceM") * 1000, 1e-9)},
    ]


def evaluate(doc):
    """🏗️ Every check the snapshot earns, in member then connection order."""
    annex = annex_of(doc)
    reliability_class = int(str(doc["reliabilityClass"])[-1])
    members = doc.get("members") or []
    checks = [] if members else [{"id": "en1995.subject.empty", "utilization": None}]
    for member in members:
        checks.extend(member_checks(annex, reliability_class, member, doc.get("ulsEquation") == "eq610ab"))
    for conn in doc.get("connections") or []:
        checks.extend(connection_checks(annex, reliability_class, conn))
    for floor in doc.get("floors") or []:
        checks.extend(floor_entity_checks(annex, doc, floor))
    for panel in doc.get("panels") or []:
        checks.extend(panel_entity_checks(annex, panel))
    for deck in doc.get("decks") or []:
        checks.extend(deck_entity_checks(annex, deck))
    return {"checks": checks}


def _row(case):
    """🔎️ Strength row of the case's class."""
    return props(case["text"]["class"])


def _fd(case, row, fk):
    """🧮️ k_mod·f_k/γ_M for the case's service class and duration."""
    n, t = case["num"], case["text"]
    kmod = n.get("kmod") or k_mod(n.get("sc", 1), duration_rank(t.get("duration", "medium")))
    return kmod * fk / gamma_m(case["annex"], row["product"])


def cat_t13(case):
    """🦶️ T-13 compression perpendicular."""
    n, row = case["num"], _row(case)
    l, b = n["l"], n["b"]
    lef = l + min(30.0, n.get("a_left", 1e9), l) + min(30.0, n.get("a_right", 1e9), l)
    kc = k_c90(row["product"], l, 0.0)
    sd = n["f_c90"] / (b * lef)
    return {"bearing": util(sd, kc * _fd(case, row, row["fc90"]) / MPA)}


def cat_t14(case):
    """🪵️ T-14 biaxial bending of a rectangle (k_m = 0.7)."""
    n, row = case["num"], _row(case)
    b, h = n["b"], n["h"]
    fm = k_h(h / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["fm"]) / MPA
    fz = k_h(b / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["fm"]) / MPA
    sy, sz = n.get("m_y", 0) / (b * h * h / 6), n.get("m_z", 0) / (h * b * b / 6)
    return {"bending_6_11": sy / fm + 0.7 * sz / fz, "bending_6_12": 0.7 * sy / fm + sz / fz}


def cat_t20(case):
    """🏛️ T-20 buckling: slenderness ratio from the radius of gyration, k_c from the quadratic root form."""
    n, row = case["num"], _row(case)
    b, h = n["b"], n["h"]
    out = {}
    for axis, lef, dim in (("y", n["lef_y"], h), ("z", n["lef_z"], b)):
        i = dim / math.sqrt(12.0)
        rel = lef / i / math.pi * math.sqrt(row["fc0"] / row["e05"])
        beta = 0.2 if row["product"] == "solid" else 0.1
        kk = 0.5 * (1 + beta * (rel - 0.3) + rel * rel)
        kc = 1.0 if rel <= 0.3 else 1.0 / (kk + math.sqrt(kk * kk - rel * rel))
        fc = _fd(case, row, row["fc0"]) / MPA
        fmd = k_h(h / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["fm"]) / MPA
        sc = n["n_c"] / (b * h)
        sm = n.get("m_y", 0) / (b * h * h / 6)
        out[axis] = (sc / (kc * fc) + (sm / fmd if axis == "y" else 0.7 * sm / fmd), rel, kc)
    return {"buckling_y": out["y"][0], "buckling_z": out["z"][0]}


def cat_t22(case):
    """🌀️ T-22 lateral torsional buckling via the exact elastic critical moment."""
    n, row = case["num"], _row(case)
    b, h = n["b"], n["h"]
    lef = n["lef"]
    e = row["e05"] / MPA
    scr = math.pi * math.sqrt(e * (h * b ** 3 / 12) * (e / 16) * (h * b ** 3 * (1 / 3 - 0.21 * b / h * (1 - b ** 4 / (12 * h ** 4))))) / (lef * (b * h * h / 6))
    lam = math.sqrt(row["fm"] / MPA / scr)
    kcrit = 1.0 if lam <= 0.75 else (1.56 - 0.75 * lam if lam <= 1.4 else 1 / lam ** 2)
    fmd = k_h(h / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["fm"]) / MPA
    return {"ltb": util(n["m_y"] / (b * h * h / 6), kcrit * fmd)}


def cat_t30(case):
    """📐️ T-30 tapered beam: the tapered-edge and parallel-edge utilisation at the governing section of the moment line (`_taper_governing`, 6.38/6.39 through the stress transformation form of k_m,α)."""
    _, tapered, parallel = _taper_governing(case)
    return {"tapered_edge": tapered, "parallel_edge": parallel}


def cat_t31(case):
    """🌈️ T-31 apex bending: k_l and k_r from the coefficient table."""
    n, row = case["num"], _row(case)
    t = math.tan(math.radians(n.get("alpha_ap_deg", 0)))
    hr = n["h_ap"] / (n["r_in"] + 0.5 * n["h_ap"])
    kl = (1 + 1.4 * t + 5.4 * t * t) + (0.35 - 8 * t) * hr + (0.6 + 8.3 * t - 7.8 * t * t) * hr ** 2 + 6 * t * t * hr ** 3
    kr = 1.0 if n["r_in"] / n["t_lam"] >= 240 else 0.76 + 0.001 * n["r_in"] / n["t_lam"]
    sd = kl * 6 * n["m_ap"] / (n["b"] * n["h_ap"] ** 2)
    fm = k_h(n["h_ap"] / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["fm"]) / MPA
    return {"apex_bending": util(sd, kr * fm)}


def cat_t45(case):
    """🪚️ T-45 support-side notch."""
    n, row = case["num"], _row(case)
    h, hef, b = n["h"], n["h_ef"], n["b"]
    al = hef / h
    kn = {"lvl": 4.5, "glulam": 6.5}.get(row["product"], 5.0)
    kv = min(1.0, kn / (math.sqrt(h) * (math.sqrt(al * (1 - al)) + 0.8 * n.get("x", 0) / h * math.sqrt(1 / al - al * al))))
    kcr = k_cr(case["annex"], row["fv"], row["product"])
    tau = 1.5 * n["v_d"] / (kcr * b * hef)
    return {"notch_shear": util(tau, kv * _fd(case, row, row["fv"]) / MPA)}


def cat_t63(case):
    """🌊️ T-63 floor frequency from the Rayleigh quotient of a simply supported strip."""
    n = case["num"]
    omega = (math.pi / n["span_m"]) ** 2 * math.sqrt(n["ei_l"] / n["mass_kg_m2"])
    f1 = omega / (2 * math.pi)
    lim = 6.0 if case["annex"] == "de" else 8.0
    return {"f1": util(lim, f1)}


def cat_t70(case):
    """🔩️ T-70/T-71 bolt in softwood: embedment, yield moment and the minimum of the six single-shear modes; without t1 only the fastener quantities (8.5.1) of nails, staples and screws."""
    n, row = case["num"], _row(case)
    if "t1" not in n:
        t = case["text"]
        kind, d, rho = t.get("kind", "dowel"), n["d"], n.get("rho_k1", row["rho"])
        panel = t.get("panel", "timber")
        if panel == "plywood":
            fh = 0.11 * rho * d ** -0.3
        elif panel in ("osb", "particleboard"):
            fh = 50.0 * d ** -0.6 * n.get("t_panel", 0.0) ** 0.2
        else:
            fh = _fh(kind, rho, d, n.get("alpha_deg", 0.0), t.get("wood", "softwood"))
        fu = n["f_u"]
        my = {"nail_square": 0.45 * fu * d ** 2.6, "staple": 240.0 * d ** 2.6}.get(kind, 0.15 * fu * (1.1 * n.get("d_root", 0.7 * d)) ** 2.6 if kind == "screw" and d <= 6 else 0.3 * fu * d ** 2.6)
        return {"f_h_k": fh * MPA, "m_y_rk": my / 1000.0}
    d, fu = n["d"], n.get("f_u", 400.0)
    fh = 0.082 * (1 - 0.01 * d) * row["rho"]
    my = 0.3 * fu * d ** 2.6
    t1, t2 = n["t1"], n["t2"]
    rk = connection_modes("bolt", False, 1, t1, t2, 0.0, d, fh, my, 0.0)
    kmod = n.get("kmod") or k_mod(n.get("sc", 1), duration_rank(case["text"].get("duration", "medium")))
    values = {"f_h_k": fh * MPA, "m_y_rk": my / 1000.0}
    if case.get("entry") == "T-71":
        values.update({"f_v_rk": rk, "f_v_rd": kmod * rk / 1.3})
    return values


def cat_t74(case):
    """📏️ T-74 bolt spacings at 30 degrees."""
    n = case["num"]
    d, a = n["d"], math.radians(n["alpha_deg"])
    req = {"a1": (4 + abs(math.cos(a))) * d, "a2": 4 * d, "a3t": max(7 * d, 80.0), "a4t": max((2 + 2 * abs(math.sin(a))) * d, 3 * d), "a4c": 3 * d}
    return {k: util(v / 1000.0, n[k] / 1000.0) for k, v in req.items() if k in n}


def cat_t83(case):
    """🧷️ T-83 slip moduli."""
    n = case["num"]
    k = n["rho_m1"] ** 1.5 * n["d"] / 23.0
    return {"k_ser": k, "k_u": 2.0 / 3.0 * k}


def cat_t90(case):
    """🧩️ T-90 two-part composite: Newmark series formulation of (EI)_ef."""
    n = case["num"]
    e1a1, e2a2 = n["e1"] * n["a1"], n["e2"] * n["a2"]
    z = (n["h1"] + n["h2"]) / 2
    ea = e1a1 * e2a2 / (e1a1 + e2a2)
    ei0 = n["e1"] * n["i1"] + n["e2"] * n["i2"]
    ei = ei0 + ea * z * z / (1 + math.pi ** 2 * ea * n["s1"] / (n["k"] * n["l"] ** 2))
    out = {"ei_ef": ei, "slip_efficiency": ei / (ei0 + ea * z * z)}
    if "q_d" in n:
        out["deflection"] = util(5 * n["q_d"] * n["l"] ** 4 / (384 * ei), n["l"] / n.get("limit_divisor", 300.0))
    return out


def cat_t97(case):
    """🪵️ T-97 three-layer CLT by the shear analogy (Kreuzinger): EI_ef = EI_A + EI_B/(1+kappa) and, with the load split M_A = M EI_A/EI_ef, M_B = M EI_B,eff/EI_ef, V_B = V EI_B,eff/EI_ef, the outer-layer stress (own bending plus the normal force M_B/z), the rolling shear tau_R = V_B/z of the cross layer and the gross shear 1.5 V/h; capacities k_mod f/gamma_M with the registry rows `clt.*` for f_r,k, G_R and the gross shear strength."""
    n, row = case["num"], _row(case)
    t_out, t_in = n["t1"], n["t2"]
    e, l = row["e0"] / MPA, n["l"]
    gr = n.get("g_r", ndp("clt.rolling-shear-modulus", case["annex"]))
    z = t_out + t_in
    ei_a = 2 * e * t_out ** 3 / 12
    ei_b = e * t_out * z * z / 2
    kappa = ei_b * math.pi ** 2 / (gr * z * z / t_in * l * l)
    ei_b_eff = ei_b / (1 + kappa)
    ei = ei_a + ei_b_eff
    out = {"ei_ef": ei}
    m, v = n.get("m_d", 0.0), n.get("v_d", 0.0)
    if "m_d" in n or "v_d" in n:
        km = n.get("kmod", 0.8)
        g = gamma_m(case["annex"], "clt")
        sigma = m / ei * e * t_out / 2 + m * ei_b_eff / ei / (z * t_out)
        out["bending"] = util(sigma, km * row["fm"] / MPA / g)
        out["rolling_shear"] = util(v * ei_b_eff / ei / z, km * n.get("f_r_k", ndp("clt.rolling-shear-strength", case["annex"])) / g)
        out["gross_shear"] = util(1.5 * v / (2 * t_out + t_in), km * n.get("f_v_clt_k", ndp("clt.gross-shear-strength", case["annex"])) / g)
    return out


def cat_t121(case):
    """📉️ T-121 Miner sum."""
    n = case["num"]
    a, b = n.get("sn_a", 15.0), n.get("sn_b", 4.0)
    d, years, out = 0.0, n.get("years", 1.0), {}
    for k in range(1, 7):
        if f"ds{k}" in n:
            dk = 0.0 if n[f"ds{k}"] < n.get("ds_cut", 0.0) else n[f"n{k}"] * years / 10 ** (a - b * math.log10(n[f"ds{k}"]))
            out[f"damage_{k}"] = dk
            d += dk
    out["miner"] = d
    if "years" in n and d > 0:
        out["life_years"] = years / d * 365.25 * 86400.0
    return out


def cat_t120(case):
    """🚶️ T-120 first frequency of the span and the single-pedestrian vertical acceleration."""
    n = case["num"]
    f1 = math.pi / (2 * n["span_m"] ** 2) * math.sqrt(n["ei_nm2"] / n["mass_kg_m"])
    a1 = (200.0 if f1 <= 2.5 else 100.0) / (n["mass_kg_m"] * n["span_m"] / 2 * n.get("zeta", 0.015))
    out = {"f1": f1, "a_vert": util(0.0 if f1 > 5 else a1, 0.7)}
    if case["text"].get("crowd_model") == "setra":
        d, w, z, m, L = n["crowd_density"], n["deck_width_m"], n.get("zeta", 0.015), n["mass_kg_m"], n["span_m"]
        psi = 0.0 if f1 < 0.5 or f1 > 2.1 else (f1 - 0.5) / 0.5 if f1 < 1.0 else 1.0 if f1 <= 1.7 else (2.1 - f1) / 0.4
        npers = d * L * w
        load = d * 280.0 * 10.8 * math.sqrt(z / npers) * psi
        out["a_crowd"] = util(2 * load * w / (math.pi * z * m), n.get("a_limit", 0.7))
    return out


def cat_t101(case):
    """🔥️ T-101 bending utilisation of a three-sided exposed rectangle after t_req minutes (reduced cross-section)."""
    n, row = case["num"], _row(case)
    t = n["t_req"]
    d_ef = (0.7 if row["product"] == "glulam" else 0.8) * t + min(1.0, t / 20.0) * 7.0
    b, h = n["b"] - 2 * d_ef, n["h"] - d_ef
    kfi = {"glulam": 1.15, "solid": 1.25, "lvl": 1.1}[row["product"]]
    return {"fire_resistance": n["m_fi"] / (b * h * h / 6) / (kfi * row["fm"] / MPA)}


def cat_t106(case):
    """🔥️ T-106 reduced properties: three exposed sides, closed-form perimeter over area."""
    n, row = case["num"], _row(case)
    t, b, h = n["t_req"], n["b"], n["h"]
    beta = {"glulam": 0.7, "solid": 0.8, "lvl": 0.7, "clt": 0.65}[row["product"]]
    br, hr = b - 2 * beta * t, h - beta * t
    ratio = (2 * hr + br) / (br * hr) * 1000.0
    scale = min(1.0, t / 20.0)
    k_bend, k_comp, k_ten = (1 - scale * ratio / c for c in (200.0, 125.0, 330.0))
    kfi = {"glulam": 1.15, "solid": 1.25, "lvl": 1.1, "clt": 1.15}[row["product"]]
    sigma = n["m_fi"] / (br * hr * hr / 6)
    return {"k_mod_fi_bending": k_bend, "k_mod_fi_compression": k_comp, "k_mod_fi_tension": k_ten, "fire_resistance": sigma / (k_bend * kfi * row["fm"] / MPA)}


def _clt_depth(thick, beta, t_end, fall_off, factor, protect, dt=1e-3):
    """🌳️ Char front by explicit time stepping (independent of the Rust event formulation)."""
    x, k, local, fell, t, times = 0.0, 0, 0.0, False, 0.0, []
    edges = [sum(thick[: i + 1]) for i in range(len(thick))]
    while t < t_end - 1e-12 and k < len(thick):
        rate = beta * (factor if fell and local < protect else 1.0)
        step = min(dt, t_end - t)
        dx = rate * step
        x += dx
        local += dx
        t += step
        if x >= edges[k]:
            overshoot = x - edges[k]
            x -= overshoot
            times.append(t)
            fell, local, k = fall_off, 0.0, k + 1
    return min(x, sum(thick)), times


def cat_t107(case):
    """🌳️ T-107 CLT char with fall-off, residual rigid longitudinal layers."""
    n, row = case["num"], _row(case)
    count = int(n["n_layers"])
    thick = [n[f"t{i}"] for i in range(1, count + 1)]
    longi = [n[f"o{i}"] > 0.5 for i in range(1, count + 1)]
    beta, t = n.get("beta_0", 0.65), n["t_req"]
    on = n.get("fall_off", 1.0) > 0.5
    depth, times = _clt_depth(thick, beta, t, on, n.get("fall_off_factor", 2.0), n.get("protect_mm", 25.0))
    plain, _ = _clt_depth(thick, beta, t, False, 2.0, 25.0)
    out = {"char_depth": depth / 1000.0, "char_depth_plain": plain / 1000.0}
    for i, tm in enumerate(times, 1):
        out[f"glue_line_{i}"] = tm * 60.0
    d_ef = depth + min(1.0, t / 20.0) * 7.0
    z, parts = 0.0, []
    for th, lg in zip(thick, longi):
        if lg and z + th > max(z, d_ef):
            parts.append((max(z, d_ef), z + th))
        z += th
    if "m_fi" in n and parts:
        area = sum(hi - lo for lo, hi in parts)
        zs = sum((hi - lo) * (hi + lo) / 2 for lo, hi in parts) / area
        inertia = sum((hi - lo) ** 3 / 12 + (hi - lo) * ((hi + lo) / 2 - zs) ** 2 for lo, hi in parts)
        reach = max(max(abs(lo - zs), abs(hi - zs)) for lo, hi in parts)
        out["fire_resistance"] = n["m_fi"] * reach / inertia / (1.15 * row["fm"] / MPA)
    return out


def cat_t104(case):
    """🔩️ T-104 connection in fire: reduced load method with the exponential time law."""
    n = case["num"]
    afi = n.get("beta_n", 0.7) * n["t_req"] * n.get("k_flux", 1.5)
    out = {"a_fi": afi / 1000.0}
    if "f_fi_d" in n:
        rd = (1.05 if case["text"].get("side_members") == "steel" else 1.15) * n["f_v_rk"]
        out["connection_fire"] = util(n["f_fi_d"], rd)
        if "f_rd_20" in n:
            eta = n["f_fi_d"] / n["f_rd_20"]
            out["eta"] = eta
            if "k_fire" in n:
                out["connection_fire_time"] = util(n["t_req"], max(0.0 if eta >= 1 else -math.log(eta) / n["k_fire"], 1e-6))
    return out


def cat_t122(case):
    """🌉️ T-122 stress-laminated deck: staged prestress losses, bar force, anchor plate pressure."""
    n = case["num"]
    staged = [n[k] for k in ("loss_relax", "loss_creep", "loss_mc") if k in n]
    keep = math.prod(1 - v for v in staged) if staged else 1 - n.get("loss", ndp("deck.default-loss", case["annex"]))
    sp = n["sigma_p_0"] * keep
    out = {"prestress_min": util(ndp("deck.min-prestress", case["annex"]), sp)}
    if "tau_h" in n:
        out["friction"] = util(n["tau_h"], n.get("mu", ndp("deck.friction-coefficient", case["annex"])) * sp)
    if all(k in n for k in ("wheel_n", "contact_mm", "h")):
        out["wheel_spread"] = util(n["wheel_n"] / ((n["contact_mm"] + 2 * n["h"]) * n["contact_mm"]), n.get("f_c_90_d", 1.0))
    if "bar_spacing" in n and "deck_depth" in n:
        force = n["sigma_p_0"] * n["bar_spacing"] * n["deck_depth"]
        if "bar_area" in n and "f_bar_d" in n:
            out["bar_force"] = util(force, n["bar_area"] * n["f_bar_d"])
        if "plate_a" in n:
            out["plate_bearing"] = util(force / n["plate_a"], n.get("k_c90", ndp("deck.k-c90", case["annex"])) * n.get("f_c_90_d", 1.0))
    return out


def cat_t93(case):
    """🏛️ T-93 mechanically jointed column: effective slenderness with the eta coefficient and the quadratic k_c root."""
    n, row = case["num"], _row(case)
    eta = n.get("eta", {"glued": 1.0, "bolted": 6.0, "connectors": 3.0}.get(case["text"].get("joint", "nailed"), 4.0))
    lam, lam1 = n["lef"] / n["i_tot"], n["l1"] / n["i1"]
    lam_ef = math.sqrt(lam ** 2 + eta * n["n_parts"] / 2 * lam1 ** 2)
    rel = lam_ef / math.pi * math.sqrt(row["fc0"] / row["e05"])
    beta = 0.2 if row["product"] == "solid" else 0.1
    kk = 0.5 * (1 + beta * (rel - 0.3) + rel * rel)
    kc = 1.0 if rel <= 0.3 else 1.0 / (kk + math.sqrt(kk * kk - rel * rel))
    return {"lambda_ef": lam_ef, "lambda_rel": rel, "k_c": kc, "spaced_column": util(n["n_c"] / n["area"], kc * _fd(case, row, row["fc0"]) / MPA)}


def cat_t03(case):
    """🛡️ T-03 partial factor γ_M of a product, accidental situation 1.0 by Table 2.3."""
    return {"gamma_m": 1.0 if case["text"].get("situation") == "accidental" else gamma_m(case["annex"], case["text"]["material"])}


def cat_t11(case):
    """🪵️ T-11 tension perpendicular to the grain, volume factor (1e7 mm³ / V)^0.2 when the stressed volume is given."""
    n, row = case["num"], _row(case)
    kvol = (1e7 / n["volume"]) ** 0.2 if "volume" in n else 1.0
    return {"tension90": util(n["sigma_t_90"], kvol * _fd(case, row, row["ft90"]) / MPA)}


ORACLE_CASES = {"T-03": cat_t03, "T-11": cat_t11, "T-13": cat_t13, "T-14": cat_t14, "T-20": cat_t20, "T-22": cat_t22, "T-30": cat_t30, "T-31": cat_t31, "T-45": cat_t45, "T-63": cat_t63, "T-70": cat_t70, "T-71": cat_t70, "T-74": cat_t74, "T-83": cat_t83, "T-90": cat_t90, "T-97": cat_t97, "T-121": cat_t121, "T-120": cat_t120, "T-101": cat_t101, "T-106": cat_t106, "T-107": cat_t107, "T-104": cat_t104, "T-122": cat_t122, "T-93": cat_t93}


def _kmod(case, row):
    """🔧 k_mod of the case: explicit `kmod` or Table 3.1 for the service class and duration."""
    n, t = case["num"], case["text"]
    return n.get("kmod") or k_mod(n.get("sc", 1), duration_rank(t.get("duration", "medium")))


def _kh_dim(case, row, key, default_mm):
    """📏️ Size factor for the dimension `kh_dim*` (mm) or the default."""
    return k_h(case["num"].get(key, default_mm) / 1000.0, row["product"], row["rho"])


def _ft_dim(case, row):
    """📏️ Tension size dimension: the larger side of solid timber, the width of other products (EN 1995-1-1 3.2 (3), 3.3 (3))."""
    n = case["num"]
    return n.get("kh_dim", max(n["b"], n["h"]) if row["product"] == "solid" else n["b"])


def cat_t01(case):
    """📚️ T-01 EN 338 / EN 14080 strength class properties."""
    row = _row(case)
    return {"f_m_k": row["fm"], "f_t_0_k": row["ft0"], "f_t_90_k": row["ft90"], "f_c_0_k": row["fc0"], "f_c_90_k": row["fc90"], "f_v_k": row["fv"], "e_0_mean": row["e0"], "e_0_05": row["e05"], "g_mean": row["g"], "rho_k": row["rho"]}


def cat_t02(case):
    """⏱️ T-02 k_mod of Table 3.1; a connection of two parts takes sqrt(k_mod,1 k_mod,2) (EN 1995-1-1 2.3.2.2 (2))."""
    n, row = case["num"], _row(case)
    k = _kmod(case, row)
    out = {"k_mod": k}
    if "kmod_2" in n:
        out["k_mod_conn"] = math.sqrt(k * n["kmod_2"])
    return out


def cat_t04(case):
    """🐌️ T-04 k_def of Table 3.2 and the final deformation u_fin = u_inst (1 + psi_2 k_def) per action."""
    n = case["num"]
    sc = service_class(n.get("sc", 1))
    kdef = K_DEF_PRODUCT[case["text"]["material"] if case["text"]["material"] in K_DEF_PRODUCT else "solid"][sc - 1]
    out = {"k_def": kdef}
    if "u_inst_g" in n and "u_inst_q" in n:
        out["u_fin"] = (n["u_inst_g"] * (1 + kdef) + n["u_inst_q"] * (1 + n.get("psi_2", 0.3) * kdef)) / 1000.0
    if "kdef_2" in n:
        out["k_def_conn"] = 2.0 * math.sqrt(kdef * n["kdef_2"])
    if "e_mean" in n:
        out["e_fin"] = n["e_mean"] / (1 + n.get("psi_2", 0.3) * kdef) * MPA
    return out


def cat_t05(case):
    """📏️ T-05 size factors k_h, k_l (LVL) and the volume factor (V0/V)^0.2 with V0 = 0.01 m3."""
    n, row = case["num"], _row(case)
    out = {"k_h": k_h(n["h"] / 1000.0, row["product"], row["rho"])}
    if row["product"] == "lvl":
        s = n.get("s_lvl", 0.12)
        out["k_h"] = min((300.0 / n["h"]) ** s, 1.2)
        out["k_l"] = min((3000.0 / n.get("length", 3000.0)) ** (s / 2.0), 1.1)
    if "volume" in n:
        out["k_vol"] = (1e7 / n["volume"]) ** 0.2
    return out


def cat_t06(case):
    """🛡️ T-06 design strengths f_d = k_mod f_k / gamma_M; bending and tension parallel carry k_h."""
    n, row = case["num"], _row(case)
    kh = k_h(n.get("h", 600.0) / 1000.0, row["product"], row["rho"])
    return {"f_m_d": kh * _fd(case, row, row["fm"]), "f_t_0_d": kh * _fd(case, row, row["ft0"]), "f_t_90_d": _fd(case, row, row["ft90"]), "f_c_0_d": _fd(case, row, row["fc0"]), "f_c_90_d": _fd(case, row, row["fc90"]), "f_v_d": _fd(case, row, row["fv"])}


def cat_t10(case):
    """➕ T-10 tension parallel to the grain, sigma_t,0,d <= k_h f_t,0,d (6.1)."""
    n, row = case["num"], _row(case)
    area = n.get("area_net", n["b"] * n["h"])
    return {"tension": util(n["n_t"] / area, k_h(_ft_dim(case, row) / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["ft0"]) / MPA)}


def cat_t12(case):
    """➖ T-12 compression parallel to the grain (6.2); the buckling check is required above the relative slenderness 0.3."""
    n, row = case["num"], _row(case)
    out = {"compression": util(n["n_c"] / (n["b"] * n["h"]), _fd(case, row, row["fc0"]) / MPA)}
    if "lef_y" in n and "lef_z" in n:
        rel = max(lambda_rel_c(n["lef_y"] / (n["h"] / math.sqrt(12.0)), row["fc0"], row["e05"]), lambda_rel_c(n["lef_z"] / (n["b"] / math.sqrt(12.0)), row["fc0"], row["e05"]))
        out["stability_required"] = 1.0 if rel > 0.3 else 0.0
    return out


def cat_t15(case):
    """✂️ T-15 shear (6.13) with k_cr; a point load within 2h of the support is reduced by beta = a/(2h) (6.1.7 (3))."""
    n, row = case["num"], _row(case)
    kcr = n.get("k_cr", k_cr(case["annex"], row["fv"], row["product"]))
    v = n["v"]
    if "v_point" in n and "a_support" in n:
        v = max(v - n["v_point"] * (1.0 - min(n["a_support"] / (2.0 * n["h"]), 1.0)), 0.0)
    tau = v * n["s_first"] / (n["b"] * n["h"] ** 3 / 12.0 * kcr * n["b"]) if "s_first" in n else 1.5 * v / (kcr * n["b"] * n["h"])
    return {"shear": util(tau, _fd(case, row, row["fv"]) / MPA)}


def _rect_torsion_alpha(ratio):
    """🧮️ Saint-Venant coefficient alpha of tau_max = T/(alpha b^2 h) from the exact trigonometric series (long side h, ratio h/b >= 1)."""
    odd = range(1, 400, 2)
    j = 1.0 / 3.0 - 64.0 / math.pi ** 5 / ratio * sum(math.tanh(k * math.pi * ratio / 2.0) / k ** 5 for k in odd)
    tau = 1.0 - 8.0 / math.pi ** 2 * sum(1.0 / (k * k) * 2.0 * math.exp(-k * math.pi * ratio / 2.0) / (1.0 + math.exp(-k * math.pi * ratio)) for k in odd)
    return j / tau


def cat_t16(case):
    """🌀️ T-16 torsion (6.14, 6.15): tau_tor = T/W_t with the Saint-Venant series, k_shape = min(1 + 0.15 h/b, 2) for rectangles, 1.2 for circles."""
    n, row = case["num"], _row(case)
    fv = _fd(case, row, row["fv"]) / MPA
    if case["text"].get("shape", "rect") == "circle":
        tau, ksh = n["m_t"] / (math.pi * n["d"] ** 3 / 16.0), 1.2
    else:
        bs, hl = min(n["b"], n["h"]), max(n["b"], n["h"])
        tau = n["m_t"] / (_rect_torsion_alpha(hl / bs) * bs * bs * hl)
        ksh = min(1.0 + 0.15 * hl / bs, 2.0) if case["text"].get("shape", "rect") == "rect" else 1.0
    out = {"torsion": util(tau, ksh * fv)}
    if "tau_v" in n:
        out["torsion_shear"] = tau / (ksh * fv) + (n["tau_v"] / fv) ** 2
    return out


def _bending_fd(case, row, key, default_mm):
    return _kh_dim(case, row, key, default_mm) * _fd(case, row, row["fm"]) / MPA


def cat_t17(case):
    """➕ T-17 axial force with biaxial bending (6.17-6.20); tension takes precedence over compression when both are given."""
    n, row = case["num"], _row(case)
    b, h = n["b"], n["h"]
    km = n.get("km", 0.7 if case["text"].get("shape", "rect") == "rect" else 1.0)
    sy, sz = n.get("m_y", 0) / (b * h * h / 6), n.get("m_z", 0) / (h * b * b / 6)
    fy, fz = _bending_fd(case, row, "kh_dim", h), _bending_fd(case, row, "kh_dim_z", b)
    if n.get("n_t", 0) > 0:
        ft = k_h(_ft_dim(case, row) / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["ft0"]) / MPA
        st = n["n_t"] / (b * h)
        return {"tension_bending_6_17": st / ft + sy / fy + km * sz / fz, "tension_bending_6_18": st / ft + km * sy / fy + sz / fz}
    sc = n.get("n_c", 0) / (b * h) / (_fd(case, row, row["fc0"]) / MPA)
    return {"compression_bending_6_19": sc ** 2 + sy / fy + km * sz / fz, "compression_bending_6_20": sc ** 2 + km * sy / fy + sz / fz}


def cat_t18(case):
    """🕳️ T-18 net section of a tension member with holes (5.2) and the hole diameter limit 0.15 h."""
    n, row = case["num"], _row(case)
    a_net = n["b"] * n["h"] - n.get("hole_d", 0.0) * n.get("hole_count", 0.0) * n["b"]
    ft = k_h(_ft_dim(case, row) / 1000.0, row["product"], row["rho"]) * _fd(case, row, row["ft0"]) / MPA
    out = {"net_section": util(n["n_t"] / a_net, ft)}
    if "hole_d" in n:
        out["hole_limit"] = util(n["hole_d"], 0.15 * n["h"])
    return out


def _col_state(case, row, lef, dim_mm):
    """🏛️ Relative slenderness and k_c of one axis of a rectangle with radius of gyration dim/sqrt(12)."""
    rel = lambda_rel_c(lef / (dim_mm / math.sqrt(12.0)), row["fc0"], row["e05"])
    return rel, k_c(rel, case["num"].get("beta_c", 0.2 if row["product"] == "solid" else 0.1))


def cat_t21(case):
    """🧭️ T-21 governing axis of the compression member with bending: 0 when y governs, 1 for z."""
    values = cat_t20(case)
    return {"governing": 0.0 if values["buckling_y"] >= values["buckling_z"] else 1.0}


def _kcrit_simplified(case, row):
    """🌀️ k_crit from the simplified critical bending stress 0.78 b^2 E_0.05/(h l_ef) (6.32) with l_ef of Table 6.1."""
    n, t = case["num"], case["text"]
    b, h = n["b"], n["h"]
    ratio = {"udl": 0.9, "point_mid": 0.8, "cantilever_udl": 0.5, "cantilever_point": 0.8}.get(t.get("load_case", "udl"), 1.0)
    lef = n.get("lef", max(ratio * n.get("l", 0.0) + {"compression": -0.5 * h, "tension": 2.0 * h}.get(t.get("load_pos", "centroid"), 0.0), 1.0))
    if n.get("restrained", 0.0) > 0.5:
        return 1.0
    return k_crit(math.sqrt(row["fm"] / (0.78 * b * b * row["e05"] / (h * lef))))


def cat_t23(case):
    """🔁 T-23 combined lateral-torsional and column buckling with weak-axis bending: (sigma_m,y/(k_crit f_m,y))^2 + sigma_c/(k_c,z f_c) + k_m sigma_m,z/f_m,z."""
    n, row = case["num"], _row(case)
    b, h = n["b"], n["h"]
    _, kcz = _col_state(case, row, n["lef_z"], b)
    km = n.get("km", 0.7 if case["text"].get("shape", "rect") == "rect" else 1.0)
    fy, fz = _bending_fd(case, row, "kh_dim", h), _bending_fd(case, row, "kh_dim_z", b)
    sy, sz = n.get("m_y", 0) / (b * h * h / 6), n.get("m_z", 0) / (h * b * b / 6)
    return {"biaxial_ltb": (sy / (_kcrit_simplified(case, row) * fy)) ** 2 + n["n_c"] / (b * h) / (kcz * _fd(case, row, row["fc0"]) / MPA) + km * sz / fz}


def cat_t24(case):
    """〰️ T-24 second-order moment with bow imperfection l/300 (solid) or l/500 and the Euler amplification 1/(1 - N/N_cr) (5.4.4)."""
    n, row = case["num"], _row(case)
    b, h, lef, nc = n["b"], n["h"], n["lef_y"], n["n_c"]
    iy = b * h ** 3 / 12.0
    ncr = math.pi ** 2 * row["e05"] / MPA * iy / lef ** 2
    m2 = (n.get("m_y", 0) + nc * lef / (300.0 if row["product"] == "solid" else 500.0)) / max(1.0 - nc / ncr, 1e-6)
    u = (nc / (b * h) / (_fd(case, row, row["fc0"]) / MPA)) ** 2 + m2 / (b * h * h / 6) / _bending_fd(case, row, "kh_dim", h)
    return {"m_ii": abs(m2) / 1e3, "second_order": 9.99 if nc >= ncr else u}


def cat_t32(case):
    """🏔️ T-32 tension perpendicular to the grain at the apex (6.4.3): sigma_t,90 = k_p 6 M/(b h^2) [- 0.6 p_d/b], f = k_dis k_vol f_t,90,d."""
    n, row, t = case["num"], _row(case), case["text"]
    a, hr = math.tan(math.radians(n.get("alpha_ap_deg", 0.0))), n["h_ap"] / (n["r_in"] + 0.5 * n["h_ap"])
    kp = 0.2 * a + (0.25 - 1.5 * a + 2.6 * a * a) * hr + (2.1 * a - 4.0 * a * a) * hr * hr
    cambered = t.get("shape", "double_tapered") == "pitched_cambered"
    volume = min(n.get("v_apex", n["b"] * n["h_ap"] ** 2 * (1 - 0.25 * a)), 2.0 / 3.0 * n["v_beam"])
    sd = kp * 6 * n["m_ap"] / (n["b"] * n["h_ap"] ** 2) - (0.6 * n.get("p_d", 0.0) / n["b"] if cambered else 0.0)
    fd = (1.7 if cambered else 1.4) * (1e7 / volume) ** 0.2 * _fd(case, row, row["ft90"]) / MPA
    out = {"apex_tension90": util(sd, fd)}
    if "tau_d" in n:
        out["apex_shear_tension"] = n["tau_d"] / (_fd(case, row, row["fv"]) / MPA) + sd / fd
    return out


def _apex_kp_stress(case):
    n = case["num"]
    a, hr = math.tan(math.radians(n.get("alpha_ap_deg", 0.0))), n["h_ap"] / (n["r_in"] + 0.5 * n["h_ap"])
    kp = 0.2 * a + (0.25 - 1.5 * a + 2.6 * a * a) * hr + (2.1 * a - 4.0 * a * a) * hr * hr
    return kp * 6 * n["m_ap"] / (n["b"] * n["h_ap"] ** 2)


def _rod_resistance(case, row):
    """🔩️ Axial resistance of n glued-in rods: n R_ax,k k_mod / 1.3."""
    n = case["num"]
    return n["n_rods"] * n["r_ax_k"] * _kmod(case, row) / n.get("gamma_m_conn", 1.3)


def cat_t33(case):
    """🔩️ T-33 apex reinforcement: F_t,90 = 0.5 sigma_t,90 b h_ap against the rods."""
    n, row = case["num"], _row(case)
    return {"apex_reinforcement": util(0.5 * _apex_kp_stress(case) * n["b"] * n["h_ap"], _rod_resistance(case, row))}


def cat_t46(case):
    """🔩️ T-46 notch reinforcement: F_t,90 = 1.3 V [3 (1 - alpha)^2 - 2 (1 - alpha)^3] against the rods."""
    n, row = case["num"], _row(case)
    a = 1.0 - n["h_ef"] / n["h"]
    return {"notch_reinforcement": util(1.3 * n["v_d"] * (3 * a * a - 2 * a ** 3), _rod_resistance(case, row))}


def cat_t50(case):
    """🪜️ T-50 system strength: k_sys = 1.1 for four or more load-sharing members at spacing <= 1.2 m (6.6)."""
    n, row = case["num"], _row(case)
    ks = 1.1 if n["n_members"] >= 4 and n["spacing_m"] <= 1.2 and n.get("distributing", 1.0) > 0.5 else 1.0
    return {"system_bending": util(n["m_y"] / (n["b"] * n["h"] ** 2 / 6), ks * _bending_fd(case, row, "kh_dim", n["h"]))}


def _beam_midspan(load_case, load, l, ei):
    """🏗️ Bending deflection of a statically determinate beam by anastruct (Euler-Bernoulli, N and mm)."""
    from anastruct import SystemElements

    ss = SystemElements(EA=1e15, EI=ei)
    if load_case.startswith("cantilever"):
        ss.add_element([[0, 0], [l, 0]])
        ss.add_support_fixed(1)
        if load_case == "cantilever_udl":
            ss.q_load(q=-load, element_id=1)
        else:
            ss.point_load(node_id=2, Fy=-load)
        ss.solve()
        return abs(ss.get_node_displacements(node_id=2)["uy"])
    ss.add_element([[0, 0], [l / 2, 0]])
    ss.add_element([[l / 2, 0], [l, 0]])
    ss.add_support_hinged(1)
    ss.add_support_roll(3)
    if load_case == "udl_ss":
        ss.q_load(q=-load, element_id=[1, 2])
    else:
        ss.point_load(node_id=2, Fy=-load)
    ss.solve()
    return abs(ss.get_node_displacements(node_id=2)["uy"])


def _shear_deflection(load_case, load, l, g, a, kappa):
    """✂️ Shear deformation by the unit-load integral kappa * int V v /(G A) dx, integrated numerically."""
    from scipy import integrate

    shear = {"udl_ss": lambda x: load * (l / 2 - x), "point_mid": lambda x: load / 2 * (1 if x < l / 2 else -1), "cantilever_udl": lambda x: load * (l - x), "cantilever_point": lambda x: load}[load_case]
    unit = {"udl_ss": lambda x: 0.5 if x < l / 2 else -0.5, "point_mid": lambda x: 0.5 if x < l / 2 else -0.5, "cantilever_udl": lambda x: 1.0, "cantilever_point": lambda x: 1.0}[load_case]
    return kappa * integrate.quad(lambda x: shear(x) * unit(x), 0.0, l, points=[l / 2], limit=200)[0] / (g * a)


def _tapered_udl_deflection(q, l, b, ha, hb, e, g, kappa):
    """📐️ Mid-span deflection of a simply supported tapered beam by the unit-load method (scipy quad)."""
    from scipy import integrate

    h = lambda x: ha + (hb - ha) * x / l
    mom, shear = (lambda x: q * x * (l - x) / 2), (lambda x: q * (l / 2 - x))
    um, us = (lambda x: min(x, l - x) / 2), (lambda x: 0.5 if x < l / 2 else -0.5)
    return integrate.quad(lambda x: mom(x) * um(x) / (e * b * h(x) ** 3 / 12) + kappa * shear(x) * us(x) / (g * b * h(x)), 0.0, l, points=[l / 2], limit=200)[0]


def _deflections(case):
    """🧮️ Instantaneous deflections of the permanent and variable load and the effective length of the limit."""
    n, row, t = case["num"], _row(case), case["text"]
    b, h, l, kappa = n["b"], n["h"], n["l"], n.get("kappa", 1.2)
    e, g = row["e0"] / MPA, row["g"] / MPA
    lc = t.get("load_case", "udl_ss")

    def one(load):
        if lc == "udl_ss_tapered":
            return _tapered_udl_deflection(load, l, b, n.get("h_a", h), n.get("h_b", h), e, g, kappa)
        return _beam_midspan(lc, load, l, e * b * h ** 3 / 12) + _shear_deflection(lc, load, l, g, b * h, kappa)

    return one(n["load_g"]), one(n.get("load_q", 0.0)), (2.0 * l if lc.startswith("cantilever") else l)


def cat_t60(case):
    """📉️ T-60 instantaneous deflection (7.2) with the shear contribution, limit l/300 (l_eff = 2 l for cantilevers)."""
    wg, wq, leff = _deflections(case)
    return {"w_inst_g": wg / 1000.0, "w_inst_q": util(wq, leff / case["num"].get("limit_divisor", 300.0))}


def cat_t61(case):
    """📈️ T-61 final and net final deflection, w_fin = w_G (1 + k_def) + w_Q (1 + psi_2 k_def) (2.2.3, 7.2)."""
    n, t = case["num"], case["text"]
    wg, wq, leff = _deflections(case)
    kdef = n.get("k_def", K_DEF_PRODUCT[t.get("material", _row(case)["product"])][service_class(n.get("sc", 1)) - 1])
    w_fin = wg * (1 + kdef) + wq * (1 + n.get("psi_2", 0.3) * kdef) + n.get("w_q2", 0.0) * (n.get("psi_0_2", 0.0) + n.get("psi_2_2", 0.0) * kdef)
    net_divisor = n.get("limit_net_divisor", 200.0 if case["annex"] == "de" else 250.0)
    return {"w_fin": util(w_fin, leff / n.get("limit_fin_divisor", 150.0)), "w_net_fin": util(w_fin - n.get("w_c", 0.0), leff / net_divisor)}


def _gamma_parts(n, l, k):
    """🔗️ Annex B (B.1-B.5): parts and connection efficiency gamma_i of a two- or three-part beam."""
    parts = [(n[f"a{i}"], n[f"i{i}"], n[f"e{i}"], n[f"h{i}"], n.get(f"s{i}", 1.0)) for i in (1, 2, 3) if f"a{i}" in n]
    gam = [1.0 / (1.0 + math.pi ** 2 * p[2] * p[0] * p[4] / (k * l * l)) if i != 1 else 1.0 for i, p in enumerate(parts)]
    return parts, gam


def _ei_gamma(parts, gam):
    ea = [g * p[2] * p[0] for g, p in zip(gam, parts)]
    h = [p[3] for p in parts]
    a2 = (ea[0] * (h[0] + h[1]) / 2 - (ea[2] * (h[1] + h[2]) / 2 if len(parts) > 2 else 0.0)) / sum(ea)
    a = [(h[0] + h[1]) / 2 - a2, a2] + ([(h[1] + h[2]) / 2 + a2] if len(parts) > 2 else [])
    return sum(p[2] * p[1] + g * p[2] * p[0] * ai * ai for p, g, ai in zip(parts, gam, a))


def cat_t62(case):
    """🪢️ T-62 slip contribution by the gamma-method (Annex B): w = 5 q l^4/(384 (EI)_ef) against l/300."""
    n = case["num"]
    parts, gam = _gamma_parts(n, n["l"], n["k_ser"])
    ei, rigid = _ei_gamma(parts, gam), _ei_gamma(parts, [1.0] * len(parts))
    return {"w_slip": util(5 * n["load_g"] * n["l"] ** 4 / (384 * ei), n["l"] / n.get("limit_divisor", 300.0)), "slip_stiffness_ratio": ei / rigid}


def _floor_bef(n):
    return min(n["width_m"], n["span_m"] / 1.1 * (n["ei_b"] / n["ei_l"]) ** 0.25)


def cat_t64(case):
    """👣️ T-64 unit point load deflection of the floor strip over the effective width b_ef (7.3.3 (3)), against a mm/kN."""
    n = case["num"]
    force = n.get("force_n", 1000.0)
    w_mm = force * n["span_m"] ** 3 / (48 * n["ei_l"] * _floor_bef(n)) * 1000.0
    return {"w_unit": util(w_mm * 1000.0 / force, n["a_mm_per_kn"])}


def cat_t65(case):
    """⚡️ T-65 unit impulse velocity response v = 4 (0.4 + 0.6 n40)/(m b l + 200) against b^(f1 zeta - 1) (7.3.3)."""
    n = case["num"]
    l, bw, ei_l, ei_b, m, z = n["span_m"], n["width_m"], n["ei_l"], n["ei_b"], n["mass_kg_m2"], n.get("zeta", 0.01)
    f1 = math.pi / (2 * l * l) * math.sqrt(ei_l / m)
    n40 = (max((40.0 / f1) ** 2 - 1.0, 0.0) * (bw / l) ** 4 * ei_l / ei_b) ** 0.25
    v = 4.0 * (0.4 + 0.6 * n40) / (m * bw * l + 200.0)
    return {"velocity": util(v, n.get("b_vel", 100.0) ** (f1 * z - 1.0))}


def cat_t66(case):
    """🌉️ T-66 walking excitation of a footbridge (EN 1995-2 7.3): f1, vertical and horizontal acceleration of n pedestrians."""
    n, t = case["num"], case["text"]
    l, z = n["span_m"], n.get("zeta", 0.015)
    f1 = math.pi / (2 * l * l) * math.sqrt(n["ei_nm2"] / n["mass_kg_m"])
    big_m = n["mass_kg_m"] * l / 2.0
    a1v, a1h = (200.0 if f1 <= 2.5 else 100.0) / (big_m * z), 50.0 / (big_m * z)
    peds = n.get("n_peds", n.get("crowd_density", 0.0) * l * n.get("deck_width_m", 0.0))
    av, ah = (0.23 * a1v * peds, 0.18 * a1h * peds) if peds > 1 else (a1v, a1h)
    out = {"f1": f1, "a_vert": util(0.0 if f1 > 5.0 else av, 0.7), "a_hor": util(0.0 if n.get("f_lat_hz", 3.0) > 2.5 else ah, 0.4 if t.get("comfort", "normal") == "crowd" else 0.2)}
    if t.get("crowd_model") == "setra":
        out["a_crowd"] = cat_t120(case)["a_crowd"]
    return out


def cat_t124(case):
    """🧰️ T-124 combined bridge assessment: the footbridge comfort quantities and the Miner sum of the stress-range blocks."""
    out = cat_t66(case)
    if "ds1" in case["num"]:
        out.update(cat_t121(case))
    return out


def _johansen_nail(case):
    """🔩️ Yield moment of a round nail/dowel (EN 1995-1-1 8.5.1 (8.14)); returns d, M_y,Rk, kind."""
    n, t = case["num"], case["text"]
    d, kind = n["d"], t.get("kind", "dowel")
    my = 0.45 * n["f_u"] * d ** 2.6 if kind == "nail_square" else 0.3 * n["f_u"] * d ** 2.6
    return d, my, kind


def _fh(kind, rho, d, alpha_deg=0.0, wood="softwood"):
    """🪵️ Embedment strength (8.15, 8.16, 8.31): small nails and staples d <= 8 mm without angle, otherwise f_h,0 / (k90 sin^2 + cos^2)."""
    if kind in ("nail", "staple") and d <= 8:
        return 0.082 * rho * d ** -0.3
    k90 = {"lvl": 1.30 + 0.015 * d, "hardwood": 0.90 + 0.015 * d}.get(wood, 1.35 + 0.015 * d)
    a = math.radians(alpha_deg)
    return 0.082 * (1 - 0.01 * d) * rho / (k90 * math.sin(a) ** 2 + math.cos(a) ** 2)


def _rope(j, fax, share):
    return j + min(fax / 4.0, share * j)


def _johansen_rk(config, kind, t1, t2, tsteel, d, fh1, fh2, my, fax):
    """🏛️ Governing characteristic load-carrying capacity per shear plane of EN 1995-1-1 8.2.2 (8.6, 8.7) and 8.2.3 (8.9-8.13)."""
    share = {"nail": 0.15, "nail_predrilled": 0.15, "nail_square": 0.25, "screw": 1.0, "bolt": 0.25}.get(kind, 0.0)
    beta = fh2 / fh1
    if config == "tt_double":
        j = 1.05 * fh1 * t1 * d / (2 + beta) * (math.sqrt(2 * beta * (1 + beta) + 4 * beta * (2 + beta) * my / (fh1 * d * t1 * t1)) - beta)
        k = 1.15 * math.sqrt(2 * beta / (1 + beta)) * math.sqrt(2 * my * fh1 * d)
        return min(fh1 * t1 * d, 0.5 * fh2 * t2 * d, _rope(j, fax, share), _rope(k, fax, share))
    if config == "tt_single":
        r = t2 / t1
        c = fh1 * t1 * d / (1 + beta) * (math.sqrt(beta + 2 * beta ** 2 * (1 + r + r * r) + beta ** 3 * r * r) - beta * (1 + r))
        dm = 1.05 * fh1 * t1 * d / (2 + beta) * (math.sqrt(2 * beta * (1 + beta) + 4 * beta * (2 + beta) * my / (fh1 * d * t1 * t1)) - beta)
        e = 1.05 * fh1 * t2 * d / (1 + 2 * beta) * (math.sqrt(2 * beta ** 2 * (1 + beta) + 4 * beta * (1 + 2 * beta) * my / (fh1 * d * t2 * t2)) - beta)
        f = 1.15 * math.sqrt(2 * beta / (1 + beta)) * math.sqrt(2 * my * fh1 * d)
        return min(fh1 * t1 * d, fh2 * t2 * d, _rope(c, fax, share), _rope(dm, fax, share), _rope(e, fax, share), _rope(f, fax, share))
    thick = lambda t: (fh1 * t * d, _rope(fh1 * t * d * (math.sqrt(2 + 4 * my / (fh1 * d * t * t)) - 1), fax, share), _rope(2.3 * math.sqrt(my * fh1 * d), fax, share))
    ratio = min(max((tsteel / d - 0.5) / 0.5, 0.0), 1.0)
    if config == "double_center":
        return min(thick(t1))
    if config == "double_outer":
        thin = min(0.5 * fh1 * t1 * d, _rope(1.15 * math.sqrt(2 * my * fh1 * d), fax, share))
        return thin + ratio * (min(0.5 * fh1 * t1 * d, _rope(2.3 * math.sqrt(my * fh1 * d), fax, share)) - thin)
    thin = min(0.4 * fh1 * t1 * d, _rope(1.15 * math.sqrt(2 * my * fh1 * d), fax, share))
    return thin + ratio * (min(thick(t1)) - thin)


def _connection_cat(case, config):
    n, row, t = case["num"], _row(case), case["text"]
    d, my, kind = _johansen_nail(case)
    rho1 = n.get("rho_k1", row["rho"])
    rho2 = n.get("rho_k2", props(t["class_2"])["rho"] if "class_2" in t else rho1)
    fh1, fh2 = _fh(kind, rho1, d, n.get("alpha1_deg", 0.0)), _fh(kind, rho2, d, n.get("alpha2_deg", 0.0))
    rk = _johansen_rk(config, kind, n["t1"], n.get("t2", 0.0), n.get("t_steel", 0.0), d, fh1, fh2, my, n.get("f_ax_rk", 0.0))
    kmod1 = _kmod(case, row)
    kmod = math.sqrt(kmod1 * n["kmod_2"]) if "kmod_2" in n else kmod1
    planes = n.get("shear_planes", 2.0 if config in ("tt_double", "double_center", "double_outer") else 1.0)
    total = kmod * rk / 1.3 * planes * n.get("n_ef", 1.0)
    out = {"f_v_rk": rk, "f_v_rd": total}
    if "f_ed" in n:
        out["lateral"] = util(n["f_ed"], total)
    return out


def cat_t72(case):
    """🔩️ T-72 timber-to-timber double shear (8.2.2, modes g-k)."""
    return _connection_cat(case, "tt_double")


def cat_t73(case):
    """🔩️ T-73 steel-to-timber (8.2.3, modes a-e, thin/thick plate interpolation)."""
    return _connection_cat(case, case["text"].get("config", "single"))


def cat_t75(case):
    """🧮️ T-75 effective number of fasteners in a row, n_ef = n^k_ef (Table 8.1) or EN 1995-1-1 (8.34) for bolts and dowels."""
    n, t = case["num"], case["text"]
    import numpy as np

    kind, nn, a1, d = t.get("kind", "dowel"), n["n"], n["a1"], n["d"]
    r = a1 / d
    if kind in ("nail", "nail_predrilled", "staple", "screw"):
        if n.get("predrilled", 0.0) > 0.5 or kind == "nail_predrilled":
            kef = float(np.interp(r, [4.0, 7.0, 10.0], [0.7, 0.85, 1.0]))
        else:
            kef = float(np.interp(r, [4.0, 7.0, 10.0, 14.0], [0.5, 0.7, 0.85, 1.0]))
        nef = nn ** kef
    elif kind == "screw_axial":
        nef = nn ** 0.9
    else:
        par = min(nn, nn ** 0.9 * (a1 / (13.0 * d)) ** 0.25)
        nef = par + min(abs(n.get("alpha_deg", 0.0)) / 90.0, 1.0) * (nn - par)
    out = {"n_ef": nef}
    if "f_v_rk" in n:
        out["f_v_ef_rk"] = nef * n["f_v_rk"]
    return out


def cat_t76(case):
    """📌️ T-76 axial withdrawal / head pull-through of nails (8.23), washers of bolts (8.5.2) and screws (8.38)."""
    n, t = case["num"], case["text"]
    kind, d, rho = t.get("kind", "dowel"), n["d"], n["rho_k"]
    if kind in ("nail", "nail_predrilled"):
        fax, fhead = 20e-6 * rho * rho, 70e-6 * rho * rho
        dh, th = n.get("d_head", 2 * d), n.get("t_head_side", 0.0)
        rk = min(fax * d * n["t_pen"], fax * d * th + fhead * dh * dh)
    elif kind == "bolt":
        rk = min(3.0 * n["f_c_90_k"] * math.pi / 4.0 * (n["d_washer"] ** 2 - (d + 1.0) ** 2), n.get("f_tens_k", math.inf))
    else:
        a = math.radians(n.get("alpha_deg", 90.0))
        l_ef, nef = n["l_ef"], n.get("n_ef", 1.0)
        fax_k = 0.52 * d ** -0.5 * l_ef ** -0.1 * 350.0 ** 0.8
        thread = nef * fax_k * d * l_ef * min(d / 8.0, 1.0) / (1.2 * math.cos(a) ** 2 + math.sin(a) ** 2) * (rho / 350.0) ** 0.8
        head = nef * n.get("f_head_k", 8.0) * n.get("d_head", 2 * d) ** 2 * (rho / 350.0) ** 0.8
        rk = min(thread, head, n.get("f_tens_k", math.inf))
    out = {"f_ax_rk": rk}
    if "f_ax_ed" in n:
        out["axial"] = util(n["f_ax_ed"], n["kmod"] * rk / 1.3)
    return out


def cat_t77(case):
    """➕ T-77 combined lateral and axial load: linear (8.27) for nails, quadratic (8.28) for screws."""
    n = case["num"]
    fa, fv = n["f_ax_ed"] / n["f_ax_rd"], n["f_v_ed"] / n["f_v_rd"]
    return {"combined": fa * fa + fv * fv if case["text"].get("kind", "dowel") == "screw" else fa + fv}


def cat_t78(case):
    """🪓️ T-78 splitting F_90,Rk = 14 b w sqrt(h_e/(1 - h_e/h)) (8.4) and block shear (A.1)."""
    n, row = case["num"], _row(case)
    kmod = _kmod(case, row)
    out = {}
    if "f_90_ed" in n:
        out["splitting"] = util(n["f_90_ed"], kmod * 14.0 * n["b"] * n.get("w", 1.0) * math.sqrt(n["he"] / (1.0 - n["he"] / n["h"])) / 1.3)
    if "a_net_t" in n and "a_net_v" in n:
        rk = max(1.5 * n["a_net_t"] * row["ft0"] / MPA, 0.7 * n["a_net_v"] * row["fv"] / MPA)
        out["block_shear"] = util(n["f_ed"], kmod * rk / 1.3)
    return out


def cat_t79(case):
    """🪛️ T-79 steel plate in a timber connection: net section 0.9 A_net f_u/1.25 and bearing k_1 alpha_b f_u d t/1.25 (EN 1993-1-8)."""
    n = case["num"]
    k1 = min(2.8 * n["e2"] / n["d0"] - 1.7, 2.5)
    ab = min(n["e1"] / (3.0 * n["d0"]), n.get("f_u_bolt", 800.0) / n["f_u_plate"], 1.0)
    return {"plate_net": util(n["f_ed"], 0.9 * n["a_net"] * n["f_u_plate"] / 1.25), "plate_bearing": util(n["f_ed"] / n.get("n_bolts", 1.0), k1 * ab * n["f_u_plate"] * n["d"] * n["t_plate"] / 1.25)}


def cat_t80(case):
    """💍️ T-80 split rings and shear plates (8.9): F_v,0,Rk = min(k1 k2 k3 k4 35 d_c^1.5; k1 k3 h_e 31.5 d_c), reduced for the angle."""
    n = case["num"]
    dc, he, rho = n["d_c"], n["h_e"], n["rho_k"]
    k1 = min(n.get("t_a1", 3 * he) / (3 * he), n.get("t_a2", 5 * he) / (5 * he), 1.0)
    k2, k3 = min(n.get("a3t", 1.5 * dc) / (1.5 * dc), 1.0), min(rho / 350.0, 1.75)
    k4 = 1.1 if case["text"].get("members", "timber") == "steel" else 1.0
    f0 = min(k1 * k2 * k3 * k4 * 35.0 * dc ** 1.5, k1 * k3 * he * 31.5 * dc)
    a = math.radians(n.get("alpha_deg", 0.0))
    fa = f0 / ((1.3 + 0.001 * dc) * math.sin(a) ** 2 + math.cos(a) ** 2)
    out = {"f_v_alpha_rk": fa}
    if "f_ed" in n:
        out["connector"] = util(n["f_ed"], n["kmod"] * fa / 1.3)
    return out


def cat_t81(case):
    """🦷️ T-81 toothed-plate connectors (8.10): F_v,0,Rk = 18 k1 k2 k3 k4 d_c^1.5."""
    n = case["num"]
    f0 = 18.0 * n.get("k1", 1.0) * n.get("k2", 1.0) * n.get("k3", 1.0) * n.get("k4", 1.0) * n["d_c"] ** 1.5
    out = {"f_v_0_rk": f0}
    if "f_ed" in n:
        out["toothed_plate"] = util(n["f_ed"], n["kmod"] * f0 / 1.3)
    return out


def cat_t82(case):
    """📎️ T-82 punched metal plate fasteners: min(k_mod f_a,k A_ef, f_t,k l_plate)/1.25 (8.8)."""
    n = case["num"]
    return {"nail_plate": util(n["f_ed"], min(n["kmod"] * n["f_a_k"] * n["a_ef"], n["f_t_k_plate"] * n["l_plate"]) / 1.25)}


def _ei_sectionproperties(sections, mesh=6.0):
    """🧱️ Elastic centroid height and EI of a stack of rectangles [(width, height, E)] from the bottom, by sectionproperties (finite element mesh)."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre import Material
    from sectionproperties.pre.library import rectangular_section

    y, geometry = 0.0, None
    for width, height, modulus in sections:
        piece = rectangular_section(d=height, b=width, material=Material("m", modulus, 0.3, 1.0, 1.0, "grey")).shift_section(-width / 2.0, y)
        geometry = piece if geometry is None else geometry + piece
        y += height
    geometry.create_mesh([mesh])
    section = Section(geometry)
    section.calculate_geometric_properties()
    return section.get_c()[1], float(section.get_eic()[0])


def cat_t91(case):
    """🏗️ T-91 glued thin-webbed I-beam (9.1.1): normal stress in the flanges, web shear and glue line, EI from a composite finite element section."""
    n, row = case["num"], _row(case)
    bf, hf, bw, hw = n["b_f"], n["h_f"], n["b_w"], n["h_w"]
    nw = n.get("e_w", 1.0) / n.get("e_f", 1.0)
    _, ei = _ei_sectionproperties([(bf, hf, 1.0), (bw, hw, nw), (bf, hf, 1.0)], mesh=min(bf, hf, bw) / 4.0)
    z = (hw + hf) / 2.0
    s_f = bf * hf * z
    s_max = s_f + nw * bw * hw * hw / 8.0
    m, v = n["m_d"], n.get("v_d", 0.0)
    fd = lambda key: _fd(case, row, row[key]) / MPA
    out = {
        "flange_mean_c": util(m * z / ei, min(fd("fc0"), fd("ft0"))),
        "flange_max": util(m * (hw / 2.0 + hf) / ei, k_h(hf / 1000.0, row["product"], row["rho"]) * fd("fm")),
        "web_shear": util(v * s_max / (ei * bw), n.get("f_v_web_d", fd("fv"))),
        "web_slenderness": util(hw, 70.0 * bw),
        "ei": ei,
    }
    if "f_glue_d" in n:
        out["glue_line"] = util(v * s_f / (ei * n.get("b_glue", bw)), n["f_glue_d"])
    return out


def cat_t92(case):
    """🏗️ T-92 glued thin-flanged T-beam (9.1.2): effective flange width min(0.1 l; 20/25 h_f; outstand) and the stresses of the transformed section."""
    n, row, t = case["num"], _row(case), case["text"]
    l, hf, bw, hw = n["l"], n["h_f"], n["b_w"], n["h_w"]
    compression = t.get("flange_side", "compression") == "compression"
    bef = bw + 2.0 * min(0.1 * l, (20.0 if compression else 25.0) * hf, n["outstand"])
    ratio = n.get("e_f", 1.0) / n.get("e_w", 1.0)
    na, ei = _ei_sectionproperties([(bw, hw, 1.0), (bef, hf, ratio)], mesh=min(bw, hf) / 4.0)
    y_f = hw + hf / 2.0
    fd = lambda key: _fd(case, row, row[key]) / MPA
    return {
        "b_ef": util(bef, max(bef, 1.0)),
        "flange": util(abs(ratio * n["m_d"] * (y_f - na) / ei), n["f_flange_d"]),
        "web_edge": util(abs(n["m_d"] * na / ei), max(fd("ft0"), fd("fc0"))),
    }


def cat_t94(case):
    """🧱️ T-94 wall racking capacity F_i,v,Rd = F_f,Rd b_i c_i / s (9.2.4.2) and diaphragm shear q = F/l (9.2.3)."""
    n, t = case["num"], case["text"]
    out = {}
    if all(k in n for k in ("f_f_rd", "b_i", "h", "s")):
        c_i = min(n["b_i"] / (n["h"] / 2.0), 1.0)
        fi = (n.get("k_d", 1.0) * n.get("k_q", 1.0) * n.get("k_n", 1.0) * n.get("k_op", 1.0) if t.get("method", "A") == "B" else 1.0) * n.get("n_panels", 1.0) * n["f_f_rd"] * n["b_i"] * c_i / n["s"]
        if "f_v_ed" in n:
            out["racking"] = util(n["f_v_ed"], fi)
            if "r_hold_down" in n:
                out["hold_down"] = util(max(n["f_v_ed"] * n["h"] / n["b_i"] - n.get("q_v", 0.0) * n["b_i"] / 2.0, 0.0), n["r_hold_down"])
    if all(k in n for k in ("f_d", "l_dia", "q_v_rd")):
        out["diaphragm_shear"] = util(n["f_d"] / n["l_dia"], n["q_v_rd"])
        if all(k in n for k in ("m_dia", "b_dia", "n_chord_rd")):
            out["chord"] = util(n["m_dia"] / n["b_dia"], n["n_chord_rd"])
    return out


def _bracing(case):
    """🪢️ Stabilising load q_d l = k_l n N_d / k_f,3 (9.37, 9.38; n members to be braced, N_d the force of each) and F_d = N_d / k_f,1 (solid timber) or N_d / k_f,2 (glued laminated timber, LVL) (9.35); k_f,1, k_f,2, k_f,3 and the l/500 limit of 9.2.5.3(2) from the registry rows `bracing.*`."""
    n, annex = case["num"], case["annex"]
    kl = min(1.0, math.sqrt(ndp("bracing.kl-reference-length", annex) / n["l_m"]))
    k_f = ndp("bracing.force-divisor-solid" if case["text"].get("product", "solid") == "solid" else "bracing.force-divisor-engineered", annex)
    return kl * n["n_braced"] * n["n_d"] / ndp("bracing.stabilising-divisor", annex), n["n_d"] / k_f


def cat_t95(case):
    """🪢️ T-95 stabilising load q_d = k_l n N_d/(k_f,3 l) in N/m and stabilising force F_d of a single member in compression (9.2.5.2(3), 9.2.5.3(1))."""
    n = case["num"]
    force, fd = _bracing(case)
    out = {"q_d": force / n["l_m"], "f_d": fd}
    if "q_rd" in n:
        out["bracing_capacity"] = util(out["q_d"], n["q_rd"])
    return out


def cat_t96(case):
    """🪢️ T-96 bracing force q_d l against F_Rd and the bracing deflection F/k against l/500 (9.2.5.3(2))."""
    n = case["num"]
    fd = _bracing(case)[0]
    out = {"bracing_force": util(fd, n["f_rd"])}
    if "k_br" in n:
        out["bracing_deflection"] = util(fd / n["k_br"], n["l_m"] * 1000.0 / ndp("bracing.deflection-limit-divisor", case["annex"]))
    return out


def cat_t98(case):
    """🌍️ T-98 behaviour factors q of EN 1998-1 Table 8.1 per structural system and ductility class."""
    table = {"portal_dowelled": (1.5, 2.5, 4.0), "nailed_wall": (1.5, 3.0, 5.0), "glued_wall": (1.5, 2.0, 2.0), "truss_nailed": (1.5, 2.0, 3.0), "cantilever": (1.5, 2.0, 2.0)}
    return {"q": table[case["text"]["system"]][("DCL", "DCM", "DCH").index(case["text"].get("ductility", "DCM"))]}


def _char_rates(material, rho_k, hp):
    """🔥️ Notional charring rates beta_0, beta_n of EN 1995-1-2 Table 3.1 and 3.4.2 (hardwood interpolated by density, panels (3.2))."""
    if material == "hardwood":
        t = min(max((rho_k - 290.0) / 160.0, 0.0), 1.0)
        return 0.65 - 0.15 * t, 0.70 - 0.15 * t
    if material == "panel":
        b0 = 0.9 * math.sqrt(450.0 / rho_k) * math.sqrt(20.0 / hp)
        return b0, b0
    return {"softwood_solid": (0.65, 0.80), "glulam": (0.65, 0.70), "lvl": (0.65, 0.70), "clt": (0.65, 0.65)}[material]


def _char_material(case):
    return case["text"].get("char_material") or {"glulam": "glulam", "lvl": "lvl", "clt": "clt"}.get(_row(case)["product"], "softwood_solid")


def cat_t100(case):
    """🔥️ T-100 charring depths d_char,0 = beta_0 t and d_char,n = beta_n t (EN 1995-1-2 3.4.2)."""
    n = case["num"]
    b0, bn = _char_rates(_char_material(case), n.get("rho_k", 350.0), n.get("h_p", 20.0))
    out = {"beta_0": b0, "beta_n": bn, "d_char_0": b0 * n["t_min"] / 1000.0, "d_char_n": bn * n["t_min"] / 1000.0}
    if n.get("series", 0.0) > 0.5:
        out.update({f"d_char_n_{m}": bn * m / 1000.0 for m in (15, 30, 60, 90, 120)})
    return out


def _fire_util(case, row, t, rate):
    """🔥️ Bending utilisation of the three-sided exposed rectangle after t minutes: d_ef = rate t + k_0 d_0 with k_0 = min(t/20, 1), d_0 = 7 mm."""
    n = case["num"]
    d_ef = rate * t + min(t / 20.0, 1.0) * 7.0
    b, h = n["b"] - d_ef * (n.get("face_left", 1.0) + n.get("face_right", 1.0)), n["h"] - d_ef * (n.get("face_top", 0.0) + n.get("face_bottom", 1.0))
    if b <= 0 or h <= 0:
        return 9.99
    return n.get("m_fi", 0.0) / (b * h * h / 6.0) / (K_FI[row["product"]] * row["fm"] / MPA)


def cat_t102(case):
    """🔥️ T-102 zero-strength layer method (EN 1995-1-2 4.2.2): d_ef = beta_0 t + k_0 d_0; fire time from the root of utilisation = 1."""
    from scipy import optimize

    n, row = case["num"], _row(case)
    b0, bn = _char_rates(_char_material(case), n.get("rho_k", row["rho"]), n.get("h_p", 20.0))
    u = lambda t, rate: _fire_util(case, row, t, rate)
    if u(240.0, b0) <= 1.0:
        t_u = 240.0
    elif u(1e-9, b0) > 1.0:
        t_u = 0.0
    else:
        t_u = optimize.brentq(lambda t: u(t, b0) - 1.0, 1e-9, 240.0, xtol=1e-9)
    return {"fire_resistance": u(n["t_req"], b0), "fire_time": n["t_req"] / max(t_u, 1e-6), "method_comparison": u(n["t_req"], bn)}


def cat_t103(case):
    """🛡️ T-103 protected members (EN 1995-1-2 3.4.3): start of charring t_ch = 2.8 h_p - 14, cladding failure t_f and the char depth."""
    n = case["num"]
    b0, _ = _char_rates(_char_material(case), n.get("rho_k", 400.0), 20.0)
    t_ch = max(2.8 * n["h_p"] - 14.0, 0.0)
    t_f = t_ch if case["text"].get("board", "F") == "F" else 0.8 * t_ch
    k2, t = 1.0 - 0.018 * n["h_p"], n["t_min"]
    t_fm = max(t_f, t_ch)
    depth = 0.0 if t <= t_ch else k2 * b0 * (t - t_ch) if t <= t_fm else k2 * b0 * (t_fm - t_ch) + 2.0 * b0 * (t - t_fm)
    return {"t_ch": t_ch * 60.0, "t_f": t_f * 60.0, "d_char_protected": depth / 1000.0}


def cat_t105(case):
    """🧱️ T-105 fire-separating assembly (EN 1995-1-2 Annex E): sum of the layer protection times against t_req."""
    n, t = case["num"], case["text"]
    total = 0.0
    for k in range(1, int(n["n_layers"]) + 1):
        hp, kind = n[f"h{k}"], t.get(f"kind{k}", "gypsum_f")
        total += hp / n.get("beta_0", 0.65) if kind == "timber" else 0.5 * hp ** 1.4 if kind == "insulation" else max(2.8 * hp - 14.0, 0.0)
    return {"separating_function": util(n["t_req"], max(total, 1e-6))}


def cat_t123(case):
    """🌉️ T-123 bridge connection: F_Ed against k_mod n F_v,Rk/gamma_M (EN 1995-2 / 8.2)."""
    n = case["num"]
    return {"bridge_connection": util(n["f_ed"], n["kmod"] * n["f_v_rk"] * n.get("n", 1.0) / 1.3)}


ORACLE_CASES_MORE = {
    "T-01": cat_t01, "T-02": cat_t02, "T-04": cat_t04, "T-05": cat_t05, "T-06": cat_t06, "T-10": cat_t10, "T-12": cat_t12, "T-15": cat_t15, "T-16": cat_t16, "T-17": cat_t17, "T-18": cat_t18,
    "T-21": cat_t21, "T-23": cat_t23, "T-24": cat_t24, "T-32": cat_t32, "T-33": cat_t33, "T-46": cat_t46, "T-50": cat_t50,
    "T-60": cat_t60, "T-61": cat_t61, "T-62": cat_t62, "T-64": cat_t64, "T-65": cat_t65, "T-66": cat_t66, "T-124": cat_t124,
    "T-72": cat_t72, "T-73": cat_t73, "T-75": cat_t75, "T-76": cat_t76, "T-77": cat_t77, "T-78": cat_t78, "T-79": cat_t79, "T-80": cat_t80, "T-81": cat_t81, "T-82": cat_t82,
    "T-91": cat_t91, "T-92": cat_t92, "T-94": cat_t94, "T-95": cat_t95, "T-96": cat_t96, "T-98": cat_t98,
    "T-100": cat_t100, "T-102": cat_t102, "T-103": cat_t103, "T-105": cat_t105, "T-123": cat_t123,
}
ORACLE_CASES.update(ORACLE_CASES_MORE)


def catalogue(doc):
    """📇️ Evaluate each `{entry, case}` with the independent formulations; keys are the outcome keys."""
    out = []
    for item in doc["cases"]:
        case = dict(item["case"])
        case["annex"] = annex_of({"annex": case.get("annex", "En")})
        case["entry"] = item["entry"]
        out.append({"entry": item["entry"], "values": ORACLE_CASES[item["entry"]](case)})
    return {"cases": out}


def _curve_t20(case, at):
    """🏛️ k_c as the smaller root of lambda_rel² k² - 2 k_ps k + 1 = 0 found with numpy, instead of the closed quadratic form."""
    import numpy as np
    row = _row(case)
    beta = case["num"].get("beta_c", 0.2 if row["product"] == "solid" else 0.1)

    def kc(rel):
        if rel <= 0.3:
            return 1.0
        roots = np.roots([rel * rel, -(1 + beta * (rel - 0.3) + rel * rel), 1.0])
        return float(min(r.real for r in roots if abs(r.imag) < 1e-12 and r.real > 0))

    return [kc(x) for x in at]


def _curve_t22(case, at):
    """🌀️ k_crit over the effective length with the simplified critical stress 0.78 b² E/(h l_ef), the three branches of 6.34 as a numpy piecewise."""
    import numpy as np
    n, row = case["num"], _row(case)
    rel = np.sqrt(row["fm"] / MPA / (0.78 * n["b"] ** 2 * row["e05"] / MPA / (n["h"] * np.asarray(at, dtype=float))))
    k = np.piecewise(rel, [rel <= 0.75, (rel > 0.75) & (rel <= 1.4), rel > 1.4], [1.0, lambda r: 1.56 - 0.75 * r, lambda r: 1.0 / (r * r)])
    return [1.0 if n.get("restrained", 0.0) > 0.5 else float(v) for v in k]


def _curve_t61(case, at):
    """⏳ w_fin over k_def from Euler-Bernoulli plus Timoshenko shear of a simply supported beam under two uniform loads."""
    import numpy as np
    n, row = case["num"], _row(case)
    b, h, l = n["b"], n["h"], n["l"]
    e, g, i, a = row["e0"] / MPA, row["g"] / MPA, b * h ** 3 / 12.0, b * h

    def w(q):
        return 5 * q * l ** 4 / (384 * e * i) + 1.2 * q * l * l / (8 * g * a)

    wg, wq, psi2 = w(n["load_g"]), w(n.get("load_q", 0.0)), n.get("psi_2", 0.3)
    return list(wg * (1 + np.asarray(at, dtype=float)) + wq * (1 + psi2 * np.asarray(at, dtype=float)))


def _curve_t70(case, at):
    """🔩️ f_h,alpha,k of a bolt in softwood over the angle (8.31) with numpy trigonometry."""
    import numpy as np
    n = case["num"]
    d = n["d"]
    f0 = 0.082 * (1 - 0.01 * d) * n["rho_k1"]
    a = np.radians(np.asarray(at, dtype=float))
    return list(f0 / ((1.35 + 0.015 * d) * np.sin(a) ** 2 + np.cos(a) ** 2))


def _curve_t103(case, at):
    """🛡️ Char depth behind the cladding over time: three linear branches built with numpy.piecewise."""
    import numpy as np
    n = case["num"]
    b0, _ = _char_rates(_char_material(case), n.get("rho_k", 400.0), 20.0)
    t_ch = max(2.8 * n["h_p"] - 14.0, 0.0)
    t_f = max(t_ch if case["text"].get("board", "F") == "F" else 0.8 * t_ch, t_ch)
    k2 = 1.0 - 0.018 * n["h_p"]
    t = np.asarray(at, dtype=float)
    return list(np.piecewise(t, [t <= t_ch, (t > t_ch) & (t <= t_f), t > t_f], [0.0, lambda x: k2 * b0 * (x - t_ch), lambda x: k2 * b0 * (t_f - t_ch) + 2.0 * b0 * (x - t_f)]))


def _curve_t45(case, at):
    """✂️ k_v over the notch depth `at` (mm) from 6.62 with numpy: alpha = (h − a)/h, the slope term 1 + 1.1 i^1.5/√h and the cap 1 (k_n 4.5 LVL, 5 solid, 6.5 glulam); 1 without a support-side notch."""
    import numpy as np
    n, row = case["num"], _row(case)
    h, x, incl = n["h"], n.get("x", 0.0), n.get("incl", 0.0)
    kn = {"lvl": 4.5, "glulam": 6.5}.get(row["product"], 5.0)
    a = 1.0 - np.asarray(at, dtype=float) / h
    safe = np.clip(a, 1e-9, 1.0 - 1e-12)
    den = np.sqrt(h) * (np.sqrt(safe * (1 - safe)) + 0.8 * x / h * np.sqrt(1 / safe - safe * safe))
    k = np.minimum(1.0, kn * (1 + 1.1 * incl ** 1.5 / np.sqrt(h)) / den)
    return [1.0 if case["text"].get("side", "support") != "support" or v >= 1.0 else float(kv) for v, kv in zip(a, k)]


def _curve_t46(case, at):
    """🔩️ F_t,90,d / V_d = 1.3 (3 (1 − α)² − 2 (1 − α)³) of the reinforced notch over the notch depth `at` (mm) with numpy polynomials."""
    import numpy as np
    h = case["num"]["h"]
    a = np.asarray(at, dtype=float) / h
    return list(1.3 * np.polyval([-2.0, 3.0, 0.0, 0.0], a))


def _curve_t31(case, at):
    """🌀️ k_l of the apex bending (6.43 – 6.47) over the inner radius `at` (mm): k_1 + k_2 h/r + k_3 (h/r)² + k_4 (h/r)³ with numpy's polynomial evaluation, h/r = h_ap / (r_in + h_ap/2)."""
    import numpy as np
    n = case["num"]
    t = np.tan(np.radians(n.get("alpha_ap_deg", 0.0)))
    hr = n["h_ap"] / (np.asarray(at, dtype=float) + 0.5 * n["h_ap"])
    return list(np.polynomial.polynomial.polyval(hr, [1 + 1.4 * t + 5.4 * t * t, 0.35 - 8 * t, 0.6 + 8.3 * t - 7.8 * t * t, 6 * t * t]))


def _curve_t32(case, at):
    """🌀️ k_p of the apex tension perpendicular (6.48 – 6.51) over the inner radius `at` (mm): k_5 + k_6 h/r + k_7 (h/r)² with numpy's polynomial evaluation."""
    import numpy as np
    n = case["num"]
    t = np.tan(np.radians(n.get("alpha_ap_deg", 0.0)))
    hr = n["h_ap"] / (np.asarray(at, dtype=float) + 0.5 * n["h_ap"])
    return list(np.polynomial.polynomial.polyval(hr, [0.2 * t, 0.25 - 1.5 * t + 2.6 * t * t, 2.1 * t - 4 * t * t]))


def _curve_t33(case, at):
    """🌀️ k_r of the apex bending (6.49) over the inner radius `at` (mm): 0.76 + 0.001 r/t below r/t = 240, else 1, with numpy.where."""
    import numpy as np
    ratio = np.asarray(at, dtype=float) / case["num"]["t_lam"]
    return list(np.where(ratio >= 240.0, 1.0, 0.76 + 0.001 * ratio))


def _moment_line(load, share, span, x):
    """📈️ The relative bending moment of a member under the load case at the distances `x` (mm) with numpy, positive where the moment has the sign of its peak and normalised to the peak (a value below zero turns against it, the field of a continuous beam): simply supported and cantilever closed forms; for `two_span_udl` and `two_span_point` (two equal spans, the point load at the middle of each span, `span` = both spans) the support moment M_B of the three-moment equation 4 L M_B = -(q L³/2) resp. -(3 P L²/4), the end reaction R_A = V_0 + M_B / L and M(x) = R_A x - loads, so the line comes from statics and not from the closed forms of the Rust profile."""
    import numpy as np
    s = np.clip(np.asarray(x, dtype=float) / span, 0.0, 1.0)
    share = 1.0 if load in ("point_mid", "cantilever_point", "two_span_point") else share
    if load.startswith("cantilever"):
        line, point = (1.0 - s) ** 2, 1.0 - s
    elif load.startswith("two_span"):
        length = span / 2.0
        xs = np.minimum(s * span, span - s * span)
        mb_q, mb_p = -length ** 2 / 8.0, -3.0 * length / 16.0
        ra_q, ra_p = length / 2.0 + mb_q / length, 0.5 + mb_p / length
        m_q = ra_q * xs - xs ** 2 / 2.0
        m_p = ra_p * xs - np.maximum(xs - length / 2.0, 0.0)
        line, point = -m_q / abs(mb_q), -m_p / abs(mb_p)
    else:
        line, point = 4.0 * s * (1.0 - s), 2.0 * np.minimum(s, 1.0 - s)
    return (1.0 - share) * line + share * point


def _taper_length(case):
    """📏️ The member length of a single tapered beam: the span `l`, twice for the two equal spans of `two_span_udl` and `two_span_point`."""
    return case["num"]["l"] * (2.0 if case["text"]["load_case"].startswith("two_span") else 1.0)


def _taper_edges(case, x):
    """🪚️ Tapered-edge and parallel-edge utilisation sigma(x) / (k_m,alpha f_m,d) and sigma(x) / f_m,d of a single tapered beam at the distances `x` (mm) from the shallow end with numpy: h(x) = h + x tan(alpha), sigma = 6 |M(x)| / (b h²) with `_moment_line` normalised to the peak `m_d` (udl_ss / udl, point_mid, cantilever_udl, cantilever_point, two_span_udl, two_span_point mixed by `point_share`; `l` is the span, both spans of a continuous beam are swept), k_h and f_m,d at the depth h(x), and k_m,alpha = 1 / sqrt(1 + (f_m,d tan(alpha) / (r f_v,d))² + (f_m,d tan²(alpha) / f_perp,d)²) of 6.38 (r = 0.75, edge in tension, f_perp = f_t,90,d) or 6.39 (r = 1.5, edge in compression, f_perp = f_c,90,d); the edge state of `side` holds where the moment has the sign of the peak and is the other one where it turns against it (the field of a continuous beam). There is no constant-moment form: a case without `l` and `load_case` raises KeyError."""
    import numpy as np
    n, t, row = case["num"], case["text"], _row(case)
    x = np.asarray(x, dtype=float)
    tan = np.tan(np.radians(n["alpha_deg"]))
    h = n["h"] + x * tan
    kh = n["kh"] if "kh" in n else np.array([k_h(v / 1000.0, row["product"], row["rho"]) for v in np.atleast_1d(h)]).reshape(h.shape)
    fmd = kh * _fd(case, row, row["fm"]) / MPA
    fvd = _fd(case, row, row["fv"]) / MPA
    rel = _moment_line(t["load_case"], n.get("point_share", 0.0), _taper_length(case), x)
    tension = (t.get("side", "tension") == "tension") ^ (rel < 0.0)
    fperp = np.where(tension, _fd(case, row, row["ft90"]), _fd(case, row, row["fc90"])) / MPA
    shear = np.where(tension, 0.75, 1.5)
    k = 1.0 / np.sqrt(1.0 + (fmd / (shear * fvd) * tan) ** 2 + (fmd / fperp * tan * tan) ** 2)
    sigma = 6.0 * n["m_d"] * np.abs(rel) / (n["b"] * h * h)
    return sigma / (k * fmd), sigma / fmd


def _taper_ratios(case, x):
    """🪚️ The tapered-edge utilisation of `_taper_edges` at the distances `x` (mm)."""
    return _taper_edges(case, x)[0]


def _taper_governing(case):
    """🎯️ The governing section of a single tapered beam on a 200001-point grid over the member length (`_taper_length`): `(x, tapered, parallel)` at the maximum of the tapered-edge utilisation."""
    import numpy as np
    grid = np.linspace(0.0, _taper_length(case), 200001)
    tapered, parallel = _taper_edges(case, grid)
    i = int(np.argmax(tapered))
    return float(grid[i]), float(tapered[i]), float(parallel[i])


def _curve_t30(case, at):
    """🪚️ The tapered-edge utilisation of `_taper_ratios` along the span at the abscissae `at` (mm)."""
    return list(_taper_ratios(case, at))


def _curve_t30_governing(case, at):
    """🎯️ The governing section of a single tapered beam: `[x, utilisation]` of `_taper_governing`, `at` unused."""
    x, tapered, _ = _taper_governing(case)
    return [x, tapered]


def _curve_t30_statics(case, at):
    """⚖️ The load of a member from its peak forces by statics, `[line load q (N/m), point load P (N), point share of the peak moment]`: the peak moment and shear of a unit line load and of a unit midspan point load come from numerical beam statics (simply supported, cantilever, `two_span` with the three-moment equation) and the 2x2 system [M_q M_P; V_q V_P] [q P] = [M V] is solved with numpy.linalg.solve; `l` is one span in m, `m_d` in N·m and `v_d` in N."""
    import numpy as np
    n, support = case["num"], case["text"]["support"]
    length = n["l"]
    grid = np.linspace(0.0, 1.0, 40001)
    load = {"simply_supported": ("udl_ss", "point_mid", 1.0), "cantilever": ("cantilever_udl", "cantilever_point", 1.0), "continuous_two_span": ("two_span_udl", "two_span_point", 2.0)}[support]
    peak = {}
    for kind, name in zip(("q", "p"), load[:2]):
        m_peak = {"udl_ss": length ** 2 / 8.0, "point_mid": length / 4.0, "cantilever_udl": length ** 2 / 2.0, "cantilever_point": length, "two_span_udl": length ** 2 / 8.0, "two_span_point": 3.0 * length / 16.0}[name]
        v_peak = {"udl_ss": length / 2.0, "point_mid": 0.5, "cantilever_udl": length, "cantilever_point": 1.0, "two_span_udl": 5.0 * length / 8.0, "two_span_point": 11.0 / 16.0}[name]
        line = np.abs(_moment_line(name, 0.0 if kind == "q" else 1.0, load[2] * length, grid * load[2] * length))
        assert abs(line.max() - 1.0) < 1e-9
        peak[kind] = (m_peak, v_peak)
    matrix = np.array([[peak["q"][0], peak["p"][0]], [peak["q"][1], peak["p"][1]]])
    q, p = np.linalg.solve(matrix, [n["m_d"], n["v_d"]])
    return [float(q), float(p), float(p * peak["p"][0] / (q * peak["q"][0] + p * peak["p"][0]))]


CURVES = {"T-30": _curve_t30, "T-30-governing": _curve_t30_governing, "T-30-statics": _curve_t30_statics, "T-20": _curve_t20, "T-22": _curve_t22, "T-61": _curve_t61, "T-70": _curve_t70, "T-103": _curve_t103, "T-45": _curve_t45, "T-46": _curve_t46, "T-31": _curve_t31, "T-32": _curve_t32, "T-33": _curve_t33}


def curves(doc):
    """📈️ The response curves of the diagrams (`case-buckling`, `case-ltb`, `case-deflection`, `case-embedment`, `case-cladding`) at the abscissae `at` of each `{entry, case, at}`, derived independently of the Rust probes."""
    out = []
    for item in doc["cases"]:
        case = dict(item["case"])
        case["annex"] = annex_of({"annex": case.get("annex", "En")})
        out.append({"entry": item["entry"], "values": [float(v) for v in CURVES[item["entry"]](case, item["at"])]})
    return {"cases": out}


def joint(doc):
    """🏗️ Two-part composite (slab on timber) by the Newmark formulation with numpy: the series axial stiffness EA_s and gamma_eff = 1/(1 + pi² EA_s s/(K l²)) give (EI)_ef, the normal force N = gamma_eff EA_s z M/(EI)_ef, the fibre stresses -N/A -+ E kappa h/2 and the fastener force N' s with the slip F/K, for K_ser, the ultimate K_u = 2/3 K_ser and the rigid composite; independent of the Annex B form of the Rust kernel."""
    import numpy as np

    out = []
    for item in doc["cases"]:
        n = item["num"]
        ea = np.array([n["e1"] * n["a1"], n["e2"] * n["a2"]])
        eas, z = ea.prod() / ea.sum(), (n["h1"] + n["h2"]) / 2.0

        def solve(k, rigid=False):
            g = 1.0 if rigid else 1.0 / (1.0 + np.pi**2 * eas * n["s1"] / (k * n["l"] ** 2))
            ei = n["e1"] * n["i1"] + n["e2"] * n["i2"] + g * eas * z * z
            force, kappa = g * eas * z * n["m_d"] / ei, n["m_d"] / ei
            top1, bottom1 = -force / n["a1"] - n["e1"] * kappa * n["h1"] / 2, -force / n["a1"] + n["e1"] * kappa * n["h1"] / 2
            top2, bottom2 = force / n["a2"] - n["e2"] * kappa * n["h2"] / 2, force / n["a2"] + n["e2"] * kappa * n["h2"] / 2
            fastener = g * eas * z * n["v_d"] / ei * n["s1"]
            return {"ei": ei, "part1": [top1, bottom1], "part2": [top2, bottom2], "fastener": fastener, "slip": fastener / k}

        out.append({"service": solve(n["k"]), "ultimate": solve(2.0 / 3.0 * n["k"]), "rigid": solve(n["k"], True)})
    return {"cases": out}


def document_cases(doc):
    """📇️ The document's catalogue cases in the `{entry, case}` shape of [`catalogue`]."""
    annex = "De" if annex_of(doc) == "de" else "En"
    return {"cases": [{"entry": c["entryId"], "case": {"annex": annex, "num": {n["id"]: n["value"] for n in c.get("numbers", [])}, "text": {t["id"]: t["value"] for t in c.get("texts", [])}}} for c in doc.get("cases", [])]}


def main():
    """🚪️ `--json` reads a snapshot on stdin; without it, runs the k_cr DE/EN shear smoke asserts."""
    if "--json" in sys.argv:
        print(json.dumps(evaluate(json.load(sys.stdin))))
        return 0
    if "--document-cases" in sys.argv:
        print(json.dumps(catalogue(document_cases(json.load(sys.stdin)))))
        return 0
    if "--joint" in sys.argv:
        print(json.dumps(joint(json.load(sys.stdin))))
        return 0
    if "--curves" in sys.argv:
        print(json.dumps(curves(json.load(sys.stdin))))
        return 0
    if "--catalogue" in sys.argv:
        print(json.dumps(catalogue(json.load(sys.stdin))))
        return 0
    u_de = util(1.5 * 15000 / (k_cr("de", 4e6) * 0.2 * 0.3), 0.8 * 4e6 / 1.3)
    u_en = util(1.5 * 15000 / (k_cr("en", 4e6) * 0.2 * 0.3), 0.8 * 4e6 / 1.3)
    assert abs(u_de - 0.2438) < 0.01, u_de
    assert abs(u_en - 0.2274) < 0.01, u_en
    print(json.dumps({"ok": True, "shear_de": u_de, "shear_en": u_en}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
