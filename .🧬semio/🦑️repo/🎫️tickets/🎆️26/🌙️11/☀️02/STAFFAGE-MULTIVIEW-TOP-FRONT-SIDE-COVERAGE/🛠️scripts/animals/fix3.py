import json, sys


def load(p):
    return json.load(open(p, encoding="utf-8-sig"))


def dump(p, d):
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def shapes(a, scale, view, id=None, bone=None):
    r = a["representations"][str(scale)]
    return [s for s in r["geometry"][view] if (id is None or s["id"] == id) and (bone is None or s["bone"] == bone)]


def need(found, label, n=1):
    if len(found) < n:
        raise SystemExit("missing " + label)
    return found


C = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
done = []


def edit(cat, slug, fn):
    p = C + cat + "/" + slug + "/🔣️.json"
    d = load(p)
    fn(d["asset"])
    dump(p, d)
    done.append(slug)


def cow(a):
    for sc in (50, 100, 200, 500):
        for s in need(shapes(a, sc, "front", "chest", "thorax"), "cow front chest"):
            s["rx"] = round(s["rx"] + 0.01, 4)
        for s in need(shapes(a, sc, "plan", "chest", "thorax"), "cow plan chest"):
            s["r0"] = round(s["r0"] + 0.01, 4)
            s["r1"] = round(s["r1"] + 0.01, 4)


def cat(a):
    for s in need(shapes(a, 200, "plan", "tail-b", "tail-4"), "cat tail-b"):
        s["x1"] = round(s["x1"] + 0.016, 4)


def gull(a):
    for sc in (100, 200):
        for bone in ("shank-n", "shank-f"):
            for s in need(shapes(a, sc, "front", None, bone), "gull leg"):
                if s["kind"] == "poly":
                    s["points"] = [[x, max(y, 0.0)] for x, y in s["points"]]


def pig(a):
    for bone in ("lumbar", "thorax"):
        for s in need(shapes(a, 500, "plan", None, bone), "pig " + bone):
            if s["kind"] == "poly" and s["role"] == "body":
                s["points"] = [[x, round(y * 0.965, 4)] for x, y in s["points"]]


def deer(a):
    for sc in (50, 100, 200):
        for s in shapes(a, sc, "front"):
            if s["bone"] in ("ear-n", "ear-f") and s["kind"] == "ellipse":
                s["cy"] = round(s["cy"] + 0.023, 4)
                s["cx"] = round(s["cx"] - 0.004 if s["cx"] > 0 else s["cx"] + 0.004, 4)


def sheep(a):
    for bone in ("lumbar", "thorax"):
        for s in need(shapes(a, 500, "plan", None, bone), "sheep " + bone):
            if s["kind"] == "poly" and s["role"] == "body":
                s["points"] = [[x, round(y * 0.955, 4)] for x, y in s["points"]]


edit("🐕️animals", "dairy-cow", cow)
edit("🐕️animals", "domestic-cat", cat)
edit("🐕️animals", "herring-gull", gull)
edit("🐕️animals", "pig", pig)
edit("🐕️animals", "roe-deer", deer)
edit("🐕️animals", "sheep", sheep)
print(done)
