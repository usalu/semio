from __future__ import annotations

import math
import re
from pathlib import Path


ROOT = Path(r"E:\semio\mit-bestand\bericht\zwischenbericht\temp")
FILES = {
    "A": (ROOT / "ableitung-profilwerte.md", 20, 11, 9, 3),
    "B": (ROOT / "ableitung-guete-baualter.md", 24, 19, 5, 0),
    "C": (ROOT / "ableitung-abtrag-zustand.md", 10, 5, 5, 0),
    "D": (ROOT / "ableitung-reuse-lca.md", 15, 14, 1, 0),
}
ALLOWED = {"vorhanden", "abgeleitet", "angenommen", "offen"}
REQUIRED = (
    "Erhebungsstand",
    "Abdeckung",
    "Status-Legende",
    "Verbindliche Kodierregeln",
    "Lueckenliste",
    "Belegte Rohnotizen",
    "such_begriffe",
    "geprueft_urls",
)


def check_file(track: str, path: Path, assigned: int, evaluated: int, gaps: int, errors: int) -> None:
    text = path.read_text(encoding="utf-8")
    for marker in REQUIRED:
        assert marker in text, f"{path.name}: missing {marker}"
    assert "| - |" not in text, f"{path.name}: ASCII dash placeholder"
    assert "`—`" in text and "`n. p.`" in text, f"{path.name}: missing null distinction"

    if track == "D":
        ids = re.findall(r"^\| D-[PTOA]\d{2} \|", text, re.MULTILINE)
    else:
        ids = re.findall(rf"^\| {track}-\d{{2}} \|", text, re.MULTILINE)
    assert len(ids) == assigned, f"{path.name}: {len(ids)} != {assigned} assigned"

    status_hits = re.findall(
        r"\|\s*(vorhanden|abgeleitet|angenommen|offen)\s*\|\s*\[",
        text,
    )
    assert len(status_hits) == assigned, f"{path.name}: {len(status_hits)} status rows"
    assert set(status_hits) <= ALLOWED
    actual_gaps = status_hits.count("offen")
    actual_evaluated = assigned - actual_gaps
    assert (actual_evaluated, actual_gaps) == (evaluated, gaps), (
        f"{path.name}: evaluated/gaps {(actual_evaluated, actual_gaps)}"
    )
    assert f"`{assigned} = {evaluated} + {gaps}`" in text
    assert re.search(rf"Fehlerquote[^\n]*{errors}/{evaluated} =", text)
    urls = re.findall(r"https?://[^)\s]+", text)
    assert len(urls) >= assigned, f"{path.name}: insufficient claim URLs"
    print(
        f"PASS {track}: assigned={assigned}, evaluated={evaluated}, "
        f"gaps={gaps}, status_rows={len(status_hits)}, urls={len(urls)}"
    )


for item in FILES.items():
    check_file(item[0], *item[1])

# Independent arithmetic reproduction for A-01/B-06, matching the formulas
# implemented in norm/en/1993.
A = 2850.0
WPL = 220_600.0
AV = 1400.0
IZ = 22.0
FY = 235.0
E = 210_000.0
L = 6.0
Q = 10.0
LCR = 3000.0
NED = 300.0
ALPHA = 0.34
GM0 = 1.0
GM1 = 1.1

med = Q * L**2 / 8
ved = Q * L / 2
mrd = WPL * FY / GM0 / 1e6
vrd = AV * FY / (math.sqrt(3) * GM0) / 1000
ncr = math.pi**2 * E * A * IZ**2 / LCR**2 / 1000
lbar = math.sqrt(A * FY / (ncr * 1000))
phi = 0.5 * (1 + ALPHA * (lbar - 0.2) + lbar**2)
chi = 1 / (phi + math.sqrt(phi**2 - lbar**2))
nbrd = chi * A * FY / GM1 / 1000

expected = {
    "MEd": (med, 45.0, 0.01),
    "VEd": (ved, 30.0, 0.01),
    "MRd": (mrd, 51.84, 0.01),
    "VRd": (vrd, 189.95, 0.02),
    "Ncr": (ncr, 317.7, 0.1),
    "lambda": (lbar, 1.452, 0.002),
    "phi": (phi, 1.767, 0.002),
    "chi": (chi, 0.360, 0.002),
    "NbRd": (nbrd, 219.5, 0.2),
    "NbUtil": (NED / nbrd, 1.367, 0.002),
}
for name, (actual, target, tolerance) in expected.items():
    assert abs(actual - target) <= tolerance, (name, actual, target)
print("PASS EN1993 arithmetic:", ", ".join(f"{k}={v[0]:.3f}" for k, v in expected.items()))
