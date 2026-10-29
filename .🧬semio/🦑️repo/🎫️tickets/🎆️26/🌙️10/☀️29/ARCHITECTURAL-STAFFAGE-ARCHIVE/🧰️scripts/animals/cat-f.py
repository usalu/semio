import sys, importlib; sys.path.insert(0, sys.argv[1]); kin = importlib.import_module('cat-kin')
a = kin.load(sys.argv[2], "domestic-cat")
f = ["scapula-fn","humerus-fn","forearm-fn","metacarpus-fn","paw-fn"]
for x in (0.2,0.25,0.278,0.3,0.34,0.4):
  for fb in ((-45,110),(-45,0),(-45,20)):
    for sc in (0,):
      o,e=a.solve_limb({},f,"contact-fn",(x,-0.215),{f[0]:("off",sc),f[1]:("solve",)+fb,f[2]:("solve",),f[3]:("abs",-85),f[4]:("abs",-12)})
      print(x,fb,round(e,4),{k:round(v) for k,v in o.items()})
