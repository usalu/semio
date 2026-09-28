# -*- coding: utf-8 -*-
"""S18 §14c: a tree row's disclosure label (the `aria-labelledby` target of its fold button) carried React's raw `useId()`
(`_r_3_`) as its DOM id — not an element id (ELEMENT_ID_PATTERN), not addressable by i18n/automation/introduction anchors, and
the engine-contract law `per-window element ids` (two world surfaces' projection panes) went red on it; the second law of that
block failed only as its cascade (the first left its mount behind, so the next `getElementById` hit the stale pane's toggle).
Now the label id is `childElementId(<row id>, "disclosureLabel")` for a row with an element id, else
`ui.tree.disclosureLabel.<generated>` — always an element id, unique per mounted row. Also removes the temporary `[DEBUG] s18 ids`
line from the engine-contract law. Idempotent."""
import pathlib

TREE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx")
LAW = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts")

EDITS = [
    (TREE, 'import { childElementId } from "../🆔️ElementId/🟦️.tsx";\n', 'import { childElementId, elementIdSegment, isElementId } from "../🆔️ElementId/🟦️.tsx";\n'),
    (TREE, "  const disclosureLabelId = reactHostPort.useId();\n  const localizedLabel = useIdLabel(id);\n",
     "  const generatedLabelId = reactHostPort.useId();\n  const disclosureLabelId = id !== undefined && isElementId(id) ? childElementId(id, \"disclosureLabel\") : childElementId(\"ui.tree.disclosureLabel\", elementIdSegment(generatedLabelId));\n  const localizedLabel = useIdLabel(id);\n"),
]


def main() -> None:
    texts: dict[pathlib.Path, str] = {}
    for path, old, new in EDITS:
        text = texts.get(path) or path.read_text(encoding="utf-8")
        if new not in text:
            assert text.count(old) == 1, (path.name, old[:80])
            text = text.replace(old, new)
        texts[path] = text
    law = LAW.read_text(encoding="utf-8")
    lines = [line for line in law.split("\n") if "[DEBUG] s18 ids" not in line]
    texts[LAW] = "\n".join(lines)
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    print("ok")


main()
