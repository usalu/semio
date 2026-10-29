import sys
sys.argv=[sys.argv[0], sys.argv[1]]
exec(open("cow-ik.py").read())
for sc in (-20,-10,0,10,20):
    r=[]
    for x in (0.65,0.75,0.85,0.95,1.05,1.14,1.25,1.4,1.55,1.65):
        o,e=fore("n",x,-1.06,{}, (0,0,0),-86,-52,sc=sc); r.append((x,round(e,3)))
    print("fore sc",sc,[t for t in r if t[1]<0.005][:1],[t for t in r if t[1]<0.005][-1:])
for x in (-0.5,-0.4,-0.3,-0.2,0.0,0.3,0.45,0.55,0.65):
    o,e=hind("n",x,-1.06,{}, (0,0,0),-86,-52); print("hind",x,round(e,3))
