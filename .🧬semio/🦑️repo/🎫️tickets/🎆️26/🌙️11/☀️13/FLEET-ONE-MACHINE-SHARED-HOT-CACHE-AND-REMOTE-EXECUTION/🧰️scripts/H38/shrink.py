import re,sys
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs"
t=open(p,encoding="utf8").read()
a=t.index("  if(ephemeral){\n   if(cold||repair||!usable)")
b=t.index("  }\n",t.index("carried:carried?.length??0};",a))+4
t=t[:a]+"  if(ephemeral)return overlayResultV1({current,usable,identity,implementation,found,computed,carried,marks,computePhases,envDigest,computedParts,live,owners,returned,globalPayload,store,P,cold,repair});\n"+t[b:]
t=t.replace('import{mergeIntoHeadV1}from"./🔀️merge/🟨️.mjs";','import{mergeIntoHeadV1}from"./🔀️merge/🟨️.mjs";\nimport{overlayResultV1}from"./🫧️overlay/🟨️.mjs";',1)
t=re.sub(r"\n[ \t]+\n","\n\n",t)
open(p+".h38tmp","w",encoding="utf8").write(t)
import os;os.replace(p+".h38tmp",p);print(len(t.encode()))
