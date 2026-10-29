import sys; sys.path.insert(0, sys.argv[1]); import kin
a = kin.load(sys.argv[2], "domestic-cat")
f = ["scapula-fn","humerus-fn","forearm-fn","metacarpus-fn","paw-fn"]
for fx in (0.27,0.28,0.29):
  for gy in (-0.225,-0.2):
      o, e = a.solve_limb({}, f, "contact-fn", (fx, gy), {f[0]:("off",0), f[1]:("solve",), f[2]:("solve",), f[3]:("abs",-85), f[4]:("abs",-12)})
      print(fx,gy,{k: round(v,1) for k,v in o.items()}, round(e,4))
