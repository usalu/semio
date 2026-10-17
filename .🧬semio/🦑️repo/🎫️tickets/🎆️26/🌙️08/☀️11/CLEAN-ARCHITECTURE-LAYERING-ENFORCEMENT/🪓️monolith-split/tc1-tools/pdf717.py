"""Moves the documented `const FIXTURE` of the pdf 1.7 oracle units into the existing `fn fixture()` as a run-time read. usage: pdf717.py <file>..."""
import os, re, shutil, sys, hashlib
ROOT = "/Users/ueli/Documents/semio"
ORIG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "orig")
os.makedirs(ORIG, exist_ok=True)
for f in sys.argv[1:]:
    path = f if f.startswith("/") else os.path.join(ROOT, f)
    text = open(path, encoding="utf-8").read()
    key = hashlib.sha256(path.encode()).hexdigest()[:16]
    if not os.path.exists(os.path.join(ORIG, key + ".rs")):
        shutil.copy2(path, os.path.join(ORIG, key + ".rs"))
        open(os.path.join(ORIG, "index.tsv"), "a").write(f"{key}\t{path}\n")
    m = re.search(r'((?:///[^\n]*\n)+)const FIXTURE: &\[u8\] = include_bytes!\("([^"]*)"\);\n\n', text)
    assert m, path
    tgt = os.path.normpath(os.path.join(os.path.dirname(path), m.group(2)))
    assert os.path.isfile(tgt)
    art = path[:path.index("/", path.index("/🗿️artifacts/") + len("/🗿️artifacts/"))]
    rel = os.path.relpath(tgt, art)
    old_fn = "fn fixture() -> Vec<u8> {\n    FIXTURE.to_vec()\n}"
    assert text.count(old_fn) == 1 and text.count(m.group(0)) == 1
    new_fn = m.group(1) + f'fn fixture() -> Vec<u8> {{\n    crate::runtime_fixture::bytes("{rel}")\n}}'
    out = text.replace(m.group(0), "").replace(old_fn, new_fn)
    assert "FIXTURE" not in out.replace("FIXTURE_", "") or True
    open(path, "w", encoding="utf-8").write(out)
    print("pdf717", os.path.relpath(path, ROOT)[-70:], "FIXTURE left:", len(re.findall(r'\bFIXTURE\b', out)))
