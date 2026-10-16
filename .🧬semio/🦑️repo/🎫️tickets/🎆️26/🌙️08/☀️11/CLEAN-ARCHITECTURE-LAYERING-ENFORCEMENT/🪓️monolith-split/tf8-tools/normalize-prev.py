import sys
src,dst=sys.argv[1],sys.argv[2]
L=open(src,encoding="utf8").read().split("\n")
def idx(prefix, start=0):
    hits=[i for i,l in enumerate(L) if i>=start and l==prefix]
    assert len(hits)==1,(prefix,hits)
    return hits[0]
assert L[6]=="export { kernelGeometry };"
kg=idx("// #region 🧱️kernelGeometry")
skm=idx("// #region 🧮️SpatialKernelMath")
ppg=idx("// #region 🧱️PrimitivePreviewGeometry")
out=[]
for i,l in enumerate(L):
    if i==6: continue
    if i==ppg: out.append("// #endregion 🌀️EdgeGeometry"); out.append("")
    out.append(l)
    if i==kg: out.append("export { kernelGeometry };")
    if i==skm: out.append("// #region 🧮️Vec")
open(dst,"w",encoding="utf8").write("\n".join(out))
