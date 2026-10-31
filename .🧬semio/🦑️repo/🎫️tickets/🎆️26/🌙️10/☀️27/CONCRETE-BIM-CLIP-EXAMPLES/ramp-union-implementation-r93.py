"""🧩 Exact planar unions of finite authored polygon footprints without runtime dependencies."""
from collections import OrderedDict,defaultdict
from copy import deepcopy
from fractions import Fraction
import math

_CACHE=OrderedDict()

class UnionCancelled(Exception):
    """🛑 A caller explicitly cancelled planar boundary preparation."""

def _cross(a,b):
    """📐 Evaluate an exact planar determinant."""
    return a[0]*b[1]-a[1]*b[0]

def _sub(a,b):
    """↔️ Subtract planar coordinates without a floating-point boundary."""
    return a[0]-b[0],a[1]-b[1]

def _area(ring):
    """📏 Compute twice a ring's oriented exact area."""
    return sum(_cross(a,b) for a,b in zip(ring,ring[1:]+ring[:1]))

def _location(point,ring):
    """📍 Return interior, boundary or exterior using exact winding predicates."""
    winding=0
    for a,b in zip(ring,ring[1:]+ring[:1]):
        turn=_cross(_sub(b,a),_sub(point,a))
        if turn==0 and all(min(a[i],b[i])<=point[i]<=max(a[i],b[i]) for i in (0,1)):
            return 0
        if a[1]<=point[1]<b[1] and turn>0:
            winding+=1
        elif b[1]<=point[1]<a[1] and turn<0:
            winding-=1
    return 1 if winding else -1

def _arrangement(polygons,notify,check):
    """🧵 Node every crossing and T junction, retaining only union-facing exact edges."""
    rings=[]
    for ring in polygons:
        check()
        points=[]
        for point in ring:
            if not points or point!=points[-1]:
                points.append(point)
        if points and points[0]==points[-1]:
            points.pop()
        if len(points)<3 or _area(points)==0:
            raise ValueError('Union operands require nonzero simple polygon rings')
        if _area(points)<0:
            points.reverse()
        rings.append(tuple(points))
    edges=sorted({tuple(sorted((a,b))) for ring in rings for a,b in zip(ring,ring[1:]+ring[:1]) if a!=b})
    bounds=[(min(a[0],b[0]),max(a[0],b[0]),min(a[1],b[1]),max(a[1],b[1])) for a,b in edges]
    nodes=[{a,b} for a,b in edges]
    order=sorted(range(len(edges)),key=lambda i:bounds[i][0])
    for offset,i in enumerate(order):
        check();a,b=edges[i];direction=_sub(b,a);box=bounds[i]
        for j in order[offset+1:]:
            other=bounds[j]
            if other[0]>box[1]:
                break
            if other[2]>box[3] or other[3]<box[2]:
                continue
            check();c,d=edges[j];second=_sub(d,c);delta=_sub(c,a);denominator=_cross(direction,second)
            if denominator:
                first=_cross(delta,second)/denominator;following=_cross(delta,direction)/denominator
                if 0<=first<=1 and 0<=following<=1:
                    point=(a[0]+first*direction[0],a[1]+first*direction[1]);nodes[i].add(point);nodes[j].add(point)
            elif _cross(delta,direction)==0:
                for point in (a,b,c,d):
                    if all(max(box[2*k],other[2*k])<=point[k]<=min(box[2*k+1],other[2*k+1]) for k in (0,1)):
                        nodes[i].add(point);nodes[j].add(point)
        notify('node',offset+1,len(edges))
    atoms=set()
    for points in nodes:
        ordered=sorted(points)
        atoms.update(zip(ordered,ordered[1:]))
    ring_bounds=[(min(p[0] for p in ring),max(p[0] for p in ring),min(p[1] for p in ring),max(p[1] for p in ring)) for ring in rings]
    boundary=set()
    for index,(a,b) in enumerate(sorted(atoms)):
        check();mid=((a[0]+b[0])/2,(a[1]+b[1])/2);left=right=False
        for ring,box in zip(rings,ring_bounds):
            if not(box[0]<=mid[0]<=box[1] and box[2]<=mid[1]<=box[3]):
                continue
            location=_location(mid,ring)
            if location>0:
                left=right=True;break
            if location==0:
                for c,d in zip(ring,ring[1:]+ring[:1]):
                    if _cross(_sub(d,c),_sub(mid,c))==0 and all(min(c[k],d[k])<=mid[k]<=max(c[k],d[k]) for k in (0,1)):
                        if sum(x*y for x,y in zip(_sub(b,a),_sub(d,c)))>0:
                            left=True
                        else:
                            right=True
            if left and right:
                break
        if left!=right:
            boundary.add((a,b) if left else (b,a))
        notify('classify',index+1,len(atoms))
    outgoing=defaultdict(set)
    for a,b in boundary:
        outgoing[a].add(b)
    loops=[]
    while boundary:
        check();a,b=min(boundary);start=a;ring=[a]
        while True:
            boundary.remove((a,b));outgoing[a].remove(b);ring.append(b)
            if b==start:
                break
            candidates=outgoing[b]
            if not candidates:
                raise ValueError('Union arrangement has an open material boundary')
            direction=_sub(b,a)
            def turn(point):
                """🧭 Keep point-touching material components on their own oriented boundary."""
                vector=_sub(point,b)
                return math.atan2(float(_cross(direction,vector)),float(sum(x*y for x,y in zip(direction,vector))))
            a,b=b,max(candidates,key=turn)
        ring.pop()
        changed=True
        while changed and len(ring)>3:
            changed=False
            for i,point in enumerate(ring):
                if _cross(_sub(point,ring[i-1]),_sub(ring[(i+1)%len(ring)],point))==0:
                    ring.pop(i);changed=True;break
        loops.append(ring)
    outers=[ring for ring in loops if _area(ring)>0];holes=[ring for ring in loops if _area(ring)<0]
    result=[{'outer':ring,'holes':[]} for ring in outers]
    for hole in holes:
        owners=[row for row in result if _location(hole[0],row['outer'])>=0]
        if not owners:
            raise ValueError('A union hole must have a containing material boundary')
        min(owners,key=lambda row:_area(row['outer']))['holes'].append(hole)
    return result

def polygon_union(polygons,progress=None,cancelled=None):
    """🗺️ Return isolated connected outer/hole regions with bounded producer-aware reuse."""
    def check():
        """🛑 Observe cancellation before cache access and each bounded arrangement unit."""
        if cancelled and cancelled():
            raise UnionCancelled('Planar union cancelled')
    def notify(phase,completed,total):
        """📣 Report phase-local union work and observe caller cancellation."""
        if progress:
            progress({'phase':phase,'completed':completed,'total':total})
        check()
    check();prepared=[]
    for ring in polygons:
        points=[]
        for point in ring:
            if len(point)!=2 or any(not math.isfinite(value) for value in point):
                raise ValueError('Union coordinates must be finite planar points')
            points.append(tuple(value if isinstance(value,Fraction) else Fraction(value) for value in point))
        prepared.append(tuple(points))
    inputs=tuple(prepared);key=(inputs,_arrangement,_location,_area,_cross,_sub)
    if key in _CACHE:
        _CACHE.move_to_end(key);notify('complete',len(inputs),len(inputs));return deepcopy(_CACHE[key])
    result=_arrangement(inputs,notify,check)
    public=[{'outer':[[float(x),float(y)] for x,y in row['outer']],'holes':[[[float(x),float(y)] for x,y in ring] for ring in row['holes']]} for row in result]
    _CACHE[key]=deepcopy(public)
    while len(_CACHE)>32:
        _CACHE.popitem(last=False)
    notify('complete',len(inputs),len(inputs))
    return public
