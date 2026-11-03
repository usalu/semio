import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import re, sys
from lib import *
A = T
files = [A.snap + "/🔣️.json", A.snap + "/🟦️.ts", A.diff + "/🔣️.json", A.diff + "/🟦️.ts", A.any + "/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts", A.mut + "/" + LEAF_DIRS["insert-case"] + "/🧬️schema/🔣️.json", A.mut + "/" + LEAF_DIRS["insert-case"] + "/🧬️schema/🟦️.ts"]
for p in files:
    s = rd(p)
    s = s.replace("En1995CaseNumber", "CaseNumber").replace("En1995CaseText", "CaseText")
    s = re.sub(r"En1995Case(Patch|Removal|Insertion|Relocation|Modification|Delta)", r"CaseRow\1", s)
    s = s.replace("CatalogueCase", "CaseRow")
    wr(p, s)
print("\n".join(LOG))
