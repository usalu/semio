import json,os,re,subprocess,sys,time,glob
R="/Users/ueli/Documents/semio";S="/Users/Shared/semio-store/v1/origins"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
def jl(p):
    try:return json.load(open(p))
    except Exception:return None
print("UTC",time.strftime("%H:%M:%S",time.gmtime()))
ss=jl(R+"/.cache/f/graph-server/serve-state.json") or {}
print("serve-state held",[(h["request"]["requester"],len(h["list"]),h["told"]) for h in ss.get("held",[])],"told",len(ss.get("told",[])),"reading",ss.get("reading"))
rec=jl(R+"/.cache/f/graph-server/read-record.json");print("read-record",rec and {k:rec[k] for k in("runner","read","status")})
log=R+"/.cache/f/graph-server/read.log"
try:
    lines=open(log,errors="replace").read().splitlines()
    own=[l for l in lines if l.startswith("[owners]")];print("server [owners] lines",len(own));print("\n".join(own[-4:])[:900])
    print("\n".join([l[:260] for l in lines if l.startswith("[fleet]")][-3:]))
except Exception as e:print(e)
print("pool.json:")
for o in sorted(os.listdir(S)):
    d=jl(S+"/"+o+"/pool.json")
    if d:print(" ",o,{k:d.get(k) for k in("role","state","units","claimed","published","stored","held","failed","running","ownersPerMinute","children","target","reason")})
r=subprocess.run(["python3","-I",T+"/🧰️scripts/R54/ready-table.py"],capture_output=True,text=True);print(r.stdout[-1800:])
