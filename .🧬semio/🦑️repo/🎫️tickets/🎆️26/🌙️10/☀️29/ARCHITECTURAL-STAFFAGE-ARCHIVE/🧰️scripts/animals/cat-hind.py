import sys, importlib, math; sys.path.insert(0, sys.argv[1]); kin = importlib.import_module('cat-kin')
a = kin.load(sys.argv[2], "domestic-cat")
h = ["thigh-hn","shank-hn","metatarsus-hn","paw-hn"]
base={"lumbar":6,"thorax-a":4}; ang=-17
w=a.fk(base,(0,0,ang)); hip=w["thigh-hn"]; print("hip",hip[4:])
for gy in (-0.222,-0.213):
  for hx in (0.02,0.03,0.043,0.06):
    for m,p in ((-82,-12),(-70,-12),(-55,-15)):
      try:
        o,e=a.solve_limb(base,h,"contact-hn",(hx,gy),{h[0]:("solve",-45,40),h[1]:("solve",),h[2]:("abs",m),h[3]:("abs",p)},root=(0,0,ang))
        print(hx,m,round(e,4),{k:round(v) for k,v in o.items()})
      except TypeError: print(hx,m,"none")
