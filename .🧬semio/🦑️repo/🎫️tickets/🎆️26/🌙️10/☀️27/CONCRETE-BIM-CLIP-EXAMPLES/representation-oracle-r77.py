def _vertices(primitive):
    """📍 Read the explicit native three-dimensional representation vertices."""
    if primitive['kind']=='polyhedron':return primitive['vertices']
    if primitive['kind']=='loft':return [[x,y,primitive['z0']] for x,y in primitive['bottom']]+[[x,y,primitive['z1']] for x,y in primitive['top']]
    assert primitive['kind']=='prism'
    return [[x,y,primitive[z]] for z in ('z0','z1') for x,y in primitive['polygon']]


def _convex(primitive):
    """💎 Certify native mesh faces against an independent SciPy convex hull."""
    import numpy as np
    from scipy.spatial import ConvexHull
    vertices=np.asarray(_vertices(primitive),dtype=float);hull=ConvexHull(vertices)
    if primitive['kind']=='polyhedron':
        for face in primitive['faces']:
            points=vertices[face];normal=np.cross(points[1]-points[0],points[2]-points[0]);length=np.linalg.norm(normal)
            assert length>1e-12
            distances=(vertices-points[0])@normal/length
            assert np.max(np.abs((points-points[0])@normal/length))<1e-8
            assert distances.min()>-1e-8 or distances.max()<1e-8
    return vertices,hull


def _slice(primitive,axis,value,convex=None):
    """✂️ Intersect actual occupied material with a coordinate plane independently."""
    from shapely.geometry import LineString,MultiPoint,Polygon,box
    from shapely.ops import unary_union
    if primitive['kind']=='prism':
        shape=Polygon(primitive['polygon'],primitive.get('holes',[]))
        if axis==2:return shape if primitive['z0']<=value<=primitive['z1'] else Polygon()
        assert axis==1
        lines=shape.intersection(LineString([(-100,value),(100,value)]))
        return unary_union([box(line.bounds[0],primitive['z0'],line.bounds[2],primitive['z1']) for line in getattr(lines,'geoms',[lines]) if line.geom_type=='LineString' and line.length>1e-10])
    vertices,hull=convex if convex is not None else _convex(primitive);remaining=[i for i in range(3) if i!=axis];points=[]
    for simplex in hull.simplices:
        for a,b in zip(simplex,list(simplex[1:])+[simplex[0]]):
            p,q=vertices[a],vertices[b];d,e=p[axis]-value,q[axis]-value
            if abs(d)<1e-12:points.append(p[remaining])
            if d*e<0:points.append((p+(q-p)*(-d)/(e-d))[remaining])
    shape=MultiPoint(points).convex_hull if points else Polygon()
    return shape if shape.geom_type=='Polygon' else Polygon()


def _projection(primitive):
    """🌄 Project every occupied native part onto the south XZ plane."""
    from shapely.geometry import MultiPoint,box
    if primitive['kind']=='prism':
        return box(min(p[0] for p in primitive['polygon']),primitive['z0'],max(p[0] for p in primitive['polygon']),primitive['z1'])
    vertices,_=_convex(primitive)
    return MultiPoint(vertices[:,[0,2]]).convex_hull


def _volume(primitives):
    """⚖️ Integrate independent GEOS slice unions with SciPy quadrature."""
    from scipy.integrate import quad
    from shapely.ops import unary_union
    compiled=[(p,min(v[2] for v in _vertices(p)),max(v[2] for v in _vertices(p)),None if p['kind']=='prism' else _convex(p)) for p in primitives]
    levels=sorted({v[2] for p in primitives for v in _vertices(p)})
    def area(z):return unary_union([_slice(p,2,z,convex) for p,lo,hi,convex in compiled if lo<z<hi]).area
    estimates=[quad(area,lo,hi,epsabs=1e-10,epsrel=1e-10,limit=200) for lo,hi in zip(levels,levels[1:])]
    assert sum(error for _,error in estimates)<1e-8
    return math.fsum(value for value,_ in estimates)


