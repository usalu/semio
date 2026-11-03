import re
for f,old,new in [("p_office.py","P = dict(hl=.09,","P = dict(solid_thigh=True, hl=.09,"),("p_reader.py","P = dict(hl=.085,","P = dict(solid_thigh=True, hl=.085,"),("p_guitarist.py","P = dict(hl=.105,","P = dict(solid_thigh=False, hl=.105,")]:
    s=open(f,encoding="utf8").read(); s=s.replace(old,new); open(f,"w",encoding="utf8").write(s)
# guitar
s=open("p_guitarist.py",encoding="utf8").read()
i=s.index("    poly(of, \"guitar-body\""); j=s.index("    if t >= 2:\n        ell(of, \"soundhole\"")
new='''    for view, out in (("f", of), ("p", op)):
        poly(out, "guitar-body", g, [(-.217, -.193), (.0, -.2), (.335, -.18), (.335, .18), (.0, .2), (-.217, .193)], 3.9, gb); sol(out, -.055, .055)
        poly(out, "guitar-neck", g, [(.31, -.024), (.64, -.024), (.64, .024), (.31, .024)], 3.85, gn); sol(out, -.024, .024)
        poly(out, "guitar-head", g, [(.64, -.044), (.78, -.044), (.78, .044), (.64, .044)], 3.86, gh); sol(out, -.04, .04)
'''
s=s[:i]+new+s[j:]
open("p_guitarist.py","w",encoding="utf8").write(s)
# skate deck
s=open("p_skate.py",encoding="utf8").read()
i=s.index("    poly(of, \"deck\""); j=s.index("    poly(of, \"truck-front\"")
new='''    prof = [(-.405, .103), (-.37, .03), (-.34, .018), (.34, .018), (.37, .03), (.405, .103), (.405, .085), (.35, -.008), (-.35, -.008), (-.405, .085)]
    for out in (of, op):
        poly(out, "deck", "board", [(x, y - .085 + .085) for x, y in prof], .5, deck); sol(out, -HALF, HALF)
'''
s=s[:i]+new+s[j:]
s=s.replace('    rect(op, "deck", "board", -.405, -HALF, .81, 2 * HALF, .5, deck, r=.09)\n','')
open("p_skate.py","w",encoding="utf8").write(s)
print("ok")
