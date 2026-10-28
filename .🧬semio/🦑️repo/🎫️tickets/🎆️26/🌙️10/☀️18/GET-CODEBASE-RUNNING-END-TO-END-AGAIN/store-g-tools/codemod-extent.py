import os,re,sys
root="/Users/ueli/Documents/semio"
S=root+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts"
sparse=S+"/🍎️image/🪶️sparse/🟦️.ts"
facade=root+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🟦️.ts"
files=[S+"/🧹️whole/🟦️.ts",S+"/📤️publication/🎛️owner/🟦️.ts",S+"/📤️publication/🌐️platform/🟦️.ts",S+"/🧹️cleanup/🎛️command/🟦️.ts",S+"/🧱️fixed/🟦️.ts",L]
redundant=re.compile(r"(\w+)\.size>((?:(?!\|\|)[^|])+?)\|\|\1\.blocks\*512n?>\2(?![\w.])")
lower=re.compile(r"\|\|(\w+)\.blocks\*512<\1\.size")
use=re.compile(r"\b(\w+)\.blocks\*512n?\b")
total=0
for f in files:
    t=open(f,encoding="utf8").read(); o=t
    t,a=redundant.subn(lambda m:m.group(1)+".size>"+m.group(2),t)
    t,b=lower.subn("",t)
    t,c=use.subn(lambda m:"nativeImageBackingAllocatedBytesV1("+m.group(1)+")",t)
    if c+a+b==0: continue
    target=facade if f==L else sparse
    rel=os.path.relpath(target,os.path.dirname(f))
    if not rel.startswith("."): rel="./"+rel
    imp='import{nativeImageBackingAllocatedBytesV1}from"'+rel+'";\n'
    if "nativeImageBackingAllocatedBytesV1}from" not in o:
        i=t.index("\n")+1 if t.startswith("import") else 0
        t=t[:i]+imp+t[i:]
    open(f,"w",encoding="utf8").write(t)
    print(os.path.basename(os.path.dirname(f)),"redundant",a,"lower",b,"use",c,"->",rel[-60:])
