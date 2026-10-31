def _wall_owned_parts(model: dict, parts: list, excluded_owners: list | tuple = (), native_interfaces: list | tuple = ()) -> tuple:
    """🧩 Keep brick and plaster outside other members' occupied material."""
    if not parts:
        return [],0.,0.
    boxes = [_material_bounds(part) for part in parts]
    box = tuple((min(p[k][0] for p in boxes),max(p[k][1] for p in boxes)) for k in range(3))
    cutters = []
    for cutter in _wall_cutters(model):
        if cutter.get("owner") in excluded_owners and cutter.get("role")!="column_bore_air_reservation":continue
        other = cutter.get("_bounds") or _material_bounds(cutter)
        if all(a[0]<b[1] and b[0]<a[1] for a,b in zip(box,other)):cutters.append(cutter)
    extra = [c for c in native_interfaces if all(a[0]<b[1] and b[0]<a[1] for a,b in zip(box,_material_bounds(c)))]
    backend = _surface_finishes()
    key = (hashlib.sha256(json.dumps([parts,cutters,extra],sort_keys=True,separators=(",",":"),default=_native_cache_scalar).encode()).hexdigest(),_subtract_prisms,_subtract_native_material,_subtract_convex_material,_native_material_cells,_material_bounds,_polyhedron_part,part_vertices,part_faces,part_outline_at,_polyhedron_volume,backend.clip_stock,backend.publish_stock,_native_public_value)
    if key not in _WALL_OWNERSHIP:
        native = [c for c in cutters if any(key in c for key in ("mesh","section","top_outline"))]
        vertical = [c for c in cutters if not any(key in c for key in ("mesh","section","top_outline"))]
        prisms = [p for p in parts if not any(key in p for key in ("mesh","section","top_outline"))]
        shaped = [p for p in parts if any(key in p for key in ("mesh","section","top_outline"))]
        owned = []
        if prisms:
            stocks = _subtract_prisms(prisms,vertical)
            owned,_ = _subtract_native_material(stocks,native+extra)
        if shaped:
            resolved,_ = _subtract_native_material(shaped,cutters+extra)
            owned += resolved
        volume = sum(_polyhedron_volume(p) for p in owned)
        before = sum(_polyhedron_volume(p) for p in parts)
        _WALL_OWNERSHIP[key] = owned,max(0.,before-volume),volume
        if len(_WALL_OWNERSHIP) > 256:
            _WALL_OWNERSHIP.pop(next(iter(_WALL_OWNERSHIP)))
    owned,displaced,volume = _WALL_OWNERSHIP[key]
    return deepcopy(owned),displaced,volume


def _interface_owner_parts(model: dict, key: str) -> list:
    """🔗 Resolve exclusive native interface material without aggregating aliases or querying the whole building."""
    owner = _find(model,key)
    if owner["class"]=="IfcRampFlight":
        parts = _ramp_geometry(model,owner)["parts"]
    elif owner["class"]=="IfcColumn":
        parts = column_geometry(model,owner)["parts"]
    elif owner["class"]=="IfcSlab":
        parts = _slab_geometry_entry(model,owner)["parts"]
    elif owner["class"]!="IfcWall" and "parts" in owner["geometry"]:
        parts = _solid_geometry(model,owner,resolve_embedded=False)["parts"]
    else:
        raise ValueError("An interface owner requires independently owned native material")
    return deepcopy(parts)


def _material_bounds(part: dict) -> tuple:
    """📦 Bound actual native vertices without replacing occupied material by its envelope."""
    vertices = part_vertices(part) if any(key in part for key in ("mesh","section","top_outline")) else [[*p,z] for z in (part["z0"],part["z1"]) for p in part["outline"]]
    return tuple((min(p[k] for p in vertices),max(p[k] for p in vertices)) for k in range(3))


def _native_material_cells(part: dict) -> list:
    """🧊 Resolve real convex material cells with exact profile predicates and no coordinate snapping."""
    metadata = {k:deepcopy(v) for k,v in part.items() if k not in ("outline","top_outline","holes","z0","z1","mesh","section")}
    if any(key in part for key in ("mesh","section","top_outline")):
        return [{**metadata,**_polyhedron_part(part_vertices(part),part_faces(part))}]
    from fractions import Fraction
    def side(a,b,c):
        return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
    def clean(points):
        ring = []
        for point in points:
            if not ring or point!=ring[-1]:ring.append(point)
        if len(ring)>1 and ring[0]==ring[-1]:ring.pop()
        while len(ring)>2:
            index = next((i for i,p in enumerate(ring) if side(ring[i-1],p,ring[(i+1)%len(ring)])==0),None)
            if index is None:break
            ring.pop(index)
        return ring
    rings = [clean([tuple(Fraction(v) for v in p) for p in ring]) for ring in [part["outline"],*part.get("holes",())]]
    def area(ring):
        return sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))
    if any(len(ring)<3 or area(ring)==0 for ring in rings):
        raise ValueError("Native material profiles require finite occupied rings")
    outer = rings[0] if area(rings[0])>0 else list(reversed(rings[0]))
    if len(rings)==1 and all(side(outer[i-1],p,outer[(i+1)%len(outer)])>0 for i,p in enumerate(outer)):
        profiles = [outer]
    else:
        stations = sorted({p[0] for ring in rings for p in ring})
        edges = [(a,b) for ring in rings for a,b in zip(ring,ring[1:]+ring[:1])]
        def at(edge,x):
            a,b = edge
            return a[1]+(b[1]-a[1])*(x-a[0])/(b[0]-a[0])
        profiles = []
        for lo,hi in zip(stations,stations[1:]):
            middle = (lo+hi)/2
            active = sorted((edge for edge in edges if min(edge[0][0],edge[1][0])<middle<max(edge[0][0],edge[1][0])),key=lambda edge:at(edge,middle))
            if len(active)%2:raise ValueError("Native material profile boundaries must form closed occupied intervals")
            for lower,upper in zip(active[::2],active[1::2]):
                ring = clean([(lo,at(lower,lo)),(hi,at(lower,hi)),(hi,at(upper,hi)),(lo,at(upper,lo))])
                if len(ring)>=3 and area(ring)!=0:profiles.append(ring if area(ring)>0 else list(reversed(ring)))
    result = []
    for ring in profiles:
        cell = {"outline":[list(p) for p in ring],"z0":part["z0"],"z1":part["z1"]}
        result.append({**metadata,**_polyhedron_part(part_vertices(cell),part_faces(cell))})
    return result


def _native_cache_scalar(value):
    """🔐 Encode exact private coordinate scalars losslessly without casting before geometry operations."""
    from fractions import Fraction
    if isinstance(value,Fraction):return {"$rational":[str(value.numerator),str(value.denominator)]}
    raise TypeError(f"Unsupported native cache scalar: {type(value).__name__}")


def _native_public_value(value):
    """📤 Publish isolated Float64 JSON values only after every operation on the private coordinates."""
    from fractions import Fraction
    if isinstance(value,Fraction):return float(value)
    if isinstance(value,dict):return {key:_native_public_value(item) for key,item in value.items()}
    if isinstance(value,list):return [_native_public_value(item) for item in value]
    if isinstance(value,tuple):return tuple(_native_public_value(item) for item in value)
    return deepcopy(value)


def _subtract_native_material(parts: list, cutters: list) -> tuple:
    """🧩 Keep exclusive material outside exact native stock planes, preserving layer and face identity."""
    if not parts or not cutters:
        return _native_public_value(parts),0.
    backend = _surface_finishes()
    key = (hashlib.sha256(json.dumps([parts,cutters],sort_keys=True,separators=(",",":"),default=_native_cache_scalar).encode()).hexdigest(),_subtract_convex_material,_native_material_cells,_material_bounds,_polyhedron_part,part_vertices,part_faces,part_outline_at,_polyhedron_volume,backend.clip_stock,backend.publish_stock,_native_public_value)
    if key not in _WALL_INTERFACES:
        indexed = [(_material_bounds(c),c) for c in cutters]
        normalized = {}
        result = []
        for part in parts:
            box = _material_bounds(part)
            active = [i for i,(other,c) in enumerate(indexed) if all(a[0]<b[1] and b[0]<a[1] for a,b in zip(box,other))]
            if not active:
                result.append(deepcopy(part))
                continue
            for i in active:
                if i not in normalized:normalized[i] = _native_material_cells(indexed[i][1])
            for cell in _native_material_cells(part):
                stock = {**cell,"material":part["material"],"role":part.get("role","owned_material")}
                result += _subtract_convex_material(stock,[c for i in active for c in normalized[i]])
        def volume(items):
            return sum(_polyhedron_volume(p) for p in items)
        _WALL_INTERFACES[key] = _native_public_value(result),max(0.,volume(parts)-volume(result))
        if len(_WALL_INTERFACES)>128:
            _WALL_INTERFACES.pop(next(iter(_WALL_INTERFACES)))
    return deepcopy(_WALL_INTERFACES[key])


def _wall_interface_cutters(model: dict, wall: dict) -> list:
    """🛡️ Resolve independent source-authoritative material before constructing a wall stock."""
    refs = wall["geometry"].get("interface_owners",())
    if len(set(refs))!=len(refs) or wall["tag"] in refs:
        raise ValueError("Wall interface owners must be distinct independent products")
    return [part for tag in refs for part in _interface_owner_parts(model,tag)]


def _wall_interface_parts(model: dict, wall: dict, parts: list) -> tuple:
    """🧱 Reserve additional source-authoritative native material in an inferred wall."""
    return _subtract_native_material(parts,_wall_interface_cutters(model,wall))


