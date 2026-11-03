"""Wave 2 features v3 (input script): moves every undiscovered prose feature `<X>/🧪️tests/🥒️.feature` into the discoverable case folder `<X>/🧪️tests/<case>/🥒️.feature`.
Scenarios no step family can execute become "Further specification assertions" (hand-worded in the step grammar's Given/When/Then order); artifact scene and schema features are re-tagged to their registered evaluate capability.
Usage: .venv/bin/python -I wave2-features-v3-promote-prose.py [--apply]"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import re
import shutil
import sys
from pathlib import Path

NORM = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
BACKUP = Path(__file__).resolve().parent.parent / "🗑️generated" / "features" / "backup-v3"
STEP = re.compile(r"^\s*(Given|When|Then|And|But)\s+(.*)$")


def parse(text):
    head, scenarios, current, tags = [], [], None, []
    for line in text.splitlines():
        stripped = line.strip()
        if re.match(r"^(Scenario Outline|Scenario):", stripped):
            current = {"name": stripped.split(":", 1)[1].strip(), "outline": stripped.startswith("Scenario Outline"), "steps": [], "rows": [], "header": None}
            scenarios.append(current)
            tags = []
        elif current is None:
            if stripped.startswith("@id-") or stripped.startswith("@level-") or stripped.startswith("@mode-"):
                continue
            head.append(line)
        elif stripped.startswith("@"):
            tags.append(stripped)
        elif STEP.match(line):
            current["steps"].append((STEP.match(line).group(1), STEP.match(line).group(2)))
        elif stripped.startswith("|"):
            cells = [cell.strip() for cell in stripped.strip("|").split("|")]
            if current["header"] is None:
                current["header"] = cells
            else:
                current["rows"].append(cells)
    return head, scenarios


def bullet(scenario):
    steps = " ".join("%s %s" % (word, rest) for word, rest in scenario["steps"])
    text = "- %s: %s" % (scenario["name"], steps)
    if scenario["outline"] and scenario["rows"]:
        ids = [row[0] for row in scenario["rows"]] if scenario["header"] and scenario["header"][0] == "id" else []
        text += " Examples (%d rows%s)" % (len(scenario["rows"]), ": " + ", ".join(ids) if ids and len(ids) <= 12 else "")
    return text


def case_name(feature, norm_relative):
    parts = feature.parent.parent.parts
    owner = feature.parent.parent.name
    artifact = re.search(r"(en19\d\d)/", str(feature))
    if artifact:
        return "%s-%s-1" % (owner, artifact.group(1))
    return owner


def retag(head, artifact):
    out = []
    for line in head:
        if line.startswith("@"):
            continue
        out.append(line)
    return ["@capability-%s-1-evaluate" % artifact, "@oracle-%s-1-compliance-independent" % artifact, "@comparison-numerical-v1"] + out


def main(apply):
    features = sorted(NORM.rglob("🧪️tests/🥒️.feature"))
    features = [path for path in features if path.parent.name == "🧪️tests" and "node_modules" not in str(path)]
    print(len(features), "undiscovered prose features")
    for feature in features:
        head, scenarios = parse(feature.read_text(encoding="utf8"))
        artifact = re.search(r"(en19\d\d)/", str(feature))
        if artifact:
            head = retag(head, artifact.group(1))
        while head and not head[-1].strip():
            head.pop()
        body = ["", "  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:"] + ["  " + bullet(scenario) for scenario in scenarios]
        name = case_name(feature, None)
        target = feature.parent / name / "🥒️.feature"
        print(("apply " if apply else "plan  ") + str(feature.relative_to(NORM)).replace("🏅️standards/🔖️1/🪆️subsets/✳️any", "~"), "->", name, len(scenarios), "scenarios")
        if apply:
            BACKUP.mkdir(parents=True, exist_ok=True)
            shutil.copy2(feature, BACKUP / (str(feature.relative_to(NORM)).replace("/", "__")))
            target.parent.mkdir(exist_ok=True)
            target.write_text("\n".join(head + body) + "\n", encoding="utf8")
            feature.unlink()


main("--apply" in sys.argv)
