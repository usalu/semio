#!/usr/bin/env python3
"""📦️ dashplan-tools: freezes the staged batch from a live root (base) and an edited mirror root (new) into staged/{manifest.json,files/,base/}.
Usage: python3 -I build_stage.py <live-root> <mirror-root>"""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
Z = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions"
RELS = [Z + "/🟦️.ts", Z + "/🧬️schema/🔣️.json", Z + "/🧪️tests/🟦️.ts", Z + "/🧪️tests/🧫️fixtures/🔣️.json", "🧰️framework/🛍️products/🦑️repo/README.md"]


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


live, mirror = sys.argv[1], sys.argv[2]
out = os.path.join(HERE, "staged")
shutil.rmtree(out, ignore_errors=True)
os.makedirs(out + "/files")
os.makedirs(out + "/base")
manifest = {}
for index, rel in enumerate(RELS):
    name = "f%03d" % index
    shutil.copy2(os.path.join(live, rel), out + "/base/" + name)
    shutil.copy2(os.path.join(mirror, rel), out + "/files/" + name)
    manifest[rel] = {"file": name, "base": sha(out + "/base/" + name), "new": sha(out + "/files/" + name)}
json.dump(manifest, open(out + "/manifest.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
shutil.copy2(os.path.join(HERE, "stage.py"), out + "/stage.py") if False else None
print("staged", len(manifest), "files")
