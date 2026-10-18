import json,re,sys
jl,pat=sys.argv[1],sys.argv[2]
width=int(sys.argv[3]) if len(sys.argv)>3 else 230
for l in open(jl,encoding="utf-8"):
    r=json.loads(l)
    if r["rc"]==0 or pat not in r["file"]: continue
    print("=====",r["file"],f'p{r["pass_"]} f{r["fail"]}')
    lines=open(r["log"],encoding="utf-8").read().split("\n")
    for i,x in enumerate(lines):
        if x.strip().startswith("^") and i>0:
            code=lines[i-1]; err=""
            for y in lines[i+1:i+6]:
                if y.startswith("error:"): err=y;break
            m=re.match(r"\s*(\d+) \|\s*(.*)",code)
            print("  L"+(m.group(1) if m else "?"),(m.group(2) if m else code)[:width],"  ##",err[:100])
        elif x.startswith("(fail)"): print("  ",x[:150])
