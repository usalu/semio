import re
s=open("human.py",encoding="utf8").read()
# front
s=s.replace('''                poly(o, f"shank-{sd}", sk, limb(lat, knee_y, ank_y, P["s0"], P["s1"], 0.004), 1.02, h.S(sc, "shank", sd), follow=True)
                fw = P["fw"]
                poly(o, f"shoe-{sd}", ft,''','''                low = f"lower-{sd}" not in skip
                fw = P["fw"]
                if low: poly(o, f"shank-{sd}", sk, limb(lat, knee_y, ank_y, P["s0"], P["s1"], 0.004), 1.02, h.S(sc, "shank", sd), follow=True)
                if low: poly(o, f"shoe-{sd}", ft,''')
s=s.replace('''                if t >= 2:
                    ell(o, f"shoe-toe-{sd}"''','''                if low and t >= 2:
                    ell(o, f"shoe-toe-{sd}"''')
s=s.replace('''                if t >= 3:
                    line(o, f"shoe-sole-{sd}"''','''                if low and t >= 3:
                    line(o, f"shoe-sole-{sd}"''')
s=s.replace('''                    if "cuff" in h.KEY: line(o, f"trouser-hem''','''                    if "hem-leg" in h.KEY: line(o, f"trouser-hem''')
# plan
s=s.replace('''                fx0 = R.o(ft)[0]; fx1 = R.t(ft)[0]; tx1 = wbox''','''                low = f"lower-{sd}" not in skip
                fx0 = R.o(ft)[0]; fx1 = R.t(ft)[0]; tx1 = wbox''') 
open("human.py","w",encoding="utf8").write(s)
print(s.count("low"))
