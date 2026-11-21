"""✍️ R03 area reviewer: sets `areaM2` and/or `body.skinFraction` of thermal regions in a catalog asset by text surgery on the number token only,
re-reads the file right before writing, checks that the parsed JSON differs in exactly the named values, writes atomically and logs to
`🗑️generated/r5-area/apply.log`. Exchange regions (`areaBasis: "exchange"`) refuse an `areaM2` change (C19).
Usage: python3 r5-area-apply.py <category>/<slug> <region>:areaM2=<v>[,skinFraction=<v>] [...] [--dry]
@see ./design.md (C19, C27, C29)"""
import glob, json, os, re, sys, tempfile, time
T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
LOG = os.path.join(T, "🗑️generated", "r5-area", "apply.log")

def asset_path(spec):
    c, s = spec.split("/")
    return glob.glob(os.path.join(ROOT, f"*{c}", s, "🔣️.json"))[0]

def region_span(text, region):
    t0 = text.index('"thermal": {')
    r0 = text.index('"regions": [', t0)
    hits = [m.start() for m in re.finditer(r'"id": "' + re.escape(region) + r'",', text[r0:])]
    if not hits:
        raise ValueError(f"region {region} not found")
    at = r0 + hits[0]
    depth, i = 0, text.rfind("{", r0, at)
    s = i
    while True:
        ch = text[i]
        if ch == '"':
            i += 1
            while text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return s, i + 1
        i += 1

def edit(text, region, key, value):
    s, e = region_span(text, region)
    m = re.compile(r'("' + key + r'": )([-0-9.eE+]+)').search(text, s, e)
    if m is None:
        raise ValueError(f"{region}: no {key}")
    return text[: m.start(2)] + json.dumps(value) + text[m.end(2):]

def expect(doc, region, key, value):
    r = next(r for r in doc["asset"]["thermal"]["regions"] if r["id"] == region)
    if key == "areaM2":
        if r.get("areaBasis") == "exchange":
            raise ValueError(f"{region} is an exchange region (C19)")
        r["areaM2"] = value
    else:
        r["body"][key] = value

def main():
    dry = "--dry" in sys.argv
    args = [a for a in sys.argv[1:] if a != "--dry"]
    spec, edits = args[0], []
    for a in args[1:]:
        region, kv = a.split(":", 1)
        for p in kv.split(","):
            k, v = p.split("=")
            edits.append((region, k, float(v)))
    path = asset_path(spec)
    for _ in range(10):
        text = open(path, encoding="utf8").read()
        doc = json.loads(text)
        new = text
        olds = []
        for region, k, v in edits:
            r = next(r for r in doc["asset"]["thermal"]["regions"] if r["id"] == region)
            olds.append(r["areaM2"] if k == "areaM2" else r["body"][k])
            new = edit(new, region, k, v)
            expect(doc, region, k, v)
        assert json.loads(new) == doc, "more than the named values changed"
        if dry:
            break
        if open(path, encoding="utf8").read() != text:
            time.sleep(0.3)
            continue
        fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path))
        with os.fdopen(fd, "w", encoding="utf8") as f:
            f.write(new)
        os.replace(tmp, path)
        break
    else:
        raise RuntimeError("file kept changing")
    line = f"{time.strftime('%Y-%m-%d %H:%M:%S')} {'DRY ' if dry else ''}{spec} " + " ".join(f"{r}.{k} {o} -> {v}" for (r, k, v), o in zip(edits, olds))
    print(line)
    if not dry:
        open(LOG, "a").write(line + "\n")

main()
