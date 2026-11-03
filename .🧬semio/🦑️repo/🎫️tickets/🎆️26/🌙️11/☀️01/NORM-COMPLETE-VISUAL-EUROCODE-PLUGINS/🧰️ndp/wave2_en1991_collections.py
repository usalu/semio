import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
put(d, "1991", "e6-01.1", "Table 3.1", "T", None, None, "", "Return periods of the climatic actions for the duration classes of a construction stage (<= 3 days, <= 3 months, <= 1 year, > 1 year) are the recommended 2, 2, 5 and 50 years as recalled from EN 1991-1-6 Table 3.1; the table text and the national values are not checked against the print, and the wind pressure ratio c_prob(1/T)^2 follows EN 1991-1-4 Eq. 4.2 with K = 0.2, n = 0.5.", part="1-6")
save(d)
