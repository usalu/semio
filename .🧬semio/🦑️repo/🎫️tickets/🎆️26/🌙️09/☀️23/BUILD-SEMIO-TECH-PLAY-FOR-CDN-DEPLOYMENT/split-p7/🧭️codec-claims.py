#!/usr/bin/env python3
"""🧭️ Applies the codec-ownership rule to one family-cut config (split lane P7): a native codec implements a foreign
framework trait for a domain type, so it must live in the crate owning that type. Text and binary snapshot codecs and the
SQLite snapshot codec join `-model`, diff codecs join `-model-diff` (created over `🧬️schema/🔺️diff`), mutation codecs
join `-mutations`, inference codecs join `-inferences`; a separate `-io-sqlite` crate is dissolved.

usage: 🧭️codec-claims.py <config.json>
"""
import json, sys
path = sys.argv[1]
c = json.load(open(path, encoding="utf-8"))
S = "🏅️standards/🔖️1/🪆️subsets/✳️any"
prefix = c["crate"].replace("_", "-")
claims = {"-model": ["^" + S + "/🚪️io/(📝️text|💾️binary)/📸️snapshot/", "^" + S + "/🚪️io/🪶️sqlite/"],
          "-model-diff": ["^" + S + "/🚪️io/(📝️text|💾️binary)/🔺️diff/"],
          "-mutations": ["^" + S + "/🚪️io/(📝️text|💾️binary)/🧬️mutations/"],
          "-inferences": ["^" + S + "/🚪️io/(📝️text|💾️binary)/💡️inferences/"]}
c["crates"] = [cr for cr in c["crates"] if cr["package"] != prefix + "-io-sqlite"]
if not any(cr["package"] == prefix + "-model-diff" for cr in c["crates"]):
    at = next(i for i, cr in enumerate(c["crates"]) if cr["package"] == prefix + "-model") + 1
    c["crates"].insert(at, {"package": prefix + "-model-diff", "owner": S + "/🧬️schema/🔺️diff", "description": prefix.split("-", 3)[-1] + " diff algebra and its text and binary codecs"})
for cr in c["crates"]:
    for suffix, rules in claims.items():
        if cr["package"] == prefix + suffix:
            cr["claims"] = [r for r in cr.get("claims", []) if r not in rules] + rules
for t in c.get("tests", []):
    if t["package"] == prefix + "-io-sqlite":
        t["package"] = prefix + "-model"
json.dump(c, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print([cr["package"] for cr in c["crates"]])
