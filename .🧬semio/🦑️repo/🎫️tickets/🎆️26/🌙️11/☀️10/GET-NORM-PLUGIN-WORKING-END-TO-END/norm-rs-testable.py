import json,re,sys
meta=json.load(open(sys.argv[1])); log=open(sys.argv[2],errors="ignore").read()
broken=set(re.findall(r"could not compile `([^`]+)`",log))
ws=set(meta["workspace_members"]); pk={p["id"]:p for p in meta["packages"]}
deps={n["id"]:[d["pkg"] for d in n["deps"]] for n in meta["resolve"]["nodes"]}
memo={}
def bad(i):
    if i in memo: return memo[i]
    memo[i]=False
    memo[i]=pk[i]["name"] in broken or any(bad(d) for d in deps.get(i,[]))
    return memo[i]
ok=[pk[i]["name"] for i in ws if not bad(i)]
print(len(ws),len(ok),file=sys.stderr)
print(" ".join("-p "+n for n in sorted(ok)))
