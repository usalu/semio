import os
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/🟦️.ts"
s=open(p,encoding="utf8").read()
old="carrierMethods:readonly string[];carrierMethods:readonly string[];"
assert s.count(old)==1
s=s.replace(old,"carrierMethods:readonly string[];")
tmp=p+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,p);print("dup fixed")
