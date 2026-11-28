p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts"
s=open(p,encoding="utf8").read()
a='" lies inside "+outer.image!.root+" ("+outer.stage+")");}'
b='" lies inside "+outer.image!.root+" ("+outer.stage+"), "+(persistentChildContext.getStore()?"child frame of "+persistentChildContext.getStore()!.options.directory:"no child frame"));}'
assert s.count(a)==1
open(p,"w",encoding="utf8").write(s.replace(a,b))
print("ok")
