import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
S = pathlib.Path(sys.argv[1]).read_text()
d = load()
PROPS = ["fmk", "ft0k", "ft90k", "fc0k", "fc90k", "fvk", "e0mean", "e005", "e90mean", "gmean", "rhok", "rhomean"]
SYM = ["f_m,k", "f_t,0,k", "f_t,90,k", "f_c,0,k", "f_c,90,k", "f_v,k", "E_0,mean", "E_0,05", "E_90,mean", "G_mean", "ρ_k", "ρ_mean"]
UNIT = ["N/mm2"] * 6 + ["N/mm2"] * 4 + ["kg/m3"] * 2
SRC = {"EN338": "EN 338:2016 Table 1", "EN14080": "EN 14080:2013 Table 5"}
old = {k: v for k, v in d.items() if k.startswith("ndp.en1995.t-01.")}
rows = re.findall(r'row\("(\w+)", (\w+(?:::\w+)?), (true|false), \[([^\]]+)\], ([^,]+), (true|false)\)', S)
assert len(rows) == 36, len(rows)
for name, prod, hw, vals, src, ver in rows:
    vals = [float(x) for x in vals.split(",")][:12]
    key = name.lower()
    docs = {"EN338": "EN 338:2016 Table 1", "EN338_UNCONFIRMED": "EN 338:2016 Table 1 (unconfirmed transcription)", "EN14080": "EN 14080:2013 Table 5"}.get(src, src.strip('"'))
    for p, s, u, v in zip(PROPS, SYM, UNIT, vals):
        i = put(d, "1995", f"strength.{key}.{p}", "Table 1" if "338" in docs else "Table 5" if "14080" in docs else "material", f"{s} {name}", v, v, u, f"{s} of {name} as used by the artifact: {v:g}")
        e = d[i]
        e["source"]["document"] = docs
        o = old.get(f"ndp.en1995.t-01.{key}.{p}")
        if o:
            e["specRef"], e["asWritten"], e["evidence"] = o["specRef"], o["asWritten"], o["evidence"]
        if ver == "true":
            e["status"] = "verified"
            e["evidence"] = (e["evidence"] + " " if e["evidence"] else "") + "row flagged verified in the code before wave 2; review E95-xx doubts the numbers of C16/C30, re-check against the print"
for k in old:
    del d[k]
DUR = ["permanent", "long", "medium", "short", "instant"]
km = re.search(r"KMOD_TABLE.*?= &\[(.*?)\n\];", S, re.S).group(1)
sc2 = dict(re.findall(r'\("([\w-]+)", \[([^\]]+)\]\),', re.search(r"KMOD_SC2_PANELS.*?= &\[(.*?)\n\];", S, re.S).group(1)))
for m in re.finditer(r'\("([\w-]+)", \[([^\]]+)\], (\[[^\]]+\]), (true|false)\)', km):
    mat, a, b, ver = m.groups()
    sc12 = [float(x) for x in a.split(",")]
    sc3 = [0.0] * 5 if b.startswith("[0.0;") else [float(x) for x in b.strip("[]").split(",")]
    s2 = [float(x) for x in sc2[mat].replace("0.0; 5", "0,0,0,0,0").split(",")] if mat in sc2 else sc12
    if mat in sc2 and len(s2) != 5: s2 = [0.0] * 5
    for sc, vals in (("sc1", sc12), ("sc2", s2), ("sc3", sc3)):
        for dn, v in zip(DUR, vals):
            i = put(d, "1995", f"kmod.{mat}.{sc}.{dn}", "Table 3.1", f"k_mod {mat} {sc.upper()} {dn}", v, v, "", f"k_mod of {mat}, service class {sc[2]}, {dn} load; 0 = not allowed")
            if ver == "true": d[i]["status"] = "verified"
for m in re.finditer(r'\("([\w-]+)", ([\d.]+), ([\d.]+), ([\d.]+), (true|false)\)', re.search(r"KDEF_TABLE.*?= &\[(.*?)\n\];", S, re.S).group(1)):
    mat, a, b, c, ver = m.groups()
    for sc, v in (("sc1", a), ("sc2", b), ("sc3", c)):
        i = put(d, "1995", f"kdef.{mat}.{sc}", "Table 3.2", f"k_def {mat} {sc.upper()}", float(v), float(v), "", f"k_def of {mat}, service class {sc[2]}; 0 = not allowed")
        if ver == "true": d[i]["status"] = "verified"
save(d)
print(len([k for k in d if k.startswith("ndp.en1995.")]))
