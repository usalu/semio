import subprocess,sys,json
B="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
T=sys.argv[1]
def bez(P,t):
    u=1-t
    return tuple(u**3*P[0][k]+3*u*u*t*P[1][k]+3*u*t*t*P[2][k]+t**3*P[3][k] for k in (0,1))
def build(c):
    P=[(0,0),(0.0105,c),(0.0105,0.01),(0.0034,0.0155)]
    pts=[bez(P,i/2000) for i in range(2001)]
    def r(h): return min(pts,key=lambda p:abs(p[1]-h))[0]
    hs=[0.00163,0.00391]
    prof=[round(r(h),5) for h in hs]
    d=lambda y0,s: "M 0 %g C 0.0105 %g 0.0105 %g 0.0034 %g C 0.0017 %g 0.0006 %g 0 %g C -0.0006 %g -0.0017 %g -0.0034 %g C -0.0105 %g -0.0105 %g 0 %g Z"%(y0,y0+c,y0+0.01,y0+0.0155,y0+0.0183,y0+0.0205,y0+0.022,y0+0.0205,y0+0.0183,y0+0.0155,y0+0.01,y0+c,y0)
    return prof,d
for c in map(float,sys.argv[2:]):
    prof,d=build(c)
    e=("const a=asset.representations['200'].geometry; a.elevation.find(s=>s.id==='flame').d='%s'; a.front.find(s=>s.id==='flame').d='%s';"
       "const f=asset.volumes['200'].body.find(b=>b.id==='flame'); f.profile[1][1]=%g; f.profile[2][1]=%g;")%(d(0,0),d(0.0185,0),prof[0],prof[1])
    out=subprocess.run([B,T+"/fixup-try.ts","tealight","200",e],capture_output=True,text=True).stdout
    print("c=",c,prof); print(out)
