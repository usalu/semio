"""🧬️ S20 pass 2 on the p2 overlay — the fault law's `app-mutation-code` rule (spec §6): the `mutation.*` namespace is the
framework's (the frozen report codes, `MutationCode`), so an app crate never raises a `mutation.*` code; + a self-test case.
Idempotent. Usage: python3 p2-census.py"""
import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance")
CENSUS = ROOT / "📋️orchestration/🟦️.ts"
FIXTURE = ROOT / "🧫️fixtures/🧮️source-census/🔣️.json"
EDITS = [
    ('"parameter-mismatch" | "framework-app-origin";', '"parameter-mismatch" | "framework-app-origin" | "app-mutation-code";'),
    ("  return facts;\n}\n\n/** 🧩️ The placeholder names of a fault text",
     "  if (owner.kind === \"app\") for (const raise of facts.raises) if (raise.code.startsWith(\"mutation.\")) facts.violations.push({ path, line: raise.line, rule: \"app-mutation-code\", detail: `${raise.code} — the mutation.* codes are the framework's frozen report codes (MutationCode)` });\n  return facts;\n}\n\n/** 🧩️ The placeholder names of a fault text"),
]
CASE = {"name": "an app never raises a code of the framework's mutation.* namespace", "path": "✏️s/🔌️plugins/🎲️dice/🗿️artifacts/🎲️die/🦀️.rs",
        "text": "fn roll() -> Fault {\n    app_fault(\"mutation.missing\")\n}\n",
        "expected": {"raises": [{"line": 2, "code": "mutation.missing", "parameters": []}], "constRaises": [], "consts": [], "declarations": [], "violations": [{"line": 2, "rule": "app-mutation-code"}]}}
text = CENSUS.read_text()
for old, new in EDITS:
    if new in text:
        continue
    assert text.count(old) == 1, old[:60]
    text = text.replace(old, new)
CENSUS.write_text(text)
fixture = FIXTURE.read_text()
if CASE["name"] not in fixture:
    end = '\n  ],\n  "faultLaw": ['
    assert fixture.count(end) == 1
    fixture = fixture.replace(end, ",\n    " + json.dumps(CASE, ensure_ascii=False) + end)
    json.loads(fixture)
    FIXTURE.write_text(fixture)
print("ok")
