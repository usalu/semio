"""Usage: python3 author-25-filled.py <slug> [--apply] [--ids scale:view:id,...] [--pen scale:view:id=mm,...]
With --pen sets `penMm` of the named shapes (text patch), e.g. the 1:500 dog legs whose pens were derived with the wrong scale.
Lists (and with --apply repairs at the root, as a text patch) closed body shapes drawn with `filled: false` in the 2D archive of an animals asset: sets `filled: true`. Strokes (`closed: false`) and detail/texture shapes are left alone. Only the shape's `filled` text changes."""
import glob, json, re, sys

slug = sys.argv[1]
only = None
for i, a in enumerate(sys.argv):
    if a == "--ids":
        only = set(sys.argv[i + 1].split(","))
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/%s/🔣️.json" % slug)[0]
text = open(path, encoding="utf-8").read()
dec = json.JSONDecoder()
rep_start = text.index('"representations"')
rep_end = text.index('\n  "symbols"', rep_start) if '\n  "symbols"' in text[rep_start:] else len(text)
patches = []
pos = rep_start
for m in re.finditer(r'"geometry": \{', text[rep_start:rep_end]):
    gstart = rep_start + m.end() - 1
    geo, gend = dec.raw_decode(text, gstart)
    scale = re.findall(r'"denominator": (\d+)', text[rep_start:gstart])[-1]
    for view in ("elevation", "front", "plan"):
        vm = re.compile(r'"%s": \[' % view).search(text, gstart, gend)
        if not vm:
            continue
        lst_start = vm.end() - 1
        lst, lst_end = dec.raw_decode(text, lst_start)
        p = lst_start + 1
        for shape in lst:
            while text[p] in " \n,":
                p += 1
            obj, e = dec.raw_decode(text, p)
            if shape.get("filled") is False and shape.get("role") == "body" and shape.get("closed", True) is not False:
                key = "%s:%s:%s" % (scale, view, shape["id"])
                if only is None or key in only:
                    patches.append((p, e, key))
            p = e
pens = {}
for i, a in enumerate(sys.argv):
    if a == "--pen":
        for kv in sys.argv[i + 1].split(","):
            k, v = kv.split("=")
            pens[k] = v
if pens:
    patches = []
    pos = rep_start
    for m in re.finditer(r'"geometry": \{', text[rep_start:rep_end]):
        gstart = rep_start + m.end() - 1
        geo, gend = dec.raw_decode(text, gstart)
        scale = re.findall(r'"denominator": (\d+)', text[rep_start:gstart])[-1]
        for view in ("elevation", "front", "plan"):
            vm = re.compile(r'"%s": \[' % view).search(text, gstart, gend)
            if not vm:
                continue
            lst_start = vm.end() - 1
            lst, lst_end = dec.raw_decode(text, lst_start)
            p = lst_start + 1
            for shape in lst:
                while text[p] in " \n,":
                    p += 1
                obj, e = dec.raw_decode(text, p)
                key = "%s:%s:%s" % (scale, view, shape["id"])
                if key in pens:
                    patches.append((p, e, key))
                p = e
for p, e, key in patches:
    print(key)
if "--apply" in sys.argv:
    fresh = open(path, encoding="utf-8").read()
    assert fresh == text, "file changed under me"
    for p, e, key in reversed(patches):
        seg = text[p:e]
        if pens:
            assert len(re.findall(r'"penMm": [0-9.eE+-]+', seg)) == 1
            seg = re.sub(r'"penMm": [0-9.eE+-]+', '"penMm": ' + pens[key], seg)
            text = text[:p] + seg + text[e:]
            continue
        assert seg.count('"filled": false') == 1
        text = text[:p] + seg.replace('"filled": false', '"filled": true') + text[e:]
    json.loads(text)
    open(path, "w", encoding="utf-8").write(text)
    print("patched", len(patches))
