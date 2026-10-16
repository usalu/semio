import json,os,re,shutil,subprocess,sys
sys.argv=[sys.argv[0]]
import oracle as o
i=int(os.environ.get("UNIT","3"))
u=o.units[i]; src=f"{o.S}/sb/new/{i:02d}"; dst=f"{o.S}/sb/sens/{i:02d}"
shutil.rmtree(dst,ignore_errors=True); shutil.copytree(src,dst)
h=os.path.join(dst,o.HELPER,"🦀️.rs"); s=open(h,encoding="utf8").read()
for name in ["pub struct RequestedAllocator","pub fn observe<","pub fn observed_requested_bytes","pub fn observe_backing<"]:
    assert name in s
    s=s.replace(name,name.replace("pub ",""),1)
open(h,"w",encoding="utf8").write(s)
rb=o.summarize(open(f"{o.S}/sb/log-{i:02d}-new.txt").read(),src)
rn=o.summarize(o.run(dst,u,["--test"]),dst)
d=rn-rb
print(u["name"],"baseline-new errors",sum(rb.values()),"after making the four helper items private:",sum(rn.values()),"new errors:",sum(d.values()))
for k,v in list(d.items())[:6]: print("  +",v,k)
