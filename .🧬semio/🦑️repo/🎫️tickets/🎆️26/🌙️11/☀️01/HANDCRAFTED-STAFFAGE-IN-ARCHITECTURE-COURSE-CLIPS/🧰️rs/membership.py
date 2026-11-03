import importlib, importlib.util, sys, time
from pathlib import Path
ROOT = Path.cwd(); sys.path.insert(0, str(ROOT))
new = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🐍️")
text = (ROOT / sys.argv[1]).read_text(encoding="utf-8").replace("Path(__file__).resolve().parent", "Path(" + repr(str(Path(new.__file__).parent)) + ")")
old = type(sys)("old_browser")
old.__file__ = new.__file__
exec(compile(text, "old_browser", "exec"), old.__dict__)
new.source_files()
scans = new._SCAN_SOURCES
t = time.time(); a = new._membership(scans); tn = time.time() - t
t = time.time(); b = old._membership(scans); to = time.time() - t
print("members", len(a), len(b), "equal", a == b, "new %.2fs old %.2fs" % (tn, to))
if a != b:
    print(sorted(set(a) ^ set(b))[:10])
