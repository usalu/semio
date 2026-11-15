"""Usage: python3 author-46-rig.py <slug> <bone> <x> <y> [--apply] — text patch of a rig bone offset (x, y in the parent frame)."""
import glob, re, sys, json
slug, bone, x, y = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/%s/🔣️.json" % slug)[0]
t = open(path, encoding="utf-8").read()
i = t.index('"id": "%s",\n' % bone, t.index('"rig"'))
m1 = re.compile(r'"x": (-?[0-9.e-]+)').search(t, i)
m2 = re.compile(r'"y": (-?[0-9.e-]+)').search(t, m1.end())
assert m1.start() - i < 600 and m2.start() - m1.end() < 20, (m1.start() - i, m2.start() - m1.end())
print("old", m1.group(1), m2.group(1), "new", x, y)
if "--apply" in sys.argv:
    out = t[: m1.start()] + '"x": ' + x + t[m1.end() : m2.start()] + '"y": ' + y + t[m2.end() :]
    json.loads(out)
    assert open(path, encoding="utf-8").read() == t
    open(path, "w", encoding="utf-8").write(out)
    print("patched")
