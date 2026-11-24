"""🪝️ Appends the silhouette repair of batch SO as the last step of a Python generator (idempotent): hook.py <generator.py> <id>"""
import os, sys
path, slug = sys.argv[1], sys.argv[2]
here = os.path.dirname(os.path.abspath(__file__))
text = open(path).read()
if "silhouette repair of batch SO" not in text:
    text = text.rstrip("\n") + f'''


if __name__ == "__main__":
    # the silhouette repair of batch SO, the last step: ops/{slug}.json of the umbrella ticket of 2026-11-22
    import sys as _sys
    from pathlib import Path as _Path
    _sys.path.insert(0, "{here}")
    from lib import repair_file as _repair_file
    _repair_file(_Path(__file__).resolve().parent / "🔣️.json", "{slug}")
'''
    open(path, "w").write(text)
    print(f"hooked {slug}")
