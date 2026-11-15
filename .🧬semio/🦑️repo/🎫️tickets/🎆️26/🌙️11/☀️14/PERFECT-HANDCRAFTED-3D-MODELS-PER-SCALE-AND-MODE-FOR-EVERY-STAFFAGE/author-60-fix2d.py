"""Author 60: 2D root repair of humans/child-on-kick-scooter (plan backpack, plan hands). python3 author-60-fix2d.py <scale> <name>"""
import sys, re
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/child-on-kick-scooter/🔣️.json"
SC = {"50": 969, "100": 4393, "200": 6909, "500": 9095}

def section(text, den, view):
    a = text.index(f'"denominator": {den},')
    a = text.index(f'"{view}": [', a)
    b = text.index("\n          ]", a)
    return a, b

def edit(den, view, ident, repl):
    t = open(P, encoding="utf8").read()
    a, b = section(t, den, view)
    seg = t[a:b]
    m = re.search(r'\{\n(?:(?!\n            \}).)*?"id": "%s",(?:(?!\n            \}).)*\n            \}' % re.escape(ident), seg, re.S)
    obj = m.group(0)
    new = obj
    for k, v in repl.items():
        new, n = re.subn(r'("%s": )[^,\n]+' % re.escape(k), lambda mm: mm.group(1) + v, new, count=1)
        assert n == 1, (k, obj)
    t = t[:a] + seg.replace(obj, new, 1) + t[b:]
    open(P, "w", encoding="utf8").write(t)
    print("edited", den, view, ident, repl)

if __name__ == "__main__":
    for den in sys.argv[1:] or ["500"]:
        edit(den, "plan", "backpack", {"x": "-0.175", "y": "-0.12", "w": "0.173", "h": "0.24", "bone": '"pelvis"'})

def split_base_500():
    t = open(P, encoding="utf8").read()
    a = t.index('"denominator": 500,')
    a = t.index('"plan": [', a)
    i = t.index('"id": "scooter-base"', a)
    s = t.rfind("{", 0, i)
    e = t.index("\n            }", i) + len("\n            }")
    obj = t[s:e]
    assert '"x1": 0.638' in obj
    new = obj.replace('"x1": 0.638', '"x1": 0.47')
    def cap(i, x0, x1, r, z, regions):
        return ('{\n              "kind": "capsule",\n              "x0": %s,\n              "y0": 0,\n              "x1": %s,\n              "y1": 0,\n              "r0": %s,\n              "r1": %s,\n              "id": "%s",\n              "bone": "frame",\n              "role": "body",\n              "z": %s,\n              "filled": true,\n              "tone": 0.5,\n              "regions": [\n%s\n              ]\n            }' % (x0, x1, r, r, i, z, ",\n".join('                "%s"' % q for q in regions)))
    extra = ",\n            ".join([cap("scooter-fork", 0.47, 0.638, 0.0375, 0.55, ["stem"]), cap("scooter-front-wheel", 0.638, 0.638, 0.05, 0.56, ["wheels"])])
    t = t[:s] + new + ",\n            " + extra + t[e:]
    open(P, "w", encoding="utf8").write(t)

def front_grips():
    t = open(P, encoding="utf8").read()
    for den in (50, 100, 200):
        a = t.index(f'"denominator": {den},')
        a = t.index('"front": [', a)
        b = t.index("\n          ]", a)
        seg = t[a:b]
        for ident, x in (("grip-l", "0.1"), ("grip-r", "-0.2075")):
            i = seg.index(f'"id": "{ident}"')
            s = seg.rfind("{", 0, i)
            e = seg.index("}", i)
            o = seg[s:e]
            n = re.sub(r'("x": )[-\d.]+', lambda m: m.group(1) + x, o, count=1)
            n = re.sub(r'("w": )[-\d.]+', lambda m: m.group(1) + "0.1075", n, count=1)
            seg = seg.replace(o, n, 1)
        t = t[:a] + seg + t[b:]
    open(P, "w", encoding="utf8").write(t)
