import os,re,sys,hashlib,json,shutil
R="/Users/ueli/Documents/semio"
B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
OUT=os.path.dirname(os.path.abspath(__file__))+"/stage/files"
HELPER=B+"/🏠️machine-state/🟦️.ts"
FILES="""🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts
🌐️remote-cache/🛰️fleet/🟦️.ts
🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts
🌐️remote-execution/🌡️load/🟦️.ts
🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts
🌐️remote-execution/🛰️sidecar/🕸️graph/🟦️.ts
🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts
🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🏃️runner/🟨️.mjs
🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🔒️guard/🟦️.ts
🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🔬️inference/🟨️.mjs
🕸️graph/📥️inference/📦️project-nodes/🔱️warm/🟨️.mjs
🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs
🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts
🧠️memory/📋️slice-report/🟨️.mjs
🪪️graph-state/🌳️tree/🟦️.ts
🪪️graph-state/📍️cursor/🟦️.ts
🪪️graph-state/📜️script.ts
🪪️graph-state/📥️replica/🟦️.ts
🪪️graph-state/📦️project-records/🟦️.ts
🪪️graph-state/🧶️nx/🟦️.ts
🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts""".split("\n")
SPECIAL=[
 (r'join\(input\.root,\.\.\.P\.directory\)','machineStatePathV1(input.root,P.directory.join("/"))'),
 (r'join\(input\.root,P\.sliceReport\.directory,P\.sliceReport\.name\)','join(machineStatePathV1(input.root,P.sliceReport.directory),P.sliceReport.name)'),
 (r'join\(root,P\.directory\),file=join\(directory,P\.name\)','machineStatePathV1(root,P.directory),file=join(directory,P.name)'),
 (r'join\(root,\.\.\.policy\.directory\.split\("/"\)\)','machineStatePathV1(root,policy.directory)'),
 (r'join\(root,policy\.directory,policy\.file\)','join(machineStatePathV1(root,policy.directory),policy.file)'),
]
R1=re.compile(r'join\(\s*([A-Za-z_][A-Za-z_.]*)\s*,\s*"\.cache"\s*,\s*"f"\s*,\s*')
R2=re.compile(r'join\(\s*([A-Za-z_][A-Za-z_.]*)\s*,\s*"\.cache"\s*,\s*"f"\s*\)')
os.makedirs(OUT,exist_ok=True)
report={}
for f in FILES:
    p=B+"/"+f; t=open(p,encoding="utf8").read(); base=hashlib.sha256(t.encode()).hexdigest()[:12]; n=0
    t,c=R2.subn(r'machineStateDirectoryV1(\1)',t); n+=c
    t,c=R1.subn(r'join(machineStateDirectoryV1(\1),',t); n+=c
    for pat,new in SPECIAL:
        t,c=re.subn(pat,new,t); n+=c
    uses=[w for w in ("machineStateDirectoryV1","machineStatePathV1") if w in t]
    if n==0: report[f]=("NOCHANGE",base); continue
    rel=os.path.relpath(HELPER,os.path.dirname(p)).replace("\\","/")
    if not rel.startswith("."): rel="./"+rel
    mjs=f.endswith(".mjs")
    line=("import{%s}from\"%s\";"%(",".join(uses),rel)) if True else ""
    lines=t.split("\n"); last=-1
    for i,l in enumerate(lines):
        if re.match(r'^import\b[^\n]*;\s*$',l) and "from" in l: last=i
    if last<0: report[f]=("NOIMPORT",base); continue
    lines.insert(last+1,line); t="\n".join(lines)
    dst=OUT+"/"+base+"-"+os.path.basename(f)+"-"+hashlib.sha256(f.encode()).hexdigest()[:6]
    open(dst,"w",encoding="utf8").write(t)
    report[f]=(n,base,dst)
json.dump(report,open(os.path.dirname(OUT)+"/report.json","w"),ensure_ascii=False,indent=1)
for k,v in report.items(): print(v[0],v[1],k.split("/")[-2] if "/" in k else k, k[:70])
