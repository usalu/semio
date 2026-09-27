#!/usr/bin/env python3
"""V1 one-off: replace one field line inside named rows of .vscode/🧩️launch.seed.jsonc AND .vscode/launch.json identically.
usage: python3 v1-launch-edit.py <spec.json>; spec = [{"name": <row name>, "old": <exact substring>, "new": <replacement>}]"""
import json, sys
ROOT = "/Users/ueli/Documents/semio/.vscode/"
for path in [ROOT + "🧩️launch.seed.jsonc"]:
    text = open(path, encoding="utf-8").read()
    for item in json.load(open(sys.argv[1])):
        marker = f'      "name": {json.dumps(item["name"], ensure_ascii=False)},\n'
        if text.count(marker) != 1:
            raise SystemExit(f"{path}: row {item['name']!r} found {text.count(marker)} times")
        start = text.index(marker)
        end = text.index("\n    }", start)
        region = text[start:end]
        if region.count(item["old"]) != 1:
            if item["new"] in region:
                print(f"{path.rsplit('/', 1)[1]}: {item['name']}: already")
                continue
            raise SystemExit(f"{path}: {item['name']}: old text found {region.count(item['old'])} times")
        text = text[:start] + region.replace(item["old"], item["new"]) + text[end:]
        print(f"{path.rsplit('/', 1)[1]}: {item['name']}: edited")
    open(path, "w", encoding="utf-8").write(text)
