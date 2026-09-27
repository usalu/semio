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

import json
import math
import sys

MPA = 1e6
GPA = 1e9


def _solid(fm, ft0, ft90, fc0, fc90, fv, e0, e05, e90, g, rho, product):
    """🧱️ One EN 338 / EN 14080 row in SI units."""
    return dict(fm=fm * MPA, ft0=ft0 * MPA, ft90=ft90 * MPA, fc0=fc0 * MPA, fc90=fc90 * MPA, fv=fv * MPA,
                e0=e0 * GPA, e05=e05 * GPA, e90=e90 * GPA, g=g * GPA, rho=rho, product=product)


STRENGTH = {
    "C14": _solid(14, 8, 0.4, 16, 2.0, 3.0, 7.0, 4.7, 0.23, 0.44, 290, "solid"),
    "C16": _solid(16, 10, 0.5, 17, 2.2, 3.2, 8.0, 5.4, 0.27, 0.50, 310, "solid"),
    "C18": _solid(18, 11, 0.5, 18, 2.2, 3.4, 9.0, 6.0, 0.30, 0.56, 320, "solid"),
    "C20": _solid(20, 12, 0.5, 19, 2.3, 3.6, 9.5, 6.4, 0.32, 0.59, 330, "solid"),
    "C22": _solid(22, 13, 0.5, 20, 2.4, 3.8, 10.0, 6.7, 0.33, 0.63, 340, "solid"),
    "C24": _solid(24, 14, 0.5, 21, 2.5, 4.0, 11.0, 7.4, 0.37, 0.69, 350, "solid"),
    "C27": _solid(27, 16, 0.6, 22, 2.6, 4.0, 11.5, 7.7, 0.38, 0.72, 370, "solid"),
    "C30": _solid(30, 18, 0.6, 23, 2.7, 4.0, 12.0, 8.0, 0.40, 0.75, 380, "solid"),
    "C35": _solid(35, 21, 0.6, 25, 2.8, 3.8, 13.0, 8.7, 0.43, 0.81, 400, "solid"),
    "C40": _solid(40, 24, 0.6, 26, 2.9, 3.8, 14.0, 9.4, 0.47, 0.88, 420, "solid"),
    "C45": _solid(45, 27, 0.6, 27, 3.1, 3.8, 15.0, 10.0, 0.50, 0.94, 440, "solid"),
    "C50": _solid(50, 30, 0.6, 29, 3.2, 3.8, 16.0, 10.7, 0.53, 1.00, 460, "solid"),
    "GL20H": _solid(20, 16, 0.5, 20, 2.5, 3.5, 8.4, 7.0, 0.30, 0.54, 340, "glulam"),
    "GL22H": _solid(22, 17.6, 0.5, 22, 2.5, 3.5, 10.5, 8.8, 0.30, 0.65, 370, "glulam"),
    "GL24H": _solid(24, 19.2, 0.5, 24, 2.5, 3.5, 11.5, 9.6, 0.30, 0.72, 385, "glulam"),
    "GL24C": _solid(24, 17, 0.5, 21.5, 2.5, 3.5, 11.0, 9.1, 0.30, 0.69, 350, "glulam"),
    "GL26H": _solid(26, 20.8, 0.5, 26, 2.5, 3.5, 12.0, 10.0, 0.30, 0.75, 405, "glulam"),
    "GL26C": _solid(26, 19, 0.5, 23, 2.5, 3.5, 11.5, 9.6, 0.30, 0.72, 385, "glulam"),
    "GL28H": _solid(28, 22.3, 0.5, 28, 2.5, 3.5, 12.6, 10.5, 0.30, 0.78, 425, "glulam"),
    "GL28C": _solid(28, 19.5, 0.5, 24, 2.5, 3.5, 12.5, 10.4, 0.30, 0.78, 390, "glulam"),
    "GL30H": _solid(30, 24, 0.5, 30, 2.5, 3.5, 13.6, 11.3, 0.30, 0.85, 430, "glulam"),
    "GL30C": _solid(30, 19.5, 0.5, 24.5, 2.5, 3.5, 13.0, 10.8, 0.30, 0.81, 390, "glulam"),
    "GL32H": _solid(32, 25.6, 0.5, 32, 2.5, 3.5, 14.2, 11.8, 0.30, 0.89, 440, "glulam"),
    "GL32C": _solid(32, 19.5, 0.5, 24.5, 2.5, 3.5, 13.5, 11.2, 0.30, 0.84, 400, "glulam"),
    "LVL32": _solid(36, 26, 0.9, 35, 6.0, 4.5, 14.0, 11.6, 0.5, 0.6, 510, "lvl"),
    "CLT100": _solid(24, 14, 0.5, 21, 2.5, 4.0, 11.0, 7.4, 0.37, 0.69, 400, "clt"),
}
ALIASES = {"LVL-32": "LVL32", "CLT": "CLT100"}

DURATIONS = ("permanent", "long", "medium", "short", "instantaneous")
K_MOD = {1: (0.60, 0.70, 0.80, 0.90, 1.10), 2: (0.60, 0.70, 0.80, 0.90, 1.10), 3: (0.50, 0.55, 0.65, 0.70, 0.90)}
K_DEF = {1: 0.60, 2: 0.80, 3: 2.00}
K_FI = {"solid": 1.25, "glulam": 1.15, "lvl": 1.1, "clt": 1.15}
BETA_N_MM_PER_MIN = {"solid": 0.80, "glulam": 0.70, "lvl": 0.70, "clt": 0.65}
GAMMA_G, GAMMA_Q = 1.35, 1.5


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
    """🛡️ EN 1995-1-1 Table 2.3 (glulam 1.25, LVL 1.2, others 1.3); DIN EN 1995-1-1/NA Table NA.2 sets 1.3 throughout."""
    if annex == "de":
        return 1.3
    return {"glulam": 1.25, "lvl": 1.2}.get(product, 1.3)


def k_cr(annex, fv):
    """✂️ Crack factor: EN 0.67, DIN EN 1995-1-1/NA min(1, 2.5/f_v,k[MPa])."""
    if annex == "en":
        return 0.67
    f = fv / MPA
    return 1.0 if f <= 0 else min(1.0, 2.5 / f)


def k_h(h, product):
    """📏️ Depth factor §3.2 / §3.3."""
    hmm = h * 1000.0
    if product == "solid":
        return 1.0 if hmm >= 150 else min((150.0 / max(hmm, 1.0)) ** 0.2, 1.3)
    return 1.0 if hmm >= 600 else min((600.0 / max(hmm, 1.0)) ** 0.1, 1.1)


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


def effective_m_crit_nm(member):
    """🪢 M_crit scaled by lateral-restraint spacing (§6.3.3)."""
    span = max(num(member, "spanM"), 1e-6)
    base = max(num(member, "mCritNm"), 1.0)
    spacing = num(member, "lateralRestraintSpacingM")
    if spacing <= 0:
        return base
    l_ef = max(min(spacing, span), 1e-6)
    return base * (span / l_ef) ** 2


def k_c(lam_rel):
    """🏛️ Column buckling §6.3.2 with β_c = 0.2."""
    if lam_rel <= 0.3:
        return 1.0
    k = 0.5 * (1 + 0.2 * (lam_rel - 0.3) + lam_rel ** 2)
    return 1.0 / (k + math.sqrt(max(k * k - lam_rel ** 2, 0.0)))


def lambda_rel_c(lam, fc0, e05):
    """📐️ Relative slenderness for compression."""
    return 0.0 if e05 <= 0 else lam / math.pi * math.sqrt(fc0 / e05)


def k_c90(bearing, support):
    """🦶️ §6.1.5 effective contact length ratio, clamped to [1, 1.5]."""
    l = max(bearing, 1e-6)
    a1 = min(0.03, l, max(support, 0.0))
    a2 = min(0.03, l)
    return max(min((l + a1 + a2) / l, 1.5), 1.0)


def k_v(h, hn, x):
    """🪚️ Notched-beam reduction §6.5.2 with k_n = 5."""
    if hn <= 0 or h <= 0:
        return 1.0
    alpha = min(max((h - hn) / h, 0.05), 1.0)
    if alpha >= 0.999:
        return 1.0
    hmm = h * 1000.0
    term = 5.0 * (1 + 1.1 / math.sqrt(hmm))
    denom = math.sqrt(hmm) * max((1 - alpha) - (max(x, 0.0) / h) ** 2, 1e-6) + term * max(math.sqrt(alpha * (1 - alpha)), 1e-6)
    return max(min(term / denom, 1.0), 0.05)


def psi(kind, category):
    """𝜓️ EN 1990 Table A1.1 (ψ₀, ψ₁, ψ₂)."""
    k = str(kind or "").strip().lower()
    c = str(category or "").strip().upper()
    if k == "permanent":
        return 1.0, 1.0, 1.0
    if k == "accidental":
        return 0.0, 0.0, 0.0
    if k == "snow":
        return 0.5, 0.2, 0.0
    if k == "snow_high":
        return 0.7, 0.5, 0.2
    if k == "wind":
        return 0.6, 0.2, 0.0
    if k == "imposed":
        if c in ("C", "D"):
            return 0.7, 0.7, 0.6
        if c == "E":
            return 1.0, 0.9, 0.8
        if c == "H":
            return 0.0, 0.0, 0.0
    return 0.7, 0.5, 0.3


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
        if support == "cantilever":
            m, v = q * l * l / 2 + f * l, q * l + f
        elif support == "continuousTwoSpan":
            m, v = q * l * l / 14 + f * l / 6, 0.6 * (q * l + f)
        else:
            m, v = q * l * l / 8 + f * l / 4, q * l / 2 + f / 2
        n = q * l + f if member.get("role") == "column" else 0.0
        return dict(m=m, v=v, n=n, nt=0.0, fc90=max(v, 0.0), q=q, f=f)
    return dict(m=num(action, "mKNm"), v=num(action, "vKN"), n=num(action, "nKN"), nt=num(action, "nTKN"),
                fc90=num(action, "fC90KN"), q=0.0, f=0.0)


def combine(terms):
    """➕️ Σ factor·internals."""
    out = dict.fromkeys(FORCES, 0.0)
    for factor, i in terms:
        for key in FORCES:
            out[key] += factor * i[key]
    return out


def combinations(member):
    """🔀️ EN 1990 eq. 6.10 ULS, SLS characteristic / frequent(ψ₁) / quasi-permanent and accidental combinations."""
    actions = member.get("actions") or []
    if not actions:
        return []
    ins = [internals(member, a) for a in actions]
    perm = [i for i, a in enumerate(actions) if family(a.get("kind")) == "permanent"]
    acc = [i for i, a in enumerate(actions) if family(a.get("kind")) == "accidental"]
    var = [i for i, a in enumerate(actions) if family(a.get("kind")) == "variable"]
    psis = {i: psi(actions[i].get("kind"), actions[i].get("category")) for i in var}

    def shortest(idx):
        return max((duration_rank(actions[i].get("loadDuration")) for i in idx), default=DURATIONS.index("medium"))

    out = []
    if not var:
        out.append(dict(id="uls.g", kind="uls", rank=shortest(perm), psi2=1.0, **combine([(GAMMA_G, ins[i]) for i in perm])))
        g = combine([(1.0, ins[i]) for i in perm])
        out.append(dict(id="sls.char.g", kind="char", rank=shortest(perm), psi2=1.0, **g))
        out.append(dict(id="sls.freq.g", kind="freq", rank=shortest(perm), psi2=1.0, **g))
        out.append(dict(id="sls.qp.g", kind="qp", rank=shortest(perm), psi2=1.0, **g))
    else:
        rank = shortest(perm + var)
        for lead in var:
            others = [v for v in var if v != lead]
            uls = [(GAMMA_G, ins[i]) for i in perm] + [(GAMMA_Q, ins[lead])] + [(GAMMA_Q * psis[v][0], ins[v]) for v in others]
            out.append(dict(id=f"uls.6.10.lead.{actions[lead]['id']}", kind="uls", rank=rank, psi2=psis[lead][2], **combine(uls)))
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


def connection_uls(conn):
    """🔗️ Governing (rank, F_Ed) over the connection's ULS combinations, `None` without actions."""
    actions = conn.get("actions") or []
    perm = [a for a in actions if family(a.get("kind")) == "permanent"]
    var = [a for a in actions if family(a.get("kind")) == "variable"]
    if not actions:
        return None
    g = sum(num(a, "fKN") for a in perm)
    rank = max((duration_rank(a.get("loadDuration")) for a in perm + var), default=DURATIONS.index("medium"))
    if not var:
        return rank, GAMMA_G * g
    candidates = []
    for lead in var:
        fed = GAMMA_G * g + GAMMA_Q * num(lead, "fKN")
        fed += sum(GAMMA_Q * psi(v.get("kind"), "")[0] * num(v, "fKN") for v in var if v is not lead)
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
    """🔥️ EN 1995-1-2 §4.2.2 reduced cross-section, k_mod,fi = γ_M,fi = 1."""
    t = num(member, "fireDurationS")
    beta = BETA_N_MM_PER_MIN[p["product"]] * 1e-3 / 60
    d_ef = beta * t + min(1.0, t / 1200) * 0.007
    b_fi, h_fi = max(num(member, "bM") - 2 * d_ef, 0.0), max(num(member, "hM") - 2 * d_ef, 0.0)
    m_rd = K_FI[p["product"]] * p["fm"] * b_fi * h_fi ** 2 / 6
    return {"id": f"en1995.1-2.4.fire.{member['id']}", "utilization": util(m_ed, max(m_rd, 1e-9))}


def bridge_checks(annex, member, p):
    """🌉️ EN 1995-2 Annex A fatigue, Annex B pedestrian comfort, ULS bending and SLS deflection."""
    mid = member["id"]
    b, h, l = num(member, "bM"), num(member, "hM"), num(member, "spanM")
    w = b * h * h / 6
    km = k_mod(member.get("serviceClass"), DURATIONS.index("medium"))
    g = gamma_m(annex, p["product"])
    q = max(num(member, "bridgeCrowdPerM2"), 0.0) * 800 * max(b, 0.05)
    coefficient = {"cantilever": 1 / 2, "continuousTwoSpan": 1 / 14}.get(member.get("support"), 1 / 8)
    m_crowd = q * l * l * coefficient
    m_ed = GAMMA_Q * m_crowd
    fmd = km * k_h(h, p["product"]) * k_crit(lambda_rel_m(w, p["fm"], effective_m_crit_nm(member))) * p["fm"] / g
    sigma_m = m_ed / w if w > 0 else 0.0
    delta = 0.4 * m_crowd / w if w > 0 else 0.0
    cycles = max(num(member, "bridgeNObs") * num(member, "bridgeTLYears"), 1.0)
    a_w = num(member, "bridgeA") if num(member, "bridgeA") > 0 else 15.0
    b_w = num(member, "bridgeB") if num(member, "bridgeB") > 0 else 4.0
    beta = num(member, "bridgeBeta") if num(member, "bridgeBeta") > 0 else 5.0
    n_r = max(10 ** (a_w - b_w * math.log10(max(delta / MPA, 1e-6))), 1.0)
    k_fat = min((n_r / cycles) ** (1 / beta), 1.0)
    ffat = km * k_fat * p["fm"] / g
    ei = p["e0"] * b * h ** 3 / 12
    mu = num(member, "massKgPerM") if num(member, "massKgPerM") > 0 else p["rho"] * b * h
    f1 = math.pi / (2 * l * l) * math.sqrt(ei / mu) if mu > 0 and ei > 0 and l > 0 else 1.0
    xi = num(member, "dampingXi") if num(member, "dampingXi") > 0 else 0.01
    a_vert = max(num(member, "bridgeCrowdPerM2"), 0.0) / 1.5 * (5 / max(f1, 0.5)) * math.sqrt(0.01 / xi) * 0.35
    out = [
        {"id": f"en1995.2.a.fatigue.{mid}", "utilization": util(delta, max(ffat, 1e-9))},
        {"id": f"en1995.2.b.avert.{mid}", "utilization": util(a_vert, 0.7)},
        {"id": f"en1995.2.b.ahor.{mid}", "utilization": util(0.3 * a_vert, 0.2)},
        {"id": f"en1995.2.uls.bending.{mid}", "utilization": util(sigma_m, max(fmd, 1e-9))},
    ]
    if l > 0:
        out.append({"id": f"en1995.2.sls.deflection.{mid}", "utilization": util(deflection(member, p, q, 0.0), l / 400)})
    return out


def member_checks(annex, member):
    """🪵️ All EN 1995-1-1 / 1-2 / 2 checks one member earns."""
    mid = member["id"]
    p = props(member.get("strengthClass", ""))
    if p is None:
        return [{"id": f"en1995.material.unknown.{mid}", "utilization": None}]
    if member.get("role") == "bridge":
        out = bridge_checks(annex, member, p)
        if num(member, "fireDurationS") > 0:
            ulses = [abs(c["m"]) for c in combinations(member) if c["kind"] == "uls"]
            out.append(fire_check(member, p, max(ulses, default=0.0)))
        return out
    combos = combinations(member)
    ulses = [c for c in combos if c["kind"] == "uls"]
    if not ulses:
        return []
    uls = last_max(ulses, lambda c: abs(c["m"]))
    b, h = num(member, "bM"), num(member, "hM")
    sc = service_class(member.get("serviceClass"))
    km = k_mod(sc, uls["rank"])
    g = gamma_m(annex, p["product"])
    area, w = b * h, b * h * h / 6
    kcrit = k_crit(lambda_rel_m(w, p["fm"], effective_m_crit_nm(member)))
    fmd = km * k_h(h, p["product"]) * kcrit * p["fm"] / g
    sigma_m = uls["m"] / w if w > 0 else 0.0
    out = [{"id": f"en1995.6.1.6.bending.{mid}", "utilization": util(sigma_m, max(fmd, 1e-9))}]
    b_ef = k_cr(annex, p["fv"]) * b
    kv = k_v(h, num(member, "notchDepthM"), num(member, "notchDistanceM"))
    tau = 1.5 * uls["v"] / (b_ef * h) / kv if b_ef * h > 0 else 0.0
    out.append({"id": f"en1995.6.1.7.shear.{mid}", "utilization": util(tau, max(km * p["fv"] / g, 1e-9))})
    if uls["nt"] > 0:
        out.append({"id": f"en1995.6.1.2.tension.{mid}", "utilization": util(uls["nt"] / area if area > 0 else 0.0, max(km * p["ft0"] / g, 1e-9))})
    if uls["n"] > 0:
        iy = math.sqrt(b * h ** 3 / 12 / max(area, 1e-12))
        iz = math.sqrt(h * b ** 3 / 12 / max(area, 1e-12))
        lam_y = num(member, "bucklingLengthYM") / iy if iy > 0 else 0.0
        lam_z = num(member, "bucklingLengthZM") / iz if iz > 0 else 0.0
        kc = min(k_c(lambda_rel_c(lam_y, p["fc0"], p["e05"])), k_c(lambda_rel_c(lam_z, p["fc0"], p["e05"])))
        fcd = km * p["fc0"] / g
        sigma_c = uls["n"] / area if area > 0 else 0.0
        out.append({"id": f"en1995.6.3.2.compression.{mid}", "utilization": util(sigma_c, max(kc * fcd, 1e-9))})
        u_c = sigma_c / (kc * fcd) if kc * fcd > 0 else 0.0
        u_m = sigma_m / fmd if fmd > 0 else 0.0
        out.append({"id": f"en1995.6.2.4.combined.{mid}", "utilization": u_c ** 2 + u_m})
    bearing = num(member, "bearingLengthM")
    if uls["fc90"] > 0 and bearing > 0:
        fc90d = km * k_c90(bearing, num(member, "supportLengthM")) * p["fc90"] / g
        sigma90 = uls["fc90"] / max(b * bearing, 1e-12)
        out.append({"id": f"en1995.6.1.5.c90.{mid}", "utilization": util(sigma90, max(fc90d, 1e-9))})
    chars = [c for c in combos if c["kind"] == "char"]
    span = num(member, "spanM")
    if chars:
        sls = last_max(chars, lambda c: abs(c["q"]))
        if span > 0 and (sls["q"] != 0 or sls["f"] != 0):
            w_inst = deflection(member, p, sls["q"], sls["f"])
            w_fin = w_inst * (1 + K_DEF[sc] * sls["psi2"])
            out.append({"id": f"en1995.7.2.winst.{mid}", "utilization": util(w_inst, span / 300)})
            out.append({"id": f"en1995.7.2.wfin.{mid}", "utilization": util(w_fin, span / (200 if annex == "de" else 250))})
    freqs = [c for c in combos if c["kind"] == "freq"]
    if freqs:
        freq = last_max(freqs, lambda c: abs(c["q"]))
        if span > 0 and (freq["q"] != 0 or freq["f"] != 0):
            w_freq = deflection(member, p, freq["q"], freq["f"])
            out.append({"id": f"en1995.7.2.wfreq.{mid}", "utilization": util(w_freq, span / 300)})
    if member.get("role") == "floor":
        out.extend(floor_checks(annex, member, p))
    if num(member, "fireDurationS") > 0:
        out.append(fire_check(member, p, uls["m"]))
    return out


def floor_checks(annex, member, p):
    """🏠️ EN 1995-1-1 §7.3: f₁, w(1 kN) stiffness, and velocity (f₁≥8 Hz) or acceleration (f₁<8 Hz)."""
    b, h = num(member, "bM"), num(member, "hM")
    l = max(num(member, "spanM"), 1e-6)
    ei = p["e0"] * b * h ** 3 / 12
    if num(member, "massKgPerM") > 0:
        mu = num(member, "massKgPerM")
    elif num(member, "massKgPerM2") > 0:
        mu = num(member, "massKgPerM2") * max(b, 0.05)
    else:
        mu = p["rho"] * b * h
    factor = {"cantilever": 0.56, "continuousTwoSpan": 1.5}.get(member.get("support"), 1.0)
    f1 = factor * math.pi / (2 * l * l) * math.sqrt(ei / mu) if mu > 0 and ei > 0 else 0.0
    w1kn = deflection(member, p, 0.0, 1000.0)
    xi = num(member, "dampingXi") if num(member, "dampingXi") > 0 else 0.01
    m_star = mu * l / 2
    a = 1 / max(m_star * math.sqrt(xi), 1e-9) * max(f1 / 8, 0.1) * 0.25 if m_star > 0 and f1 > 0 else 0.0
    m_area = num(member, "massKgPerM2") if num(member, "massKgPerM2") > 0 else mu
    v = math.sqrt(math.pi / (0.8 * m_area * xi)) if m_area > 0 and xi > 0 else 0.0
    b_vel = 100.0
    v_lim = b_vel ** (f1 * xi - 1.0) if f1 > 0 and xi > 0 else 0.0
    w_lim = 0.0015 if annex == "de" else 0.0017
    out = [
        {"id": f"en1995.7.3.f1.{member['id']}", "utilization": util(8.0, max(f1, 1e-9))},
        {"id": f"en1995.7.3.stiffness.{member['id']}", "utilization": util(w1kn, w_lim)},
    ]
    if f1 + 1e-9 >= 8.0:
        out.append({"id": f"en1995.7.3.velocity.{member['id']}", "utilization": util(v, max(v_lim, 1e-12))})
    else:
        a_lim = 0.05 if annex == "de" else 0.10
        out.append({"id": f"en1995.7.3.acceleration.{member['id']}", "utilization": util(a, a_lim)})
    return out


def johansen_single(t1, t2, d, fh1, fh2, my, fax):
    """🔩️ §8.2.2 eq. 8.6 timber–timber single shear."""
    beta = fh2 / fh1 if fh1 > 0 else 1.0
    r = t1 / t2 if t2 > 0 else 1.0
    modes = [
        fh1 * t1 * d,
        fh2 * t2 * d,
        fh1 * t1 * d / (1 + beta) * (math.sqrt(beta + 2 * beta ** 2 * (1 + r + r * r) + beta ** 3 * r * r) - beta * (1 + r)),
        1.05 * fh1 * t1 * d / (2 + beta) * (math.sqrt(2 * beta * (1 + beta) + 4 * beta * (2 + beta) * my / max(fh1 * d * t1 * t1, 1e-18)) - beta),
        1.05 * fh1 * t2 * d / (1 + 2 * beta) * (math.sqrt(2 * beta ** 2 * (1 + beta) + 4 * beta * (1 + 2 * beta) * my / max(fh1 * d * t2 * t2, 1e-18)) - beta),
        1.15 * math.sqrt(2 * beta / (1 + beta)) * math.sqrt(2 * my * fh1 * d),
    ]
    base = min(modes[:2] + [max(x, 0.0) for x in modes[2:]])
    return base + min(fax / 4, 0.25 * base)


def johansen_double(t1, t2, d, fh1, fh2, my, fax):
    """🔩️ §8.2.2 eq. 8.7 timber–timber double shear (per plane)."""
    beta = fh2 / fh1 if fh1 > 0 else 1.0
    modes = [
        fh1 * t1 * d,
        0.5 * fh2 * t2 * d,
        max(1.05 * fh1 * t1 * d / (2 + beta) * (math.sqrt(2 * beta * (1 + beta) + 4 * beta * (2 + beta) * my / max(fh1 * d * t1 * t1, 1e-18)) - beta), 0.0),
        max(1.15 * math.sqrt(2 * beta / (1 + beta)) * math.sqrt(2 * my * fh1 * d), 0.0),
    ]
    base = min(modes)
    return base + min(fax / 4, 0.25 * base)


def johansen_steel_central(t, d, fh, my, fax):
    """🛡️ §8.2.3 eq. 8.10 thick plate, also eq. 8.13 central plate in double shear (per plane)."""
    bearing = fh * t * d
    base = max(min(bearing, bearing * (math.sqrt(2 + 4 * my / max(fh * d * t * t, 1e-18)) - 1), 2.3 * math.sqrt(my * fh * d)), 0.0)
    return base + min(fax / 4, 0.25 * base)


def johansen_steel(t, t_steel, d, fh, my, fax):
    """🔩️ §8.2.3 steel-to-timber single shear: eq. 8.9 thin (t_steel ≤ 0.5·d), eq. 8.10 thick (≥ d), linear between."""
    base = max(min(0.4 * fh * t * d, 1.15 * math.sqrt(2 * my * fh * d)), 0.0)
    thin = base + min(fax / 4, 0.25 * base)
    thick = johansen_steel_central(t, d, fh, my, fax)
    ratio = min(max((t_steel / max(d, 1e-12) - 0.5) / 0.5, 0.0), 1.0)
    return thin + ratio * (thick - thin)


def connection_checks(annex, conn):
    """🔗️ Johansen capacity and Table 8.2–8.5 spacing for one dowel-type connection."""
    cid = conn["id"]
    p = props(conn.get("strengthClass", ""))
    if p is None:
        return [{"id": f"en1995.conn.material.{cid}", "utilization": None}]
    governing = connection_uls(conn)
    if governing is None:
        return []
    rank, fed = governing
    fastener = str(conn.get("fastenerType", "")).lower()
    d = num(conn, "diameterM")
    t1, t2 = num(conn, "t1M"), num(conn, "t2M")
    dmm = d * 1000
    fh = (0.082 * p["rho"] * dmm ** -0.3 if "nail" in fastener and dmm < 8 else 0.082 * (1 - 0.01 * dmm) * p["rho"]) * MPA
    my = 0.3 * (num(conn, "fUK") / MPA) * dmm ** 2.6 / 1000
    fax = 0.2 * fh * d * t1 if "screw" in fastener else 0.0
    planes_count = int(conn.get("shearPlanes") or 0)
    steel = bool(conn.get("steelPlate"))
    if steel and planes_count >= 2:
        fvrk = johansen_steel_central(t1, d, fh, my, fax)
    elif steel:
        fvrk = johansen_steel(t1, num(conn, "steelPlateThicknessM"), d, fh, my, fax)
    elif planes_count >= 2:
        fvrk = johansen_double(t1, t2, d, fh, fh, my, fax)
    else:
        fvrk = johansen_single(t1, t2, d, fh, fh, my, fax)
    planes = float(max(planes_count, 1))
    rows = max(int(conn.get("rows") or 0), 1)
    n_total = int(conn.get("number") or 0)
    n = max(int(round(n_total / rows)), 1)
    spacing = num(conn, "spacingM")
    nef_row = 1.0 if n <= 1 else min(n, n ** 0.9 * (max(spacing, 1e-6) / (13 * max(d, 1e-6))) ** 0.25)
    nef = nef_row * rows
    fvrd = k_mod(conn.get("serviceClass"), rank) * nef * planes * fvrk / 1.3
    clause = "8.2.3" if steel else "8.2.2"
    dd = max(d, 1e-6)
    a1, a3, a4 = (5 * dd, 10 * dd, 5 * dd) if ("nail" in fastener or "screw" in fastener) else (5 * dd, 7 * dd, 3 * dd)
    return [
        {"id": f"en1995.{clause}.johansen.{cid}", "utilization": util(fed, fvrd)},
        {"id": f"en1995.8.spacing.a1.{cid}", "utilization": a1 / max(spacing, 1e-12)},
        {"id": f"en1995.8.spacing.a3t.{cid}", "utilization": a3 / max(num(conn, "endDistanceM"), 1e-12)},
        {"id": f"en1995.8.spacing.a4t.{cid}", "utilization": a4 / max(num(conn, "edgeDistanceM"), 1e-12)},
    ]


def evaluate(doc):
    """🏗️ Every check the snapshot earns, in member then connection order."""
    annex = annex_of(doc)
    members = doc.get("members") or []
    checks = [] if members else [{"id": "en1995.subject.empty", "utilization": None}]
    for member in members:
        checks.extend(member_checks(annex, member))
    for conn in doc.get("connections") or []:
        checks.extend(connection_checks(annex, conn))
    return {"checks": checks}


def main():
    """🚪️ `--json` reads a snapshot on stdin; without it, runs the k_cr DE/EN shear smoke asserts."""
    if "--json" in sys.argv:
        print(json.dumps(evaluate(json.load(sys.stdin))))
        return 0
    u_de = util(1.5 * 15000 / (k_cr("de", 4e6) * 0.2 * 0.3), 0.8 * 4e6 / 1.3)
    u_en = util(1.5 * 15000 / (k_cr("en", 4e6) * 0.2 * 0.3), 0.8 * 4e6 / 1.3)
    assert abs(u_de - 0.2438) < 0.01, u_de
    assert abs(u_en - 0.2274) < 0.01, u_en
    print(json.dumps({"ok": True, "shear_de": u_de, "shear_en": u_en}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
