"""✏️ [rusttests] applies every edit of the lane to a root that holds the base bytes (live plus the unlanded stack). usage: edit.py <root>"""
import os, re, shutil, subprocess, sys
import os as _o, sys as _s; _s.path.insert(0, _o.path.dirname(_o.path.abspath(__file__)))
from lib import *
root = sys.argv[1]
def path(rel): return os.path.join(root, rel)
def load(rel): return open(path(rel), encoding="utf8").read()
def save(rel, text):
    os.makedirs(os.path.dirname(path(rel)), exist_ok=True); open(path(rel), "w", encoding="utf8").write(text)
subprocess.run([sys.executable, "-I", os.path.join(HERE, "edit_helper.py"), path(HELPER)], check=True)
shutil.copyfile(os.path.join(HERE, "src", "envelope.schema.json"), path(HELPER_SCHEMA)) if os.makedirs(os.path.dirname(path(HELPER_SCHEMA)), exist_ok=True) is None else None
os.makedirs(os.path.dirname(path(NATIVE)), exist_ok=True); shutil.copyfile(os.path.join(HERE, "src", "native-birth.ts"), path(NATIVE))
def declare(rel, suite, scratch):
    t = load(rel)
    assert "declareLibraryBuildTestEnvelopeV1" not in t, rel
    lines = t.split("\n")
    last = max(i for i, l in enumerate(lines) if re.match(r"\s*import\b", l) and (re.search(r"\bfrom\s*[\"']", l) or re.match(r"\s*import\s*[\"']", l)))
    imp = 'import{declareLibraryBuildTestEnvelopeV1}from"%s";' % relative_import(rel, NATIVE)
    args = '{suite:"%s"%s}' % (suite, ',scratch:"checkout"' if scratch == "checkout" else "")
    lines[last + 1:last + 1] = [imp, "declareLibraryBuildTestEnvelopeV1(%s);" % args]
    save(rel, "\n".join(lines))
for rel, suite, scratch in ROWS: declare(rel, suite, scratch)
if os.path.exists(os.path.join(HERE, "edit_extra.py")): exec(open(os.path.join(HERE, "edit_extra.py"), encoding="utf8").read())
