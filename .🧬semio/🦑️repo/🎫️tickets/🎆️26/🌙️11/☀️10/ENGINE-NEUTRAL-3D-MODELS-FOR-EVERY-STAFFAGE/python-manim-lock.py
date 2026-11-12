"""Rebuilds the pinned package rows of `🧱️native/🔣️.json` from micromamba dry-run solves (`solve-<host>.json`, made with
`micromamba create --no-rc --override-channels -c conda-forge --platform <host> --dry-run --json -p <empty> <specs>`).
Usage: python3 python-manim-lock.py <solve-dir> <manifest.json>. Keeps the pinned installer rows; rewrites `specs` and `packages`."""
import json, sys
solves, manifest = sys.argv[1:3]
SPECS = ["cairo>=1.18,<1.19", "pkgconf", "zlib", "freetype", "expat"]
doc = json.load(open(manifest))
packages = []
for host in ("osx-arm64", "osx-64"):
    for p in sorted(json.load(open(f"{solves}/solve-{host}.json"))["actions"]["LINK"], key=lambda p: p["name"]):
        assert p["subdir"] in (host, "noarch") and p["url"].startswith("https://conda.anaconda.org/conda-forge/"), p["url"]
        packages.append({"host": host, "name": p["name"], "version": p["version"], "build": p["build_string"], "subdir": p["subdir"], "archive": p["fn"], "url": p["url"], "sha256": p["sha256"], "md5": p["md5"], "bytes": p["size"]})
doc["specs"], doc["packages"] = SPECS, packages
json.dump(doc, open(manifest, "w"), indent=2, ensure_ascii=False)
open(manifest, "a").write("\n")
print(len(packages))
