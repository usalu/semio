import math
for season, sign in (("winter", -1), ("summer", 1)):
    phi, delta = math.radians(47.4), math.radians(sign * 23.44)
    track = []
    for step in range(-900, 901):
        hour = math.radians(step / 5)
        alt = math.degrees(math.asin(math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.cos(hour)))
        if alt >= 3:
            track.append(((180 + math.degrees(math.atan2(math.sin(hour), math.cos(hour) * math.sin(phi) - math.tan(delta) * math.cos(phi)))) % 360, alt))
    print(season, min(b for b, _ in track), max(b for b, _ in track), max(a for _, a in track))
