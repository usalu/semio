import os,re,json,sys
root="/Users/ueli/Documents/semio/🧰️framework"
path=root+"/🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🧪️tests/🔣️.json"
pat=re.compile(r"evaluateExtractedFunctionV1|new Function\(")
live=json.load(open(path,encoding="utf8"))
for rel in sys.argv[1:]:
    rel=rel[len(root)+1:] if rel.startswith(root+"/") else rel
    n=len(pat.findall(open(os.path.join(root,rel),encoding="utf8").read()))
    old=live["ceilings"].get(rel)
    if n: live["ceilings"][rel]=n
    else: live["ceilings"].pop(rel,None)
    print(rel[-70:],old,"->",n)
live["ceilings"]=dict(sorted(live["ceilings"].items()))
json.dump(live,open(path,"w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(live["ceilings"]),sum(live["ceilings"].values()))
