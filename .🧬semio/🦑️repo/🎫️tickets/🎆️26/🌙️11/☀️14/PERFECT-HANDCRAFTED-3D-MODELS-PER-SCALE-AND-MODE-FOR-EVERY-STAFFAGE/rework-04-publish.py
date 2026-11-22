"""📮 Author 04: writes a validated candidate document into the catalog. Every top-level member of `asset` that differs from the catalog file (except `volumes`) is replaced as a whole in two-space JSON, every other byte of the file stays; the `volumes` go to the source `volumes/<category>/<slug>.json` and are spliced by `volumes-insert.py`; the manifest row takes the new `heightM` and `widthM`.

Usage: python3 rework-04-publish.py <category>/<slug>
The catalog file and the manifest are re-read immediately before writing.
"""
import json
import os
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
S = os.path.normpath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"))
CATEGORY = {"humans": "👤️humans"}


def member_range(text, name):
    start = text.index('\n    "%s": ' % name)
    open_at = start + len('\n    "%s": ' % name)
    depth, quoted, i = 0, False, open_at
    if text[open_at] not in "{[":
        end = open_at
        while text[end] not in ",\n":
            end += 1
        return open_at, end
    while i < len(text):
        ch = text[i]
        if quoted:
            if ch == "\\":
                i += 1
            elif ch == '"':
                quoted = False
        elif ch == '"':
            quoted = True
        elif ch in "{[":
            depth += 1
        elif ch in "}]":
            depth -= 1
            if depth == 0:
                return open_at, i + 1
        i += 1
    raise SystemExit("unbalanced")


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n    ")


def main():
    category, slug = sys.argv[1].split("/")
    candidate = json.load(open(os.path.join(T, "🗑️generated", "rework-04", slug, "🔣️.json"), encoding="utf8"))["asset"]
    path = os.path.join(S, "🗂️catalog", CATEGORY[category], slug, "🔣️.json")
    text = open(path, encoding="utf8").read()
    current = json.loads(text)["asset"]
    if list(candidate.keys()) != list(current.keys()):
        raise SystemExit("member order differs: %s vs %s" % (list(candidate.keys()), list(current.keys())))
    changed = [k for k in current if k != "volumes" and candidate[k] != current[k]]
    for name in changed:
        a, b = member_range(text, name)
        text = text[:a] + dump(candidate[name]) + text[b:]
    check = json.loads(text)["asset"]
    for k in current:
        if k != "volumes" and check[k] != candidate[k]:
            raise SystemExit("member %s not written as intended" % k)
    open(path, "w", encoding="utf8").write(text)
    source = os.path.join(T, "volumes", category, slug + ".json")
    open(source, "w", encoding="utf8").write(json.dumps({"volumes": candidate["volumes"]}, ensure_ascii=False) + "\n")
    subprocess.run([sys.executable, os.path.join(T, "volumes-insert.py"), source, path], check=True)
    manifest_path = os.path.join(S, "🗂️catalog", "🔣️.json")
    manifest = open(manifest_path, encoding="utf8").read()
    at = manifest.index('"id": "%s"' % slug)
    for key in ("heightM", "widthM"):
        k = manifest.index('"%s": ' % key, at)
        end = k
        while manifest[end] not in ",\n":
            end += 1
        manifest = manifest[:k] + '"%s": %s' % (key, json.dumps(candidate["dimensions"][key])) + manifest[end:]
    json.loads(manifest)
    open(manifest_path, "w", encoding="utf8").write(manifest)
    print("published", sys.argv[1], "members:", ", ".join(changed), "+ volumes; manifest heightM", candidate["dimensions"]["heightM"], "widthM", candidate["dimensions"]["widthM"])


main()
