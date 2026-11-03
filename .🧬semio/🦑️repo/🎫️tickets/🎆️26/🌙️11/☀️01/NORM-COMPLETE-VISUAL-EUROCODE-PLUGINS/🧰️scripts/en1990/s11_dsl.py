from spec import *
import glob, re
n = 0
for p in glob.glob(SUB + "/🖼️assets/*/*/🗣️.dsl.semio"):
    t = open(p, encoding="utf8").read()
    if "fatigue-spectra" in t: continue
    lines = t.split("\n"); out = []; mode = None
    for ln in lines:
        if ln.startswith("variables [id:TEXT category:TEXT qk:NUM]"):
            ln = ln.replace("qk:NUM]", "qk:NUM group:TEXT]"); mode = "var"
        elif ln.startswith("}"): mode = None
        elif mode == "var" and ln.strip(): ln = ln + ' ""'
        out.append(ln)
    t = "\n".join(out)
    assert 'group:TEXT' in t
    if not t.endswith("\n"): t += "\n"
    t += "fatigue-spectra [id:TEXT member-id:TEXT detail-category:NUM stress-range:NUM cycles:NUM] {\n}\nreliability-analyses [id:TEXT resistance-distribution:TEXT resistance-mean:NUM resistance-sd:NUM action-distribution:TEXT action-mean:NUM action-sd:NUM] {\n}\ntest-series [id:TEXT quantity:TEXT distribution:TEXT sample-size:NUM mean:NUM std-dev:NUM variation-known:NUM conversion-factor:NUM required:NUM] {\n}\n"
    open(p, "w", encoding="utf8").write(t); n += 1
print(n)
