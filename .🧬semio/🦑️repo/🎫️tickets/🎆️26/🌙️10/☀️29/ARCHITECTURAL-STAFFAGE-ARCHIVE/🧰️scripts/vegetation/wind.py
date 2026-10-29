import sys, json, math
# usage: wind.py "name:cy:rx:ry,..." v10 cd [alpha] [depthFactor]
masses=[m.split(':') for m in sys.argv[1].split(',')]
v10=float(sys.argv[2]); cd=float(sys.argv[3]); alpha=float(sys.argv[4]) if len(sys.argv)>4 else 0.22; dep=float(sys.argv[5]) if len(sys.argv)>5 else 0.9
tot=0;mom=0
for n,cy,rx,ry in masses:
    cy,rx,ry=float(cy),float(rx),float(ry)
    A=math.pi*ry*rx*dep; v=v10*(max(cy,1)/10)**alpha; F=0.5*1.25*cd*A*v*v; tot+=F; mom+=F*cy
    print(f"{n}: A={A:.1f} m2 v={v:.1f} F={F:.0f} N")
print(f"sum {tot:.0f} N, moment about base {mom:.0f} Nm")
