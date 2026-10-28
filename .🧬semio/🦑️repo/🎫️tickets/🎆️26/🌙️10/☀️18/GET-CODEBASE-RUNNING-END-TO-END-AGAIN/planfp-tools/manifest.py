"""🧾 Writes staged/manifest.json from staged/files.json, staged/base and staged/files (sha256 of each).
usage: python3 -I manifest.py"""
import hashlib, json, os
here = os.path.dirname(os.path.abspath(__file__)); s = os.path.join(here, "staged")
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
m = json.load(open(os.path.join(s, "files.json"), encoding="utf8"))
out = {rel: {"file": name, "base": sha(os.path.join(s, "base", name)), "new": sha(os.path.join(s, "files", name))} for name, rel in m.items()}
json.dump(out, open(os.path.join(s, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(out), "files")
