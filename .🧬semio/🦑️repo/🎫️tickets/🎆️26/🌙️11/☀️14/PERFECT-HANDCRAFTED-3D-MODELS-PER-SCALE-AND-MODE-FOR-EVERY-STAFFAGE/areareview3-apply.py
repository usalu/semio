"""Reviewer 3 apply: area decisions of areareview3-decisions.py (number token of areaM2, areaBasis line) and default-pose fractions (existing number tokens only), by text surgery
checked by JSON comparison, written atomically, re-read right before writing. Usage: python3 areareview3-apply.py [--dry]"""
import glob, importlib.util, json, os, re, sys, tempfile, time

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
DRY = "--dry" in sys.argv

def load(name, file):
    s = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    m = importlib.util.module_from_spec(s)
    s.loader.exec_module(m)
    return m

dec = load("dec3", "areareview3-decisions.py")
uns = load("uns", "areareview2-unsection.py")
parse = uns.parse

def path_of(slug):
    return glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]

def edit_area(text, region, value, basis):
    mt = min(re.finditer(r"\n( +)\"thermal\": \{\n( +)\"\w+\"", text), key=lambda m: len(m.group(1)))
    i1, i2 = len(mt.group(1)), len(mt.group(2))
    k = " " * (i2 + 2 * (i2 - i1))
    kc = " " * (i2 + (i2 - i1))
    r0 = text.index("\n" + " " * i2 + "\"regions\": [", mt.start())
    r1 = text.index("\n" + " " * i2 + "]", r0)
    marker = f"\n{k}\"id\": \"{region}\","
    at = text.index(marker, r0)
    if at > r1 or text.count(marker, r0, r1) != 1:
        raise ValueError(f"region {region} not unique")
    end = text.find("\n" + kc + "}", at)
    m = re.compile(r"(\n" + k + r"\"areaM2\": )([-0-9.eE+]+)(,)").search(text, at, end)
    if m is None:
        raise ValueError(f"no areaM2 for {region}")
    tok = value if value is not None else m.group(2)
    out = text[: m.start(2)] + tok + text[m.end(2):]
    if basis:
        seg = out[at: out.find("\n" + kc + "}", at)]
        if "\"areaBasis\"" not in seg:
            c = out.index(",", m.start(2) + len(tok))
            out = out[: c + 1] + "\n" + k + "\"areaBasis\": \"exchange\"," + out[c + 1:]
    return out

def edit_sf(text, region, value):
    mt = min(re.finditer(r"\n( +)\"thermal\": \{\n( +)\"\w+\"", text), key=lambda m: len(m.group(1)))
    i1, i2 = len(mt.group(1)), len(mt.group(2))
    k = " " * (i2 + 2 * (i2 - i1))
    kc = " " * (i2 + (i2 - i1))
    r0 = text.index("\n" + " " * i2 + "\"regions\": [", mt.start())
    marker = f"\n{k}\"id\": \"{region}\","
    at = text.index(marker, r0)
    end = text.find("\n" + kc + "}", at)
    m = re.compile(r"(\"skinFraction\": )([-0-9.eE+]+)").search(text, at, end)
    if m is None:
        raise ValueError(f"no skinFraction for {region}")
    return text[: m.start(2)] + value + text[m.end(2):]

def edit_pose(text, doc, region, key, value):
    pid = doc["presentation"]["defaultPose"]
    end, root = parse(text, 0)
    poses = root["asset"][3]["poses"][3]
    for p in poses:
        sub = p[3]
        if json.loads(text[sub["id"][1]:sub["id"][2]]) == pid:
            reg = sub["thermal"][3]["regions"][3][region][3]
            if key == "exposedFraction":
                s, e = reg[key][1], reg[key][2]
            else:
                c = reg["contact"][3]["fraction"]
                s, e = c[1], c[2]
            return text[:s] + json.dumps(value) + text[e:]
    raise ValueError("pose not found")

def expect_pose(doc, region, key, value):
    pid = doc["presentation"]["defaultPose"]
    for p in doc["poses"]:
        if p["id"] == pid:
            r = p["thermal"]["regions"][region]
            if key == "exposedFraction":
                r[key] = value
            else:
                r["contact"]["fraction"] = value

def write(path, new):
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path))
    with os.fdopen(fd, "w", encoding="utf8") as f:
        f.write(new)
    os.replace(tmp, path)

def apply(path, fn, expect):
    for _ in range(8):
        text = open(path, encoding="utf8").read()
        new = fn(text)
        if new == text:
            return False
        exp = json.loads(text)
        expect(exp)
        assert json.loads(new) == exp, f"{path}: more than the target changed"
        if DRY:
            return True
        if open(path, encoding="utf8").read() != text:
            time.sleep(0.2)
            continue
        write(path, new)
        return True
    raise RuntimeError(f"{path}: file kept changing")

def main():
    log = []
    for (slug, region), (act, val, why) in sorted(dec.R.items()):
        if act in ("D", "A") and act != "X":
            continue
        path = path_of(slug)
        tok = None if val is None else json.dumps(val)
        basis = act in ("X", "SX")
        def fn(text):
            return edit_area(text, region, tok, basis)
        def ex(doc, region=region, val=val, basis=basis):
            for r in doc["asset"]["thermal"]["regions"]:
                if r["id"] == region:
                    if val is not None:
                        r["areaM2"] = val
                    if basis:
                        r["areaBasis"] = "exchange"
        if apply(path, fn, ex):
            log.append((slug, region, val))
    for slug, region, key, val, why in dec.POSES:
        path = path_of(slug)
        doc = json.load(open(path))["asset"]
        def fn(text):
            return edit_pose(text, doc, region, key, val)
        def ex(d, region=region, key=key, val=val):
            expect_pose(d["asset"], region, key, val)
        if apply(path, fn, ex):
            log.append((slug, region, key, val))
    for slug, region, val, old, *sf in getattr(dec, 'BODY', []):
        path = path_of(slug)
        tok = json.dumps(val)
        def fn(text, region=region, tok=tok, sf=sf):
            t = edit_area(text, region, tok, False)
            return edit_sf(t, region, json.dumps(sf[0])) if sf else t
        def ex(doc, region=region, val=val, sf=sf):
            for r in doc['asset']['thermal']['regions']:
                if r['id'] == region:
                    r['areaM2'] = val
                    if sf:
                        r['body']['skinFraction'] = sf[0]
        if apply(path, fn, ex):
            log.append((slug, region, val))
    for x in log:
        print("applied", x)
    print(len(log), "changes")

main()
