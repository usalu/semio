#!/usr/bin/env python3
"""🔨️ F1 — merges the fixtures `🔨️f1-step-generate.py` produced into each STEP subset's own
`🧪️oracle/🔣️.json`, registering a `fixtureManifests[]` (schema `fixture/v2`) entry per mutation and,
for `cc1..cc6` (which have no subset-scoped `steputils` reader yet — only `base` does), a new
`third-party-library` oracle entry mirroring the one already registered for `base`."""
import json
import os

REPO = os.getcwd()
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
BASE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets"
COMMAND = f"uv run --group test python3 {TICKET}/🔨️f1-step-generate.py"

SUBSET_ORACLE_ID = {
    "✳️base": "steputils-step-ap214-base-mutate-reader",
    "✳️cc1": "steputils-step-ap214-cc1-mutate-reader",
    "✳️cc2": "steputils-step-ap214-cc2-mutate-reader",
    "✳️cc3": "steputils-step-ap214-cc3-mutate-reader",
    "✳️cc4": "steputils-step-ap214-cc4-mutate-reader",
    "✳️cc5": "steputils-step-ap214-cc5-mutate-reader",
    "✳️cc6": "steputils-step-ap214-cc6-mutate-reader",
}
SUBSET_CAPABILITY = {
    "✳️base": "step-ap214-base-mutate",
    "✳️cc1": "step-ap214-cc1-mutate",
    "✳️cc2": "step-ap214-cc2-mutate",
    "✳️cc3": "step-ap214-cc3-mutate",
    "✳️cc4": "step-ap214-cc4-mutate",
    "✳️cc5": "step-ap214-cc5-mutate",
    "✳️cc6": "step-ap214-cc6-mutate",
}
SUBSET_NAME = {"✳️base": "base", "✳️cc1": "cc1", "✳️cc2": "cc2", "✳️cc3": "cc3", "✳️cc4": "cc4", "✳️cc5": "cc5", "✳️cc6": "cc6"}


def new_oracle_entry(subset_emoji):
    subset = SUBSET_NAME[subset_emoji]
    return {
        "id": SUBSET_ORACLE_ID[subset_emoji],
        "kind": "third-party-library",
        "ecosystem": "python",
        "package": "steputils",
        "version": "0.1",
        "engine": {"family": "steputils", "implementation": "steputils Part-21/STEP reader+writer", "version": "0.1"},
        "capabilities": [SUBSET_CAPABILITY[subset_emoji]],
        "license": "MIT",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://github.com/mozman/steputils",
        "comparisonProfiles": ["semantic-step-v1"],
        "rationale": (
            f"📖️ F1 (ticket SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION). "
            f"Mirrors the sibling `steputils-step-ap214-base-mutate-reader` entry registered for `base`, scoped to "
            f"`{subset}`'s own `{SUBSET_CAPABILITY[subset_emoji]}` capability — this subset shares the exact same real "
            f"committed fixture (`shared://🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp`) and entity vocabulary as "
            f"`base`, just filtered to this conformance class. steputils 0.1 (mozman, MIT) both READS "
            f"(`p21.readfile`) and WRITES (`StepFile.save`) real ISO 10303-21 Part-21 text — verified directly this "
            f"session by round-tripping this subset's own fixture and diffing the result: a real parse into typed "
            f"`Entity`/`ParameterList`/`Reference`/`Keyword` objects, re-serialized through steputils' own writer, "
            f"never through this repository's `ruststep`-adjacent or production STEP writer. Used here as the "
            f"generator for every `fixtureManifests[]` entry below this rationale registers: `before` is this "
            f"fixture round-tripped unmodified, `after` is the same load with exactly one raw entity/header edit "
            f"applied through steputils' own object model before saving — so both sides of every diff are genuinely "
            f"produced by an independent third-party parser+writer, never predicted by this repository's own "
            f"StepMutation/StepDiff implementation."
        ),
    }


def merge_subset(subset_emoji, fragments):
    subset_root = os.path.join(REPO, BASE, subset_emoji)
    oracle_path = os.path.join(subset_root, "🧪️oracle", "🔣️.json")
    with open(oracle_path, "r", encoding="utf-8") as f:
        doc = json.load(f)

    oracle_id = SUBSET_ORACLE_ID[subset_emoji]
    if subset_emoji != "✳️base":
        existing_ids = {o["id"] for o in doc.get("oracles", [])}
        if oracle_id not in existing_ids:
            doc.setdefault("oracles", []).append(new_oracle_entry(subset_emoji))

    doc.setdefault("fixtureManifests", [])
    existing_fixture_ids = {fm["id"] for fm in doc["fixtureManifests"]}

    for frag in fragments:
        if frag["id"] in existing_fixture_ids:
            continue
        entry = {
            "schema": "semio.repository-test.fixture/v2",
            "id": frag["id"],
            "class": "third-party-generated",
            "target": {"artifact": "s.stdio.step", "standard": "ap214", "subset": SUBSET_NAME[subset_emoji]},
            "mutation": frag["mutation"],
            "outcome": "applied",
            "units": {"length": "millimetre", "angle": "radian"},
            "files": frag["files"],
            "generator": {
                "oracle": oracle_id,
                "packageVersion": "0.1",
                "engineFamily": "steputils",
                "engineVersion": "0.1",
                "command": f"{COMMAND}",
                "platform": "darwin-arm64",
            },
            "provenance": {
                "source": "generated",
                "license": "MIT (steputils)",
                "attribution": "Generated with steputils 0.1 (MIT, mozman) via a real parse+write round trip against this repository's own committed real-world fixture 🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp",
                "security": "scanned-clean",
                "privacy": "no-personal-data",
            },
            "comparisonProfile": "semantic-step-v1",
            "reproducible": True,
            "family": "structural",
            "notes": frag["note"],
        }
        doc["fixtureManifests"].append(entry)

    with open(oracle_path, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print(f"merged {len(fragments)} fixture(s) into {oracle_path}")


def main():
    frag_path = os.path.join(REPO, TICKET, "🗑️generated", "f1-step-fragments.json")
    with open(frag_path, "r", encoding="utf-8") as f:
        fragments_by_subset = json.load(f)
    for subset_emoji, fragments in fragments_by_subset.items():
        merge_subset(subset_emoji, fragments)


if __name__ == "__main__":
    main()
