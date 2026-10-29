import sys, importlib, math; sys.path.insert(0, sys.argv[1]); kin = importlib.import_module('cat-kin')
a = kin.load(sys.argv[2], "domestic-cat")
for ang,gy,base in ((-4,-0.225,{"lumbar":4}),(-17,-0.235,{"lumbar":6,"thorax-a":4}),(-26,-0.24,{"lumbar":10,"thorax-a":6,"thorax-b":2})):
    w=a.fk(base,(-0.02,0,ang)); sh=w["humerus-fn"]; hip=w["thigh-hn"]
    print(ang,"shoulder",round(sh[4],3),round(sh[5]-gy,3),"dist to fore target", round(math.hypot(0.42-sh[4], gy-sh[5]),3),"hip",round(hip[4],3),round(hip[5]-gy,3), "hipdist to paw", round(math.hypot(0-hip[4], gy-hip[5]),3))
