import json, re, subprocess, sys, difflib, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import de
repo = pathlib.Path("/Users/ueli/Documents/semio")
out = pathlib.Path(__file__).parent / "patches" / "cad-spec-json-1.patch"
files = subprocess.run(["git", "ls-files", "-z"], cwd=repo, capture_output=True, text=True, check=True).stdout.split("\0")
files = sorted(f for f in files if "modelDefinitions/" in f and "/🕹️interactions/" in f and f.endswith(".json"))
if len(files) != 60: sys.exit(f"FAIL-CLOSED: {len(files)} specs, expected 60")
rx = re.compile(r'^(\s*)"(label|prompt|text)": ("(?:[^"\\]|\\.)*")(,?)$')
kind = {"label": None, "prompt": de.PROMPT, "text": None}
lit = lambda s: json.dumps(s, ensure_ascii=False)
chunks, total, same = [], 0, 0
for f in files:
    src = (repo / f).read_text(encoding="utf8")
    spec_label_done = False
    res = []
    for line in src.splitlines(True):
        m = rx.match(line.rstrip("\n"))
        if not m:
            res.append(line); continue
        ind, key, raw, comma = m.groups()
        en = json.loads(raw)
        if key == "label":
            table = de.SPEC if len(ind) == 2 else de.LABEL
            g = table[en]
        elif key == "prompt":
            g = de.PROMPT[en]
        else:
            g = de.text(en)
        if not g.strip(): sys.exit("FAIL-CLOSED: empty translation " + en)
        total += 1; same += g == en
        row = f'{{ "en": {lit(en)}, "de": {lit(g)} }}'
        res.append(f'{ind}"{key}": {{\n{ind}  "native": {row},\n{ind}  "reuse": {row}\n{ind}}}{comma}\n')
    new = "".join(res)
    json.loads(new)
    chunks.append("".join(difflib.unified_diff(src.splitlines(True), new.splitlines(True), "a/" + f, "b/" + f)))
out.write_text("".join(chunks))
print("wrote", out, "strings", total, "identical en=de", same)
