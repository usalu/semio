import sys
from pathlib import Path

path = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
text = path.read_bytes().decode("utf-8")
crlf = "\r\n" in text
text = text.replace("\r\n", "\n")


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:60]
    text = text.replace(old, new)


swap('''@functools.lru_cache(maxsize=None)
def _sun_samples(orientation, season):
    """🕘 Hourly sun positions of the solstice that shine on the façade, with their share of the beam (cos of incidence)."""
''', '''@functools.lru_cache(maxsize=None)
def _beam_samples(orientation, season):
    """🔦 Hourly solstice sun positions in front of the façade plane (angle across the normal below 90°), each with its beam weight cos(altitude) × cos(incidence), which is 0 at 90°."""
''')
swap('''        if altitude >= sun["minimum_altitude"] and across < 89:
            profile''', '''        if altitude >= sun["minimum_altitude"] and across < 90:
            profile''')
swap('''    return tuple(samples)


def sun_samples(''', '''    return tuple(samples)


@functools.lru_cache(maxsize=None)
def _sun_samples(orientation, season):
    """🕘 Hourly sun positions of the solstice that shine on the façade (angle across the normal below 89°), with their share of the beam (cos of incidence)."""
    return tuple(item for item in _beam_samples(orientation, season) if item["across"] < 89)


def sun_samples(''')
swap('''def _shading(season, orientation, overhang, window_height, gap):
    """⛱️ Beam-weighted share of the window height in the overhang's shadow: depth × tan(profile angle) minus the wall gap above the window."""
    samples = _sun_samples(orientation, season)
    if overhang <= 0 or not samples:
        return 0.0
    shaded = (min(1.0, max(0.0, (overhang * math.tan(math.radians(item["profile"])) - gap) / window_height)) for item in samples)
    return sum(item["weight"] * share for item, share in zip(samples, shaded, strict=True)) / sum(item["weight"] for item in samples)
''', '''def _shading(season, orientation, overhang, window_height, gap):
    """⛱️ Shaded beam and delivered beam: the shaded share of each hour's window height (depth × tan(profile angle) minus the wall gap above the window) weighted by the beam it delivers, over the larger of the delivered beam and the climate beam reference."""
    samples = _beam_samples(orientation, season)
    _, values = _frozen()
    beam = sum(item["weight"] for item in samples)
    shaded = 0.0 if overhang <= 0 else sum(item["weight"] * min(1.0, max(0.0, (overhang * math.tan(math.radians(item["profile"])) - gap) / window_height)) for item in samples)
    return shaded / max(beam, values["climate"]["beam_reference"]), beam
''')
swap('''    shading = {season: _shading(season, orientation, overhang, window_height, gap) for season in ("winter", "summer")}
''', '''    shaded = {season: _shading(season, orientation, overhang, window_height, gap) for season in ("winter", "summer")}
    shading, beam = ({season: shaded[season][index] for season in shaded} for index in (0, 1))
''')
swap('''    direct = {season: climate[f"direct_share_{season}"] for season in shading}
''', '''    direct = {season: climate[f"direct_share_{season}"] for season in shading}
    reach = {season: direct[season] * min(1.0, beam[season] / climate["beam_reference"]) for season in shading}
''')
swap('''    return {
        "window_area": window,''', '''    return {
        "conductance": conductance,
        "heating_losses": losses, "heating_gains": gains, "heating_eta": heating_eta, "heating_used": heating_eta * gains, "cop": systems["cop"],
        "cooling_gains": heat, "cooling_removable": removable, "cooling_eta": cooling_eta, "cooling_removed": cooling_eta * removable, "eer": systems["eer"],
        "internal_winter": climate["internal_gains_winter"], "internal_summer": climate["internal_gains_summer"],
        "beam_winter": beam["winter"], "beam_summer": beam["summer"], "direct_winter": reach["winter"], "direct_summer": reach["summer"],
        "lighting_power": lighting["power_density"] * floor / 1000, "lighting_hours": lighting["hours"], "lamps_on": 1 - autonomy,
        "window_area": window,''')
swap('''    heating_demand = max(0.0, losses - _utilization(gains / losses, climate["utilization_exponent_heating"]) * gains)''',
     '''    heating_eta = _utilization(gains / losses, climate["utilization_exponent_heating"])
    heating_demand = max(0.0, losses - heating_eta * gains)''')
swap('''    cooling_demand = max(0.0, heat - _utilization(removable / heat, climate["utilization_exponent_cooling"]) * removable)''',
     '''    cooling_eta = _utilization(removable / heat, climate["utilization_exponent_cooling"])
    cooling_demand = max(0.0, heat - cooling_eta * removable)''')
if crlf:
    text = text.replace("\n", "\r\n")
path.write_bytes(text.encode("utf-8"))
print("ok", crlf)
