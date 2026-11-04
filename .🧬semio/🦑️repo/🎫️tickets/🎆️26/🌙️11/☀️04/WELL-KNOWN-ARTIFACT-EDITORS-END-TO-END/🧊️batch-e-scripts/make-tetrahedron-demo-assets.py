import sys
V=[(0,0,0),(1,0,0),(0,1,0),(0,0,1)]
F=[(0,2,1),(0,1,3),(0,3,2),(1,2,3)]
def sub(a,b): return tuple(x-y for x,y in zip(a,b))
def cross(a,b): return (a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])
def norm(n):
    l=sum(c*c for c in n)**0.5
    return tuple(c/l for c in n)
def f(x): return repr(float(x)).rstrip('0').rstrip('.') if float(x)!=int(x) else str(int(x))
stl=["solid tetrahedron"]
for t in F:
    a,b,c=(V[i] for i in t)
    n=norm(cross(sub(b,a),sub(c,a)))
    stl.append("  facet normal "+" ".join(f(x) for x in n))
    stl.append("    outer loop")
    for p in (a,b,c): stl.append("      vertex "+" ".join(f(x) for x in p))
    stl.append("    endloop"); stl.append("  endfacet")
stl.append("endsolid tetrahedron")
open(sys.argv[1],'w').write("\n".join(stl)+"\n")
ply=["ply","format ascii 1.0","comment tetrahedron","element vertex 4","property float x","property float y","property float z","element face 4","property list uchar int vertex_indices","end_header"]
ply+=[" ".join(f(x) for x in v) for v in V]
ply+=["3 "+" ".join(map(str,t)) for t in F]
open(sys.argv[2],'w').write("\n".join(ply)+"\n")
