import sys
exec(open(sys.argv[1]+"/pigeon-emit.py").read().split("P = []; CL = []")[0])
for H in (0.1,0.105,0.11):
 for x in (-0.04,-0.05):
  for tars,toes in ((-100,-40),(-95,-35),(-90,-30)):
    a1=a.solve_limb({},LEG("n"),"contact-n",(x,-H),{"thigh-n":("solve",),"shank-n":("solve",),"tarsus-n":("abs",tars),"toes-n":("abs",toes)})
    print(H,x,tars,toes,round(a1[1],4),{k:round(v) for k,v in a1[0].items()})
