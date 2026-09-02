import json
import os
import re
import subprocess

TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

territory = json.load(open(f"{TICKET}/🗑️generated/a4-territory-before.json"))

uri_re = re.compile(r"Fixture (asset|shared|local)://(\S+) does not resolve")

results = []
for b in territory:
    scope = b["scope"]
    m = uri_re.search(b["summary"])
    if not m:
        results.append({"scope": scope, "summary": b["summary"], "scheme": None})
        continue
    scheme, uri = m.group(1), m.group(2)
    # derive feature file dir
    feature_path = scope
    case_dir = os.path.dirname(feature_path)
    tests_dir = os.path.dirname(case_dir)  # .../🧪️tests
    owner_dir = os.path.dirname(tests_dir)  # the case owner directory

    if scheme == "asset":
        base = owner_dir
        target = os.path.join(base, uri)
    elif scheme == "shared":
        target = os.path.join(owner_dir, "🧫️fixtures", uri)
    elif scheme == "local":
        target = os.path.join(case_dir, "🧫️fixtures", uri)
    else:
        target = None

    exists = os.path.exists(target) if target else None

    # try to find candidate by basename (without extension) fuzzy match under owner_dir
    fname = os.path.basename(uri)
    stem, ext = os.path.splitext(fname)
    candidates = []
    search_root = owner_dir
    if os.path.isdir(search_root):
        try:
            out = subprocess.run(
                ["find", search_root, "-iname", f"*{ext}"],
                capture_output=True, text=True, timeout=20
            ).stdout.strip().splitlines()
            candidates = out
        except Exception as e:
            candidates = [f"ERROR: {e}"]

    results.append({
        "scope": scope,
        "scheme": scheme,
        "uri": uri,
        "owner_dir": owner_dir,
        "expected_target": target,
        "expected_exists": exists,
        "candidates_same_ext": candidates,
    })

with open(f"{TICKET}/🗑️generated/a4-classify-raw.json", "w") as f:
    json.dump(results, f, indent=2, ensure_ascii=False)

print("done", len(results))
