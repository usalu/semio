def hollow_polyline(points: list, diameter: float, wall_thickness: float, segments: int = 24, exact: bool = False) -> dict:
    """🪈️ Construct hollow faceted tube stocks with shared bisector miters and transported circle generators."""
    from fractions import Fraction
    def q(value):
        return value if isinstance(value,Fraction) else Fraction(str(value))
    if len(points)<2 or segments<8 or segments%4 or not 0<wall_thickness<diameter/2:
        raise ValueError('Hollow tube construction requires a finite path, positive gauge and fourfold angular sampling')
    centers = [[q(v) for v in point] for point in points]
    if any(len(point)!=3 for point in centers):raise ValueError('Tube stations require three coordinates')
    def dot(a,b):return sum(x*y for x,y in zip(a,b))
    def cross(a,b):return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
    def unit(vector):
        length = math.sqrt(float(dot(vector,vector)))
        if not math.isfinite(length) or length==0:raise ValueError('Tube axes and miter normals must be finite and nonzero')
        return [v/q(length) for v in vector]
    axes = [[b[k]-a[k] for k in range(3)] for a,b in zip(centers,centers[1:])]
    directions = [unit(axis) for axis in axes]
    normals = [directions[0],*[unit([a[k]+b[k] for k in range(3)]) for a,b in zip(directions,directions[1:])],directions[-1]]
    basis = [Fraction(int(k==min(range(3),key=lambda j:abs(directions[0][j])))) for k in range(3)]
    axis = directions[0];projection = dot(basis,axis)/dot(axis,axis)
    first = unit([basis[k]-axis[k]*projection for k in range(3)]);second = unit(cross(axis,first));radius = q(diameter)/2
    def angle(index):
        quarter = segments//4;quadrant,step = divmod(index,quarter);swap = 2*step>quarter
        reduced = quarter-step if swap else step
        if reduced==0:c,s = Fraction(1),Fraction(0)
        elif 2*reduced==quarter:c = s = q(math.sqrt(.5))
        else:c,s = q(math.cos(2*math.pi*reduced/segments)),q(math.sin(2*math.pi*reduced/segments))
        if swap:c,s = s,c
        return ((c,s),(-s,c),(-c,-s),(s,-c))[quadrant]
    ring = [[centers[0][k]+radius*(first[k]*c+second[k]*s) for k in range(3)] for c,s in map(angle,range(segments))]
    rings = [ring]
    for axis,center,normal in zip(axes,centers[1:],normals[1:]):
        denominator = dot(axis,normal)
        if denominator<=0:raise ValueError('A tube miter must advance along its segment')
        ring = [[p[k]+axis[k]*dot(normal,[center[j]-p[j] for j in range(3)])/denominator for k in range(3)] for p in ring]
        rings.append(ring)
    ratio = (radius-q(wall_thickness))/radius
    inner = [[[c[k]+ratio*(p[k]-c[k]) for k in range(3)] for p in ring] for c,ring in zip(centers,rings)]
    def mesh(a,b):
        n = len(a)
        return {'mesh':{'vertices':a+b,'faces':[list(reversed(range(n))),list(range(n,2*n))]+[[i,(i+1)%n,n+(i+1)%n,n+i] for i in range(n)]}}
    outer = [mesh(a,b) for a,b in zip(rings,rings[1:])];core = [mesh(a,b) for a,b in zip(inner,inner[1:])];metal = []
    for a,b,c,d in zip(rings,rings[1:],inner,inner[1:]):
        for i in range(segments):
            j = (i+1)%segments
            metal.append(mesh([a[i],a[j],c[j],c[i]],[b[i],b[j],d[j],d[i]]))
    caps = [[{'normal':[-v for v in normals[i]],'offset':-dot(normals[i],centers[i]),'point':centers[i]},{'normal':normals[i+1],'offset':dot(normals[i+1],centers[i+1]),'point':centers[i+1]}]for i in range(len(axes))]
    result = {'outer':outer,'core':core,'metal':metal,'rings':rings,'cap_planes':caps}
    def publish(value):
        if isinstance(value,Fraction):return float(value)
        if isinstance(value,list):return [publish(v) for v in value]
        if isinstance(value,dict):return {k:publish(v) for k,v in value.items()}
        return value
    return result if exact else publish(result)
