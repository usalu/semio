import sys, json, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import din18599
v = din18599.vectors()
print([round(r["eta"],3) for r in v["monthsRustModel"]]);print([round(r["eta"],3) for r in v["monthsRustModelWithStandardEta"]])
print(v["coefficients"], v["standardMeta"]); print(v["annual"]); print(v["utilisationLimits"])
for r in v["utilisationFactor"][:3]: print(r)
