"""🔮 Promotes the two npm helper libraries to registrations of their own, so the dependency audit
classifies them by name instead of by an accidental cross-ecosystem name collision."""
import json

PATH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json"

ENTRIES = [
    {
        "id": "puzzle-2d-jsonschema-js-payloads",
        "kind": "third-party-library",
        "ecosystem": "javascript",
        "package": "jsonschema",
        "version": "1.5.0",
        "capabilities": ["puzzle-2d-1-mutate"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "MIT",
        "testOnly": True,
        "engine": {"family": "jsonschema-js", "implementation": "tdegrunt/jsonschema draft-07 validator on bun", "version": "1.5.0"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://github.com/tdegrunt/jsonschema",
        "rationale": (
            "The second ecosystem's payload validator, in `../🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts`. It stands "
            "in for `ajv`, which is DECLINED here on a rule this repository enforces mechanically: `ajv` is declared "
            "`production-runtime` by five packages in this tree and `verify dependencies literal-external` counts an "
            "oracle package production can reach as an `oracle-conflict`, which is this repository comparing itself "
            "with itself. This package carries the same draft-07 role with no production reachability, and its "
            "independence from `puzzle-2d-jsonschema-payloads` is what makes the 24 leaf-schema disagreements those "
            "two found credible: two validators, two ecosystems, the same 24 rejections."
        ),
    },
    {
        "id": "puzzle-2d-fast-json-patch-diff",
        "kind": "third-party-library",
        "ecosystem": "javascript",
        "package": "fast-json-patch",
        "version": "3.1.1",
        "capabilities": ["puzzle-2d-1-mutate"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "MIT",
        "testOnly": True,
        "engine": {"family": "fast-json-patch", "implementation": "Starcounter-Jack RFC 6902 implementation on bun", "version": "3.1.1"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://github.com/Starcounter-Jack/JSON-Patch",
        "rationale": (
            "The second ecosystem's differ, in `../🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts`, on the same terms as "
            "`puzzle-2d-jsonpatch-diff`: derive RFC 6902 from `(before, after)`, apply it back to reproduce the "
            "committed after-snapshot, and hold the typed `Puzzle2dDiff` to the members those op paths reach and to "
            "a `patched` set that is exactly the records RFC 6902 needs an operation for. Equality is asked of the "
            "library itself — an empty patch between the reproduction and the committed file — because "
            "`ordered-json-v1` holds array order significant and key order never, which a string comparison gets "
            "wrong. An unrelated implementation of the same RFC agreeing with the Python one is what rules out a "
            "differ-specific artefact."
        ),
    },
]


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        document = json.load(handle)
    known = {entry["id"] for entry in document["oracles"]}
    document["oracles"] = document["oracles"] + [entry for entry in ENTRIES if entry["id"] not in known]
    for entry in document["oracles"]:
        if entry["id"] == "puzzle-2d-graphology-graph":
            entry.pop("packages", None)
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print("oracles=%s" % [entry["id"] for entry in document["oracles"]])


if __name__ == "__main__":
    main()
