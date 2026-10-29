def _wall_path(model: dict, wall: dict) -> list:
    """🔁 Preserve explicit native wall closure without changing polygon outline normalization."""
    raw = wall["geometry"]["path"]
    path = _expand(model,raw)
    endpoints = []
    for item,last in ((raw[0],False),(raw[-1],True)):
        if isinstance(item,list):
            point = item
        elif "bezier" in item:
            point = item["bezier"][-1 if last else 0]
        else:
            arc = item["arc"]
            angle = math.radians(arc["end" if last else "start"])
            point = [arc["centre"][0]+arc["radius"]*math.cos(angle),arc["centre"][1]+arc["radius"]*math.sin(angle)]
        endpoints.append(point)
    if len(path)>2 and math.dist(*endpoints)<1e-10 and math.dist(path[0],path[-1])>1e-10:
        path.append(list(path[0]))
    return path


