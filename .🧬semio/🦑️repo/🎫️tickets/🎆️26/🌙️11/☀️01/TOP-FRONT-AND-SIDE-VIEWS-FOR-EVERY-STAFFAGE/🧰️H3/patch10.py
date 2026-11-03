s=open("human.py",encoding="utf8").read()
s=s.replace('fl = .07 if sc == "500" else .028','fl = .08 if sc == "500" else .03')
s=s.replace('ellsol(o, "head", hb, sk_[0], max(sk_[2], hr_[2]), sk_[1], sk_[3], hx, 5.0, h.S(sc, "skin"))','ellsol(o, "head", hb, sk_[0], max(sk_[2], hr_[2]), sk_[1], sk_[3], hx, 5.0, {"tone": .4, "regions": [h.S(sc, "skin")["region"], h.S(sc, "hair")["region"]]})')
s=s.replace('ell(o, f"elbow-{sd}", fa, lat, el_y, P["a1"], P["a1"], 4.65,','ell(o, f"elbow-{sd}", fa, lat, el_y, max(P["a1"], FIST[t][0]), max(P["a1"], FIST[t][0]), 4.65,')
s=s.replace('ell(o, f"fist-{sd}", hd, lat, wr_y, P["hr"] * 1.02, P["hr"] * 1.0,','ell(o, f"fist-{sd}", hd, lat, wr_y, max(P["hr"] * 1.02, FIST[t][1]), max(P["hr"], FIST[t][1]),')
s=s.replace("SIDES = (","FIST = {0: (.09, .09), 1: (.055, .05), 2: (0, 0), 3: (0, 0)}\nSIDES = (",1)
open("human.py","w",encoding="utf8").write(s)
t=open("p_reader.py",encoding="utf8").read()
i=t.index('    poly(of, "pad-cover"'); j=t.index('    cap(op, "pad-cover"')
t=t[:i]+'    th = .05 if t == 0 else .012\n    limbsol(h, of, "pad-cover", "pad", th, th, lat, 6.4, s("pad-cover"), zr=pwid)\n'+t[j:]
i=t.index('    poly(of, "page-sheet"'); j=t.index('    cap(op, "page-sheet"')
t=t[:i]+'    limbsol(h, of, "page-sheet", "page", th, th, lat, 6.5, s("page-sheet"), zr=gwid)\n'+t[j:]
open("p_reader.py","w",encoding="utf8").write(t)
