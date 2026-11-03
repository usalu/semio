import sys, importlib, pickle
from fractions import Fraction
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
part, cutters, result = pickle.load(open("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/rb2/capture.pkl", "rb"))
def show(label, p):
    vs = api.part_vertices(p)
    print(label, "volume", float(api._native_parts_exact_volume([p])), "nverts", len(vs), "faces", [len(f) for f in api.part_faces(p)])
    for v in vs: print("    ", tuple(float(c) for c in v))
show("A", part)
for i, c in enumerate(cutters): show(f"B{i}", c)
for i, c in enumerate(result): show(f"R{i}", c)
def convex(p):
    vs = [tuple(Fraction(c) for c in v) for v in api.part_vertices(p)]
    bad = 0
    for face in api.part_faces(p):
        a = vs[face[0]]
        for i in range(1, len(face)-1):
            u = [vs[face[i]][k]-a[k] for k in range(3)]; w = [vs[face[i+1]][k]-a[k] for k in range(3)]
            n = [u[1]*w[2]-u[2]*w[1], u[2]*w[0]-u[0]*w[2], u[0]*w[1]-u[1]*w[0]]
            if any(n): break
        off = sum(n[k]*a[k] for k in range(3)); vals = [sum(n[k]*v[k] for k in range(3))-off for v in vs]
        if max(vals) > 0 and min(vals) < 0: bad += 1
    return bad
print("nonconvex faces A,B0,B1:", convex(part), [convex(c) for c in cutters])
print("==== stepwise")
for k in range(len(cutters) + 1):
    r = api._reference_material_parts(part, cutters[:k])
    print("after", k, "cutters:", len(r), "cells, volume", float(api._native_parts_exact_volume(r)))
r1 = api._reference_material_parts(part, [cutters[0]])
print("A-B0 vs expected: ", float(api._native_parts_exact_volume([part])) - float(api._native_parts_exact_volume([cutters[0]])) , "(upper bound; overlap makes it larger)")
r01 = api._reference_material_parts(part, [cutters[1]])
print("A-B1 (sliver) volume", float(api._native_parts_exact_volume(r01)), "cells", len(r01))
for c in r1:
    vs = [tuple(float(x) for x in v) for v in api.part_vertices(c)]
    print("A-B0 cell volume", float(api._native_parts_exact_volume([c])), "x", min(v[0] for v in vs), max(v[0] for v in vs), "y", min(v[1] for v in vs), max(v[1] for v in vs))
print("==== plane steps for B1")
backend = api._surface_finishes()
cell = api._native_material_cells(cutters[1])
print("B1 native cells", len(cell))
cell = cell[0]
vertices = api.part_vertices(cell)
print("vertex types", type(vertices[0][0]))
def planes(cell):
    vertices = api.part_vertices(cell)
    for face in api.part_faces(cell):
        anchor = vertices[face[0]]
        for i in range(1, len(face)-1):
            u, v = [[vertices[index][k]-anchor[k] for k in range(3)] for index in (face[i], face[i+1])]
            normal = [u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0]]
            if any(normal): break
        else: raise ValueError("degenerate")
        yield {"normal": normal, "offset": sum(n*x for n, x in zip(normal, anchor)), "points": [anchor, vertices[face[i]], vertices[face[i+1]]]}
stocks = [backend.clip_stock(c, [], exact=True) for c in api._native_material_cells(part)]
current = stocks[0]
for n, plane in enumerate(planes(cell)):
    flipped = {**plane, "normal": [-v for v in plane["normal"]], "offset": -plane["offset"]}
    outside = backend.clip_stock(current, [flipped])
    ov = None if outside is None else float(api._native_parts_exact_volume([{"mesh": outside._exact_mesh}]))
    nxt = backend.clip_stock(current, [plane])
    iv = None if nxt is None else float(api._native_parts_exact_volume([{"mesh": nxt._exact_mesh}]))
    print("plane", n, "normal", [float(x) for x in plane["normal"]], "outside vol", ov, "inside vol", iv)
    current = nxt
    if current is None: break
print("==== exact face planarity of B1 and A and B0")
import itertools
def planarity(label, p):
    vs = [tuple(Fraction(c) for c in v) for v in api.part_vertices(p)]
    for fi, face in enumerate(api.part_faces(p)):
        if len(face) < 4: continue
        worst = 0
        normals = []
        for i, j, k in itertools.combinations(range(len(face)), 3):
            a, b, c = vs[face[i]], vs[face[j]], vs[face[k]]
            u = [b[t]-a[t] for t in range(3)]; w = [c[t]-a[t] for t in range(3)]
            n = [u[1]*w[2]-u[2]*w[1], u[2]*w[0]-u[0]*w[2], u[0]*w[1]-u[1]*w[0]]
            normals.append(n)
        planar = True
        a = vs[face[0]]
        # exact coplanarity: det of (v1-v0, v2-v0, v3-v0) for 4-gons
        d = [[vs[face[t]][s]-a[s] for s in range(3)] for t in (1, 2, 3)]
        det = d[0][0]*(d[1][1]*d[2][2]-d[1][2]*d[2][1]) - d[0][1]*(d[1][0]*d[2][2]-d[1][2]*d[2][0]) + d[0][2]*(d[1][0]*d[2][1]-d[1][1]*d[2][0])
        print(label, "face", fi, "len", len(face), "coplanar det", float(det), "exactly planar" if det == 0 else "NOT PLANAR")
planarity("A", part)
planarity("B0", cutters[0])
planarity("B1", cutters[1])
print("==== orientation of face planes (vertex side counts)")
def orient(label, p):
    vs = [tuple(Fraction(c) for c in v) for v in api.part_vertices(p)]
    for fi, face in enumerate(api.part_faces(p)):
        a = vs[face[0]]
        for i in range(1, len(face)-1):
            u = [vs[face[i]][k]-a[k] for k in range(3)]; w = [vs[face[i+1]][k]-a[k] for k in range(3)]
            n = [u[1]*w[2]-u[2]*w[1], u[2]*w[0]-u[0]*w[2], u[0]*w[1]-u[1]*w[0]]
            if any(n): break
        off = sum(n[k]*a[k] for k in range(3)); vals = [sum(n[k]*v[k] for k in range(3))-off for v in vs]
        print(label, "face", fi, "pos", sum(1 for x in vals if x > 0), "neg", sum(1 for x in vals if x < 0), "zero", sum(1 for x in vals if x == 0), "max|n|", float(max(abs(c) for c in n)), "tri index", i)
orient("A", part); orient("B0", cutters[0]); orient("B1", cutters[1])
vs = [tuple(Fraction(c) for c in v) for v in api.part_vertices(cutters[1])]
print("B1 vertex offsets from first (1e-18 units)")
for v in vs: print("   ", [float((v[k]-vs[0][k])*10**18) for k in range(3)])
print("faces", api.part_faces(cutters[1]))
