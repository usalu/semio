import json
import pathlib

schema = json.loads(pathlib.Path("framework-placeholder").read_text(encoding="utf-8"))
