import subprocess,sys,json,time,os
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E50/"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
for rel in sys.argv[1:]:
    while os.getloadavg()[0]>40: time.sleep(60)
    t=time.time()
    p=subprocess.run(["repo","test-file","--json",B+rel],cwd="/Users/ueli/Documents/semio",capture_output=True,text=True)
    n=str(__import__("zlib").crc32(rel.encode()))
    open(G+"out-"+n+".txt","w").write(p.stdout+"\n----\n"+p.stderr)
    try:
        d=json.JSONDecoder().raw_decode(p.stdout[p.stdout.index("{"):])[0]["files"][0]
        tail=d.get("tail") or ""
        errs=[l.strip()[:200] for l in tail.split("\n") if l.startswith("error:") or l.startswith("(fail)")][:8]
        print(p.returncode,round(time.time()-t),n,d["status"],d.get("pass"),d.get("fail"),rel)
        for e in errs: print("    ",e)
        if not errs: print("    ",(d.get("error") or "")[:300])
    except Exception as e:
        print(p.returncode,round(time.time()-t),n,"NOJSON",rel,p.stdout[:200],p.stderr[-300:])
    sys.stdout.flush()
