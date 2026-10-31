def _ramp_stringer_reference_stock(model: dict, obj: dict) -> dict:
    """📐 Preserve the attested stringer section on its original source-affine flight soffit."""
    g = obj['geometry']['interface_construction']
    if g['kind']!='ramp_stringer':raise ValueError('A stringer interface requires its native ramp construction')
    q = _source_fraction
    flight = _find(model,g['flight'])
    if flight['class']!='IfcRampFlight':raise ValueError('A stringer requires a native flight')
    stock = _ramp_reference_stock(model,flight);vertices = stock['mesh']['vertices']
    ends = sorted({p[1]:p[2] for p in vertices[:4]}.items());(y0,r0),(y1,r1) = ends
    width,depth = q(g['width']),q(g['depth'])
    if g['side']=='right':x0 = max(p[0] for p in vertices);x1 = x0+width
    else:x1 = min(p[0] for p in vertices);x0 = x1-width
    return {'mesh':{'vertices':[[x0,y0,r0-depth],[x1,y0,r0-depth],[x1,y1,r1-depth],[x0,y1,r1-depth],[x0,y0,r0],[x1,y0,r0],[x1,y1,r1],[x0,y1,r1]],'faces':[[3,2,1,0],[4,5,6,7]]+[[i,(i+1)%4,4+(i+1)%4,4+i]for i in range(4)]},'material':_material(model,obj)['tag'],'role':'original_ramp_stringer'}


