#!/usr/bin/env python3
"""🧊️ R10 one-off codemod (session 14b): routes the host modules' direct `three` imports through the ui module's interface
(`@semio-tech/ui-react`, whose 📰️Three.js region re-exports every needed name explicitly). Per file: every
`import {…} from "three"` / `"three/…"` statement is removed and its specifiers are appended to the file's first value
import from `@semio-tech/ui-react` (created after the last removed statement when the file has none). Renames:
`OrbitControls` from the three addon becomes `ThreeOrbitControls`. Refuses a file whose three imports it cannot parse.
Usage: python3 three-route.py [--apply]
"""
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
FILES = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer/🟦️.tsx",
]
THREE_IMPORT = re.compile(r'^import (type )?\{([^}]*)\} from "three(?:/[^"]*)?";\n', re.M)
UI_IMPORT = re.compile(r'^import (type )?\{\n?([^}]*)\} from "@semio-tech/ui-react";\n', re.M)
RENAMES = {"OrbitControls as ThreeOrbitControls": "ThreeOrbitControls"}


def specifiers(body: str, type_only: bool) -> list[str]:
    names = [part.strip() for part in body.replace("\n", " ").split(",") if part.strip()]
    names = [RENAMES.get(name, name) for name in names]
    return [f"type {name}" if type_only and not name.startswith("type ") else name for name in names]


def route(text: str) -> str:
    found = list(THREE_IMPORT.finditer(text))
    if not found:
        return text
    moved: list[str] = []
    for match in found:
        moved += specifiers(match.group(2), match.group(1) is not None)
    anchor = found[-1].end()
    text = THREE_IMPORT.sub("", text)
    ui = next((match for match in UI_IMPORT.finditer(text) if match.group(1) is None), None)
    if ui is None:
        typed = next(UI_IMPORT.finditer(text), None)
        if typed is not None:
            existing = specifiers(typed.group(2), True)
            merged = existing + [name for name in moved if name not in existing]
            return text[: typed.start()] + 'import { ' + ", ".join(merged) + ' } from "@semio-tech/ui-react";\n' + text[typed.end():]
        removed_before = sum(len(match.group(0)) for match in found)
        at = anchor - removed_before
        return text[:at] + 'import { ' + ", ".join(moved) + ' } from "@semio-tech/ui-react";\n' + text[at:]
    existing = [part.strip() for part in ui.group(2).split(",") if part.strip()]
    merged = existing + [name for name in moved if name not in existing]
    multiline = "\n" in ui.group(0).split("}")[0]
    if multiline:
        block = "import {\n" + "".join(f"  {name},\n" for name in merged) + '} from "@semio-tech/ui-react";\n'
    else:
        block = "import { " + ", ".join(merged) + ' } from "@semio-tech/ui-react";\n'
    return text[: ui.start()] + block + text[ui.end():]


def main() -> int:
    apply = "--apply" in sys.argv
    for relative in FILES:
        path = ROOT / relative
        before = path.read_text(encoding="utf-8")
        after = route(before)
        left = [line for line in after.splitlines() if re.search(r'from "three(?:/[^"]*)?";', line)]
        if left:
            print(f"REFUSED {relative}: unparsed three import {left[0][:80]}")
            return 1
        print(f"{'write' if apply and after != before else 'would write' if after != before else 'unchanged'} {relative.split('/')[-2]}")
        if apply and after != before:
            path.write_text(after, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
