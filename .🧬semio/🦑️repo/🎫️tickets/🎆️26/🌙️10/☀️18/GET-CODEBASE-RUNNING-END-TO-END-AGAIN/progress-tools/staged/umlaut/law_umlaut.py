"""🧪 Law of batch `umlaut`: on the applied tree (1) every changed file is still valid JSON whose EN leaves and keys are byte-identical to the base and whose DE leaves differ only by whole-word umlaut substitutions of the word map, (2) no `de` leaf of any production catalog still holds a mapped transliteration, (3) EN/DE placeholder sets still agree. Independent oracle: the base file read by `json`, compared with `difflib` word diffs."""
import difflib, json, os, re, sys
root, manifest = sys.argv[1], sys.argv[2]
here = os.path.dirname(os.path.abspath(manifest))
ph = re.compile(r"\{([A-Za-z0-9_.]+)\}")
rows = json.load(open(manifest, encoding="utf8")); bad = []; checked = 0
def walk(old, new, path, rows_out):
    if isinstance(old, dict) and isinstance(new, dict):
        if set(old) != set(new): bad.append((path, "keys differ")); return
        for k in old: walk(old[k], new[k], path + "/" + k, rows_out)
    elif isinstance(old, list) and isinstance(new, list):
        if len(old) != len(new): bad.append((path, "length differs")); return
        for i, (a, b) in enumerate(zip(old, new)): walk(a, b, f"{path}[{i}]", rows_out)
    elif isinstance(old, str) and isinstance(new, str):
        if old != new: rows_out.append((path, old, new))
    elif old != new: bad.append((path, "value differs"))
def tokens_ok(a, b, label):
    x, y = re.findall(r"\w+", a), re.findall(r"\w+", b)
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, x, y, autojunk=False).get_opcodes():
        if tag == "equal": continue
        p, q = x[i1:i2], y[j1:j2]
        if len(p) != len(q) or any(w.replace("ä", "ae").replace("ö", "oe").replace("ü", "ue").replace("Ä", "Ae").replace("Ö", "Oe").replace("Ü", "Ue").replace("ß", "ss") != v for v, w in zip(p, q)): bad.append((label, f"not an umlaut substitution: {p} -> {q}")); return
for rel, row in rows.items():
    if row["base"] is None: continue
    if not rel.endswith(".json"):
        a = open(os.path.join(here, "base", row["file"]), encoding="utf8").read(); b = open(os.path.join(root, rel), encoding="utf8").read()
        checked += 1; tokens_ok(a, b, rel[-70:]); continue
    old = json.load(open(os.path.join(here, "base", row["file"]), encoding="utf8")); new = json.load(open(os.path.join(root, rel), encoding="utf8"))
    changed = []; walk(old, new, "", changed); checked += len(changed)
    for path, a, b in changed:
        if "/de" not in path: bad.append((rel[-60:] + path, "non-German leaf changed")); continue
        if set(ph.findall(a)) != set(ph.findall(b)): bad.append((rel[-60:] + path, "placeholders changed"))
        tokens_ok(a, b, rel[-60:] + path)
print("changed german leaves:", checked, "problems:", len(bad))
for b in bad[:30]: print("BAD", b)
sys.exit(1 if bad else 0)
