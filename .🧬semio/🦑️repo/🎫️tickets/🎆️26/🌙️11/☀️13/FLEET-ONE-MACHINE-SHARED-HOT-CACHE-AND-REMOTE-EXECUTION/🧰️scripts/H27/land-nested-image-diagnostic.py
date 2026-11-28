p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts"
s=open(p,encoding="utf8").read()
a='else if([...persistentReceivers].some(row=>row.image&&row.stage!=="closed"&&options.directory.startsWith(row.image.root+sep)))throw Error("Nested persistent image requires actual parent scope");'
b='else{const outer=[...persistentReceivers].find(row=>row.image&&row.stage!=="closed"&&options.directory.startsWith(row.image.root+sep));if(outer)throw Error("Nested persistent image requires actual parent scope: "+options.directory+" lies inside "+outer.image!.root+" ("+outer.stage+")");}'
assert s.count(a)==1
open(p,"w",encoding="utf8").write(s.replace(a,b))
print("ok")
