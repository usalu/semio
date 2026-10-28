import re
from pathlib import Path
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
p = clip / "🧮️model/🔣️.json"
s = p.read_text(encoding="utf-8")
a = s.index('    "shading": {'); b = s.index('    "daylight": {')
s = s[:a] + s[b:]
old = '      "height": {"value": 3, "unit": "m", "note": {"en": "Room and façade height.", "de": "Raum- und Fassadenhöhe."}}\n'
assert old in s
s = s.replace(old, old.rstrip("\n") + ',\n      "head": {"value": 2.7, "unit": "m", "note": {"en": "Window head height above the floor; the overhang slab sits on top of the wall, so 0.3 m of wall stay above the window.", "de": "Höhe der Fensteroberkante über dem Boden; die Vordachplatte sitzt auf der Wandoberkante, über dem Fenster bleiben also 0,3 m Wand."}}\n')
a = s.index('  "sun": {'); b = s.index('  "baseline":')
s = s[:a] + '''  "sun": {
    "latitude": 47.4,
    "declination": 23.44,
    "minimum_altitude": 3,
    "winter_altitude": 19.16,
    "summer_altitude": 66.04,
    "low_summer_altitude": 30,
    "winter_azimuth": [130.2, 229.8],
    "summer_azimuth": [58, 302],
    "note": {"en": "Sun path on the solstices at 47.4° latitude: noon altitudes, typical low summer sun in the morning and evening, and the compass bearings between which the sun is above 3° (180° is south). The overhang is judged at the sun-path point that meets the façade best.", "de": "Sonnenbahn zu den Sonnenwenden auf 47,4° Breite: Mittagshöhen, typische tiefe Sommersonne morgens und abends und die Kompasswinkel, zwischen denen die Sonne über 3° steht (180° ist Süd). Das Vordach wird an dem Punkt der Sonnenbahn beurteilt, der zur Fassade passt."}
  },
''' + s[b:]
s = s.replace(" ° ", "° ").replace("180 ° ist", "180° ist").replace("360 ° ist", "360° ist")
p.write_text(s, encoding="utf-8", newline="\n")
p = clip / "🧮️model/🐍️.py"
s = p.read_text(encoding="utf-8")
a = s.index("def _shading("); b = s.index("@functools.lru_cache(maxsize=None)\ndef _evaluate(")
s = s[:a] + '''@functools.lru_cache(maxsize=None)
def _sun_track(season):
    """🛤️ Solstice sun path as (compass bearing, altitude) pairs while the sun stands above the minimum altitude."""
    study, _ = _frozen()
    sun = study["sun"]
    phi, delta = math.radians(sun["latitude"]), math.radians(sun["declination"] if season == "summer" else -sun["declination"])
    track = []
    for step in range(-900, 901):
        hour = math.radians(step / 5)
        altitude = math.degrees(math.asin(math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.cos(hour)))
        if altitude >= sun["minimum_altitude"]:
            bearing = (180 + math.degrees(math.atan2(math.sin(hour), math.cos(hour) * math.sin(phi) - math.tan(delta) * math.cos(phi)))) % 360
            track.append((bearing, altitude))
    return tuple(track)


def sun_state(orientation, season):
    """🌞 The sun-path point that meets the façade best: bearing, altitude, angle across the façade normal and profile angle in section."""
    bearing, altitude = min(_sun_track(season), key=lambda point: abs((point[0] - orientation + 180) % 360 - 180))
    across = abs((bearing - orientation + 180) % 360 - 180)
    front = across < 89
    profile = min(85.0, math.degrees(math.atan(math.tan(math.radians(altitude)) / max(math.cos(math.radians(across)), 1e-3)))) if front else altitude
    return {"bearing": bearing, "altitude": altitude, "profile": profile, "front": front, "across": across}


def _shading(season, orientation, overhang, window_height, gap):
    """⛱️ Share of the window height in the overhang's shadow: depth × tan(profile angle) minus the wall gap above the window."""
    state = sun_state(orientation, season)
    if overhang <= 0 or not state["front"]:
        return 0.0
    return min(1.0, max(0.0, (overhang * math.tan(math.radians(state["profile"])) - gap) / window_height))


''' + s[b:]
s = s.replace('''    window_height = wwr * room["height"]
''', '''    window_height = min(wwr * room["height"], room["head"] - 0.05)
    gap = room["height"] - room["head"]
''')
s = s.replace('_shading(season, phi, overhang, window_height) for season', '_shading(season, orientation, overhang, window_height, gap) for season')
s = s.replace("room, climate, envelope, daylight, systems, lighting, prices, carbon = (", "room, climate, envelope, daylight, systems, lighting, prices, carbon = (")
p.write_text(s, encoding="utf-8", newline="\n")
p = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/model-tools/build_schema.py")
s = p.read_text(encoding="utf-8")
s = s.replace('"room": ["width", "depth", "height"],', '"room": ["width", "depth", "height", "head"],')
s = s.replace('''    "shading": ["orientation_floor", "orientation_exponent", "winter_height_ratio", "summer_height_ratio"],\n''', "")
s = s.replace('''strict({"latitude": number, "winter_altitude"''', '''strict({"latitude": number, "declination": number, "minimum_altitude": number, "winter_altitude"''')
p.write_text(s, encoding="utf-8", newline="\n")
