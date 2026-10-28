"""python3 mk-program-patch.py <tsc-listFilesOnly.txt> <project> <target>[,<target>...] <out.json> [drop-input ...]
Builds a measure.mjs patch that replaces the dependency view (`^default`) and the `reads` input of the targets by the files of the TypeScript program
(tsc --listFilesOnly, the third-party oracle) plus the package manifests above them; own `default` and every other input stay."""
import json, os, sys
listing, project, targets, out = sys.argv[1:5]
drop = set(sys.argv[5:]) or {"^default", "reads"}
ROOT = "/Users/ueli/Documents/semio/"
files = set()
with open(listing, encoding="utf-8") as handle:
    for line in handle:
        line = line.strip()
        if line.startswith(ROOT) and "/node_modules/" not in line:
            files.add(line[len(ROOT):])
manifests = set()
for f in files:
    parts = f.split("/")
    for depth in range(len(parts) - 1, -1, -1):
        m = "/".join(parts[:depth] + ["package.json"])
        if os.path.exists(ROOT + m):
            manifests.add(m)
            break
body = ["{workspaceRoot}/" + f for f in sorted(files | manifests)]
patch = {project: {"targets": {}, "namedInputs": {}}}
frag = json.load(open("/private/tmp/claude-501/keys-g-fragments.json")) if os.path.exists("/private/tmp/claude-501/keys-g-fragments.json") else {}
for t in targets.split(","):
    patch[project]["targets"][t] = {"__replace": sorted(drop), "__append": body}
json.dump(patch, open(out, "w"), ensure_ascii=False)
print(len(files), "program files", len(manifests), "manifests")
