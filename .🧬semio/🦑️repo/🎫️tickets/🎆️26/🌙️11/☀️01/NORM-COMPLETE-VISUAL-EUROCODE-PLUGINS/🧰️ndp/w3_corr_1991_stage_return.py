import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
NOTE = "EN 1991-1-6 Table 3.1 (spec E16.01 [N]): return period of the climatic actions of a transient execution stage by its duration, recommended value {}. The DE NA replaces the table with modified return periods ([V] in the spec); no DE value is available in the repo text, so the DE cell stays unset until the NA print is keyed and the check reports 'awaiting values' for annex DE."
for key, label, value in (("days3", "duration up to 3 days", 2.0), ("months3", "duration over 3 days up to 3 months", 5.0), ("year1", "duration over 3 months up to 1 year", 10.0), ("longer", "duration over 1 year", 50.0)):
    put(d, "1991", f"1-6.stage-return.{key}", "Table 3.1", f"T return period, {label}", value, None, "a", NOTE.format(f"{value:g} years for the {label}"), part="1-6", locator="Table 3.1")
    d[f"ndp.en1991.1-6.stage-return.{key}"]["source"]["document"] = "DIN EN 1991-1-6/NA"
e = d["ndp.en1991.e6-01.1"]
e.update(symbol="K, n", clause="Eq. 4.2", asWritten="Basis of the reduced wind pressure of a construction stage: the ratio c_prob(1/T)^2 follows EN 1991-1-4 Eq. 4.2 with K = 0.2 and n = 0.5 (constants recalled, not checked against the print). The return periods of the duration classes are the rows ndp.en1991.1-6.stage-return.*.")
e["source"]["locator"] = "Eq. 4.2"
save(d); print(len(d))
