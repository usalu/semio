def _local_primary_reference(raw,owned,primary):
    """🧮 Apply exact set identities to bbox-local primary candidates without a global primary union."""
    from shapely.ops import unary_union
    from shapely.strtree import STRtree
    assert all(p.is_valid and p.area>0 for p in raw+owned+primary)
    tree=STRtree(primary);remainders=[];common=[]
    for polygon in raw:
        remainder=polygon
        for index in sorted(int(i) for i in tree.query(polygon)):
            remainder=remainder.difference(primary[index])
        remainders.append(remainder)
    for polygon in owned:
        for index in sorted(int(i) for i in tree.query(polygon)):
            intersection=polygon.intersection(primary[index])
            if intersection.area>0:common.append(intersection)
    return unary_union(remainders),unary_union(common).area


