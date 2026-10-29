import sys
exec(open(sys.argv[1]+"/pigeon-emit.py").read().split("P = []; CL = []")[0])
def tip(trunk, n, hd, H):
    base={"trunk":trunk}; base.update(absang(base,NECK,n+(hd,)))
    w=a.fk(base); t=a.anchor(w,"beak-tip"); return (round(t[0],3), round(t[1]+H,3))
for trunk in (-15,-25,-35):
  for n in ((20,0,-20),(15,-10,-35),(10,-20,-40),(30,0,-30)):
    for hd in (-60,-75):
      for H in (0.08,):
        print(trunk,n,hd,tip(trunk,n,hd,H))
