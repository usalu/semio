import subprocess,re,collections,sys
S="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/"
files=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","--",S],capture_output=True,text=True).stdout.split("\n")
files=[f for f in files if f.endswith(".rs") and "🧫️fixtures" not in f]
def fam(path):
    p=path[len(S):].split("/")
    if p[0]=="🧬️schema":
        m={"📸️snapshot":"snapshot","💡️inferences":"inferences","🧬️mutations":"mutations","📏️authored":"authored","🔺️diff":"diff"}
        if len(p)>1 and p[1] in m: return "schema."+m[p[1]]
        return "schema.root"
    if p[0]=="🚪️io":
        sub={"📤️export":"export","📥️import":"import","📝️text":"text","💾️binary":"binary","🪶️sqlite":"sqlite"}
        if len(p)>1 and p[1] in sub: return "io."+sub[p[1]]
        return "io.root"
    return {"✏️editor":"editor","👁️viewer":"viewer","📚️examples":"examples","🖌️render":"render","🧪️tests":"tests"}.get(p[0],p[0])
names={"snapshot":"schema.snapshot","inferences":"schema.inferences","mutations":"schema.mutations","authored":"schema.authored","diff":"schema.diff"}
dep=collections.defaultdict(collections.Counter)
for f in files:
    try: t=open(f).read()
    except: continue
    src=fam(f)
    for m in re.finditer(r"(?:any|crate)::(schema|io|editor|viewer|examples|render)(?:::(\w+))?",t):
        a,b=m.group(1),m.group(2)
        if a=="schema": tgt=names.get(b,"schema.root")
        elif a=="io": tgt="io."+(b if b in("export","import","text","binary","sqlite") else "root")
        else: tgt=a
        if tgt!=src: dep[src][tgt]+=1
for s in sorted(dep):
    print(s,"->",", ".join(f"{t}:{n}" for t,n in dep[s].most_common()))
