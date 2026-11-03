import importlib
import time

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
root = S._cache_root()
live = {S.cast_key(entry) for entry in S._manifest()["casts"].values()}
keys = [p for p in root.iterdir() if p.is_dir() and not p.name.startswith(".")]
now = time.time()
size = {p: sum(f.stat().st_size for f in p.rglob("*") if f.is_file()) for p in keys}
stale = [p for p in keys if p.name not in live]
print("key folders", len(keys), "live keys", len(live & {p.name for p in keys}), "stale", len(stale))
print("total GB", round(sum(size.values()) / 1e9, 2), "stale GB", round(sum(size[p] for p in stale) / 1e9, 2))
age = [(now - p.stat().st_mtime) / 3600 for p in stale]
print("stale older than 24 h:", sum(a > 24 for a in age), "older than 1 h:", sum(a > 1 for a in age))
print("tmp leftovers:", [p.name for p in root.iterdir() if p.name.startswith(".")][:10])
