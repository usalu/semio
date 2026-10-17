import sys,re,os,json,subprocess
n=sys.argv[1]; elem=sys.argv[2]; G=os.environ["G"]; R=os.environ["R"]
files=[]
for l in open(f"{G}/c4p27-refs-{n}.txt",encoding="utf8"):
    m=re.match(r"^(.*?\.(?:tsx?|mts|cts)):(\d+):(.*)$",l.rstrip("\n"))
    if m: files.append(m.group(1))
rel=os.path.relpath(os.path.join(os.environ["E"],elem),R)
out=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","--cached","--others","--exclude-standard","--",rel],cwd=R,capture_output=True,text=True).stdout.split("\n")
files+= [f for f in out if re.search(r"\.(tsx?|mts|cts)$",f)]
seen=[]
for f in files:
    if f not in seen: seen.append(f)
print(json.dumps([os.path.join(R,f) for f in seen],ensure_ascii=False,indent=1))
