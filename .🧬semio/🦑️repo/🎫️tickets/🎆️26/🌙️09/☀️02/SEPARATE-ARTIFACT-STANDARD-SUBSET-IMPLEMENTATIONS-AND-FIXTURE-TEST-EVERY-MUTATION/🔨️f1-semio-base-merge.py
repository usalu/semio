#!/usr/bin/env python3
"""🔨️ F1 — merges the seven `apply-<arm>` envelope fixtures into `s.stdio.semio@v1/base`'s own
`🧪️oracle/🔣️.json`. `base` registers no oracle of its own (its committed no-oracle decision explains
why a routing-level second implementation was rejected), so each entry's `generator.oracle` names
the WRAPPED ARM's own already-registered `verified-native-second-implementation` oracle instead —
the registry's oracle table is repository-wide, and the fixture's real content genuinely was produced
by that arm's own registered generator, merely wrapped in the envelope this file owns."""
import json
import os

REPO = os.getcwd()
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"

ARM_ORACLE = {
    "animation": "semio-animation-python-independent",
    "audio": "semio-audio-python-independent",
    "flow": "semio-flow-python-independent",
    "model": "semio-model-python-independent",
    "presentation": "semio-presentation-python-independent",
    "value": "semio-value-python-independent",
    "video": "semio-video-python-independent",
}


def main():
    frag_path = os.path.join(REPO, TICKET, "🗑️generated", "f1-semio-base-fragments.json")
    with open(frag_path, encoding="utf-8") as f:
        fragments = json.load(f)

    oracle_path = os.path.join(REPO, ROOT, "✳️base", "🧪️oracle", "🔣️.json")
    with open(oracle_path, encoding="utf-8") as f:
        doc = json.load(f)

    doc.setdefault("fixtureManifests", [])
    existing_ids = {fm["id"] for fm in doc["fixtureManifests"]}

    for frag in fragments:
        if frag["id"] in existing_ids:
            continue
        arm = frag["mutation"]
        entry = {
            "schema": "semio.repository-test.fixture/v2",
            "id": frag["id"],
            "class": "third-party-generated",
            "target": {"artifact": "s.stdio.semio", "standard": "v1", "subset": "base"},
            "mutation": arm,
            "outcome": "applied",
            "units": {"length": "unitless", "angle": "radian"},
            "files": frag["files"],
            "generator": {
                "oracle": ARM_ORACLE[arm],
                "packageVersion": "n/a",
                "engineFamily": "cpython",
                "engineVersion": "3",
                "command": f"uv run --group test python3 {TICKET}/🔨️f1-semio-base-generate.py",
                "platform": "darwin-arm64",
            },
            "provenance": {
                "source": "generated",
                "license": "AGPL-3.0-only",
                "attribution": (
                    f"The envelope threading is this file's own; the wrapped `{arm}` content and its "
                    f"real mutation effect were produced by `{ARM_ORACLE[arm]}` — that ARM subset's own "
                    "already-registered verified-native-second-implementation oracle (registered in "
                    f"`../../../../🪆️subsets/✳️{arm}/🧪️oracle/🔣️.json`, visible here because the oracle "
                    "registry is repository-wide) — not a third-party package; `class: "
                    "third-party-generated` is used only because it is the schema's own closest-fitting "
                    "enum value for a generator-produced fixture."
                ),
                "security": "scanned-clean",
                "privacy": "no-personal-data",
            },
            "comparisonProfile": "ordered-json-v1",
            "reproducible": True,
            "family": "structural",
            "notes": (
                f"Wraps `{frag['wrapped_kind']}`, applied to the real `{arm}` document via that arm's own "
                f"apply_mutation, inside the envelope's own SemioSnapshot{{schema, subset: "
                f"SemioSubsetSnapshot::{arm.capitalize()}(...)}} shape (confirmed against this subset's "
                "own committed `replaces-the-envelope-wrapping-a-value-subset` vector's JSON encoding). "
                f"Demonstrates the envelope's own routing law this subset's no-oracle decision names: a "
                f"wrapped mutation whose arm ({arm!r}) matches the base snapshot's own arm threads "
                "through to a real, observable content change."
            ),
        }
        doc["fixtureManifests"].append(entry)

    with open(oracle_path, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print(f"merged {len(fragments)} fixture(s) into {oracle_path}")


if __name__ == "__main__":
    main()
