import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
NOTE = "EN 1991-1-4 Table 7.5 (hipped roofs, zones F to N by pitch angle and wind direction) is not available: the cell is UNVERIFIED and left unset so no value is invented; the check reports 'awaiting values' until it is keyed. The zone geometry follows Figure 7.9 (also unverified)."
for zone in "FGHIJKLMN":
    put(d, "1991", f"1-4.hipped-cpe10.{zone.lower()}", "Table 7.5", f"c_pe,10 zone {zone} hipped roof", None, None, "", f"{NOTE} Cell: c_pe,10, zone {zone}.", part="1-4", locator="Table 7.5")
    put(d, "1991", f"1-4.hipped-cpe1.{zone.lower()}", "Table 7.5", f"c_pe,1 zone {zone} hipped roof", None, None, "", f"{NOTE} Cell: c_pe,1, zone {zone}.", part="1-4", locator="Table 7.5")
save(d); print(len(d))
