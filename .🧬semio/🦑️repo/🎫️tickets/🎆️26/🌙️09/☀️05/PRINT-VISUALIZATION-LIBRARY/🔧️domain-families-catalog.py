"""🔧️ Copies the verified option sets of the FAMILIES-DOMAIN probe into the catalogue.

Reads the probe document (the only place where every option set has been compiled and proven
pairwise distinct) and writes the same options plus the demo table onto the catalogue entries of
the sixteen domain families. The catalogue file is re-read immediately before the write and
written atomically, because other agents edit other sections of it at the same time.
"""

import io
import json
import os
import re
import sys

PROBE = sys.argv[1]
CATALOG = sys.argv[2]

FAMILY_DEFAULT_DATA = {
    "text-viz": "demo-text-terms",
    "spatial-layout": "demo-scatter",
    "performance": "demo-hierarchy-deep",
    "schedule": "demo-sprint",
    "version-control": "demo-repo",
    "engineering-diagram": "demo-machine",
    "optimization": "demo-assignment",
    "analytical": "demo-causes",
    "notation": "demo-molecule",
    "logistics": "demo-shipments",
    "niche": "demo-survey",
    "monitoring": "demo-services",
    "neural": "demo-neural",
    "survey": "demo-survey",
    "election": "demo-election",
    "biology": "demo-biodiversity",
}

CASE = re.compile(r"^\\Case\{([^}]*)\}\{([^}]*)\}\{(.*)\}$")


def split_options(text):
    out, depth, cur = [], 0, ""
    for ch in text:
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur)
    result = {}
    for item in out:
        key, _, value = item.partition("=")
        key = key.strip()
        value = value.strip()
        if value.startswith("{") and value.endswith("}"):
            value = value[1:-1]
        if value in ("true", "false"):
            result[key] = value == "true"
        elif re.fullmatch(r"-?\d+", value):
            result[key] = int(value)
        elif re.fullmatch(r"-?\d*\.\d+", value):
            result[key] = float(value)
        else:
            result[key] = value
    return result


cases = {}
for line in io.open(PROBE, encoding="utf-8"):
    match = CASE.match(line.strip())
    if match:
        cases[match.group(1)] = (match.group(2), split_options(match.group(3)))

catalog = json.load(io.open(CATALOG, encoding="utf-8"))
touched, missing = 0, []
for entry in catalog["kinds"]:
    family = entry.get("family")
    if family not in FAMILY_DEFAULT_DATA:
        continue
    slug = entry["slug"]
    if slug not in cases:
        missing.append(entry["id"])
        continue
    probe_family, options = cases[slug]
    if probe_family != family:
        missing.append(entry["id"] + " (family mismatch)")
        continue
    entry["options"] = options
    entry["data"] = options.get("data", FAMILY_DEFAULT_DATA[family])
    touched += 1

tmp = CATALOG + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="\n") as handle:
    json.dump(catalog, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
os.replace(tmp, CATALOG)
print("entries updated:", touched)
print("not covered by the probe:", missing)
