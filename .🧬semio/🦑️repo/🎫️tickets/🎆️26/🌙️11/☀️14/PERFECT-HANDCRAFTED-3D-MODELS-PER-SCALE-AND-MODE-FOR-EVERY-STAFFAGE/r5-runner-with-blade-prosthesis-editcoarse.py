"""🧵 Round 5: the liner cuff of the 1:50 runner stands 2.5 mm proud of the thigh; the 1:100, 1:200 and 1:500 trunks take the V-shaped round-5 stations (waist 0.28 m, chest 0.33 m, shoulder line 0.376 m), the shorts top sits under the singlet hem and the shorts hems stand proud of the thighs."""
import re, sys
def edit(p, pairs):
    s = open(p).read()
    for a, b, n in pairs:
        if s.count(a) != n: sys.exit(f"{p}: {s.count(a)} x {a[:70]}")
        s = s.replace(a, b)
    open(p, "w").write(s)
edit("r5-runner-with-blade-prosthesis-body50.ts", [("[[0.335, 0.008, 0, 0.066, 0.0735], [0.37, 0.007, 0, 0.0615, 0.0675],", "[[0.34, 0.008, 0.002, 0.0675, 0.0745], [0.37, 0.007, 0.002, 0.064, 0.0695],", 1)])
c = "r5-runner-with-blade-prosthesis-coarse.ts"
s = open(c).read()
rows = {
 "body100": '[[-0.078, 0.01, 0, 0.17, 0.122, 2.3], [0.03, -0.002, 0, 0.146, 0.099, 2.3], [0.09, -0.005, 0, 0.14, 0.094, 2.3], [0.23, -0.0055, 0, 0.157, 0.1135, 2.3], [0.35, -0.002, 0, 0.171, 0.123, 2.3], [0.44, 0.002, 0, 0.188, 0.104, 2.2], [0.475, 0.0035, 0, 0.157, 0.083, 2.1], [0.512, 0.0055, 0, 0.076, 0.0635]]',
 "body200": '[[-0.078, 0.01, 0, 0.17, 0.122, 2.3], [0.09, -0.005, 0, 0.141, 0.095, 2.3], [0.3, -0.003, 0, 0.166, 0.12, 2.3], [0.44, 0.002, 0, 0.188, 0.104, 2.2], [0.512, 0.0055, 0, 0.08, 0.066, 2]]',
 "body500": '[[-0.078, 0.01, 0, 0.17, 0.122, 2.3], [0.09, -0.005, 0, 0.143, 0.097, 2.3], [0.33, -0.002, 0, 0.17, 0.122, 2.3], [0.44, 0.002, 0, 0.186, 0.104, 2.2], [0.5, 0.0055, 0, 0.1, 0.07, 2]]',
}
for fn, r in rows.items():
    i = s.index(f"function {fn}(")
    j = s.index('k.loft("singlet", "torso", M.singlet, T.singlet, ', i) + len('k.loft("singlet", "torso", M.singlet, T.singlet, ')
    e = s.index(', "singlet");', j)
    s = s[:j] + r + s[e:]
s = s.replace("[0.07, 0.008, 0, 0.158, 0.106, 2.3]], \"shorts\");", "[0.05, 0.01, 0, 0.152, 0.097, 2.3]], \"shorts\");").replace("[0.125, 0.0095, -0.001, 0.0835, 0.0945]], g), \"shorts\"", "[0.125, 0.0095, -0.001, 0.0855, 0.0965]], g), \"shorts\"")
s = s.replace("/** 🏃️ Bodies 1:100", "/** 🏃️ Bodies 1:100 (round 5: V-shaped trunk stations shared in proportion with 1:50)")
open(c, "w").write(s)
l = "r3-runner-lists.ts"
s = open(l).read()
a = '[[-0.078, 0.014, 0, 0.174, 0.127, 2.3], [0.09, -0.002, 0, 0.144, 0.101, 2.3], [0.3, -0.007, 0, 0.162, 0.116, 2.3], [0.42, 0.005, 0, 0.162, 0.094, 2.2], [0.47, 0.007, 0, 0.115, 0.076, 2]]'
if s.count(a) != 1: sys.exit("lists th-singlet")
s = s.replace(a, '[[-0.078, 0.01, 0, 0.17, 0.122, 2.3], [0.09, -0.005, 0, 0.143, 0.097, 2.3], [0.33, -0.002, 0, 0.17, 0.122, 2.3], [0.44, 0.002, 0, 0.186, 0.104, 2.2], [0.475, 0.004, 0, 0.142, 0.08, 2.1]]')
open(l, "w").write(s)
print("ok")
