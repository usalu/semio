import subprocess,sys,json,os
root=sys.argv[1]; flags=sys.argv[2:]
env=dict(os.environ,CARGO_MANIFEST_DIR=root)
p=subprocess.run(["rustc","+nightly-2026-07-20","--edition","2021","--crate-type","lib","--crate-name","combined","--emit=metadata","--error-format=json","-o",root+"/.raw.rmeta",*flags,root+"/__combined.rs"],capture_output=True,text=True,env=env)
out=[]
for l in p.stderr.split("\n"):
    if not l.startswith("{"): continue
    try: m=json.loads(l)
    except: continue
    if m.get("level")!="error": continue
    sp=[s for s in m.get("spans",[]) if s["is_primary"]]
    out.append((m.get("code",{}) and m["code"]["code"] or "",m["message"],sp[0]["file_name"] if sp else "",sp[0]["line_start"] if sp else 0))
for o in out: print("\t".join(map(str,o)))
