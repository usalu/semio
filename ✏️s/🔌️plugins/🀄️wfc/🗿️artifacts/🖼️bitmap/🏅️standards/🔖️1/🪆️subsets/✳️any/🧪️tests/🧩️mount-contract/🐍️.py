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


if __name__ == "__main__":
    sys.exit(main())
