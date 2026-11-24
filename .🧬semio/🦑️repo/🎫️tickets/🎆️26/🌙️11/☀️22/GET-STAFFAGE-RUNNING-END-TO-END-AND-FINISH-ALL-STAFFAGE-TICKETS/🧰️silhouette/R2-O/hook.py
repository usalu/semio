"""🪝️ Appends the silhouette repair of batch R2-O as the last step of a Python generator (idempotent), after the step of batch SO where there is one: hook.py <generator.py> <id>"""
import os, sys
path, slug = sys.argv[1], sys.argv[2]
here = os.path.dirname(os.path.abspath(__file__))
text = open(path).read()
if "silhouette repair of batch R2-O" not in text:
    text = text.rstrip("\n") + f'''


if __name__ == "__main__":
    # the silhouette repair of batch R2-O, the last step: ops/{slug}.json of the umbrella ticket of 2026-11-22
    import importlib.util as _util
    from pathlib import Path as _Path
    _spec = _util.spec_from_file_location("silhouette_r2o_lib", "{here}/lib.py")
    _r2o = _util.module_from_spec(_spec)
    _spec.loader.exec_module(_r2o)
    _r2o.repair_file(_Path(__file__).resolve().parent / "🔣️.json", "{slug}")
'''
    open(path, "w").write(text)
    print(f"hooked {slug}")
