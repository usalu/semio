#!/usr/bin/env python3
"""table.py: read-only numbers for R10 from the sidecar logs, the counters under .cache/f and the unit-cache event logs of this machine. Prints markdown."""
import os,re,json,glob,collections,datetime
R="/Users/Shared/semio";F=R+"/.cache/f";U=R+"/.cache/b/r/m/u"
def size(p):
    try:return os.path.getsize(p)
    except OSError:return 0
def ts(ms):return datetime.datetime.fromtimestamp(ms/1000).strftime("%m-%d %H:%M")
print("## Counters (unit-counters, one byte per decision; since %s)"%ts(int(open(F+"/unit-counters/since").read())))
c={k:size(F+"/unit-counters/"+k) for k in("localHit","remoteHit","miss","stored","skipped")}
look=c["localHit"]+c["remoteHit"]+c["miss"]
print("| localHit | remoteHit | miss | stored | skipped | lookups (hit+hit+miss) | share of lookups served without compiling |\n|---|---|---|---|---|---|---|")
print("| %d | %d | %d | %d | %d | %d | %.1f %% |"%(c["localHit"],c["remoteHit"],c["miss"],c["stored"],c["skipped"],look,100*(c["localHit"]+c["remoteHit"])/max(1,look)))
print("\n## Offload counters\n| out | refused | fellBack |\n|---|---|---|\n| %d | %d | %d |"%tuple(size(F+"/offload-counters/"+k) for k in("out","refused","fellBack")))
print("\n| reason | count |\n|---|---|")
for p in sorted(glob.glob(F+"/offload-counters/reason/*"),key=lambda p:-size(p)):print("| %s | %d |"%(os.path.basename(p),size(p)))
rows=[]
for f in glob.glob(U+"/x/*.jsonl"):
    for l in open(f,errors="ignore"):
        try:rows.append(json.loads(l))
        except ValueError:pass
at=[r["atMs"] for r in rows if "atMs" in r]
print("\n## Event logs of this machine (%s to %s, %d events)"%(ts(min(at)),ts(max(at)),len(rows)))
kinds=collections.Counter(r["kind"] for r in rows)
print("| kind | count |\n|---|---|")
for k,v in kinds.most_common():print("| %s | %d |"%(k,v))
print("\nmiss reasons / skips:\n\n| kind | reason | count |\n|---|---|---|")
rc=collections.Counter()
for r in rows:
    if r["kind"] in("offload-declined","offload-local-fallback","remote-skip","bypass","failed"):
        why=re.sub(r"[0-9a-f]{12,}","H",(r.get("reason") or ""));d=r.get("detail") or ""
        if r["kind"]=="offload-declined":why+=": "+re.sub(r"[0-9a-f]{12,}","H",d)[:40]
        if why.startswith("path-dependent"):why="path-dependent (compiled bytes hold a machine path)"
        rc[(r["kind"],why[:80])]+=1
for (k,w),v in rc.most_common(14):print("| %s | %s | %d |"%(k,w,v))
crates=collections.defaultdict(set)
for r in rows:
    if r["kind"]=="miss" and r.get("reason"):crates[r["crate"]].add(r["key"])
multi=[c for c,k in crates.items() if len(k)>1]
print("\ncrates with more than one miss key in the window (key churn or real source changes): %d of %d"%(len(multi),len(crates)))
print("\n## Scheduler log (build-scheduler.log)")
sl=open(F+"/sidecars/build-scheduler.log",errors="ignore").read().splitlines()
pol=collections.Counter(re.sub(r"^.*policy: ","",l) for l in sl if "worker policy" in l)
print("| refusal by the worker policy | count |\n|---|---|")
for k,v in pol.most_common():print("| %s | %d |"%(k,v))
print("\nretried: %d, declined: %d, refused by a worker: %d, worker-joined: %d, worker-left: %d"%(sum("scheduler retried" in l for l in sl),sum("scheduler declined" in l for l in sl),sum("scheduler refused" in l for l in sl),sum("worker-joined" in l for l in sl),sum("worker-left" in l for l in sl)))
print("\n## Cache server log (build-cache.log.1, PUT failures)")
bl=open(F+"/sidecars/build-cache.log.1",errors="ignore").read()
puts=re.findall(r"PUT - /main/cas/([0-9a-f]+) failed",bl);low=len(re.findall(r"refused process.disk.low-space",bl))
print("PUT failed lines: %d for %d distinct blobs; 'disk.low-space' refusals in the same file: %d; partition usage in the refusals: %s"%(len(puts),len(set(puts)),low,sorted(set(re.findall(r'usedPercent: (\d+)',bl)))[:3]))
cl=open(F+"/sidecars/build-cache.log",errors="ignore").read()
print("build-cache.log (current): restarts %d, PUT failures %d"%(cl.count("cache code changed"),len(re.findall(r"PUT .* failed",cl))))
claims=0;stored=0
try:
    for l in open(F+"/cache-server/v1/claims.jsonl"):claims+=1
except OSError:pass
print("\nclaims.jsonl lines: %d; entries in the cache index: main/ac %d, semio/ac %d"%(claims,len(os.listdir(F+"/cache-server/v1/main/ac")),len(os.listdir(F+"/cache-server/v1/semio/ac"))))
