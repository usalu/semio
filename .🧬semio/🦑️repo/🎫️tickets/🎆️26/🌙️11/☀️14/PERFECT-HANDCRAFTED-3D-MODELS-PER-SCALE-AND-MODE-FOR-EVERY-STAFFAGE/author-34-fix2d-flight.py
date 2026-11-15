import json, math, re
F = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/stair-flight/🔣️.json"
A = math.radians(38.853374)
UP = (math.sin(A), math.cos(A))
STEP = 0.38523
t = open(F, encoding="utf8").read().split("\n")
start = next(i for i, l in enumerate(t) if l == '        "denominator": 500,')
end = next(i for i in range(start, len(t)) if t[i] == '        "decisions": {')
I = "              "


def pts_lines(pts):
    L = [I + '"points": [']
    for i, p in enumerate(pts):
        L += [I + "  [", I + "    %s," % json.dumps(p[0]), I + "    %s" % json.dumps(p[1]), I + "  ]" + ("," if i < len(pts) - 1 else "")]
    L.append(I + "],")
    return L


def find(i0, i1, text):
    return next(i for i in range(i0, i1) if t[i].strip() == text)


k = 0
for n in range(1, 13):
    sid = '"id": "step-%02d",' % n
    i = find(start, end, sid)
    f = next(j for j in range(i, i + 12) if t[j].strip() == '"filled": false,')
    t[f] = t[f].replace("false", "true")
    ps = next(j for j in range(i, i + 40) if t[j] == I + '"points": [')
    pe = next(j for j in range(ps, ps + 20) if t[j] == I + '],')
    ce = pe + 1
    assert t[ce].strip() == '"closed": false'
    sh = (n - 1) * STEP
    a = (0.12824 + sh, 0.20702)
    b = (0.38523 + sh, 0.0)
    pts = [a, b, (b[0] - 0.075 * UP[0], b[1] - 0.075 * UP[1]), (a[0] - 0.075 * UP[0], a[1] - 0.075 * UP[1])]
    pts = [[round(x, 5), round(y, 5)] for x, y in pts]
    t[ps:ce + 1] = pts_lines(pts) + [I + '"closed": true']
    end = next(i for i in range(start, len(t)) if t[i] == '        "decisions": {')
i = find(start, end, '"id": "head-bearing",')
j = next(j for j in range(i, i + 14) if t[j].strip() == '"penMm": 0.35,')
t[j] = t[j].replace("0.35", "0.13")
fr = next(i for i in range(start, end) if t[i] == '          "front": [')
for n in range(1, 13):
    i = find(fr, end, '"id": "tread-%02d",' % n)
    j = next(j for j in range(i, i + 16) if t[j].strip().startswith('"y":'))
    y = float(t[j].strip()[4:].rstrip(","))
    t[j] = t[j].split('"y"')[0] + '"y": %s,' % json.dumps(round(y - 0.015, 5))
    j = next(j for j in range(i, i + 16) if t[j].strip().startswith('"h":'))
    t[j] = t[j].split('"h"')[0] + '"h": 0.075'
i = find(start, end, '"id": "flight",')
j = next(j for j in range(i, i + 20) if t[j].strip().startswith('"w":'))
t[j] = t[j].split('"w"')[0] + '"w": 3.63,'
k2 = next(j for j in range(j, j + 6) if t[j] == "            },")
new = ['            {', '              "id": "stringer-end",', '              "bone": "stringer",', '              "role": "body",', '              "z": 1,', '              "filled": true,', '              "tone": 0.4,', '              "region": "stringer-steel",', '              "material": "steel",', '              "kind": "rect",', '              "x": 3.6,', '              "y": -0.08,', '              "w": 0.25511,', '              "h": 0.16', '            },']
t[k2 + 1:k2 + 1] = new
txt = "\n".join(t)
rep = {
    "A dark sloping stringer stroke (0.35 mm, the 160 mm HEA depth) with twelve short tread strokes above it; the open risers stay open, so the flight reads as a sloping band with a rhythm of twelve steps.": "A dark sloping stringer band (the 160 mm HEA depth) with twelve tread bars above it, each drawn at the 0.075 m smallest feature (0.15 mm) as a filled parallelogram; the open risers stay open, so the flight reads as a sloping band with a rhythm of twelve steps.",
    "Ein dunkler geneigter Wangenstrich (0,35 mm, die Tiefe 160 mm der HEA) mit zwölf kurzen Stufenstrichen darüber; die offenen Setzstufen bleiben offen, der Lauf liest sich als geneigtes Band mit zwölf Stufen im Rhythmus.": "Ein dunkles geneigtes Wangenband (die Tiefe 160 mm der HEA) mit zwölf Stufenbalken darüber, jeder mit dem kleinsten Merkmal 0,075 m (0,15 mm) als gefülltes Parallelogramm gezeichnet; die offenen Setzstufen bleiben offen, der Lauf liest sich als geneigtes Band mit zwölf Stufen im Rhythmus.",
    "the plan is the flight footprint 1.2 m wide with the walking line and arrow.": "the plan is the tread footprint 1.2 m wide with the stringer head beyond it, the walking line and arrow.",
    "die Draufsicht ist der Laufgrundriss, 1,2 m breit, mit Lauflinie und Pfeil.": "die Draufsicht ist der Stufengrundriss, 1,2 m breit, mit dem Wangenkopf dahinter, Lauflinie und Pfeil.",
}
for a, b in rep.items():
    assert txt.count(a) == 1, a[:40]
    txt = txt.replace(a, b)
json.loads(txt)
open(F, "w", encoding="utf8").write(txt)
print("ok")
