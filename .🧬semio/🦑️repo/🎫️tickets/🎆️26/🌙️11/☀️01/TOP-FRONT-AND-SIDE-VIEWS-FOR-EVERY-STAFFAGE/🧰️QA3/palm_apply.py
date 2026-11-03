from abd_exact import run
AZ = {"l1": 100, "l2": 188, "l3": 148, "l4": 125, "r1": -80, "r2": 8, "r3": -38, "r4": 55}
chains = {n: (f"fr-{n}a", f"fr-{n}b", az) for n, az in AZ.items()}
def edit(a, bones):
    a["dimensions"]["widthM"] = 6.3


run("date-palm", "palm.before.json", chains, edit=edit)
