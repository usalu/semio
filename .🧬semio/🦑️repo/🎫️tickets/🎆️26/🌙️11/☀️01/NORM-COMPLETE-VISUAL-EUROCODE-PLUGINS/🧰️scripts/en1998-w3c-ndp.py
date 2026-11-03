"""EN 1998 wave 3c: evidence notes naming the exact EN 1998-1 clauses of the unverified RC detailing rows (column width 5.4.1.2.1 / 5.5.1.2.1, omega_wd 5.4.3.2.2 / 5.5.3.2.2, rho limits 5.4.3.1.2 / 5.4.3.2.1 and 5.5.3.1.2 / 5.5.3.2.1) and registers the beam minimum width that was a literal in the check.
Usage: /home/deck/semio/.venv/bin/python -I en1998-w3c-ndp.py"""
import pathlib, subprocess, sys

NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

TAIL = "; UNVERIFIED: the number is not read from the printed standard, check it against the clause before release"
d = load()
put(d, "1998", "en1.rc.beam-min-b.dcm", "5.4.1.2.1", "Minimum width of primary seismic beams DCM", 0.20, 0.20, "m", "Simplified minimum beam width 0.20 m as used by the artifact", part="1", keep=True)
put(d, "1998", "en1.rc.beam-min-b.dch", "5.5.1.2.1", "Minimum width of primary seismic beams DCH", 0.20, 0.20, "m", "Simplified minimum beam width 0.20 m as used by the artifact", part="1", keep=True)
put(d, "1998", "en1.rc.beam-min-b.low", "5.4.1.2.1", "Minimum width of beams DCL or unspecified", 0.20, 0.20, "m", "Simplified minimum beam width 0.20 m as used by the artifact", part="1", keep=True)
notes = {
    "column-min-b.dcm": "EN 1998-1 5.4.1.2.1 (cross-section dimensions of primary seismic columns, DCM): check the minimum width against the printed clause" + TAIL,
    "column-min-b.dch": "EN 1998-1 5.5.1.2.1 (cross-section dimensions of primary seismic columns, DCH): check the minimum width against the printed clause" + TAIL,
    "column-min-b.low": "EN 1998-1 5.4.1.2.1 (DCL falls back to the DCM clause): check against the printed clause" + TAIL,
    "beam-min-b.dcm": "EN 1998-1 5.4.1.2.1 (primary seismic beams, DCM): the check compares the web width b_w with this minimum; check the number against the printed clause" + TAIL,
    "beam-min-b.dch": "EN 1998-1 5.5.1.2.1 (primary seismic beams, DCH): check the number against the printed clause" + TAIL,
    "beam-min-b.low": "EN 1998-1 5.4.1.2.1: check the number against the printed clause" + TAIL,
    "omega-wd-min.dcm": "EN 1998-1 5.4.3.2.2 (confinement of critical regions of columns, DCM; Eq. 5.15 gives the required alpha*omega_wd, this row is the simplified floor): check against the printed clause" + TAIL,
    "omega-wd-min.dch": "EN 1998-1 5.5.3.2.2 (confinement of critical regions of columns, DCH; 5.5.3.2.2 uses Eq. 5.15 with the DCH minimum): check against the printed clause" + TAIL,
    "omega-wd-min.low": "EN 1998-1 5.4.3.2.2 / 5.5.3.2.2: DCL has no confinement requirement in the artifact" + TAIL,
}
for key, note in notes.items():
    d[f"ndp.en1998.en1.rc.{key}"]["evidence"] = note
for key in [k for k in d if k.startswith("ndp.en1998.en1.rc.rho-")]:
    role = key.split(".")[-2]
    clause = "5.4.3.1.2 / 5.5.3.1.2 (beams)" if role == "beam" else "5.4.3.2.1 / 5.5.3.2.1 (columns)" if role == "column" else "5.4.3 / 5.5.3"
    d[key]["evidence"] = f"EN 1998-1 {clause}: longitudinal reinforcement ratio limits; check against the printed clause" + TAIL
save(d)
subprocess.run([sys.executable, "-I", str(NDP_TOOLS / "codegen.py"), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
