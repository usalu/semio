import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SPEC = "📐️specs/en1990-en1991-target.md"
BASE = f"POLICY PLACEHOLDER, NOT A NORMATIVE VALUE. The spec only requires that an annex value edit is an explicit, user-visible, justified event ({SPEC} l.16 NdpOverridden, l.118 PsiOverridden with mandatory justification, l.449 and l.465 override toggle, l.713 three-state badge) and says nothing about which factors a National Annex lets a project replace. The allowance below was chosen by the artifact owner and must be replaced by the printed EN 1990 / DIN EN 1990/NA wording"
rows = {
    "a1.override.gamma.allowed": ("A1.3", "EN 1990 Annex A1/A2 sets gamma_G, gamma_Q as NDPs of the National Annex; EN: allowed (placeholder), DE: not allowed (placeholder)", f"{BASE}; clause reference A1.3 (partial factors of Table A1.2) is taken from the artifact's own use, the spec has no line for gamma."),
    "a1.override.psi.allowed": ("A1.1", "EN 1990 Table A1.1 psi values are NDPs; project override with justification: allowed in EN and DE (placeholder)", f"{BASE}; spec anchor l.118 (psi override with mandatory justification, validated psi0 >= psi1 >= psi2)."),
    "a1.override.xi.allowed": ("A1.3", "EN 1990 Table A1.2(B) xi is an NDP; EN: allowed (placeholder), DE: not allowed (placeholder)", f"{BASE}; clause reference A1.3 is taken from the artifact's own use, the spec has no line for xi."),
    "a1.override.k_fi.allowed": ("A1.3", "EN 1990 Table B3 K_FI follows the reliability class (spec l.48: K_FI(RC1..3) = 0.9/1.0/1.1); override not allowed in EN and DE (placeholder)", f"{BASE}; spec anchor l.48 (K_FI fixed per reliability class, 'edge: not applied to favourable actions or gamma_M')."),
}
for key, (clause, spec, ev) in rows.items():
    e = d[f"ndp.en1990.{key}"]
    e["specRef"] = f"POLICY PLACEHOLDER (unverified): {spec}"
    e["evidence"] = ev
    e["status"] = "unverified"
save(d)
print(len(rows))
