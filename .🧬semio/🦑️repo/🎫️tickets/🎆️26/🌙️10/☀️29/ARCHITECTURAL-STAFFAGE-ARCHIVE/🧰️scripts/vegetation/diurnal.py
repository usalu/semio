import json,subprocess,sys
# usage: diurnal.py <id> '<emitter strengths json: {emitter:[9 values]}>' '<sun-exposed region id or "">'  -> design/<id>-diurnal.txt (tracks from the engine's own balance at hourly solar levels)
i=sys.argv[1]; em=json.loads(sys.argv[2]); sunreg=sys.argv[3]
o=subprocess.check_output(["repo","tools","exec","bun","--","thermal-calc.ts",f"assets/{i}.json"],text=True)
d=json.loads(next(l for l in o.splitlines() if l.startswith("JSON"))[4:]); T=d["temps"]
H=[0,5,6,8,10,12,14,16,18,20,24]; idx=[0,0,1,2,3,4,5,6,7,0,0]  # solar level index per hour key: 0,0,50,300,600,800,650,350,80,0,0
hrs=[0,5,7,9,12,15,18,20,24]
def tr(t,keys): return '{"target":"%s","ease":"smooth","keys":[%s]}'%(t,",".join('{"t":%s,"v":%s}'%(a,b) for a,b in keys))
rows=[tr(f"emitter:{e}",list(zip(hrs,v))) for e,v in em.items()]
for r,v in T.items(): rows.append(tr(f"region:{r}:surfaceTempC",[(h,v[k]) for h,k in zip(H,idx)]))
if sunreg: rows.append(tr(f"region:{sunreg}:exposedFraction",list(zip([0,6,8,10,12,14,16,18,24],[0.3,0.3,0.6,0.85,1,0.85,0.55,0.3,0.3]))))
open(f"design/{i}-diurnal.txt","w").write(",\n".join(rows)+"\n")
print("latent kW per solar",[round(x/1000,1) for x in d["latentW"]])
