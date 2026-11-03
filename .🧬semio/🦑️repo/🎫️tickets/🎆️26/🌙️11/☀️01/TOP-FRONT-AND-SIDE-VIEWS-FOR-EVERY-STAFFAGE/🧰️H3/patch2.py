s=open("human.py",encoding="utf8").read()
s=s.replace('''tip_y = R.t(ft)[1]
                poly(o, f"thigh-{sd}"''','''tip_y = R.t(ft)[1]
                flat = abs(ank_y - tip_y) < 0.03
                if flat: tip_y = ank_y - P.get("drop", 0.09)
                poly(o, f"thigh-{sd}"''')
s=s.replace('''1.1, h.S(sc, "shoe", sd), follow=True)
                if low and t >= 2:
                    ell(o, f"shoe-toe-{sd}", tb, lat, R.o(tb)[1] + 0.02,''','''1.1, h.S(sc, "shoe", sd), follow=not flat)
                tb0 = tip_y if flat else R.o(tb)[1]
                if low and t >= 2 and "toe" not in skip:
                    ell(o, f"shoe-toe-{sd}", tb, lat, tb0 + 0.02,''')
s=s.replace('''if low and t >= 3:
                    line(o, f"shoe-sole-{sd}", tb, [(lat - fw * 0.96, R.o(tb)[1] + 0.004), (lat + fw * 0.96, R.o(tb)[1] + 0.004)], 1.15, 0.9)''','''if low and t >= 3:
                    line(o, f"shoe-sole-{sd}", tb if "toe" not in skip else ft, [(lat - fw * 0.96, tb0 + 0.004), (lat + fw * 0.96, tb0 + 0.004)], 1.15, 0.9)''')
s=s.replace('''if low: cap(o, f"shoe-heel-{sd}", ft, fx0 - 0.025, lat, fx1, lat, fw * 0.85, fw, 1.2, h.S(sc, "shoe", sd), follow=True)
                if low: cap(''','''nt = "toe" in skip
                if low: cap(o, f"shoe-heel-{sd}", ft, fx0 - 0.025, lat, fx1 - (fw * 0.8 - 0.012 if nt else 0), lat, fw * 0.85, fw * (0.8 if nt else 1), 1.2, h.S(sc, "shoe", sd), follow=True)
                if low and not nt: cap(''')
open("human.py","w",encoding="utf8").write(s)
s=open("humanlib.py",encoding="utf8").read()
s=s.replace('''                    if "regions" in sh: d["regions"] = sh["regions"]''','''                    if "regions" in sh: d["regions"] = sh["regions"]
                    if "material" in sh: d["material"] = sh["material"]''')
open("humanlib.py","w",encoding="utf8").write(s)
print("ok")
