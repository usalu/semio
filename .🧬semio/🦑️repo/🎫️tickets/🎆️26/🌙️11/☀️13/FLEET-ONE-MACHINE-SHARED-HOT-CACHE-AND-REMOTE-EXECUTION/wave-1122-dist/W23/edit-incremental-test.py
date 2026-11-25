import os
path="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts"
src=open(path,encoding="utf8").read()
reps=[
('const {withArtifactRelativeDirectoryV1}=await import("@semio-tech/framework-filesystem/observation/descriptor"),root=scratch(),message=process.platform==="win32"','const {withArtifactRelativeDirectoryV1,artifactRelativeRefusalV1}=await import("@semio-tech/framework-filesystem/observation/descriptor"),root=scratch(),message=process.platform==="win32"'),
('try{await capture(true);}catch(error){observed=(error as Error).message;}','try{await capture(true);}catch(error){observed=artifactRelativeRefusalV1(error);}'),
]
for old,new in reps:
    assert src.count(old)==1,old
    src=src.replace(old,new)
tmp=path+".w23.tmp"
open(tmp,"w",encoding="utf8").write(src);os.replace(tmp,path)
print("ok")
