"""🧪️ The bitmap mount contract, read by an implementation that links nothing of this repository —
the language-agnostic half of `🦀️.rs` beside it. Run it from the repository root:

    python3 "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🐍️.py"
"""

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SUBSET = os.path.abspath(os.path.join(HERE, "..", ".."))
CONTRACT = os.path.join(SUBSET, "🧫️fixtures", "🧩️mount-contract", "🔣️.json")
MUTATIONS = os.path.join(SUBSET, "🧬️schema", "🧬️mutations")


def main():
    with open(CONTRACT, encoding="utf-8") as handle:
        contract = json.load(handle)
    problems = []

    if contract["editorAppId"] != "s.wfc.bitmap@1/*#editor":
        problems.append("the editor app id is not the canonical surface id")
    if contract["viewerAppId"] != "s.wfc.bitmap@1/*#viewer":
        problems.append("the viewer app id is not the canonical surface id")
    if contract["dialect"] != "s.wfc.bitmap@1/*":
        problems.append("the dialect coordinate does not match the app ids")
    if contract["artifactKindId"] != "2d.wfcbitmap":
        problems.append("the OS artifact kind id drifted")
    if contract["inferenceToolId"] != "s.wfc.bitmap.solve":
        problems.append("the inference tool id drifted")

    declared = contract["mutationKinds"]
    on_disk = sorted(
        json.load(open(os.path.join(MUTATIONS, entry, "🔣️.json"), encoding="utf-8"))["semanticKind"]
        for entry in os.listdir(MUTATIONS)
        if os.path.isfile(os.path.join(MUTATIONS, entry, "🔣️.json"))
    )
    if sorted(declared) != on_disk:
        problems.append("the declared mutation vocabulary differs from the kind directories on disk: " + repr(sorted(declared)) + " vs " + repr(on_disk))

    for entry in contract["examples"]:
        if entry["inputWidth"] <= 0 or entry["inputHeight"] <= 0:
            problems.append("example " + entry["id"] + " declares a degenerate sample")
        if entry["paletteSize"] < 2:
            problems.append("example " + entry["id"] + " declares fewer than two colours, which teaches no adjacency")
        for locale in ("en", "de"):
            if not entry["label"].get(locale):
                problems.append("example " + entry["id"] + " has no " + locale + " label")

    for problem in problems:
        print("FAIL " + problem)
    print("checked the bitmap mount contract, " + str(len(problems)) + " problems")
    return 1 if problems else 0


def adapter():
    """🧭️ The platform entry point: this reader answers the four scenarios from the committed statement and the kind
    directories on disk, in the SUBJECT role — it is a second implementation of reading the statement, never a
    reference for the Rust half, which measures the real surfaces."""
    from semio_repo_test import Adapter, Outcome

    def answer(projection):
        return Outcome(projection, raw=json.dumps(projection, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))

    def statement(ctx):
        return json.loads(ctx.fixture_bytes("shared://🧩️mount-contract/🔣️.json").decode("utf-8"))

    def surface_ids(ctx):
        contract = statement(ctx)
        expected = {"editorAppId": "s.wfc.bitmap@1/*#editor", "viewerAppId": "s.wfc.bitmap@1/*#viewer", "artifactKindId": "2d.wfcbitmap", "inferenceToolId": "s.wfc.bitmap.solve"}
        drift = [key for key, value in expected.items() if contract[key] != value]
        if drift or contract["dialect"] != "s.wfc.bitmap@1/*":
            raise AssertionError("surface-ids: the committed statement drifted from the canonical ids: %r" % (drift or ["dialect"]))
        return answer({key: contract[key] for key in expected})

    def window_kinds(ctx):
        kinds = statement(ctx)["windowKindIds"]
        if kinds != ["wfc-bitmap-input", "wfc-bitmap-output"]:
            raise AssertionError("window-kinds: the committed roster is %r" % kinds)
        return answer({"editor": kinds, "viewer": kinds})

    def mutation_vocabulary(ctx):
        declared = statement(ctx)["mutationKinds"]
        on_disk = sorted(json.load(open(os.path.join(MUTATIONS, entry, "🔣️.json"), encoding="utf-8"))["semanticKind"] for entry in os.listdir(MUTATIONS) if os.path.isfile(os.path.join(MUTATIONS, entry, "🔣️.json")))
        if sorted(declared) != on_disk:
            raise AssertionError("mutation-vocabulary: the declared roster %r differs from the kind directories on disk %r" % (sorted(declared), on_disk))
        return answer({"mutationKinds": declared})

    def examples(ctx):
        rows = statement(ctx)["examples"]
        for entry in rows:
            if entry["inputWidth"] <= 0 or entry["inputHeight"] <= 0 or entry["paletteSize"] < 2 or not all(entry["label"].get(locale) for locale in ("en", "de")):
                raise AssertionError("examples: %s is degenerate, single-coloured or not localized" % entry["id"])
        return answer({"examples": [{key: entry[key] for key in ("id", "inputWidth", "inputHeight", "paletteSize")} for entry in rows]})

    return Adapter("python").subject("surface-ids", surface_ids).subject("window-kinds", window_kinds).subject("mutation-vocabulary", mutation_vocabulary).subject("examples", examples)


if __name__ == "__main__":
    sys.exit(main())
