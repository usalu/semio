def _guard_reference_stocks(model: dict, obj: dict) -> list:
    """🛝️ Construct original guard bands from source-affine flight controls before material ownership cuts."""
    g = obj['geometry']['interface_construction']
    if g['kind']!='ramp_guard_bands':raise ValueError('A guard interface requires native ramp band construction')
    q = _source_fraction
    ramp = _find(model,g['ramp'])
    if ramp['class']!='IfcRamp':raise ValueError('A guard construction requires a native ramp')
    faces = [[3,2,1,0],[4,5,6,7]]+[[i,(i+1)%4,4+(i+1)%4,4+i] for i in range(4)]
    result = []
    def make(x0,x1,y0,y1,b0,b1,t0,t1,role):
        result.append({'mesh':{'vertices':[[x0,y0,b0],[x1,y0,b0],[x1,y1,b1],[x0,y1,b1],[x0,y0,t0],[x1,y0,t0],[x1,y1,t1],[x0,y1,t1]],'faces':deepcopy(faces)},'material':'MAT-RC','role':role})
    flights = {}
    for tag in {band['flight'] for band in g['bands']}|({g['arrival_upstand']['flight']} if 'arrival_upstand' in g else set()):
        flight = _find(model,tag)
        if flight['class']!='IfcRampFlight' or flight['geometry']['ramp']!=ramp['tag']:raise ValueError('A guard band requires a flight of its declared ramp')
        source = flight['geometry'];top = _ramp_reference_stock(model,flight)['mesh']['vertices'][4:]
        lo,hi = q(_z(model,source['from_level'])),q(_z(model,source['to_level']))
        walking = [lo+(hi-lo)*q(v) for v in source['portion']]
        by_y = {p[1]:p[2] for p in top}
        ends = sorted((q(p[1]),z,by_y[q(p[1])]) for p,z in zip(source['path'],walking))
        flights[tag] = ends
    width,height,cap,over = [q(g[key]) for key in ('core_width','core_height','cap_height','cap_overhang')]
    if 'arrival_upstand' in g:
        up = g['arrival_upstand'];(y0,_,r0),(y1,_,r1) = flights[up['flight']];x,w,h = q(up['axis_x']),q(up['width']),q(up['height'])
        make(x-w/2,x+w/2,y0,y1,r0,r1,r0+h,r1+h,'original_arrival_upstand')
    for band in g['bands']:
        (y0,w0,r0),(y1,w1,r1) = flights[band['flight']];x = q(band['axis_x'])
        make(x-width/2,x+width/2,y0,y1,r0,r1,w0+height,w1+height,'original_guard_core')
        make(x-width/2-over,x+width/2+over,y0,y1,w0+height,w1+height,w0+height+cap,w1+height+cap,'original_guard_cap')
    if g['landing_returns']:
        rg = ramp['geometry'];mid = next(p for p in rg['landings'] if p['ownership']=='ramp_component')
        walk = [[q(v) for v in p] for p in _expand(model,mid['walking_outline'])];stock = [[q(v) for v in p] for p in _expand(model,mid['outline'])]
        sx0,sx1 = min(p[0] for p in stock),max(p[0] for p in stock);wx0,wx1 = min(p[0] for p in walk),max(p[0] for p in walk)
        y0,wy1,sy1 = min(p[1] for p in walk),max(p[1] for p in walk),max(p[1] for p in stock)
        lo,hi = q(_z(model,rg['from_level'])),q(_z(model,rg['to_level']));floor = lo+(hi-lo)*q(mid['portion']);rc = floor-q(rg['finish_thickness']['value'])
        for a,b in ((sx0,wx0),(wx1,sx1)):
            make(a,b,y0,wy1,rc,rc,floor+height,floor+height,'original_landing_side_core')
            make(a-over,b+over,y0,wy1,floor+height,floor+height,floor+height+cap,floor+height+cap,'original_landing_side_cap')
        make(sx0,sx1,wy1,sy1,rc,rc,floor+height,floor+height,'original_landing_end_core')
        make(sx0-over,sx1+over,wy1-over,sy1+over,floor+height,floor+height,floor+height+cap,floor+height+cap,'original_landing_end_cap')
    return result


