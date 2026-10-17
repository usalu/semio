import json,glob,os,subprocess,sys
sys.path.insert(0,os.path.dirname(__file__))
import fixrt
R="/Users/ueli/Documents/semio/"
files=set()
for l in open(fixrt.ORIG+"/index.tsv"):
    h,sha,p=l.rstrip("\n").split("\t")
    if p.endswith(".rs") and os.path.exists(p): files.add(p)
for f in glob.glob(os.path.dirname(__file__)+"/specs/*.json"):
    files.add(R+json.load(open(f))["dir"]+"/🦀️.rs")
bad=0
for p in sorted(files):
    r=subprocess.run(["rustfmt","+nightly-2026-07-20","--edition","2021","--emit","stdout","--config","skip_children=true",p],capture_output=True,text=True)
    if r.returncode!=0:
        bad+=1; print("FAIL",p.split("subsets/")[-1][-80:],r.stderr[:300])
print("files",len(files),"parse failures",bad)
