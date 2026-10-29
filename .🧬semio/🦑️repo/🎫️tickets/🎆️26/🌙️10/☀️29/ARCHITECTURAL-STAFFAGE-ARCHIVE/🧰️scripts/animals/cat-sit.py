import sys; sys.path.insert(0, sys.argv[1]); import kin
a = kin.load(sys.argv[2], "domestic-cat")
h = ["thigh-hn","shank-hn","metatarsus-hn","paw-hn"]
rootS=(0,0,52); bs={"lumbar":4,"head":-51}
w=a.fk(bs,rootS); rx,ry=a.anchor(w,"contact-rump"); print(rx,ry,"hip",w["thigh-hn"][4:])
for d in (0.12,0.14,0.16,0.18,0.2):
  for m,p in ((-12,-4),(-20,-8),(-30,-10)):
    try:
        o,e=a.solve_limb(bs,h,"contact-hn",(rx+d,ry),{h[0]:("solve",),h[1]:("solve",),h[2]:("abs",m),h[3]:("abs",p)},root=rootS)
        print(d,m,p,{k:round(v,1) for k,v in o.items()},round(e,4))
    except Exception as ex: print(d,m,p,"none")
