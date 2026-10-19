import os,re,sys
pat=re.compile(sys.argv[1]);skip={"node_modules","dist","target",".git","bazel-out","pkg","temp"}
for d,ds,fs in os.walk(sys.argv[2]):
    ds[:]=[x for x in ds if x not in skip]
    for f in fs:
        if not re.search(r"\.(ts|mjs|mts|json|md|cjs|tsx|jsonc)$",f):continue
        p=os.path.join(d,f)
        try:t=open(p,encoding="utf8").read()
        except:continue
        if pat.search(t):print(p)
