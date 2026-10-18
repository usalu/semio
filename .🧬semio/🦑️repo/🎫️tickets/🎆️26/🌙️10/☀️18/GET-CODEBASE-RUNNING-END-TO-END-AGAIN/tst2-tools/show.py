import json,re,sys
jl=sys.argv[1]; pat=sys.argv[2]; n=int(sys.argv[3]) if len(sys.argv)>3 else 40
for l in open(jl,encoding="utf-8"):
    r=json.loads(l)
    if r["rc"]!=0 and pat in r["file"]:
        print("=====",r["file"][-110:],f'p{r["pass_"]} f{r["fail"]} e{r["err"]}')
        t=open(r["log"],encoding="utf-8").read()
        lines=[x for x in t.split("\n") if re.match(r"^(error|\(fail\)|Expected|Received|  Expected|  Received)|^\s+(at |^)",x) is not None and not x.strip().startswith("at ")]
        for x in lines[:n]: print(x[:300])
