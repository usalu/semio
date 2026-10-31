def tube_routes(routes: list, exact: bool = False) -> list:
    """🧭 Resolve declared terminal anchors on native route axes without rounding affine intersections."""
    from fractions import Fraction
    from copy import deepcopy
    records = {r['id']:r for r in routes};resolved = {};visiting = set()
    if len(records)!=len(routes):raise ValueError('Tube route identifiers must be unique')
    def resolve(key):
        if key in resolved:return resolved[key]
        if key in visiting or key not in records:raise ValueError('Tube anchors require an acyclic existing host route')
        visiting.add(key);record = records[key];points = [[v if isinstance(v,Fraction)else Fraction(str(v))for v in p]for p in record['points']]
        if len(points)<2 or any(len(p)!=3 for p in points):raise ValueError('Tube route stations require three coordinates')
        if 'end_anchor'in record:
            anchor = record['end_anchor'];host = resolve(anchor['route']);axis = 'XYZ'.index(anchor['axis'])
            if len(host)!=2:raise ValueError('A tube terminal anchor requires one straight host segment')
            delta = [host[1][k]-host[0][k]for k in range(3)];fixed = [k for k in range(3)if k!=axis]
            station = max(fixed,key=lambda k:abs(delta[k]))
            if delta[station]==0:raise ValueError('A tube terminal cannot locate its host along the constrained axis')
            parameter = (points[-1][station]-host[0][station])/delta[station]
            if not 0<=parameter<=1 or any(points[-1][k]!=host[0][k]+parameter*delta[k]for k in fixed):raise ValueError('A tube terminal must lie on its finite host route')
            points[-1][axis] = host[0][axis]+parameter*delta[axis]
        visiting.remove(key);resolved[key] = points
        return points
    return [{**deepcopy(record),'points':resolve(record['id']) if exact else [[float(v)for v in p]for p in resolve(record['id'])]}for record in routes]
