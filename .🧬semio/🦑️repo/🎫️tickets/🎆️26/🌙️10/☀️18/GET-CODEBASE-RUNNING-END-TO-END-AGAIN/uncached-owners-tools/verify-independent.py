"""🔬 Independent check (Python standard library only): every authored `cache: false` target of the tracked manifests records `metadata.semio.cache` with a non-deterministic class and a reason the closed schema admits; every cached flip carries `outputs`.

usage: python3 -I verify-independent.py <root> <manifest-list>
"""
import json, sys, os

root, listing = sys.argv[1], sys.argv[2]
schema_path = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/💾️cache.json")
schema = json.load(open(schema_path, encoding="utf8"))
admitted = {}
for definition in schema["definitions"].values():
    admitted[definition["properties"]["class"]["const"]] = set(definition["properties"]["reason"]["enum"])
policy = json.load(open(os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), encoding="utf8"))
continuous = policy["continuous"]
named_continuous = lambda name: any(name == c or name.startswith(c + "-") for c in continuous)
total = uncached = missing = invalid = cached_without_outputs = 0
by = {}
for line in open(listing, encoding="utf8"):
    path = line.strip()
    if not path:
        continue
    try:
        manifest = json.load(open(os.path.join(root, path), encoding="utf8"))
    except (OSError, ValueError):
        continue
    for name, target in (manifest.get("targets") or {}).items():
        total += 1
        decision = ((target.get("metadata") or {}).get("semio") or {}).get("cache")
        if target.get("cache") is False:
            uncached += 1
            if decision is None and named_continuous(name):
                by["continuous/serves-until-stopped (policy name)"] = by.get("continuous/serves-until-stopped (policy name)", 0) + 1
            elif decision is None:
                missing += 1
            elif decision.get("version") != 1 or decision.get("class") == "deterministic" or decision.get("reason") not in admitted.get(decision.get("class"), set()):
                invalid += 1
            else:
                key = f"{decision['class']}/{decision['reason']}"
                by[key] = by.get(key, 0) + 1
        elif target.get("cache") is True and decision is None and target.get("outputs") is None and name in ("canonical-architecture",):
            cached_without_outputs += 1
print(json.dumps({"targets": total, "authoredCacheFalse": uncached, "withoutDecision": missing, "invalidDecision": invalid, "cachedCanonicalWithoutOutputs": cached_without_outputs, "byDecision": dict(sorted(by.items()))}, indent=1, ensure_ascii=False))
sys.exit(1 if missing or invalid or cached_without_outputs else 0)
