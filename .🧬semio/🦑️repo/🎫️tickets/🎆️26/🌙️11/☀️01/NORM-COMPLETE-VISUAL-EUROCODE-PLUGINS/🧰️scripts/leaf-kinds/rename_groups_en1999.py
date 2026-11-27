"""Renames the numbered en1999 group crates (`-2`) to family names across dirs, Cargo, script.ts and build-owners. Usage: python -I rename_groups_en1999.py"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

ART = NORM + "/🗿️artifacts/🪶️en1999"
MAP = [("connection-action-2", "connection-action-components"), ("cold-formed-action-2", "cold-formed-action-components"), ("member-action-2", "member-action-components"), ("shell-action-2", "shell-action-components"), ("section-element-2", "section-element-stiffeners"), ("connection-2", "connection-fasteners"), ("member-2", "member-web-details")]
def validated_registry(text):
    """Mirrors the registry reader of the build: a version 1 object, unique names, roots, native roots and package names, and a manifest that is exactly <root>/📋️project.json or <root>/Cargo.toml."""
    import json
    data = json.loads(text)
    assert data.get("version") == 1 and isinstance(data.get("owners"), list) and isinstance(data.get("catalogProviders"), list), "not a version 1 registry"
    names, roots, packages = set(), set(), set()
    for o in data["owners"]:
        assert o["name"] and o["name"] not in names, "duplicate owner name " + o["name"]
        assert o["root"] not in roots, "duplicate owner root " + o["root"]
        assert any(o["manifest"] == (n if o["root"] == "." else o["root"] + "/" + n) for n in ("📋️project.json", "Cargo.toml")), "manifest mismatch " + o["manifest"]
        names.add(o["name"])
        for r in [o["root"]] + o.get("nativeRoots", []):
            assert r not in roots, "duplicate root " + r
            roots.add(r)
        for k in o["packages"]:
            assert k not in packages, "duplicate package " + k
            packages.add(k)
    return text


def write_atomic(f, text):
    """Stages the text beside the file (validated first when it is the root registry), keeps the mode and renames it into place, so a refusal or a crash leaves the old file byte for byte."""
    if f == OWNERS:
        validated_registry(text)
    tmp = "%s.%d.tmp" % (f, os.getpid())
    try:
        with open(tmp, "w", encoding="utf-8") as h:
            h.write(text)
        os.chmod(tmp, os.stat(f).st_mode & 0o7777)
        os.replace(tmp, f)
    finally:
        if os.path.exists(tmp):
            os.remove(tmp)


files = [OWNERS]
for d, _, fs in os.walk(ART):
    files += [os.path.join(d, f) for f in fs if f.endswith((".rs", ".toml", ".ts", ".json", ".lock"))]
n = 0
for f in files:
    t = rd(f)
    u = t
    for old, new in MAP:
        for a, b in ((old, new), (old.replace("-", "_"), new.replace("-", "_"))):
            u = re.sub(r"(?<![A-Za-z0-9])%s(?![A-Za-z0-9])" % re.escape(a), b, u)
    if u != t:
        write_atomic(f, u)
        n += 1
print("files", n)
base = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
for x in sorted(os.listdir(base)):
    for old, new in MAP:
        if x.endswith(old) and re.sub(r"^[^a-z]+", "", x) == old:
            os.rename(base + "/" + x, base + "/" + x[: -len(old)] + new)
            print(x, "->", new)
