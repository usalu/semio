def _contains_ring(inner,outer,check):
    """🏷️ Certify every exact hole-boundary interval against a candidate containing outer."""
    if any(_location(point,outer)<0 for point in inner):
        return False
    for a,b in zip(inner,inner[1:]+inner[:1]):
        check();points={a,b};direction=_sub(b,a)
        for c,d in zip(outer,outer[1:]+outer[:1]):
            check();second=_sub(d,c);delta=_sub(c,a);denominator=_cross(direction,second)
            if denominator:
                first=_cross(delta,second)/denominator;following=_cross(delta,direction)/denominator
                if 0<=first<=1 and 0<=following<=1:
                    points.add((a[0]+first*direction[0],a[1]+first*direction[1]))
            elif _cross(delta,direction)==0:
                points.update(point for point in (c,d) if all(min(a[i],b[i])<=point[i]<=max(a[i],b[i]) for i in (0,1)))
        ordered=sorted(points)
        if any(_location(((p[0]+q[0])/2,(p[1]+q[1])/2),outer)<0 for p,q in zip(ordered,ordered[1:])):
            return False
    return True

