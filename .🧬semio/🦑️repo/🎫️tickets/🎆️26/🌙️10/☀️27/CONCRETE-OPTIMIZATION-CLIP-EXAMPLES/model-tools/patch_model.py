import sys
from pathlib import Path
p = Path(sys.argv[1])
s = p.read_text(encoding="utf-8")
a = s.index("def _extremum")
b = s.index("def orientation_study():")
new = '''@_memo
def _orientation_study():
    """🧭 Scan the bearing of the glazed façade on a 1° grid for optima, the courtyard constraint and a greedy descent."""
    study, values = _frozen()
    settings = study["orientation"]
    low, high = settings["range"]
    design = {**settings["design"]}
    energy = lambda angle: _evaluate(float(angle), design["wwr"], design["overhang"], design["insulation"])["electricity"]
    angles = list(range(low, high + 1))
    scan = [energy(angle) for angle in angles]
    minima, maxima = [], []
    for index in range(1, len(angles) - 1):
        for sign, found in ((1, minima), (-1, maxima)):
            if sign * scan[index] < sign * scan[index - 1] and sign * scan[index] <= sign * scan[index + 1]:
                found.append(angles[index])
    constraint = settings["constraint"]
    candidates = lambda bounds: [angle for angle in (*minima, *bounds) if bounds[0] <= angle <= bounds[1]]
    reference, tariff, step = energy(settings["reference"]), values["prices"]["tariff"], settings["neighbour_step"]
    current = settings["start"]
    descent = []
    while True:
        neighbours = [{"theta": angle, "electricity": energy(angle)} for angle in (current - step, current + step) if low <= angle <= high]
        descent.append({"theta": current, "electricity": energy(current), "neighbours": neighbours})
        best = min(neighbours, key=lambda item: item["electricity"])
        if best["electricity"] >= energy(current):
            break
        current = best["theta"]
    return {
        "design": design, "range": [low, high],
        "local_minima": minima, "local_minima_electricity": [energy(angle) for angle in minima],
        "local_maxima": maxima, "local_maxima_electricity": [energy(angle) for angle in maxima],
        "global_minimum": min(candidates([low, high]), key=energy), "constraint": constraint,
        "feasible_minimum": min(candidates(constraint), key=energy),
        "reference": settings["reference"], "reference_electricity": reference, "tariff": tariff,
        "samples": [[angle, value] for angle, value in zip(angles, scan, strict=True)],
        "savings": [[angle, reference - value, (reference - value) * tariff] for angle, value in zip(angles, scan, strict=True)],
        "start": settings["start"], "neighbour_step": step, "descent": descent, "sun": settings["sun"],
    }


'''
s = s[:a] + new + s[b:]
s = s.replace('''    "daylight": factor, "autonomy": autonomy,''', '''    "sun_winter": _irradiance("winter", phi) / _peak("winter"), "sun_summer": _irradiance("summer", phi) / _peak("summer"),
        "daylight": factor, "autonomy": autonomy,''')
s = s.replace('''def _shading(''', '''@functools.lru_cache(maxsize=None)
def _peak(season):
    """🌞 Strongest vertical irradiance of the season over all bearings."""
    return max(_irradiance(season, math.radians(angle)) for angle in range(360))


def _shading(''', 1)
s = s.replace('''    study, _ = _frozen()
    return _surface(n)''', '''    return _surface(n)''')
p.write_text(s, encoding="utf-8")
