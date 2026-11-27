import os,re,sys
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
here=os.path.dirname(os.path.abspath(__file__))
path=L+"🟦️.ts"
s=open(path,encoding="utf8").read()
new=open(here+"/new-sites.ts.txt",encoding="utf8").read()
a=s.index("/** 🔎️ The calls of one source that open a path built by hand")
b=s.index("/** 📂️ Lists the scanned source files")
s=s[:a]+new+s[b:]
def sub(old,newt):
    global s
    assert s.count(old)==1,(old[:50],s.count(old));s=s.replace(old,newt)
sub("mountPatterns:readonly string[];openers:readonly string[];leaseCallees:readonly string[];leasedParameters:readonly string[];","mountPatterns:readonly string[];rootPatterns:readonly string[];rootTypes:readonly string[];openers:readonly string[];leaseCallees:readonly string[];leasedParameters:readonly string[];leasedArguments:readonly string[];")
sub("which words name the mount of a cache image, which calls open a file,","which words name the mount of a cache image, which helpers, values and types carry a path below an image (root patterns, root types), which calls open a file,")
sub("which calls and parameters mean that the read runs under the shared lease of the image, and the documented call sites.","which calls, parameters and arguments mean that the read runs under the shared lease of the image, and the documented call sites.")
sub("/** 🧭️ Whether a source names a path below the mount of a cache image. */\nexport function namesImageMountV1(source:string,policy:ImageLeaseLawPolicyV1):boolean{return matches(source,policy.mountPatterns);}","/** 🧭️ Whether a source names a path below the mount of a cache image, or a helper, value or type that carries one. */\nexport function namesImageMountV1(source:string,policy:ImageLeaseLawPolicyV1):boolean{return matches(source,[...policy.mountPatterns,...policy.rootPatterns,...policy.rootTypes]);}")
tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
print("law ts ok")
