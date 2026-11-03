import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
B = "Open question, standard text not at hand: "
put(d, "1991", "1-3.drift-mu-w-reading", "5.3.6", "mu_w limit reading", 0.0, 0.0, "", B + "0 floors mu_w at 0.8 even when gamma h/s_k is smaller (conservative, larger mu_w); 1 lets gamma h/s_k cap mu_w when it is below 0.8", part="1-3")
save(d); print(len(d))
