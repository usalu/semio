import os,json
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:60],s.count(old));s=s.replace(old,new)
    tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
patch(L+"🧬️schema/🔣️.json",[
 ('    "leasedArguments",\n','    "leasedArguments",\n    "carrierMethods",\n'),
 ('    "leasedArguments": {\n      "type": "array",','    "carrierMethods": {\n      "type": "array",\n      "minItems": 1,\n      "description": "Names of methods that store their arguments in their receiver (`candidates.push(path)`, `known.set(key, path)`): when an argument carries a path below an image, the receiver carries it on.",\n      "items": {\n        "type": "string",\n        "minLength": 1\n      }\n    },\n    "leasedArguments": {\n      "type": "array",'),
])
patch(L+"📒️allowlist/🔣️.json",[(' "leasedArguments": [',' "carrierMethods": [\n  "push",\n  "unshift",\n  "add",\n  "set"\n ],\n "leasedArguments": [')])
path=L+"🧪️tests/🧫️fixtures/🔣️.json"
d=json.loads(open(path,encoding="utf8").read());allow=json.loads(open(L+"📒️allowlist/🔣️.json",encoding="utf8").read())
p=d["policy"];p["carrierMethods"]=allow["carrierMethods"]
order=["version","roots","skipDirectories","extensions","skipSuffixes","mountPatterns","rootPatterns","rootTypes","openers","leaseCallees","leasedParameters","leasedArguments","carrierMethods","classes","entries"]
d["policy"]={k:p[k] for k in order}
C=lambda name,source,unleased:{"name":name,"source":source,"named":True,"unleased":unleased}
d["cases"]+=[
 C("a path stored in a list and read back through a loop","const base=cacheAreaDirectoryV1(workspace,ledger,area);\nconst found=[];\nfound.push(join(base,\"a\"));\nfor(const path of found)unlinkSync(path);",["unlinkSync"]),
 C("a path stored in a map with a key and read back","const base=cacheAreaDirectoryV1(workspace,ledger,area);\nconst stamps=new Map();\nstamps.set(\"k\",base);\nconst at=stamps.get(\"k\");\nstatSync(at);",["statSync"]),
 C("a list of plain names stays clean","const base=cacheAreaDirectoryV1(workspace,ledger,area);\nconst names=[];\nnames.push(\"a\");\nreaddirSync(names[0]);",[]),
]
tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n");os.replace(tmp,path)
print("carriers ok")
