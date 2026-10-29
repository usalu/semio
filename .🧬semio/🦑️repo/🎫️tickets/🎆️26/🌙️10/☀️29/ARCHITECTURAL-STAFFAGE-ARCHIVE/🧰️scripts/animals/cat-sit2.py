import sys, importlib; sys.path.insert(0, sys.argv[1]); kin = importlib.import_module('cat-kin')
a = kin.load(sys.argv[2], "domestic-cat")
f = ["scapula-fn","humerus-fn","forearm-fn","metacarpus-fn","paw-fn"]
for ra in (46,50,54):
  for tb in (-6,-10,-14):
   for sc in (0,-20):
    rootS=(0,0,ra); bs={"lumbar":4,"thorax-a":tb,"thorax-b":tb}
    w=a.fk(bs,rootS); rx,ry=a.anchor(w,"contact-rump"); sh=w["humerus-fn"]
    for dx in (0.0,0.03):
        try:
            o,e=a.solve_limb(bs,f,"contact-fn",(sh[4]+dx,ry),{f[0]:("off",sc),f[1]:("solve",-10,60),f[2]:("solve",),f[3]:("abs",-80),f[4]:("abs",-15)},root=rootS)
            print(ra,tb,sc,dx,round(e,4),{k:round(v) for k,v in o.items()})
        except Exception as ex: print(ra,tb,sc,dx,"none")
