"""🩹 Third excavator 2D repair (author 36): 1:100, 1:200 and 1:500 drew the house and the counterweight 0.16 m lower than 1:50, which closes the open gap between the tracks' car body and the house that the real machine has around the slew ring (1:50 shows the gap with the ring in it). The coarse 3D body then holds more volume than the finer one and the scales contradict each other. The house and counterweight bottoms go back to 0 (world 1.0 m) at all scales, and the slew ring is drawn in the elevation (100, 200, 500) and in the front (200, 500) as at 1:50. Text-level; runs once."""
import glob, re
P = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/excavator/*.json")[0]
t = open(P, encoding="utf8").read()
lines = t.split("\n")
out = []
n = 0
i = 0
while i < len(lines):
    ln = lines[i]
    if ln.lstrip().startswith('"d": "M') and "-0.16" in ln:
        ln = ln.replace(",-0.16", ",0")
        n += 1
    if ln.strip() == '"y": -0.16,' and lines[i + 2].strip() in ('"h": 1.41,', '"h": 2.13,'):
        h = lines[i + 2]
        ln = ln.replace("-0.16", "0")
        lines[i + 2] = h.replace("1.41", "1.25").replace("2.13", "1.97")
        n += 1
    out.append(ln)
    i += 1
t = "\n".join(out)
RING = '''      {
       "id": "slew-ring",
       "bone": "undercarriage",
       "role": "detail",
       "z": 2.6,
       "filled": true,
       "tone": 0.3,
       "kind": "rect",
       "x": -0.8,
       "y": 0.86,
       "w": 1.6,
       "h": 0.14,
       "material": "paint-steel"
      },
'''
base = t.index('"representations"')
for sc, views in ((100, ("elevation",)), (200, ("elevation", "front")), (500, ("elevation", "front"))):
    for v in views:
        k = t.index(f'"denominator": {sc},', base)
        j = t.index(f'"{v}": [\n', k)
        j2 = j + len(f'"{v}": [\n')
        end = t.index("\n     ]", j2)
        if '"id": "slew-ring"' in t[j2:end]:
            continue
        t = t[:j2] + RING + t[j2:]
        n += 1
print("edits", n)
open(P, "w", encoding="utf8").write(t)
