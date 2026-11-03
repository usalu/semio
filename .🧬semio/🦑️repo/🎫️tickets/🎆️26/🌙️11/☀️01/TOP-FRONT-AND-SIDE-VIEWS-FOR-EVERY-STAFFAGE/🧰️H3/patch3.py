s=open("human.py",encoding="utf8").read()
s=s.replace('''                poly(o, f"thigh-{sd}", th, limb(lat, hip_y, knee_y, P["t0"], P["t1"], 0.004), 1.0, h.S(sc, "thigh", sd), follow=True)
''','''                if P.get("solid_thigh"):
                    L_ = R.b[th]["lengthM"]
                    poly(o, f"thigh-{sd}", th, [(0, -P["t0"]), (L_, -P["t1"]), (L_, P["t1"]), (0, P["t0"])], 1.0, h.S(sc, "thigh", sd)); sol(o, lat - P["t0"], lat + P["t0"])
                else:
                    poly(o, f"thigh-{sd}", th, limb(lat, hip_y, knee_y, P["t0"], P["t1"], 0.004), 1.0, h.S(sc, "thigh", sd), follow=True)
''')
s=s.replace('''                if low: poly(o, f"shoe-{sd}", ft, [(lat - P["s1"] * 1.08''','''                if low and flat:
                    fl_ = R.b[ft]["lengthM"]; dr_ = P.get("drop", 0.09)
                    poly(o, f"shoe-{sd}", ft, [(-0.05, -dr_), (-0.05, 0.03), (fl_ * 0.45, 0.03), (fl_, -dr_ * 0.5), (fl_, -dr_)], 1.1, h.S(sc, "shoe", sd)); sol(o, lat - fw, lat + fw)
                elif low: poly(o, f"shoe-{sd}", ft, [(lat - P["s1"] * 1.08''')
open("human.py","w",encoding="utf8").write(s)
l=open("humanlib.py",encoding="utf8").read()
if "def sol(" not in l:
    l+='''

def sol(o, z0, z1):
    """Marks the last shape as an extruded solid: its bone-local profile is placed with the full bone transform."""
    if o.items and o.items[-1]["id"] in o.ids:
        o.items[-1]["solid"] = {"kind": "extruded", "z0": r4(z0), "z1": r4(z1)}
'''
    open("humanlib.py","w",encoding="utf8").write(l)
print("ok")
