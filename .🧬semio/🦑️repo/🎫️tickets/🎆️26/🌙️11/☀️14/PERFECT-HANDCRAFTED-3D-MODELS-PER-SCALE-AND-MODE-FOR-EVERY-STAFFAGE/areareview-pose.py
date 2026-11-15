"""Surgical edit of a region state in a pose's thermal state: python3 areareview-pose.py <slug> <region> '<json state>' <pose id>..."""
import glob, importlib.util, json, os, sys, tempfile, time
T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("u", os.path.join(T, "areareview-unsection.py")); u = importlib.util.module_from_spec(spec); spec.loader.exec_module(u)
slug, region, state = sys.argv[1], sys.argv[2], json.loads(sys.argv[3]); poses = sys.argv[4:]
path = glob.glob(f"{u.CAT}/*/{slug}/🔣️.json")[0]
for _ in range(8):
    text = open(path, encoding="utf8").read()
    _, root = u.parse(text, 0)
    arr = root["asset"][3]["poses"][3]
    edits = []
    for p in arr:
        sub = p[3]
        if json.loads(text[sub["id"][1]:sub["id"][2]]) in poses:
            r = sub["thermal"][3]["regions"][3][region]
            ks = r[0]; ind = len(text[text.rfind("\n", 0, ks) + 1:ks])
            unit = ind - len(text[text.rfind("\n", 0, sub["thermal"][0]) + 1:sub["thermal"][0]]) - 2 if False else 2
            body = json.dumps(state, ensure_ascii=False, indent=unit).replace("\n", "\n" + " " * ind)
            edits.append((r[1], r[2], body))
    new = text
    for a, b, body in sorted(edits, reverse=True): new = new[:a] + body + new[b:]
    exp = json.loads(text)
    for p in exp["asset"]["poses"]:
        if p["id"] in poses: p["thermal"]["regions"][region] = state
    assert json.loads(new) == exp
    if open(path, encoding="utf8").read() != text: time.sleep(0.2); continue
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path))
    with os.fdopen(fd, "w", encoding="utf8") as f: f.write(new)
    os.replace(tmp, path); print("edited", len(edits)); break
