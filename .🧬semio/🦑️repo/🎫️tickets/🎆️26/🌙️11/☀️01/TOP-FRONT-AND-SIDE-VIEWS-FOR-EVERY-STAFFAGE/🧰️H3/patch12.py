s=open("human.py",encoding="utf8").read()
s=s.replace('max(P["a1"], FIST[t][0]), max(P["a1"], FIST[t][0]), 4.65,','max(P["a1"], min(FIST[t][0], P["a0"])), max(P["a1"], min(FIST[t][0], P["a0"])), 4.65,')
s=s.replace('max(P["hr"] * 1.02, FIST[t][1]), max(P["hr"], FIST[t][1]),','max(P["hr"] * 1.02, min(FIST[t][1], P["a0"])), max(P["hr"], min(FIST[t][1], P["a0"])),')
s=s.replace('fl = .08 if sc == "500" else .03','fl = .095 if sc == "500" else .034')
# shoes shorter at 1:500
s=s.replace('if low: cap(o, f"shoe-heel-{sd}", ft, fx0 - 0.025, lat, fx1 - (fw * 0.8 - 0.012 if nt else 0), lat,','if low: cap(o, f"shoe-heel-{sd}", ft, fx0 - 0.025, lat, fx1 - (fw * 0.8 - 0.012 if nt else (fw - 0.02 if sc == "500" else 0)), lat,')
open("human.py","w",encoding="utf8").write(s)
