"""🗂️ S20 fault classes (row 12, coordinator decision 2026-09-29 20:5x): pre-classifies every declared code (app
declarations from the overlay census + the framework catalog) by code grammar, then text cues, into the review work
lists the family helpers correct by hand: `.🧬semio/🌐hub/s14-s20-sets/class/family-<A|BD|CEFG|H>.json` (one entry per line:
owner, code, class, rule, en). Never overwrites a work list (a reviewed list is the source of truth for the codemod).
Usage: python3 class-heuristic.py"""
from __future__ import annotations

import json
import re
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
CENSUS = OVERLAY / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🤖️generated/🎯️acceptance/🧯️fault-census.json"
CATALOG = OVERLAY / "🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🔣️.json"
OUT = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class")
CLASSES = ("input-invalid", "precondition-failed", "conflict", "permission-denied", "unavailable", "cancelled", "internal")
FAMILIES = {
    "BD": ["🗄️stdio", "🏗️fem", "🖍️draw"],
    "CEFG": ["🧩️puzzle", "🌊️flow", "🌀️procedural", "🎬️sequence", "🖨️raster", "🀄️wfc", "➗️mathematical", "📋️forms", "🔱️trinity", "🪐️space",
             "🏛️architect", "💡️reasoning", "🌿️vcs", "💠️lowpoly", "🌍️gis", "📐️cad", "🗒️note", "🏭️process", "📕️norm", "📏️layout"],
    "H": ["✒️writer", "📸️remodel", "🔋️energy", "🎞️animate", "🪵️sourcing", "🧱️block", "🕸️dag", "🎥️shooting", "🎪️demonstrator", "📜️imperative", "📖️playbook"],
}
CODE_RULES = [
    ("cancelled", r"cancel|aborted"),
    ("permission-denied", r"permission|read-only|readonly|denied|forbidden|access-revoked|unauthori[sz]ed|not-allowed"),
    ("conflict", r"conflict|stale|generation-mismatch|revision-mismatch|concurrent"),
    ("unavailable", r"unavailable|busy|time-?out|timed-out|not-ready|offline|throttl|not-wired|unbound|budget|saturat|backpressure|in-flight|retry"),
    ("internal", r"internal|invariant|poison|corrupt|unreachable|route-mismatch|tool-mismatch|owner|work-repeated|encode|serializ|leak|retire|registry|page-invalid|wire|protocol|dispatch|cleanup|maintenance|close|assembly|reducer|admission"),
    ("input-invalid", r"invalid|argument|args|value-required|parse|malformed|format|too-large|too-long|too-many|out-of-range|nonfinite|non-finite|unknown|unsupported|not-supported|grammar|syntax|limit|capacity|range|decode|unreadable|not-finite|negative|zero"),
    ("precondition-failed", r"missing|gone|not-found|required|no-|empty|locked|exists|duplicate|already|not-|selection|detached|changed|finished|needs"),
]
TEXT_RULES = [
    ("internal", r"report the problem|reload the (app|document|editor)|could not be (updated|prepared|completed)"),
    ("unavailable", r"wait a moment|try again later|is busy|not available right now"),
    ("input-invalid", r"correct it|correct them|enter (a|an|one|finite)|choose another|is not supported|must be"),
    ("precondition-failed", r"no longer exists|first\b|select|open (a|an|the)"),
]


def classify(code: str, en: str) -> tuple[str, str]:
    for cls, pattern in CODE_RULES:
        match = re.search(pattern, code.lower())
        if match:
            return cls, f"code:{match.group(0)}"
    for cls, pattern in TEXT_RULES:
        match = re.search(pattern, en.lower())
        if match:
            return cls, f"text:{match.group(0)}"
    return "precondition-failed", "default"


def main() -> None:
    census = json.loads(CENSUS.read_text())
    catalog = json.loads(CATALOG.read_text())["faults"]
    owners: dict[str, list[dict]] = {family: [] for family in ("A", *FAMILIES)}
    seen: set[tuple[str, str]] = set()
    for declaration in census["declarations"]:
        plugin = declaration["path"].split("/")[2]
        if (plugin, declaration["code"]) in seen:
            continue
        seen.add((plugin, declaration["code"]))
        family = next(name for name, plugins in FAMILIES.items() if plugin in plugins)
        cls, rule = classify(declaration["code"], declaration["en"])
        owners[family].append({"owner": plugin, "code": declaration["code"], "class": cls, "rule": rule, "en": declaration["en"]})
    for entry in catalog:
        cls, rule = classify(entry["code"], entry["en"])
        owners["A"].append({"owner": "framework", "code": entry["code"], "class": cls, "rule": rule, "en": entry["en"]})
    for family, entries in owners.items():
        target = OUT / f"family-{family}.json"
        entries.sort(key=lambda entry: (entry["owner"], entry["code"]))
        counts = {cls: sum(entry["class"] == cls for entry in entries) for cls in CLASSES}
        defaults = sum(entry["rule"] == "default" for entry in entries)
        if target.exists():
            print(f"family {family}: kept existing {target.name}")
            continue
        target.write_text("[\n" + ",\n".join(json.dumps(entry, ensure_ascii=False) for entry in entries) + "\n]\n")
        print(f"family {family}: {len(entries)} codes {counts} default={defaults} → {target}")


if __name__ == "__main__":
    main()
