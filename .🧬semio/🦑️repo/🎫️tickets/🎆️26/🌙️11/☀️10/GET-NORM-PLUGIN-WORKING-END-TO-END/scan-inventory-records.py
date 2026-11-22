"""🔎️ Lists content-addressed closure inventory leaves whose sha256 differs from their name."""
import hashlib, os, sys
for directory in sys.argv[1:]:
    bad = []
    for name in os.listdir(directory):
        if not name.endswith(".json"): continue
        path = os.path.join(directory, name)
        with open(path, "rb") as f: digest = hashlib.sha256(f.read()).hexdigest()
        if digest != name[:-5]: bad.append((name, os.path.getsize(path), os.path.getmtime(path)))
    print(directory, "bad", len(bad))
    for row in bad[:20]: print(" ", row)
