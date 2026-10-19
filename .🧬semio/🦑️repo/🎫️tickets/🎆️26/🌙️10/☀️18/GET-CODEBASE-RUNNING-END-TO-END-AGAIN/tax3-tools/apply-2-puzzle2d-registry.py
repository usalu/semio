# 🔧 Registers the puzzle 2d domain-nested mutation tree in mutationDomainOwners (run after the taxonomy freeze lifts). Row derived from the leaf descriptors (semanticKind).
import json,os
repo="/Users/ueli/Documents/semio"
import os as _os
L=_os.environ.get("TAX3_L") or f"{repo}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
rel="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
root=f"{repo}/{rel}"
row={}
for dom in sorted(os.listdir(root)):
    dp=os.path.join(root,dom)
    if not os.path.isdir(dp) or dom in ("🧪️tests","📦️packages") or os.path.isfile(dp+"/🔣️.json"): continue
    leaves={}
    for leaf in sorted(os.listdir(dp)):
        f=os.path.join(dp,leaf,"🔣️.json")
        if os.path.isfile(f): leaves[leaf]=json.load(open(f,encoding="utf8"))["semanticKind"]
    if leaves: row[dom]=leaves
p=f"{L}/🔣️taxonomy.json"
s=open(p,encoding="utf8").read()
anchor='    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations": {\n'
assert s.count(anchor)==1 and rel not in s.split('"mutationDomainOwners"')[1].split('"subsetDirectoryOverrides"')[0] if '"subsetDirectoryOverrides"' in s else True
body=json.dumps(row,ensure_ascii=False,indent=2).split("\n")
block=f'    "{rel}": '+"\n".join([body[0]]+["    "+l for l in body[1:]])+",\n"
s=s.replace(anchor,block+anchor)
open(p,"w",encoding="utf8").write(s)
json.loads(s)
print("registered",sum(len(v) for v in row.values()),"leaves")
