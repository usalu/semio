#!/usr/bin/env python3
"""🔨️ F1 — merges the six additional `apply-<arm>` envelope fixtures (image, text, table, graph,
object, kit) `🔨️f1-semio-base-generate2.py` produced into `s.stdio.semio@v1/base`'s own
`🧪️oracle/🔣️.json`, same pattern as `🔨️f1-semio-base-merge.py`: each entry's `generator.oracle`
names the wrapped arm's own already-registered `verified-native-second-implementation` oracle."""
import json
import os

REPO = os.getcwd()
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"


def main():
    frag_path = os.path.join(REPO, TICKET, "🗑️generated", "f1-semio-base-fragments2.json")
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
        oracle = frag["oracle"]
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
                "oracle": oracle,
                "packageVersion": "n/a",
                "engineFamily": "cpython",
                "engineVersion": "3",
                "command": f"uv run --group test python3 {TICKET}/🔨️f1-semio-base-generate2.py",
                "platform": "darwin-arm64",
            },
            "provenance": {
                "source": "generated",
                "license": "AGPL-3.0-only",
                "attribution": (
                    f"The envelope threading is this file's own; the wrapped `{arm}` content and its "
                    f"real mutation effect were produced by `{oracle}` — that ARM subset's own "
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
                f"Applies {json.dumps(frag['envelope_mutation']['payload']['mutation'], ensure_ascii=False)} "
                f"to the real `{arm}` document via that arm's own apply_mutation, inside the envelope's "
                f"own SemioSnapshot{{schema, subset: SemioSubsetSnapshot::{arm.capitalize()}(...)}} shape. "
                f"Demonstrates the envelope's own routing law: a wrapped mutation whose arm ({arm!r}) "
                "matches the base snapshot's own arm threads through to a real, observable content "
                "change."
            ),
        }
        doc["fixtureManifests"].append(entry)

    with open(oracle_path, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print(f"merged {len(fragments)} fixture(s) into {oracle_path}")


if __name__ == "__main__":
    main()
